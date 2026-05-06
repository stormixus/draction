use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use chrono::Utc;
use draction_domain::workflow::Workflow;
use draction_engine::workflow_engine::WorkflowEngine;
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
    pub offset: Option<u32>,
}

pub async fn list(
    Query(q): Query<ListQuery>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let limit = q.limit.unwrap_or(50);
    let offset = q.offset.unwrap_or(0);
    match state.db.list_runs(q.status.as_deref(), limit, offset) {
        Ok(runs) => {
            let total = state.db.count_runs(q.status.as_deref()).unwrap_or(0);
            (StatusCode::OK, Json(json!({
                "items": runs,
                "total": total,
                "limit": limit,
                "offset": offset,
            }))).into_response()
        }
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

pub async fn retry(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let run = match state.db.get_run(&id) {
        Err(e) => return err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
        Ok(None) => return err(StatusCode::NOT_FOUND, "NOT_FOUND", "Run not found").into_response(),
        Ok(Some(run)) => run,
    };

    if run.status != "failed" {
        return err(StatusCode::BAD_REQUEST, "BAD_REQUEST", "Only failed runs can be retried").into_response();
    }

    let event = match state.db.get_event(&run.event_id) {
        Err(e) => return err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
        Ok(None) => return err(StatusCode::NOT_FOUND, "NOT_FOUND", "Event not found").into_response(),
        Ok(Some(evt)) => evt,
    };

    let files: Vec<serde_json::Value> = match serde_json::from_str(&event.files_json) {
        Err(_) => return err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "Invalid files payload").into_response(),
        Ok(f) => f,
    };
    let work_dir = match files.first().and_then(|f| f.get("path")).and_then(|p| p.as_str()) {
        Some(path) => path.to_string(),
        None => return err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "No file path in event").into_response(),
    };

    let workflows_path = state.base_dir.join("workflows.json");
    let data = match tokio::fs::read_to_string(&workflows_path).await {
        Err(_) => return err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "Failed to read workflows").into_response(),
        Ok(d) => d,
    };
    let workflows: Vec<Workflow> = match serde_json::from_str(&data) {
        Err(_) => return err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", "Invalid workflows data").into_response(),
        Ok(w) => w,
    };

    let workflow = match workflows.iter().find(|wf| wf.id == run.workflow_id) {
        Some(wf) => wf.clone(),
        None => return err(StatusCode::NOT_FOUND, "NOT_FOUND", "Workflow not found").into_response(),
    };

    if let Err(e) = state.db.update_run_status(&id, "running", None, None, None) {
        return err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response();
    }

    let registry = draction_engine::default_registry();
    let engine = WorkflowEngine::new(registry);

    match engine.execute(&id, &run.event_id, &workflow, &work_dir).await {
        Ok(()) => {
            let now = Utc::now().to_rfc3339();
            if let Err(e) = state.db.update_run_status(&id, "completed", Some(&now), None, Some("[]")) {
                return err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response();
            }
        }
        Err(error) => {
            let error_payload = json!({
                "code": "WORKFLOW_ERROR",
                "message": error.to_string(),
                "retryable": true,
            });
            let now = Utc::now().to_rfc3339();
            if let Err(e) = state.db.update_run_status(
                &id,
                "failed",
                Some(&now),
                Some(&error_payload.to_string()),
                Some("[]"),
            ) {
                return err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response();
            }
        }
    }

    match state.db.get_run(&id) {
        Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
        Ok(Some(run)) => (StatusCode::OK, Json(json!(run))).into_response(),
        Ok(None) => err(StatusCode::NOT_FOUND, "NOT_FOUND", "Run not found after retry").into_response(),
    }
}
