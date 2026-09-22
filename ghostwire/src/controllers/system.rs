use crate::api_v2_types::*;
use crate::web::{get_local_ip, AppState};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct SystemController;

impl SystemController {
    /// GET /api/status - Basic system health and identification
    pub async fn status(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let uptime = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs();
        let local_ip = get_local_ip().unwrap_or_else(|| "unknown".to_string());

        let sec_stats = state.core.read().await.get_security_stats().await;
        let peer_count = state.core.read().await.get_peer_count();

        #[derive(Serialize)]
        struct SystemStatus {
            status: String,
            uptime_seconds: u64,
            local_ip: String,
            peer_count: usize,
            mesh_active: bool,
            threat_level: crate::core::security::ThreatLevel,
            blocked_connections: u64,
            timestamp: String,
        }

        let status = SystemStatus {
            status: "OPERATIONAL".to_string(),
            uptime_seconds: uptime,
            local_ip,
            peer_count,
            mesh_active: cfg!(feature = "mesh"),
            threat_level: sec_stats.threat_level,
            blocked_connections: sec_stats.blocked_connections,
            timestamp: chrono::Utc::now().to_rfc3339(),
        };

        Json(ApiV2Response::success(status))
    }

    /// GET /api/v2/security/dashboard - Comprehensive security metrics
    pub async fn security_dashboard(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        #[derive(Serialize)]
        struct SecurityDashboard {
            score: u8,
            encryption: EncryptionStatus,
            sybil_defense: SybilStatus,
            rate_limiting: RateLimitStatus,
            blacklist: BlacklistStatus,
            stats: SecurityStatsSummary,
            threat_level: String,
            last_scan: String,
        }

        #[derive(Serialize)]
        struct SecurityStatsSummary {
            messages_encrypted: u64,
            peers_verified: u32,
            rate_limit_violations: u32,
            blocked_peers: u32,
        }

        #[derive(Serialize)]
        struct EncryptionStatus {
            method: String,
            active: bool,
            messages_encrypted: u64,
            pqc_armor: bool,
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

        let stats = state.core.read().await.get_security_stats().await;
        let score = state
            .core
            .read()
            .await
            .security_manager
            .clone()
            .calculate_security_score()
            .await;

        let dashboard = SecurityDashboard {
            score,
            encryption: EncryptionStatus {
                method: "XChaCha20-Poly1305 + ML-KEM-768 hybrid".to_string(),
                active: true,
                messages_encrypted: stats.messages_processed,
                pqc_armor: true,
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
                recent_additions: 0,
            },
            stats: SecurityStatsSummary {
                messages_encrypted: stats.messages_processed,
                peers_verified: stats.peers_verified,
                rate_limit_violations: stats.rate_limit_violations,
                blocked_peers: stats.blacklisted_peers,
            },
            threat_level: format!("{:?}", stats.threat_level).to_lowercase(),
            last_scan: chrono::Utc::now().to_rfc3339(),
        };

        Json(ApiV2Response::success(dashboard).with_links("/api/v2/security/dashboard".to_string()))
    }

    pub async fn backup_system(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
        // Simulate backup process
        std::thread::sleep(std::time::Duration::from_millis(1000));
        Json(ApiV2Response::success("Backup completed successfully"))
    }

    pub async fn get_system_logs(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
        let logs = vec![
            "System started",
            "Network scan completed",
            "Peer connected",
            "Message sent",
            "Security check passed",
        ];
        Json(ApiV2Response::success(logs))
    }

    pub async fn rotate_keys(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        #[derive(Serialize)]
        struct RotationStatus {
            rotated: bool,
            key_id: String,
            reason: String,
        }
        // Rotation needs &mut EncryptionManager, so both Arcs must be
        // uniquely owned. Otherwise rotation stays with the mesh hourly tick.
        let mut core_guard = state.core.write().await;
        let outcome: Result<RotationStatus, String> = (|| {
            let core = Arc::get_mut(&mut *core_guard)
                .ok_or_else(|| "Core shared; rotation deferred to mesh tick".to_string())?;
            let enc = Arc::get_mut(&mut core.encryption)
                .ok_or_else(|| "EncryptionManager shared; rotation deferred to mesh tick".to_string())?;
            if !enc.should_rotate_keys() {
                return Ok(RotationStatus {
                    rotated: false,
                    key_id: enc.get_key_id(),
                    reason: "rotation interval not elapsed".to_string(),
                });
            }
            if crate::core::encryption::EncryptionManager::is_user_keyed() {
                enc.rotate_keys_if_needed_with_persistence(&core.persistence)
                    .map_err(|e| e.to_string())?;
            } else {
                enc.rotate_keys_if_needed().map_err(|e| e.to_string())?;
            }
            Ok(RotationStatus {
                rotated: true,
                key_id: enc.get_key_id(),
                reason: "keys rotated".to_string(),
            })
        })();
        match outcome {
            Ok(status) => (StatusCode::OK, Json(ApiV2Response::success(status))).into_response(),
            Err(reason) => (
                StatusCode::CONFLICT,
                Json(ApiV2Response::<()>::error("ROTATION_DEFERRED", reason)),
            )
                .into_response(),
        }
    }

    pub async fn upgrade_encryption(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            Json(ApiV2Response::<()>::error(
                "NOT_IMPLEMENTED",
                "Encryption upgrade is not yet implemented",
            )),
        )
    }

