use crate::api_v2_types::*;
use crate::web::AppState;
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Json};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct SetupRequest {
    pub nickname: String,
    pub password: String,
}

pub struct IdentityController;

impl IdentityController {
    /// GET /api/v2/profile - Full user profile/identity
    pub async fn get_profile(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        #[derive(Serialize)]
        struct ProfileResponse {
            id: String,
            did: String,
            username: String, // Deprecated, mapping to nickname for backward compat
            nickname: String,
            bio: String,
            public_key: String,
            key_id: String,
            created_at: String,
            dht_published: bool,
            security_score: u8,
            setup_complete: bool,
        }

        let metadata = state.core.read().await.get_metadata().await;
        let security_score = state
            .core
            .read()
            .await
            .security_manager
            .clone()
            .calculate_security_score()
            .await;

        let profile = ProfileResponse {
            id: state.core.read().await.get_identity_id().await,
            did: state.core.read().await.get_did().await,
            username: metadata.nickname.clone(), // Map nickname to username for UI compat
            nickname: metadata.nickname,
            bio: metadata.bio,
            public_key: general_purpose::STANDARD.encode(state.core.read().await.get_public_key()),
            key_id: state.core.read().await.get_key_id(),
            created_at: chrono::Utc::now().to_rfc3339(),
            dht_published: false,
            security_score,
            setup_complete: metadata.setup_complete,
        };

        Json(ApiV2Response::success(profile).with_links("/api/v2/profile".to_string()))
    }

    /// POST /api/identity/setup - Initialize secure profile (First Time Setup)
    pub async fn setup(
        State(state): State<Arc<AppState>>,
        Json(req): Json<SetupRequest>,
    ) -> impl IntoResponse {
        // Validate input
        if req.password.len() < 8 {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::InvalidInput.as_str(),
                    "Password must be at least 8 characters",
                )),
            )
                .into_response();
        }

        let core = state.core.read().await;

        // 0. Unlock Identity using default password (required for first-time setup)
        if core.is_locked().await {
            if let Err(e) = core.unlock("ghostwire").await {
                tracing::error!("Failed to unlock identity for setup: {}", e);
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(ApiV2Response::<()>::error(
                        StandardErrorCode::InvalidCredentials.as_str(),
                        "Failed to unlock identity. Ensure default password is valid.",
                    )),
                )
                    .into_response();
            }
        }

        // 1. Update Metadata (Nickname & Setup Complete flag)
        core.update_metadata(Some(req.nickname.clone()), None, None, Some(true))
            .await;

        // 2. Re-encrypt and Save Identity with NEW password
        if let Err(e) = core.save_to_disk(&req.password).await {
            tracing::error!("Failed to save new identity configuration: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::InternalError.as_str(),
                    format!("Failed to secure identity: {}", e),
                )),
            )
                .into_response();
        }

        tracing::info!("Identity setup complete for {}", req.nickname);

        Json(ApiV2Response::success(serde_json::json!({
            "status": "identity_secured",
            "nickname": req.nickname
        })))
        .into_response()
    }

    /// PUT /api/v2/profile/username - Update username
    pub async fn update_username(
        State(state): State<Arc<AppState>>,
        Json(req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        #[derive(Serialize)]
        struct UsernameUpdateResponse {
            username: String,
            updated_at: String,
        }

        let username = req.get("username").and_then(|v| v.as_str()).unwrap_or("");

        if username.is_empty() || username.len() > 32 {
            return Json(ApiV2Response::<()>::error(
                "INVALID_USERNAME",
                "Username must be 1-32 characters",
            ))
            .into_response();
        }

        // Save username as nickname in metadata
        state
            .core
            .read()
            .await
            .update_metadata(Some(username.to_string()), None, None, None)
            .await;

        Json(ApiV2Response::success(UsernameUpdateResponse {
            username: username.to_string(),
            updated_at: chrono::Utc::now().to_rfc3339(),
        }))
        .into_response()
    }

    /// PUT /api/v2/profile/metadata - Update profile metadata
    pub async fn update_metadata(
        State(state): State<Arc<AppState>>,
        Json(req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let nickname = req
            .get("nickname")
            .and_then(|v| v.as_str().map(|s| s.to_string()));
        let bio = req
            .get("bio")
            .and_then(|v| v.as_str().map(|s| s.to_string()));
        let avatar_url = req
            .get("avatar_url")
            .and_then(|v| v.as_str().map(|s| s.to_string()));

        state
            .core
            .read()
            .await
            .update_metadata(nickname, bio, avatar_url, None)
            .await;

        Json(ApiV2Response::success(serde_json::json!({
            "status": "updated",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })))
    }

    /// GET /api/v2/profile/public-key - Get public key
    pub async fn get_public_key(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        #[derive(Serialize)]
        struct PublicKeyResponse {
            key_id: String,
            algorithm: String,
            public_key: String,
            created_at: String,
        }

        Json(ApiV2Response::success(PublicKeyResponse {
            key_id: state.core.read().await.get_key_id(),
            algorithm: "Ed25519".to_string(),
            public_key: general_purpose::STANDARD.encode(state.core.read().await.get_public_key()),
            created_at: chrono::Utc::now().to_rfc3339(),
        }))
    }

    pub async fn report_error(Json(req): Json<ErrorReportRequest>) -> impl IntoResponse {
        eprintln!("[REMOTE ERROR REPORT] {}", req.error);
        Json(ApiV2Response::success(
            serde_json::json!({ "status": "reported" }),
        ))
    }

    pub async fn get_tutorial() -> impl IntoResponse {
        Json(ApiV2Response::success(
            crate::core::tutorial::get_tutorial_steps(),
        ))
    }
}

#[derive(Deserialize)]
pub struct ErrorReportRequest {
    pub error: String,
}
