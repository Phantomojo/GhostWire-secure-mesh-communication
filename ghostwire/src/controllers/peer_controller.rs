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

/// Append a peer id to the on-disk blocklist.
///
/// The `TrustStore` is in-memory and rebuilt from scratch on restart, so a
/// block recorded there alone would silently lapse the next time the daemon
/// starts — a user who blocked a peer would find them connected again with no
/// warning. This writes the id to `blocked_peers.json` under the GhostWire
/// home so the decision survives a restart.
///
/// This is deliberately separate from `SecurityManager::blacklist`, which is an
/// in-memory, IP-keyed, 1-hour-TTL rate-limit list for connection attempts. That
/// is a different concern from a durable, peer-identity ban and reusing it
/// would give a user block that expires after an hour.
async fn persist_blocklist(peer_id: &str) {
    use std::collections::BTreeSet;

    let dir = crate::controllers::profiles::ProfilesController::ghostwire_dir();
    if std::fs::create_dir_all(&dir).is_err() {
        tracing::warn!(target: "social", "cannot create GhostWire dir for blocklist");
        return;
    }
    let path = dir.join("blocked_peers.json");

    // Read-modify-write. BTreeSet keeps the file deterministic for diffing.
    let mut set: BTreeSet<String> = std::fs::read_to_string(&path)
        .ok()
        .and_then(|s| serde_json::from_str::<Vec<String>>(&s).ok())
        .map(|v| v.into_iter().collect())
        .unwrap_or_default();
    if !set.insert(peer_id.to_string()) {
        return; // already present
    }
    match serde_json::to_string_pretty(&set.into_iter().collect::<Vec<_>>()) {
        Ok(json) => {
            if let Err(e) = std::fs::write(&path, json) {
                tracing::warn!(target: "social", "failed to persist block: {}", e);
            }
        }
        Err(e) => tracing::warn!(target: "social", "failed to serialise blocklist: {}", e),
    }
}

