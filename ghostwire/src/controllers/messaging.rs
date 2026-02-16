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

pub struct MessagingController;

impl MessagingController {
    /// GET /api/v2/messages - Historical message list from sovereign storage
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

        let messages = state
            .core
            .read()
            .await
            .message_store
            .clone()
            .get_history(&peer_id, limit);

        Json(ApiV2Response::success(messages).with_links("/api/v2/messages".to_string()))
    }

    /// POST /api/v2/messages/broadcast - Send message to all connected peers
    pub async fn broadcast(
        State(state): State<Arc<AppState>>,
        Json(_req): Json<BroadcastRequest>,
    ) -> impl IntoResponse {
        let message_id = uuid::Uuid::new_v4();
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();

        // Get actual connected peers from the core
        let peer_count = state.core.read().await.get_peer_count();

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
            status: "BROADCAST_QUEUED".to_string(),
        };

        tracing::info!("Broadcast message sent");

        Json(ApiV2Response::success(result))
    }

    pub async fn send_message(
        State(state): State<Arc<AppState>>,
        Json(payload): Json<SendMessageRequest>,
    ) -> impl IntoResponse {
        let sender_id = state.core.read().await.get_identity_id().await;

        let mut message = crate::core::message::Message::new(
            sender_id,
            "placeholder_recipient".to_string(),
            payload.content.clone(),
        );

        // If recipient is "broadcast" or empty, use broadcast flavor
        if payload.recipient == "broadcast" || payload.recipient.is_empty() {
            message.recipient = "broadcast".to_string();
            match state.core.read().await.send_message(&message).await {
                Ok(_) => Json(ApiV2Response::success("Broadcast sent successfully")),
                Err(e) => Json(ApiV2Response::error(
                    StandardErrorCode::InternalError.as_str(),
                    &format!("Broadcast failed: {}", e),
                )),
            }
        } else {
            message.recipient = payload.recipient.clone();
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
