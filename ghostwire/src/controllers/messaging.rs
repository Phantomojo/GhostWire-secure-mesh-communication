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

/// Maximum plaintext message bytes accepted at the public API boundary.
///
/// Transport-specific framing and encrypted-envelope limits remain enforced by
/// the relevant transport. This boundary prevents accepting obviously oversized
/// requests while keeping dispatch failures truthful to clients.
const MAX_MESSAGE_SIZE: usize = 10 * 1024;
const PRIVATE_DELIVERY_UNAVAILABLE_MARKER: &str = "PRIVATE_DELIVERY_UNAVAILABLE";

fn public_direct_dispatch_error(error: &anyhow::Error) -> (StandardErrorCode, &'static str) {
    if error
        .to_string()
        .contains(PRIVATE_DELIVERY_UNAVAILABLE_MARKER)
    {
        (
            StandardErrorCode::PrivateDeliveryUnavailable,
            "Private delivery is unavailable because no privacy-preserving route is ready",
        )
    } else {
        (
            StandardErrorCode::InternalError,
            "Direct message could not be dispatched",
        )
    }
}

pub struct MessagingController;

#[derive(Serialize)]
struct BroadcastResult {
    message_id: String,
    recipients: usize,
    timestamp: u64,
    /// `BROADCAST_DISPATCHED` means the local mesh accepted a dispatch request;
    /// it is not a recipient-delivery acknowledgement.
    status: String,
}

impl MessagingController {
    /// GET /api/v2/messages - Message list from in-memory + sovereign storage.
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
            .and_then(|limit| limit.parse().ok())
            .unwrap_or(50)
            .min(1000);

        // Clone a stable core snapshot before awaiting storage access so this
        // request does not block profile/core replacement.
        let core = state.core.read().await.clone();
        let messages = core.get_messages(&peer_id, limit).await;

        Json(ApiV2Response::success(messages).with_links("/api/v2/messages".to_string()))
    }

    /// POST /api/v2/messages/broadcast - Dispatch a message to connected peers.
    pub async fn broadcast(
        State(state): State<Arc<AppState>>,
        Json(req): Json<BroadcastRequest>,
    ) -> impl IntoResponse {
        if req.message.len() > MAX_MESSAGE_SIZE {
            return Json(ApiV2Response::<BroadcastResult>::error(
                StandardErrorCode::InvalidInput.as_str(),
                format!("Message too large (max {MAX_MESSAGE_SIZE} bytes)"),
            ));
        }
        if req.message.trim().is_empty() {
            return Json(ApiV2Response::<BroadcastResult>::error(
                StandardErrorCode::InvalidInput.as_str(),
                "Message content cannot be empty",
            ));
        }

        let core = state.core.read().await.clone();
        let sender_id = core.get_identity_id().await;
        let message =
            crate::core::message::Message::new(sender_id, "broadcast".to_string(), req.message);

        if let Err(error) = core.send_message(&message).await {
            // Preserve source detail only in local logs. A remote caller must
            // not receive topology, transport, or implementation diagnostics.
            tracing::warn!(error = %error, message_id = %message.id, "Broadcast dispatch failed");
            return Json(ApiV2Response::<BroadcastResult>::error(
                StandardErrorCode::InternalError.as_str(),
                "Broadcast could not be dispatched",
            ));
        }

        let peer_count = core.get_peer_count();
        let result = BroadcastResult {
            // The response must refer to the exact message dispatched above,
            // rather than a second unrelated UUID.
            message_id: message.id.to_string(),
            recipients: peer_count,
            timestamp: message.timestamp,
            status: "BROADCAST_DISPATCHED".to_string(),
        };

        tracing::info!(message_id = %message.id, recipients = peer_count, "Broadcast dispatched");
        Json(ApiV2Response::success(result))
    }

    /// POST /api/v2/messages - Dispatch a public broadcast or direct message.
    pub async fn send_message(
        State(state): State<Arc<AppState>>,
        Json(payload): Json<SendMessageRequest>,
    ) -> impl IntoResponse {
        if payload.content.len() > MAX_MESSAGE_SIZE {
            return Json(ApiV2Response::error(
                StandardErrorCode::InvalidInput.as_str(),
                format!("Message too large (max {MAX_MESSAGE_SIZE} bytes)"),
            ));
        }
        if payload.content.trim().is_empty() {
            return Json(ApiV2Response::error(
                StandardErrorCode::InvalidInput.as_str(),
                "Message content cannot be empty",
            ));
        }
        if payload.recipient.len() > 256 {
            return Json(ApiV2Response::error(
                StandardErrorCode::InvalidInput.as_str(),
                "Recipient ID too long",
            ));
        }
        if payload.msg_type.is_some() {
            return Json(ApiV2Response::error(
                StandardErrorCode::InvalidInput.as_str(),
                "Message type selection is not supported by this endpoint",
            ));
        }

        let core = state.core.read().await.clone();
        let sender_id = core.get_identity_id().await;
        let is_broadcast = payload.recipient.is_empty() || payload.recipient == "broadcast";
        let recipient = if is_broadcast {
            "broadcast".to_string()
        } else {
            payload.recipient.clone()
        };
        let message = crate::core::message::Message::new(sender_id, recipient, payload.content);

        if is_broadcast {
            match core.send_message(&message).await {
                Ok(()) => Json(ApiV2Response::success("Broadcast dispatched successfully")),
                Err(error) => {
                    tracing::warn!(error = %error, message_id = %message.id, "Broadcast dispatch failed");
                    Json(ApiV2Response::error(
                        StandardErrorCode::InternalError.as_str(),
                        "Broadcast could not be dispatched",
                    ))
                }
            }
        } else {
            match core.send_direct_message(payload.recipient, &message).await {
                Ok(()) => Json(ApiV2Response::success(
                    "Direct message dispatched successfully",
                )),
                Err(error) => {
                    tracing::warn!(error = %error, message_id = %message.id, "Direct-message dispatch failed");
                    let (code, message) = public_direct_dispatch_error(&error);
                    Json(ApiV2Response::error(code.as_str(), message))
                }
            }
        }
    }
}

#[derive(Deserialize)]
pub struct SendMessageRequest {
    pub recipient: String,
    pub content: String,
    /// Reserved for a future closed protocol-owned message-kind enum.
    /// Arbitrary strings are rejected rather than silently ignored.
    pub msg_type: Option<String>,
}

#[derive(Deserialize)]
pub struct BroadcastRequest {
    pub message: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn private_route_failure_uses_stable_public_error_code() {
        let error = anyhow::anyhow!(
            "PRIVATE_DELIVERY_UNAVAILABLE: no privacy-preserving route is available"
        );
        let (code, message) = public_direct_dispatch_error(&error);

        assert!(matches!(
            code,
            StandardErrorCode::PrivateDeliveryUnavailable
        ));
        assert_eq!(
            message,
            "Private delivery is unavailable because no privacy-preserving route is ready"
        );
    }

    #[test]
    fn unrelated_dispatch_failure_stays_generic() {
        let error = anyhow::anyhow!("connection reset");
        let (code, message) = public_direct_dispatch_error(&error);

        assert!(matches!(code, StandardErrorCode::InternalError));
        assert_eq!(message, "Direct message could not be dispatched");
    }
}