impl PeerController {
    /// GET /api/v2/peers — All known peers with their connection states
    ///
    /// ponytail: merges social_graph + mesh topology. social_graph isn't populated
    /// from libp2p events yet, so topology is the real source. When social_graph
    /// gets wired to swarm events, this deduplicates naturally.
    pub async fn list(State(state): State<Arc<AppState>>) -> impl IntoResponse {
        use crate::core::social::social_graph::PeerFilter;

        let core = state.core.read().await;
        let all_peers = core.social_graph.list_peers(PeerFilter::All).await;
        let pending_inbound = core.social_graph.pending_inbound().await;

        let topo = core.get_mesh_topology().await;
        let topo_nodes = topo.as_ref().map(|t| &t.nodes);

        let mut seen = std::collections::HashSet::new();
        let mut peers_json: Vec<serde_json::Value> = Vec::new();

        for p in &all_peers {
            seen.insert(p.node_id.clone());

            // Live mesh state WINS over the social graph's record.
            //
            // The two can disagree, and the social graph is the one that is
            // wrong: the topology is written by real ConnectionEstablished /
            // ConnectionClosed events, while a social-graph entry is a record
            // of a handshake that can go stale. Observed live on 2026-09-28:
            // this endpoint reported
            //
            //     {"node_id":"12D3KooWRuoJ...","connection_state":"Disconnected",
            //      "is_online":false}
            //
            // for a peer that was simultaneously answering pings
            // (latency_ms 175, 70, 108, 52, 97, 56 across six samples).
            // Previously the topology was only consulted for peers the social
            // graph had never seen (`if seen.contains(id) { continue; }`), so
            // exactly the peers with a stale social-graph record -- the ones you
            // most want an accurate answer about -- never got the live value.
            let live = topo_nodes.and_then(|nodes| nodes.get(&p.node_id));
            let is_online = live.map(|n| n.is_online).unwrap_or(p.is_online);
            let connection_state: String = match live {
                Some(n) => {
                    if n.is_online {
                        "Connected".to_string()
                    } else {
                        "Disconnected".to_string()
                    }
                }
                None => format!("{:?}", p.connection_state),
            };

            peers_json.push(serde_json::json!({
                "node_id": p.node_id,
                "did": p.did,
                "nickname": p.nickname,
                "avatar_cid": p.avatar_cid,
                "connection_state": connection_state,
                "is_online": is_online,
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
        State(state): State<Arc<AppState>>,
        Path(node_id): Path<String>,
    ) -> impl IntoResponse {
        if node_id.is_empty() {
            return (
                StatusCode::BAD_REQUEST,
                Json(ApiV2Response::<()>::error(
                    "INVALID_NODE_ID",
                    "node id is required",
                )),
            )
                .into_response();
        }

        // Enforcement point: record the block in the TrustStore.
        // `TrustStore::block` creates a Blocked record even for a peer that is
        // not yet known, so blocking someone you have never met is not
        // silently a no-op (regression test in reputation.rs).
        let newly_inserted = {
            let core = state.core.read().await;
            core.trust_store.block(&node_id).await
        };
        // The TrustStore is in-memory and rebuilt on restart, so persist the
        // block or it silently lapses on the next daemon start.
        persist_blocklist(&node_id).await;

        tracing::info!(
            target: "social",
            "Peer blocked: {} (new record: {})",
            &node_id[..8.min(node_id.len())],
            newly_inserted
        );

        Json(ApiV2Response::success(serde_json::json!({
            "status": "blocked",
            "node_id": node_id,
            "new_record": newly_inserted,
        })))
        .into_response()
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
    ///
    /// Returns a MEASURED round-trip. The request is put on the wire and the
    /// call only completes when the peer's reply arrives, so a peer that never
    /// answers yields a timeout rather than a fabricated number.
    pub async fn ping(
        State(state): State<Arc<AppState>>,
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
        let peer_id: libp2p::PeerId = match node_id.parse() {
            Ok(p) => p,
            Err(e) => {
                return (
                    StatusCode::BAD_REQUEST,
                    Json(ApiV2Response::<()>::error(
                        "INVALID_PEER_ID",
                        format!("node_id is not a valid peer id: {}", e),
                    )),
                )
                    .into_response();
            }
        };

        // Refuse an explicitly blocked peer: a latency probe would otherwise
        // open a live connection to someone the user has banned.
        {
            let core = state.core.read().await;
            if core.trust_store.is_blocked(node_id).await {
                return (
                    StatusCode::FORBIDDEN,
                    Json(ApiV2Response::<()>::error(
                        "PEER_BLOCKED",
                        "peer is blocked",
                    )),
                )
                    .into_response();
            }
        }

        let mesh = {
            let core = state.core.read().await;
            core.mesh_manager.clone()
        };
        let Some(mesh) = mesh else {
            return (
                StatusCode::SERVICE_UNAVAILABLE,
                Json(ApiV2Response::<()>::error(
                    "MESH_UNAVAILABLE",
                    "mesh is not running",
                )),
            )
                .into_response();
        };

        // Bounded: a silent peer must not pin an HTTP worker indefinitely.
        //
        // Hold a READ guard for the probe; a WRITE guard is what deadlocked.
        //
        // The handler held the exclusive lock for the whole 10s probe, so the
        // driver could neither process the `PingPeer` command nor receive the
        // peer's `ChatResponse("ACK")` that resolves it — both need to take the
        // same lock. A read guard does not block the driver's own writes.
        //
        // This was a real bug, not a theoretical one: every ping returned
        // PING_FAILED after a 10s timeout, on a transport that demonstrably
        // works. The cross-machine run logged a completed 1248-byte handshake in
        // both directions over that very same request_response protocol.
        let guard = mesh.read().await;
        match tokio::time::timeout(std::time::Duration::from_secs(10), guard.ping_peer(peer_id))
            .await
        {
            Ok(Ok(rtt_ms)) => Json(ApiV2Response::success(serde_json::json!({
                "node_id": node_id,
                // Field name matches what the web UI already declares on
                // `peers.ping` (`{ latency_ms: number }`). Emitting `rtt_ms`
                // here would have left the UI reading `undefined` and hiding
                // every latency reading behind its `!= null` guards.
                "latency_ms": rtt_ms,
            })))
            .into_response(),
            Ok(Err(e)) => (
                StatusCode::BAD_GATEWAY,
                Json(ApiV2Response::<()>::error(
                    "PING_FAILED",
                    format!("no reply from peer: {}", e),
                )),
            )
                .into_response(),
            Err(_) => (
                StatusCode::GATEWAY_TIMEOUT,
                Json(ApiV2Response::<()>::error(
                    "PING_TIMEOUT",
                    "peer did not reply within 10s",
                )),
            )
                .into_response(),
        }
    }
}
