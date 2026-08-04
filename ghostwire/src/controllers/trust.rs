//! Trust Controller
//!
//! Handles trust scoring and peer verification operations.

use crate::api_v2_types::*;
use crate::web::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use std::sync::Arc;

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
                Json(ApiV2Response::<()>::error("VERIFY_ERROR", e.to_string())),
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
