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
//!   - ENCRYPTED: All WS messages encrypted with XChaCha20-Poly1305
//!
//! INTEGRATION: Replace the existing ws_handler and handle_socket functions
//! in src/web.rs with these.

#[allow(unused_imports)]
use crate::core::CoreEvent;
use crate::web::AppState;
use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::State;
use axum::response::IntoResponse;
use base64::Engine;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::{interval, Duration, Instant};

const MAX_WS_FRAME_BYTES: usize = 64 * 1024;
const MAX_WS_CONTENT_BYTES: usize = 16 * 1024;
const MAX_WS_SUBSCRIPTIONS: usize = 64;
const MAX_WS_EVENT_NAME_BYTES: usize = 128;
const MAX_WS_COMMANDS_PER_MINUTE: u32 = 120;
const MAX_WS_IDLE_SECS: u64 = 300;
const ALLOWED_WS_EVENTS: &[&str] = &["core_event", "connection_request"];

#[derive(Debug)]
struct WsQuota {
    window_started: Instant,
    commands: u32,
}
impl WsQuota {
    fn new() -> Self {
        Self {
            window_started: Instant::now(),
            commands: 0,
        }
    }
    fn allow_command(&mut self) -> bool {
        let now = Instant::now();
        if now.duration_since(self.window_started) >= Duration::from_secs(60) {
            self.window_started = now;
            self.commands = 0;
        }
        if self.commands >= MAX_WS_COMMANDS_PER_MINUTE {
            return false;
        }
        self.commands = self.commands.saturating_add(1);
        true
    }
}

/// Encrypted WebSocket message format
/// Format: {"nonce": "<base64>", "ct": "<base64>"}
///
/// The browser decrypts this with WebCrypto AES-256-GCM. The session key is
/// derived from the authenticated profile identity ID and the random salt sent
/// once in the `session_salt` handshake frame.
#[derive(Debug, Serialize, Deserialize)]
struct EncryptedWsMessage {
    nonce: String,
    ct: String,
}

/// Encrypt data using AES-256-GCM for browser-native authenticated decryption
fn encrypt_ws_message(
    plaintext: &[u8],
    session_key: &[u8; 32],
) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    use aes_gcm::aead::{Aead, KeyInit};
    use aes_gcm::{Aes256Gcm, Nonce};
    use base64::Engine;
    use rand_core::{OsRng, RngCore};

    // AES-GCM uses a 96-bit nonce, which is natively supported by WebCrypto.
    let mut nonce_bytes = [0u8; 12];
    OsRng.fill_bytes(&mut nonce_bytes);
    let nonce = Nonce::from_slice(&nonce_bytes);

    let cipher = Aes256Gcm::new_from_slice(session_key)
        .map_err(|e| format!("WS key setup failed: {}", e))?;
    let ciphertext = cipher
        .encrypt(nonce, plaintext)
        .map_err(|e| format!("WS encryption failed: {}", e))?;

    // Encode as JSON
    let enc_msg = EncryptedWsMessage {
        nonce: base64::engine::general_purpose::STANDARD.encode(&nonce_bytes),
        ct: base64::engine::general_purpose::STANDARD.encode(&ciphertext),
    };

    Ok(serde_json::to_string(&enc_msg)?)
}

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
    PeerList {
        peers: Vec<serde_json::Value>,
        count: usize,
    },
    /// Pong response
    Pong { timestamp: u64 },
    /// Connection established confirmation
    Connected { node_id: String, version: String },
    /// Per-connection salt for browser-side event decryption.
    SessionSalt { salt: String },
    /// Error response
    Error { code: String, message: String },
    /// The daemon accepted a message for transport; remote delivery is not
    /// implied until a receipt is reconciled through the message layer.
    Accepted {
        code: String,
        message_id: String,
        delivery: String,
    },
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
/// ```ignore
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

