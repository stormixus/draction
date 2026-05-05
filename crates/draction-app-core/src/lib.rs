use anyhow::{Context, Result};
use chrono::Utc;
use draction_api::state::AppState;
use draction_db::DractionDb;
use draction_domain::ids;
use draction_domain::rule::{Condition, Op, Rule, ThenAction};
use draction_domain::workflow::{Workflow, WorkflowNode};
use draction_engine::rule_engine::{self, EvalCtx};
use draction_engine::workflow_engine::WorkflowEngine;
use draction_events::{Envelope, EventBus};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::Arc;

#[derive(Clone)]
pub struct DractionRuntime {
    pub base_dir: PathBuf,
    pub api_port: u16,
    db: Arc<DractionDb>,
    event_bus: Arc<EventBus>,
}

impl fmt::Debug for DractionRuntime {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("DractionRuntime")
            .field("base_dir", &self.base_dir)
            .field("api_port", &self.api_port)
            .finish_non_exhaustive()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IngestResult {
    pub original: String,
    pub inbox_path: String,
    pub size_bytes: u64,
    pub sha256: String,
    pub action: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RunSummary {
    pub id: String,
    pub event_id: String,
    pub rule_id: String,
    pub workflow_id: String,
    pub status: String,
    pub started_at: String,
    pub finished_at: Option<String>,
    pub error_json: Option<String>,
    pub artifacts_json: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleSummary {
    pub id: String,
    pub name: String,
    pub enabled: bool,
    pub order_index: i64,
    pub workflow_id: String,
}

impl DractionRuntime {
    pub async fn bootstrap() -> Result<Self> {
        let home = dirs::home_dir().context("Cannot find home directory")?;
        let base_dir = home.join("Draction");
        tokio::fs::create_dir_all(&base_dir)
            .await
            .with_context(|| format!("Create {}", base_dir.display()))?;

        let lock_path = base_dir.join(".lock");
        draction_lifecycle::lock::acquire_lock(&lock_path)
            .with_context(|| format!("Acquire lock at {}", lock_path.display()))?;

        let db = Arc::new(DractionDb::open(&base_dir.join("draction.db"))?);
        db.mark_running_as_failed()?;

        let auth_token = draction_api::auth::load_or_create_token(&base_dir)?;
        let event_bus = Arc::new(EventBus::new(256));
        let state = AppState {
            db: db.clone(),
            base_dir: base_dir.clone(),
            auth_token,
            event_bus: event_bus.clone(),
        };
        let api_port = draction_api::start_server(9400, state).await?;

        ensure_rules(&base_dir)?;
        ensure_workflows(&base_dir)?;

        Ok(Self {
            base_dir,
            api_port,
            db,
            event_bus,
        })
    }

    pub fn list_runs(&self, limit: u32) -> Result<Vec<RunSummary>> {
        let rows = self.db.list_runs(None, limit)?;
        Ok(rows
            .into_iter()
            .map(|row| RunSummary {
                id: row.id,
                event_id: row.event_id,
                rule_id: row.rule_id,
                workflow_id: row.workflow_id,
                status: row.status,
                started_at: row.started_at,
                finished_at: row.finished_at,
                error_json: row.error_json,
                artifacts_json: row.artifacts_json,
            })
            .collect())
    }

    pub fn list_rules(&self) -> Result<Vec<RuleSummary>> {
        Ok(ensure_rules(&self.base_dir)?
            .into_iter()
            .map(|rule| RuleSummary {
                workflow_id: rule.then.workflow_id,
                id: rule.id,
                name: rule.name,
                enabled: rule.enabled,
                order_index: rule.order_index,
            })
            .collect())
    }

    pub async fn ingest_paths(&self, paths: Vec<PathBuf>) -> Result<Vec<IngestResult>> {
        let rules = ensure_rules(&self.base_dir)?;
        let workflows = ensure_workflows(&self.base_dir)?;
        let registry = draction_engine::default_registry();
        let engine = WorkflowEngine::new(registry);
        let inbox = draction_inbox::ingest::inbox_dir(&self.base_dir);

        let mut all_files = Vec::new();
        for path in paths {
            if path.exists() {
                all_files.extend(collect_files(&path).await?);
            } else {
                tracing::warn!(path = %path.display(), "Dropped path does not exist");
            }
        }

        if all_files.is_empty() {
            anyhow::bail!("No files found in the dropped items");
        }

        let mut results = Vec::new();

        for src in all_files {
            let dest = draction_inbox::ingest::ingest_file(&src, &inbox, true)
                .await
                .with_context(|| format!("Ingest failed for {}", src.display()))?;

            let sha256 = draction_inbox::file_ops::compute_sha256(&dest).await?;
            let size = draction_inbox::file_ops::file_size(&dest).await?;
            let original_name = src
                .file_name()
                .unwrap_or_default()
                .to_string_lossy()
                .to_string();
            let ext = src
                .extension()
                .unwrap_or_default()
                .to_string_lossy()
                .to_lowercase();

            let event_id = ids::new_event_id();
            let occurred_at = Utc::now().to_rfc3339();
            let files_payload = json!([{
                "path": dest.to_string_lossy(),
                "name": original_name,
                "ext": ext,
                "sizeBytes": size,
                "sha256": sha256,
            }]);
            let source_payload = json!({
                "kind": "desktop_drop",
                "deviceName": std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string()),
                "ip": "127.0.0.1",
            });

            self.db.insert_event(
                &event_id,
                &occurred_at,
                &source_payload.to_string(),
                &files_payload.to_string(),
            )?;

            self.event_bus.emit(Envelope {
                channel: "events".to_string(),
                payload: json!({
                    "type": "EVENT_INGESTED",
                    "eventId": event_id,
                    "time": occurred_at,
                    "source": source_payload,
                    "files": files_payload,
                }),
            });

            let eval_ctx = build_eval_ctx(&original_name, &ext, size);
            let action_desc: Option<String>;

            if let Some(rule) = rule_engine::match_first_rule(&rules, &eval_ctx) {
                if let Some(workflow) = workflows.iter().find(|wf| wf.id == rule.then.workflow_id) {
                    let run_id = ids::new_run_id();
                    let started_at = Utc::now().to_rfc3339();

                    self.db.insert_run(
                        &run_id,
                        &event_id,
                        &rule.id,
                        &workflow.id,
                        "running",
                        &started_at,
                    )?;

                    self.event_bus.emit(Envelope {
                        channel: "events".to_string(),
                        payload: json!({
                            "type": "RUN_STARTED",
                            "runId": run_id,
                            "eventId": event_id,
                            "ruleId": rule.id,
                            "workflowId": workflow.id,
                            "startedAt": started_at,
                        }),
                    });

                    let work_dir = dest.to_string_lossy().to_string();
                    match engine
                        .execute(&run_id, &event_id, workflow, &work_dir)
                        .await
                    {
                        Ok(()) => {
                            let summary = format!("{} -> {}", rule.name, workflow.name);
                            action_desc = Some(summary.clone());
                            self.db.update_run_status(
                                &run_id,
                                "completed",
                                Some(&Utc::now().to_rfc3339()),
                                None,
                                Some("[]"),
                            )?;
                            self.event_bus.emit(Envelope {
                                channel: "events".to_string(),
                                payload: json!({
                                    "type": "RUN_FINISHED",
                                    "runId": run_id,
                                    "eventId": event_id,
                                    "ruleId": rule.id,
                                    "workflowId": workflow.id,
                                    "summary": summary,
                                    "artifacts": [],
                                }),
                            });
                        }
                        Err(error) => {
                            let error_payload = json!({
                                "code": "WORKFLOW_ERROR",
                                "message": error.to_string(),
                                "retryable": false,
                            });
                            action_desc = Some(format!("{} failed: {}", rule.name, error));
                            self.db.update_run_status(
                                &run_id,
                                "failed",
                                Some(&Utc::now().to_rfc3339()),
                                Some(&error_payload.to_string()),
                                Some("[]"),
                            )?;
                            self.event_bus.emit(Envelope {
                                channel: "events".to_string(),
                                payload: json!({
                                    "type": "RUN_FAILED",
                                    "runId": run_id,
                                    "eventId": event_id,
                                    "ruleId": rule.id,
                                    "workflowId": workflow.id,
                                    "failedNodeId": null,
                                    "error": error_payload,
                                    "partialArtifacts": [],
                                }),
                            });
                        }
                    }
                } else {
                    action_desc = Some(format!("Workflow not found: {}", rule.then.workflow_id));
                }
            } else {
                action_desc = Some("No matching rule - kept in Inbox".to_string());
            }

            results.push(IngestResult {
                original: original_name,
                inbox_path: dest.to_string_lossy().to_string(),
                size_bytes: size,
                sha256,
                action: action_desc,
            });
        }

        Ok(results)
    }
}

async fn collect_files(path: &Path) -> Result<Vec<PathBuf>> {
    if path.is_file() {
        return Ok(vec![path.to_path_buf()]);
    }

    if !path.is_dir() {
        return Ok(Vec::new());
    }

    let mut files = Vec::new();
    let mut stack = vec![path.to_path_buf()];

    while let Some(dir) = stack.pop() {
        let mut entries = tokio::fs::read_dir(&dir)
            .await
            .with_context(|| format!("Cannot read dir {}", dir.display()))?;

        while let Some(entry) = entries.next_entry().await? {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                stack.push(entry_path);
            } else if entry_path.is_file() {
                files.push(entry_path);
            }
        }
    }

    Ok(files)
}

fn rules_path(base: &Path) -> PathBuf {
    base.join("rules.json")
}

fn workflows_path(base: &Path) -> PathBuf {
    base.join("workflows.json")
}

fn ensure_rules(base: &Path) -> Result<Vec<Rule>> {
    let path = rules_path(base);
    if !path.exists() {
        let defaults = default_rules();
        std::fs::write(&path, serde_json::to_string_pretty(&defaults)?)?;
        return Ok(defaults);
    }

    let data = std::fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&data)?)
}

