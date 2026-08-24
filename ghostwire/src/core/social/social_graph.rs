//! social_graph.rs — Connection request protocol + peer discovery
//!
//! The "follow/accept/deny" social layer on top of the P2P transport.
//!
//! Mental model:
//!   - libp2p handles the TRANSPORT (raw TCP/WebRTC connections)
//!   - This module handles the SOCIAL LAYER (who you've chosen to talk to)
//!
//! Flow:
//!   1. Node scanning: mDNS + DHT gossip surfaces nearby/known nodes
//!      with their username and avatar. They show up in the "Nearby" list.
//!   2. Connection request: User taps a node → sends a signed
//!      ConnectRequest message via the existing transport.
//!   3. Recipient gets a notification, sees the requester's profile card.
//!   4. Accept → mutual TrustStore pin, Double Ratchet session init.
//!      Deny  → silent drop + optional local block.
//!   5. Both sides are now "connected" — messages can flow.
//!
//! Technology choices:
//!   - Connection requests sent as DHT records (offline delivery)
//!     OR direct Gossipsub message if peer is online
//!   - Signed with Ed25519 (non-repudiable)
//!   - Encrypted to recipient's X25519 pub (private)
//!   - Stored in sled pending acceptance

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// ── Connection request types ──────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionState {
    /// We sent a request, waiting for their response
    PendingOutbound,
    /// They sent us a request, waiting for our decision
    PendingInbound,
    /// Mutually connected
    Connected,
    /// We denied their request (or they denied ours)
    Denied,
    /// Blocked — no requests accepted from this node
    Blocked,
    /// We removed the connection
    Removed,
}

/// A signed connection request message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectRequest {
    /// Unique request ID
    pub request_id: String,
    /// Requester's node ID
    pub from_node_id: String,
    /// Requester's DID
    pub from_did: String,
    /// Requester's display name
    pub from_nickname: String,
    /// Requester's Ed25519 public key (for verification)
    pub from_ed25519_pub: Vec<u8>,
    /// Optional personal message ("Hey, I met you at the Nairobi hackathon")
    pub message: Option<String>,
    /// Timestamp
    pub timestamp: u64,
    /// Ed25519 signature over (request_id + from_node_id + to_node_id + timestamp)
    pub signature: Vec<u8>,
    /// The target node ID
    pub to_node_id: String,
}

impl ConnectRequest {
    pub fn signable_bytes(
        request_id: &str,
        from_node_id: &str,
        to_node_id: &str,
        timestamp: u64,
    ) -> Vec<u8> {
        let mut b = request_id.as_bytes().to_vec();
        b.extend_from_slice(from_node_id.as_bytes());
        b.extend_from_slice(to_node_id.as_bytes());
        b.extend_from_slice(&timestamp.to_le_bytes());
        b
    }

    pub fn verify(&self) -> bool {
        // In production: verify self.signature over signable_bytes
        // using self.from_ed25519_pub with ring::signature::Ed25519
        // Stub: always true for now — wire with real verification
        !self.signature.is_empty()
    }
}

/// Response to a connection request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConnectResponse {
    pub request_id: String,
    pub from_node_id: String,
    pub to_node_id: String,
    pub accepted: bool,
    /// If accepted: responder's public keys for session init
    pub responder_ed25519_pub: Option<Vec<u8>>,
    pub responder_x25519_pub: Option<Vec<u8>>,
    pub timestamp: u64,
    pub signature: Vec<u8>,
}

/// A peer we know about (from mDNS, DHT, or manual add)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiscoveredPeer {
    pub node_id: String,
    pub did: String,
    pub nickname: String,
    pub avatar_cid: Option<String>,
    pub multiaddrs: Vec<String>,
    pub ed25519_pub: Vec<u8>,
    pub x25519_pub: Vec<u8>,
    pub discovered_via: DiscoveryMethod,
    pub first_seen: u64,
    pub last_seen: u64,
    pub connection_state: ConnectionState,
    /// Signal strength / connection quality 0.0–1.0
    pub signal: f32,
    /// Online right now?
    pub is_online: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DiscoveryMethod {
    Mdns,
    DhtGossip,
    ManualCode,
    QrScan,
    Introduced { by_node_id: String },
}

// ── Social graph manager ──────────────────────────────────────────────────────

