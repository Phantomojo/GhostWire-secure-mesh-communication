use crate::api_v2_types::*;
use crate::core::Core;
use crate::web::{revoke_sessions, AppState};
use axum::{
    extract::{Json, State},
    http::StatusCode,
    response::{IntoResponse, Response},
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

/// Where the node currently sits in its identity lifecycle.
///
/// This is the SINGLE authority on whether a public (unauthenticated)
/// profile-lifecycle operation is allowed. It replaces three separate guards,
/// two of which were byte-identical copies of the same predicate, and the third
/// of which used a filesystem scan as a proxy for a fact already recorded in
/// identity metadata. See `ProfilesController::classify_authority`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProfileAuthority {
    /// Setup has never completed: the first-run wizard owns profile creation.
    FirstRun,
    /// An identity exists and is locked: the lock screen may manage profiles.
    LockedScreen,
    /// Fully set up and unlocked: profile changes are authenticated-only.
    Cockpit,
}

impl ProfileAuthority {
    /// May a PUBLIC endpoint perform a profile-lifecycle operation?
    ///
    /// The cockpit case is deliberately false. Once an identity is set up and
    /// unlocked, changing profiles must go through the authenticated routes
    /// (which carry a session and are re-checked there), never the public
    /// first-run / lock-screen endpoints.
    pub fn allows_public_lifecycle(self) -> bool {
        matches!(
            self,
            ProfileAuthority::FirstRun | ProfileAuthority::LockedScreen
        )
    }
}

impl ProfilesController {
    const ACTIVE_PROFILE_FILE: &'static str = ".active_profile";

    pub(crate) fn ghostwire_dir() -> std::path::PathBuf {
        if let Ok(ghost_home) = std::env::var("GHOSTWIRE_HOME") {
            std::path::PathBuf::from(ghost_home)
        } else {
            dirs::home_dir()
                .unwrap_or_else(|| std::path::PathBuf::from("."))
                .join(".ghostwire")
        }
    }

