use draction_app_core::{DractionRuntime, IngestResult};
use serde::Serialize;
use std::path::PathBuf;
use tauri::{AppHandle, Emitter, Manager};

#[derive(Clone, Serialize)]
struct TauriIngestProgress {
    file_name: String,
    bytes_copied: u64,
    total_bytes: u64,
    percent: f64,
}

#[tauri::command]
pub async fn ingest_files(
    paths: Vec<String>,
    app_handle: AppHandle,
) -> Result<Vec<IngestResult>, String> {
    let runtime = app_handle
        .state::<DractionRuntime>()
        .inner()
        .clone();

    let path_bufs: Vec<PathBuf> = paths.iter().map(PathBuf::from).collect();

    // Create a progress sender that forwards to Tauri events
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<draction_app_core::IngestProgress>();
    let handle = app_handle.clone();
    tauri::async_runtime::spawn(async move {
        while let Some(p) = rx.recv().await {
            let _ = handle.emit(
                "ingest-progress",
                TauriIngestProgress {
                    file_name: p.file_name,
                    bytes_copied: p.bytes_copied,
                    total_bytes: p.total_bytes,
                    percent: p.percent,
                },
            );
        }
    });

    runtime
        .ingest_paths(path_bufs, Some(tx))
        .await
        .map_err(|e| format!("Ingest failed: {e}"))
}