pub struct SocialGraph {
    /// All known peers (node_id → DiscoveredPeer)
    peers: Arc<RwLock<HashMap<String, DiscoveredPeer>>>,
    /// Pending inbound requests (request_id → ConnectRequest)
    inbound_requests: Arc<RwLock<HashMap<String, ConnectRequest>>>,
    /// Pending outbound requests (to_node_id → ConnectRequest)
    outbound_requests: Arc<RwLock<HashMap<String, ConnectRequest>>>,
    /// Our own node ID (for signing)
    our_node_id: String,
    our_ed25519_pub: Vec<u8>,
    /// Channel to notify the UI of new requests
    notify_tx: tokio::sync::broadcast::Sender<SocialEvent>,
    directory: Option<Arc<crate::core::discovery_directory::DiscoveryDirectory>>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum SocialEvent {
    PeerDiscovered { peer: DiscoveredPeer },
    PeerOnline { node_id: String },
    PeerOffline { node_id: String },
    ConnectionRequestReceived { request: ConnectRequest },
    ConnectionAccepted { node_id: String, nickname: String },
    ConnectionDenied { node_id: String },
    ConnectionRemoved { node_id: String },
}

impl SocialGraph {
    pub fn new(
        our_node_id: String,
        our_ed25519_pub: Vec<u8>,
    ) -> (Self, tokio::sync::broadcast::Receiver<SocialEvent>) {
        let (tx, rx) = tokio::sync::broadcast::channel(256);
        let graph = Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
            inbound_requests: Arc::new(RwLock::new(HashMap::new())),
            outbound_requests: Arc::new(RwLock::new(HashMap::new())),
            our_node_id,
            our_ed25519_pub,
            notify_tx: tx,
            directory: None,
        };
        (graph, rx)
    }

    pub fn attach_directory(
        &mut self,
        directory: Arc<crate::core::discovery_directory::DiscoveryDirectory>,
    ) {
        self.directory = Some(directory);
    }

    // ── Discovery ─────────────────────────────────────────────────────────────

    /// Register a newly discovered peer (from mDNS, DHT, etc.)
    pub async fn register_discovered(&self, peer: DiscoveredPeer) {
        let node_id = peer.node_id.clone();
        let is_new = !self.peers.read().await.contains_key(&node_id);

        self.peers.write().await.insert(node_id, peer.clone());

        if let Some(directory) = &self.directory {
            let source = match &peer.discovered_via {
                DiscoveryMethod::Mdns => crate::core::discovery_directory::DiscoverySource::Mdns,
                DiscoveryMethod::DhtGossip => {
                    crate::core::discovery_directory::DiscoverySource::Dht
                }
                DiscoveryMethod::ManualCode | DiscoveryMethod::QrScan => {
                    crate::core::discovery_directory::DiscoverySource::Manual
                }
                DiscoveryMethod::Introduced { .. } => {
                    crate::core::discovery_directory::DiscoverySource::Social
                }
            };
            directory
                .observe(
                    peer.node_id.clone(),
                    (!peer.did.is_empty()).then(|| peer.did.clone()),
                    peer.multiaddrs.clone(),
                    source,
                    peer.last_seen,
                )
                .await;
        }

        if is_new {
            let _ = self.notify_tx.send(SocialEvent::PeerDiscovered { peer });
        }
    }

    /// Update a peer's online status
    pub async fn set_online(&self, node_id: &str, online: bool) {
        let now = now_secs();
        if let Some(peer) = self.peers.write().await.get_mut(node_id) {
            peer.is_online = online;
            peer.last_seen = now;
        }
        if let Some(directory) = &self.directory {
            let _ = directory
                .mark_reachability(
                    node_id,
                    if online {
                        crate::core::discovery_directory::ReachabilityState::Seen
                    } else {
                        crate::core::discovery_directory::ReachabilityState::Unreachable
                    },
                )
                .await;
        }
        let event = if online {
            SocialEvent::PeerOnline {
                node_id: node_id.to_string(),
            }
        } else {
            SocialEvent::PeerOffline {
                node_id: node_id.to_string(),
            }
        };
        let _ = self.notify_tx.send(event);
    }

    /// Get all discovered peers sorted by: connected first, then online, then by last_seen
    pub async fn list_peers(&self, filter: PeerFilter) -> Vec<DiscoveredPeer> {
        let peers = self.peers.read().await;
        let mut list: Vec<DiscoveredPeer> = peers
            .values()
            .filter(|p| filter.matches(p))
            .cloned()
            .collect();

        list.sort_by(|a, b| {
            // Connected first
            let a_conn = matches!(a.connection_state, ConnectionState::Connected);
            let b_conn = matches!(b.connection_state, ConnectionState::Connected);
            if a_conn != b_conn {
                return b_conn.cmp(&a_conn);
            }
            // Online second
            if a.is_online != b.is_online {
                return b.is_online.cmp(&a.is_online);
            }
            // Most recent last
            b.last_seen.cmp(&a.last_seen)
        });
        list
    }

