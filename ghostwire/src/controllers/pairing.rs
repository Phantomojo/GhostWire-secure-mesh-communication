//! Pairing Controller
//!
//! Handles device pairing operations.

use crate::api_v2_types::*;
use crate::controllers::network::NetworkController;
use crate::web::AppState;
use axum::{
    extract::State,
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct ApplyInviteRequest {
    pub token: String,
}

impl PairingController {
    pub async fn generate_invite(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        match core.generate_invite_token(1, 24).await {
            Ok(token) => Json(ApiV2Response::success(serde_json::json!({
                "token": token,
                "expires_in_hours": 24,
                "max_uses": 1,
            })))
            .into_response(),
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiV2Response::<()>::error("INVITE_ERROR", e.to_string())),
            )
                .into_response(),
        }
    }

    pub async fn apply_invite(
        State(state): State<Arc<AppState>>,
        Json(req): Json<ApplyInviteRequest>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;
        match core.apply_invite_token(&req.token).await {
            Ok(()) => Json(ApiV2Response::success(serde_json::json!({
                "status": "applied",
                "message": "IFAC seed updated. Restart RNS to join the new overlay.",
            })))
            .into_response(),
            Err(e) => (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error("INVITE_INVALID", e.to_string())),
            )
                .into_response(),
        }
    }
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
                Json(ApiV2Response::<()>::error("PAIR_ERROR", e.to_string())),
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
