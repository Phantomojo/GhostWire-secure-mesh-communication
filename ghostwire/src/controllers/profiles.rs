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
    /// V2 List Profiles
    pub async fn list(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let home = dirs::home_dir().unwrap_or_else(|| std::path::PathBuf::from("."));
        let ghost_dir = home.join(".ghostwire");
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

    /// V2 Create Profile
    pub async fn create(
        State(_state): State<Arc<AppState>>,
        Json(req): Json<CreateProfileRequest>,
    ) -> impl IntoResponse {
        // Validate name (alphanumeric only for safety)
        if !req
            .name
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::InvalidInput.as_str(),
                    "Invalid profile name. Use alphanumeric, _, -",
                )),
            )
                .into_response();
        }

        // Check if exists
        let home = dirs::home_dir().unwrap();
        let path = home
            .join(".ghostwire")
            .join(format!("identity_{}.json", req.name));
        if path.exists() {
            return (
                StatusCode::CONFLICT,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::InvalidInput.as_str(),
                    "Profile already exists",
                )),
            )
                .into_response();
        }

        // ACTUALLY CREATE IT
        match crate::core::identity::core_identity::Identity::new(Some(req.name.clone())) {
            Ok(_) => Json(ApiV2Response::success(format!(
                "Profile '{}' created and initialized",
                req.name
            )))
            .into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::InternalError.as_str(),
                    format!("Failed to initialize identity: {}", e),
                )),
            )
                .into_response(),
        }
    }

    /// V2 Delete Profile
    pub async fn delete(
        State(state): State<Arc<AppState>>,
        axum::extract::Path(name): axum::extract::Path<String>,
    ) -> impl IntoResponse {
        // Cannot delete active profile
        let current = {
            let core = state.core.read().await;
            let identity = core.identity.read().await;
            identity.profile.clone()
        };

        if let Some(curr_name) = current {
            if curr_name == name {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiV2Response::<()>::error(
                        StandardErrorCode::InvalidInput.as_str(),
                        "Cannot delete active profile. Switch to another profile first.",
                    )),
                )
                    .into_response();
            }
        }

        // 1. Delete Identity JSON
        let home = dirs::home_dir().unwrap();
        let id_path = home
            .join(".ghostwire")
            .join(format!("identity_{}.json", name));
        if id_path.exists() {
            if let Err(e) = fs::remove_file(id_path) {
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

        // 2. Delete Vault (Sled DB)
        let vault_path = home.join(".ghostwire").join("vault").join(&name);
        if vault_path.exists() {
            if let Err(e) = fs::remove_dir_all(vault_path) {
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

        let home = dirs::home_dir().unwrap();
        let ghost_dir = home.join(".ghostwire");

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
