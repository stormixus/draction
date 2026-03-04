use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use draction_domain::workflow::Workflow;
use serde_json::json;

use crate::state::AppState;

fn err(status: StatusCode, code: &str, message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (status, Json(json!({ "error": { "code": code, "message": message } })))
}

fn workflows_path(state: &AppState) -> std::path::PathBuf {
    state.base_dir.join("workflows.json")
}

fn read_workflows(state: &AppState) -> anyhow::Result<Vec<Workflow>> {
    let path = workflows_path(state);
    if !path.exists() {
        return Ok(vec![]);
    }
    let content = std::fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&content)?)
}

fn write_workflows(state: &AppState, workflows: &[Workflow]) -> anyhow::Result<()> {
    std::fs::write(workflows_path(state), serde_json::to_string_pretty(workflows)?)?;
    Ok(())
}

pub async fn list(State(state): State<AppState>) -> impl IntoResponse {
    match read_workflows(&state) {
        Ok(wf) => (StatusCode::OK, Json(json!(wf))).into_response(),
        Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
    }
}

pub async fn get_one(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match read_workflows(&state) {
        Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
        Ok(wfs) => match wfs.into_iter().find(|w| w.id == id) {
            Some(wf) => (StatusCode::OK, Json(json!(wf))).into_response(),
            None => err(StatusCode::NOT_FOUND, "NOT_FOUND", "Workflow not found").into_response(),
        },
    }
}

pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<Workflow>,
) -> impl IntoResponse {
    let mut wfs = match read_workflows(&state) {
        Ok(w) => w,
        Err(e) => return err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
    };
    if wfs.iter().any(|w| w.id == body.id) {
        return err(StatusCode::CONFLICT, "CONFLICT", "Workflow with this id already exists").into_response();
    }
    wfs.push(body.clone());
    match write_workflows(&state, &wfs) {
        Ok(_) => (StatusCode::CREATED, Json(json!(body))).into_response(),
        Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
    }
}

pub async fn update(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(body): Json<Workflow>,
) -> impl IntoResponse {
    let mut wfs = match read_workflows(&state) {
        Ok(w) => w,
        Err(e) => return err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
    };
    let pos = match wfs.iter().position(|w| w.id == id) {
        Some(p) => p,
        None => return err(StatusCode::NOT_FOUND, "NOT_FOUND", "Workflow not found").into_response(),
    };
    wfs[pos] = body.clone();
    match write_workflows(&state, &wfs) {
        Ok(_) => (StatusCode::OK, Json(json!(body))).into_response(),
        Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
    }
}
