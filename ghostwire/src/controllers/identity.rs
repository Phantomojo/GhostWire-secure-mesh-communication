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

/// Body for `POST /api/v2/identity/pair`.
#[derive(Deserialize)]
pub struct PairRequestBody {
    /// The peer's libp2p address, learned by meeting them.
    pub peer_id: String,
    /// One reachable address for that peer.
    pub multiaddr: String,
    /// Optional DID claim to verify `peer_id` against.
    #[serde(default)]
    pub expected_did: Option<String>,
}

impl IdentityController {
    /// Pair with a peer you have met, and dial it.
    ///
    /// This is the endpoint Option B depends on. `peer_id` + `multiaddr` are the
    /// two things you learn by meeting someone: their mesh address and one place
    /// to reach them. Submitting both registers the peer so DID resolution can
    /// find them, then dials.
    ///
    /// There was no HTTP path to this before. `MeshManager::add_address` and
    /// `dial_verified_peer` existed, and `/api/connect_peer` existed, but the
    /// last takes a `node_id` and sends a social-connect REQUEST — so two nodes
    /// had no way to be introduced at all. That is why the loopback DID proof
    /// ended at "No known peer matches DID", and why cross-machine discovery had
    /// nothing to test.
    ///
    /// `expected_did` is optional but strongly recommended: when supplied, the
    /// peer_id must actually hash to that DID, or the request is refused. That
    /// turns pairing into a verified statement of identity rather than a dial.
    pub async fn pair_peer(
        State(state): State<Arc<AppState>>,
        Json(req): Json<PairRequestBody>,
    ) -> impl IntoResponse {
        let peer_id = match req.peer_id.parse::<libp2p::PeerId>() {
            Ok(p) => p,
            Err(_) => {
                return ApiV2Response::<()>::error_response(
                    StandardErrorCode::InvalidInput.as_str(),
                    format!("peer_id is not a valid libp2p PeerId: {}", req.peer_id),
                )
                .into_response()
            }
        };

        // Identity check, when a DID was supplied. Refuse rather than dial: a
        // peer_id that does not match the claimed DID is either a mistake or an
        // attempt to pair the wrong node.
        if let Some(expected) = &req.expected_did {
            if !crate::core::deref::peer_id_matches_did(&peer_id, expected) {
                return ApiV2Response::<()>::error_response(
                    StandardErrorCode::InvalidInput.as_str(),
                    format!(
                        "peer_id does not match the claimed DID {expected}. Refusing                          to pair: either the wrong address was entered, or the                          peer is not who they claim."
                    ),
                )
                .into_response();
            }
        }

        let multiaddr = match req.multiaddr.parse::<libp2p::multiaddr::Multiaddr>() {
            Ok(m) => m,
            Err(e) => {
                return ApiV2Response::<()>::error_response(
                    StandardErrorCode::InvalidInput.as_str(),
                    format!("multiaddr is not valid: {e}"),
                )
                .into_response()
            }
        };

        let core = state.core.read().await;
        match core.pair_and_dial(peer_id, multiaddr).await {
            Ok(_) => {
                #[derive(Serialize)]
                struct PairResponse {
                    paired: bool,
                    peer_id: String,
                    did_verified: bool,
                }
                Json(ApiV2Response::success(PairResponse {
                    paired: true,
                    peer_id: peer_id.to_string(),
                    did_verified: req.expected_did.is_some(),
                }))
                .into_response()
            }
            Err(e) => ApiV2Response::<()>::error_response(
                StandardErrorCode::InternalError.as_str(),
                format!("Pairing failed: {e}"),
            )
            .into_response(),
        }
    }

