use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use serde_json::json;

use crate::state::AppState;

fn err(status: StatusCode, code: &str, message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (status, Json(json!({ "error": { "code": code, "message": message } })))
}

#[derive(Deserialize)]
pub struct ListQuery {
    pub limit: Option<u32>,
}

pub async fn list(
    Query(q): Query<ListQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let limit = q.limit.unwrap_or(50);
    match state.db.list_events(limit) {
        Ok(events) => (StatusCode::OK, Json(json!(events))).into_response(),
        Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
    }
}

pub async fn undo(
    Path(event_id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let entry = {
        let mut stack = state.undo_stack.lock().unwrap();
        match stack.try_undo(&event_id) {
            Ok(result) => result,
            Err(e) => return err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
        }
    };

    match entry {
        None => err(StatusCode::NOT_FOUND, "UNDO_NOT_FOUND", "No undo entry for this event, or the undo window has expired").into_response(),
        Some(e) => {
            // Reverse the file operation
            if e.is_copy {
                // File was copied to inbox. Check if it's still there (may have been moved by workflow).
                match tokio::fs::metadata(&e.dst_path).await {
                    Ok(_) => {
                        if let Err(rm_err) = tokio::fs::remove_file(&e.dst_path).await {
                            return err(StatusCode::INTERNAL_SERVER_ERROR, "UNDO_FAILED", &format!("Failed to remove inbox copy: {}", rm_err)).into_response();
                        }
                        tracing::info!(path = %e.dst_path, "Undo: removed inbox copy");
                    }
                    Err(_) => {
                        // File was already moved/processed by a workflow
                        return (StatusCode::OK, Json(serde_json::json!({
                            "event_id": e.event_id,
                            "undone": false,
                            "message": "File was already processed by a workflow and cannot be restored from inbox"
                        }))).into_response();
                    }
                }
            } else {
                if let Err(mv_err) = tokio::fs::rename(&e.dst_path, &e.src_path).await {
                    return err(StatusCode::INTERNAL_SERVER_ERROR, "UNDO_FAILED", &format!("Failed to restore file: {}", mv_err)).into_response();
                }
                tracing::info!(from = %e.dst_path, to = %e.src_path, "Undo: restored file");
            }
            (StatusCode::OK, Json(serde_json::json!({
                "event_id": e.event_id,
                "undone": {
                    "src_path": e.src_path,
                    "dst_path": e.dst_path
                }
            }))).into_response()
        }
    }
}
