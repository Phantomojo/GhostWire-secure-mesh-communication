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
        // Use the EFFECTIVE mesh manager, not `core.mesh_manager` directly.
        //
        // Core carries two slots: `mesh_manager`, populated at startup when the
        // mesh can be built without an unlocked profile, and
        // `late_mesh_manager`, populated by unlock. `effective_mesh_manager()`
        // checks both. Reading only the first reports "not initialized" on any
        // node whose mesh was installed by unlock — observed live as a node
        // with its swarm listening on 4201 while health claimed
        // mesh_enabled=false / "not initialized".
        //
        // Anything gating on this (readiness probes, an operator, bootstrap)
        // was told the node had no mesh when it had one.
        let (mesh_enabled, mesh_status) = match core.effective_mesh_manager().await {
            Some(_) => (true, "initialized".to_string()),
            None => {
                let identity_locked = core.identity.read().await.is_locked();
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
    pub async fn ready(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        let identity_locked = core.identity.read().await.is_locked();

        #[cfg(feature = "mesh")]
        let mesh_ok = core.mesh_manager.is_some();

        #[cfg(not(feature = "mesh"))]
        let mesh_ok = true;

        let ready = !identity_locked && mesh_ok;

        let message = if ready {
            "GhostWire is ready — identity unlocked and mesh operational"
        } else if identity_locked {
            "GhostWire not ready — identity is locked; unlock via POST /api/v2/profile/unlock"
        } else {
            "GhostWire not ready — mesh not initialized"
        };

        let status = if ready {
            StatusCode::OK
        } else {
            StatusCode::SERVICE_UNAVAILABLE
        };

        (
            status,
            Json(ApiV2Response::success(serde_json::json!({
                "ready": ready,
                "message": message
            }))),
        )
            .into_response()
    }

    /// GET /api/v2/live - Liveness check (is process alive?)
    pub async fn live() -> impl IntoResponse {
        Json(ApiV2Response::success("alive")).into_response()
    }
}

#[cfg(test)]
mod ready_tests {
    use super::*;
    use crate::config::Config;
    use crate::core::Core;
    use crate::web::StartupConfig;
    use std::sync::atomic::AtomicUsize;

    async fn state_with_core(test_name: &str) -> Arc<AppState> {
        let profile = Some(format!("test_{}_{}", test_name, std::process::id()));
        let core = Core::new(Config::default(), Vec::new(), None, profile)
            .await
            .expect("Core::new in ready test");
        Arc::new(AppState {
            core: Arc::new(tokio::sync::RwLock::new(Arc::new(core))),
            config: StartupConfig {
                bootstrap_nodes: vec![],
                sim_addr: None,
                mesh_host: "127.0.0.1".to_string(),
                mesh_port: 8080,
            },
            full_config: Config::default(),
            ws_count: Arc::new(AtomicUsize::new(0)),
        })
    }

    async fn body_json(resp: axum::response::Response) -> serde_json::Value {
        let bytes = axum::body::to_bytes(resp.into_body(), usize::MAX)
            .await
            .expect("read response body");
        serde_json::from_slice(&bytes).expect("response is JSON")
    }

    #[tokio::test]
    async fn ready_reports_true_with_200_when_identity_unlocked() {
        let _home = crate::core::encryption::test_home();
        let state = state_with_core("ready_unlocked").await;

        let resp = HealthController::ready(State(state.clone())).await.into_response();
        let status = resp.status();
        let body = body_json(resp).await;

        assert_eq!(status, StatusCode::OK);
        assert_eq!(body["data"]["ready"], serde_json::json!(true));
        assert!(body["data"]["message"].as_str().is_some());
    }

    #[tokio::test]
    async fn ready_reports_false_with_503_when_identity_locked() {
        let _home = crate::core::encryption::test_home();
        let state = state_with_core("ready_locked").await;

        {
            let core = state.core.read().await;
            // Identity::lock() refuses without a cached encrypted copy, so the
            // password save that production setup performs must happen first.
            core.identity
                .read()
                .await
                .save_to_disk("test-password-123")
                .expect("save identity with password");
            core.lock().await.expect("lock core");
        }

        let resp = HealthController::ready(State(state)).await.into_response();
        let status = resp.status();
        let body = body_json(resp).await;

        assert_eq!(
            status,
            StatusCode::SERVICE_UNAVAILABLE,
            "a readiness endpoint must carry the failure in the status code"
        );
        assert_eq!(body["data"]["ready"], serde_json::json!(false));
        assert!(
            body["data"]["message"]
                .as_str()
                .unwrap_or("")
                .contains("locked"),
            "message must name the blocking condition, got: {}",
            body["data"]["message"]
        );
    }
}
