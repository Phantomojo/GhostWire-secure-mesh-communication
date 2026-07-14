use crate::api_v2_types::*;
use crate::web::AppState;
use axum::{
    extract::{Query, State},
    response::IntoResponse,
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
    ) -> impl IntoResponse {
        // ✅ SECURITY: Validate broadcast message size
        if req.message.len() > MAX_MESSAGE_SIZE {
            return Json(ApiV2Response::error(
                StandardErrorCode::InvalidInput.as_str(),
                &format!("Message too large (max {} bytes)", MAX_MESSAGE_SIZE),
            ));
        }

        let message_id = uuid::Uuid::new_v4();
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
        }

        let result = BroadcastResult {
            message_id: message_id.to_string(),
            recipients: peer_count,
            timestamp,
            status: status.to_string(),
        };

        tracing::info!("Broadcast message sent to {} peers", peer_count);

        Json(ApiV2Response::success(result))
    }

    pub async fn send_message(
        State(state): State<Arc<AppState>>,
        Json(payload): Json<SendMessageRequest>,
    ) -> impl IntoResponse {
        // ✅ SECURITY: Validate message size
        if payload.content.len() > MAX_MESSAGE_SIZE {
            return Json(ApiV2Response::error(
                StandardErrorCode::InvalidInput.as_str(),
                &format!("Message too large (max {} bytes)", MAX_MESSAGE_SIZE),
            ));
        }

        // ✅ SECURITY: Validate content is not empty
        if payload.content.trim().is_empty() {
            return Json(ApiV2Response::error(
                StandardErrorCode::InvalidInput.as_str(),
                "Message content cannot be empty",
            ));
        }

        // ✅ SECURITY: Validate recipient format
        if payload.recipient.len() > 256 {
            return Json(ApiV2Response::error(
                StandardErrorCode::InvalidInput.as_str(),
                "Recipient ID too long",
            ));
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
                Ok(_) => Json(ApiV2Response::success("Broadcast sent successfully")),
                Err(e) => Json(ApiV2Response::error(
                    StandardErrorCode::InternalError.as_str(),
                    &format!("Broadcast failed: {}", e),
                )),
            }
        } else {
            match state
                .core
                .read()
                .await
                .send_direct_message(payload.recipient, &message)
                .await
            {
                Ok(_) => Json(ApiV2Response::success("Direct message sent successfully")),
                Err(e) => Json(ApiV2Response::error(
                    StandardErrorCode::InternalError.as_str(),
                    &format!("Transmission failed: {}", e),
                )),
            }
        }
    }
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
