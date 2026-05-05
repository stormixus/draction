use axum::middleware;
use axum::routing::{get, patch, post};
use axum::Router;

use crate::state::AppState;

pub fn build_router(state: AppState) -> Router {
    let api_routes = Router::new()
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
        // Settings
        .route("/api/v1/settings", get(crate::handlers::settings::get_settings).put(crate::handlers::settings::update_settings))
        // Watcher
        .route("/api/v1/watcher/start", post(crate::handlers::watcher::start_watcher))
        .route("/api/v1/watcher/stop", post(crate::handlers::watcher::stop_watcher))
        .route_layer(middleware::from_fn_with_state(
            state.clone(),
            crate::middleware::auth::bearer_auth,
        ));

    Router::new()
        // WebSocket (auth checked via query param in handler)
        .route("/ws", get(crate::ws::upgrade))
        // About (no auth needed)
        .route("/api/v1/settings/about", get(crate::handlers::settings::get_about))
        .merge(api_routes)
        .with_state(state)
}
