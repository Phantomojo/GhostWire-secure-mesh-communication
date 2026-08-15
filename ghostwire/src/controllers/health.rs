// Health check endpoint for GhostWire - Simplified version
// File: ghostwire/src/controllers/health.rs

use crate::api_v2_types::*;
use crate::web::AppState;
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Serialize, Deserialize, Debug)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub message: String,
    pub timestamp: String,
    #[cfg(feature = "mesh")]
    pub mesh_enabled: bool,
    #[cfg(feature = "mesh")]
    pub mesh_status: String,
}

pub struct HealthController;

impl HealthController {
    pub async fn health(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;

        #[cfg(feature = "mesh")]
        let (mesh_enabled, mesh_status) = match &*core.network_stack.mesh_manager.load() {
            Some(_) => (true, "initialized".to_string()),
            None => {
                let identity_locked = core.identity_stack.identity.read().await.is_locked();
                if identity_locked {
                    (
                        false,
                        "blocked: identity is locked — unlock via POST /api/v2/profile/unlock"
                            .to_string(),
                    )
                } else {
                    (false, "not initialized".to_string())
                }
            }
        };

        let response = HealthResponse {
            status: "healthy".to_string(),
            version: env!("CARGO_PKG_VERSION").to_string(),
            message: "GhostWire API is running".to_string(),
            timestamp: chrono::Utc::now().to_rfc3339(),
            #[cfg(feature = "mesh")]
            mesh_enabled,
            #[cfg(feature = "mesh")]
            mesh_status,
        };

        (StatusCode::OK, Json(ApiV2Response::success(response))).into_response()
    }

    /// GET /api/v2/ready - Readiness check
    pub async fn ready(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
        Json(ApiV2Response::success("ready")).into_response()
    }

    /// GET /api/v2/live - Liveness check (is process alive?)
    pub async fn live() -> impl IntoResponse {
        Json(ApiV2Response::success("alive")).into_response()
    }
}
