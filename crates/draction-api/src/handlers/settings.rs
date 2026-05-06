use axum::{Json, extract::State, http::StatusCode, response::IntoResponse};
use draction_domain::settings::Settings;
use serde_json::json;

use crate::state::AppState;

fn err(status: StatusCode, code: &str, message: &str) -> (StatusCode, Json<serde_json::Value>) {
    (
        status,
        Json(json!({ "error": { "code": code, "message": message } })),
    )
}

pub async fn get_settings(State(state): State<AppState>) -> impl IntoResponse {
    let settings = Settings::load(&state.base_dir).await.unwrap_or_default();
    (
        StatusCode::OK,
        Json(serde_json::to_value(settings).unwrap()),
    )
        .into_response()
}

pub async fn update_settings(
    State(state): State<AppState>,
    Json(body): Json<serde_json::Value>,
) -> impl IntoResponse {
    let mut settings = Settings::load(&state.base_dir).await.unwrap_or_default();

    // Partial merge: only update fields present in the request
    if let Some(incoming_obj) = body.as_object() {
        let mut current = serde_json::to_value(&settings).unwrap();
        if let Some(current_obj) = current.as_object_mut() {
            for (key, value) in incoming_obj {
                current_obj.insert(key.clone(), value.clone());
            }
            if let Ok(merged) = serde_json::from_value::<Settings>(current) {
                settings = merged;
            }
        }
    }

    // Validate
    if settings.undo_window_seconds == 0 {
        return err(
            StatusCode::UNPROCESSABLE_ENTITY,
            "VALIDATION_ERROR",
            "undo_window_seconds must be positive",
        )
        .into_response();
    }
    if settings.undo_history_depth == 0 || settings.undo_history_depth > 50 {
        return err(
            StatusCode::UNPROCESSABLE_ENTITY,
            "VALIDATION_ERROR",
            "undo_history_depth must be between 1 and 50",
        )
        .into_response();
    }
    if settings.concurrency == 0 || settings.concurrency > 8 {
        return err(
            StatusCode::UNPROCESSABLE_ENTITY,
            "VALIDATION_ERROR",
            "concurrency must be between 1 and 8",
        )
        .into_response();
    }
    if settings.inbox_size_limit_gb == 0 || settings.inbox_size_limit_gb > 1024 {
        return err(
            StatusCode::UNPROCESSABLE_ENTITY,
            "VALIDATION_ERROR",
            "inbox_size_limit_gb must be between 1 and 1024",
        )
        .into_response();
    }
    if settings.max_file_size_mb == 0 {
        return err(
            StatusCode::UNPROCESSABLE_ENTITY,
            "VALIDATION_ERROR",
            "max_file_size_mb must be positive",
        )
        .into_response();
    }
    let valid_themes = ["system", "light", "dark"];
    if !valid_themes.contains(&settings.theme.as_str()) {
        return err(
            StatusCode::UNPROCESSABLE_ENTITY,
            "VALIDATION_ERROR",
            "theme must be one of: system, light, dark",
        )
        .into_response();
    }
    let valid_conflicts = ["append_timestamp", "rename", "overwrite", "skip"];
    if !valid_conflicts.contains(&settings.conflict_resolution.as_str()) {
        return err(
            StatusCode::UNPROCESSABLE_ENTITY,
            "VALIDATION_ERROR",
            "conflict_resolution is invalid",
        )
        .into_response();
    }
    let valid_match_policies = ["first_match", "first_wins", "all_matches", "all_matching"];
    if !valid_match_policies.contains(&settings.match_policy.as_str()) {
        return err(
            StatusCode::UNPROCESSABLE_ENTITY,
            "VALIDATION_ERROR",
            "match_policy is invalid",
        )
        .into_response();
    }
    let valid_log_levels = ["trace", "debug", "info", "warn", "error"];
    if !valid_log_levels.contains(&settings.log_level.as_str()) {
        return err(
            StatusCode::UNPROCESSABLE_ENTITY,
            "VALIDATION_ERROR",
            "log_level is invalid",
        )
        .into_response();
    }

    if let Err(e) = settings.save(&state.base_dir).await {
        return err(
            StatusCode::INTERNAL_SERVER_ERROR,
            "INTERNAL_ERROR",
            &e.to_string(),
        )
        .into_response();
    }

    (
        StatusCode::OK,
        Json(serde_json::to_value(settings).unwrap()),
    )
        .into_response()
}

pub async fn get_about() -> impl IntoResponse {
    (
        StatusCode::OK,
        Json(json!({
            "version": env!("CARGO_PKG_VERSION"),
            "name": "Draction",
            "build": "dev"
        })),
    )
        .into_response()
}
