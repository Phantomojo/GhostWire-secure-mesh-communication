// API v2 Endpoints
use axum::{
    extract::{Query, State},
    response::IntoResponse,
    Json,
};
use base64::{engine::general_purpose, Engine as _};
use serde::Serialize;
use std::sync::Arc;

use crate::web::AppState;

use crate::api_v2_types::ApiV2Response;

/// GET /api/v2/profile - Full user profile/identity
pub async fn get_profile_v2(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    #[derive(Serialize)]
    struct ProfileResponse {
        id: String,
        did: String,
        username: String,
        nickname: String,
        bio: String,
        public_key: String,
        key_id: String,
        created_at: String,
        dht_published: bool,
        security_score: u8,
    }

    let metadata = state.core.get_metadata();

    let username = match std::fs::read_to_string("username.txt") {
        Ok(name) => name.trim().to_string(),
        Err(_) => "GhostUser".to_string(),
    };

    let profile = ProfileResponse {
        id: state.core.get_identity_id(),
        did: state.core.get_did(),
        username: username,
        nickname: metadata.nickname,
        bio: metadata.bio,
        public_key: general_purpose::STANDARD.encode(state.core.get_public_key()),
        key_id: state.core.get_key_id(),
        created_at: chrono::Utc::now().to_rfc3339(), // TODO: Get from storage
        dht_published: false,                        // TODO: Get from DHT status
        security_score: 95,                          // TODO: Calculate from security stats
    };

    let response = ApiV2Response::success(profile).with_links("/api/v2/profile".to_string());

    Json(response)
}

/// PUT /api/v2/profile/username - Update username
pub async fn update_username_v2(
    State(_state): State<Arc<AppState>>,
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

    // Save username
    if let Err(e) = std::fs::write("username.txt", username) {
        return Json(ApiV2Response::<()>::error(
            "SAVE_FAILED",
            format!("Failed to save username: {}", e),
        ))
        .into_response();
    }

    Json(ApiV2Response::success(UsernameUpdateResponse {
        username: username.to_string(),
        updated_at: chrono::Utc::now().to_rfc3339(),
    }))
    .into_response()
}

/// PUT /api/v2/profile/metadata - Update profile metadata
pub async fn update_metadata_v2(
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

    state.core.update_metadata(nickname, bio, avatar_url);

    Json(ApiV2Response::success(serde_json::json!({
        "status": "updated",
        "timestamp": chrono::Utc::now().to_rfc3339()
    })))
}

/// PUT /api/v2/security/armor - Toggle Post-Quantum Armor
pub async fn toggle_pqc_armor_v2(
    State(state): State<Arc<AppState>>,
    Json(req): Json<serde_json::Value>,
) -> impl IntoResponse {
    let enabled = req
        .get("enabled")
        .and_then(|v| v.as_bool())
        .unwrap_or(false);

    state.core.set_pqc_mode(enabled).await;

    Json(ApiV2Response::success(serde_json::json!({
        "pqc_armor": enabled,
        "mode": if enabled { "HYBRID_KYBER_768_ACTIVE" } else { "STANDARD_ECDH_ONLY" }
    })))
}

/// PUT /api/v2/mesh/sharding - Update DHT sharding region
pub async fn update_sharding_v2(
    State(state): State<Arc<AppState>>,
    Json(req): Json<serde_json::Value>,
) -> impl IntoResponse {
    let shard = req
        .get("shard")
        .and_then(|v| v.as_str())
        .unwrap_or("global");

    match state.core.set_dht_shard(shard).await {
        Ok(_) => Json(ApiV2Response::success(serde_json::json!({
            "shard": shard,
            "status": "switching"
        }))),
        Err(e) => Json(ApiV2Response::<serde_json::Value>::error(
            "SHARD_UPDATE_FAILED",
            e.to_string(),
        )),
    }
}

/// GET /api/v2/profile/public-key - Get public key with QR code
pub async fn get_public_key_v2(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    #[derive(Serialize)]
    struct PublicKeyResponse {
        key_id: String,
        algorithm: String,
        public_key: String,
        created_at: String,
    }

    Json(ApiV2Response::success(PublicKeyResponse {
        key_id: state.core.get_key_id(),
        algorithm: "Ed25519".to_string(),
        public_key: general_purpose::STANDARD.encode(state.core.get_public_key()),
        created_at: chrono::Utc::now().to_rfc3339(),
    }))
}

/// GET /api/v2/security/dashboard - Security dashboard stats
pub async fn get_security_dashboard_v2(State(state): State<Arc<AppState>>) -> impl IntoResponse {
    #[derive(Serialize)]
    struct SecurityDashboard {
        score: u8,
        encryption: EncryptionStatus,
        sybil_defense: SybilStatus,
        rate_limiting: RateLimitStatus,
        blacklist: BlacklistStatus,
        last_scan: String,
    }

    #[derive(Serialize)]
    struct EncryptionStatus {
        method: String,
        active: bool,
        messages_encrypted: u64,
    }

    #[derive(Serialize)]
    struct SybilStatus {
        method: String,
        difficulty: u8,
        verified_peers: u32,
    }

    #[derive(Serialize)]
    struct RateLimitStatus {
        active: bool,
        violations_today: u32,
    }

    #[derive(Serialize)]
    struct BlacklistStatus {
        entries: u32,
        recent_additions: u32,
    }

    // Get real security stats from SecurityManager
    let stats = state.core.get_security_stats().await;

    let dashboard = SecurityDashboard {
        score: 95, // TODO: Calculate from stats
        encryption: EncryptionStatus {
            method: "AES-256-GCM".to_string(),
            active: true,
            messages_encrypted: stats.messages_processed,
        },
        sybil_defense: SybilStatus {
            method: "Proof-of-Work".to_string(),
            difficulty: 16,
            verified_peers: stats.peers_verified,
        },
        rate_limiting: RateLimitStatus {
            active: true,
            violations_today: stats.rate_limit_violations,
        },
        blacklist: BlacklistStatus {
            entries: stats.blacklisted_peers,
            recent_additions: 0, // TODO: Track this
        },
        last_scan: chrono::Utc::now().to_rfc3339(),
    };

    Json(ApiV2Response::success(dashboard).with_links("/api/v2/security/dashboard".to_string()))
}

/// GET /api/v2/peers - Paginated peer list
pub async fn get_peers_v2(
    State(_state): State<Arc<AppState>>,
    Query(params): Query<std::collections::HashMap<String, String>>,
) -> impl IntoResponse {
    #[derive(Serialize)]
    struct PeerResponse {
        id: String,
        username: String,
        status: String,
        last_seen: String,
        trust_score: u8,
    }

    // Parse pagination params
    let page: usize = params.get("page").and_then(|p| p.parse().ok()).unwrap_or(1);
    let per_page: usize = params
        .get("per_page")
        .and_then(|p| p.parse().ok())
        .unwrap_or(20)
        .min(100); // Max 100 per page

    // Get peers (TODO: from actual peer manager)
    let all_peers = vec![]; // Placeholder
    let total = all_peers.len();

    // Paginate
    let start = (page - 1) * per_page;
    let _end = start + per_page;
    let peers: Vec<PeerResponse> = all_peers.into_iter().skip(start).take(per_page).collect();

    Json(
        ApiV2Response::success(peers)
            .with_pagination(total, page, per_page)
            .with_links(format!("/api/v2/peers?page={}&per_page={}", page, per_page)),
    )
}
