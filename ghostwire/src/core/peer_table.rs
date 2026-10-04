//! peer_table.rs — Unified Peer Table for cross-transport peer management
//!
//! Maps a single peer identity to multiple transport addresses, enabling
//! libp2p ↔ Reticulum cross-transport routing and discovery.

use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Canonical identity — deterministic from master_seed via BLAKE3
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct IdentityId(pub [u8; 32]);

impl IdentityId {
    pub fn from_bytes(bytes: &[u8]) -> Self {
        let mut id = [0u8; 32];
        let len = bytes.len().min(32);
        id[..len].copy_from_slice(&bytes[..len]);
        Self(id)
    }

    pub fn short_hex(&self) -> String {
        hex::encode(&self.0[..8])
    }
}

/// Transport kind — matches the existing TransportType enum
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub enum TransportKind {
    Quic,
    StealthTcp,
    Mesh,
    Bluetooth,
    WiFi,
    Reticulum,
}

/// Transport-specific address
#[derive(Clone, Debug)]
pub enum TransportAddr {
    /// libp2p Multiaddr as string (avoids libp2p crate dependency in this module)
    LibP2p(String),
    /// RNS AddressHash (16 bytes)
    Reticulum([u8; 16]),
}

/// Transport-specific address with quality metrics
#[derive(Clone, Debug)]
pub struct TransportAddress {
    pub address: TransportAddr,
    pub last_seen: Instant,
    pub rtt_ms: Option<u64>,
    pub success_rate: f64,
    pub success_count: u64,
    pub failure_count: u64,
}

/// A single peer identity mapped to multiple transport addresses
#[derive(Clone, Debug)]
pub struct UnifiedPeer {
    pub identity_id: IdentityId,
    pub libp2p_peer_id: Option<String>,
    pub reticulum_address: Option<[u8; 16]>,
    pub transports: HashMap<TransportKind, TransportAddress>,
    pub last_seen: Instant,
    pub quality: f64,
    pub capabilities: Vec<String>,
    pub version: Option<String>,
}

/// Query type for flexible peer lookup
#[derive(Clone, Debug)]
pub enum PeerQuery {
    Identity(IdentityId),
    LibP2p(String),
    Reticulum([u8; 16]),
}

/// Summary for monitoring
#[derive(Debug)]
pub struct PeerTableSummary {
    pub total_peers: usize,
    pub peers_with_libp2p: usize,
    pub peers_with_reticulum: usize,
    pub peers_with_both: usize,
    pub avg_quality: f64,
}

/// The unified peer table mapping identities to transport addresses
pub struct UnifiedPeerTable {
    peers: HashMap<IdentityId, UnifiedPeer>,
    by_libp2p_peer_id: HashMap<String, IdentityId>,
    by_reticulum_address: HashMap<[u8; 16], IdentityId>,
    max_stale_duration: Duration,
    max_peers: usize,
}

impl UnifiedPeerTable {
    pub fn new(max_peers: usize, max_stale_duration: Duration) -> Self {
        Self {
            peers: HashMap::new(),
            by_libp2p_peer_id: HashMap::new(),
            by_reticulum_address: HashMap::new(),
            max_stale_duration,
            max_peers,
        }
    }

    /// Upsert a peer discovered via any transport
    pub fn upsert(
        &mut self,
        identity: IdentityId,
        transport: TransportKind,
        address: TransportAddr,
    ) {
        // Evict oldest if at capacity
        if self.peers.len() >= self.max_peers && !self.peers.contains_key(&identity) {
            if let Some(oldest_id) = self
                .peers
                .iter()
                .min_by_key(|(_, p)| p.last_seen)
                .map(|(id, _)| id.clone())
            {
                self.remove(&oldest_id);
            }
        }

        let now = Instant::now();
        // `address` is moved into `transports` below, and the transport-specific
        // identifier assignment needs it on both paths, so clone once up front.
        let address_for_index = address.clone();
        if let Some(peer) = self.peers.get_mut(&identity) {
            // Merge: add/update this transport's address
            peer.transports.insert(
                transport,
                TransportAddress {
                    address,
                    last_seen: now,
                    rtt_ms: None,
                    success_rate: 0.5,
                    success_count: 0,
                    failure_count: 0,
                },
            );
            peer.last_seen = now;

            // ALSO set the transport-specific identifier on the merge path.
            //
            // This was missing, and it is not cosmetic. The identifier assignment
            // only happened in the new-peer branch below, so a peer discovered
            // FIRST over libp2p and LATER over Reticulum never had
            // `reticulum_address` set — the address was in `transports` but not in
            // the field the router reads.
            //
            // `TransportRouter::route` checks `peer.reticulum_address.is_some()`,
            // so such a peer was reported as having no Reticulum route at all.
            // That makes dual-routing structurally impossible for the common case
            // (libp2p first, Reticulum learned later), which is exactly the
            // topology this project is built around.
            //
            // Caught by tests/transport_router_rns.rs:
            // `critical_messages_dual_route_when_both_transports_exist` returned
            // LibP2p instead of Dual, and `recording_outcomes_changes_what_the_
            // router_prefers` could never select Reticulum no matter how many
            // successes it recorded.
            match &address_for_index {
                TransportAddr::LibP2p(addr) => {
                    peer.libp2p_peer_id = Some(addr.clone());
                    self.by_libp2p_peer_id
                        .insert(addr.clone(), identity.clone());
                }
                TransportAddr::Reticulum(hash) => {
                    peer.reticulum_address = Some(*hash);
                    self.by_reticulum_address
                        .insert(*hash, identity.clone());
                }
            }
        } else {
            // New peer
            let mut peer = UnifiedPeer {
                identity_id: identity.clone(),
                libp2p_peer_id: None,
                reticulum_address: None,
                transports: HashMap::new(),
                last_seen: now,
                quality: 0.5,
                capabilities: Vec::new(),
                version: None,
            };
            peer.transports.insert(
                transport,
                TransportAddress {
                    address: address.clone(),
                    last_seen: now,
                    rtt_ms: None,
                    success_rate: 0.5,
                    success_count: 0,
                    failure_count: 0,
                },
            );

            // Set transport-specific identifier
            match &address {
                TransportAddr::LibP2p(addr) => {
                    peer.libp2p_peer_id = Some(addr.clone());
                    self.by_libp2p_peer_id
                        .insert(addr.clone(), identity.clone());
                }
                TransportAddr::Reticulum(hash) => {
                    peer.reticulum_address = Some(*hash);
                    self.by_reticulum_address
                        .insert(*hash, identity.clone());
                }
            }

            self.peers.insert(identity, peer);
        }
    }

