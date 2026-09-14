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
    ///
    /// ponytail: merges social_graph + mesh topology. social_graph isn't populated
    /// from libp2p events yet, so topology is the real source. When social_graph
    /// gets wired to swarm events, this deduplicates naturally.
    pub async fn list(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        use crate::core::social::social_graph::{ConnectionState, PeerFilter};

        let core = state.core.read().await;
        let all_peers = core.social_graph.list_peers(PeerFilter::All).await;
        let pending_inbound = core.social_graph.pending_inbound().await;

        let topo = core.get_mesh_topology().await;
        let topo_nodes = topo.as_ref().map(|t| &t.nodes);

        let mut seen = std::collections::HashSet::new();
        let mut peers_json: Vec<serde_json::Value> = Vec::new();

        for p in &all_peers {
            seen.insert(p.node_id.clone());
            peers_json.push(serde_json::json!({
                "node_id": p.node_id,
                "did": p.did,
                "nickname": p.nickname,
                "avatar_cid": p.avatar_cid,
                "connection_state": format!("{:?}", p.connection_state),
                "is_online": p.is_online,
                "signal": p.signal,
                "discovered_via": format!("{:?}", p.discovered_via),
                "last_seen": p.last_seen,
            }));
        }

        if let Some(nodes) = topo_nodes {
            for (id, node) in nodes {
                if seen.contains(id) {
                    continue;
                }
                peers_json.push(serde_json::json!({
                    "node_id": id,
                    "nickname": node.username,
                    "connection_state": if node.is_online { "Connected" } else { "Disconnected" },
                    "is_online": node.is_online,
                    "signal": node.connection_quality,
                    "last_seen": node.last_seen,
                    "threat_level": format!("{:?}", node.threat_level),
                }));
            }
        }

        let peer_count = peers_json.len();
        let connected_count = peers_json
            .iter()
            .filter(|p| {
                p.get("is_online")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false)
            })
            .count();

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

        tracing::warn!(target: "social", "Connection request unavailable; social-graph send is not wired for {}", &req.node_id[..8.min(req.node_id.len())]);

        (
            StatusCode::NOT_IMPLEMENTED,
            Json(ApiV2Response::<()>::error(
                "PEER_CONNECT_UNAVAILABLE",
                "Peer connection requests are not wired to the authenticated social graph yet",
            )),
        )
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

        tracing::warn!(target: "social", "Connection request response unavailable; social-graph mutation is not wired for {}", req.request_id);
        (
            StatusCode::NOT_IMPLEMENTED,
            Json(ApiV2Response::<()>::error(
                "PEER_REQUEST_RESPONSE_UNAVAILABLE",
                "Peer request responses are not wired to the authenticated social graph yet",
            )),
        )
            .into_response()
    }

    /// DELETE /api/v2/peers/:node_id — Remove a connection
    pub async fn disconnect(
        State(_state): State<Arc<AppState>>,
        Path(node_id): Path<String>,
    ) -> impl IntoResponse {
        tracing::warn!(target: "social", "Disconnect unavailable; social-graph removal is not wired for {}", &node_id[..8.min(node_id.len())]);
        (
            StatusCode::NOT_IMPLEMENTED,
            Json(ApiV2Response::<()>::error(
                "PEER_DISCONNECT_UNAVAILABLE",
                "Peer disconnect is not wired to the authenticated social graph yet",
            )),
        )
    }

    /// POST /api/v2/peers/:node_id/block — Block a node
    pub async fn block(
        State(_state): State<Arc<AppState>>,
        Path(node_id): Path<String>,
    ) -> impl IntoResponse {
        tracing::warn!(target: "social", "Block unavailable; social-graph mutation is not wired for {}", &node_id[..8.min(node_id.len())]);
        (
            StatusCode::NOT_IMPLEMENTED,
            Json(ApiV2Response::<()>::error(
                "PEER_BLOCK_UNAVAILABLE",
                "Peer blocking is not wired to the authenticated social graph yet",
            )),
        )
    }

    /// GET /api/v2/peers/:node_id/profile — Get a peer's public profile
    pub async fn get_profile(
        State(_state): State<Arc<AppState>>,
        Path(node_id): Path<String>,
    ) -> impl IntoResponse {
        tracing::warn!(target: "peers", "Peer profile lookup unavailable for {}", &node_id[..8.min(node_id.len())]);
        (
            StatusCode::NOT_IMPLEMENTED,
            Json(ApiV2Response::<()>::error(
                "PEER_PROFILE_UNAVAILABLE",
                "Peer profile lookup is not wired to the canonical discovery directory yet",
            )),
        )
    }

    /// POST /api/v2/peers/ping — Ping a peer and get latency
    pub async fn ping(
        State(_state): State<Arc<AppState>>,
        Json(req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let node_id = req.get("node_id").and_then(|v| v.as_str()).unwrap_or("");
        if node_id.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error(
                    "INVALID_NODE_ID",
                    "node_id is required",
                )),
            )
                .into_response();
        }
        tracing::warn!(target: "peers", "Peer ping unavailable for {}", &node_id[..8.min(node_id.len())]);
        (
            StatusCode::NOT_IMPLEMENTED,
            Json(ApiV2Response::<()>::error(
                "PEER_PING_UNAVAILABLE",
                "Peer ping is not wired to the live mesh manager yet",
            )),
        ))
    }
}