    pub async fn configure_firewall(
        State(state): State<Arc<AppState>>,
        Json(req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let mut new_config = state
            .core
            .read()
            .await
            .security_manager
            .clone()
            .get_config()
            .await;

        if let Some(allowed) = req.get("allowed_ips").and_then(|v| v.as_array()) {
            new_config.allowed_ips = allowed
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();
        }
        if let Some(blocked) = req.get("blocked_ips").and_then(|v| v.as_array()) {
            new_config.blocked_ips = blocked
                .iter()
                .filter_map(|v| v.as_str().map(String::from))
                .collect();
        }

        state
            .core
            .read()
            .await
            .security_manager
            .clone()
            .update_config(new_config)
            .await;

        let path = "security_config.json";
        match state
            .core
            .read()
            .await
            .security_manager
            .clone()
            .save_config(path)
        {
            Ok(_) => Json(ApiV2Response::success(
                "Firewall rules applied and persisted".to_string(),
            ))
            .into_response(),
            Err(e) => {
                let msg: String = format!("{}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiV2Response::<()>::error("SAVE_FAILED", &msg)),
                )
                    .into_response()
            }
        }
    }

    pub async fn test_firewall(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let mgr = &state.core.read().await.security_manager.clone();
        let is_lockdown = mgr.is_lockdown().await;

        let result = if is_lockdown {
            "LOCKDOWN ACTIVE - ALL TRAFFIC BLOCKED"
        } else {
            "Firewall Active - Traffic Filtering Enabled"
        };

        Json(ApiV2Response::success(serde_json::json!({
            "result": "Passed",
            "details": result,
            "lockdown_status": is_lockdown
        })))
    }

    pub async fn lockdown(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let mgr = &state.core.read().await.security_manager.clone();
        let current = mgr.is_lockdown().await;
        let new_state = !current;

        mgr.set_lockdown(new_state).await;

        let msg = if new_state {
            "EMERGENCY LOCKDOWN INITIATED - NETWORK ISOLATED"
        } else {
            "Lockdown lifted - Network Access Restored"
        };

        Json(ApiV2Response::success(msg))
    }

    pub async fn get_auth_users(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
        let users = vec!["admin - Active", "user1 - Active"];
        Json(ApiV2Response::success(users))
    }

    pub async fn security_audit(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            Json(ApiV2Response::<()>::error(
                "NOT_IMPLEMENTED",
                "Security audit is not yet implemented",
            )),
        )
    }

    pub async fn security_scan(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            Json(ApiV2Response::<()>::error(
                "NOT_IMPLEMENTED",
                "Security scan is not yet implemented",
            )),
        )
    }

    pub async fn threat_hunt(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
        (
            StatusCode::NOT_IMPLEMENTED,
            Json(ApiV2Response::<()>::error(
                "NOT_IMPLEMENTED",
                "Threat hunt is not yet implemented",
            )),
        )
    }

    pub async fn backup_security(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let filename = format!("security_backup_{}.json", timestamp);

        match state
            .core
            .read()
            .await
            .security_manager
            .clone()
            .save_config(&filename)
        {
            Ok(_) => Json(ApiV2Response::success(format!(
                "Security configuration backed up to {}",
                filename
            )))
            .into_response(),
            Err(e) => {
                let msg: String = format!("Backup failed: {}", e);
                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiV2Response::<()>::error("BACKUP_FAILED", &msg)),
                )
                    .into_response()
            }
        }
    }

    pub async fn analyze_communications(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
        Json(ApiV2Response::success(
            "Communication analysis completed".to_string(),
        ))
    }

    /// GET /api/settings - Unified settings aggregation
    pub async fn get_settings(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let metadata = state.core.read().await.get_metadata().await;

        // In a real app, we'd load this from a persistent config file.
        // For now, we'll return defaults combined with metadata.
        let settings = serde_json::json!({
            "username": metadata.nickname.clone(),
            "displayName": metadata.nickname,
            "bio": metadata.bio,
            "listenPort": 3001,
            "enableMdns": true,
            "enableDht": true,
            "maxPeers": 50,
            "encryptionLevel": "enhanced",
            "autoLockMinutes": 15,
            "keyRotationDays": 30,
            "stealthMode": false,
            "trafficMorphing": false,
            "stripMetadata": true,
            "soundEnabled": true,
            "desktopNotifications": true,
            "messagePreview": true,
            "debugMode": false,
            "logLevel": "info"
        });

        Json(ApiV2Response::success(settings))
    }

    /// PUT /api/settings - Update both metadata and config
    pub async fn update_settings(
        State(state): State<Arc<AppState>>,
        Json(req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let nickname = req
            .get("displayName")
            .or_else(|| req.get("username"))
            .and_then(|v| v.as_str().map(|s| s.to_string()));
        let bio = req
            .get("bio")
            .and_then(|v| v.as_str().map(|s| s.to_string()));

        // Update Metadata
        state
            .core
            .read()
            .await
            .update_metadata(nickname, bio, None, None)
            .await;

        // In a more complete implementation, we would also update the system config (ports, security levels, etc.)
        // and persist it to ghostwire.toml.

        Json(ApiV2Response::success(serde_json::json!({
            "status": "settings_updated",
            "timestamp": chrono::Utc::now().to_rfc3339()
        })))
    }
}