    /// This node's own addresses.
    ///
    /// Returns the DID (canonical, shareable) and the libp2p PeerId (a delivery
    /// detail under Option B, disclosed only to peers you have paired with).
    ///
    /// This endpoint exists because nothing served the operator's own addresses.
    /// The PeerId was computed at mesh construction (mesh.rs:340) and discarded;
    /// `get_profile` returns the DID but no mesh address; and no route returned
    /// it at all. So a user could read their DID from `identity.json` on disk
    /// but had no way to see, copy, or share their mesh address from the app —
    /// which makes the node unaddressable and leaves DID resolution with nothing
    /// to present during pairing.
    ///
    /// Requires an unlocked profile: the keypair is only available then.
    pub async fn local_addresses(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;

        match core.local_addresses().await {
            Some((did, peer_id)) => {
                #[derive(Serialize)]
                struct AddressesResponse {
                    /// Canonical identifier. Safe to share publicly.
                    did: String,
                    /// Delivery detail. Share only when pairing with someone you
                    /// have met — publishing it alongside the DID in discovery
                    /// is the correlation Option B exists to prevent.
                    peer_id: String,
                    locked: bool,
                }
                Json(ApiV2Response::success(AddressesResponse {
                    did,
                    peer_id,
                    locked: false,
                }))
                .into_response()
            }
            None => {
                // Locked, or the mesh is not up. Say which — "no addresses" with
                // no reason is the kind of unhelpful 200 this codebase has been
                // removing.
                let locked = core.identity.read().await.is_locked();
                ApiV2Response::<serde_json::Value>::error_response(
                    StandardErrorCode::InvalidInput.as_str(),
                    if locked {
                        "Profile is locked — unlock before reading addresses"
                    } else {
                        "Mesh is not running, so no libp2p address exists yet"
                    },
                )
                .into_response()
            }
        }
    }

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
            /// `None` when the creation time was never recorded. Reported as null rather
            /// than filled in with the current time.
            created_at: Option<String>,
            dht_published: bool,
            security_score: u8,
            locked: bool,
            identity_locked: bool,
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
            created_at: state.core.read().await.get_created_at().await,
            dht_published: false,
            security_score,
            // `Core::is_locked()` reads the IDENTITY lock, which is a different
            // subsystem from the PERSISTENCE lock that actually refuses message
            // writes with "persistence is locked; unlock the profile first". A
            // node reported `locked: false` here while every write failed, so the
            // field now reports persistence readiness and the identity state is
            // surfaced separately rather than conflated under one name.
            locked: !state.core.read().await.persistence_is_unlocked().await,
            identity_locked: state.core.read().await.is_locked().await,
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

