/// Tauri commands for file ingestion + rule→workflow pipeline

use crate::commands::undo::AppUndoStack;
use crate::AppEventBus;
use draction_domain::ids;
use draction_domain::rule::Rule;
use draction_domain::workflow::Workflow;
use draction_engine::rule_engine::{self, EvalCtx};
use draction_engine::workflow_engine::WorkflowEngine;
use draction_events::Envelope;
use draction_inbox::undo::UndoEntry;
use serde::Serialize;
use serde_json::json;
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use tauri::{AppHandle, Emitter};

const LARGE_FILE_THRESHOLD: u64 = 100 * 1024 * 1024; // 100 MB
const CHUNK_SIZE: usize = 1024 * 1024; // 1 MB

#[derive(Clone, Serialize)]
struct IngestProgress {
    file_name: String,
    bytes_copied: u64,
    total_bytes: u64,
    percent: f64,
}

async fn copy_with_progress(
    src: &Path,
    dest: &Path,
    app_handle: &AppHandle,
) -> Result<(), String> {
    use tokio::io::{AsyncReadExt, AsyncWriteExt};

    let file_name = src
        .file_name()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    let metadata = tokio::fs::metadata(src)
        .await
        .map_err(|e| format!("Metadata error: {e}"))?;
    let total_bytes = metadata.len();

    let mut reader = tokio::fs::File::open(src)
        .await
        .map_err(|e| format!("Open src error: {e}"))?;
    let mut writer = tokio::fs::File::create(dest)
        .await
        .map_err(|e| format!("Create dest error: {e}"))?;

    let mut buf = vec![0u8; CHUNK_SIZE];
    let mut bytes_copied: u64 = 0;

    loop {
        let n = reader
            .read(&mut buf)
            .await
            .map_err(|e| format!("Read error: {e}"))?;
        if n == 0 {
            break;
        }
        writer
            .write_all(&buf[..n])
            .await
            .map_err(|e| format!("Write error: {e}"))?;
        bytes_copied += n as u64;

        let percent = if total_bytes > 0 {
            (bytes_copied as f64 / total_bytes as f64) * 100.0
        } else {
            100.0
        };

        let _ = app_handle.emit(
            "ingest-progress",
            IngestProgress {
                file_name: file_name.clone(),
                bytes_copied,
                total_bytes,
                percent,
            },
        );
    }

    writer
        .flush()
        .await
        .map_err(|e| format!("Flush error: {e}"))?;

    Ok(())
}

#[derive(Debug, Serialize, Clone)]
pub struct IngestResult {
    pub original: String,
    pub inbox_path: String,
    pub size_bytes: u64,
    pub sha256: String,
    pub action: Option<String>,
}

