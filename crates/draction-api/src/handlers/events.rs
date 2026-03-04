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

pub async fn undo(Path(_event_id): Path<String>) -> impl IntoResponse {
    err(StatusCode::NOT_IMPLEMENTED, "NOT_IMPLEMENTED", "Undo is not yet implemented").into_response()
}
