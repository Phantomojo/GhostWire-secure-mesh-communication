//! Network Controller
//!
//! Handles network status, mesh health, and QR code contact operations.

use crate::api_v2_types::*;
use crate::web::AppState;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

pub struct NetworkController;

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

impl NetworkController {
    pub async fn status(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        let metrics = core.monitor.get_metrics().await;

        // Wire actual transport mode and network state
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

        // Wire x25519_pub from encryption manager
        let hybrid_pub = core.encryption.export_public_key();
        let x25519_pub = if hybrid_pub.len() >= 64 {
            hybrid_pub[32..64].to_vec()
        } else {
            vec![]
        };

        // Wire multiaddrs from mesh listeners
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

        // Sign the contact card with identity keypair
        let card_bytes = serde_json::to_vec(&card).unwrap_or_default();
        let identity = core.identity.read().await;
        let signature = identity
            .sign(&card_bytes)
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