    pub fn load_active_profile() -> Option<String> {
        let path = Self::ghostwire_dir().join(Self::ACTIVE_PROFILE_FILE);
        let value = fs::read_to_string(path).ok()?;
        let value = value.trim();
        if value.is_empty() || value == "default" {
            None
        } else if value
            .chars()
            .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            Some(value.to_string())
        } else {
            None
        }
    }

    fn persist_active_profile(profile_name: &str) -> std::io::Result<()> {
        fs::write(
            Self::ghostwire_dir().join(Self::ACTIVE_PROFILE_FILE),
            format!("{profile_name}\n"),
        )
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

    /// Derive the current lifecycle authority from authoritative identity state.
    ///
    /// This replaces three separate guards:
    ///
    ///   1. `create_first`  -> `if setup_complete && !locked { 403 }`
    ///   2. `select_locked` -> `if setup_complete && !locked { 403 }`  (identical)
    ///   3. inside `select` -> a scan of the GhostWire directory for any
    ///      `identity*.json`, rejecting a profile switch on an unlocked node
    ///      with "Lock the active profile before selecting another profile".
    ///
    /// Guards 1 and 2 were byte-identical copies, which is duplication that
    /// drifts rather than defence in depth. Guard 3 was the actual bug: it
    /// used the presence of a file on disk as a proxy for "has setup already
    /// happened", but `create_first` WRITES that file, so creating a profile
    /// guaranteed the very next select would be refused. On a fresh node
    /// (`setup_complete == false`) there is no lock available to satisfy it, so
    /// the first-run flow could never complete and every retry failed
    /// identically. The UI looked broken with no way forward.
    ///
    /// The filesystem scan was never a security boundary - it was a redundant,
    /// less accurate restatement of `setup_complete`, which is exactly the
    /// condition guards 1 and 2 already used. Deriving authority from
    /// `setup_complete` + `is_locked` alone is strictly more precise, and the
    /// authenticated cockpit path keeps its own separate checks.
    pub async fn classify_authority(state: &Arc<AppState>) -> ProfileAuthority {
        let core = state.core.read().await;
        let setup_complete = core.get_metadata().await.setup_complete;
        let locked = core.is_locked().await;
        Self::authority_for(setup_complete, locked)
    }

    /// Pure form of [`Self::classify_authority`], so the decision table is
    /// unit-testable without a running Core.
    pub fn authority_for(setup_complete: bool, locked: bool) -> ProfileAuthority {
        if !setup_complete {
            ProfileAuthority::FirstRun
        } else if locked {
            ProfileAuthority::LockedScreen
        } else {
            ProfileAuthority::Cockpit
        }
    }

    /// Public first-run profile creation. This is intentionally separate from
    /// the authenticated Settings endpoint and only works before setup is
    /// complete, or while the active configured identity is locked.
    pub async fn create_first(
        State(state): State<Arc<AppState>>,
        Json(req): Json<CreateProfileRequest>,
    ) -> Response {
        if !Self::classify_authority(&state)
            .await
            .allows_public_lifecycle()
        {
            return (
                StatusCode::FORBIDDEN,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::Unauthorized.as_str(),
                    "Create a profile from Settings while the active cockpit is unlocked",
                )),
            )
                .into_response();
        }
        Self::create(State(state), Json(req)).await.into_response()
    }

    /// Locked-screen profile selection. This endpoint is only available before
    /// the active node has completed setup; authenticated profile switching
    /// remains on the normal `/api/profile/select` route.
    pub async fn select_locked(
        State(state): State<Arc<AppState>>,
        Json(req): Json<SelectProfileRequest>,
    ) -> Response {
        if !Self::classify_authority(&state)
            .await
            .allows_public_lifecycle()
        {
            return (
                StatusCode::FORBIDDEN,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::Unauthorized.as_str(),
                    "Locked-screen profile selection requires a locked daemon",
                )),
            )
                .into_response();
        }
        Self::select_locked_inner(State(state), Json(req))
            .await
            .into_response()
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
        if req.name.is_empty()
            || req.name.len() > 48
            || !req
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
        if let Some(nickname) = &req.nickname {
            if nickname.trim().is_empty() || nickname.len() > 96 {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiV2Response::<()>::error(
                        StandardErrorCode::InvalidInput.as_str(),
                        "Nickname must be between 1 and 96 characters",
                    )),
                )
                    .into_response();
            }
        }
        tracing::info!(
            target: "profile_creation",
            msg = "🔧 Creating profile",
            profile_name = %req.name,
            with_setup = req.nickname.is_some() && req.password.is_some(),
        );

        let result = if let (Some(nickname), Some(password)) = (&req.nickname, &req.password) {
            // Validate password length in bytes, matching every interface.
            if password.as_bytes().len() < 12 {
                tracing::warn!(
                    target: "profile_creation",
                    msg = "❌ Password too short",
                    password_length = password.len(),
                    minimum_length = 12
                );
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiV2Response::<()>::error(
                        StandardErrorCode::InvalidInput.as_str(),
                        "Passphrase must be at least 12 bytes when provided",
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

    /// Emergency wipe with staged detach, deletion, and post-delete verification.
    pub async fn wipe_all(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        tracing::warn!(msg = "EMERGENCY WIPE REQUESTED");
        revoke_sessions();
        {
            let core = state.core.read().await.clone();
            if let Err(error) = core.shutdown().await {
                return (
                    StatusCode::SERVICE_UNAVAILABLE,
                    Json(ApiV2Response::<()>::error(
                        StandardErrorCode::InternalError.as_str(),
                        format!("Wipe aborted: active core shutdown failed: {}", error),
                    )),
                )
                    .into_response();
            }
        }

        let ghost_dir = Self::ghostwire_dir();
        match crate::core::secure_wipe::wipe_data_dir(&ghost_dir) {
            Ok(report) if report.verified => Json(ApiV2Response::success(report)).into_response(),
            Ok(report) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::InternalError.as_str(),
                    format!("Wipe incomplete; residuals: {:?}", report.residuals),
                )),
            )
                .into_response(),
            Err(error) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::InternalError.as_str(),
                    format!("Wipe failed: {}", error),
                )),
            )
                .into_response(),
        }
    }

    /// V2 Select Profile
    /// Authenticated profile selection (Settings screen / cockpit).
    ///
    /// This is the privileged path: the caller already holds a session and the
    /// active identity is unlocked, so a profile switch is permitted.
    pub async fn select(
        State(state): State<Arc<AppState>>,
        Json(req): Json<SelectProfileRequest>,
    ) -> impl IntoResponse {
        Self::select_with_authority(state, req, true).await
    }

    /// Public lock-screen path. Authority has ALREADY been checked by
    /// `select_locked` against `classify_authority`, so the redundant
    /// filesystem-based lock guard must not run again - that guard is what
    /// made first-run impossible.
    pub async fn select_locked_inner(
        State(state): State<Arc<AppState>>,
        Json(req): Json<SelectProfileRequest>,
    ) -> impl IntoResponse {
        Self::select_with_authority(state, req, false).await
    }

    async fn select_with_authority(
        state: Arc<AppState>,
        req: SelectProfileRequest,
        require_lock: bool,
    ) -> impl IntoResponse {
        if req.profile_name.is_empty()
            || !req
                .profile_name
                .chars()
                .all(|c| c.is_alphanumeric() || c == '_' || c == '-')
        {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::InvalidInput.as_str(),
                    "Invalid profile name. Use alphanumeric, _, - only",
                )),
            )
                .into_response();
        }
        let identity_path = if req.profile_name == "default" {
            Self::ghostwire_dir().join("identity.json")
        } else {
            Self::ghostwire_dir().join(format!("identity_{}.json", req.profile_name))
        };
        if !identity_path.is_file() {
            return (
                StatusCode::NOT_FOUND,
                Json(ApiV2Response::<()>::error(
                    StandardErrorCode::NotFound.as_str(),
                    "Profile does not exist",
                )),
            )
                .into_response();
        }
        let profile_name = if req.profile_name == "default" {
            None
        } else {
            Some(req.profile_name.clone())
        };

        // Selecting the already-active profile must be idempotent. Rebuilding
        // Core here would reopen the same sled vault while the current Core
        // still owns its file lock, producing a misleading WouldBlock error.
        let (current_profile, active_locked) = {
            let core = state.core.read().await;
            let current_profile = core
                .identity
                .read()
                .await
                .profile
                .clone()
                .unwrap_or_else(|| "default".to_string());
            let active_locked = core.is_locked().await;
            (current_profile, active_locked)
        };
        if current_profile == req.profile_name {
            return Json(ApiV2Response::success(serde_json::json!({
                "status": "profile_already_selected",
                "profile": req.profile_name,
            })))
            .into_response();
        }
        // The lock requirement applies ONLY to the privileged cockpit path.
        //
        // The public lock-screen path (`select_locked`) has already been
        // authorised by `classify_authority`, which is derived from
        // authoritative identity state. Re-applying the old filesystem scan
        // here is what dead-ended first-run: `create_first` writes
        // identity_<name>.json, so the scan always found a "persisted" profile
        // and refused the select that immediately followed, with no lock
        // available to satisfy it. The scan was a redundant, less accurate
        // restatement of `setup_complete` - see `classify_authority`.
        if require_lock && !active_locked {
            // An unlocked active session means there is a live vault that must
            // not be double-opened, so switching profiles requires the lock
            // (fail-closed).
            let any_persisted = fs::read_dir(Self::ghostwire_dir())
                .map(|entries| {
                    entries.filter_map(|e| e.ok()).any(|entry| {
                        entry
                            .file_name()
                            .to_str()
                            .map(|f| {
                                (f.starts_with("identity_") && f.ends_with(".json"))
                                    || f == "identity.json"
                            })
                            .unwrap_or(false)
                    })
                })
                .unwrap_or(false);
            if any_persisted {
                return (
                    StatusCode::CONFLICT,
                    Json(ApiV2Response::<()>::error(
                        StandardErrorCode::ProfileLoadFailed.as_str(),
                        "Lock the active profile before selecting another profile",
                    )),
                )
                    .into_response();
            }
        }

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
                let mesh_port = state.config.mesh_port;
                let core_for_tasks = core_arc.clone();
                tokio::spawn(async move {
                    core_for_tasks
                        .start_nervous_system(mesh_host, mesh_port)
                        .await;
                });

                revoke_sessions();
                if let Err(e) = Self::persist_active_profile(&req.profile_name) {
                    tracing::error!(error = %e, "Failed to persist active profile");
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiV2Response::<()>::error(
                            StandardErrorCode::InternalError.as_str(),
                            "Profile switched but active profile persistence failed",
                        )),
                    )
                        .into_response();
                }
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

