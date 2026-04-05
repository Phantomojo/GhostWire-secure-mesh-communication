/// Enhanced profile creation with comprehensive error logging
/// This file provides detailed debugging for profile creation errors

use crate::api_v2_types::*;
use crate::core::Core;
use crate::web::AppState;
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::fs;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CreateProfileRequest {
    pub name: String,
    /// Optional: if provided with password, create profile with this nickname and secure it immediately.
    pub nickname: Option<String>,
    /// Optional: if provided with nickname, save identity with this password (min 8 chars).
    pub password: Option<String>,
}

#[derive(Deserialize)]
pub struct SelectProfileRequest {
    pub profile_name: String,
}

#[derive(Serialize)]
pub struct ProfileListResponse {
    pub profiles: Vec<String>,
    pub current: String,
}

pub struct ProfilesController;

impl ProfilesController {
    fn ghostwire_dir() -> std::path::PathBuf {
        if let Ok(ghost_home) = std::env::var("GHOSTWIRE_HOME") {
            std::path::PathBuf::from(ghost_home)
        } else {
            dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."))
                .join(".ghostwire")
        }
    }

    /// V2 Create Profile - ENHANCED WITH DETAILED ERROR LOGGING
    pub async fn create(
        State(_state): State<Arc<AppState>>,
        Json(req): Json<CreateProfileRequest>,
    ) -> impl IntoResponse {
        // === STEP 1: Log request details ===
        tracing::info!(
            target: "profile_creation",
            msg = "📝 Profile creation request received",
            profile_name = %req.name,
            has_nickname = req.nickname.is_some(),
            has_password = req.password.is_some(),
        );

        // === STEP 2: Validate name ===
        if !req
            .name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            tracing::warn!(
                target: "profile_creation",
                msg = "❌ Invalid profile name",
                profile_name = %req.name,
                reason = "Contains invalid characters"
            );
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::InvalidInput.as_str(),
                    "Invalid profile name. Use alphanumeric, _, - only",
                )),
            )
                .into_response();
        }

        // === STEP 3: Check ghostwire directory ===
        let ghost_dir = Self::ghostwire_dir();
        tracing::info!(
            target: "profile_creation",
            msg = "📂 Checking ghostwire directory",
            directory = ?ghost_dir,
            exists = ghost_dir.exists(),
            is_dir = ghost_dir.is_dir(),
        );

        // Try to create directory if it doesn't exist
        if !ghost_dir.exists() {
            tracing::info!(
                target: "profile_creation",
                msg = "🔨 Creating ghostwire directory",
                directory = ?ghost_dir
            );
            if let Err(e) = fs::create_dir_all(&ghost_dir) {
                let error_msg = format!("Failed to create data directory: {}", e);
                tracing::error!(
                    target: "profile_creation",
                    msg = "❌ CRITICAL: Cannot create ghostwire directory",
                    directory = ?ghost_dir,
                    error = %e,
                    error_kind = ?e.kind(),
                );
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiV2Response::<()>::error(
                        StandardErrorCode::InternalError.as_str(),
                        error_msg,
                    )),
                )
                    .into_response();
            }
        }

        // Check if directory is writable
        let test_file = ghost_dir.join(".write_test");
        if let Err(e) = fs::write(&test_file, "test") {
            let error_msg = format!("Data directory is not writable: {}", e);
            tracing::error!(
                target: "profile_creation",
                msg = "❌ CRITICAL: Directory not writable",
                directory = ?ghost_dir,
                error = %e,
            );
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::InternalError.as_str(),
                    error_msg,
                )),
            )
                .into_response();
        }
        let _ = fs::remove_file(&test_file); // Clean up test file

        // === STEP 4: Check if profile already exists ===
        let id_filename = if req.name == "default" {
            "identity.json".to_string()
        } else {
            format!("identity_{}.json", req.name)
        };
        let path = ghost_dir.join(&id_filename);
        
        tracing::info!(
            target: "profile_creation",
            msg = "🔍 Checking if profile exists",
            profile_name = %req.name,
            identity_file = ?path,
            exists = path.exists(),
        );

        if path.exists() {
            tracing::warn!(
                target: "profile_creation",
                msg = "⚠️ Profile already exists",
                profile_name = %req.name,
            );
            return (
                StatusCode::CONFLICT,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::InvalidInput.as_str(),
                    "Profile already exists",
                )),
            )
                .into_response();
        }

        // === STEP 5: Prepare profile argument ===
        let profile_arg = if req.name == "default" {
            None
        } else {
            Some(req.name.clone())
        };

        // === STEP 6: Create profile ===
        tracing::info!(
            target: "profile_creation",
            msg = "🔧 Creating profile",
            profile_name = %req.name,
            with_setup = req.nickname.is_some() && req.password.is_some(),
        );

        let result = if let (Some(nickname), Some(password)) = (&req.nickname, &req.password) {
            // Validate password length
            if password.len() < 8 {
                tracing::warn!(
                    target: "profile_creation",
                    msg = "❌ Password too short",
                    password_length = password.len(),
                    minimum_length = 8
                );
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiV2Response::<()>::error(
                        StandardErrorCode::InvalidInput.as_str(),
                        "Password must be at least 8 characters when provided",
                    )),
                )
                    .into_response();
            }

            tracing::info!(
                target: "profile_creation",
                msg = "🔐 Creating profile with nickname and password",
                profile_name = %req.name,
                nickname = %nickname,
            );

            crate::core::identity::core_identity::Identity::create_with_setup(
                profile_arg,
                nickname.clone(),
                password,
            )
        } else {
            tracing::info!(
                target: "profile_creation",
                msg = "🆕 Creating basic profile (no nickname/password)",
                profile_name = %req.name,
            );

            crate::core::identity::core_identity::Identity::new(profile_arg)
        };

        // === STEP 7: Handle result ===
        match result {
            Ok(_) => {
                tracing::info!(
                    target: "profile_creation",
                    msg = "✅ Profile created successfully",
                    profile_name = %req.name,
                );
                Json(ApiV2Response::success(serde_json::json!({
                    "status": "created",
                    "profile": req.name,
                    "message": if req.nickname.is_some() && req.password.is_some() {
                        "Profile created and secured with provided nickname and password"
                    } else {
                        "Profile created; switch to it and run setup to set nickname and password"
                    }
                })))
                .into_response()
            }
            Err(e) => {
                let error_msg = format!("Failed to create profile: {}", e);
                tracing::error!(
                    target: "profile_creation",
                    msg = "❌ CRITICAL: Profile creation failed",
                    profile_name = %req.name,
                    error = %e,
                    error_debug = format!("{:?}", e),
                    error_cause = format!("{}", e),
                );
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiV2Response::<()>::error(
                        StandardErrorCode::InternalError.as_str(),
                        error_msg,
                    )),
                )
                    .into_response()
            }
        }
    }

    // ... rest of the methods remain the same ...
}