        // Build and start the mesh, exactly as `unlock_profile` does below.
        //
        // A daemon that starts with a user-keyed profile defers MeshManager
        // construction: `Core::new` sees "Encryption private state is user-keyed
        // and locked at startup" (encryption.rs:396) and logs "Identity is
        // locked. Deferring MeshManager initialization" followed by "Failed to
        // auto-start mesh: Mesh networking not available".
        //
        // `unlock_profile` recovers from that by calling
        // `initialize_mesh_after_unlock()` then `start_mesh()`. This handler did
        // NOT — so a node created through setup stayed meshless forever: the API
        // answered, `setup_complete` was true, the UI showed a working node, and
        // every send failed with "Recipient encryption key not ready" because no
        // peer could ever connect. Setup is the path a first-time user takes, so
        // this was the default experience.
        //
        // Done before the success response, so "identity_secured" now means the
        // node is actually usable. `save_to_disk` above persisted the key; this
        // makes the running process use it.
        #[cfg(feature = "mesh")]
        {
            if let Err(error) = core.initialize_mesh_after_unlock().await {
                tracing::error!(target: "identity_setup", error = %error, "Failed to initialize mesh after identity setup");
                return (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(ApiV2Response::<()>::error(
                        StandardErrorCode::InternalError.as_str(),
                        "Identity secured but mesh initialization failed",
                    )),
                )
                    .into_response();
            }

            let mesh_host = state.config.mesh_host.clone();
            let mesh_port = state.config.mesh_port;
            let listen_addr = format!("/ip4/{}/tcp/{}", mesh_host, mesh_port);
            if let Err(error) = core.start_mesh(&listen_addr).await {
                tracing::error!(target: "identity_setup", error = %error, "Failed to start mesh listener after identity setup");
                return (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(ApiV2Response::<()>::error(
                        StandardErrorCode::InternalError.as_str(),
                        "Identity secured but mesh listener failed to start",
                    )),
                )
                    .into_response();
            }
        }

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
            return ApiV2Response::<()>::error_response(
                "INVALID_USERNAME",
                "Username must be 1-32 characters",
            )
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
            /// When the key was generated, not when this request arrived. This
            /// was a second `chrono::Utc::now()` reported as a creation time.
            created_at: Option<String>,
        }

        Json(ApiV2Response::success(PublicKeyResponse {
            key_id: state.core.read().await.get_key_id(),
            algorithm: "Ed25519".to_string(),
            public_key: general_purpose::STANDARD.encode(state.core.read().await.get_public_key()),
            created_at: state.core.read().await.get_created_at().await,
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
                //
                // `hybrid_mode` is honoured here. It was declared in
                // config.rs:58 and read by NOTHING — so `libp2p-only` and
                // `reticulum-only` both silently ran libp2p, and an operator
                // could not restrict a node to one transport. Worse, nobody
                // could tell from a connection which transport carried it, which
                // makes the RNS discovery dial gate untestable.
                //
                // Resolution order: an explicit mode in config wins; if unset or
                // unrecognised, fall back to the two independent booleans so
                // existing behaviour is unchanged.
                #[cfg(feature = "reticulum")]
                {
                    let mode = core.config.network.hybrid_mode.trim().to_ascii_lowercase();
                    let want_rns = core.config.network.reticulum_enabled
                        && mode != "libp2p-only";
                    let want_libp2p = mode != "reticulum-only";

                    if !want_rns && core.config.network.reticulum_enabled {
                        tracing::info!(
                            target: "profile_unlock",
                            mode = %mode,
                            "hybrid_mode suppresses the Reticulum transport"
                        );
                    }
                    if want_rns {
                        if let Err(error) = core.init_reticulum().await {
                            tracing::warn!(target: "profile_unlock", error = %error, "Reticulum initialization degraded; libp2p mesh remains available");
                        }
                    }
                    if !want_libp2p {
                        tracing::info!(
                            target: "profile_unlock",
                            mode = %mode,
                            "hybrid_mode is reticulum-only: libp2p mesh is not started"
                        );
                        // Belt and braces: the daemon may already have a mesh
                        // from a previous unlock in the same process.
                        #[cfg(feature = "mesh")]
                        if let Some(mesh) = core.effective_mesh_manager().await {
                            if let Ok(mut mgr) = mesh.try_write() {
                                let _ = mgr.stop().await;
                            }
                        }
                    }
                    tracing::info!(
                        target: "profile_unlock",
                        mode = %if mode.is_empty() { "<unset>" } else { &mode },
                        want_libp2p,
                        want_rns,
                        "Active transports after profile unlock"
                    );
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

#[cfg(test)]
mod transport_mode_tests {
    /// Resolution of the two transport switches, mirroring the logic in
    /// `unlock_profile`.
    ///
    /// `hybrid_mode` was declared in config.rs:58 and read by nothing, so
    /// `libp2p-only` and `reticulum-only` both silently ran libp2p and an
    /// operator could not restrict a node to one transport. These pin the
    /// resolution so a later edit cannot quietly reintroduce that.
    fn resolve(reticulum_enabled: bool, mode: &str) -> (bool, bool) {
        let mode = mode.trim().to_ascii_lowercase();
        let want_rns = reticulum_enabled && mode != "libp2p-only";
        let want_libp2p = mode != "reticulum-only";
        (want_libp2p, want_rns)
    }

    #[test]
    fn hybrid_mode_suppresses_rns() {
        // The bug: reticulum_enabled = true plus hybrid_mode = "libp2p-only"
        // used to start RNS anyway, because nothing read hybrid_mode.
        let (libp2p, rns) = resolve(true, "libp2p-only");
        assert!(libp2p, "libp2p must still run");
        assert!(!rns, "hybrid_mode = libp2p-only must suppress Reticulum");
    }

    #[test]
    fn hybrid_mode_suppresses_libp2p() {
        let (libp2p, rns) = resolve(true, "reticulum-only");
        assert!(!libp2p, "hybrid_mode = reticulum-only must suppress libp2p");
        assert!(rns, "Reticulum must still start");
    }

    #[test]
    fn hybrid_mode_runs_both() {
        let (libp2p, rns) = resolve(true, "hybrid");
        assert!(libp2p);
        assert!(rns);
    }

    #[test]
    fn unset_mode_falls_back_to_the_booleans() {
        // Unset must behave exactly as before, so existing configs are
        // unaffected by this change.
        let (libp2p, rns) = resolve(false, "");
        assert!(libp2p, "libp2p is the default medium");
        assert!(!rns, "reticulum_enabled = false means no RNS");

        let (libp2p, rns) = resolve(true, "");
        assert!(libp2p);
        assert!(rns);
    }

    #[test]
    fn mode_is_case_and_whitespace_insensitive() {
        assert_eq!(
            resolve(true, "  LIBP2P-ONLY "),
            resolve(true, "libp2p-only")
        );
        assert_eq!(
            resolve(true, " Reticulum-Only"),
            resolve(true, "reticulum-only")
        );
    }

    #[test]
    fn unknown_mode_does_not_disable_anything() {
        // An operator typo must not silently turn off a transport.
        let (libp2p, rns) = resolve(true, "hybird");
        assert!(libp2p, "a typo must not disable libp2p");
        assert!(rns, "a typo must not disable RNS");
    }
}
