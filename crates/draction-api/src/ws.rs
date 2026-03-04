use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::Response,
};
use draction_events::EventBus;
use std::sync::Arc;

use crate::state::AppState;

pub async fn upgrade(ws: WebSocketUpgrade, State(state): State<AppState>) -> Response {
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