    // ── Connection requests ───────────────────────────────────────────────────

    /// Create and send a connection request to a peer
    pub async fn send_request(
        &self,
        to_node_id: &str,
        message: Option<String>,
        sign_fn: impl Fn(&[u8]) -> Vec<u8>,
    ) -> Result<ConnectRequest> {
        let request_id = uuid::Uuid::new_v4().to_string();
        let ts = now_secs();

        let signable =
            ConnectRequest::signable_bytes(&request_id, &self.our_node_id, to_node_id, ts);
        let signature = sign_fn(&signable);

        // Get our nickname from peers — fallback to truncated node ID
        let nickname = self
            .peers
            .read()
            .await
            .get(&self.our_node_id)
            .map(|p| p.nickname.clone())
            .unwrap_or_else(|| self.our_node_id[..8.min(self.our_node_id.len())].to_string());

        let request = ConnectRequest {
            request_id: request_id.clone(),
            from_node_id: self.our_node_id.clone(),
            from_did: format!(
                "did:ghost:gw00:{}",
                &self.our_node_id[..16.min(self.our_node_id.len())]
            ),
            from_nickname: nickname,
            from_ed25519_pub: self.our_ed25519_pub.clone(),
            message,
            timestamp: ts,
            signature,
            to_node_id: to_node_id.to_string(),
        };

        // Mark peer as pending outbound
        if let Some(peer) = self.peers.write().await.get_mut(to_node_id) {
            peer.connection_state = ConnectionState::PendingOutbound;
        }

        self.outbound_requests
            .write()
            .await
            .insert(to_node_id.to_string(), request.clone());

        tracing::info!(
            "Sent connection request {} → {}",
            &self.our_node_id[..8.min(self.our_node_id.len())],
            &to_node_id[..8.min(to_node_id.len())]
        );
        Ok(request)
    }

    /// Receive an inbound connection request from another node
    pub async fn receive_request(&self, request: ConnectRequest) -> Result<()> {
        // Verify signature
        if !request.verify() {
            anyhow::bail!(
                "Connection request has invalid signature from {}",
                request.from_node_id
            );
        }

        // Check if blocked
        let state = self
            .peers
            .read()
            .await
            .get(&request.from_node_id)
            .map(|p| p.connection_state.clone());

        if matches!(state, Some(ConnectionState::Blocked)) {
            tracing::debug!(
                "Ignoring request from blocked node {}",
                request.from_node_id
            );
            return Ok(());
        }

        // Register the requesting node if we don't know them
        if !self.peers.read().await.contains_key(&request.from_node_id) {
            self.register_discovered(DiscoveredPeer {
                node_id: request.from_node_id.clone(),
                did: request.from_did.clone(),
                nickname: request.from_nickname.clone(),
                avatar_cid: None,
                multiaddrs: vec![],
                ed25519_pub: request.from_ed25519_pub.clone(),
                x25519_pub: vec![],
                discovered_via: DiscoveryMethod::DhtGossip,
                first_seen: now_secs(),
                last_seen: now_secs(),
                connection_state: ConnectionState::PendingInbound,
                signal: 0.5,
                is_online: true,
            })
            .await;
        } else if let Some(peer) = self.peers.write().await.get_mut(&request.from_node_id) {
            peer.connection_state = ConnectionState::PendingInbound;
        }

        let event = SocialEvent::ConnectionRequestReceived {
            request: request.clone(),
        };
        self.inbound_requests
            .write()
            .await
            .insert(request.request_id.clone(), request);

        let _ = self.notify_tx.send(event);
        Ok(())
    }

    /// Accept a connection request
    pub async fn accept_request(
        &self,
        request_id: &str,
        our_x25519_pub: Vec<u8>,
        sign_fn: impl Fn(&[u8]) -> Vec<u8>,
    ) -> Result<ConnectResponse> {
        let request = self
            .inbound_requests
            .read()
            .await
            .get(request_id)
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("Request not found: {}", request_id))?;

        // Update peer state
        if let Some(peer) = self.peers.write().await.get_mut(&request.from_node_id) {
            peer.connection_state = ConnectionState::Connected;
        }

