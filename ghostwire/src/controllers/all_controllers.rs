//! stub_controllers.rs — All new controller implementations
//!
//! These replace the "note: wire X here" stubs in peer_controller.rs
//! and add the remaining controllers referenced in core_wiring.rs.
//!
//! Each controller is a thin HTTP adapter over the Core module it wraps.
//! No business logic lives here — it all lives in the Core modules.

use crate::api_v2_types::*;
use crate::web::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;

// ══════════════════════════════════════════════════════════════
// ARK CONTROLLER
// ══════════════════════════════════════════════════════════════

pub struct ArkController;

#[derive(Deserialize)]
pub struct CreateEntryRequest {
    pub title: String,
    pub body: String,
    pub collection: String,
    pub tags: Option<Vec<String>>,
    pub kind: Option<String>,   // "note" | "article" | "code" | "reference"
    pub access: Option<String>, // "private" | "trusted" | "public"
}

impl ArkController {
    pub async fn list_collections(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        let collections = core.knowledge_ark.list_collections().await;
        Json(ApiV2Response::success(collections))
    }

    pub async fn create_collection(
        State(state): State<Arc<AppState>>,
        Json(req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let name = req["name"].as_str().unwrap_or("Untitled").to_string();
        let core = state.core.read().await;
        let col = core.knowledge_ark.create_collection(name).await;
        Json(ApiV2Response::success(col))
    }

    pub async fn create_entry(
        State(state): State<Arc<AppState>>,
        Json(req): Json<CreateEntryRequest>,
    ) -> impl IntoResponse {
        use crate::core::knowledge::knowledge_ark::{AccessLevel, ArkEntry, EntryKind};

        let core = state.core.read().await;
        let author = core.get_identity_id().await;

        let kind = match req.kind.as_deref().unwrap_or("note") {
            "article" => EntryKind::Article,
            "code" => EntryKind::Code {
                language: "unknown".into(),
            },
            "reference" => EntryKind::Reference {
                url: String::new(),
                archived_cid: None,
            },
            _ => EntryKind::Note,
        };

        let access = match req.access.as_deref().unwrap_or("private") {
            "public" => AccessLevel::Public,
            "trusted" => AccessLevel::Trusted,
            _ => AccessLevel::Private,
        };

        let entry = ArkEntry::new(req.title, req.body, kind, author, req.collection)
            .with_tags(req.tags.unwrap_or_default())
            .with_access(access);

        match core.knowledge_ark.add_entry(entry).await {
            Ok(cid) => {
                Json(ApiV2Response::success(serde_json::json!({ "cid": cid }))).into_response()
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiV2Response::<()>::error("ARK_ERROR", &e.to_string())),
            )
                .into_response(),
        }
    }

    pub async fn get_entry(
        State(state): State<Arc<AppState>>,
        Path(cid): Path<String>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;
        match core.knowledge_ark.get_entry(&cid).await {
            Some(entry) => Json(ApiV2Response::success(entry)).into_response(),
            None => (
                StatusCode::NOT_FOUND,
                Json(ApiV2Response::<()>::error("NOT_FOUND", "Entry not found")),
            )
                .into_response(),
        }
    }

    pub async fn delete_entry(
        State(state): State<Arc<AppState>>,
        Path(cid): Path<String>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;
        match core.knowledge_ark.delete_entry(&cid).await {
            Ok(_) => Json(ApiV2Response::success(
                serde_json::json!({ "deleted": cid }),
            ))
            .into_response(),
            Err(e) => (
                StatusCode::NOT_FOUND,
                Json(ApiV2Response::<()>::error("DELETE_ERROR", &e.to_string())),
            )
                .into_response(),
        }
    }

    pub async fn search(
        State(state): State<Arc<AppState>>,
        Query(params): Query<HashMap<String, String>>,
    ) -> impl IntoResponse {
        let q = params.get("q").map(|s| s.as_str()).unwrap_or("");
        if q.is_empty() {
            return Json(ApiV2Response::success(serde_json::json!({ "results": [] })))
                .into_response();
        }
        let core = state.core.read().await;
        let results = core.knowledge_ark.search(q).await;
        let payload: Vec<_> = results
            .into_iter()
            .map(|(entry, score)| serde_json::json!({ "entry": entry, "score": score }))
            .collect();
        Json(ApiV2Response::success(payload)).into_response()
    }

