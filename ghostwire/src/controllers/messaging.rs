use crate::api_v2_types::*;
use crate::web::AppState;
use axum::{
    extract::{Query, State},
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

/// ✅ SECURITY: Message size limit (10KB)
/// Prevents memory exhaustion attacks
const MAX_MESSAGE_SIZE: usize = 10 * 1024; // 10KB

pub struct MessagingController;

impl MessagingController {
    /// GET /api/v2/messages - Message list from in-memory + sovereign storage
    pub async fn get_messages(
        State(state): State<Arc<AppState>>,
        Query(params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
        let peer_id = params
            .get("peer_id")
            .cloned()
            .unwrap_or_else(|| "all".to_string());
        let limit: usize = params
            .get("limit")
            .and_then(|l| l.parse().ok())
            .unwrap_or(50);

        // ✅ SECURITY: Cap limit to prevent DoS
        let limit = limit.min(1000);

        // Read from both in-memory buffer (incoming mesh messages) and encrypted store
        let messages = state.core.read().await.get_messages(&peer_id, limit).await;

        Json(ApiV2Response::success(messages).with_links("/api/v2/messages".to_string()))
    }

    /// POST /api/v2/messages/broadcast - Send message to all connected peers
    pub async fn broadcast(
        State(state): State<Arc<AppState>>,
        Json(req): Json<BroadcastRequest>,
    ) -> Response {
        // ✅ SECURITY: Validate broadcast message size
        if req.message.len() > MAX_MESSAGE_SIZE {
            return ApiV2Response::<serde_json::Value>::error_response(
                StandardErrorCode::InvalidInput.as_str(),
                format!("Message too large (max {} bytes)", MAX_MESSAGE_SIZE),
            )
            .into_response();
        }

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        let sender_id = state.core.read().await.get_identity_id().await;
        let message = crate::core::message::Message::new(
            sender_id,
            "broadcast".to_string(),
            req.message.clone(),
        );

        // Actually send via mesh
        let send_result = state.core.read().await.send_message(&message).await;

        let peer_count = state.core.read().await.get_peer_count();
        let status = if send_result.is_ok() {
            "BROADCAST_SENT"
        } else {
            "BROADCAST_QUEUED"
        };

        #[derive(Serialize)]
        struct BroadcastResult {
            message_id: String,
            recipients: usize,
            timestamp: u64,
            status: String,
            delivery: String,
        }

        let result = BroadcastResult {
            message_id: message.id.to_string(),
            recipients: peer_count,
            timestamp,
            status: status.to_string(),
            delivery: if send_result.is_ok() {
                "receipt_pending".to_string()
            } else {
                "queued".to_string()
            },
        };

        tracing::info!("Broadcast message sent to {} peers", peer_count);

        Json(ApiV2Response::success(result)).into_response()
    }

    pub async fn send_message(
        State(state): State<Arc<AppState>>,
        Json(payload): Json<SendMessageRequest>,
    ) -> Response {
        // ✅ SECURITY: Validate message size
        if payload.content.len() > MAX_MESSAGE_SIZE {
            return ApiV2Response::<serde_json::Value>::error_response(
                StandardErrorCode::InvalidInput.as_str(),
                format!("Message too large (max {} bytes)", MAX_MESSAGE_SIZE),
            )
            .into_response();
        }

        // ✅ SECURITY: Validate content is not empty
        if payload.content.trim().is_empty() {
            return ApiV2Response::<serde_json::Value>::error_response(
                StandardErrorCode::InvalidInput.as_str(),
                "Message content cannot be empty",
            )
            .into_response();
        }

        // ✅ SECURITY: Validate recipient format
        if payload.recipient.len() > 256 {
            return ApiV2Response::<serde_json::Value>::error_response(
                StandardErrorCode::InvalidInput.as_str(),
                "Recipient ID too long",
            )
            .into_response();
        }

        let sender_id = state.core.read().await.get_identity_id().await;

        let recipient = if payload.recipient == "broadcast" || payload.recipient.is_empty() {
            "broadcast".to_string()
        } else {
            payload.recipient.clone()
        };

        let message = crate::core::message::Message::new(
            sender_id,
            recipient.clone(),
            payload.content.clone(),
        );

        // If recipient is "broadcast" or empty, use broadcast flavor
        if payload.recipient == "broadcast" || payload.recipient.is_empty() {
            match state.core.read().await.send_message(&message).await {
                Ok(_) => Json(ApiV2Response::<serde_json::Value>::success(
                    serde_json::json!({
                        "status": "accepted",
                        "delivery": "receipt_pending",
                        "message_id": message.id.to_string(),
                        "recipient": recipient,
                    }),
                ))
                .into_response(),
                Err(e) => send_error_response(&e),
            }
        } else {
            match state
                .core
                .read()
                .await
                .send_direct_message(payload.recipient, &message)
                .await
            {
                Ok(_) => Json(ApiV2Response::<serde_json::Value>::success(
                    serde_json::json!({
                        "status": "accepted",
                        "delivery": "receipt_pending",
                        "message_id": message.id.to_string(),
                        "recipient": recipient,
                    }),
                ))
                .into_response(),
                Err(e) => send_error_response(&e),
            }
        }
    }
}

/// Turn a send failure into the most specific status it deserves.
///
/// A locked profile is not an internal error: the node is healthy, the caller
/// simply has to unlock before it can store anything. Reporting it as a 500
/// tells the client to retry or page someone, when the fix is a single unlock
/// call. The distinction matters because, before persistence failures were
/// propagated at all, this endpoint answered `{"status":"accepted"}` with a
/// 200 while nothing was written at all.
fn send_error_response(e: &anyhow::Error) -> axum::response::Response {
    let msg = e.to_string();
    if msg.contains("persistence is locked") {
        return ApiV2Response::<serde_json::Value>::error_response(
            "PERSISTENCE_LOCKED",
            format!(
                "{msg} — unlock the profile (POST /api/v2/profile/unlock) before \
                 sending; the message was NOT stored"
            ),
        )
        .into_response();
    }
    if msg.contains("NOT persisted") {
        return ApiV2Response::<serde_json::Value>::error_response("PERSISTENCE_FAILED", msg)
            .into_response();
    }
    ApiV2Response::<serde_json::Value>::error_response(
        StandardErrorCode::InternalError.as_str(),
        format!("Transmission failed: {msg}"),
    )
    .into_response()
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub recipient: String,
    pub content: String,
    pub msg_type: Option<String>,
}

#[derive(Deserialize)]
pub struct BroadcastRequest {
    pub message: String,
}
