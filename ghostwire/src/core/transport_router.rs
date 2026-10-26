//! transport_router.rs — Multi-transport routing via Unified Peer Table
//!
//! Selects the best transport for each peer using Thompson Sampling scores
//! from the UnifiedPeerTable. Supports dual routing for critical messages.

use crate::core::peer_table::{IdentityId, PeerQuery, TransportKind, UnifiedPeerTable};
use std::sync::Arc;
use tokio::sync::RwLock;

/// The result of transport selection for a specific peer
#[derive(Debug, Clone)]
pub enum RoutingDecision {
    /// Use libp2p (peer_id as string, multiaddr as string)
    LibP2p {
        peer_id: String,
        addr: String,
    },
    /// Use Reticulum (16-byte AddressHash)
    Reticulum {
        address_hash: [u8; 16],
    },
    /// Use both simultaneously (for critical messages)
    Dual {
        libp2p_peer_id: Option<String>,
        rns_address: Option<[u8; 16]>,
    },
    /// No route available
    Unreachable,
}

/// Message priority affects routing decisions
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessagePriority {
    Normal,
    Important,
    Critical,
}

/// Routes messages to peers across multiple transports
pub struct TransportRouter {
    peer_table: Arc<RwLock<UnifiedPeerTable>>,
}

impl TransportRouter {
    pub fn new(peer_table: Arc<RwLock<UnifiedPeerTable>>) -> Self {
        Self { peer_table }
    }

    /// Determine the best transport for a specific peer
    pub async fn route(
        &self,
        identity: &IdentityId,
        priority: MessagePriority,
    ) -> RoutingDecision {
        let table = self.peer_table.read().await;
        let peer = match table.lookup(&PeerQuery::Identity(identity.clone())) {
            Some(p) => p,
            None => return RoutingDecision::Unreachable,
        };

        let has_libp2p = peer.libp2p_peer_id.is_some();
        let has_rns = peer.reticulum_address.is_some();

        // Critical messages: dual routing if both transports available
        if priority == MessagePriority::Critical && has_libp2p && has_rns {
            return RoutingDecision::Dual {
                libp2p_peer_id: peer.libp2p_peer_id.clone(),
                rns_address: peer.reticulum_address,
            };
        }

        // Single transport: use what's available
        if has_libp2p && !has_rns {
            if let Some(ref pid) = peer.libp2p_peer_id {
                // Find the best libp2p transport address
                for kind in &[TransportKind::Mesh] {
                    if let Some(addr) = peer.transports.get(kind) {
                        if let crate::core::peer_table::TransportAddr::LibP2p(multiaddr) =
                            &addr.address
                        {
                            return RoutingDecision::LibP2p {
                                peer_id: pid.clone(),
                                addr: multiaddr.clone(),
                            };
                        }
                    }
                }
            }
            return RoutingDecision::Unreachable;
        }

        if has_rns && !has_libp2p {
            if let Some(addr) = peer.reticulum_address {
                return RoutingDecision::Reticulum { address_hash: addr };
            }
            return RoutingDecision::Unreachable;
        }

        // Both available: pick highest success_rate
        let mut best_kind: Option<TransportKind> = None;
        let mut best_rate: f64 = 0.0;
        for (kind, transport_addr) in &peer.transports {
            if transport_addr.success_rate > best_rate {
                best_rate = transport_addr.success_rate;
                best_kind = Some(kind.clone());
            }
        }

        match best_kind {
            Some(TransportKind::Mesh) => {
                if let Some(ref pid) = peer.libp2p_peer_id {
                    if let Some(addr) = peer.transports.get(&best_kind.unwrap_or(TransportKind::Mesh)) {
                        if let crate::core::peer_table::TransportAddr::LibP2p(multiaddr) =
                            &addr.address
                        {
                            return RoutingDecision::LibP2p {
                                peer_id: pid.clone(),
                                addr: multiaddr.clone(),
                            };
                        }
                    }
                }
                RoutingDecision::Unreachable
            }
            Some(TransportKind::Reticulum) => {
                if let Some(addr) = peer.reticulum_address {
                    RoutingDecision::Reticulum { address_hash: addr }
                } else {
                    RoutingDecision::Unreachable
                }
            }
            _ => RoutingDecision::Unreachable,
        }
    }

    /// Record the outcome of a send attempt for Thompson Sampling
    pub async fn record_outcome(
        &self,
        identity: &IdentityId,
        transport: &TransportKind,
        success: bool,
        rtt_ms: Option<u64>,
    ) {
        let mut table = self.peer_table.write().await;
        table.record_outcome(identity, transport, success, rtt_ms);
    }
}