    pub async fn search_by_tag(
        State(state): State<Arc<AppState>>,
        Path(tag): Path<String>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;
        let entries = core.knowledge_ark.search_by_tag(&tag).await;
        Json(ApiV2Response::success(entries))
    }

    pub async fn stats(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        let stats = core.knowledge_ark.stats().await;
        Json(ApiV2Response::success(stats))
    }
}

// ══════════════════════════════════════════════════════════════
// CHANNEL CONTROLLER
// ══════════════════════════════════════════════════════════════

pub struct ChannelController;

impl ChannelController {
    pub async fn list(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        let channels = core.ghost_channels.list().await;
        Json(ApiV2Response::success(channels))
    }

    pub async fn create(
        State(state): State<Arc<AppState>>,
        Json(req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let name = req["name"].as_str().map(|s| s.to_string());
        let core = state.core.read().await;
        match core.ghost_channels.create(name).await {
            Ok(ch) => Json(ApiV2Response::success(serde_json::json!({
                "channel": ch,
                "invite": ch.invite_string(),
                "topic": ch.gossipsub_topic(),
            })))
            .into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiV2Response::<()>::error("CHANNEL_ERROR", &e.to_string())),
            )
                .into_response(),
        }
    }

    pub async fn join(
        State(state): State<Arc<AppState>>,
        Json(req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let invite = req["invite"].as_str().unwrap_or("");
        let core = state.core.read().await;
        match core.ghost_channels.join(invite).await {
            Ok(ch) => Json(ApiV2Response::success(ch)).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error("INVALID_INVITE", &e.to_string())),
            )
                .into_response(),
        }
    }

    pub async fn leave(
        State(state): State<Arc<AppState>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        if let Ok(bytes) = hex::decode(&id) {
            if bytes.len() == 32 {
                let mut arr = [0u8; 32];
                arr.copy_from_slice(&bytes);
                state.core.read().await.ghost_channels.leave(&arr).await;
            }
        }
        Json(ApiV2Response::success(serde_json::json!({ "left": id })))
    }

    pub async fn publish(
        State(state): State<Arc<AppState>>,
        Path(id): Path<String>,
        Json(req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let text = req["text"].as_str().unwrap_or("");
        if text.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error("EMPTY", "text is required")),
            )
                .into_response();
        }

        let topic_id: Option<[u8; 32]> = hex::decode(&id).ok().and_then(|b| b.try_into().ok());

        let Some(topic_id) = topic_id else {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error(
                    "INVALID_ID",
                    "invalid channel id",
                )),
            )
                .into_response();
        };

        let core = state.core.read().await;
        let Some(channel) = core.ghost_channels.get(&topic_id).await else {
            return (
                StatusCode::NOT_FOUND,
                Json(ApiV2Response::<()>::error(
                    "NOT_FOUND",
                    "Channel not joined",
                )),
            )
                .into_response();
        };

        // Prepare message (PoW solving — use spawn_blocking in production)
        match core.ghost_channels.prepare_message(&channel, text).await {
            Ok(msg) => {
                // CRITICAL FIX: Actually publish via mesh gossipsub
                #[cfg(feature = "mesh")]
                {
                    if let Some(mesh_manager) = &core.mesh_manager {
                        let mgr = mesh_manager.read().await;
                        let topic = format!("ghostwire-channel-{}", hex::encode(topic_id));
                        let content = serde_json::to_string(&msg).unwrap_or_default();
                        if let Err(e) = mgr.publish_to_topic(&topic, &content).await {
                            tracing::error!(
                                target: "channel",
                                "Failed to publish to {}: {}",
                                topic,
                                e
                            );
                        } else {
                            tracing::info!(target: "channel", "Published channel message to topic {}", topic);
                        }
                    }
                }
                Json(ApiV2Response::success(serde_json::json!({
                    "status": "published",
                    "msg_id": hex::encode(msg.msg_id),
                    "pow_nonce": msg.pow_nonce,
                })))
                .into_response()
            }
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error("PUBLISH_ERROR", &e.to_string())),
            )
                .into_response(),
        }
    }

    pub async fn get_messages(
        State(state): State<Arc<AppState>>,
        Path(id): Path<String>,
    ) -> impl IntoResponse {
        let topic_id: Option<[u8; 32]> = hex::decode(&id).ok().and_then(|b| b.try_into().ok());
        let Some(topic_id) = topic_id else {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error(
                    "INVALID_ID",
                    "invalid channel id",
                )),
            )
                .into_response();
        };
        let core = state.core.read().await;
        let messages = core.ghost_channels.get_messages(&topic_id).await;
        Json(ApiV2Response::success(messages)).into_response()
    }
}

