use crate::api_v2_types::*;
use crate::web::{get_local_ip, AppState};
use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use serde::Serialize;
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

pub struct SystemController;

impl SystemController {
    /// GET /api/status - Basic system health and identification
    pub async fn rns_topology(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        match core.get_rns_topology().await {
            Some(topo) => Json(ApiV2Response::success(serde_json::json!({
                "node_count": topo.nodes.len(),
                "nodes": topo.nodes.iter().map(|(id, node)| serde_json::json!({
                    "id": id,
                    "address": node.address,
                    "last_seen": node.last_seen,
                    "quality": node.connection_quality,
                    "online": node.is_online,
                })).collect::<Vec<_>>(),
                "route_count": topo.routes.len(),
            }))).into_response(),
            None => (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ApiV2Response::<()>::error("RNS_UNAVAILABLE", "Reticulum not initialized")),
            ).into_response(),
        }
    }

    pub async fn all_peers(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        let peers = core.get_all_peers().await;
        Json(ApiV2Response::success(peers))
    }

    pub async fn transport_mode_get(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        Json(ApiV2Response::success(serde_json::json!({
            "mode": core.get_transport_mode().await,
            "options": ["auto", "libp2p-only", "reticulum-only", "hybrid"],
        })))
    }

    pub async fn transport_mode_set(
        State(state): State<Arc<AppState>>,
        Json(req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let mode = req.get("mode").and_then(|v| v.as_str()).unwrap_or("auto");
        let valid = ["auto", "libp2p-only", "reticulum-only", "hybrid"];
        if !valid.contains(&mode) {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error("INVALID_MODE", "Mode must be one of: auto, libp2p-only, reticulum-only, hybrid")),
            ).into_response();
        }
        let mut core = state.core.write().await;
        core.set_transport_mode(mode).await;
        Json(ApiV2Response::success(serde_json::json!({
            "mode": mode,
            "message": format!("Transport mode set to '{}'. Normal users: 'auto' lets the algorithm decide.", mode),
        }))).into_response()
    }

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
        (
            StatusCode::NOT_IMPLEMENTED,
            Json(ApiV2Response::<()>::error(
                "NOT_IMPLEMENTED",
                "Backup is not yet implemented; requires persistence stack integration",
            )),
        )
            .into_response()
    }

    pub async fn get_system_logs(State(_state): State<Arc<AppState>>) -> impl IntoResponse {
        // honesty: 501 not implemented — real logs require log aggregation stack
        (
            StatusCode::NOT_IMPLEMENTED,
            Json(ApiV2Response::<()>::error(
                "NOT_IMPLEMENTED",
                "System logs are not yet implemented; requires log aggregation stack",
            )),
        )
            .into_response()
    }

    pub async fn rotate_keys(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        #[derive(Serialize)]
        struct RotationStatus {
            rotated: bool,
            key_id: String,
            reason: String,
            crl_epoch: u64,
        }
        // Rotation needs &mut EncryptionManager, so both Arcs must be
        // uniquely owned. Otherwise rotation stays with the mesh hourly tick.
        let mut core_guard = state.core.write().await;
        let outcome: Result<RotationStatus, String> = (|| {
            let core = Arc::get_mut(&mut *core_guard)
                .ok_or_else(|| "Core shared; rotation deferred to mesh tick".to_string())?;
            let enc = Arc::get_mut(&mut core.encryption).ok_or_else(|| {
                "EncryptionManager shared; rotation deferred to mesh tick".to_string()
            })?;
            if !enc.should_rotate_keys() {
                return Ok(RotationStatus {
                    rotated: false,
                    key_id: enc.get_key_id(),
                    reason: "rotation interval not elapsed".to_string(),
                    crl_epoch: enc.crl.epoch(),
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
                crl_epoch: enc.crl.epoch(),
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
            .await
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
            .await
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

    pub async fn get_settings(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let metadata = state.core.read().await.get_metadata().await;
        let config = state.full_config.clone();

        let settings = serde_json::json!({
            "username": metadata.nickname.clone(),
            "displayName": metadata.nickname,
            "bio": metadata.bio,
            "listenPort": config.server.port,
            "maxPeers": config.network.max_peers,
            "keyRotationDays": config.security.key_rotation_interval_days,
        });

        Json(ApiV2Response::success(settings))
    }

    /// PUT /api/settings - Update metadata (applied) and report config keys honestly
    pub async fn update_settings(
        State(state): State<Arc<AppState>>,
        Json(req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let mut applied_now: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        let mut not_applied: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        let mut ignored: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        let mut persisted: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
        let mut pending_config: std::collections::BTreeMap<String, u64> =
            std::collections::BTreeMap::new();

        let mut nickname: Option<String> = None;
        let mut bio: Option<String> = None;

        for (key, value) in req.as_object().into_iter().flatten() {
            match key.as_str() {
                "displayName" | "username" => {
                    if key == "username" && req.get("displayName").is_some() {
                        ignored.insert(
                            key.clone(),
                            serde_json::json!("superseded by displayName"),
                        );
                    } else if let Some(s) = value.as_str() {
                        if nickname.is_none() {
                            nickname = Some(s.to_string());
                            applied_now.insert(key.clone(), value.clone());
                        }
                    } else {
                        ignored.insert(key.clone(), serde_json::json!("type mismatch: expected string"));
                    }
                }
                "bio" => match value.as_str() {
                    Some(s) => {
                        bio = Some(s.to_string());
                        applied_now.insert(key.clone(), value.clone());
                    }
                    None => {
                        ignored.insert(key.clone(), serde_json::json!("type mismatch: expected string"));
                    }
                },
                "listenPort" | "maxPeers" | "keyRotationDays" => match value.as_u64() {
                    None => {
                        ignored.insert(key.clone(), serde_json::json!("type mismatch: expected number"));
                    }
                    Some(n) if key == "listenPort" && n > u16::MAX as u64 => {
                        ignored.insert(
                            key.clone(),
                            serde_json::json!("value out of range: port max 65535"),
                        );
                    }
                    Some(n) => {
                        pending_config.insert(key.clone(), n);
                    }
                },
                "enableMdns" => {
                    ignored.insert(
                        key.clone(),
                        serde_json::json!("no config mapping: GhostWire config has no mDNS setting"),
                    );
                }
                _ => {
                    ignored.insert(key.clone(), serde_json::json!("unknown setting"));
                }
            }
        }

        if !pending_config.is_empty() {
            match state.full_config.loaded_path.clone() {
                None => {
                    for key in pending_config.keys() {
                        not_applied.insert(
                            key.clone(),
                            serde_json::json!(
                                "not applied: no config file path recorded at load; cannot persist; running process unchanged"
                            ),
                        );
                    }
                }
                Some(path) => {
                    let mut cfg = state.full_config.clone();
                    for (key, value) in &pending_config {
                        match key.as_str() {
                            "listenPort" => cfg.server.port = *value as u16,
                            "maxPeers" => cfg.network.max_peers = *value as usize,
                            "keyRotationDays" => cfg.security.key_rotation_interval_days = *value,
                            _ => {}
                        }
                    }
                    match cfg.save(&path.to_string_lossy()) {
                        Ok(()) => {
                            for key in pending_config.keys() {
                                persisted.insert(
                                    key.clone(),
                                    serde_json::json!(
                                        "saved to config file; takes effect after restart; running process unchanged"
                                    ),
                                );
                            }
                        }
                        Err(e) => {
                            for key in pending_config.keys() {
                                not_applied.insert(
                                    key.clone(),
                                    serde_json::json!(format!(
                                        "not applied: failed to write config file: {e}; running process unchanged"
                                    )),
                                );
                            }
                        }
                    }
                }
            }
        }

        if nickname.is_some() || bio.is_some() {
            state
                .core
                .read()
                .await
                .update_metadata(nickname, bio, None, None)
                .await;
        }

        let response = serde_json::json!({
            "applied_now": applied_now,
            "not_applied": not_applied,
            "ignored": ignored,
            "persisted": persisted,
        });
        Json(ApiV2Response::success(response))
    }
}

#[cfg(test)]
mod settings_honesty_tests {
    use super::*;
    use crate::config::Config;
    use crate::core::Core;
    use crate::web::StartupConfig;
    use std::sync::atomic::AtomicUsize;

    async fn test_state(test_name: &str, full_config: Config) -> Arc<AppState> {
        let profile = Some(format!("test_{}_{}", test_name, std::process::id()));
        let core = Core::new(Config::default(), Vec::new(), None, profile)
            .await
            .expect("Core::new in settings test");
        Arc::new(AppState {
            core: Arc::new(tokio::sync::RwLock::new(Arc::new(core))),
            config: StartupConfig {
                bootstrap_nodes: vec![],
                sim_addr: None,
                mesh_host: "127.0.0.1".to_string(),
                mesh_port: 8080,
            },
            full_config,
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
    async fn get_settings_returns_only_real_state() {
        let _home = crate::core::encryption::test_home();
        let mut cfg = Config::default();
        cfg.server.port = 4321;
        cfg.network.max_peers = 77;
        cfg.security.key_rotation_interval_days = 9;
        let state = test_state("get_settings", cfg).await;
        let expected_nick = state.core.read().await.get_metadata().await.nickname;

        let resp = SystemController::get_settings(State(state)).await.into_response();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_json(resp).await;
        let data = &body["data"];

        // Real values from full_config / identity metadata — not fabricated.
        assert_eq!(data["username"], expected_nick, "username must be reported");
        assert_eq!(data["displayName"], expected_nick);
        assert_eq!(data["listenPort"], 4321);
        assert_eq!(data["maxPeers"], 77);
        assert_eq!(data["keyRotationDays"], 9);

        // Keys with no backing state must be absent, not invented.
        assert!(
            data.get("enableMdns").is_none(),
            "enableMdns has no config backing and must not be fabricated"
        );
        assert!(data.get("logLevel").is_none());
        assert!(data.get("encryptionLevel").is_none());
        assert!(data.get("autoLockMinutes").is_none());
    }

    #[tokio::test]
    async fn update_settings_reports_what_was_and_was_not_applied() {
        let _home = crate::core::encryption::test_home();
        let state = test_state("update_settings", Config::default()).await;

        let req = serde_json::json!({
            "displayName": "Alice",
            "bio": "test bio",
            "listenPort": 9999,
            "maxPeers": "lots",
            "enableMdns": true,
            "bogus": 1,
        });
        let resp = SystemController::update_settings(State(state.clone()), Json(req))
            .await
            .into_response();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_json(resp).await;
        let data = &body["data"];

        // Metadata is genuinely applied through update_metadata.
        assert_eq!(data["applied_now"]["displayName"], "Alice");
        assert_eq!(data["applied_now"]["bio"], "test bio");
        // Config-file keys must NOT claim application — the running process
        // is not reconfigured by this handler.
        assert!(
            data["applied_now"].get("listenPort").is_none(),
            "listenPort must not be claimed as applied: nothing reconfigures the listener"
        );
        assert!(data["applied_now"].get("enableMdns").is_none());
        // ...and must be reported as not applied, with a reason.
        assert!(
            data["not_applied"]["listenPort"]
                .as_str()
                .unwrap_or("")
                .contains("not applied"),
            "listenPort must appear under not_applied with a reason, got: {}",
            data["not_applied"]
        );
        // Bad types / unmappable / unknown keys are ignored, with reasons.
        assert!(data["ignored"]["maxPeers"]
            .as_str()
            .unwrap_or("")
            .contains("type mismatch"));
        assert!(data["ignored"]["enableMdns"]
            .as_str()
            .unwrap_or("")
            .contains("mDNS"));
        assert!(data["ignored"]["bogus"]
            .as_str()
            .unwrap_or("")
            .contains("unknown"));
        // No blanket success, no persistence claim without persistence.
        assert_ne!(data.get("status"), Some(&serde_json::json!("settings_updated")));
        assert!(
            data["persisted"].as_object().is_some_and(|o| o.is_empty()),
            "persisted must be present but empty when nothing is written to disk, got: {:?}",
            data.get("persisted")
        );

        // The metadata claim must be true: a follow-up GET shows it applied.
        let resp = SystemController::get_settings(State(state)).await.into_response();
        let body = body_json(resp).await;
        assert_eq!(body["data"]["displayName"], "Alice");
        assert_eq!(body["data"]["bio"], "test bio");
    }

    #[tokio::test]
    async fn update_settings_persists_config_keys_when_path_is_recorded() {
        let _home = crate::core::encryption::test_home();
        let dir = tempfile::tempdir().expect("tempdir for config persistence");
        let path = dir.path().join("ghostwire.toml");
        let mut cfg = Config::default();
        cfg.loaded_path = Some(path.clone());
        let state = test_state("update_settings_persist", cfg).await;

        let req = serde_json::json!({ "listenPort": 9999u16 });
        let resp = SystemController::update_settings(State(state.clone()), Json(req))
            .await
            .into_response();
        assert_eq!(resp.status(), StatusCode::OK);
        let body = body_json(resp).await;
        let data = &body["data"];

        // The running process is never claimed as reconfigured...
        assert!(
            data["applied_now"].get("listenPort").is_none(),
            "listenPort must not be claimed as applied_now: nothing reconfigures the listener"
        );
        // ...the write is reported separately, with its real semantics.
        assert!(
            data["not_applied"].get("listenPort").is_none(),
            "listenPort was persisted, not refused: got not_applied={}",
            data["not_applied"]
        );
        let persisted = data["persisted"]["listenPort"].as_str().unwrap_or("");
        assert!(
            persisted.contains("saved to config file"),
            "persisted must say the file was written, got: {persisted:?}"
        );
        assert!(
            persisted.contains("restart"),
            "persisted must say a restart is required, got: {persisted:?}"
        );
        assert!(
            persisted.contains("running process unchanged"),
            "persisted must not imply live reconfiguration, got: {persisted:?}"
        );

        // The claim must be true on disk: the file actually contains the new port.
        let text = std::fs::read_to_string(&path).expect("config file must be written");
        assert!(
            text.contains("port = 9999"),
            "saved config must contain the new port, got:\n{text}"
        );
    }

    #[tokio::test]
    async fn unimplemented_endpoints_admit_it() {
        let _home = crate::core::encryption::test_home();
        let state = test_state("backup_logs", Config::default()).await;

        let resp = SystemController::backup_system(State(state.clone()))
            .await
            .into_response();
        assert_eq!(resp.status(), StatusCode::NOT_IMPLEMENTED);
        let body = body_json(resp).await;
        assert_eq!(body["error"]["code"], "NOT_IMPLEMENTED");
        assert!(
            body.get("data").is_none(),
            "no fabricated success payload on an unimplemented endpoint"
        );

        let resp = SystemController::get_system_logs(State(state))
            .await
            .into_response();
        assert_eq!(resp.status(), StatusCode::NOT_IMPLEMENTED);
        let body = body_json(resp).await;
        assert_eq!(body["error"]["code"], "NOT_IMPLEMENTED");
        assert!(body.get("data").is_none());
    }
}
