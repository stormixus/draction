use anyhow::{Context, Result};
use chrono::Utc;
use draction_api::state::AppState;
use draction_db::DractionDb;
use draction_domain::ids;
use draction_domain::rule::{Condition, Op, Rule, ThenAction};
use draction_domain::settings::Settings;
use draction_domain::workflow::{Workflow, WorkflowNode};
use draction_engine::rule_engine::{self, EvalCtx};
use draction_engine::workflow_engine::WorkflowEngine;
use draction_events::{Envelope, EventBus};
use draction_inbox::undo::{UndoEntry, UndoStack};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::HashMap;
use std::fmt;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex};

pub mod watcher;

const LARGE_FILE_THRESHOLD: u64 = 100 * 1024 * 1024; // 100 MB
const CHUNK_SIZE: usize = 1024 * 1024; // 1 MB

#[derive(Clone)]
pub struct DractionRuntime {
    pub base_dir: PathBuf,
    pub api_port: u16,
    pub db: Arc<DractionDb>,
    pub event_bus: Arc<EventBus>,
    pub undo_stack: Arc<Mutex<UndoStack>>,
    pub auth_token: String,
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

#[derive(Debug, Clone)]
pub struct IngestProgress {
    pub file_name: String,
    pub bytes_copied: u64,
    pub total_bytes: u64,
    pub percent: f64,
}

pub type ProgressSender = tokio::sync::mpsc::UnboundedSender<IngestProgress>;

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
        if let Ok(dir) = std::env::var("DRACTION_BASE_DIR") {
            return Self::bootstrap_with_base(PathBuf::from(dir)).await;
        }
        let home = dirs::home_dir().context("Cannot find home directory")?;
        Self::bootstrap_with_base(home.join("Draction")).await
    }

    pub async fn bootstrap_with_base(base_dir: PathBuf) -> Result<Self> {
        tokio::fs::create_dir_all(&base_dir)
            .await
            .with_context(|| format!("Create {}", base_dir.display()))?;

        let lock_path = base_dir.join(".lock");
        draction_lifecycle::lock::acquire_lock(&lock_path)
            .with_context(|| format!("Acquire lock at {}", lock_path.display()))?;

        let db = Arc::new(DractionDb::open(&base_dir.join("draction.db"))?);
        db.mark_running_as_failed()?;

        let auth_token = draction_api::auth::load_or_create_token(&base_dir).await?;
        let event_bus = Arc::new(EventBus::new(256));
        let undo_stack = Arc::new(Mutex::new(UndoStack::new()));
        // Channel bridge: watcher handlers send paths here, runtime consumes them
        let (watcher_tx, mut watcher_rx) =
            tokio::sync::mpsc::unbounded_channel::<Vec<PathBuf>>();

        let state = AppState {
            db: db.clone(),
            base_dir: base_dir.clone(),
            auth_token: auth_token.clone(),
            event_bus: event_bus.clone(),
            undo_stack: undo_stack.clone(),
            watcher_flag: Arc::new(Mutex::new(None)),
            watcher_tx: Arc::new(Mutex::new(Some(watcher_tx))),
        };

        let api_port = draction_api::start_server(9400, state).await?;

        // Build runtime and spawn the watcher path consumer
        let runtime = Self {
            base_dir,
            api_port,
            db,
            event_bus,
            undo_stack,
            auth_token,
        };

        let consumer_runtime = runtime.clone();
        tokio::spawn(async move {
            while let Some(paths) = watcher_rx.recv().await {
                if let Err(e) = consumer_runtime.ingest_paths(paths, None).await {
                    tracing::error!("Watch folder ingest failed: {}", e);
                }
            }
        });

        ensure_rules(&runtime.base_dir).await?;
        ensure_workflows(&runtime.base_dir).await?;

        Ok(runtime)
    }

    pub fn list_runs(&self, limit: u32, offset: u32) -> Result<Vec<RunSummary>> {
        let rows = self.db.list_runs(None, limit, offset)?;
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

    pub async fn list_rules(&self) -> Result<Vec<RuleSummary>> {
        Ok(ensure_rules(&self.base_dir).await?
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

    pub async fn ingest_paths(
        &self,
        paths: Vec<PathBuf>,
        progress: Option<ProgressSender>,
    ) -> Result<Vec<IngestResult>> {
        let rules = ensure_rules(&self.base_dir).await?;
        let workflows = ensure_workflows(&self.base_dir).await?;
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

        use tokio::task::JoinSet;

        let mut join_set: JoinSet<anyhow::Result<IngestResult>> = JoinSet::new();
        let semaphore = Arc::new(tokio::sync::Semaphore::new(4));

        for src in all_files {
            let runtime = self.clone();
            let rules = rules.clone();
            let workflows = workflows.clone();
            let inbox = inbox.clone();
            let sem = semaphore.clone();
            let progress = progress.clone();

            join_set.spawn(async move {
                let _permit = sem.acquire().await.unwrap();

                let original_name = src
                    .file_name()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_string();

                let src_size =
                    tokio::fs::metadata(&src).await.map(|m| m.len()).unwrap_or(0);

                // Check file size limit
                let settings = Settings::load(&runtime.base_dir).await.unwrap_or_default();
                let max_bytes = settings.max_file_size_mb * 1024 * 1024;
                if src_size > max_bytes {
                    let size_str = if src_size >= 1024 * 1024 * 1024 {
                        format!("{:.1} GB", src_size as f64 / (1024.0 * 1024.0 * 1024.0))
                    } else if src_size >= 1024 * 1024 {
                        format!("{:.1} MB", src_size as f64 / (1024.0 * 1024.0))
                    } else {
                        format!("{} KB", src_size / 1024)
                    };
                    let msg = format!(
                        "File '{}' ({}) exceeds max size ({} MB)",
                        original_name,
                        size_str,
                        settings.max_file_size_mb,
                    );
                    runtime.event_bus.emit(Envelope {
                        channel: "errors".to_string(),
                        payload: json!({
                            "type": "FILE_TOO_LARGE",
                            "file": original_name,
                            "size_bytes": src_size,
                            "max_bytes": max_bytes,
                            "message": msg,
                        }),
                    });
                    return Ok(IngestResult {
                        original: original_name,
                        inbox_path: String::new(),
                        size_bytes: src_size,
                        sha256: String::new(),
                        action: Some(msg),
                    });
                }

                // Copy to inbox: use chunked copy for large files, fast path for small
                let dest = if src_size > LARGE_FILE_THRESHOLD && progress.is_some() {
                    let file_name = src.file_name().unwrap_or_default();
                    let dest_path = inbox.join(file_name);
                    tokio::fs::create_dir_all(&inbox)
                        .await
                        .with_context(|| format!("Create inbox dir {}", inbox.display()))?;
                    copy_with_progress(&src, &dest_path, progress.as_ref().unwrap())
                        .await?;
                    dest_path
                } else {
                    draction_inbox::ingest::ingest_file(&src, &inbox, true)
                        .await
                        .with_context(|| format!("Ingest failed for {}", src.display()))?
                };

                let sha256 = draction_inbox::file_ops::compute_sha256(&dest).await?;
                let size = draction_inbox::file_ops::file_size(&dest).await?;
                let ext = src
                    .extension()
                    .unwrap_or_default()
                    .to_string_lossy()
                    .to_lowercase();

                let event_id = ids::new_event_id();

                // Push undo entry
                {
                    let entry = UndoEntry {
                        event_id: event_id.clone(),
                        src_path: src.to_string_lossy().to_string(),
                        dst_path: dest.to_string_lossy().to_string(),
                        is_copy: true,
                        created_at: Utc::now(),
                    };
                    if let Ok(mut stack) = runtime.undo_stack.lock() {
                        stack.push(entry);
                    }
                }

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
                    "deviceName": std::env::var("HOSTNAME")
                        .unwrap_or_else(|_| "localhost".to_string()),
                    "ip": "127.0.0.1",
                });

                runtime.db.insert_event(
                    &event_id,
                    &occurred_at,
                    &source_payload.to_string(),
                    &files_payload.to_string(),
                )?;

                runtime.event_bus.emit(Envelope {
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

                if let Some(rule) =
                    rule_engine::match_first_rule(&rules, &eval_ctx)
                {
                    if let Some(workflow) =
                        workflows.iter().find(|wf| wf.id == rule.then.workflow_id)
                    {
                        let run_id = ids::new_run_id();
                        let started_at = Utc::now().to_rfc3339();

                        runtime.db.insert_run(
                            &run_id,
                            &event_id,
                            &rule.id,
                            &workflow.id,
                            "running",
                            &started_at,
                        )?;

                        runtime.event_bus.emit(Envelope {
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

                        let registry = draction_engine::default_registry();
                        let engine = WorkflowEngine::new(registry);
                        let work_dir = dest.to_string_lossy().to_string();
                        match engine
                            .execute(&run_id, &event_id, workflow, &work_dir)
                            .await
                        {
                            Ok(()) => {
                                let summary =
                                    format!("{} -> {}", rule.name, workflow.name);
                                action_desc = Some(summary.clone());
                                runtime.db.update_run_status(
                                    &run_id,
                                    "completed",
                                    Some(&Utc::now().to_rfc3339()),
                                    None,
                                    Some("[]"),
                                )?;
                                runtime.event_bus.emit(Envelope {
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
                                action_desc = Some(format!(
                                    "{} failed: {}",
                                    rule.name, error
                                ));
                                runtime.db.update_run_status(
                                    &run_id,
                                    "failed",
                                    Some(&Utc::now().to_rfc3339()),
                                    Some(&error_payload.to_string()),
                                    Some("[]"),
                                )?;
                                runtime.event_bus.emit(Envelope {
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
                        action_desc = Some(format!(
                            "Workflow not found: {}",
                            rule.then.workflow_id
                        ));
                    }
                } else {
                    action_desc =
                        Some("No matching rule - kept in Inbox".to_string());
                }

                Ok(IngestResult {
                    original: original_name,
                    inbox_path: dest.to_string_lossy().to_string(),
                    size_bytes: size,
                    sha256,
                    action: action_desc,
                })
            });
        }

        let mut results = Vec::new();
        while let Some(outcome) = join_set.join_next().await {
            match outcome {
                Ok(Ok(result)) => results.push(result),
                Ok(Err(e)) => {
                    tracing::error!("File ingest failed: {e:#}");
                }
                Err(join_err) => {
                    tracing::error!("Ingest task panicked: {join_err}");
                }
            }
        }

        Ok(results)
    }
}

async fn copy_with_progress(src: &Path, dest: &Path, sender: &ProgressSender) -> Result<()> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let file_name = src
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let metadata = tokio::fs::metadata(src).await?;
    let total_bytes = metadata.len();

    let mut reader = tokio::fs::File::open(src).await?;
    let mut writer = tokio::fs::File::create(dest).await?;

    let mut buf = vec![0u8; CHUNK_SIZE];
    let mut bytes_copied: u64 = 0;

    loop {
        let n = reader.read(&mut buf).await?;
        if n == 0 {
            break;
        }
        writer.write_all(&buf[..n]).await?;
        bytes_copied += n as u64;

        let percent = if total_bytes > 0 {
            (bytes_copied as f64 / total_bytes as f64) * 100.0
        } else {
            100.0
        };

        let _ = sender.send(IngestProgress {
            file_name: file_name.clone(),
            bytes_copied,
            total_bytes,
            percent,
        });
    }

    writer.flush().await?;
    Ok(())
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

async fn ensure_rules(base: &Path) -> Result<Vec<Rule>> {
    let path = rules_path(base);
    if !path.exists() {
        let defaults = default_rules();
        tokio::fs::write(&path, serde_json::to_string_pretty(&defaults)?).await?;
        return Ok(defaults);
    }

    let data = tokio::fs::read_to_string(&path).await?;
    Ok(serde_json::from_str(&data)?)
}

async fn ensure_workflows(base: &Path) -> Result<Vec<Workflow>> {
    let path = workflows_path(base);
    if !path.exists() {
        let defaults = default_workflows();
        tokio::fs::write(&path, serde_json::to_string_pretty(&defaults)?).await?;
        return Ok(defaults);
    }

    let data = tokio::fs::read_to_string(&path).await?;
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