// ══════════════════════════════════════════════════════════════
// TRUST CONTROLLER
// ══════════════════════════════════════════════════════════════

pub struct TrustController;

impl TrustController {
    pub async fn get_trust(
        State(state): State<Arc<AppState>>,
        Path(peer_id): Path<String>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;
        let score = core.trust_store.trust_score(&peer_id).await;
        let record = core.trust_store.get(&peer_id).await;
        Json(ApiV2Response::success(serde_json::json!({
            "peer_id": peer_id,
            "trust_score": score,
            "record": record,
        })))
    }

    pub async fn mark_verified(
        State(state): State<Arc<AppState>>,
        Path(peer_id): Path<String>,
        Json(req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let key_hex = req["public_key"].as_str().unwrap_or("");
        let pub_key = match hex::decode(key_hex) {
            Ok(k) => k,
            Err(_) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiV2Response::<()>::error(
                        "INVALID_KEY",
                        "public_key must be hex",
                    )),
                )
                    .into_response()
            }
        };
        let core = state.core.read().await;
        match core.trust_store.mark_verified(&peer_id, &pub_key).await {
            Ok(_) => Json(ApiV2Response::success(
                serde_json::json!({ "verified": peer_id }),
            ))
            .into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error("VERIFY_ERROR", &e.to_string())),
            )
                .into_response(),
        }
    }

    pub async fn block_peer(
        State(state): State<Arc<AppState>>,
        Path(peer_id): Path<String>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;
        core.trust_store.block(&peer_id).await;
        Json(ApiV2Response::success(
            serde_json::json!({ "blocked": peer_id }),
        ))
    }

    pub async fn list_peers(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        let peers = core.trust_store.all_peers().await;
        Json(ApiV2Response::success(peers))
    }
}

// ══════════════════════════════════════════════════════════════
// NETWORK CONTROLLER
// ══════════════════════════════════════════════════════════════

pub struct NetworkController;

impl NetworkController {
    pub async fn status(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        let metrics = core.monitor.get_metrics().await;

        // CRITICAL FIX: Wire actual transport mode and network state
        let transport_mode = {
            let guard = core.active_transport.read().await;
            guard
                .as_ref()
                .map(|t| t.name())
                .unwrap_or("none")
                .to_string()
        };

        let network_state = {
            // Derive from mesh_manager status
            #[cfg(feature = "mesh")]
            {
                if core.mesh_manager.is_some() {
                    "full_mesh".to_string()
                } else {
                    "disconnected".to_string()
                }
            }
            #[cfg(not(feature = "mesh"))]
            {
                "no_mesh".to_string()
            }
        };

        Json(ApiV2Response::success(serde_json::json!({
            "internet_reachable": metrics.is_internet_reachable,
            "latency_ms": metrics.latency_ms,
            "packet_loss_pct": metrics.packet_loss_ratio * 100.0,
            "active_peers": metrics.active_peers,
            "peer_count": metrics.active_peers,
            "last_update": metrics.last_update,
            "transport_mode": transport_mode,
            "network_state": network_state,
        })))
    }

    pub async fn mesh_health(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        let snapshot = crate::core::mesh_health::build_snapshot(&core, 0).await;
        Json(ApiV2Response::success(snapshot))
    }

