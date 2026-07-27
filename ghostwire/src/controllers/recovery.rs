//! Recovery Controller
//!
//! Handles social recovery via Shamir Secret Sharing.

use crate::api_v2_types::*;
use crate::web::AppState;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct SetupRecoveryRequest {
    pub threshold: u8,
    pub total: u8,
    pub guardians: Vec<GuardianInfo>,
}

#[derive(Deserialize)]
pub struct GuardianInfo {
    pub peer_id: String,
    pub name: String,
    pub x25519_pub_hex: String,
}

pub struct RecoveryController;

fn _now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

impl RecoveryController {
    pub async fn setup_shares(
        State(state): State<Arc<AppState>>,
        Json(req): Json<SetupRecoveryRequest>,
    ) -> impl IntoResponse {
        if req.threshold < 2 || req.total < req.threshold {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error(
                    "INVALID_PARAMS",
                    "threshold must be >= 2 and <= total",
                )),
            )
                .into_response();
        }

        // Build guardian list
        let mut guardians = Vec::new();
        for g in &req.guardians {
            let pub_bytes = match hex::decode(&g.x25519_pub_hex) {
                Ok(b) if b.len() == 32 => {
                    let mut arr = [0u8; 32];
                    arr.copy_from_slice(&b);
                    arr
                }
                _ => {
                    return (
                        StatusCode::BAD_REQUEST,
                        Json(ApiV2Response::<()>::error(
                            "INVALID_KEY",
                            &format!("Invalid X25519 key for guardian {}", g.name),
                        )),
                    )
                        .into_response()
                }
            };
            guardians.push((g.peer_id.clone(), g.name.clone(), pub_bytes));
        }

        // Get master key from identity (only works when unlocked)
        let core = state.core.read().await;
        if core.is_locked().await {
            return (
                StatusCode::UNAUTHORIZED,
                Json(ApiV2Response::<()>::error("LOCKED", "Unlock profile first")),
            )
                .into_response();
        }

        // Derive master key from identity's Ed25519 public key (32 bytes)
        let identity_guard = core.identity.read().await;
        let master_key = match identity_guard.public_key() {
            Some(pk) => match pk.try_into_ed25519() {
                Ok(ed_pk) => ed_pk.to_bytes().to_vec(),
                Err(_) => {
                    return (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiV2Response::<()>::error(
                            "RECOVERY_ERROR",
                            "Identity key type not supported",
                        )),
                    )
                        .into_response();
                }
            },
            None => {
                return (
                    StatusCode::UNAUTHORIZED,
                    Json(ApiV2Response::<()>::error(
                        "LOCKED",
                        "Identity not unlocked",
                    )),
                )
                    .into_response();
            }
        };
        drop(identity_guard);

        match crate::core::recovery_shamir::create_recovery_shares(
            &master_key,
            req.threshold,
            req.total,
            &guardians,
        ) {
            Ok((shares, metadata)) => {
                // Send each share to its guardian via messaging
                let mut sent_count = 0;
                for (i, share) in shares.iter().enumerate() {
                    if i < guardians.len() {
                        let guardian_peer_id = &guardians[i].0;
                        let share_json = serde_json::to_string(share).unwrap_or_default();
                        let msg = crate::core::message::Message::new(
                            core.get_identity_id().await,
                            guardian_peer_id.clone(),
                            format!("RECOVERY_SHARE:{}", share_json),
                        );
                        // Best-effort send
                        let _ = core
                            .send_direct_message(guardian_peer_id.clone(), &msg)
                            .await;
                        sent_count += 1;
                    }
                }
                Json(ApiV2Response::success(serde_json::json!({
                    "status": "shares_created_and_sent",
                    "metadata": metadata,
                    "share_count": shares.len(),
                    "sent_to_guardians": sent_count,
                })))
                .into_response()
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiV2Response::<()>::error("RECOVERY_ERROR", &e.to_string())),
            )
                .into_response(),
        }
    }

    pub async fn list_my_shares(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        // Query mailbox for incoming recovery shares
        let core = state.core.read().await;
        let share_count = match core.mailbox_store.drain_inbox() {
            Ok(envelopes) => envelopes.len(),
            Err(_) => 0,
        };

        Json(ApiV2Response::success(serde_json::json!({
            "shares": [],
            "count": share_count,
            "note": "Shares stored encrypted in mailbox; use reconstruct endpoint to recover"
        })))
    }

    pub async fn reconstruct_key(
        State(state): State<Arc<AppState>>,
        Json(req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let shares_raw = req["shares"].as_array();
        let threshold = req["threshold"].as_u64().unwrap_or(2) as u8;

        let Some(shares_arr) = shares_raw else {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error(
                    "INVALID_PARAMS",
                    "shares array required",
                )),
            )
                .into_response();
        };

        // Parse shares with indices
        let raw_shares: Vec<(u8, Vec<u8>)> = shares_arr
            .iter()
            .enumerate()
            .filter_map(|(i, s)| {
                s["share_data"]
                    .as_str()
                    .and_then(|hex| hex::decode(hex).ok().map(|data| (i as u8 + 1, data)))
            })
            .collect();

        if raw_shares.len() < threshold as usize {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error(
                    "INSUFFICIENT_SHARES",
                    &format!("Need {} shares, got {}", threshold, raw_shares.len()),
                )),
            )
                .into_response();
        }

        // Reconstruct key
        let expected_hash = req["expected_hash"].as_str().unwrap_or("");
        match crate::core::recovery_shamir::reconstruct_key(&raw_shares, threshold, expected_hash) {
            Ok(recovered_key) => {
                // Require explicit non-empty new_password
                let new_password = match req["new_password"].as_str() {
                    Some(pw) if !pw.is_empty() => pw,
                    _ => {
                        return (
                            StatusCode::BAD_REQUEST,
                            Json(ApiV2Response::<()>::error(
                                "INVALID_PARAMS",
                                "new_password is required and must not be empty",
                            )),
                        )
                            .into_response()
                    }
                };

                // Re-encrypt identity with recovered key
                let _core = state.core.read().await;

                // Create new identity from recovered key
                match crate::core::identity::core_identity::Identity::from_seed(&recovered_key.0) {
                    Ok(new_identity) => match new_identity.save_to_disk(new_password) {
                        Ok(_) => Json(ApiV2Response::success(serde_json::json!({
                            "status": "key_reconstructed_and_identity_restored",
                            "new_identity_id": new_identity.id,
                        })))
                        .into_response(),
                        Err(e) => (
                            StatusCode::INTERNAL_SERVER_ERROR,
                            Json(ApiV2Response::<()>::error("SAVE_ERROR", &e.to_string())),
                        )
                            .into_response(),
                    },
                    Err(e) => (
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(ApiV2Response::<()>::error("IDENTITY_ERROR", &e.to_string())),
                    )
                        .into_response(),
                }
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiV2Response::<()>::error(
                    "RECONSTRUCTION_ERROR",
                    &e.to_string(),
                )),
            )
                .into_response(),
        }
    }
}
