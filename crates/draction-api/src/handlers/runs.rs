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
    pub status: Option<String>,
    pub limit: Option<u32>,
}

pub async fn list(
    Query(q): Query<ListQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let limit = q.limit.unwrap_or(50);
    match state.db.list_runs(q.status.as_deref(), limit) {
        Ok(runs) => (StatusCode::OK, Json(json!(runs))).into_response(),
        Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
    }
}

pub async fn get_one(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match state.db.get_run(&id) {
        Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
        Ok(Some(run)) => (StatusCode::OK, Json(json!(run))).into_response(),
        Ok(None) => err(StatusCode::NOT_FOUND, "NOT_FOUND", "Run not found").into_response(),
    }
}

pub async fn retry(Path(_id): Path<String>) -> impl IntoResponse {
    err(StatusCode::NOT_IMPLEMENTED, "NOT_IMPLEMENTED", "Retry is not yet implemented").into_response()
}
