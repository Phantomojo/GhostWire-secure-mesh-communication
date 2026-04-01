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

    /// V2 List Profiles
    pub async fn list(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let ghost_dir = Self::ghostwire_dir();
        let mut profiles = Vec::new();

        if let Ok(entries) = fs::read_dir(ghost_dir) {
            for entry in entries.filter_map(|e| e.ok()) {
                if let Some(fname) = entry.file_name().to_str() {
                    if fname.starts_with("identity_") && fname.ends_with(".json") {
                        let profile_name = fname
                            .trim_start_matches("identity_")
                            .trim_end_matches(".json");
                        if !profile_name.is_empty() {
                            profiles.push(profile_name.to_string());
                        }
                    } else if fname == "identity.json" {
                        profiles.push("default".to_string());
                    }
                }
            }
        }

        let current = state
            .core
            .read()
            .await
            .identity
            .clone()
            .read()
            .await
            .profile
            .clone();

        Json(ApiV2Response::success(ProfileListResponse {
            profiles,
            current: current.unwrap_or_else(|| "default".to_string()),
        }))
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

    /// V2 Delete Profile
    pub async fn delete(
        State(state): State<Arc<AppState>>,
        axum::extract::Path(name): axum::extract::Path<String>,
    ) -> impl IntoResponse {
        // Cannot delete active profile ("default" when profile is None)
        let current = {
            let core = state.core.read().await;
            let identity = core.identity.read().await;
            identity.profile.clone()
        };
        let is_active = (name == "default" && current.is_none()) || current.as_ref() == Some(&name);
        if is_active {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::InvalidInput.as_str(),
                    "Cannot delete active profile. Switch to another profile first.",
                )),
            )
                .into_response();
        }

        let ghostwire_dir = Self::ghostwire_dir();
        let vault_dir = ghostwire_dir.join("vault");

        // 1. Delete identity file: "default" -> identity.json, else -> identity_{name}.json
        let id_filename = if name == "default" {
            "identity.json".to_string()
        } else {
            format!("identity_{}.json", name)
        };
        let id_path = ghostwire_dir.join(id_filename);
        if id_path.exists() {
            if let Err(e) = fs::remove_file(&id_path) {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiV2Response::<()>::error(
                        StandardErrorCode::InternalError.as_str(),
                        format!("Failed to delete identity file: {}", e),
                    )),
                )
                    .into_response();
            }
        }

        // 2. Delete vault (Sled DB dir): "default" -> ghost_default, else -> ghost_{name}
        let vault_subdir = if name == "default" {
            "ghost_default".to_string()
        } else {
            format!("ghost_{}", name)
        };
        let vault_path = vault_dir.join(vault_subdir);
        if vault_path.exists() {
            if let Err(e) = fs::remove_dir_all(&vault_path) {
                tracing::warn!("Failed to remove vault directory for {}: {}", name, e);
            }
        }

        Json(ApiV2Response::success(format!(
            "Profile '{}' purged from system",
            name
        )))
        .into_response()
    }

    /// Emergency Wipe
    pub async fn wipe_all(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
        tracing::warn!(msg = "⚠️ EMERGENCY WIPE TRIGGERED - PURGING ALL DATA");

        let ghost_dir = Self::ghostwire_dir();

        if ghost_dir.exists() {
            if let Err(e) = fs::remove_dir_all(&ghost_dir) {
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiV2Response::<()>::error(
                        StandardErrorCode::InternalError.as_str(),
                        format!("Failed to purge data directory: {}", e),
                    )),
                )
                    .into_response();
            }
        }

        // Shutdown after a short delay
        tokio::spawn(async {
            tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
            std::process::exit(0);
        });

        Json(ApiV2Response::success(
            "Emergency wipe completed. System is shutting down.".to_string(),
        ))
        .into_response()
    }

    /// V2 Select Profile
    pub async fn select(
        State(state): State<Arc<AppState>>,
        Json(req): Json<SelectProfileRequest>,
    ) -> impl IntoResponse {
        let profile_name = if req.profile_name == "default" {
            None
        } else {
            Some(req.profile_name.clone())
        };

        tracing::info!("Switching to profile: {:?}", profile_name);

        // 1. Shutdown active core
        {
            let core = state.core.read().await.clone();
            if let Err(e) = core.shutdown().await {
                tracing::error!(
                    msg = "Error during active core shutdown",
                    error = e.to_string()
                );
            }
        }

        // 2. Initialize new Core
        let new_core_res = Core::new(
            state.full_config.clone(),
            state.config.bootstrap_nodes.clone(),
            state.config.sim_addr.clone(),
            profile_name,
        )
        .await;

        match new_core_res {
            Ok(new_core) => {
                let core_arc = Arc::new(new_core);

                // Swap the core in AppState
                {
                    let mut core_guard = state.core.write().await;
                    *core_guard = core_arc.clone();
                }

                // Start background tasks
                let mesh_host = state.config.mesh_host.clone();
                let core_for_tasks = core_arc.clone();
                tokio::spawn(async move {
                    core_for_tasks.start_nervous_system(mesh_host).await;
                });

                Json(ApiV2Response::success(serde_json::json!({
                    "status": "profile_switched",
                    "profile": req.profile_name
                })))
                .into_response()
            }
            Err(e) => {
                tracing::error!(
                    msg = "Failed to initialize core for profile switch",
                    error = e.to_string()
                );
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiV2Response::<()>::error(
                        StandardErrorCode::CoreInitializationFailed.as_str(),
                        format!("Failed to initialize core: {}", e),
                    )),
                )
                    .into_response()
            }
        }
    }
}