    /// Look up a peer by any identifier type
    pub fn lookup(&self, query: &PeerQuery) -> Option<&UnifiedPeer> {
        match query {
            PeerQuery::Identity(id) => self.peers.get(id),
            PeerQuery::LibP2p(peer_id) => self
                .by_libp2p_peer_id
                .get(peer_id)
                .and_then(|id| self.peers.get(id)),
            PeerQuery::Reticulum(addr) => self
                .by_reticulum_address
                .get(addr)
                .and_then(|id| self.peers.get(id)),
        }
    }

    /// Remove a peer by IdentityId
    pub fn remove(&mut self, identity: &IdentityId) {
        if let Some(peer) = self.peers.remove(identity) {
            if let Some(pid) = &peer.libp2p_peer_id {
                self.by_libp2p_peer_id.remove(pid);
            }
            if let Some(addr) = &peer.reticulum_address {
                self.by_reticulum_address.remove(addr);
            }
        }
    }

    /// Remove all stale entries
    pub fn gc_stale(&mut self) -> Vec<IdentityId> {
        let now = Instant::now();
        let stale: Vec<IdentityId> = self
            .peers
            .iter()
            .filter(|(_, p)| now.duration_since(p.last_seen) > self.max_stale_duration)
            .map(|(id, _)| id.clone())
            .collect();
        for id in &stale {
            self.remove(id);
        }
        stale
    }

    /// Get all peers reachable via a specific transport
    pub fn peers_on_transport(&self, transport: &TransportKind) -> Vec<&UnifiedPeer> {
        self.peers
            .values()
            .filter(|p| p.transports.contains_key(transport))
            .collect()
    }

    /// Get the best transport for a specific peer
    pub fn best_transport(&self, identity: &IdentityId) -> Option<(TransportKind, f64)> {
        self.peers.get(identity).and_then(|peer| {
            peer.transports
                .iter()
                .max_by(|a, b| {
                    a.1.success_rate
                        .partial_cmp(&b.1.success_rate)
                        .unwrap_or(std::cmp::Ordering::Equal)
                })
                .map(|(kind, addr)| (kind.clone(), addr.success_rate))
        })
    }

    /// Record a transmission outcome for Thompson Sampling
    pub fn record_outcome(
        &mut self,
        identity: &IdentityId,
        transport: &TransportKind,
        success: bool,
        rtt_ms: Option<u64>,
    ) {
        if let Some(peer) = self.peers.get_mut(identity) {
            if let Some(addr) = peer.transports.get_mut(transport) {
                if success {
                    addr.success_count += 1;
                } else {
                    addr.failure_count += 1;
                }
                let total = addr.success_count + addr.failure_count;
                addr.success_rate = if total > 0 {
                    addr.success_count as f64 / total as f64
                } else {
                    0.5
                };
                addr.rtt_ms = rtt_ms;
                addr.last_seen = Instant::now();
            }
            peer.last_seen = Instant::now();
            // Update overall quality as avg of transport success rates
            let rates: Vec<f64> = peer.transports.values().map(|t| t.success_rate).collect();
            if !rates.is_empty() {
                peer.quality = rates.iter().sum::<f64>() / rates.len() as f64;
            }
        }
    }

    pub fn peer_count(&self) -> usize {
        self.peers.len()
    }

    pub fn peer_count_for_transport(&self, transport: &TransportKind) -> usize {
        self.peers
            .values()
            .filter(|p| p.transports.contains_key(transport))
            .count()
    }

    pub fn summary(&self) -> PeerTableSummary {
        let with_libp2p = self
            .peers
            .values()
            .filter(|p| p.libp2p_peer_id.is_some())
            .count();
        let with_reticulum = self
            .peers
            .values()
            .filter(|p| p.reticulum_address.is_some())
            .count();
        let with_both = self
            .peers
            .values()
            .filter(|p| p.libp2p_peer_id.is_some() && p.reticulum_address.is_some())
            .count();
        let avg_quality = if self.peers.is_empty() {
            0.0
        } else {
            self.peers.values().map(|p| p.quality).sum::<f64>() / self.peers.len() as f64
        };
        PeerTableSummary {
            total_peers: self.peers.len(),
            peers_with_libp2p: with_libp2p,
            peers_with_reticulum: with_reticulum,
            peers_with_both: with_both,
            avg_quality,
        }
    }

    /// Get all peers (for iteration)
    pub fn all_peers(&self) -> &HashMap<IdentityId, UnifiedPeer> {
        &self.peers
    }
}