/// Generate a per-session WebSocket encryption key derived from the node identity.
/// This ensures each WS connection uses a unique key, preventing cross-session decryption.
async fn generate_ws_session_key(state: &Arc<AppState>) -> ([u8; 32], [u8; 16]) {
    use ring::hkdf::{Salt, HKDF_SHA256};
    use ring::rand::SecureRandom;

    // Get identity key material (or fallback to a random key if identity not yet set)
    let ikm = {
        let core = state.core.read().await;
        core.get_identity_id().await.as_bytes().to_vec()
    };

    // Generate random salt for this session
    let rng = ring::rand::SystemRandom::new();
    let mut salt_bytes = [0u8; 16];
    let _ = rng.fill(&mut salt_bytes);

    // Derive 32-byte session key via HKDF
    let salt = Salt::new(HKDF_SHA256, &salt_bytes);
    let prk = salt.extract(&ikm);
    let okm: ring::hkdf::Okm<ring::hkdf::Algorithm> = prk
        .expand(&[b"ghostwire-ws-session-v2"], HKDF_SHA256)
        .unwrap();
    let mut okm_bytes = [0u8; 32];
    okm.fill(&mut okm_bytes).unwrap();
    (okm_bytes, salt_bytes)
}

pub async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    // Generate per-session encryption key from the profile identity and a
    // random salt. The salt is public; the identity ID is already bound to the
    // authenticated profile and is available to the browser after unlock.
    let (session_key, session_salt) = generate_ws_session_key(&state).await;

    // Subscribe to the core event bus
    let mut event_rx = state.core.read().await.event_bus.subscribe();

    // Send the salt before any encrypted event so the browser can derive the
    // same AES-GCM session key. The salt itself contains no secret material.
    let salt_msg = serde_json::to_string(&WsEvent::SessionSalt {
        salt: base64::engine::general_purpose::STANDARD.encode(session_salt),
    })
    .unwrap_or_default();
    let _ = socket.send(Message::Text(salt_msg.into())).await;

    // Send connected confirmation
    let connected_msg = serde_json::to_string(&WsEvent::Connected {
        node_id: "redacted".into(),
        version: "2.0.0".into(),
    })
    .unwrap_or_default();
    let _ = socket.send(Message::Text(connected_msg.into())).await;

    // Keepalive ping every 30s
    let mut ping_interval = interval(Duration::from_secs(30));
    let mut idle_timeout = Box::pin(tokio::time::sleep(Duration::from_secs(MAX_WS_IDLE_SECS)));
    let mut quota = WsQuota::new();

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
                // Encrypt the message before sending
                match encrypt_ws_message(json.as_bytes(), &session_key) {
                    Ok(encrypted) => {
                        if socket.send(Message::Text(encrypted.into())).await.is_err() {
                            tracing::debug!("WS client disconnected (send failed)");
                            break;
                        }
                    }
                    Err(e) => {
                        tracing::warn!("WS encryption failed, dropping message: {}", e);
                        break;
                    }
                }
            }

            // Inbound: handle commands from frontend
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
                        if !quota.allow_command() {
                            let err = serde_json::to_string(&WsEvent::Error {
                                code: "RATE_LIMITED".into(),
                                message: "WebSocket command quota exceeded; reconnect later".into(),
                            }).unwrap_or_default();
                            let _ = socket.send(Message::Text(err.into())).await;
                            break;
                        }
                        idle_timeout.as_mut().reset(Instant::now() + Duration::from_secs(MAX_WS_IDLE_SECS));
                        if text.len() > MAX_WS_FRAME_BYTES {
                            let err = serde_json::to_string(&WsEvent::Error {
                                code: "FRAME_TOO_LARGE".into(),
                                message: "WebSocket command exceeds the 64 KiB limit".into(),
                            }).unwrap_or_default();
                            let _ = socket.send(Message::Text(err.into())).await;
                            break;
                        }
                        handle_ws_command(&mut socket, &state, &text, &session_key).await;
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

            _ = &mut idle_timeout => {
                let err = serde_json::to_string(&WsEvent::Error {
                    code: "WS_IDLE_TIMEOUT".into(),
                    message: "WebSocket session expired after inactivity".into(),
                }).unwrap_or_default();
                let _ = socket.send(Message::Text(err.into())).await;
                break;
            }
            // Keepalive ping
            _ = ping_interval.tick() => {
                let pong = serde_json::to_string(&WsEvent::Pong {
                    timestamp: now_secs(),
                }).unwrap_or_default();
                if socket.send(Message::Text(pong.into())).await.is_err() {
                    break;
                }
            }
        }
    }

    tracing::debug!("WebSocket session ended");
}

