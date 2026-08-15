//! Authenticated local WebSocket event and command handler.
//!
//! The WebSocket route is protected by the daemon authentication middleware.
//! It intentionally relies on the authenticated local transport (or a future
//! externally terminated TLS deployment) rather than advertising an ad-hoc
//! application-layer encryption scheme with no key agreement or client support.

use crate::web::AppState;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use serde::{Deserialize, Serialize};
use std::sync::atomic::Ordering;
use std::sync::Arc;
use tokio::time::{interval, Duration};

/// Maximum plaintext command content accepted at the WebSocket boundary.
/// Transport-specific encrypted-frame limits remain enforced lower in the
/// stack; this early bound avoids passing obviously oversized user input into
/// routing or storage.
const MAX_WS_MESSAGE_BYTES: usize = 10 * 1024;
const MAX_RECIPIENT_BYTES: usize = 256;
const PRIVATE_DELIVERY_UNAVAILABLE_MARKER: &str = "PRIVATE_DELIVERY_UNAVAILABLE";

/// Inbound commands the frontend can send over WebSocket.
#[derive(Debug, Deserialize)]
#[serde(tag = "cmd", rename_all = "snake_case")]
pub enum WsCommand {
    /// Subscribe to specific event types only. Event filtering remains a
    /// future capability; current subscriptions receive the authenticated local
    /// event stream.
    Subscribe { events: Vec<String> },
    /// Dispatch a message through the same core command path used by HTTP.
    SendMessage { recipient: String, content: String },
    /// Request current peer count.
    GetPeers,
    /// Ping keepalive.
    Ping,
}

/// Outbound events sent to an authenticated local frontend.
#[derive(Debug, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum WsEvent {
    /// A CoreEvent relayed to the frontend. All CoreEvent serialization must
    /// remain privacy-reviewed before remote exposure is ever permitted.
    CoreEvent { payload: serde_json::Value },
    /// Response to `GetPeers`.
    PeerList {
        peers: Vec<serde_json::Value>,
        count: usize,
    },
    /// A message accepted by the local dispatcher. This is not recipient
    /// acknowledgement or final delivery.
    MessageDispatched {
        message_id: String,
        delivery_class: String,
    },
    /// Pong response.
    Pong { timestamp: u64 },
    /// Connection-established confirmation.
    Connected { node_id: String, version: String },
    /// Public-safe command error. Do not include raw source errors, routes,
    /// peers, secrets, or message content.
    Error { code: String, message: String },
}

/// Upgrade an already authenticated WebSocket request.
pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<Arc<AppState>>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state))
}

pub async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    state.ws_count.fetch_add(1, Ordering::Relaxed);

    // Clone a stable core snapshot before awaiting identity/event operations so
    // the WebSocket does not retain the application-core outer lock.
    let core = state.core.read().await.clone();
    let mut event_rx = core.event_bus.subscribe();
    let node_id = core.get_identity_id().await;

    if send_ws_event(
        &mut socket,
        &WsEvent::Connected {
            node_id: node_id.clone(),
            version: "2.0.0".into(),
        },
    )
    .await
    .is_err()
    {
        state.ws_count.fetch_sub(1, Ordering::Relaxed);
        return;
    }

    let mut ping_interval = interval(Duration::from_secs(30));

    loop {
        tokio::select! {
            Ok(event) = event_rx.recv() => {
                let payload = match serde_json::to_value(&event) {
                    Ok(value) => value,
                    Err(_) => continue,
                };
                if send_ws_event(&mut socket, &WsEvent::CoreEvent { payload }).await.is_err() {
                    tracing::debug!("Authenticated WebSocket client disconnected while receiving an event");
                    break;
                }
            }

            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        if !handle_ws_command(&mut socket, &state, &text).await {
                            break;
                        }
                    }
                    Some(Ok(Message::Ping(data))) => {
                        if socket.send(Message::Pong(data)).await.is_err() {
                            break;
                        }
                    }
                    Some(Ok(Message::Close(_))) | None => {
                        tracing::debug!("Authenticated WebSocket client disconnected cleanly");
                        break;
                    }
                    Some(Err(error)) => {
                        tracing::warn!(error = %error, "WebSocket receive error");
                        break;
                    }
                    _ => {}
                }
            }

            _ = ping_interval.tick() => {
                if send_ws_event(&mut socket, &WsEvent::Pong { timestamp: now_secs() }).await.is_err() {
                    break;
                }
            }
        }
    }

    state.ws_count.fetch_sub(1, Ordering::Relaxed);
    tracing::debug!(
        node = %short_node_id(&node_id),
        "Authenticated WebSocket session ended"
    );
}