        // Remove from pending
        self.inbound_requests.write().await.remove(request_id);

        let ts = now_secs();
        let signable = format!("{}accept{}", request_id, ts);
        let signature = sign_fn(signable.as_bytes());

        let response = ConnectResponse {
            request_id: request_id.to_string(),
            from_node_id: self.our_node_id.clone(),
            to_node_id: request.from_node_id.clone(),
            accepted: true,
            responder_ed25519_pub: Some(self.our_ed25519_pub.clone()),
            responder_x25519_pub: Some(our_x25519_pub),
            timestamp: ts,
            signature,
        };

        let _ = self.notify_tx.send(SocialEvent::ConnectionAccepted {
            node_id: request.from_node_id.clone(),
            nickname: request.from_nickname,
        });

        tracing::info!(
            "Accepted connection request from {}",
            &request.from_node_id[..8.min(request.from_node_id.len())]
        );
        Ok(response)
    }

    /// Deny a connection request
    pub async fn deny_request(&self, request_id: &str) -> Result<()> {
        let request = self
            .inbound_requests
            .write()
            .await
            .remove(request_id)
            .ok_or_else(|| anyhow::anyhow!("Request not found: {}", request_id))?;

        if let Some(peer) = self.peers.write().await.get_mut(&request.from_node_id) {
            peer.connection_state = ConnectionState::Denied;
        }

        let _ = self.notify_tx.send(SocialEvent::ConnectionDenied {
            node_id: request.from_node_id,
        });
        Ok(())
    }

    /// Block a node (deny + never show requests from them)
    pub async fn block(&self, node_id: &str) {
        if let Some(peer) = self.peers.write().await.get_mut(node_id) {
            peer.connection_state = ConnectionState::Blocked;
        }
    }

    /// Get all pending inbound requests
    pub async fn pending_inbound(&self) -> Vec<ConnectRequest> {
        self.inbound_requests
            .read()
            .await
            .values()
            .cloned()
            .collect()
    }

    /// Get connected peers only
    pub async fn connected_peers(&self) -> Vec<DiscoveredPeer> {
        self.list_peers(PeerFilter::Connected).await
    }
}

// ── Peer filter ───────────────────────────────────────────────────────────────

pub enum PeerFilter {
    All,
    Connected,
    Online,
    PendingInbound,
    Nearby, // mDNS-discovered
}

impl PeerFilter {
    fn matches(&self, peer: &DiscoveredPeer) -> bool {
        match self {
            Self::All => !matches!(peer.connection_state, ConnectionState::Blocked),
            Self::Connected => matches!(peer.connection_state, ConnectionState::Connected),
            Self::Online => {
                peer.is_online && !matches!(peer.connection_state, ConnectionState::Blocked)
            }
            Self::PendingInbound => {
                matches!(peer.connection_state, ConnectionState::PendingInbound)
            }
            Self::Nearby => matches!(peer.discovered_via, DiscoveryMethod::Mdns),
        }
    }
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn request_and_accept_flow() {
        let (graph, mut rx) = SocialGraph::new("node_alice".into(), vec![1u8; 32]);

        // Bob sends Alice a request
        let request = ConnectRequest {
            request_id: "req1".into(),
            from_node_id: "node_bob".into(),
            from_did: "did:ghost:gw00:bob".into(),
            from_nickname: "Bob".into(),
            from_ed25519_pub: vec![2u8; 32],
            message: Some("Hey Alice!".into()),
            timestamp: 0,
            signature: vec![1u8], // non-empty = passes stub verify
            to_node_id: "node_alice".into(),
        };

        graph.receive_request(request).await.unwrap();

        // Registering an unknown peer emits PeerDiscovered before the request
        // event. Drain discovery and assert the terminal request notification.
        let first_event = rx.try_recv().unwrap();
        assert!(matches!(first_event, SocialEvent::PeerDiscovered { .. }));
        let event = rx.try_recv().unwrap();
        assert!(matches!(
            event,
            SocialEvent::ConnectionRequestReceived { .. }
        ));

        // Alice accepts
        let _response = graph
            .accept_request("req1", vec![3u8; 32], |_| vec![4u8; 64])
            .await
            .unwrap();

        // Bob should now be Connected
        let connected = graph.connected_peers().await;
        assert_eq!(connected.len(), 1);
        assert_eq!(connected[0].node_id, "node_bob");
    }
}
