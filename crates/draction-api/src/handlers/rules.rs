use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use draction_domain::rule::Rule;
use serde_json::json;

use crate::state::AppState;

fn err(status: StatusCode, code: &str, message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (status, Json(json!({ "error": { "code": code, "message": message } })))
}

fn rules_path(state: &AppState) -> std::path::PathBuf {
    state.base_dir.join("rules.json")
}

fn read_rules(state: &AppState) -> anyhow::Result<Vec<Rule>> {
    let path = rules_path(state);
    if !path.exists() {
        return Ok(vec![]);
    }
    let content = std::fs::read_to_string(&path)?;
    Ok(serde_json::from_str(&content)?)
}

fn write_rules(state: &AppState, rules: &[Rule]) -> anyhow::Result<()> {
    std::fs::write(rules_path(state), serde_json::to_string_pretty(rules)?)?;
    Ok(())
}

pub async fn list(State(state): State<AppState>) -> impl IntoResponse {
    match read_rules(&state) {
        Ok(rules) => (StatusCode::OK, Json(json!(rules))).into_response(),
        Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
    }
}

pub async fn get_one(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    match read_rules(&state) {
        Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
        Ok(rules) => match rules.into_iter().find(|r| r.id == id) {
            Some(rule) => (StatusCode::OK, Json(json!(rule))).into_response(),
            None => err(StatusCode::NOT_FOUND, "NOT_FOUND", "Rule not found").into_response(),
        },
    }
}

pub async fn create(
    State(state): State<AppState>,
    Json(body): Json<Rule>,
) -> impl IntoResponse {
    let mut rules = match read_rules(&state) {
        Ok(r) => r,
        Err(e) => return err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
    };
    if rules.iter().any(|r| r.id == body.id) {
        return err(StatusCode::CONFLICT, "CONFLICT", "Rule with this id already exists").into_response();
    }
    rules.push(body.clone());
    match write_rules(&state, &rules) {
        Ok(_) => (StatusCode::CREATED, Json(json!(body))).into_response(),
        Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
    }
}

pub async fn update(
    Path(id): Path<String>,
    State(state): State<AppState>,
    Json(body): Json<Rule>,
) -> impl IntoResponse {
    let mut rules = match read_rules(&state) {
        Ok(r) => r,
        Err(e) => return err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
    };
    let pos = match rules.iter().position(|r| r.id == id) {
        Some(p) => p,
        None => return err(StatusCode::NOT_FOUND, "NOT_FOUND", "Rule not found").into_response(),
    };
    rules[pos] = body.clone();
    match write_rules(&state, &rules) {
        Ok(_) => (StatusCode::OK, Json(json!(body))).into_response(),
        Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
    }
}

pub async fn remove(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let mut rules = match read_rules(&state) {
        Ok(r) => r,
        Err(e) => return err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
    };
    let len_before = rules.len();
    rules.retain(|r| r.id != id);
    if rules.len() == len_before {
        return err(StatusCode::NOT_FOUND, "NOT_FOUND", "Rule not found").into_response();
    }
    match write_rules(&state, &rules) {
        Ok(_) => (StatusCode::OK, Json(json!({ "ok": true }))).into_response(),
        Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
    }
}

pub async fn toggle_enabled(
    Path(id): Path<String>,
    State(state): State<AppState>,
) -> impl IntoResponse {
    let mut rules = match read_rules(&state) {
        Ok(r) => r,
        Err(e) => return err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
    };
    let pos = match rules.iter().position(|r| r.id == id) {
        Some(p) => p,
        None => return err(StatusCode::NOT_FOUND, "NOT_FOUND", "Rule not found").into_response(),
    };
    rules[pos].enabled = !rules[pos].enabled;
    let updated = rules[pos].clone();
    match write_rules(&state, &rules) {
        Ok(_) => (StatusCode::OK, Json(json!(updated))).into_response(),
        Err(e) => err(StatusCode::INTERNAL_SERVER_ERROR, "INTERNAL_ERROR", &e.to_string()).into_response(),
    }
}