#[cfg(test)]
mod authority_tests {
    use super::{ProfileAuthority, ProfilesController};

    /// The complete decision table. Both facts come from authoritative identity
    /// state (metadata + lock), never from a filesystem scan.
    #[test]
    fn authority_truth_table() {
        use ProfileAuthority::*;

        // Fresh node: setup has never run, nothing to protect.
        assert_eq!(ProfilesController::authority_for(false, false), FirstRun);
        // Fresh node that reports locked (no identity yet) is still first-run.
        assert_eq!(ProfilesController::authority_for(false, true), FirstRun);

        // Set up but locked: the lock screen may manage profiles.
        assert_eq!(ProfilesController::authority_for(true, true), LockedScreen);

        // Set up and unlocked: authenticated cockpit only.
        assert_eq!(ProfilesController::authority_for(true, false), Cockpit);
    }

    /// The cockpit case must NEVER allow a public lifecycle operation. This is
    /// the security property the old duplicated guards existed to provide.
    #[test]
    fn cockpit_never_allows_public_lifecycle() {
        assert!(!ProfileAuthority::Cockpit.allows_public_lifecycle());
    }

    /// First-run and locked-screen are exactly the two states where the public
    /// first-run / lock-screen endpoints are legitimate.
    #[test]
    fn public_lifecycle_allowed_only_before_cockpit() {
        assert!(ProfileAuthority::FirstRun.allows_public_lifecycle());
        assert!(ProfileAuthority::LockedScreen.allows_public_lifecycle());
        assert!(!ProfileAuthority::Cockpit.allows_public_lifecycle());
    }

    /// REGRESSION for the dead-end: a fresh node creates a profile, which
    /// persists identity_<name>.json. The authority MUST NOT change just
    /// because a file appeared on disk - it is still FirstRun, so the
    /// following select is allowed and the first-run flow can complete.
    ///
    /// The old guard scanned the directory and returned 409 here, stranding
    /// the user with the profile created but never selected.
    #[test]
    fn creating_a_profile_does_not_escalate_authority() {
        let before = ProfilesController::authority_for(false, false);
        // Whatever the filesystem now looks like, the inputs are unchanged.
        let after = ProfilesController::authority_for(false, false);
        assert_eq!(before, after);
        assert_eq!(after, ProfileAuthority::FirstRun);
        assert!(
            after.allows_public_lifecycle(),
            "a fresh node must still be able to select the profile it just created"
        );
    }
}
