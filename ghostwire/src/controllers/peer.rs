use crate::api_v2_types::*;
use crate::web::AppState;
use axum::{extract::State, response::IntoResponse, Json};
// use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct PeerController;

impl PeerController {
    /// GET /api/v2/peers - List discovered/connected peers
    pub async fn list(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let peer_count = state.core.read().await.get_peer_count();
        Json(
            ApiV2Response::success(serde_json::json!({
                "count": peer_count,
                "peers": []
            }))
            .with_links("/api/v2/peers".to_string()),
        )
    }

    pub async fn connect(
        State(_state): State<Arc<AppState>>,
        Json(_req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        Json(ApiV2Response::success("Connection initiated"))
    }

    pub async fn disconnect(
        State(_state): State<Arc<AppState>>,
        Json(_req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        Json(ApiV2Response::success("Disconnection initiated"))
    }

    pub async fn ping(
        State(_state): State<Arc<AppState>>,
        Json(_req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        Json(ApiV2Response::success(serde_json::json!({
            "status": "pong",
            "latency_ms": 15
        })))
    }
}
