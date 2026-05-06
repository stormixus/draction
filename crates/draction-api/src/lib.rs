pub mod auth;
pub mod router;
pub mod handlers;
pub mod middleware;
pub mod state;
pub mod ws;

use anyhow::Result;
use axum::http::{header, Method};
use axum::routing::get;
use axum::Router;
use state::AppState;
use tower_http::cors::{Any, CorsLayer};

pub async fn start_server(port: u16, app_state: AppState) -> Result<u16> {
    for attempt in 0..10u16 {
        let try_port = port + attempt;
        let addr = std::net::SocketAddr::from(([127, 0, 0, 1], try_port));

        let listener = match tokio::net::TcpListener::bind(addr).await {
            Ok(l) => l,
            Err(_) => {
                tracing::warn!("Port {} in use, trying next", try_port);
                continue;
            }
        };

        let cors = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods([
                Method::GET,
                Method::POST,
                Method::PUT,
                Method::PATCH,
                Method::DELETE,
                Method::OPTIONS,
            ])
            .allow_headers([header::AUTHORIZATION, header::CONTENT_TYPE]);

        let app = Router::new()
            .route("/api/v1/health", get(health))
            .merge(router::build_router(app_state.clone()))
            .layer(cors);

        tracing::info!("API server listening on {}", addr);

        tokio::spawn(async move {
            axum::serve(listener, app)
                .await
                .expect("API server error");
        });

        return Ok(try_port);
    }

    anyhow::bail!("Could not bind to any port in range {}..{}", port, port + 9)
}

async fn health() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({ "status": "ok" }))
}