/// Collect all files from a path. If directory, walk recursively.
async fn collect_files(path: &PathBuf) -> Result<Vec<PathBuf>, String> {
    if path.is_file() {
        return Ok(vec![path.clone()]);
    }
    if !path.is_dir() {
        return Ok(vec![]);
    }

    let mut files = Vec::new();
    let mut stack = vec![path.clone()];

    while let Some(dir) = stack.pop() {
        let mut entries = tokio::fs::read_dir(&dir)
            .await
            .map_err(|e| format!("Cannot read dir {}: {}", dir.display(), e))?;

        while let Some(entry) = entries
            .next_entry()
            .await
            .map_err(|e| format!("Read dir entry error: {}", e))?
        {
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

/// Load rules from ~/Draction/rules.json, creating defaults if missing.
async fn load_rules(base: &std::path::Path) -> Result<Vec<Rule>, String> {
    let rules_path = base.join("rules.json");

    if !rules_path.exists() {
        let defaults = default_rules();
        let json = serde_json::to_string_pretty(&defaults)
            .map_err(|e| format!("JSON serialize: {e}"))?;
        tokio::fs::write(&rules_path, &json)
            .await
            .map_err(|e| format!("Write rules.json: {e}"))?;
        tracing::info!("Created default rules at {}", rules_path.display());
        return Ok(defaults);
    }

    let data = tokio::fs::read_to_string(&rules_path)
        .await
        .map_err(|e| format!("Read rules.json: {e}"))?;
    serde_json::from_str(&data).map_err(|e| format!("Parse rules.json: {e}"))
}

/// Load workflows from ~/Draction/workflows.json, creating defaults if missing.
async fn load_workflows(base: &std::path::Path) -> Result<Vec<Workflow>, String> {
    let wf_path = base.join("workflows.json");

    if !wf_path.exists() {
        let defaults = default_workflows();
        let json = serde_json::to_string_pretty(&defaults)
            .map_err(|e| format!("JSON serialize: {e}"))?;
        tokio::fs::write(&wf_path, &json)
            .await
            .map_err(|e| format!("Write workflows.json: {e}"))?;
        tracing::info!("Created default workflows at {}", wf_path.display());
        return Ok(defaults);
    }

    let data = tokio::fs::read_to_string(&wf_path)
        .await
        .map_err(|e| format!("Read workflows.json: {e}"))?;
    serde_json::from_str(&data).map_err(|e| format!("Parse workflows.json: {e}"))
}

fn default_rules() -> Vec<Rule> {
    use draction_domain::rule::*;
    vec![
        Rule {
            id: "rule_images_to_photos".into(),
            name: "Images → Photos".into(),
            enabled: true,
            order_index: 0,
            when: Condition::Predicate {
                field: "ext".into(),
                op: Op::In,
                value: json!(["jpg", "jpeg", "png", "gif", "webp", "svg", "bmp", "tiff", "heic"]),
            },
            then: ThenAction {
                workflow_id: "wf_move_to_photos".into(),
            },
        },
        Rule {
            id: "rule_videos_to_videos".into(),
            name: "Videos → Videos".into(),
            enabled: true,
            order_index: 1,
            when: Condition::Predicate {
                field: "ext".into(),
                op: Op::In,
                value: json!(["mp4", "mov", "avi", "mkv", "webm", "m4v"]),
            },
            then: ThenAction {
                workflow_id: "wf_move_to_videos".into(),
            },
        },
        Rule {
            id: "rule_docs_to_documents".into(),
            name: "Documents → Documents".into(),
            enabled: true,
            order_index: 2,
            when: Condition::Predicate {
                field: "ext".into(),
                op: Op::In,
                value: json!(["pdf", "doc", "docx", "xls", "xlsx", "ppt", "pptx", "txt", "md", "csv"]),
            },
            then: ThenAction {
                workflow_id: "wf_move_to_documents".into(),
            },
        },
    ]
}

fn default_workflows() -> Vec<Workflow> {
    use draction_domain::workflow::*;
    vec![
        Workflow {
            id: "wf_move_to_photos".into(),
            name: "Move to Photos".into(),
            nodes: vec![WorkflowNode {
                id: "n1".into(),
                node_type: "move".into(),
                params: json!({ "dest": "~/Draction/Photos" }),
            }],
            edges: vec![],
        },
        Workflow {
            id: "wf_move_to_videos".into(),
            name: "Move to Videos".into(),
            nodes: vec![WorkflowNode {
                id: "n1".into(),
                node_type: "move".into(),
                params: json!({ "dest": "~/Draction/Videos" }),
            }],
            edges: vec![],
        },
        Workflow {
            id: "wf_move_to_documents".into(),
            name: "Move to Documents".into(),
            nodes: vec![WorkflowNode {
                id: "n1".into(),
                node_type: "move".into(),
                params: json!({ "dest": "~/Draction/Documents" }),
            }],
            edges: vec![],
        },
    ]
}

/// Build EvalCtx from a single file's metadata.
fn build_eval_ctx(name: &str, ext: &str, size: u64) -> EvalCtx {
    let mut ctx: EvalCtx = HashMap::new();
    ctx.insert("name".into(), json!(name));
    ctx.insert("ext".into(), json!(ext));
    ctx.insert("size_bytes".into(), json!(size));
    ctx
}

/// Ingest dropped files/folders: copy to inbox → match rules → execute workflow.
#[tauri::command]
pub async fn ingest_files(
    paths: Vec<String>,
    undo_stack: tauri::State<'_, AppUndoStack>,
    event_bus: tauri::State<'_, AppEventBus>,
    app_handle: AppHandle,
) -> Result<Vec<IngestResult>, String> {
    let home = dirs::home_dir().ok_or("Cannot find home directory")?;
    let base = home.join("Draction");
    let inbox = draction_inbox::ingest::inbox_dir(&base);

    // Ensure base dir exists
    tokio::fs::create_dir_all(&base)
        .await
        .map_err(|e| format!("Create Draction dir: {e}"))?;

    // Load rules & workflows
    let rules = load_rules(&base).await?;
    let workflows = load_workflows(&base).await?;

    // Build workflow engine
    let registry = draction_engine::default_registry();
    let engine = WorkflowEngine::new(registry);

    // Flatten directories into individual files
    let mut all_files = Vec::new();
    for path_str in &paths {
        let src = PathBuf::from(path_str);
        if !src.exists() {
            tracing::warn!("Path not found: {}", path_str);
            continue;
        }
        let files = collect_files(&src).await?;
        tracing::info!("Collected {} file(s) from {}", files.len(), path_str);
        all_files.extend(files);
    }

    if all_files.is_empty() {
        return Err("No files found in the dropped items".into());
    }

    let event_id = ids::new_event_id();
    let mut results = Vec::new();

    for src in &all_files {
        // 1. Copy to inbox (chunked for large files, fast path for small files)
        let src_size = tokio::fs::metadata(src)
            .await
            .map(|m| m.len())
            .unwrap_or(0);

        let dest = if src_size > LARGE_FILE_THRESHOLD {
            // Determine destination path the same way ingest_file would
            let file_name = src.file_name().unwrap_or_default();
            let dest_path = inbox.join(file_name);
            tokio::fs::create_dir_all(&inbox)
                .await
                .map_err(|e| format!("Create inbox dir: {e}"))?;
            copy_with_progress(src, &dest_path, &app_handle).await?;
            dest_path
        } else {
            draction_inbox::ingest::ingest_file(src, &inbox, true)
                .await
                .map_err(|e| format!("Ingest failed for {}: {}", src.display(), e))?
        };

        // 1b. Push undo entry for this ingest
        {
            let entry = UndoEntry {
                event_id: event_id.clone(),
                src_path: src.to_string_lossy().to_string(),
                dst_path: dest.to_string_lossy().to_string(),
                is_copy: true,
                created_at: chrono::Utc::now(),
            };
            if let Ok(mut stack) = undo_stack.0.lock() {
                stack.push(entry);
            }
        }

        // 2. Compute metadata
        let sha256 = draction_inbox::file_ops::compute_sha256(&dest)
            .await
            .map_err(|e| format!("SHA256 failed: {e}"))?;
        let size = draction_inbox::file_ops::file_size(&dest)
            .await
            .map_err(|e| format!("Size failed: {e}"))?;

        let original_name = src.file_name().unwrap_or_default().to_string_lossy().to_string();
        let ext = src.extension().unwrap_or_default().to_string_lossy().to_lowercase();

        tracing::info!(
            file = %original_name,
            dest = %dest.display(),
            size = size,
            sha256 = %&sha256[..12],
            "File ingested"
        );

        // 3. Emit EVENT_INGESTED
        event_bus.0.emit(Envelope {
            channel: "events".into(),
            payload: json!({
                "type": "EVENT_INGESTED",
                "eventId": event_id,
                "time": chrono::Utc::now().to_rfc3339(),
                "source": {
                    "kind": "desktop_drop",
                    "deviceName": std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".into()),
                    "ip": "127.0.0.1"
                },
                "files": [{
                    "path": dest.to_string_lossy(),
                    "name": original_name,
                    "ext": ext,
                    "sizeBytes": size,
                    "sha256": sha256
                }]
            }),
        });

        // 4. Match rules
        let eval_ctx = build_eval_ctx(&original_name, &ext, size);
        let mut action_desc: Option<String> = None;

        if let Some(rule) = rule_engine::match_first_rule(&rules, &eval_ctx) {
            tracing::info!(rule = %rule.name, file = %original_name, "Rule matched");

            // 5. Execute workflow
            if let Some(wf) = workflows.iter().find(|w| w.id == rule.then.workflow_id) {
                let run_id = ids::new_run_id();
                let work_dir = dest.to_string_lossy().to_string();

                // Emit RUN_STARTED
                event_bus.0.emit(Envelope {
                    channel: "events".into(),
                    payload: json!({
                        "type": "RUN_STARTED",
                        "runId": run_id,
                        "eventId": event_id,
                        "ruleId": rule.id,
                        "workflowId": wf.id,
                        "startedAt": chrono::Utc::now().to_rfc3339()
                    }),
                });

                match engine.execute(&run_id, &event_id, wf, &work_dir).await {
                    Ok(()) => {
                        action_desc = Some(format!("{} → {}", rule.name, wf.name));
                        tracing::info!(rule = %rule.name, workflow = %wf.name, "Workflow completed");

                        // Emit RUN_FINISHED
                        event_bus.0.emit(Envelope {
                            channel: "events".into(),
                            payload: json!({
                                "type": "RUN_FINISHED",
                                "runId": run_id,
                                "eventId": event_id,
                                "ruleId": rule.id,
                                "workflowId": wf.id,
                                "summary": format!("{} → {}", rule.name, wf.name),
                                "artifacts": []
                            }),
                        });
                    }
                    Err(e) => {
                        tracing::error!(rule = %rule.name, err = %e, "Workflow failed");
                        action_desc = Some(format!("{} (failed: {})", rule.name, e));

                        // Emit RUN_FAILED
                        event_bus.0.emit(Envelope {
                            channel: "events".into(),
                            payload: json!({
                                "type": "RUN_FAILED",
                                "runId": run_id,
                                "eventId": event_id,
                                "ruleId": rule.id,
                                "workflowId": wf.id,
                                "failedNodeId": null,
                                "error": {
                                    "code": "WORKFLOW_ERROR",
                                    "message": e.to_string(),
                                    "retryable": false
                                },
                                "partialArtifacts": []
                            }),
                        });
                    }
                }
            } else {
                tracing::warn!(workflow_id = %rule.then.workflow_id, "Workflow not found");
            }
        } else {
            tracing::info!(file = %original_name, "No rule matched — file stays in inbox");
            action_desc = Some("No matching rule — kept in Inbox".into());
        }

        results.push(IngestResult {
            original: original_name,
            inbox_path: dest.to_string_lossy().to_string(),
            size_bytes: size,
            sha256,
            action: action_desc,
        });
    }

    tracing::info!("{} file(s) processed", results.len());
    Ok(results)
}