fn ensure_workflows(base: &Path) -> Result<Vec<Workflow>> {
    let path = workflows_path(base);
    if !path.exists() {
        let defaults = default_workflows();
        std::fs::write(&path, serde_json::to_string_pretty(&defaults)?)?;
        return Ok(defaults);
    }

    let data = std::fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&data)?)
}

fn default_rules() -> Vec<Rule> {
    vec![
        Rule {
            id: "rule_images_to_photos".to_string(),
            name: "Images -> Photos".to_string(),
            enabled: true,
            order_index: 0,
            when: Condition::Predicate {
                field: "ext".to_string(),
                op: Op::In,
                value: json!([
                    "jpg", "jpeg", "png", "gif", "webp", "svg", "bmp", "tiff", "heic"
                ]),
            },
            then: ThenAction {
                workflow_id: "wf_move_to_photos".to_string(),
            },
        },
        Rule {
            id: "rule_videos_to_videos".to_string(),
            name: "Videos -> Videos".to_string(),
            enabled: true,
            order_index: 1,
            when: Condition::Predicate {
                field: "ext".to_string(),
                op: Op::In,
                value: json!(["mp4", "mov", "avi", "mkv", "webm", "m4v"]),
            },
            then: ThenAction {
                workflow_id: "wf_move_to_videos".to_string(),
            },
        },
        Rule {
            id: "rule_docs_to_documents".to_string(),
            name: "Documents -> Documents".to_string(),
            enabled: true,
            order_index: 2,
            when: Condition::Predicate {
                field: "ext".to_string(),
                op: Op::In,
                value: json!([
                    "pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "md", "csv"
                ]),
            },
            then: ThenAction {
                workflow_id: "wf_move_to_documents".to_string(),
            },
        },
    ]
}

