use crate::api_v2_types::*;
use crate::core::security_v2::{EntityType, ThreatLevel};
use crate::web::{current_session_epoch, AppState, Claims, JWT_SECRET};
use axum::http::StatusCode;
use axum::{extract::State, response::IntoResponse, Json};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{SystemTime, UNIX_EPOCH};

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
            locked: bool,
            has_identity: bool,
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
            locked: state.core.read().await.is_locked().await,
            has_identity: true,
            setup_complete: metadata.setup_complete,
        };

        Json(ApiV2Response::success(profile).with_links("/api/v2/profile".to_string()))
    }

    /// POST /api/identity/setup - Initialize secure profile (First Time Setup)
    pub async fn setup(
        State(state): State<Arc<AppState>>,
        Json(req): Json<SetupRequest>,
    ) -> impl IntoResponse {
        tracing::info!(target: "identity_setup", "🔐 Identity setup request for: {}", req.nickname);

        // Validate input
        if req.password.as_bytes().len() < 12 {
            tracing::warn!(target: "identity_setup", "❌ Passphrase too short: {} bytes", req.password.as_bytes().len());
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::InvalidInput.as_str(),
                    "Passphrase must be at least 12 bytes",
                )),
            )
                .into_response();
        }

        let core = state.core.read().await;

        tracing::info!(target: "identity_setup", "🔓 Checking first-run identity state");

        // Guard: setup must only be possible on a FRESH identity.
        //
        // The previous guard checked only `is_locked()`. A fresh guest daemon
        // has an UNLOCKED identity, so on an unauthenticated route (this path
        // is in the public allowlist in web.rs) any LAN client could POST an
        // arbitrary >=12-byte passphrase and have `save_to_disk` re-encrypt the
        // on-disk identity under a key THEY chose, receiving a valid admin JWT
        // in return. That is full identity takeover with no credential.
        //
        // `setup_complete` is the correct signal: it is set to true only by this
        // handler's own `update_metadata(.., Some(true))` call, and is already
        // used this way by `create_first` / `select_locked` in
        // controllers/profiles.rs. Checking both conditions preserves the
        // existing locked-identity behaviour.
        let already_set_up = core.get_metadata().await.setup_complete;
        if already_set_up || core.is_locked().await {
            return (
                StatusCode::CONFLICT,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::InvalidCredentials.as_str(),
                    "Identity setup already completed; unlock it or create a named profile",
                )),
            )
                .into_response();
        }

        // 1. Update Metadata (Nickname & Setup Complete flag)
        core.update_metadata(Some(req.nickname.clone()), None, None, Some(true))
            .await;

        // 2. Re-encrypt and Save Identity with NEW password
        if let Err(e) = core.save_to_disk(&req.password).await {
            tracing::error!(target: "identity_setup", "❌ Failed to save identity: {}", e);
            return (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::InternalError.as_str(),
                    format!("Failed to secure identity: {}", e),
                )),
            )
                .into_response();
        }

        tracing::info!(target: "identity_setup", "✅ Identity setup complete for {}", req.nickname);

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let claims = Claims {
            sub: core.get_identity_id().await,
            exp: (now + 60 * 60 * 24) as usize,
            session_epoch: current_session_epoch(),
        };
        let token = match jsonwebtoken::encode(
            &jsonwebtoken::Header::default(),
            &claims,
            &jsonwebtoken::EncodingKey::from_secret(JWT_SECRET.as_bytes()),
        ) {
            Ok(token) => token,
            Err(e) => {
                tracing::error!(target: "identity_setup", error = %e, "Failed to issue setup session");
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiV2Response::<()>::error(
                        StandardErrorCode::InternalError.as_str(),
                        "Identity secured but session issuance failed",
                    )),
                )
                    .into_response();
            }
        };
        Json(ApiV2Response::success(serde_json::json!({
            "status": "identity_secured",
            "nickname": req.nickname,
            "token": token
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

    /// POST /api/v2/profile/lock - Lock profile with audit logging
    pub async fn lock_profile(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let core = state.core.read().await;

        // Check if already locked
        if core.is_locked().await {
            return Json(ApiV2Response::success(serde_json::json!({
                "status": "already_locked",
                "timestamp": now
            })))
            .into_response();
        }

        // Lock the profile
        match core.lock().await {
            Ok(_) => {
                // Audit log the successful lock
                let record = crate::core::security_v2::SecurityAuditRecord {
                    event_type: "PROFILE_LOCK".to_string(),
                    entity_type: EntityType::Node,
                    entity_id: core.get_identity_id().await,
                    threat_level: ThreatLevel::Low,
                    timestamp: now,
                    description: "Profile locked successfully".to_string(),
                };
                core.audit_logger.log_event(&record);

                tracing::info!(target: "profile_lock", "✅ Profile locked for user: {}", core.get_identity_id().await);

                Json(ApiV2Response::success(serde_json::json!({
                    "status": "locked",
                    "timestamp": now
                })))
                .into_response()
            }
            Err(e) => {
                // Audit log the failed lock
                let record = crate::core::security_v2::SecurityAuditRecord {
                    event_type: "PROFILE_LOCK_FAILED".to_string(),
                    entity_type: EntityType::Node,
                    entity_id: core.get_identity_id().await,
                    threat_level: ThreatLevel::Medium,
                    timestamp: now,
                    description: format!("Profile lock failed: {}", e),
                };
                core.audit_logger.log_event(&record);

                tracing::error!(target: "profile_lock", "❌ Profile lock failed: {}", e);

                (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ApiV2Response::<()>::error(
                        StandardErrorCode::InternalError.as_str(),
                        format!("Failed to lock profile: {}", e),
                    )),
                )
                    .into_response()
            }
        }
    }

    /// POST /api/v2/profile/recover-legacy - Explicitly migrate a legacy
    /// machine-sidecar identity. This is intentionally separate from normal
    /// startup and must remain a local operator recovery action.
    pub async fn recover_legacy_profile(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        match core.recover_legacy_identity().await {
            Ok(()) => Json(ApiV2Response::success(serde_json::json!({
                "status": "recovered_and_migrated"
            })))
            .into_response(),
            Err(error) => (
                StatusCode::UNAUTHORIZED,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::InvalidCredentials.as_str(),
                    format!("Legacy identity recovery failed: {}", error),
                )),
            )
                .into_response(),
        }
    }
    /// POST /api/v2/profile/unlock - Unlock profile with rate limiting and audit logging
    pub async fn unlock_profile(
        State(state): State<Arc<AppState>>,
        Json(req): Json<UnlockProfileRequest>,
    ) -> impl IntoResponse {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let core = state.core.read().await;
        let identity_id = core.get_identity_id().await;

        // Check rate limiting - is this key locked out?
        if core
            .failed_attempt_tracker
            .is_locked_out(&identity_id)
            .await
        {
            // Audit log the lockout attempt
            let record = crate::core::security_v2::SecurityAuditRecord {
                event_type: "PROFILE_UNLOCK_LOCKOUT".to_string(),
                entity_type: EntityType::Node,
                entity_id: identity_id.clone(),
                threat_level: ThreatLevel::High,
                timestamp: now,
                description: "Account locked due to too many failed attempts".to_string(),
            };
            core.audit_logger.log_event(&record);

            tracing::warn!(target: "profile_unlock", "⚠️ Account locked out due to failed attempts: {}", identity_id);

            return (
                StatusCode::TOO_MANY_REQUESTS,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::RateLimited.as_str(),
                    "Too many failed attempts. Account temporarily locked.",
                )),
            )
                .into_response();
        }

        // Attempt to unlock
        match core.unlock(&req.password).await {
            Ok(_) => {
                // Reset failed attempts on successful unlock
                core.failed_attempt_tracker.reset(&identity_id).await;

                // Audit log the successful unlock
                let record = crate::core::security_v2::SecurityAuditRecord {
                    event_type: "PROFILE_UNLOCK".to_string(),
                    entity_type: EntityType::Node,
                    entity_id: identity_id.clone(),
                    threat_level: ThreatLevel::Low,
                    timestamp: now,
                    description: "Profile unlocked successfully".to_string(),
                };
                core.audit_logger.log_event(&record);

                tracing::info!(target: "profile_unlock", "✅ Profile unlocked for user: {}", identity_id);

                // Locked profiles skip MeshManager during Core construction. Build the
                // manager in-place after unlock instead of rebuilding Core; the existing
                // Core still owns the profile database lock and cannot be reconstructed
                // safely while the request is running.
                #[cfg(feature = "mesh")]
                {
                    if let Err(error) = core.initialize_mesh_after_unlock().await {
                        tracing::error!(target: "profile_unlock", error = %error, "Failed to initialize mesh after profile unlock");
                        return (
                            StatusCode::SERVICE_UNAVAILABLE,
                            Json(ApiV2Response::<()>::error(
                                StandardErrorCode::InternalError.as_str(),
                                "Profile unlocked but mesh initialization failed",
                            )),
                        )
                            .into_response();
                    }

                    let mesh_host = state.config.mesh_host.clone();
                    let mesh_port = state.config.mesh_port;
                    let listen_addr = format!("/ip4/{}/tcp/{}", mesh_host, mesh_port);
                    if let Err(error) = core.start_mesh(&listen_addr).await {
                        tracing::error!(target: "profile_unlock", error = %error, "Failed to start mesh listener after profile unlock");
                        return (
                            StatusCode::SERVICE_UNAVAILABLE,
                            Json(ApiV2Response::<()>::error(
                                StandardErrorCode::InternalError.as_str(),
                                "Profile unlocked but mesh listener failed to start",
                            )),
                        )
                            .into_response();
                    }
                }

                // Reticulum is an independent medium. Initialize it after the
                // profile is unlocked, but do not make a broken optional RNS
                // interface take down the working libp2p mesh.
                #[cfg(feature = "reticulum")]
                if core.config.network.reticulum_enabled {
                    if let Err(error) = core.init_reticulum().await {
                        tracing::warn!(target: "profile_unlock", error = %error, "Reticulum initialization degraded; libp2p mesh remains available");
                    }
                }

                let expiration = now + 60 * 60 * 24;
                let claims = Claims {
                    sub: identity_id.clone(),
                    exp: expiration as usize,
                    session_epoch: current_session_epoch(),
                };
                let token = match jsonwebtoken::encode(
                    &jsonwebtoken::Header::default(),
                    &claims,
                    &jsonwebtoken::EncodingKey::from_secret(JWT_SECRET.as_bytes()),
                ) {
                    Ok(token) => token,
                    Err(e) => {
                        tracing::error!(target: "profile_unlock", error = %e, "Failed to issue unlock session");
                        return (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ApiV2Response::<()>::error(
                                StandardErrorCode::InternalError.as_str(),
                                "Identity unlocked but session issuance failed",
                            )),
                        )
                            .into_response();
                    }
                };
                Json(ApiV2Response::success(serde_json::json!({
                    "status": "unlocked",
                    "timestamp": now,
                    "token": token
                })))
                .into_response()
            }
            Err(e) => {
                // Record failed attempt
                core.failed_attempt_tracker
                    .record_failure(&identity_id)
                    .await;

                // Audit log the failed unlock
                let record = crate::core::security_v2::SecurityAuditRecord {
                    event_type: "PROFILE_UNLOCK_FAILED".to_string(),
                    entity_type: EntityType::Node,
                    entity_id: identity_id.clone(),
                    threat_level: ThreatLevel::Medium,
                    timestamp: now,
                    description: format!("Profile unlock failed: {}", e),
                };
                core.audit_logger.log_event(&record);

                tracing::warn!(target: "profile_unlock", "❌ Profile unlock failed for {}: {}", identity_id, e);

                (
                    StatusCode::UNAUTHORIZED,
                    Json(ApiV2Response::<()>::error(
                        StandardErrorCode::InvalidCredentials.as_str(),
                        "Invalid password",
                    )),
                )
                    .into_response()
            }
        }
    }
}

#[derive(Deserialize)]
pub struct UnlockProfileRequest {
    pub password: String,
}

#[derive(Deserialize)]
pub struct ErrorReportRequest {
    pub error: String,
}
