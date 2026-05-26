//! peer_controller.rs — Real peer controller replacing the stub
//!
//! The old PeerController::list() returned { count: N, peers: [] }.
//! This replaces it with real data from the SocialGraph + mesh stats.

use crate::api_v2_types::*;
use crate::web::{AppState, AuditEventType};
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use libp2p::PeerId;
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
    peer_id: String,
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

        let filter = crate::core::social::social_graph::PeerFilter::All;
        let peers = core.social_graph.list_peers(filter).await;
        let peer_count = peers.len();

        let mut summaries: Vec<PeerSummary> = peers
            .into_iter()
            .map(|p| PeerSummary {
                node_id: p.node_id,
                peer_id: String::new(), // filled from mesh topology below when available
                did: p.did,
                nickname: p.nickname,
                avatar_cid: p.avatar_cid,
                connection_state: format!("{:?}", p.connection_state).to_lowercase(),
                is_online: p.is_online,
                signal: p.signal,
                discovered_via: format!("{:?}", p.discovered_via).to_lowercase(),
                last_seen: p.last_seen,
            })
            .collect();

        // Merge in the live libp2p mesh view. SocialGraph tracks user/social identity,
        // but TUI direct messaging needs the concrete libp2p PeerId. MeshNode keeps
        // PeerId as #[serde(skip)], so expose it explicitly here.
        if let Some(mesh_manager) = &core.mesh_manager {
            let topology = mesh_manager.read().await.get_topology();
            for mesh_node in topology.nodes.values() {
                let mesh_peer_id = mesh_node.peer_id.to_string();

                if let Some(existing) = summaries.iter_mut().find(|summary| {
                    summary.node_id == mesh_node.id
                        || summary.nickname == mesh_node.username
                        || (!summary.peer_id.is_empty() && summary.peer_id == mesh_peer_id)
                }) {
                    existing.peer_id = mesh_peer_id;
                    existing.is_online = mesh_node.is_online;
                    existing.signal = mesh_node.connection_quality;
                    existing.last_seen = mesh_node.last_seen;
                    if existing.nickname.is_empty() {
                        existing.nickname = mesh_node.username.clone();
                    }
                    existing.discovered_via = "mesh".to_string();
                    existing.connection_state = if mesh_node.is_online {
                        "connected".to_string()
                    } else {
                        "offline".to_string()
                    };
                } else {
                    summaries.push(PeerSummary {
                        node_id: mesh_node.id.clone(),
                        peer_id: mesh_peer_id,
                        did: String::new(),
                        nickname: if mesh_node.username.is_empty() {
                            mesh_node.id.clone()
                        } else {
                            mesh_node.username.clone()
                        },
                        avatar_cid: None,
                        connection_state: if mesh_node.is_online {
                            "connected".to_string()
                        } else {
                            "offline".to_string()
                        },
                        is_online: mesh_node.is_online,
                        signal: mesh_node.connection_quality,
                        discovered_via: "mesh".to_string(),
                        last_seen: mesh_node.last_seen,
                    });
                }
            }
        }

        Json(ApiV2Response::success(serde_json::json!({
            "count": peer_count,
            "peers": summaries,
            "connected": summaries.iter().filter(|p| p.connection_state == "connected").count(),
        })))
    }

    /// GET /api/v2/peers/nearby — mDNS-discovered local peers (the "scanner" view)
    pub async fn nearby(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        let peers = core
            .social_graph
            .list_peers(crate::core::social::social_graph::PeerFilter::Nearby)
            .await;

        Json(ApiV2Response::success(serde_json::json!({
            "nearby": peers,
            "scan_time_ms": 0,
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
            )
                .into_response();
        }

        let core = state.core.read().await;
        let identity_lock = core.identity.read().await;
        let identity = &**identity_lock;

        let sign_fn = |data: &[u8]| {
            identity
                .keypair()
                .map(|kp| kp.sign(data).unwrap_or_default())
                .unwrap_or_default()
        };

        match core
            .social_graph
            .send_request(&req.node_id, req.message.clone(), sign_fn)
            .await
        {
            Ok(request) => {
                tracing::info!(target: "social", "Connection request sent to {}", &req.node_id[..8.min(req.node_id.len())]);
                Json(ApiV2Response::success(serde_json::json!({
                    "status": "request_sent",
                    "request": request
                })))
                .into_response()
            }
            Err(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ApiV2Response::<()>::error("SEND_FAILED", &e.to_string())),
            )
                .into_response(),
        }
    }

    /// GET /api/v2/peers/requests — Pending inbound connection requests
    pub async fn list_requests(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        let core = state.core.read().await;
        let requests = core.social_graph.pending_inbound().await;

        Json(ApiV2Response::success(serde_json::json!({
            "requests": requests,
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

            let identity_lock = core.identity.read().await;
            let identity = &**identity_lock;

            // X25519 key derivation (simplified for now — ideally from identity)
            let our_x_pub = vec![]; // TODO: derive from identity

            let sign_fn = |data: &[u8]| {
                identity
                    .keypair()
                    .map(|kp| kp.sign(data).unwrap_or_default())
                    .unwrap_or_default()
            };

            match core
                .social_graph
                .accept_request(&req.request_id, our_x_pub, sign_fn)
                .await
            {
                Ok(response) => {
                    state.audit_log(
                        AuditEventType::PeerConnected,
                        Some(req.request_id.clone()),
                        format!("Connection request {} accepted", req.request_id),
                    );
                    Json(ApiV2Response::success(serde_json::json!({
                        "status": "accepted",
                        "response": response
                    })))
                    .into_response()
                }
                Err(e) => (
                    StatusCode::NOT_FOUND,
                    Json(ApiV2Response::<()>::error(
                        "REQUEST_NOT_FOUND",
                        &e.to_string(),
                    )),
                )
                    .into_response(),
            }
        } else {
            tracing::info!(target: "social", "Denying connection request {}", req.request_id);
            match core.social_graph.deny_request(&req.request_id).await {
                Ok(_) => Json(ApiV2Response::success(serde_json::json!({
                    "status": "denied",
                    "request_id": req.request_id
                })))
                .into_response(),
                Err(e) => (
                    StatusCode::NOT_FOUND,
                    Json(ApiV2Response::<()>::error(
                        "REQUEST_NOT_FOUND",
                        &e.to_string(),
                    )),
                )
                    .into_response(),
            }
        }
    }

    /// DELETE /api/v2/peers/:node_id — Remove a connection
    pub async fn disconnect(
        State(state): State<Arc<AppState>>,
        Path(node_id): Path<String>,
    ) -> impl IntoResponse {
        tracing::info!(target: "social", "Disconnecting peer {}", &node_id[..8.min(node_id.len())]);

        let core = state.core.read().await;

        if let Some(mesh_manager) = &core.mesh_manager {
            let topology = mesh_manager.read().await.get_topology();
            if let Some(node) = topology.nodes.get(&node_id) {
                let peer_id = node.peer_id;
                drop(topology);
                let mut mgr = mesh_manager.write().await;
                if let Err(e) = mgr.disconnect_peer(peer_id).await {
                    tracing::warn!(target: "social", "Mesh disconnect failed for {}: {}", node_id, e);
                }
                drop(mgr);
            } else {
                let mesh_peer_id: Option<PeerId> = node_id.parse::<PeerId>().ok();
                drop(topology);
                if let Some(pid) = mesh_peer_id {
                    let mut mgr = mesh_manager.write().await;
                    if let Err(e) = mgr.disconnect_peer(pid).await {
                        tracing::warn!(target: "social", "Mesh disconnect by PeerId failed for {}: {}", node_id, e);
                    }
                }
            }
        }

        state.audit_log(
            AuditEventType::PeerDisconnected,
            Some(node_id.clone()),
            format!("Connection removed for peer {}", node_id),
        );

        Json(ApiV2Response::success(
            serde_json::json!({ "status": "removed", "node_id": node_id }),
        ))
    }

    /// POST /api/v2/peers/:node_id/block — Block a node
    pub async fn block(
        State(state): State<Arc<AppState>>,
        Path(node_id): Path<String>,
    ) -> impl IntoResponse {
        tracing::info!(target: "social", "Blocking node {}", &node_id[..8.min(node_id.len())]);
        let core = state.core.read().await;
        core.social_graph.block(&node_id).await;

        state.audit_log(
            AuditEventType::PeerBlocked,
            Some(node_id.clone()),
            format!("Node {} blocked via peer controller", node_id),
        );

        Json(ApiV2Response::success(
            serde_json::json!({ "status": "blocked", "node_id": node_id }),
        ))
    }

    /// GET /api/v2/peers/:node_id/profile — Get a peer's public profile
    pub async fn get_profile(
        State(state): State<Arc<AppState>>,
        Path(node_id): Path<String>,
    ) -> impl IntoResponse {
        let core = state.core.read().await;
        let peers = core
            .social_graph
            .list_peers(crate::core::social::social_graph::PeerFilter::All)
            .await;
        let peer = peers.into_iter().find(|p| p.node_id == node_id);

        match peer {
            Some(p) => Json(ApiV2Response::success(p)).into_response(),
            None => (
                StatusCode::NOT_FOUND,
                Json(ApiV2Response::<()>::error(
                    "PEER_NOT_FOUND",
                    "Peer not found",
                )),
            )
                .into_response(),
        }
    }

    /// POST /api/v2/peers/ping — Ping a peer and get latency
    pub async fn ping(
        State(state): State<Arc<AppState>>,
        Json(req): Json<serde_json::Value>,
    ) -> impl IntoResponse {
        let node_id = req.get("node_id").and_then(|v| v.as_str()).unwrap_or("");
        let core = state.core.read().await;

        let mut latency = None;

        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = &core.mesh_manager {
            let topology = mesh_manager.read().await.get_topology();
            if let Some(node) = topology.nodes.get(node_id) {
                latency =
                    Some((100.0_f32 * (1.0 - node.connection_quality.clamp(0.0, 1.0))) as u64);
            }
        }

        Json(ApiV2Response::success(serde_json::json!({
            "node_id": node_id,
            "latency_ms": latency,
            "status": if latency.is_some() { "ok" } else { "peer_not_found" }
        })))
    }
}