fn default_workflows() -> Vec<Workflow> {
    vec![
        Workflow {
            id: "wf_move_to_photos".to_string(),
            name: "Move to Photos".to_string(),
            nodes: vec![WorkflowNode {
                id: "n1".to_string(),
                node_type: "move".to_string(),
                params: json!({ "dest": "~/Draction/Photos" }),
            }],
            edges: vec![],
        },
        Workflow {
            id: "wf_move_to_videos".to_string(),
            name: "Move to Videos".to_string(),
            nodes: vec![WorkflowNode {
                id: "n1".to_string(),
                node_type: "move".to_string(),
                params: json!({ "dest": "~/Draction/Videos" }),
            }],
            edges: vec![],
        },
        Workflow {
            id: "wf_move_to_documents".to_string(),
            name: "Move to Documents".to_string(),
            nodes: vec![WorkflowNode {
                id: "n1".to_string(),
                node_type: "move".to_string(),
                params: json!({ "dest": "~/Draction/Documents" }),
            }],
            edges: vec![],
        },
    ]
}

fn build_eval_ctx(name: &str, ext: &str, size: u64) -> EvalCtx {
    let mut ctx: EvalCtx = HashMap::new();
    ctx.insert("name".to_string(), json!(name));
    ctx.insert("ext".to_string(), json!(ext));
    ctx.insert("size_bytes".to_string(), json!(size));
    ctx
}
