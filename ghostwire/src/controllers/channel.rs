//! Channel Controller
//!
//! Handles ghost channel operations: list, create, join, leave.

use crate::api_v2_types::*;
use crate::web::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

pub struct ChannelController;

impl ChannelController {
    pub async fn list(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        let channels = core.ghost_channels.list().await;
        Json(ApiV2Response::success(channels))
    }

    pub async fn create(
        State(state): State<Arc<AppState>>,
        Json(req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let name = req["name"].as_str().map(|s| s.to_string());
        // Validate channel name
        if let Some(ref n) = name {
            if n.is_empty() || n.len() > 64 || !n.chars().all(|c| c.is_alphanumeric() || c == '_' || c == '-' || c == ' ') {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiV2Response::<()>::error(
                        "INVALID_NAME",
                        "Channel name must be 1-64 characters, alphanumeric/spaces/hyphens/underscores only.",
                    )),
                )
                    .into_response();
            }
        }
        let core = state.core.read().await;
        match core.ghost_channels.create(name).await {
            Ok(ch) => {
                let invite = ch.invite_string();
                let topic = ch.gossipsub_topic();
                Json(ApiV2Response::success(serde_json::json!({
                    "topic_id": ch.topic_id,
                    "local_name": ch.local_name,
                    "description": ch.description,
                    "created_at": ch.created_at,
                    "invite": invite,
                    "topic": topic,
                })))
                .into_response()
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiV2Response::<()>::error("CHANNEL_ERROR", e.to_string())),
            )
                .into_response(),
        }
    }

    pub async fn join(
        State(state): State<Arc<AppState>>,
        Json(req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let invite = req["invite"].as_str().unwrap_or("");
        if invite.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error("INVALID_INVITE", "Invite code required")),
            )
                .into_response();
        }
        let core = state.core.read().await;
        match core.ghost_channels.join(invite).await {
            Ok(ch) => {
                let topic = ch.gossipsub_topic();
                Json(ApiV2Response::success(serde_json::json!({
                    "topic_id": ch.topic_id,
                    "local_name": ch.local_name,
                    "description": ch.description,
                    "created_at": ch.created_at,
                    "topic": topic,
                })))
                .into_response()
            }
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error("JOIN_ERROR", e.to_string())),
            )
                .into_response(),
        }
    }

    pub async fn leave(
        State(state): State<Arc<AppState>>,
        Path(channel_id): Path<String>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;
        if let Ok(bytes) = hex::decode(&channel_id) {
            if bytes.len() == 32 {
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&bytes);
                core.ghost_channels.leave(&arr).await;
                return Json(ApiV2Response::success(serde_json::json!({ "left": channel_id })))
                    .into_response();
            }
        }
        (
            StatusCode::BAD_REQUEST,
            Json(ApiV2Response::<()>::error("INVALID_CHANNEL_ID", "Invalid channel ID (expected 32-byte hex)")),
        )
            .into_response()
    }

    pub async fn publish(
        State(state): State<Arc<AppState>>,
        Path(id): Path<String>,
        Json(req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let text = req["text"].as_str().unwrap_or("");
        if text.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error("INVALID_MESSAGE", "Message text required")),
            )
                .into_response();
        }
        if text.len() > 10000 {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error("MESSAGE_TOO_LONG", "Channel message exceeds 10000 character limit")),
            )
                .into_response();
        }

        let topic_id: Option<[u8; 32]> = hex::decode(&id).ok().and_then(|b| b.try_into().ok());
        let Some(topic_id) = topic_id else {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error("INVALID_ID", "invalid channel id")),
            )
                .into_response();
        };

        let core = state.core.read().await;
        let Some(channel) = core.ghost_channels.get(&topic_id).await else {
            return (
                StatusCode::NOT_FOUND,
                Json(ApiV2Response::<()>::error("NOT_FOUND", "Channel not joined")),
            )
                .into_response();
        };

        match core.ghost_channels.prepare_message(&channel, text).await {
            Ok(msg) => {
                #[cfg(feature = "mesh")]
                {
                    if let Some(mesh_manager) = &core.mesh_manager {
                        let mgr = mesh_manager.read().await;
                        let topic = format!("ghostwire-channel-{}", hex::encode(topic_id));
                        let content = serde_json::to_string(&msg).unwrap_or_default();
                        let _ = mgr.publish_to_topic(&topic, &content).await;
                    }
                }
                Json(ApiV2Response::success(serde_json::json!({
                    "status": "published",
                    "msg_id": hex::encode(msg.msg_id),
                    "pow_nonce": msg.pow_nonce,
                })))
                .into_response()
            }
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error("PUBLISH_ERROR", e.to_string())),
            )
                .into_response(),
        }
    }

    pub async fn get_messages(
        State(state): State<Arc<AppState>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        let topic_id: Option<[u8; 32]> = hex::decode(&id).ok().and_then(|b| b.try_into().ok());
        let Some(topic_id) = topic_id else {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error("INVALID_ID", "invalid channel id")),
            )
                .into_response();
        };
        let core = state.core.read().await;
        let messages = core.ghost_channels.get_messages(&topic_id).await;
        Json(ApiV2Response::success(messages)).into_response()
    }
}