async fn handle_ws_command(socket: &mut WebSocket, state: &Arc<AppState>, text: &str) -> bool {
    let command: WsCommand = match serde_json::from_str(text) {
        Ok(command) => command,
        Err(_) => {
            return send_ws_event(
                socket,
                &WsEvent::Error {
                    code: "FRAME_MALFORMED".into(),
                    message: "Invalid command JSON".into(),
                },
            )
            .await
            .is_ok();
        }
    };

    match command {
        WsCommand::Ping => send_ws_event(
            socket,
            &WsEvent::Pong {
                timestamp: now_secs(),
            },
        )
        .await
        .is_ok(),

        WsCommand::GetPeers => {
            let core = state.core.read().await.clone();
            send_ws_event(
                socket,
                &WsEvent::PeerList {
                    peers: Vec::new(),
                    count: core.get_peer_count(),
                },
            )
            .await
            .is_ok()
        }

        WsCommand::SendMessage { recipient, content } => {
            if content.len() > MAX_WS_MESSAGE_BYTES {
                return send_ws_event(
                    socket,
                    &WsEvent::Error {
                        code: "FRAME_SIZE_LIMIT".into(),
                        message: "Message exceeds the permitted size".into(),
                    },
                )
                .await
                .is_ok();
            }
            if content.trim().is_empty() {
                return send_ws_event(
                    socket,
                    &WsEvent::Error {
                        code: "INVALID_INPUT".into(),
                        message: "Message content cannot be empty".into(),
                    },
                )
                .await
                .is_ok();
            }
            if recipient.len() > MAX_RECIPIENT_BYTES {
                return send_ws_event(
                    socket,
                    &WsEvent::Error {
                        code: "INVALID_INPUT".into(),
                        message: "Recipient identifier is too long".into(),
                    },
                )
                .await
                .is_ok();
            }

            let core = state.core.read().await.clone();
            let sender = core.get_identity_id().await;
            let is_broadcast = recipient.is_empty() || recipient == "broadcast";
            let message = crate::core::message::Message::new(
                sender,
                if is_broadcast {
                    "broadcast".to_string()
                } else {
                    recipient.clone()
                },
                content,
            );

            let dispatch = if is_broadcast {
                core.send_message(&message).await
            } else {
                core.send_direct_message(recipient, &message).await
            };

            match dispatch {
                Ok(()) => send_ws_event(
                    socket,
                    &WsEvent::MessageDispatched {
                        message_id: message.id.to_string(),
                        delivery_class: if is_broadcast {
                            "broadcast".into()
                        } else {
                            "direct".into()
                        },
                    },
                )
                .await
                .is_ok(),
                Err(error) => {
                    tracing::warn!(error = %error, message_id = %message.id, "WebSocket message dispatch failed");
                    let (code, message) = if error
                        .to_string()
                        .contains(PRIVATE_DELIVERY_UNAVAILABLE_MARKER)
                    {
                        (
                            "PRIVATE_DELIVERY_UNAVAILABLE",
                            "Private delivery is unavailable because no privacy-preserving route is ready",
                        )
                    } else {
                        ("TRANSPORT_UNREACHABLE", "Message could not be dispatched")
                    };
                    send_ws_event(
                        socket,
                        &WsEvent::Error {
                            code: code.into(),
                            message: message.into(),
                        },
                    )
                    .await
                    .is_ok()
                }
            }
        }

        WsCommand::Subscribe { events } => {
            // Preserve the current API shape while filtering is designed against
            // a stable, privacy-reviewed event schema.
            tracing::debug!(
                requested_event_types = events.len(),
                "WebSocket event subscription requested"
            );
            true
        }
    }
}

async fn send_ws_event(socket: &mut WebSocket, event: &WsEvent) -> Result<(), ()> {
    let json = serde_json::to_string(event).map_err(|_| ())?;
    socket
        .send(Message::Text(json.into()))
        .await
        .map_err(|_| ())
}

fn short_node_id(node_id: &str) -> &str {
    &node_id[..node_id.len().min(8)]
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
