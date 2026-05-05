use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use draction_events::EventBus;
use serde::Deserialize;
use std::sync::Arc;

use crate::state::AppState;

#[derive(Deserialize)]
pub struct WsParams {
    token: Option<String>,
}

pub async fn upgrade(
    ws: WebSocketUpgrade,
    Query(params): Query<WsParams>,
    State(state): State<AppState>,
) -> Response {
    let token = params.token.unwrap_or_default();
    if token != state.auth_token {
        return (
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": {
                    "code": "UNAUTHORIZED",
                    "message": "Missing or invalid token"
                }
            })),
        )
            .into_response();
    }

    ws.on_upgrade(move |socket| handle_socket(socket, state.event_bus))
}

async fn handle_socket(mut socket: WebSocket, event_bus: Arc<EventBus>) {
    let mut rx = event_bus.subscribe();

    loop {
        tokio::select! {
            result = rx.recv() => {
                match result {
                    Ok(envelope) => {
                        let json = match serde_json::to_string(&envelope) {
                            Ok(s) => s,
                            Err(e) => {
                                tracing::warn!("Failed to serialize envelope: {}", e);
                                continue;
                            }
                        };
                        if socket.send(Message::Text(json.into())).await.is_err() {
                            // Client disconnected
                            break;
                        }
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Lagged(n)) => {
                        tracing::warn!("WS client lagged, skipped {} messages", n);
                    }
                    Err(tokio::sync::broadcast::error::RecvError::Closed) => {
                        break;
                    }
                }
            }
            msg = socket.recv() => {
                match msg {
                    Some(Ok(_)) => {
                        // Ignore client messages (read-only stream)
                    }
                    Some(Err(_)) | None => {
                        // Client disconnected
                        break;
                    }
                }
            }
        }
    }

    tracing::debug!("WS client disconnected");
}
