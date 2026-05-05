use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use notify::{Event, EventKind, RecursiveMode, Watcher};
use serde::Deserialize;
use serde_json::json;
use std::path::PathBuf;
use std::sync::atomic::Ordering;
use tokio::sync::mpsc;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct StartWatcherRequest {
    pub paths: Vec<String>,
}

pub async fn start_watcher(
    State(state): State<AppState>,
    Json(body): Json<StartWatcherRequest>,
) -> impl IntoResponse {
    let watch_paths: Vec<PathBuf> = body.paths.into_iter().map(PathBuf::from).collect();

    // Validate paths exist
    for p in &watch_paths {
        if !p.exists() {
            return (
                StatusCode::UNPROCESSABLE_ENTITY,
                Json(json!({ "error": { "code": "INVALID_PATH", "message": format!("Path does not exist: {}", p.display()) } })),
            ).into_response();
        }
    }

    // Resolve the tx from state up front
    let tx = {
        let guard = state.watcher_tx.lock().unwrap();
        match guard.as_ref() {
            Some(tx) => tx.clone(),
            None => {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({ "error": { "code": "INTERNAL_ERROR", "message": "Ingest channel not available" } })),
                ).into_response();
            }
        }
    };

    // Create stop flag
    let stop_flag = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let flag_clone = stop_flag.clone();

    // Debounce channel
    let (event_tx, mut event_rx) = mpsc::channel::<PathBuf>(256);

    // Debounce consumer: batch files within 500ms windows, send batch to ingest tx
    tokio::spawn(async move {
        let mut pending: Vec<PathBuf> = Vec::new();
        loop {
            tokio::select! {
                path = event_rx.recv() => {
                    match path {
                        Some(p) => pending.push(p),
                        None => break,
                    }
                    tokio::time::sleep(std::time::Duration::from_millis(500)).await;
                    let files = std::mem::take(&mut pending);
                    if !files.is_empty() {
                        tracing::info!("Watch folder detected {} file(s)", files.len());
                        let _ = tx.send(files);
                    }
                }
            }
        }
    });

    // Set up filesystem watcher
    let watcher = match notify::recommended_watcher(move |res: Result<Event, notify::Error>| {
        if let Ok(event) = res {
            if matches!(event.kind, EventKind::Create(_)) {
                for path in event.paths {
                    let _ = event_tx.blocking_send(path);
                }
            }
        }
    }) {
        Ok(w) => w,
        Err(e) => {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": { "code": "WATCHER_ERROR", "message": e.to_string() } })),
            ).into_response();
        }
    };

    let mut watcher = watcher;
    for path in &watch_paths {
        if let Err(e) = watcher.watch(path, RecursiveMode::NonRecursive) {
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({ "error": { "code": "WATCH_ERROR", "message": format!("Cannot watch {}: {}", path.display(), e) } })),
            ).into_response();
        }
        tracing::info!("Watching folder: {}", path.display());
    }

    // Store stop flag in state
    {
        let mut guard = state.watcher_flag.lock().unwrap();
        *guard = Some(flag_clone);
    }

    // Keep watcher alive in a background task
    let keep_flag = stop_flag.clone();
    tokio::spawn(async move {
        let _watcher = watcher;
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            if keep_flag.load(Ordering::Relaxed) {
                break;
            }
        }
    });

    (StatusCode::OK, Json(json!({ "status": "started" }))).into_response()
}

pub async fn stop_watcher(
    State(state): State<AppState>,
) -> impl IntoResponse {
    let mut guard = state.watcher_flag.lock().unwrap();
    match guard.as_ref() {
        Some(flag) => {
            flag.store(true, Ordering::Relaxed);
            *guard = None;
            (StatusCode::OK, Json(json!({ "status": "stopped" }))).into_response()
        }
        None => {
            (StatusCode::CONFLICT, Json(json!({ "error": { "code": "NOT_RUNNING", "message": "No active watcher" } }))).into_response()
        }
    }
}
