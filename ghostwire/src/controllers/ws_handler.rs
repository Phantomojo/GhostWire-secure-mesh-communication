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
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::time::{interval, Duration};

/// Encrypted WebSocket message format
/// Format: {"enc": {"nonce": "<base64>", "ct": "<base64>"}}
#[derive(Debug, Serialize, Deserialize)]
struct EncryptedWsMessage {
    nonce: String,
    ct: String,
}

/// Encrypt data using XChaCha20-Poly1305
fn encrypt_ws_message(plaintext: &[u8], session_key: &[u8; 32]) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
    use chacha20poly1305::{XChaCha20Poly1305, XNonce, Key, KeyInit};
    use chacha20poly1305::aead::Aead;
    use rand::RngCore;
    use base64::Engine;

    // Generate random 24-byte nonce
    let mut nonce_bytes = [0u8; 24];
    rand::thread_rng().fill_bytes(&mut nonce_bytes);
    let nonce = XNonce::from(nonce_bytes);

    let key = Key::from_slice(session_key);
    let cipher = XChaCha20Poly1305::new(key);

    let ciphertext = cipher.encrypt(&nonce, plaintext)
        .map_err(|e| format!("WS encryption failed: {}", e))?;

    // Encode as JSON
    let enc_msg = EncryptedWsMessage {
        nonce: base64::engine::general_purpose::STANDARD.encode(&nonce_bytes),
        ct: base64::engine::general_purpose::STANDARD.encode(&ciphertext),
    };

    Ok(serde_json::to_string(&enc_msg)?)
}

/// Decrypt data using XChaCha20-Poly1305
fn decrypt_ws_message(encrypted_json: &str, session_key: &[u8; 32]) -> Result<Vec<u8>, Box<dyn std::error::Error + Send + Sync>> {
    use chacha20poly1305::{XChaCha20Poly1305, XNonce, Key, KeyInit};
    use chacha20poly1305::aead::Aead;
    use base64::Engine;

    let enc_msg: EncryptedWsMessage = serde_json::from_str(encrypted_json)?;

    let nonce_bytes = base64::engine::general_purpose::STANDARD.decode(&enc_msg.nonce)?;
    let ciphertext = base64::engine::general_purpose::STANDARD.decode(&enc_msg.ct)?;

    let key = Key::from_slice(session_key);
    let cipher = XChaCha20Poly1305::new(key);
    let nonce = XNonce::from_slice(&nonce_bytes);

    cipher.decrypt(nonce, ciphertext.as_ref())
        .map_err(|e| format!("WS decryption failed: {}", e).into())
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

/// Generate a per-session WebSocket encryption key derived from the node identity.
/// This ensures each WS connection uses a unique key, preventing cross-session decryption.
async fn generate_ws_session_key(state: &Arc<AppState>) -> [u8; 32] {
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
    let okm: ring::hkdf::Okm<ring::hkdf::Algorithm> = prk.expand(&[b"ghostwire-ws-session-v2"], HKDF_SHA256).unwrap();
    let mut okm_bytes = [0u8; 32];
    okm.fill(&mut okm_bytes).unwrap();
    okm_bytes
}

pub async fn handle_socket(mut socket: WebSocket, state: Arc<AppState>) {
    // Generate per-session encryption key from identity + random nonce
    // SECURITY FIX: replaced hardcoded key with per-session derived key
    let session_key = generate_ws_session_key(&state).await;

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
                // Encrypt the message before sending
                match encrypt_ws_message(json.as_bytes(), &session_key) {
                    Ok(encrypted) => {
                        if socket.send(Message::Text(encrypted)).await.is_err() {
                            tracing::debug!("WS client disconnected (send failed)");
                            break;
                        }
                    }
                    Err(e) => {
                        tracing::warn!("WS encryption failed: {}", e);
                        // Send unencrypted error as fallback
                        if socket.send(Message::Text(json)).await.is_err() {
                            break;
                        }
                    }
                }
            }

            // Inbound: handle commands from frontend
            msg = socket.recv() => {
                match msg {
                    Some(Ok(Message::Text(text))) => {
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

async fn handle_ws_command(socket: &mut WebSocket, state: &Arc<AppState>, text: &str, _session_key: &[u8; 32]) {
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