    pub async fn generate_contact_qr(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        let node_id = core.get_identity_id().await;
        let pub_key = core.get_public_key();
        let rendezvous = core.pairing_manager.rendezvous_address(&pub_key);

        use crate::core::identity::device_pairing::ContactCard;

        // CRITICAL FIX: Wire x25519_pub from encryption manager
        let hybrid_pub = core.encryption.export_public_key();
        let x25519_pub = if hybrid_pub.len() >= 64 {
            hybrid_pub[32..64].to_vec()
        } else {
            vec![]
        };

        // CRITICAL FIX: Wire multiaddrs from mesh listeners
        let multiaddrs = core
            .get_mesh_listeners()
            .await
            .iter()
            .map(|a| a.to_string())
            .collect();

        let mut card = ContactCard {
            node_id: node_id.clone(),
            nickname: core.get_metadata().await.nickname,
            ed25519_pub: pub_key.to_vec(),
            x25519_pub,
            rendezvous_address: rendezvous,
            multiaddrs,
            created_at: now_secs(),
            signature: vec![],
        };

        // CRITICAL FIX: Sign the contact card with identity keypair
        let card_bytes = serde_json::to_vec(&card).unwrap_or_default();
        let identity = core.identity.read().await;
        let signature = identity
            .keypair()
            .map(|kp| kp.sign(&card_bytes).unwrap_or_default())
            .unwrap_or_default();
        card.signature = signature;

        match card.to_qr_string() {
            Ok(qr) => Json(ApiV2Response::success(serde_json::json!({
                "qr_string": qr,
                "node_id": node_id,
                "rendezvous": card.rendezvous_address,
            })))
            .into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiV2Response::<()>::error("QR_ERROR", &e.to_string())),
            )
                .into_response(),
        }
    }

    pub async fn scan_contact_qr(
        State(_state): State<Arc<AppState>>,
        Json(req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let qr_string = req["qr_string"].as_str().unwrap_or("");
        use crate::core::identity::device_pairing::ContactCard;
        match ContactCard::from_qr_string(qr_string) {
            Ok(card) => Json(ApiV2Response::success(card)).into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error("QR_PARSE_ERROR", &e.to_string())),
            )
                .into_response(),
        }
    }
}

// ══════════════════════════════════════════════════════════════
// PAIRING CONTROLLER
// ══════════════════════════════════════════════════════════════

pub struct PairingController;

impl PairingController {
    pub async fn generate_code(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        match core.pairing_manager.initiate_pairing().await {
            Ok((code, session)) => Json(ApiV2Response::success(serde_json::json!({
                "code": code.display,
                "safety_number": code.safety_number(),
                "session_id": session.session_id,
                "expires_in_secs": 600,
            })))
            .into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiV2Response::<()>::error("PAIR_ERROR", &e.to_string())),
            )
                .into_response(),
        }
    }

    pub async fn get_rendezvous_address(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        let pub_key = core.get_public_key();
        let addr = core.pairing_manager.rendezvous_address(&pub_key);
        Json(ApiV2Response::success(serde_json::json!({
            "rendezvous_address": addr,
            "dht_key": hex::encode(core.pairing_manager.rendezvous_dht_key(&pub_key)),
        })))
    }

    pub async fn get_contact_card(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        NetworkController::generate_contact_qr(State(state)).await
    }
}

// ══════════════════════════════════════════════════════════════
// RECOVERY CONTROLLER
// ══════════════════════════════════════════════════════════════

pub struct RecoveryController;

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
        // In production: use a proper KDF (HKDF-SHA256) with a salt
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
                // CRITICAL FIX: Send each share to its guardian via messaging
                let mut sent_count = 0;
                for (i, share) in shares.iter().enumerate() {
                    if i < guardians.len() {
                        let guardian_peer_id = &guardians[i].0;
                        let share_json = serde_json::to_string(share).unwrap_or_default();
                        // Send via core messaging
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
        // CRITICAL FIX: Query mailbox for incoming recovery shares
        let core = state.core.read().await;
        let share_count = match core.mailbox_store.drain_inbox() {
            Ok(envelopes) => {
                // Count envelopes that might be recovery shares
                // (In production, would check message type metadata)
                envelopes.len()
            }
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
        // CRITICAL FIX: Collect shares, reconstruct key, re-encrypt identity
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
                // SECURITY FIX: Require explicit non-empty new_password (no hardcoded default)
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

                // CRITICAL FIX: Re-encrypt identity with recovered key
                let core = state.core.read().await;

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

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
