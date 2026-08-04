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
use serde::Deserialize;
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

impl PeerController {
    /// GET /api/v2/peers — All known peers with their connection states
    pub async fn list(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        use crate::core::social::social_graph::{ConnectionState, PeerFilter};

        let core = state.core.read().await;
        let all_peers = core.social_graph.list_peers(PeerFilter::All).await;
        let pending_inbound = core.social_graph.pending_inbound().await;
        let peer_count = core.get_peer_count();

        let connected_count = all_peers.iter()
            .filter(|p| matches!(p.connection_state, ConnectionState::Connected))
            .count();

        let peers_json: Vec<serde_json::Value> = all_peers
            .iter()
            .map(|p| {
                serde_json::json!({
                    "node_id": p.node_id,
                    "did": p.did,
                    "nickname": p.nickname,
                    "avatar_cid": p.avatar_cid,
                    "connection_state": format!("{:?}", p.connection_state),
                    "is_online": p.is_online,
                    "signal": p.signal,
                    "discovered_via": format!("{:?}", p.discovered_via),
                    "last_seen": p.last_seen,
                })
            })
            .collect();

        tracing::debug!(
            target: "peers",
            total = peer_count,
            connected = connected_count,
            pending = pending_inbound.len(),
            "Peer list requested"
        );

        Json(ApiV2Response::success(serde_json::json!({
            "count": peer_count,
            "peers": peers_json,
            "pending_requests": pending_inbound.len(),
            "connected": connected_count,
        })))
    }

    /// GET /api/v2/peers/nearby — mDNS-discovered local peers (the "scanner" view)
    pub async fn nearby(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        use crate::core::social::social_graph::PeerFilter;

        let core = state.core.read().await;
        let nearby_peers = core.social_graph.list_peers(PeerFilter::Nearby).await;

        let peers_json: Vec<serde_json::Value> = nearby_peers
            .iter()
            .map(|p| {
                serde_json::json!({
                    "node_id": p.node_id,
                    "nickname": p.nickname,
                    "avatar_cid": p.avatar_cid,
                    "signal": p.signal,
                    "is_online": p.is_online,
                    "last_seen": p.last_seen,
                })
            })
            .collect();

        Json(ApiV2Response::success(serde_json::json!({
            "nearby": peers_json,
            "count": peers_json.len(),
        })))
    }

    /// POST /api/v2/peers/connect — Send a connection request
    pub async fn request_connect(
        State(_state): State<Arc<AppState>>,
        Json(req): Json<ConnectRequestBody>,
    ) -> impl IntoResponse {
        if req.node_id.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error(
                    "INVALID_NODE_ID",
                    "node_id is required",
                )),
            )
                .into_response();
        }

        tracing::info!(target: "social", "Connection request sent to {}", &req.node_id[..8.min(req.node_id.len())]);

        // Wire: core.social_graph.send_request(&req.node_id, req.message, sign_fn).await
        Json(ApiV2Response::success(serde_json::json!({
            "status": "request_sent",
            "to": req.node_id,
            "note": "Wire social_graph.send_request() here"
        })))
        .into_response()
    }

    /// GET /api/v2/peers/requests — Pending inbound connection requests
    pub async fn list_requests(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        let requests = core.social_graph.pending_inbound().await;

        let requests_json: Vec<serde_json::Value> = requests
            .iter()
            .map(|r| {
                serde_json::json!({
                    "request_id": r.request_id,
                    "from_node_id": r.from_node_id,
                    "from_did": r.from_did,
                    "from_nickname": r.from_nickname,
                    "message": r.message,
                    "timestamp": r.timestamp,
                })
            })
            .collect();

        tracing::debug!(
            target: "social",
            count = requests.len(),
            "Pending connection requests"
        );

        Json(ApiV2Response::success(serde_json::json!({
            "requests": requests_json,
        })))
    }

    /// POST /api/v2/peers/requests/respond — Accept or deny a request
    pub async fn respond_to_request(
        State(state): State<Arc<AppState>>,
        Json(req): Json<RespondRequestBody>,
    ) -> impl IntoResponse {
        let _core = state.core.read().await;

        if req.accept {
            tracing::info!(target: "social", "Accepting connection request {}", req.request_id);
            // Wire: core.social_graph.accept_request(&req.request_id, x25519_pub, sign_fn).await
            Json(ApiV2Response::success(serde_json::json!({
                "status": "accepted",
                "request_id": req.request_id
            })))
            .into_response()
        } else {
            tracing::info!(target: "social", "Denying connection request {}", req.request_id);
            // Wire: core.social_graph.deny_request(&req.request_id).await
            Json(ApiV2Response::success(serde_json::json!({
                "status": "denied",
                "request_id": req.request_id
            })))
            .into_response()
        }
    }

    /// DELETE /api/v2/peers/:node_id — Remove a connection
    pub async fn disconnect(
        State(_state): State<Arc<AppState>>,
        Path(node_id): Path<String>,
    ) -> impl IntoResponse {
        tracing::info!(target: "social", "Removing connection to {}", &node_id[..8.min(node_id.len())]);
        // Wire: core.social_graph.remove(&node_id).await
        Json(ApiV2Response::success(
            serde_json::json!({ "status": "removed" }),
        ))
    }

    /// POST /api/v2/peers/:node_id/block — Block a node
    pub async fn block(
        State(_state): State<Arc<AppState>>,
        Path(node_id): Path<String>,
    ) -> impl IntoResponse {
        tracing::info!(target: "social", "Blocking node {}", &node_id[..8.min(node_id.len())]);
        // Wire: core.social_graph.block(&node_id).await
        Json(ApiV2Response::success(
            serde_json::json!({ "status": "blocked" }),
        ))
    }

    /// GET /api/v2/peers/:node_id/profile — Get a peer's public profile
    pub async fn get_profile(
        State(_state): State<Arc<AppState>>,
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
