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
        let core = state.core.read().await;
        match core.ghost_channels.create(name).await {
            Ok(ch) => Json(ApiV2Response::success(serde_json::json!({
                "channel": ch,
                "invite": ch.invite_string(),
                "topic": ch.gossipsub_topic(),
            })))
            .into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiV2Response::<()>::error("CHANNEL_ERROR", &e.to_string())),
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
            Ok(ch) => Json(ApiV2Response::success(serde_json::json!({
                "channel": ch,
                "topic": ch.gossipsub_topic(),
            })))
            .into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error("JOIN_ERROR", &e.to_string())),
            )
                .into_response(),
        }
    }

    pub async fn leave(
        State(state): State<Arc<AppState>>,
        Path(channel_id): Path<String>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;
        match core.ghost_channels.leave(&channel_id).await {
            Ok(_) => Json(ApiV2Response::success(serde_json::json!({ "left": channel_id })))
                .into_response(),
            Err(e) => (
                StatusCode::NOT_FOUND,
                Json(ApiV2Response::<()>::error("LEAVE_ERROR", &e.to_string())),
            )
                .into_response(),
        }
    }

    pub async fn list_members(
        State(state): State<Arc<AppState>>,
        Path(channel_id): Path<String>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;
        let members = core.ghost_channels.list_members(&channel_id).await;
        Json(ApiV2Response::success(members))
    }

    pub async fn get_invite(
        State(state): State<Arc<AppState>>,
        Path(channel_id): Path<String>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;
        match core.ghost_channels.get_invite(&channel_id).await {
            Some(invite) => Json(ApiV2Response::success(serde_json::json!({
                "invite": invite,
            })))
            .into_response(),
            None => (
                StatusCode::NOT_FOUND,
                Json(ApiV2Response::<()>::error("NOT_FOUND", "Channel not found")),
            )
                .into_response(),
        }
    }

    pub async fn rotate_invite(
        State(state): State<Arc<AppState>>,
        Path(channel_id): Path<String>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;
        match core.ghost_channels.rotate_invite(&channel_id).await {
            Ok(invite) => Json(ApiV2Response::success(serde_json::json!({
                "invite": invite,
            })))
            .into_response(),
            Err(e) => (
                StatusCode::NOT_FOUND,
                Json(ApiV2Response::<()>::error("ROTATE_ERROR", &e.to_string())),
            )
                .into_response(),
        }
    }

    pub async fn stats(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        let stats = core.ghost_channels.stats().await;
        Json(ApiV2Response::success(stats))
    }

    pub async fn set_privacy(
        State(state): State<Arc<AppState>>,
        Path((channel_id, privacy)): Path<(String, String)>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;
        let privacy_enum = match privacy.as_str() {
            "public" => crate::core::channels::ChannelPrivacy::Public,
            "private" => crate::core::channels::ChannelPrivacy::Private,
            _ => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiV2Response::<()>::error(
                        "INVALID_PRIVACY",
                        "Must be 'public' or 'private'",
                    )),
                )
                    .into_response();
            }
        };
        match core
            .ghost_channels
            .set_privacy(&channel_id, privacy_enum)
            .await
        {
            Ok(_) => Json(ApiV2Response::success(serde_json::json!({
                "channel_id": channel_id,
                "privacy": privacy,
            })))
            .into_response(),
            Err(e) => (
                StatusCode::NOT_FOUND,
                Json(ApiV2Response::<()>::error("PRIVACY_ERROR", &e.to_string())),
            )
                .into_response(),
        }
    }

    pub async fn get_channel_info(
        State(state): State<Arc<AppState>>,
        Path(channel_id): Path<String>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;
        match core.ghost_channels.get_info(&channel_id).await {
            Some(info) => Json(ApiV2Response::success(info)).into_response(),
            None => (
                StatusCode::NOT_FOUND,
                Json(ApiV2Response::<()>::error("NOT_FOUND", "Channel not found")),
            )
                .into_response(),
        }
    }

    pub async fn list_topics(
        State(state): State<Arc<AppState>>,
        Path(channel_id): Path<String>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;
        let topics = core.ghost_channels.list_topics(&channel_id).await;
        Json(ApiV2Response::success(topics))
    }

    pub async fn add_topic(
        State(state): State<Arc<AppState>>,
        Path((channel_id, topic)): Path<(String, String)>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;
        match core.ghost_channels.add_topic(&channel_id, &topic).await {
            Ok(_) => Json(ApiV2Response::success(serde_json::json!({
                "channel_id": channel_id,
                "topic": topic,
            })))
            .into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiV2Response::<()>::error("TOPIC_ERROR", &e.to_string())),
            )
                .into_response(),
        }
    }

    pub async fn remove_topic(
        State(state): State<Arc<AppState>>,
        Path((channel_id, topic)): Path<(String, String)>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;
        match core.ghost_channels.remove_topic(&channel_id, &topic).await {
            Ok(_) => Json(ApiV2Response::success(serde_json::json!({
                "channel_id": channel_id,
                "topic": topic,
                "removed": true,
            })))
            .into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiV2Response::<()>::error("TOPIC_ERROR", &e.to_string())),
            )
                .into_response(),
        }
    }
}
