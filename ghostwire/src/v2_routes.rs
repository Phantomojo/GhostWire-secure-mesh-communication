// V2 Routes Implementation
// Implements the missing /api/v2/* endpoints

use crate::api_v2_types::ApiV2Response;
use crate::core::mesh_health::build_snapshot;
use crate::web::AppState;
use axum::{extract::State, Json};
use std::sync::Arc;

/// GET /api/v2/system/mesh-health
/// Returns comprehensive mesh health metrics
pub async fn get_mesh_health(
    State(state): State<Arc<AppState>>,
) -> impl axum::response::IntoResponse {
    // Get WebSocket subscriber count
    let ws_count = 0; // TODO: Track actual subscriber count

    // Get core reference
    let core_guard = state.core.read().await;

    // Build health snapshot from core
    let snapshot = build_snapshot(&core_guard, ws_count).await;

    Json(ApiV2Response::success(snapshot))
}

/// GET /api/v2/security/dashboard
/// Returns security metrics and threat assessment
pub async fn get_security_dashboard(
    State(state): State<Arc<AppState>>,
) -> impl axum::response::IntoResponse {
    let core = state.core.read().await;

    // Get security stats from SecurityManager
    let sec_manager = &core.security_manager;
    let stats = sec_manager.get_stats().await;

    // Get identity lock status
    let identity_locked = {
        let identity = core.identity.read().await;
        identity.is_locked()
    };

    // Build dashboard from available metrics
    let dashboard = SecurityDashboard {
        security_score: stats.peers_verified as f32 / 100.0 * 100.0,
        threat_level: format!("{:?}", stats.threat_level),
        blocked_connections: stats.blocked_connections,
        failed_attempts: stats.rate_limit_violations,
        sybil_rejections: 0, // TODO: Track from security manager
        audit_events_24h: stats.total_events,
        identity_locked,
        encryption_active: true,
        firewall_active: sec_manager.is_blacklist_enabled().await,
    };

    Json(ApiV2Response::success(dashboard))
}

/// Security dashboard response structure
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
pub struct SecurityDashboard {
    pub security_score: f32,
    pub threat_level: String,
    pub blocked_connections: u64,
    pub failed_attempts: u32,
    pub sybil_rejections: u32,
    pub audit_events_24h: u64,
    pub identity_locked: bool,
    pub encryption_active: bool,
    pub firewall_active: bool,
}
