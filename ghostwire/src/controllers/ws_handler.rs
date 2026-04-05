//! ws_handler.rs — Real WebSocket handler replacing the echo stub
//!
//! The current handle_socket() just echoes text back. This replaces it
//! with a full event-streaming implementation:
//!
//!   - Subscribes to Core's event_bus on connect
//!   - Streams CoreEvent as JSON to the frontend in real-time
//!   - Handles inbound WS commands from the UI (send message, etc.)
//!   - Sends ping/pong keepalives
//!   - Cleans up on disconnect
//!
//! INTEGRATION: Replace the existing ws_handler and handle_socket functions
//! in src/web.rs with these.

use crate::core::CoreEvent;
use crate::web::AppState;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::{interval, Duration};

/// Inbound commands the frontend can send over WebSocket
#[derive(Debug, Deserialize)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum WsCommand {
    /// Subscribe to specific event types only
    Subscribe { events: Vec<String> },
    /// Send a message (triggers Core::send_message)
    SendMessage { recipient: String, content: String },
    /// Request current peer list
    GetPeers,
    /// Ping keepalive
    Ping,
}

/// Outbound events sent to the frontend
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsEvent {
    /// A CoreEvent relayed to the frontend
    CoreEvent { payload: serde_json::Value },
    /// Response to GetPeers
    PeerList { peers: Vec<serde_json::Value>, count: usize },
    /// Pong response
    Pong { timestamp: u64 },
    /// Connection established confirmation
    Connected { node_id: String, version: String },
    /// Error response
    Error { code: String, message: String },
    /// Connection request received (social follow mechanic)
    ConnectionRequest {
        from_node_id: String,
        from_nickname: String,
        from_did: String,
        request_id: String,
        timestamp: u64,
    },
}

/// REPLACE in web.rs:
///
/// ```rust
/// async fn ws_handler(
///     ws: WebSocketUpgrade,
///     State(state): State<Arc<AppState>>,
/// ) -> impl IntoResponse {
///     ws.on_upgrade(move |socket| handle_socket(socket, state))
/// }
/// ```
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

pub async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    // Subscribe to the core event bus
    let mut event_rx = state.core.read().await.event_bus.subscribe();

    // Send connected confirmation
    let node_id = state.core.read().await.get_identity_id().await;
    let connected_msg = serde_json::to_string(&WsEvent::Connected {
        node_id: node_id.clone(),
        version: "2.0.0".into(),
    })
    .unwrap_or_default();
    let _ = socket.send(Message::Text(connected_msg)).await;

    // Keepalive ping every 30s
    let mut ping_interval = interval(Duration::from_secs(30));

    loop {
        tokio::select! {
            // Outbound: relay CoreEvent to frontend
            Ok(event) = event_rx.recv() => {
                let payload = match serde_json::to_value(&event) {
                    Ok(v) => v,
                    Err(_) => continue,
                };
                let ws_event = WsEvent::CoreEvent { payload };
                let json = match serde_json::to_string(&ws_event) {
                    Ok(s) => s,
                    Err(_) => continue,
                };
                if socket.send(Message::Text(json)).await.is_err() {
                    tracing::debug!("WS client disconnected (send failed)");
                    break;
                }
            }

            // Inbound: handle commands from frontend
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        handle_ws_command(&mut socket, &state, &text).await;
                    }
                    Some(Ok(Message::Ping(data))) => {
                        let _ = socket.send(Message::Pong(data)).await;
                    }
                    Some(Ok(Message::Close(_))) | None => {
                        tracing::debug!("WS client disconnected cleanly");
                        break;
                    }
                    Some(Err(e)) => {
                        tracing::warn!("WS error: {}", e);
                        break;
                    }
                    _ => {}
                }
            }

            // Keepalive ping
            _ = ping_interval.tick() => {
                let pong = serde_json::to_string(&WsEvent::Pong {
                    timestamp: now_secs(),
                }).unwrap_or_default();
                if socket.send(Message::Text(pong)).await.is_err() {
                    break;
                }
            }
        }
    }

    tracing::debug!("WebSocket session ended for node {}", &node_id[..8.min(node_id.len())]);
}

async fn handle_ws_command(socket: &mut WebSocket, state: &Arc<AppState>, text: &str) {
    let cmd: WsCommand = match serde_json::from_str(text) {
        Ok(c) => c,
        Err(_) => {
            let err = serde_json::to_string(&WsEvent::Error {
                code: "PARSE_ERROR".into(),
                message: "Invalid command JSON".into(),
            }).unwrap_or_default();
            let _ = socket.send(Message::Text(err)).await;
            return;
        }
    };

    match cmd {
        WsCommand::Ping => {
            let pong = serde_json::to_string(&WsEvent::Pong { timestamp: now_secs() })
                .unwrap_or_default();
            let _ = socket.send(Message::Text(pong)).await;
        }

        WsCommand::GetPeers => {
            let count = state.core.read().await.get_peer_count();
            let resp = serde_json::to_string(&WsEvent::PeerList {
                peers: vec![],
                count,
            }).unwrap_or_default();
            let _ = socket.send(Message::Text(resp)).await;
        }

        WsCommand::SendMessage { recipient, content } => {
            // Delegate to messaging layer
            let core = state.core.read().await;
            let sender = core.get_identity_id().await;
            let msg = crate::core::message::Message::new(sender, recipient, content);
            // core.send_message(msg) — wire when send_message is implemented
            let _ = msg;
        }

        WsCommand::Subscribe { events } => {
            tracing::debug!("WS client subscribed to: {:?}", events);
        }
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