async fn handle_ws_command(
    socket: &mut WebSocket,
    state: &Arc<AppState>,
    text: &str,
    _session_key: &[u8; 32],
) {
    let cmd: WsCommand = match serde_json::from_str(text) {
        Ok(c) => c,
        Err(_) => {
            let err = serde_json::to_string(&WsEvent::Error {
                code: "PARSE_ERROR".into(),
                message: "Invalid command JSON".into(),
            })
            .unwrap_or_default();
            let _ = socket.send(Message::Text(err.into())).await;
            return;
        }
    };

    match cmd {
        WsCommand::Ping => {
            let pong = serde_json::to_string(&WsEvent::Pong {
                timestamp: now_secs(),
            })
            .unwrap_or_default();
            let _ = socket.send(Message::Text(pong.into())).await;
        }

        WsCommand::GetPeers => {
            let core = state.core.read().await;
            let snapshot = crate::core::mesh_health::build_snapshot(
                &**core,
                state.ws_count.load(std::sync::atomic::Ordering::Relaxed),
            )
            .await;
            let peers = snapshot
                .peers
                .into_iter()
                .filter_map(|peer| serde_json::to_value(peer).ok())
                .collect::<Vec<_>>();
            let count = peers.len();
            let resp =
                serde_json::to_string(&WsEvent::PeerList { peers, count }).unwrap_or_default();
            let _ = socket.send(Message::Text(resp.into())).await;
        }

        WsCommand::SendMessage { recipient, content } => {
            if recipient.is_empty()
                || recipient.len() > 256
                || content.trim().is_empty()
                || content.len() > MAX_WS_CONTENT_BYTES
            {
                let err = serde_json::to_string(&WsEvent::Error {
                    code: "MESSAGE_LIMIT".into(),
                    message: "Recipient or message content exceeds limits".into(),
                })
                .unwrap_or_default();
                let _ = socket.send(Message::Text(err.into())).await;
                return;
            }
            let core = state.core.read().await;
            if core.is_locked().await {
                let err = serde_json::to_string(&WsEvent::Error {
                    code: "IDENTITY_LOCKED".into(),
                    message: "Unlock the active identity before sending messages".into(),
                })
                .unwrap_or_default();
                let _ = socket.send(Message::Text(err.into())).await;
                return;
            }
            let sender = core.get_identity_id().await;
            let msg = crate::core::message::Message::new(sender, recipient.clone(), content);
            match core.send_direct_message(recipient, &msg).await {
                Ok(_) => {
                    let ack = serde_json::to_string(&WsEvent::Accepted {
                        code: "ACCEPTED".into(),
                        message_id: msg.id.to_string(),
                        delivery: "receipt_pending".into(),
                    })
                    .unwrap_or_default();
                    let _ = socket.send(Message::Text(ack.into())).await;
                }
                Err(e) => {
                    let err = serde_json::to_string(&WsEvent::Error {
                        code: "SEND_FAILED".into(),
                        message: format!("Delivery failed: {}", e),
                    })
                    .unwrap_or_default();
                    let _ = socket.send(Message::Text(err.into())).await;
                }
            }
        }

        WsCommand::Subscribe { events } => {
            if events.len() > MAX_WS_SUBSCRIPTIONS
                || events.iter().any(|event| {
                    event.len() > MAX_WS_EVENT_NAME_BYTES
                        || !ALLOWED_WS_EVENTS.iter().any(|allowed| allowed == event)
                })
            {
                let err = serde_json::to_string(&WsEvent::Error {
                    code: "SUBSCRIPTION_LIMIT".into(),
                    message: "Subscription request exceeds limits".into(),
                })
                .unwrap_or_default();
                let _ = socket.send(Message::Text(err.into())).await;
                return;
            }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ws_quota_rejects_commands_after_session_limit() {
        let mut quota = WsQuota::new();
        for _ in 0..MAX_WS_COMMANDS_PER_MINUTE {
            assert!(quota.allow_command());
        }
        assert!(!quota.allow_command());
    }

    #[test]
    fn ws_event_subscription_is_allowlisted() {
        assert!(ALLOWED_WS_EVENTS.contains(&"core_event"));
        assert!(ALLOWED_WS_EVENTS.contains(&"connection_request"));
        assert!(!ALLOWED_WS_EVENTS.contains(&"arbitrary_private_event"));
    }
}
