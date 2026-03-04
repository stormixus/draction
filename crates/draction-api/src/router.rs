use axum::routing::{get, patch, post};
use axum::Router;

use crate::state::AppState;

pub fn build_router(state: AppState) -> Router {
    Router::new()
        // Rules
        .route("/api/v1/rules", get(crate::handlers::rules::list).post(crate::handlers::rules::create))
        .route("/api/v1/rules/{id}", get(crate::handlers::rules::get_one).put(crate::handlers::rules::update).delete(crate::handlers::rules::remove))
        .route("/api/v1/rules/{id}/enabled", patch(crate::handlers::rules::toggle_enabled))
        // Workflows
        .route("/api/v1/workflows", get(crate::handlers::workflows::list).post(crate::handlers::workflows::create))
        .route("/api/v1/workflows/{id}", get(crate::handlers::workflows::get_one).put(crate::handlers::workflows::update))
        // Runs
        .route("/api/v1/runs", get(crate::handlers::runs::list))
        .route("/api/v1/runs/{id}", get(crate::handlers::runs::get_one))
        .route("/api/v1/runs/{id}/retry", post(crate::handlers::runs::retry))
        // Events
        .route("/api/v1/events", get(crate::handlers::events::list))
        .route("/api/v1/events/{event_id}/undo", post(crate::handlers::events::undo))
        // WebSocket
        .route("/ws", get(crate::ws::upgrade))
        .with_state(state)
}
