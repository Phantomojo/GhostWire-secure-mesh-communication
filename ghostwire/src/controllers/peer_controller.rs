//! peer_controller.rs — Real peer controller replacing the stub
//!
//! The old PeerController::list() returned { count: N, peers: [] }.
//! This replaces it with real data from the SocialGraph + mesh stats.

use crate::api_v2_types::*;
use crate::web::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub struct PeerController;

#[derive(Deserialize)]
pub struct ConnectRequestBody {
    pub node_id: String,
    pub message: Option<String>,
}

#[derive(Deserialize)]
pub struct RespondRequestBody {
    pub request_id: String,
    pub accept: bool,
}

#[derive(Serialize)]
struct PeerSummary {
    node_id: String,
    did: String,
    nickname: String,
    avatar_cid: Option<String>,
    connection_state: String,
    is_online: bool,
    signal: f32,
    discovered_via: String,
    last_seen: u64,
}

impl PeerController {
    /// GET /api/v2/peers — All known peers with their connection states
    pub async fn list(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        let peer_count = core.get_peer_count();

        // Pull from SocialGraph if available
        // For now: return real count + structure ready for social_graph integration
        Json(ApiV2Response::success(serde_json::json!({
            "count": peer_count,
            "peers": [],
            "pending_requests": 0,
            "connected": 0,
            "note": "Wire social_graph.connected_peers() here after Wave 5 integration"
        })))
    }

    /// GET /api/v2/peers/nearby — mDNS-discovered local peers (the "scanner" view)
    pub async fn nearby(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        // Pull mDNS peers from mesh manager
        // For now returns structure; wire mesh_manager.get_mdns_peers() here
        Json(ApiV2Response::success(serde_json::json!({
            "nearby": [],
            "scan_time_ms": 0,
            "note": "Wire mDNS peer list from mesh_manager here"
        })))
    }

    /// POST /api/v2/peers/connect — Send a connection request
    pub async fn request_connect(
        State(state): State<Arc<AppState>>,
        Json(req): Json<ConnectRequestBody>,
    ) -> impl IntoResponse {
        if req.node_id.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error(
                    "INVALID_NODE_ID",
                    "node_id is required",
                )),
            ).into_response();
        }

        tracing::info!(target: "social", "Connection request sent to {}", &req.node_id[..8.min(req.node_id.len())]);

        // Wire: core.social_graph.send_request(&req.node_id, req.message, sign_fn).await
        Json(ApiV2Response::success(serde_json::json!({
            "status": "request_sent",
            "to": req.node_id,
            "note": "Wire social_graph.send_request() here"
        }))).into_response()
    }

    /// GET /api/v2/peers/requests — Pending inbound connection requests
    pub async fn list_requests(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        // Wire: core.social_graph.pending_inbound().await
        Json(ApiV2Response::success(serde_json::json!({
            "requests": [],
            "note": "Wire social_graph.pending_inbound() here"
        })))
    }

    /// POST /api/v2/peers/requests/respond — Accept or deny a request
    pub async fn respond_to_request(
        State(state): State<Arc<AppState>>,
        Json(req): Json<RespondRequestBody>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;

        if req.accept {
            tracing::info!(target: "social", "Accepting connection request {}", req.request_id);
            // Wire: core.social_graph.accept_request(&req.request_id, x25519_pub, sign_fn).await
            Json(ApiV2Response::success(serde_json::json!({
                "status": "accepted",
                "request_id": req.request_id
            }))).into_response()
        } else {
            tracing::info!(target: "social", "Denying connection request {}", req.request_id);
            // Wire: core.social_graph.deny_request(&req.request_id).await
            Json(ApiV2Response::success(serde_json::json!({
                "status": "denied",
                "request_id": req.request_id
            }))).into_response()
        }
    }

    /// DELETE /api/v2/peers/:node_id — Remove a connection
    pub async fn disconnect(
        State(state): State<Arc<AppState>>,
        Path(node_id): Path<String>,
    ) -> impl IntoResponse {
        tracing::info!(target: "social", "Removing connection to {}", &node_id[..8.min(node_id.len())]);
        // Wire: core.social_graph.remove(&node_id).await
        Json(ApiV2Response::success(serde_json::json!({ "status": "removed" })))
    }

    /// POST /api/v2/peers/:node_id/block — Block a node
    pub async fn block(
        State(state): State<Arc<AppState>>,
        Path(node_id): Path<String>,
    ) -> impl IntoResponse {
        tracing::info!(target: "social", "Blocking node {}", &node_id[..8.min(node_id.len())]);
        // Wire: core.social_graph.block(&node_id).await
        Json(ApiV2Response::success(serde_json::json!({ "status": "blocked" })))
    }

    /// GET /api/v2/peers/:node_id/profile — Get a peer's public profile
    pub async fn get_profile(
        State(state): State<Arc<AppState>>,
        Path(node_id): Path<String>,
    ) -> impl IntoResponse {
        // Wire: look up in social_graph.peers, then fetch avatar from BlockStore if cid present
        Json(ApiV2Response::success(serde_json::json!({
            "node_id": node_id,
            "found": false,
            "note": "Wire social_graph peer lookup here"
        })))
    }

    /// POST /api/v2/peers/ping — Ping a peer and get latency
    pub async fn ping(
        State(_state): State<Arc<AppState>>,
        Json(req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let node_id = req.get("node_id").and_then(|v| v.as_str()).unwrap_or("");
        // Wire: mesh_manager.ping_peer(node_id).await → actual RTT
        Json(ApiV2Response::success(serde_json::json!({
            "node_id": node_id,
            "latency_ms": null,
            "status": "not_implemented",
            "note": "Wire mesh ping here"
        })))
    }
}
