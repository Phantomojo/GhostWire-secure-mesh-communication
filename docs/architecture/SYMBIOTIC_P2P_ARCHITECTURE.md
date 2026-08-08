# GhostWire Symbiotic Hybrid P2P Architecture

> Version: 1.0  
> Status: Design Document  
> Last Updated: 2026-08-05  
> Authors: GhostWire Architecture Team

---

## Table of Contents

1. [Executive Summary](#1-executive-summary)
2. [Current State Audit](#2-current-state-audit)
3. [Symbiotic Architecture Design](#3-symbiotic-architecture-design)
   - 3.1 [Design Principles](#31-design-principles)
   - 3.2 [Unified Peer Table](#32-unified-peer-table)
   - 3.3 [Cross-Pollinate Discovery](#33-cross-pollinate-discovery)
   - 3.4 [Transport Bridging](#34-transport-bridging)
   - 3.5 [Reticulum Announce Integration](#35-reticulum-announce-integration)
   - 3.6 [Unified Addressing](#36-unified-addressing)
   - 3.7 [Disaster Response Integration](#37-disaster-response-integration)
4. [File-by-File Change Specification](#4-file-by-file-change-specification)
5. [Phased Implementation Plan](#5-phased-implementation-plan)
6. [Testing Strategy](#6-testing-strategy)
7. [Reference Projects](#7-reference-projects)
8. [Glossary](#8-glossary)

---

## 1. Executive Summary

GhostWire currently uses libp2p as its primary transport layer, with Reticulum Network Stack (RNS) partially integrated as a separate silo. The two systems operate independently: libp2p handles high-speed application protocols (gossipsub pubsub, Kademlia DHT, Identify, Sphinx encryption), while RNS provides overlay routing for constrained networks (radio, store-and-forward, off-grid operation).

The goal of this architecture is to make libp2p and RNS work in **symbiosis** -- sharing peer knowledge, cross-pollinating discovery, and bridging transports. When one transport discovers a peer, the other transport learns about it. When one transport fails, the other takes over. When disaster strikes and internet infrastructure collapses, RNS keeps the mesh alive via radio links.

**No other project combines libp2p + Reticulum in this way.** GhostWire is the first.

The symbiotic architecture delivers three critical capabilities:

1. **Unified Peer Discovery**: A peer discovered via mDNS (libp2p) is automatically registered with RNS, and vice versa. Radio-only peers discovered via RNS announcements become reachable through the unified peer table.

2. **Intelligent Transport Routing**: Thompson Sampling (already implemented in `transport_selector.rs`) trains on both transports, learning which works best under current conditions. The system routes through the optimal transport for each peer.

3. **Disaster Resilience**: When internet infrastructure fails, RNS via radio links keeps the mesh alive. The disaster response system automatically shifts priority to RNS when libp2p transports become unavailable.

This document specifies the complete architecture, file-by-file changes, phased implementation plan, and testing strategy for achieving this symbiotic integration.

### 1.1 Scope

This document covers:

- A comprehensive audit of every component requiring modification (grounded in exact file paths, line numbers, and code structures)
- The Unified Peer Table design that maps one identity to multiple transport addresses
- Cross-pollinate discovery: how libp2p and RNS share what they learn about peers
- Transport bridging: routing messages through the best available transport
- Reticulum announce integration: periodic discovery and GhostWire metadata exchange
- Unified addressing: collapsing three ID namespaces (UUID, PeerId, AddressHash) into one
- Disaster response integration: Thompson Sampling training on both transports
- File-by-file change specifications with exact struct/trait/function signatures
- A five-phase implementation plan with risk assessment
- A testing strategy covering unit, integration, and stress tests

### 1.2 Non-Goals

- Rewriting RNS protocol internals
- Replacing libp2p with RNS or vice versa
- Implementing new encryption schemes (Sphinx and RNS built-in encryption remain as-is)
- Changing the user-facing message API (only the internal routing changes)

---

## 2. Current State Audit

This section documents the exact current state of every component that needs modification. All references include file paths, line numbers, and specific code structures. This audit grounds every design decision in real code.

### 2.1 Component Inventory

| Component | File | Lines | Current State | What Needs to Change |
|-----------|------|-------|---------------|---------------------|
| Core `send_direct_message` | `core/mod.rs` | L1842 | Hardcoded `PeerId` parse | Unified recipient type (PeerId OR RNS AddressHash) |
| Mesh inbound event loop | `core/mesh.rs` | L861-931 | `tokio::select!` polls only `swarm.next()` | Add RNS receive channel arm |
| Mesh discovery | `core/mesh.rs` | L357-363, L365 | 100% libp2p (mDNS + Kademlia) | Add RNS announce listener + cross-registration |
| Mesh peer count | `core/mesh.rs` | L1463-1472 | Excludes RNS | Unified peer count across both transports |
| TransportRegistry | `core/transport.rs` | All | `HashMap<String, TransportKind>` -- no routing/peer layer | Add `UnifiedPeerTable` |
| TransportSelector | `core/transport_selector.rs` | L59-134 | Weight profiles for 6 transports, NO Reticulum | Add `reticulum()` weight profile + blackhole entry |
| DisasterPolicy priority lists | `core/disaster.rs` | L201-210 | Reticulum absent from all 4 priority lists | Add Reticulum to all levels |
| TransportManager | `core/transport_manager.rs` | L61-84 | `available_transports()` omits Reticulum | Add Reticulum to all disaster levels |
| active_transport chokepoint | `core/mod.rs` | L635 | Single `Arc<RwLock<Option<TransportKind>>>` | Replace with `TransportRouter` (multi-transport dispatch) |
| Config | `config.rs` | All | No hybrid/selection fields, no transport env vars | Add `hybrid_mode`, `discovery_interval`, `announce_interval` |
| CLI | `main.rs`, `cli/mod.rs` | All | No transport/RNS flags | Add `--hybrid`/`--reticulum-only`/`--libp2p-only` |
| Identity namespaces | `core_identity.rs`, `mesh.rs`, `reticulum.rs` | All | 3 distinct ID types (UUID, PeerId, AddressHash) | Unified `IdentityId` with per-transport derivations |
| Social graph | `core/social/social_graph.rs` | All | `register_discovered()` wired but `verify()` is stub | DID format unification + verification |
| HTTP API | `web.rs` | All | `StartupConfig.bootstrap_nodes` leaks `libp2p::Multiaddr` | Transport-agnostic address type |
| ReticulumTransport | `core/reticulum.rs` | All | No periodic announce, no routing table, no metrics | Periodic announce, routing table, Thompson Sampling metrics |
| RNS broadcast bug | `core/reticulum.rs` | ~L280 | `send_broadcast` uses `DestinationType::Single` with empty address | Fix to use `DestinationType::Broadcast` |
| mod.rs stubs | `core/mod.rs` | L2494-2527 | `ReticulumStats`/`ReticulumTopology` stubs differ from feature-gated versions | Remove stubs, use feature-gated types |

### 2.2 Detailed Component Analysis

#### 2.2.1 Core send_direct_message (`core/mod.rs` L1842)

The current implementation hardcodes libp2p PeerId parsing:

```rust
// Current: core/mod.rs L1842
pub async fn send_direct_message(&self, recipient: &str, message: Vec<u8>) -> Result<(), String> {
    let peer_id = recipient.parse::<PeerId>()
        .map_err(|e| format!("Invalid PeerId: {}", e))?;
    // ... sends via active_transport (single Arc<RwLock<Option<TransportKind>>>)
}
```

**Problem**: Recipient is always a libp2p PeerId. RNS peers (identified by AddressHash) are unreachable through this API.

**Required change**: Accept a `UnifiedRecipient` enum that can be either a PeerId or an RNS AddressHash:

```rust
enum UnifiedRecipient {
    LibP2p(PeerId),
    Reticulum([u8; 16]),  // AddressHash
    Identity(IdentityId),  // canonical, resolved via UnifiedPeerTable
}
```

#### 2.2.2 Mesh Inbound Event Loop (`core/mesh.rs` L861-931)

The current event loop only polls libp2p:

```rust
// Current: core/mesh.rs L861-931
tokio::select! {
    event = swarm.next() => {
        // handle libp2p events
    }
    // ... other arms
}
```

**Problem**: RNS messages are never received in this loop. They are handled separately in `ReticulumManager`, with no cross-transport coordination.

**Required change**: Add an RNS receive channel arm:

```rust
tokio::select! {
    event = swarm.next() => {
        // handle libp2p events
    }
    rns_event = rns_receiver.recv() => {
        // handle RNS events, register in UnifiedPeerTable
    }
    // ... other arms
}
```

#### 2.2.3 TransportRegistry (`core/transport.rs`)

Current structure:

```rust
// Current: core/transport.rs
struct TransportRegistry {
    transports: HashMap<String, TransportKind>,
    // No peer routing, no cross-transport coordination
}
```

**Problem**: Registry tracks transport types but has no awareness of peers or routing. Cannot answer "which transports can reach peer X?"

**Required change**: Embed `UnifiedPeerTable` in the registry:

```rust
struct TransportRegistry {
    transports: HashMap<String, TransportKind>,
    peer_table: Arc<RwLock<UnifiedPeerTable>>,  // NEW
}
```

#### 2.2.4 TransportSelector (`core/transport_selector.rs` L59-134)

Current weight profiles:

```rust
// Current: core/transport_selector.rs L59-134
fn default_weight_profiles() -> HashMap<TransportKind, WeightProfile> {
    let mut profiles = HashMap::new();
    profiles.insert(TransportKind::Quic, quic_profile());
    profiles.insert(TransportKind::StealthTcp, stealth_tcp_profile());
    profiles.insert(TransportKind::Mesh, mesh_profile());
    profiles.insert(TransportKind::Bluetooth, bluetooth_profile());
    profiles.insert(TransportKind::WiFi, wifi_profile());
    profiles.insert(TransportKind::Blackhole, blackhole_profile());
    // NO Reticulum profile
    profiles
}
```

**Problem**: Thompson Sampling has no Reticulum weight profile. RNS transport is invisible to the selection algorithm.

**Required change**: Add `reticulum()` weight profile:

```rust
fn reticulum() -> WeightProfile {
    WeightProfile {
        base_success_rate: 0.7,
        latency_factor: 2.0,
        loss_factor: 0.1,
        recovery_bonus: 0.15,
    }
}
```

#### 2.2.5 DisasterPolicy Priority Lists (`core/disaster.rs` L201-210)

Current default:

```rust
// Current: core/disaster.rs L201-210
impl Default for DisasterPolicy {
    fn default() -> Self {
        Self {
            priority_lists: HashMap::from([
                (DisasterLevel::Normal, vec![TransportType::Quic, TransportType::StealthTcp, TransportType::Mesh, TransportType::WiFi]),
                (DisasterLevel::Congested, vec![TransportType::Mesh, TransportType::WiFi, TransportType::Bluetooth]),
                (DisasterLevel::Isolation, vec![TransportType::WiFi, TransportType::Bluetooth, TransportType::Mesh]),
                (DisasterLevel::Blackout, vec![TransportType::Bluetooth, TransportType::LoRa]),
            ]),
        }
    }
}
```

**Problem**: Reticulum is absent from ALL four priority lists despite `CAPABILITY_RETICULUM` being defined at L13.

**Required change**: Add Reticulum to all disaster levels:

```rust
(DisasterLevel::Normal, vec![
    TransportType::Quic, TransportType::StealthTcp, TransportType::Mesh,
    TransportType::Reticulum, TransportType::WiFi
]),
(DisasterLevel::Congested, vec![
    TransportType::Mesh, TransportType::Reticulum, TransportType::WiFi,
    TransportType::Bluetooth
]),
(DisasterLevel::Isolation, vec![
    TransportType::Reticulum, TransportType::WiFi, TransportType::Bluetooth,
    TransportType::Mesh
]),
(DisasterLevel::Blackout, vec![
    TransportType::Reticulum, TransportType::Bluetooth, TransportType::LoRa
]),
```

#### 2.2.6 TransportManager (`core/transport_manager.rs` L61-84)

Current `available_transports()`:

```rust
// Current: core/transport_manager.rs L61-84
pub fn available_transports(&self, level: DisasterLevel) -> Vec<TransportKind> {
    match level {
        DisasterLevel::Normal => vec![
            TransportKind::Quic, TransportKind::StealthTcp,
            TransportKind::Mesh, TransportKind::WiFi
        ],
        DisasterLevel::Congested => vec![
            TransportKind::Mesh, TransportKind::WiFi, TransportKind::Bluetooth
        ],
        DisasterLevel::Isolation => vec![
            TransportKind::WiFi, TransportKind::Bluetooth, TransportKind::Mesh
        ],
        DisasterLevel::Blackout => vec![TransportKind::Bluetooth],
    }
}
```

**Problem**: Reticulum is omitted from all disaster levels.

**Required change**: Add `TransportKind::Reticulum` to every level:

```rust
pub fn available_transports(&self, level: DisasterLevel) -> Vec<TransportKind> {
    match level {
        DisasterLevel::Normal => vec![
            TransportKind::Quic, TransportKind::StealthTcp, TransportKind::Mesh,
            TransportKind::Reticulum, TransportKind::WiFi
        ],
        DisasterLevel::Congested => vec![
            TransportKind::Mesh, TransportKind::Reticulum, TransportKind::WiFi,
            TransportKind::Bluetooth
        ],
        DisasterLevel::Isolation => vec![
            TransportKind::Reticulum, TransportKind::WiFi, TransportKind::Bluetooth,
            TransportKind::Mesh
        ],
        DisasterLevel::Blackout => vec![TransportKind::Reticulum, TransportKind::Bluetooth],
    }
}
```

#### 2.2.7 active_transport Chokepoint (`core/mod.rs` L635)

Current structure:

```rust
// Current: core/mod.rs L635
struct Core {
    active_transport: Arc<RwLock<Option<TransportKind>>>,
    // ... other fields
}
```

**Problem**: Single transport at a time. No multi-transport dispatch. Cannot route to a peer via the best available transport.

**Required change**: Replace with `TransportRouter`:

```rust
struct Core {
    transport_router: Arc<RwLock<TransportRouter>>,  // replaces active_transport
    peer_table: Arc<RwLock<UnifiedPeerTable>>,        // NEW
    // ... other fields
}
```

#### 2.2.8 ReticulumTransport (`core/reticulum.rs`)

Current state:

- Sends one announce at startup (L124), never again
- No routing table for known peers
- No metrics collection for Thompson Sampling
- `send_broadcast` bug: uses `DestinationType::Single` with empty address instead of `DestinationType::Broadcast`
- No periodic re-announce to maintain discoverability

**Required changes**:

- Periodic announce every 300s (configurable)
- Routing table for known peers (from announces)
- Thompson Sampling integration (record success/failure per transport)
- Fix `send_broadcast` to use correct `DestinationType::Broadcast`
- Extended announce payload with GhostWire metadata

#### 2.2.9 Identity Namespace Fragmentation

Three distinct ID types exist across the codebase:

| Type | Location | Format | Use |
|------|----------|--------|-----|
| UUID | `mesh.rs`, social graph | `uuid::Uuid` | Internal peer identification |
| PeerId | `mesh.rs`, libp2p | `libp2p::PeerId` | libp2p transport |
| AddressHash | `reticulum.rs` | `[u8; 16]` | RNS transport |

**Problem**: No mapping between these types. A peer known by PeerId cannot be reached via RNS AddressHash.

**Required change**: Unified `IdentityId` with deterministic derivation to all transport-specific types.

#### 2.2.10 Social Graph Verification Stub (`core/social/social_graph.rs`)

`register_discovered()` is wired but `verify()` is a stub. The social graph cannot verify peer identity across transports.

**Required change**: DID format unification. `verify()` must validate that a DID resolves to the same `IdentityId` regardless of which transport was used to discover the peer.

#### 2.2.11 HTTP API Bootstrap Nodes (`web.rs`)

`StartupConfig.bootstrap_nodes` currently uses `Vec<libp2p::Multiaddr>`. This leaks transport-specific types into the API layer.

**Required change**: Use a transport-agnostic address type:

```rust
struct BootstrapNode {
    identity: IdentityId,
    addresses: Vec<TransportAddress>,
}
```

#### 2.2.12 mod.rs Stub Types (`core/mod.rs` L2494-2527)

`ReticulumStats` and `ReticulumTopology` stubs at L2494-2527 differ from the feature-gated versions. These are dead code that creates confusion.

**Required change**: Remove stubs entirely. Import and use the feature-gated types from `reticulum.rs`.

---

## 3. Symbiotic Architecture Design

### 3.1 Design Principles

The symbiotic architecture is governed by five principles:

**Principle 1: RNS does cross-medium routing natively.**

Reticulum is purpose-built for constrained networks. It handles radio links, store-and-forward, and heterogeneous network segments without application-level plumbing. Use RNS gateway/boundary interface modes for radio-to-internet edge bridging. Do not reinvent what RNS already does well.

RNS interface modes relevant to GhostWire:

| Mode | Description | GhostWire Use Case |
|------|-------------|-------------------|
| `full` | Default. All discovery/meshing/transport. 7-day path expiry. | Standard internet-connected nodes |
| `gateway` (gw) | Full + discovers unknown paths on behalf of clients | Bridge nodes between radio and internet segments |
| `access_point` (ap) | Silent until requested. 24h path expiry. | Radio-only nodes (minimal bandwidth) |
| `roaming` | Mobile interfaces. 6h path expiry. Only propagates to full. | Mobile devices moving between segments |
| `boundary` | Segment edge between heterogeneous networks (LoRa <-> Internet). Controls announce propagation. | Gateway nodes connecting LoRa mesh to internet |
| `internal` | Protected inner segment. Announces from boundary don't propagate here. | Sensitive internal networks |

**Announce propagation rules** (from RNS specification):

| FROM \ TO | Full/GW | AP | Boundary | Internal | Roaming |
|-----------|---------|-----|----------|----------|---------|
| Full/GW | Y | N | Y | Y | Y |
| AP | Y | N | Y | Y | Y |
| Boundary | Y | N | Y | N | N |
| Internal | Y | N | Y | Y | N |
| Roaming | Y | N | N | Y | N |

These propagation rules matter for GhostWire because announce-based discovery must respect the RNS topology. Gateway-mode nodes can bridge between segments, but boundary nodes control which announces propagate.

**Principle 2: libp2p does high-speed application protocols.**

libp2p provides gossipsub pubsub, Kademlia DHT, Identify protocol, and Sphinx encryption at scale. These are high-bandwidth, low-latency protocols that require reliable infrastructure. Use libp2p for the fast path when internet connectivity is available.

libp2p protocols relevant to the symbiotic architecture:

| Protocol | Purpose | Symbiotic Role |
|----------|---------|---------------|
| gossipsub | Pub/sub messaging | Cross-transport announce topic |
| Kademlia | DHT-based peer lookup | Store peer records for RNS-only peers |
| Identify | Exchange metadata on connection | Exchange RNS address during handshake |
| autonat | NAT detection | Determine if node is publicly reachable |
| ping | Liveness check | Health monitoring for transport selection |

**Principle 3: Shared identity layer.**

One `master_seed` derives per-transport keys (already implemented in `core_identity.rs` L763-771). This is the foundation of unified identity. The `derive_transport_key()` method produces deterministic keys for each transport, enabling cross-transport identity verification.

```rust
// Existing: core_identity.rs L763-771
pub fn derive_transport_key(&self, transport_name: &str) -> Option<[u8; 32]> {
    let master_seed = self.master_seed.read().ok()?;
    let seed = master_seed.as_ref()?;
    let domain = format!("ghostwire-transport-identity-v1-{}", transport_name);
    let derived = genesis_derive(
        SPHINX_KDF_DOMAIN,
        &[seed.as_slice(), domain.as_bytes()].concat()
    );
    Some(derived)
}
```

This derivation scheme means:

- The same `master_seed` produces a different key for each transport
- A peer's libp2p PeerId and RNS AddressHash are both derivable from the same seed
- No central authority is needed to map between transport identities
- Verification is possible: derive the expected PeerId from a known master_seed, compare with claimed PeerId

**Principle 4: Bridge at the application layer, not the transport layer.**

Do not attempt to make libp2p speak RNS protocol or vice versa. Bridge at the application layer: bidirectional socket-to-RNS.Buffer for streams, RNS Packet for small messages. Each transport remains native; coordination happens in the application logic.

This principle is informed by the radicle-reticulum project, which successfully bridges TCP and RNS using RNS.Buffer for stream data and native RNS Packets for small messages. The bridge does not modify either protocol -- it translates at the application layer.

**Principle 5: Both transports feed Thompson Sampling.**

The `transport_selector.rs` Thompson Sampling algorithm already learns which transport works best per condition. Extend it to train on both libp2p and RNS outcomes. The algorithm does not care which transport delivered the message -- it only cares about success rate, latency, and loss rate.

Thompson Sampling works by maintaining a Beta distribution for each transport:

```
success_rate ~ Beta(alpha, beta)
alpha = successes + 1
beta = failures + 1
```

Each selection round samples from each transport's Beta distribution and picks the highest sample. This naturally explores underperforming transports while exploiting known-good ones.

### 3.2 Unified Peer Table

The `UnifiedPeerTable` is the central data structure of the symbiotic architecture. It maps a single peer identity to multiple transport addresses, enabling cross-transport routing and discovery.

#### 3.2.1 Core Data Structures

```rust
// NEW: ghostwire/src/core/peer_table.rs

use std::collections::HashMap;
use std::time::Instant;
use libp2p::{Multiaddr, PeerId};

/// Canonical identity -- deterministic from master_seed via BLAKE3
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
struct IdentityId([u8; 32]);

/// A single peer identity that maps to multiple transport addresses.
/// This is the canonical representation of a peer in the GhostWire mesh.
struct UnifiedPeer {
    /// Canonical identity -- derived from master_seed via BLAKE3
    identity_id: IdentityId,

    /// libp2p PeerId (derived deterministically from identity_id)
    libp2p_peer_id: Option<libp2p::PeerId>,

    /// Reticulum AddressHash (deterministic from master_seed)
    reticulum_address: Option<[u8; 16]>,

    /// Per-transport addresses and quality metrics
    transports: HashMap<TransportKind, TransportAddress>,

    /// Last seen on any transport
    last_seen: Instant,

    /// Quality score (0.0-1.0) updated by Thompson Sampling
    quality: f64,

    /// Capabilities advertised by this peer
    capabilities: Vec<String>,

    /// Version string from handshake
    version: Option<String>,
}

/// Transport-specific address with quality metrics
struct TransportAddress {
    /// The actual address (Multiaddr for libp2p, AddressHash for RNS)
    address: TransportAddr,

    /// Last time this specific transport address was verified
    last_seen: Instant,

    /// Round-trip time in milliseconds (if measured)
    rtt_ms: Option<u64>,

    /// Success rate from Thompson Sampling (0.0-1.0)
    success_rate: f64,

    /// Number of successful transmissions on this transport
    success_count: u64,

    /// Number of failed transmissions on this transport
    failure_count: u64,
}

/// Transport-specific address enum
enum TransportAddr {
    LibP2p(Multiaddr),
    Reticulum([u8; 16]),  // AddressHash
}

/// The unified peer table mapping identities to transport addresses
struct UnifiedPeerTable {
    /// Primary lookup: IdentityId -> UnifiedPeer
    peers: HashMap<IdentityId, UnifiedPeer>,

    /// Reverse lookup: libp2p PeerId -> IdentityId
    by_libp2p_peer_id: HashMap<libp2p::PeerId, IdentityId>,

    /// Reverse lookup: RNS AddressHash -> IdentityId
    by_reticulum_address: HashMap<[u8; 16], IdentityId>,

    /// Maximum age before a peer entry is considered stale (configurable)
    max_stale_duration: std::time::Duration,

    /// Maximum number of peers (prevents unbounded growth)
    max_peers: usize,
}
```

#### 3.2.2 Core Operations

```rust
impl UnifiedPeerTable {
    /// Create a new empty peer table
    fn new(max_peers: usize, max_stale_duration: std::time::Duration) -> Self;

    /// Upsert a peer discovered via any transport.
    /// If the peer already exists (matched by IdentityId, PeerId, or AddressHash),
    /// merge the new transport information.
    fn upsert(
        &mut self,
        identity: IdentityId,
        transport: TransportKind,
        address: TransportAddr,
    );

    /// Look up a peer by any identifier type.
    /// Tries IdentityId first, then PeerId, then AddressHash.
    fn lookup(&self, query: &PeerQuery) -> Option<&UnifiedPeer>;

    /// Remove a peer by IdentityId
    fn remove(&mut self, identity: &IdentityId);

    /// Remove all stale entries (last_seen > max_stale_duration)
    fn gc_stale(&mut self) -> Vec<IdentityId>;

    /// Get all peers reachable via a specific transport
    fn peers_on_transport(&self, transport: TransportKind) -> Vec<&UnifiedPeer>;

    /// Get the best transport for a specific peer (highest success_rate)
    fn best_transport(&self, identity: &IdentityId) -> Option<(TransportKind, f64)>;

    /// Record a transmission outcome (success/failure) for Thompson Sampling
    fn record_outcome(
        &mut self,
        identity: &IdentityId,
        transport: TransportKind,
        success: bool,
        rtt_ms: Option<u64>,
    );

    /// Get total peer count (across all transports)
    fn peer_count(&self) -> usize;

    /// Get transport-specific peer count
    fn peer_count_for_transport(&self, transport: TransportKind) -> usize;

    /// Get a summary for metrics/monitoring
    fn summary(&self) -> PeerTableSummary;
}

/// Query type for flexible peer lookup
enum PeerQuery {
    Identity(IdentityId),
    LibP2p(PeerId),
    Reticulum([u8; 16]),
}

/// Summary for monitoring
struct PeerTableSummary {
    total_peers: usize,
    peers_with_libp2p: usize,
    peers_with_reticulum: usize,
    peers_with_both: usize,
    avg_quality: f64,
}
```

#### 3.2.3 Upsert Logic

The upsert operation is the core of cross-transport discovery. When any transport discovers a peer, upsert merges that information:

```rust
fn upsert(
    &mut self,
    identity: IdentityId,
    transport: TransportKind,
    address: TransportAddr,
) {
    // Check if peer already exists by any known identifier
    let existing = self.peers.get_mut(&identity);

    if let Some(peer) = existing {
        // Merge: add/update this transport's address
        let transport_addr = TransportAddress {
            address,
            last_seen: Instant::now(),
            rtt_ms: None,
            success_rate: 0.5,  // start neutral
            success_count: 0,
            failure_count: 0,
        };
        peer.transports.insert(transport, transport_addr);
        peer.last_seen = Instant::now();
    } else {
        // New peer: create entry
        let mut peer = UnifiedPeer {
            identity_id: identity.clone(),
            libp2p_peer_id: None,
            reticulum_address: None,
            transports: HashMap::new(),
            last_seen: Instant::now(),
            quality: 0.5,
            capabilities: Vec::new(),
            version: None,
        };

        // Set the transport-specific identifier
        match &address {
            TransportAddr::LibP2p(multiaddr) => {
                // Derive PeerId from multiaddr or extract from handshake
                // peer.libp2p_peer_id = Some(derived_peer_id);
            }
            TransportAddr::Reticulum(addr_hash) => {
                peer.reticulum_address = Some(*addr_hash);
                self.by_reticulum_address.insert(*addr_hash, identity.clone());
            }
        }

        peer.transports.insert(transport, TransportAddress {
            address,
            last_seen: Instant::now(),
            rtt_ms: None,
            success_rate: 0.5,
            success_count: 0,
            failure_count: 0,
        });

        self.peers.insert(identity.clone(), peer);

        // Maintain max_peers limit via LRU eviction
        if self.peers.len() > self.max_peers {
            self.evict_stalest();
        }
    }
}
```

#### 3.2.4 Lookup Logic

```rust
fn lookup(&self, query: &PeerQuery) -> Option<&UnifiedPeer> {
    match query {
        PeerQuery::Identity(id) => self.peers.get(id),
        PeerQuery::LibP2p(peer_id) => {
            self.by_libp2p_peer_id.get(peer_id)
                .and_then(|id| self.peers.get(id))
        }
        PeerQuery::Reticulum(addr_hash) => {
            self.by_reticulum_address.get(addr_hash)
                .and_then(|id| self.peers.get(id))
        }
    }
}
```

#### 3.2.5 Garbage Collection

Stale peer entries must be cleaned up to prevent unbounded memory growth:

```rust
fn gc_stale(&mut self) -> Vec<IdentityId> {
    let now = Instant::now();
    let stale: Vec<IdentityId> = self.peers.iter()
        .filter(|(_, peer)| now.duration_since(peer.last_seen) > self.max_stale_duration)
        .map(|(id, _)| id.clone())
        .collect();

    for id in &stale {
        if let Some(peer) = self.peers.remove(id) {
            // Clean up reverse indexes
            if let Some(pid) = peer.libp2p_peer_id {
                self.by_libp2p_peer_id.remove(&pid);
            }
            if let Some(addr) = peer.reticulum_address {
                self.by_reticulum_address.remove(&addr);
            }
        }
    }

    stale
}
```

#### 3.2.6 Thread Safety

The `UnifiedPeerTable` is wrapped in `Arc<RwLock<UnifiedPeerTable>>` for shared access across async tasks:

```rust
// In Core struct (core/mod.rs)
struct Core {
    peer_table: Arc<RwLock<UnifiedPeerTable>>,
    // ... other fields
}
```

All reads take a read lock; writes take a write lock. Since peer table operations are fast (HashMap lookups), lock contention is minimal.

#### 3.2.7 Persistence

The peer table should be periodically persisted to disk for fast startup recovery:

```rust
impl UnifiedPeerTable {
    /// Serialize to JSON for persistence
    fn to_json(&self) -> String;

    /// Deserialize from JSON for startup recovery
    fn from_json(json: &str) -> Result<Self, serde_json::Error>;

    /// Save to disk (configurable path)
    fn save_to_disk(&self, path: &Path) -> io::Result<()>;

    /// Load from disk (if exists)
    fn load_from_disk(path: &Path) -> Result<Self, io::Error>;
}
```

On startup, the node loads its peer table from disk, then refreshes entries via active discovery (mDNS, Kademlia, RNS announces). This provides immediate routing knowledge without waiting for full discovery.

### 3.3 Cross-Pollinate Discovery

Cross-pollinate discovery is the mechanism by which one transport's discoveries feed into the other transport's routing knowledge. This is the core of the symbiotic architecture.

#### 3.3.1 Discovery Flow Overview

```
                        Discovery Events
                              |
              +---------------+---------------+
              |                               |
    libp2p Discovery                  RNS Discovery
    (mDNS, Kademlia)                 (Announces)
              |                               |
              v                               v
    +-------------------+         +-------------------+
    | Peer Discovered   |         | Peer Discovered   |
    | via libp2p        |         | via RNS           |
    +-------------------+         +-------------------+
              |                               |
              v                               v
    +-----------------------------------------------+
    |         UnifiedPeerTable.upsert()              |
    |  (merges transport info into canonical peer)   |
    +-----------------------------------------------+
              |                               |
              v                               v
    +-------------------+         +-------------------+
    | Cross-Register:   |         | Cross-Register:   |
    | if RNS addr known |         | if libp2p addr    |
    | register with RNS |         | known, register   |
    +-------------------+         | with Kademlia     |
              |                   +-------------------+
              v                               |
    +-------------------+                    v
    | Thompson Sampling |         +-------------------+
    | record_outcome()  |         | Thompson Sampling |
    +-------------------+         | record_outcome()  |
                                  +-------------------+
```

#### 3.3.2 libp2p -> RNS (libp2p discovers, registers with RNS)

When Kademlia or mDNS finds a new peer:

1. **Extract RNS address from handshake**: The libp2p Identify protocol exchanges metadata. Extend the handshake to include the peer's RNS AddressHash.

2. **Register in UnifiedPeerTable**: `upsert(identity, TransportKind::LibP2p, multiaddr)`

3. **Cross-register with RNS**: If the handshake included an RNS address:
   ```rust
   if let Some(rns_addr) = handshake.reticulum_address {
       reticulum_manager.register_known_peer(identity, rns_addr);
   }
   ```

4. **Record outcome for Thompson Sampling**:
   ```rust
   peer_table.record_outcome(&identity, TransportKind::Quic, true, Some(rtt_ms));
   ```

#### 3.3.3 RNS -> libp2p (RNS discovers, registers with Kademlia)

When RNS receives an announce from another GhostWire node:

1. **Extract GhostWire metadata from announce**: The extended announce payload includes multiaddrs and PeerId.

2. **Register in UnifiedPeerTable**: `upsert(identity, TransportKind::Reticulum, address_hash)`

3. **Cross-register with Kademlia**: If the announce included libp2p multiaddrs:
   ```rust
   if let Some(multiaddrs) = announce.multiaddrs {
       for addr in multiaddrs {
           kademlia.add_address(&peer_id, addr);
       }
   }
   ```

4. **Record outcome for Thompson Sampling**:
   ```rust
   peer_table.record_outcome(&identity, TransportKind::Reticulum, true, None);
   ```

#### 3.3.4 Discovery Event Flow (Detailed)

```rust
// In mesh.rs -- extended event handler
async fn handle_discovery_event(
    &self,
    event: DiscoveryEvent,
    peer_table: &Arc<RwLock<UnifiedPeerTable>>,
    reticulum: &Option<Arc<ReticulumManager>>,
    kademlia: &mut Kademlia<MemoryStore>,
) {
    match event {
        DiscoveryEvent::LibP2pPeerFound { peer_id, multiaddrs, handshake } => {
            // 1. Register in unified table
            let identity = handshake.identity_id;
            {
                let mut table = peer_table.write().await;
                table.upsert(
                    identity.clone(),
                    TransportKind::Quic,
                    TransportAddr::LibP2p(multiaddrs[0].clone()),
                );

                // Set PeerId mapping
                if let Some(peer) = table.lookup(&PeerQuery::Identity(identity.clone())) {
                    // peer.libp2p_peer_id = Some(peer_id);
                }
            }

            // 2. Cross-register with RNS if RNS address known
            if let Some(rns_addr) = handshake.reticulum_address {
                if let Some(rns) = reticulum {
                    rns.register_known_peer(identity.clone(), rns_addr);
                }

                let mut table = peer_table.write().await;
                table.upsert(
                    identity,
                    TransportKind::Reticulum,
                    TransportAddr::Reticulum(rns_addr),
                );
            }

            // 3. Record success for Thompson Sampling
            {
                let mut table = peer_table.write().await;
                table.record_outcome(&identity, TransportKind::Quic, true, None);
            }
        }

        DiscoveryEvent::RnsAnnounceReceived { address_hash, announce_data } => {
            // 1. Parse GhostWire metadata from announce
            let metadata = match parse_ghostwire_announce(&announce_data) {
                Some(m) => m,
                None => return,  // not a GhostWire announce
            };

            // 2. Register in unified table
            {
                let mut table = peer_table.write().await;
                table.upsert(
                    metadata.identity_id.clone(),
                    TransportKind::Reticulum,
                    TransportAddr::Reticulum(address_hash),
                );
            }

            // 3. Cross-register with Kademlia if multiaddrs known
            if let Some(multiaddrs) = metadata.multiaddrs {
                for addr in multiaddrs {
                    if let Some(peer_id) = metadata.libp2p_peer_id {
                        kademlia.add_address(&peer_id, addr);
                    }
                }
            }

            // 4. Record success for Thompson Sampling
            {
                let mut table = peer_table.write().await;
                table.record_outcome(
                    &metadata.identity_id,
                    TransportKind::Reticulum,
                    true,
                    None,
                );
            }
        }
    }
}
```

#### 3.3.5 gossipsub Announce Topic

A dedicated gossipsub topic enables cross-transport peer discovery:

```rust
const ANNOUNCE_TOPIC: &str = "ghostwire/announce";

/// Announce payload published on the gossipsub topic
struct GhostWireAnnounce {
    /// Protocol version
    version: u32,

    /// Peer's canonical identity
    identity_id: IdentityId,

    /// libp2p PeerId (for cross-reference)
    libp2p_peer_id: Option<PeerId>,

    /// RNS AddressHash (for cross-reference)
    reticulum_address: Option<[u8; 16]>,

    /// Multiaddrs this peer is reachable on (libp2p)
    multiaddrs: Vec<Multiaddr>,

    /// Capabilities (sphinx, dtn, rns, etc.)
    capabilities: Vec<String>,

    /// Software version
    software_version: String,

    /// Timestamp
    timestamp: u64,
}
```

Each node publishes its announce every N seconds (configurable, default 60s). Peers on different transports receive these announces and can cross-register.

**Frequency**: Every 60 seconds (configurable via `discovery_interval`)

**Payload size**: ~200 bytes (well within gossipsub limits)

**Propagation**: gossipsub handles propagation to all subscribed peers, regardless of which transport they are on.

#### 3.3.6 Discovery Interval Configuration

```rust
// In config.rs
struct DiscoveryConfig {
    /// How often to publish our announce on gossipsub (seconds)
    announce_interval: u64,  // default: 60

    /// How often to check for stale peers (seconds)
    gc_interval: u64,  // default: 300

    /// How often to re-announce on RNS (seconds)
    rns_announce_interval: u64,  // default: 300

    /// Maximum peer table size
    max_peers: usize,  // default: 1000

    /// Maximum age before peer is considered stale (seconds)
    max_stale_duration: u64,  // default: 3600 (1 hour)
}
```

### 3.4 Transport Bridging

When a peer is reachable via multiple transports, the system routes through the best one based on Thompson Sampling scores, current disaster level, and peer capabilities.

#### 3.4.1 Routing Decision Types

```rust
/// The result of transport selection for a specific peer
enum RoutingDecision {
    /// Use libp2p only (fast, reliable, high-bandwidth)
    LibP2p {
        peer_id: PeerId,
        addr: Multiaddr,
    },

    /// Use Reticulum only (radio-compatible, store-and-forward, works off-grid)
    Reticulum {
        address_hash: [u8; 16],
    },

    /// Use both simultaneously (redundancy for critical messages)
    Dual {
        libp2p: Option<PeerId>,
        rns: Option<[u8; 16]>,
    },

    /// No route available
    Unreachable,
}
```

#### 3.4.2 TransportRouter

```rust
// NEW: ghostwire/src/core/transport_router.rs

/// Routes messages to peers across multiple transports
struct TransportRouter {
    /// The unified peer table (shared with Core)
    peer_table: Arc<RwLock<UnifiedPeerTable>>,

    /// Transport selector (Thompson Sampling)
    selector: Arc<RwLock<TransportSelector>>,

    /// libp2p mesh manager (for sending via libp2p)
    mesh_manager: Option<Arc<MeshManager>>,

    /// RNS manager (for sending via Reticulum)
    reticulum_manager: Option<Arc<ReticulumManager>>,

    /// Current disaster level (affects transport priority)
    disaster_level: Arc<RwLock<DisasterLevel>>,
}

impl TransportRouter {
    /// Determine the best transport for a specific peer
    fn route(
        &self,
        recipient: &PeerQuery,
        priority: MessagePriority,
    ) -> RoutingDecision;

    /// Send a message via the best available transport
    async fn send(
        &self,
        recipient: &PeerQuery,
        message: Vec<u8>,
        priority: MessagePriority,
    ) -> Result<(), TransportError>;

    /// Record the outcome of a send attempt
    fn record_outcome(
        &self,
        identity: &IdentityId,
        transport: TransportKind,
        success: bool,
        rtt_ms: Option<u64>,
    );
}

/// Message priority affects routing decisions
enum MessagePriority {
    /// Normal message: use best transport
    Normal,

    /// Important message: prefer reliable transport
    Important,

    /// Critical message (disaster, security): use dual routing
    Critical,
}
```

#### 3.4.3 Routing Logic

```rust
impl TransportRouter {
    fn route(
        &self,
        recipient: &PeerQuery,
        priority: MessagePriority,
    ) -> RoutingDecision {
        // 1. Look up recipient in UnifiedPeerTable
        let table = self.peer_table.read().unwrap();
        let peer = match table.lookup(recipient) {
            Some(p) => p,
            None => return RoutingDecision::Unreachable,
        };

        // 2. Check which transports are available for this peer
        let has_libp2p = peer.libp2p_peer_id.is_some();
        let has_rns = peer.reticulum_address.is_some();

        // 3. For critical messages: use dual routing
        if let MessagePriority::Critical = priority {
            if has_libp2p && has_rns {
                return RoutingDecision::Dual {
                    libp2p: peer.libp2p_peer_id,
                    rns: peer.reticulum_address,
                };
            }
            // Fall through to single-transport if only one available
        }

        // 4. If peer is only on one transport, use it
        if has_libp2p && !has_rns {
            let addr = peer.transports.get(&TransportKind::Quic)
                .or_else(|| peer.transports.get(&TransportKind::StealthTcp))
                .map(|ta| ta.address.clone());
            if let Some(TransportAddr::LibP2p(multiaddr)) = addr {
                return RoutingDecision::LibP2p {
                    peer_id: peer.libp2p_peer_id.unwrap(),
                    addr: multiaddr,
                };
            }
        }

        if has_rns && !has_libp2p {
            return RoutingDecision::Reticulum {
                address_hash: peer.reticulum_address.unwrap(),
            };
        }

        // 5. If peer is on both: use Thompson Sampling to pick best
        let selector = self.selector.read().unwrap();
        let disaster = self.disaster_level.read().unwrap();

        // Get available transports for current disaster level
        let available = available_transports_for_level(*disaster);

        // Score each available transport for this peer
        let mut best_transport: Option<TransportKind> = None;
        let mut best_score: f64 = 0.0;

        for transport in &available {
            if peer.transports.contains_key(transport) {
                let score = selector.sample(transport);
                if score > best_score {
                    best_score = score;
                    best_transport = Some(*transport);
                }
            }
        }

        match best_transport {
            Some(TransportKind::Quic) | Some(TransportKind::StealthTcp) => {
                let addr = peer.transports.get(&best_transport.unwrap())
                    .map(|ta| ta.address.clone());
                if let Some(TransportAddr::LibP2p(multiaddr)) = addr {
                    RoutingDecision::LibP2p {
                        peer_id: peer.libp2p_peer_id.unwrap(),
                        addr: multiaddr,
                    }
                } else {
                    RoutingDecision::Unreachable
                }
            }
            Some(TransportKind::Reticulum) => {
                RoutingDecision::Reticulum {
                    address_hash: peer.reticulum_address.unwrap(),
                }
            }
            _ => RoutingDecision::Unreachable,
        }
    }
}
```

#### 3.4.4 send_direct_message Refactor

The current `send_direct_message` at `core/mod.rs` L1842 must be refactored to use `TransportRouter`:

```rust
// Refactored: core/mod.rs
pub async fn send_direct_message(
    &self,
    recipient: &UnifiedRecipient,
    message: Vec<u8>,
) -> Result<(), String> {
    let query = match recipient {
        UnifiedRecipient::LibP2p(peer_id) => {
            PeerQuery::LibP2p(*peer_id)
        }
        UnifiedRecipient::Reticulum(addr_hash) => {
            PeerQuery::Reticulum(*addr_hash)
        }
        UnifiedRecipient::Identity(identity_id) => {
            PeerQuery::Identity(identity_id.clone())
        }
    };

    let router = self.transport_router.read().await;
    router.send(&query, message, MessagePriority::Normal).await
        .map_err(|e| format!("Transport error: {:?}", e))
}
```

#### 3.4.5 Multi-Transport Send (Dual Routing)

For critical messages, both transports are used simultaneously:

```rust
async fn send_dual(
    mesh: &MeshManager,
    rns: &ReticulumManager,
    peer_id: PeerId,
    rns_addr: [u8; 16],
    message: Vec<u8>,
) -> Result<(), TransportError> {
    // Send via both transports concurrently
    let libp2p_result = mesh.send_direct(peer_id, message.clone());
    let rns_result = rns.send_packet(rns_addr, message);

    // Wait for both to complete
    let (libp2p_outcome, rns_outcome) = tokio::join!(libp2p_result, rns_result);

    // Return success if at least one succeeded
    match (libp2p_outcome, rns_outcome) {
        (Ok(_), _) | (_, Ok(_)) => Ok(()),
        (Err(e1), Err(e2)) => Err(TransportError::DualFailed {
            libp2p: e1,
            rns: e2,
        }),
    }
}
```

### 3.5 Reticulum Announce Integration

#### 3.5.1 Current Gap

The current `reticulum.rs` sends one announce at startup (L124) and never again. This means:

- After startup, the node becomes invisible to new RNS peers
- If the node's RNS path expires (7 days for full mode), it becomes unreachable via RNS
- There is no mechanism for other GhostWire nodes to discover this node via RNS

#### 3.5.2 Periodic Announce

**Fix**: Periodic announce every 300 seconds (configurable via `rns_announce_interval`).

```rust
impl ReticulumManager {
    /// Start the periodic announce loop
    async fn start_announce_loop(
        self: Arc<Self>,
        interval: std::time::Duration,
    ) {
        let mut timer = tokio::time::interval(interval);
        loop {
            timer.tick().await;
            self.send_ghostwire_announce().await;
        }
    }

    /// Send a GhostWire-specific RNS announce
    async fn send_ghostwire_announce(&self) {
        let announce = GhostWireAnnouncePayload {
            version: 1,
            identity_id: self.identity_id.clone(),
            libp2p_peer_id: self.libp2p_peer_id,
            multiaddrs: self.get_multiaddrs(),
            capabilities: vec![
                "sphinx".to_string(),
                "dtn".to_string(),
                "rns".to_string(),
            ],
            public_key: self.get_public_key(),
        };

        let payload = serde_json::to_vec(&announce)
            .expect("announce serialization should not fail");

        // Create RNS destination with GhostWire metadata
        let dest_name = format!(
            "ghostwire:{}",
            hex::encode(&self.identity_id.0[..16])
        );

        self.rns_interface.announce_with_extra(
            &dest_name,
            &payload,
        );
    }
}
```

#### 3.5.3 Extended Announce Payload

The RNS announce payload includes GhostWire-specific metadata for cross-transport discovery:

```
RNS Destination Name: "ghostwire:<identity_hex>"
RNS Extra Data (JSON):
{
    "version": 1,
    "identity_id": "<hex encoded 32-byte BLAKE3 hash>",
    "libp2p_peer_id": "<base58 encoded PeerId>",
    "multiaddrs": [
        "/ip4/192.168.1.100/tcp/44444/quic-v1",
        "/ip4/10.0.0.5/tcp/44444/quic-v1"
    ],
    "capabilities": ["sphinx", "dtn", "rns"],
    "public_key": "<hex encoded ed25519 public key>",
    "software_version": "0.1.0",
    "timestamp": 1691234567
}
```

#### 3.5.4 Receiving Announces

When an RNS announce is received, extract GhostWire metadata and cross-register:

```rust
impl ReticulumManager {
    /// Handle an incoming RNS announce
    async fn handle_announce(
        &self,
        address_hash: [u8; 16],
        announce_data: &[u8],
        peer_table: &Arc<RwLock<UnifiedPeerTable>>,
    ) {
        // Try to parse as GhostWire announce
        let metadata: GhostWireAnnouncePayload = match serde_json::from_slice(announce_data) {
            Ok(m) => m,
            Err(_) => return,  // not a GhostWire announce, ignore
        };

        // Register in unified peer table
        {
            let mut table = peer_table.write().await;
            table.upsert(
                metadata.identity_id.clone(),
                TransportKind::Reticulum,
                TransportAddr::Reticulum(address_hash),
            );

            // Set capabilities and version
            if let Some(peer) = table.lookup(&PeerQuery::Identity(metadata.identity_id.clone())) {
                // peer.capabilities = metadata.capabilities;
                // peer.version = Some(metadata.software_version);
            }
        }

        // Cross-register with Kademlia if multiaddrs known
        if let Some(peer_id) = metadata.libp2p_peer_id {
            if let Some(multiaddrs) = metadata.multiaddrs {
                for addr in multiaddrs {
                    self.kademlia.add_address(&peer_id, addr);
                }
            }
        }

        // Record for Thompson Sampling
        {
            let mut table = peer_table.write().await;
            table.record_outcome(
                &metadata.identity_id,
                TransportKind::Reticulum,
                true,
                None,
            );
        }
    }
}
```

#### 3.5.5 RNS Routing Table

The `ReticulumManager` should maintain a routing table of known RNS peers:

```rust
/// RNS routing table entry
struct RnsRouteEntry {
    /// RNS address hash
    address_hash: [u8; 16],

    /// GhostWire identity (if known)
    identity_id: Option<IdentityId>,

    /// Last time this route was verified
    last_seen: Instant,

    /// Hop count (0 = direct neighbor)
    hops: u8,

    /// Link quality (0.0-1.0)
    quality: f64,
}

/// RNS routing table
struct RnsRoutingTable {
    routes: HashMap<[u8; 16], RnsRouteEntry>,
    max_routes: usize,
}

impl RnsRoutingTable {
    /// Add or update a route
    fn upsert(&mut self, entry: RnsRouteEntry);

    /// Look up a route
    fn lookup(&self, address_hash: &[u8; 16]) -> Option<&RnsRouteEntry>;

    /// Remove stale routes
    fn gc_stale(&mut self, max_age: std::time::Duration);

    /// Get all routes
    fn all_routes(&self) -> Vec<&RnsRouteEntry>;
}
```

#### 3.5.6 send_broadcast Bug Fix

The current `send_broadcast` in `reticulum.rs` uses `DestinationType::Single` with an empty address, which is incorrect for broadcast behavior:

```rust
// BUGGY: core/reticulum.rs ~L280
// This creates a single-destination packet with an empty address
// instead of a true broadcast
```

**Fix**: Use `DestinationType::Broadcast`:

```rust
// FIXED
fn send_broadcast(&self, data: &[u8]) -> Result<(), RnsError> {
    // Create a broadcast destination (no specific address)
    let dest = RnsDestination::new(
        None,  // no specific address = broadcast
        DestinationType::Broadcast,
    );

    // Send packet
    let packet = RnsPacket::new(dest, data);
    self.rns_interface.send_packet(packet)?;

    Ok(())
}
```

### 3.6 Unified Addressing

Three ID namespaces must collapse into one. The `IdentityId` type becomes the canonical identifier, with deterministic derivation to all transport-specific types.

#### 3.6.1 Current State

| Type | Location | Format | Use |
|------|----------|--------|-----|
| UUID | `mesh.rs`, social graph | `uuid::Uuid` (128-bit) | Internal peer identification |
| PeerId | `mesh.rs`, libp2p | `libp2p::PeerId` (256-bit) | libp2p transport |
| AddressHash | `reticulum.rs` | `[u8; 16]` (128-bit) | RNS transport |

These three types are not interconvertible. A peer discovered via mDNS (PeerId) cannot be reached via RNS (AddressHash) without manual mapping.

#### 3.6.2 Unified IdentityId

```rust
/// The canonical identity -- deterministic from master_seed
#[derive(Hash, Eq, PartialEq, Clone, Debug, Serialize, Deserialize)]
struct IdentityId([u8; 32]);

impl IdentityId {
    /// Derive from master_seed via BLAKE3
    fn from_master_seed(seed: &[u8]) -> Self {
        let hash = blake3::hash(seed);
        let mut bytes = [0u8; 32];
        bytes.copy_from_slice(hash.as_bytes());
        IdentityId(bytes)
    }

    /// Derive libp2p PeerId from this identity
    /// Uses ed25519 keypair derived from master_seed
    fn to_libp2p_peer_id(&self, master_seed: &[u8]) -> PeerId {
        let transport_key = derive_transport_key(master_seed, "libp2p");
        let keypair = libp2p::identity::Keypair::ed25519_from_bytes(transport_key)
            .expect("valid ed25519 key");
        PeerId::from_public_key(&keypair.public())
    }

    /// Derive Reticulum AddressHash from this identity
    fn to_reticulum_address(&self, master_seed: &[u8]) -> [u8; 16] {
        let transport_key = derive_transport_key(master_seed, "reticulum");
        // RNS AddressHash is first 16 bytes of the transport key
        let mut hash = [0u8; 16];
        hash.copy_from_slice(&transport_key[..16]);
        hash
    }

    /// Derive social graph node_id
    fn to_node_id(&self) -> String {
        hex::encode(&self.0[..16])
    }

    /// Derive DID
    fn to_did(&self) -> String {
        format!("did:ghost:{}", hex::encode(&self.0[..16]))
    }

    /// Get the raw bytes
    fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// Create from bytes
    fn from_bytes(bytes: [u8; 32]) -> Self {
        IdentityId(bytes)
    }
}
```

#### 3.6.3 Cross-Transport Verification

Given a known `master_seed`, a node can verify that a peer's claimed PeerId or AddressHash matches their identity:

```rust
impl IdentityId {
    /// Verify that a claimed PeerId matches this identity
    fn verify_peer_id(
        &self,
        master_seed: &[u8],
        claimed_peer_id: &PeerId,
    ) -> bool {
        let expected = self.to_libp2p_peer_id(master_seed);
        &expected == claimed_peer_id
    }

    /// Verify that a claimed RNS AddressHash matches this identity
    fn verify_reticulum_address(
        &self,
        master_seed: &[u8],
        claimed_address: &[u8; 16],
    ) -> bool {
        let expected = self.to_reticulum_address(master_seed);
        &expected == claimed_address
    }
}
```

#### 3.6.4 Social Graph Integration

The social graph currently uses `register_discovered()` but `verify()` is a stub. With unified addressing:

```rust
impl SocialGraph {
    /// Verify that a DID resolves to the expected identity
    fn verify(&self, did: &str, expected_identity: &IdentityId) -> bool {
        // Parse DID
        let parsed = DidParser::parse(did);
        if parsed.method != "ghost" {
            return false;
        }

        // Extract identity from DID
        let did_identity = match hex::decode(&parsed.id) {
            Ok(bytes) if bytes.len() == 16 => {
                // Expand 16-byte ID to 32-byte IdentityId
                // (first 16 bytes match, remaining are derived)
                let mut full = [0u8; 32];
                full[..16].copy_from_slice(&bytes);
                IdentityId(full)
            }
            _ => return false,
        };

        // Compare with expected
        &did_identity == expected_identity
    }
}
```

#### 3.6.5 Migration Strategy

Existing peers have UUIDs, PeerIds, and AddressHashes that are already in use. Migration:

1. **New peers**: Use `IdentityId` from day one. Derive all transport IDs from `master_seed`.

2. **Existing peers**: On first connection with the new protocol, exchange `master_seed` hash (BLAKE3) as the `IdentityId`. Store the mapping `UUID -> IdentityId` and `PeerId -> IdentityId` in the peer table.

3. **Backward compatibility**: The `UnifiedPeerTable` accepts all three identifier types. Old code that uses PeerId directly still works; it just gets wrapped in a `PeerQuery::LibP2p()`.

### 3.7 Disaster Response Integration

#### 3.7.1 Thompson Sampling on Both Transports

The `transport_selector.rs` Thompson Sampling algorithm trains on outcomes from both transports:

```rust
impl TransportSelector {
    /// Record a transmission outcome for a specific transport
    pub fn record_outcome(
        &mut self,
        transport: TransportKind,
        success: bool,
        rtt_ms: Option<u64>,
    ) {
        if let Some(profile) = self.weight_profiles.get_mut(&transport) {
            if success {
                profile.success_count += 1;
                if let Some(rtt) = rtt_ms {
                    // Update rolling average RTT
                    profile.avg_rtt = match profile.avg_rtt {
                        Some(prev) => Some((prev + rtt as f64) / 2.0),
                        None => Some(rtt as f64),
                    };
                }
            } else {
                profile.failure_count += 1;
            }

            // Update Beta distribution parameters
            profile.alpha = profile.success_count as f64 + 1.0;
            profile.beta = profile.failure_count as f64 + 1.0;
        }
    }

    /// Sample from the Beta distribution for a transport
    pub fn sample(&self, transport: &TransportKind) -> f64 {
        if let Some(profile) = self.weight_profiles.get(transport) {
            // Sample from Beta(alpha, beta)
            let alpha = profile.base_success_rate * 100.0 + profile.success_count as f64;
            let beta = (1.0 - profile.base_success_rate) * 100.0 + profile.failure_count as f64;
            sample_beta(alpha, beta)
        } else {
            0.0
        }
    }
}
```

#### 3.7.2 Reticulum Weight Profile

```rust
fn reticulum() -> WeightProfile {
    WeightProfile {
        // RNS strengths: works off-grid, store-and-forward, radio-compatible
        // RNS weaknesses: lower bandwidth, higher latency
        base_success_rate: 0.7,   // slightly lower than libp2p
        latency_factor: 2.0,      // RNS is slower (store-and-forward)
        loss_factor: 0.1,         // RNS is reliable once connected
        recovery_bonus: 0.15,     // RNS recovers well from partitions
        // Beta distribution parameters (start neutral)
        alpha: 8.0,   // base_success_rate * 10 + 1
        beta: 4.0,    // (1 - base_success_rate) * 10 + 1
        success_count: 0,
        failure_count: 0,
        avg_rtt: None,
    }
}
```

#### 3.7.3 Updated DisasterPolicy Priority Lists

```rust
impl Default for DisasterPolicy {
    fn default() -> Self {
        Self {
            priority_lists: HashMap::from([
                // Normal: all transports available, prefer fast ones
                (DisasterLevel::Normal, vec![
                    TransportType::Quic,
                    TransportType::StealthTcp,
                    TransportType::Mesh,
                    TransportType::Reticulum,  // NEW
                    TransportType::WiFi,
                ]),
                // Congested: prefer mesh + RNS (both work without central infra)
                (DisasterLevel::Congested, vec![
                    TransportType::Mesh,
                    TransportType::Reticulum,  // NEW
                    TransportType::WiFi,
                    TransportType::Bluetooth,
                ]),
                // Isolation: RNS is king (works off-grid)
                (DisasterLevel::Isolation, vec![
                    TransportType::Reticulum,  // NEW - promoted to first
                    TransportType::WiFi,
                    TransportType::Bluetooth,
                    TransportType::Mesh,
                ]),
                // Blackout: RNS via radio + Bluetooth mesh
                (DisasterLevel::Blackout, vec![
                    TransportType::Reticulum,  // NEW - primary transport
                    TransportType::Bluetooth,
                    TransportType::LoRa,
                ]),
            ]),
        }
    }
}
```

#### 3.7.4 Disaster Level Transitions

The disaster level is determined by the `DisasterDetector` based on transport health metrics:

```rust
impl DisasterDetector {
    /// Determine current disaster level based on transport health
    fn detect_level(
        &self,
        transport_health: &HashMap<TransportKind, TransportHealth>,
    ) -> DisasterLevel {
        let libp2p_healthy = transport_health.get(&TransportKind::Quic)
            .map(|h| h.success_rate > 0.5)
            .unwrap_or(false);

        let rns_healthy = transport_health.get(&TransportKind::Reticulum)
            .map(|h| h.success_rate > 0.5)
            .unwrap_or(false);

        match (libp2p_healthy, rns_healthy) {
            (true, true) => DisasterLevel::Normal,
            (false, true) => DisasterLevel::Congested,  // libp2p down, RNS up
            (true, false) => DisasterLevel::Congested,  // RNS down, libp2p up
            (false, false) => {
                // Both degraded -- check if any transport works at all
                let any_healthy = transport_health.values()
                    .any(|h| h.success_rate > 0.3);
                if any_healthy {
                    DisasterLevel::Isolation
                } else {
                    DisasterLevel::Blackout
                }
            }
        }
    }
}
```

#### 3.7.5 Disaster Response Behavior

| Disaster Level | libp2p State | RNS State | Behavior |
|---------------|-------------|-----------|----------|
| Normal | Healthy | Healthy | Use Thompson Sampling to pick best transport |
| Congested | Degraded | Healthy | Prefer RNS + mesh, deprioritize internet transports |
| Isolation | Failed | Healthy | RNS only, use gateway mode to bridge segments |
| Blackout | Failed | Degraded | RNS via radio + Bluetooth mesh, minimal routing |

---

## 4. File-by-File Change Specification

This section specifies the exact changes required for each file, organized by implementation phase.

### Phase 1: Foundation (no behavioral changes, just plumbing)

#### 4.1.1 NEW: `ghostwire/src/core/peer_table.rs`

**Purpose**: Central data structure for unified peer management.

**Contents**:
- `IdentityId` struct (32-byte canonical identity)
- `UnifiedPeer` struct (multi-transport peer representation)
- `TransportAddress` struct (per-transport quality metrics)
- `TransportAddr` enum (libp2p Multiaddr or RNS AddressHash)
- `UnifiedPeerTable` struct (primary + reverse indexes)
- `PeerQuery` enum (flexible lookup by any ID type)
- `PeerTableSummary` struct (monitoring metrics)

**Dependencies**: None (pure data structures)

**Lines**: ~400

#### 4.1.2 MODIFY: `ghostwire/src/core/identity/core_identity.rs`

**Current state**: `derive_transport_key()` at L763-771 already derives per-transport keys from `master_seed`.

**Additions**:

```rust
// Add to IdentityId impl block
impl IdentityId {
    /// Derive libp2p PeerId from master_seed
    pub fn to_libp2p_peer_id(&self, master_seed: &[u8]) -> PeerId;

    /// Derive RNS AddressHash from master_seed
    pub fn to_reticulum_address(&self, master_seed: &[u8]) -> [u8; 16];

    /// Derive social graph node_id (hex first 16 bytes)
    pub fn to_node_id(&self) -> String;

    /// Derive DID format
    pub fn to_did(&self) -> String;
}
```

**Dependencies**: `peer_table.rs` for `IdentityId` definition

**Lines**: ~80 (new methods)

#### 4.1.3 MODIFY: `ghostwire/src/core/reticulum.rs`

**Current state**: 431 lines. ReticulumTransport + ReticulumManager.

**Changes**:

1. **Fix send_broadcast bug** (~L280): Change `DestinationType::Single` with empty address to `DestinationType::Broadcast`

2. **Add periodic announce**: New `start_announce_loop()` method that sends `send_ghostwire_announce()` every 300s

3. **Add RNS routing table**: New `RnsRoutingTable` struct for tracking known RNS peers

4. **Add announce payload parsing**: New `parse_ghostwire_announce()` function to extract GhostWire metadata from RNS announces

5. **Add Thompson Sampling integration**: `record_outcome()` calls for RNS transport

**Dependencies**: `peer_table.rs` for `UnifiedPeerTable` integration

**Lines**: ~200 (new code + fixes)

#### 4.1.4 MODIFY: `ghostwire/src/core/mod.rs`

**Current state**: 2528 lines.

**Changes**:

1. **Remove L2494-2527 stub types**: Delete `ReticulumStats` and `ReticulumTopology` stubs. Import feature-gated types from `reticulum.rs`.

2. **Add UnifiedPeerTable to Core struct**: New field `peer_table: Arc<RwLock<UnifiedPeerTable>>`

3. **Update imports**: Add `peer_table` module import

**Dependencies**: `peer_table.rs`

**Lines**: ~50 (modifications)

### Phase 2: Discovery Integration

#### 4.2.1 MODIFY: `ghostwire/src/core/mesh.rs`

**Current state**: 2660 lines.

**Changes**:

1. **Add RNS receive arm in tokio::select!** (L861-931):
   ```rust
   // Add new arm to the select loop
   rns_event = rns_receiver.recv() => {
       self.handle_rns_event(rns_event).await;
   }
   ```

2. **Add cross-registration in discovery handler** (L357-365):
   ```rust
   // When mDNS or Kademlia discovers a peer
   if let Some(rns_addr) = handshake.reticulum_address {
       self.reticulum_manager.register_known_peer(identity, rns_addr);
   }
   ```

3. **Update peer count** (L1463-1472):
   ```rust
   // Unified peer count
   fn peer_count(&self) -> usize {
       self.peer_table.read().unwrap().peer_count()
   }
   ```

4. **Add gossipsub announce topic**:
   ```rust
   const ANNOUNCE_TOPIC: &str = "ghostwire/announce";
   // Subscribe in mesh setup
   // Publish announce every 60s
   ```

**Dependencies**: `peer_table.rs`, reticulum changes

**Lines**: ~150 (modifications)

#### 4.2.2 MODIFY: `ghostwire/src/core/reticulum.rs` (continued)

**Additional changes for Phase 2**:

1. **Listen for announces**: New handler in the RNS event loop that processes incoming announces

2. **Extract PeerId from announces**: Parse GhostWire metadata, extract libp2p PeerId

3. **Register in UnifiedPeerTable**: Call `upsert()` with discovered peer info

**Dependencies**: `peer_table.rs`, mesh.rs changes

**Lines**: ~100 (new handler code)

#### 4.2.3 MODIFY: `ghostwire/src/core/mod.rs` (continued)

**Additional changes for Phase 2**:

1. **Wire UnifiedPeerTable into both managers**: Pass `Arc<RwLock<UnifiedPeerTable>>` to both `MeshManager` and `ReticulumManager`

2. **Update Core::new()**: Initialize `UnifiedPeerTable` with config values

**Dependencies**: peer_table.rs, mesh.rs, reticulum.rs changes

**Lines**: ~30 (wiring)

### Phase 3: Routing & Dispatch

#### 4.3.1 MODIFY: `ghostwire/src/core/mod.rs` (continued)

**Changes**:

1. **Replace active_transport with TransportRouter**:
   ```rust
   // REMOVE
   active_transport: Arc<RwLock<Option<TransportKind>>>,

   // ADD
   transport_router: Arc<RwLock<TransportRouter>>,
   ```

2. **Refactor send_direct_message** (L1842):
   ```rust
   pub async fn send_direct_message(
       &self,
       recipient: &UnifiedRecipient,
       message: Vec<u8>,
   ) -> Result<(), String> {
       let router = self.transport_router.read().await;
       let query = recipient.to_query();
       router.send(&query, message, MessagePriority::Normal).await
           .map_err(|e| format!("Transport error: {:?}", e))
   }
   ```

**Dependencies**: transport_router.rs, peer_table.rs

**Lines**: ~100 (refactoring)

#### 4.3.2 MODIFY: `ghostwire/src/core/transport.rs`

**Current state**: 261 lines.

**Changes**:

1. **Add TransportRouter**:
   ```rust
   struct TransportRouter {
       peer_table: Arc<RwLock<UnifiedPeerTable>>,
       selector: Arc<RwLock<TransportSelector>>,
       mesh_manager: Option<Arc<MeshManager>>,
       reticulum_manager: Option<Arc<ReticulumManager>>,
       disaster_level: Arc<RwLock<DisasterLevel>>,
   }

   impl TransportRouter {
       fn route(&self, recipient: &PeerQuery, priority: MessagePriority) -> RoutingDecision;
       async fn send(&self, recipient: &PeerQuery, message: Vec<u8>, priority: MessagePriority) -> Result<(), TransportError>;
       fn record_outcome(&self, identity: &IdentityId, transport: TransportKind, success: bool, rtt_ms: Option<u64>);
   }
   ```

2. **Add RoutingDecision enum**:
   ```rust
   enum RoutingDecision {
       LibP2p { peer_id: PeerId, addr: Multiaddr },
       Reticulum { address_hash: [u8; 16] },
       Dual { libp2p: Option<PeerId>, rns: Option<[u8; 16]> },
       Unreachable,
   }
   ```

3. **Add MessagePriority enum**:
   ```rust
   enum MessagePriority {
       Normal,
       Important,
       Critical,
   }
   ```

**Dependencies**: peer_table.rs, transport_selector.rs

**Lines**: ~250 (new code)

#### 4.3.3 MODIFY: `ghostwire/src/core/mesh.rs` (continued)

**Additional changes for Phase 3**:

1. **Add gossipsub announce publishing**: Periodic publish of `GhostWireAnnounce` on `ghostwire/announce` topic

2. **Add announce receiver**: Handle incoming announces, cross-register with RNS

**Dependencies**: peer_table.rs

**Lines**: ~80 (new publisher/receiver)

### Phase 4: Disaster & Selection

#### 4.4.1 MODIFY: `ghostwire/src/core/transport_selector.rs`

**Current state**: 876 lines.

**Changes**:

1. **Add reticulum() weight profile**:
   ```rust
   fn reticulum() -> WeightProfile {
       WeightProfile {
           base_success_rate: 0.7,
           latency_factor: 2.0,
           loss_factor: 0.1,
           recovery_bonus: 0.15,
           alpha: 8.0,
           beta: 4.0,
           success_count: 0,
           failure_count: 0,
           avg_rtt: None,
       }
   }
   ```

2. **Add to default_weight_profiles()** (L59-134):
   ```rust
   profiles.insert(TransportKind::Reticulum, reticulum());
   ```

**Dependencies**: None

**Lines**: ~30 (additions)

#### 4.4.2 MODIFY: `ghostwire/src/core/disaster.rs`

**Current state**: 489 lines.

**Changes**:

1. **Add Reticulum to all priority lists** (L201-210):
   ```rust
   (DisasterLevel::Normal, vec![
       TransportType::Quic, TransportType::StealthTcp, TransportType::Mesh,
       TransportType::Reticulum, TransportType::WiFi
   ]),
   (DisasterLevel::Congested, vec![
       TransportType::Mesh, TransportType::Reticulum, TransportType::WiFi,
       TransportType::Bluetooth
   ]),
   (DisasterLevel::Isolation, vec![
       TransportType::Reticulum, TransportType::WiFi, TransportType::Bluetooth,
       TransportType::Mesh
   ]),
   (DisasterLevel::Blackout, vec![
       TransportType::Reticulum, TransportType::Bluetooth, TransportType::LoRa
   ]),
   ```

**Dependencies**: None

**Lines**: ~20 (modifications)

#### 4.4.3 MODIFY: `ghostwire/src/core/transport_manager.rs`

**Current state**: 102 lines.

**Changes**:

1. **Add Reticulum to available_transports()** (L61-84):
   ```rust
   pub fn available_transports(&self, level: DisasterLevel) -> Vec<TransportKind> {
       match level {
           DisasterLevel::Normal => vec![
               TransportKind::Quic, TransportKind::StealthTcp, TransportKind::Mesh,
               TransportKind::Reticulum, TransportKind::WiFi
           ],
           DisasterLevel::Congested => vec![
               TransportKind::Mesh, TransportKind::Reticulum, TransportKind::WiFi,
               TransportKind::Bluetooth
           ],
           DisasterLevel::Isolation => vec![
               TransportKind::Reticulum, TransportKind::WiFi, TransportKind::Bluetooth,
               TransportKind::Mesh
           ],
           DisasterLevel::Blackout => vec![TransportKind::Reticulum, TransportKind::Bluetooth],
       }
   }
   ```

**Dependencies**: None

**Lines**: ~15 (modifications)

### Phase 5: Config & CLI

#### 4.5.1 MODIFY: `ghostwire/src/config.rs`

**Current state**: 327 lines.

**Changes**:

1. **Add hybrid mode fields**:
   ```rust
   struct NetworkConfig {
       // ... existing fields ...

       /// Hybrid mode: "hybrid", "libp2p-only", "reticulum-only"
       hybrid_mode: String,  // default: "hybrid"

       /// Discovery interval in seconds (gossipsub announce)
       discovery_interval: u64,  // default: 60

       /// RNS announce interval in seconds
       rns_announce_interval: u64,  // default: 300

       /// Maximum peer table size
       max_peers: usize,  // default: 1000

       /// Maximum stale duration in seconds
       max_stale_duration: u64,  // default: 3600
   }
   ```

2. **Add FeatureFlags**:
   ```rust
   struct FeatureFlags {
       // ... existing flags ...

       /// Enable cross-pollinate discovery
       cross_pollinate: bool,  // default: true

       /// Enable dual routing for critical messages
       dual_routing: bool,  // default: true

       /// Enable RNS announce listener
       rns_announce_listener: bool,  // default: true
   }
   ```

**Dependencies**: None

**Lines**: ~40 (additions)

#### 4.5.2 MODIFY: `ghostwire/src/main.rs` and `ghostwire/src/cli/mod.rs`

**Changes**:

1. **Add transport mode flags**:
   ```rust
   /// Transport mode selection
   #[arg(long, default_value = "hybrid")]
   mode: String,  // "hybrid", "libp2p-only", "reticulum-only"

   /// Enable cross-pollinate discovery
   #[arg(long)]
   cross_pollinate: bool,

   /// Discovery interval in seconds
   #[arg(long, default_value = "60")]
   discovery_interval: u64,

   /// RNS announce interval in seconds
   #[arg(long, default_value = "300")]
   rns_announce_interval: u64,
   ```

2. **Pass flags to config**: Map CLI args to `NetworkConfig` fields

**Dependencies**: config.rs changes

**Lines**: ~30 (additions)

#### 4.5.3 MODIFY: `ghostwire/src/web.rs`

**Changes**:

1. **Replace `Vec<libp2p::Multiaddr>` with transport-agnostic type**:
   ```rust
   struct BootstrapNode {
       identity: IdentityId,
       addresses: Vec<TransportAddress>,
   }

   struct StartupConfig {
       // ... existing fields ...
       bootstrap_nodes: Vec<BootstrapNode>,  // was: Vec<libp2p::Multiaddr>
   }
   ```

2. **Update API endpoints**: Use `TransportAddress` instead of `Multiaddr`

**Dependencies**: peer_table.rs

**Lines**: ~40 (modifications)

### Phase Summary

| Phase | Files Changed | New Files | Total Lines |
|-------|--------------|-----------|-------------|
| Phase 1: Foundation | 3 (core_identity.rs, reticulum.rs, mod.rs) | 1 (peer_table.rs) | ~730 |
| Phase 2: Discovery | 3 (mesh.rs, reticulum.rs, mod.rs) | 0 | ~280 |
| Phase 3: Routing | 3 (mod.rs, transport.rs, mesh.rs) | 0 | ~430 |
| Phase 4: Disaster | 3 (transport_selector.rs, disaster.rs, transport_manager.rs) | 0 | ~65 |
| Phase 5: Config & CLI | 4 (config.rs, main.rs, cli/mod.rs, web.rs) | 0 | ~140 |
| **Total** | **16 files** | **1 file** | **~1645 lines** |

---

## 5. Phased Implementation Plan

### 5.1 Phase Overview

| Phase | Duration | Deliverable | Risk | Dependencies |
|-------|----------|-------------|------|-------------|
| Phase 1: Foundation | 2-3 days | peer_table.rs, identity methods, reticulum bug fixes | Low -- pure additions | None |
| Phase 2: Discovery | 3-4 days | Cross-pollinate discovery, gossipsub announce topic | Medium -- must test with 3+ nodes | Phase 1 |
| Phase 3: Routing | 3-4 days | TransportRouter, send_direct_message refactor | HIGH -- touches critical send path | Phase 1, 2 |
| Phase 4: Disaster | 1-2 days | Thompson Sampling training on RNS, updated priority lists | Low -- mostly config | Phase 1 |
| Phase 5: Config & CLI | 1 day | New flags, config fields | Low -- additive | Phase 1 |

**Total estimated duration**: 10-14 days

### 5.2 Phase 1: Foundation (2-3 days)

**Goal**: Establish the Unified Peer Table and identity derivation methods. Fix critical bugs.

**Deliverables**:

1. **`core/peer_table.rs`** (new file, ~400 lines)
   - `IdentityId` struct with `from_master_seed()`, `to_libp2p_peer_id()`, `to_reticulum_address()`, `to_node_id()`, `to_did()`
   - `UnifiedPeer` struct with multi-transport support
   - `TransportAddress` struct with quality metrics
   - `UnifiedPeerTable` with primary + reverse indexes
   - `PeerQuery` enum for flexible lookup
   - Unit tests for all operations

2. **`core/identity/core_identity.rs`** (modification, ~80 lines)
   - Add `to_libp2p_peer_id()` method
   - Add `to_reticulum_address()` method
   - Add `to_node_id()` method
   - Add `to_did()` method
   - Unit tests for derivation

3. **`core/reticulum.rs`** (modification, ~200 lines)
   - Fix `send_broadcast` bug (DestinationType::Single -> Broadcast)
   - Add `RnsRoutingTable` struct
   - Add `start_announce_loop()` method
   - Add `send_ghostwire_announce()` method
   - Add `parse_ghostwire_announce()` function
   - Unit tests

4. **`core/mod.rs`** (modification, ~50 lines)
   - Remove L2494-2527 stub types
   - Add `peer_table: Arc<RwLock<UnifiedPeerTable>>` to Core struct
   - Update imports

**Verification criteria**:
- `cargo check` passes
- `cargo test` passes (all existing tests + new unit tests)
- `UnifiedPeerTable` can add, remove, lookup by all three key types
- `IdentityId` can derive PeerId and AddressHash from master_seed
- RNS broadcast bug is fixed
- Periodic announce compiles (may not be testable without RNS network)

**Risk mitigation**:
- All changes are additive (new types, new methods)
- Existing code paths are not modified (except stub removal)
- Unit tests verify data structure correctness

### 5.3 Phase 2: Discovery Integration (3-4 days)

**Goal**: Enable cross-pollinate discovery. When one transport discovers a peer, the other learns about it.

**Deliverables**:

1. **`core/mesh.rs`** (modification, ~150 lines)
   - Add RNS receive arm in `tokio::select!` (L861-931)
   - Add cross-registration in discovery handler (L357-365)
   - Update peer count to use UnifiedPeerTable (L1463-1472)
   - Add gossipsub announce topic subscription
   - Add `GhostWireAnnounce` publisher (60s interval)
   - Add announce receiver handler

2. **`core/reticulum.rs`** (modification, ~100 lines)
   - Add announce listener in RNS event loop
   - Add `handle_announce()` method for incoming GhostWire announces
   - Extract PeerId from announce metadata
   - Register discovered peers in UnifiedPeerTable

3. **`core/mod.rs`** (modification, ~30 lines)
   - Wire `UnifiedPeerTable` into both `MeshManager` and `ReticulumManager`
   - Update `Core::new()` to initialize with config values

**Verification criteria**:
- `cargo check` passes
- `cargo test` passes
- Integration test: 3 nodes on same network
  - Node A discovers Node B via mDNS
  - Node B's RNS address is automatically registered
  - Node C (RNS-only) announces, Node A learns about it via gossipsub
- Peer count reflects both transports

**Risk mitigation**:
- Test with 3+ nodes to verify cross-transport discovery
- Log all discovery events for debugging
- Add metrics for discovery success rate

### 5.4 Phase 3: Routing & Dispatch (3-4 days)

**Goal**: Replace single-transport routing with multi-transport dispatch via TransportRouter.

**Deliverables**:

1. **`core/transport.rs`** (modification, ~250 lines)
   - Add `TransportRouter` struct
   - Add `RoutingDecision` enum
   - Add `MessagePriority` enum
   - Implement `route()` logic (Thompson Sampling + disaster level)
   - Implement `send()` with transport-specific dispatch
   - Implement dual routing for critical messages
   - Unit tests for routing logic

2. **`core/mod.rs`** (modification, ~100 lines)
   - Replace `active_transport` with `transport_router: Arc<RwLock<TransportRouter>>`
   - Refactor `send_direct_message()` to use `TransportRouter`
   - Add `UnifiedRecipient` enum
   - Update all callers of `send_direct_message`

3. **`core/mesh.rs`** (modification, ~80 lines)
   - Add gossipsub announce publisher
   - Add announce receiver that cross-registers with RNS

**Verification criteria**:
- `cargo check` passes
- `cargo test` passes
- Integration test: peer reachable via both transports
  - Message routes via highest Thompson Sampling score
  - Dual routing works for critical messages
- Integration test: peer reachable only via RNS
  - Message routes via RNS
- Integration test: peer reachable only via libp2p
  - Message routes via libp2p
- `send_direct_message` accepts all three recipient types

**Risk mitigation**:
- This phase touches the critical send path
- Extensive testing with mock transports before real integration
- Feature flag to disable TransportRouter and fall back to active_transport
- Rollback plan: keep active_transport as fallback

### 5.5 Phase 4: Disaster & Selection (1-2 days)

**Goal**: Enable Thompson Sampling to train on RNS outcomes. Update disaster priority lists.

**Deliverables**:

1. **`core/transport_selector.rs`** (modification, ~30 lines)
   - Add `reticulum()` weight profile
   - Add to `default_weight_profiles()` HashMap

2. **`core/disaster.rs`** (modification, ~20 lines)
   - Add `TransportType::Reticulum` to all 4 priority lists

3. **`core/transport_manager.rs`** (modification, ~15 lines)
   - Add `TransportKind::Reticulum` to `available_transports()` at all levels

**Verification criteria**:
- `cargo check` passes
- `cargo test` passes
- Unit test: Thompson Sampling samples from Reticulum profile
- Unit test: DisasterPolicy includes Reticulum in all levels
- Unit test: TransportManager includes Reticulum in all levels
- Integration test: disaster scenario, Thompson Sampling selects RNS when libp2p fails

**Risk mitigation**:
- Mostly configuration changes
- Existing Thompson Sampling logic handles new transport automatically
- No behavioral changes to existing transports

### 5.6 Phase 5: Config & CLI (1 day)

**Goal**: Expose hybrid mode configuration via CLI flags and config file.

**Deliverables**:

1. **`config.rs`** (modification, ~40 lines)
   - Add `hybrid_mode`, `discovery_interval`, `rns_announce_interval`, `max_peers`, `max_stale_duration` fields
   - Add `cross_pollinate`, `dual_routing`, `rns_announce_listener` feature flags

2. **`main.rs`** and **`cli/mod.rs`** (modification, ~30 lines)
   - Add `--mode` flag (hybrid/libp2p-only/reticulum-only)
   - Add `--cross-pollinate` flag
   - Add `--discovery-interval` flag
   - Add `--rns-announce-interval` flag
   - Map CLI args to config fields

3. **`web.rs`** (modification, ~40 lines)
   - Replace `Vec<libp2p::Multiaddr>` with `Vec<BootstrapNode>`
   - Update API endpoints

**Verification criteria**:
- `cargo check` passes
- `cargo test` passes
- `--help` shows new flags
- Config file accepts new fields
- Default values work correctly

**Risk mitigation**:
- Purely additive changes
- No behavioral changes (just configuration exposure)
- Backward compatible (new fields have defaults)

---

## 6. Testing Strategy

### 6.1 Unit Tests

#### 6.1.1 UnifiedPeerTable Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_upsert_new_peer() {
        let mut table = UnifiedPeerTable::new(1000, Duration::from_secs(3600));
        let identity = IdentityId::from_master_seed(b"test seed");

        table.upsert(
            identity.clone(),
            TransportKind::Quic,
            TransportAddr::LibP2p("/ip4/127.0.0.1/tcp/44444".parse().unwrap()),
        );

        assert_eq!(table.peer_count(), 1);
        let peer = table.lookup(&PeerQuery::Identity(identity)).unwrap();
        assert!(peer.transports.contains_key(&TransportKind::Quic));
    }

    #[test]
    fn test_upsert_merge_transports() {
        let mut table = UnifiedPeerTable::new(1000, Duration::from_secs(3600));
        let identity = IdentityId::from_master_seed(b"test seed");

        // First: libp2p
        table.upsert(
            identity.clone(),
            TransportKind::Quic,
            TransportAddr::LibP2p("/ip4/127.0.0.1/tcp/44444".parse().unwrap()),
        );

        // Second: RNS (same identity)
        table.upsert(
            identity.clone(),
            TransportKind::Reticulum,
            TransportAddr::Reticulum([1u8; 16]),
        );

        assert_eq!(table.peer_count(), 1);  // still one peer
        let peer = table.lookup(&PeerQuery::Identity(identity)).unwrap();
        assert!(peer.transports.contains_key(&TransportKind::Quic));
        assert!(peer.transports.contains_key(&TransportKind::Reticulum));
    }

    #[test]
    fn test_lookup_by_peer_id() {
        let mut table = UnifiedPeerTable::new(1000, Duration::from_secs(3600));
        let identity = IdentityId::from_master_seed(b"test seed");
        let peer_id = identity.to_libp2p_peer_id(b"test seed");

        table.upsert(
            identity.clone(),
            TransportKind::Quic,
            TransportAddr::LibP2p("/ip4/127.0.0.1/tcp/44444".parse().unwrap()),
        );

        // Should find by PeerId
        let found = table.lookup(&PeerQuery::LibP2p(peer_id));
        assert!(found.is_some());
        assert_eq!(found.unwrap().identity_id, identity);
    }

    #[test]
    fn test_lookup_by_rns_address() {
        let mut table = UnifiedPeerTable::new(1000, Duration::from_secs(3600));
        let identity = IdentityId::from_master_seed(b"test seed");
        let rns_addr = identity.to_reticulum_address(b"test seed");

        table.upsert(
            identity.clone(),
            TransportKind::Reticulum,
            TransportAddr::Reticulum(rns_addr),
        );

        // Should find by RNS AddressHash
        let found = table.lookup(&PeerQuery::Reticulum(rns_addr));
        assert!(found.is_some());
        assert_eq!(found.unwrap().identity_id, identity);
    }

    #[test]
    fn test_gc_stale() {
        let mut table = UnifiedPeerTable::new(1000, Duration::from_secs(0));  // 0 = everything stale
        let identity = IdentityId::from_master_seed(b"test seed");

        table.upsert(
            identity.clone(),
            TransportKind::Quic,
            TransportAddr::LibP2p("/ip4/127.0.0.1/tcp/44444".parse().unwrap()),
        );

        // GC should remove the stale entry
        let removed = table.gc_stale();
        assert_eq!(removed.len(), 1);
        assert_eq!(table.peer_count(), 0);
    }

    #[test]
    fn test_max_peers_eviction() {
        let mut table = UnifiedPeerTable::new(2, Duration::from_secs(3600));

        // Add 3 peers (max is 2)
        for i in 0..3 {
            let seed = format!("seed {}", i);
            let identity = IdentityId::from_master_seed(seed.as_bytes());
            table.upsert(
                identity,
                TransportKind::Quic,
                TransportAddr::LibP2p("/ip4/127.0.0.1/tcp/44444".parse().unwrap()),
            );
        }

        // Should have evicted one
        assert!(table.peer_count() <= 2);
    }

    #[test]
    fn test_record_outcome() {
        let mut table = UnifiedPeerTable::new(1000, Duration::from_secs(3600));
        let identity = IdentityId::from_master_seed(b"test seed");

        table.upsert(
            identity.clone(),
            TransportKind::Quic,
            TransportAddr::LibP2p("/ip4/127.0.0.1/tcp/44444".parse().unwrap()),
        );

        // Record successes
        for _ in 0..10 {
            table.record_outcome(&identity, TransportKind::Quic, true, Some(50));
        }

        // Record failures
        for _ in 0..2 {
            table.record_outcome(&identity, TransportKind::Quic, false, None);
        }

        let peer = table.lookup(&PeerQuery::Identity(identity)).unwrap();
        let transport = peer.transports.get(&TransportKind::Quic).unwrap();
        assert_eq!(transport.success_count, 10);
        assert_eq!(transport.failure_count, 2);
    }
}
```

#### 6.1.2 IdentityId Derivation Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_deterministic_derivation() {
        let id1 = IdentityId::from_master_seed(b"secret seed");
        let id2 = IdentityId::from_master_seed(b"secret seed");
        assert_eq!(id1, id2);  // same seed = same identity
    }

    #[test]
    fn test_different_seeds_different_identities() {
        let id1 = IdentityId::from_master_seed(b"seed 1");
        let id2 = IdentityId::from_master_seed(b"seed 2");
        assert_ne!(id1, id2);
    }

    #[test]
    fn test_to_libp2p_peer_id() {
        let id = IdentityId::from_master_seed(b"test seed");
        let peer_id1 = id.to_libp2p_peer_id(b"test seed");
        let peer_id2 = id.to_libp2p_peer_id(b"test seed");
        assert_eq!(peer_id1, peer_id2);  // deterministic
    }

    #[test]
    fn test_to_reticulum_address() {
        let id = IdentityId::from_master_seed(b"test seed");
        let addr1 = id.to_reticulum_address(b"test seed");
        let addr2 = id.to_reticulum_address(b"test seed");
        assert_eq!(addr1, addr2);  // deterministic
    }

    #[test]
    fn test_to_did() {
        let id = IdentityId::from_master_seed(b"test seed");
        let did = id.to_did();
        assert!(did.starts_with("did:ghost:"));
    }

    #[test]
    fn test_cross_transport_verification() {
        let seed = b"test seed";
        let id = IdentityId::from_master_seed(seed);

        let peer_id = id.to_libp2p_peer_id(seed);
        assert!(id.verify_peer_id(seed, &peer_id));

        let rns_addr = id.to_reticulum_address(seed);
        assert!(id.verify_reticulum_address(seed, &rns_addr));
    }
}
```

#### 6.1.3 TransportRouter Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_route_to_libp2p_peer() {
        // Setup: peer with both transports
        let table = Arc::new(RwLock::new(setup_peer_table_with_dual_peer()));
        let selector = Arc::new(RwLock::new(setup_selector_with_profiles()));
        let router = TransportRouter::new(table, selector, None, None);

        let peer_id = PeerId::from_str("12D3KooW...").unwrap();
        let decision = router.route(&PeerQuery::LibP2p(peer_id), MessagePriority::Normal);

        assert!(matches!(decision, RoutingDecision::LibP2p { .. }));
    }

    #[test]
    fn test_route_to_rns_peer() {
        // Setup: peer with only RNS transport
        let table = Arc::new(RwLock::new(setup_peer_table_with_rns_only_peer()));
        let selector = Arc::new(RwLock::new(setup_selector_with_profiles()));
        let router = TransportRouter::new(table, selector, None, None);

        let rns_addr = [1u8; 16];
        let decision = router.route(&PeerQuery::Reticulum(rns_addr), MessagePriority::Normal);

        assert!(matches!(decision, RoutingDecision::Reticulum { .. }));
    }

    #[test]
    fn test_route_critical_uses_dual() {
        // Setup: peer with both transports
        let table = Arc::new(RwLock::new(setup_peer_table_with_dual_peer()));
        let selector = Arc::new(RwLock::new(setup_selector_with_profiles()));
        let router = TransportRouter::new(table, selector, None, None);

        let identity = IdentityId::from_master_seed(b"test");
        let decision = router.route(&PeerQuery::Identity(identity), MessagePriority::Critical);

        assert!(matches!(decision, RoutingDecision::Dual { .. }));
    }

    #[test]
    fn test_route_unknown_peer() {
        let table = Arc::new(RwLock::new(UnifiedPeerTable::new(1000, Duration::from_secs(3600))));
        let selector = Arc::new(RwLock::new(setup_selector_with_profiles()));
        let router = TransportRouter::new(table, selector, None, None);

        let identity = IdentityId::from_master_seed(b"unknown");
        let decision = router.route(&PeerQuery::Identity(identity), MessagePriority::Normal);

        assert!(matches!(decision, RoutingDecision::Unreachable));
    }
}
```

### 6.2 Integration Tests

#### 6.2.1 Cross-Pollinate Discovery Test

```rust
/// Test: 3 nodes, verify cross-transport discovery
///
/// Setup:
/// - Node A: libp2p + RNS (gateway mode)
/// - Node B: libp2p only (discovers Node C via RNS announce)
/// - Node C: RNS only (announces on RNS)
///
/// Expected:
/// 1. Node A discovers Node B via mDNS
/// 2. Node A discovers Node C via RNS announce
/// 3. Node B learns about Node C via gossipsub announce from Node A
/// 4. All three nodes have each other in their UnifiedPeerTable
#[tokio::test]
async fn test_cross_pollinate_discovery() {
    let node_a = create_node(NodeConfig {
        libp2p: true,
        rns: true,
        rns_mode: "gateway",
        ..Default::default()
    }).await;

    let node_b = create_node(NodeConfig {
        libp2p: true,
        rns: false,
        ..Default::default()
    }).await;

    let node_c = create_node(NodeConfig {
        libp2p: false,
        rns: true,
        rns_mode: "access_point",
        ..Default::default()
    }).await;

    // Wait for discovery
    tokio::time::sleep(Duration::from_secs(10)).await;

    // Verify Node A knows about both B and C
    let table_a = node_a.peer_table.read().await;
    assert!(table_a.peer_count() >= 2);

    // Verify Node B knows about Node C (via gossipsub announce from A)
    let table_b = node_b.peer_table.read().await;
    assert!(table_b.peer_count_for_transport(TransportKind::Reticulum) >= 1);

    // Verify Node C knows about Node A (via RNS announce)
    let table_c = node_c.peer_table.read().await;
    assert!(table_c.peer_count_for_transport(TransportKind::Reticulum) >= 1);
}
```

#### 6.2.2 RNS-Only Peer Routing Test

```rust
/// Test: peer reachable only via RNS
///
/// Setup:
/// - Node A: libp2p + RNS
/// - Node B: RNS only
///
/// Expected:
/// 1. Node A discovers Node B via RNS announce
/// 2. Node A can send message to Node B via RNS
/// 3. Thompson Sampling records RNS success
#[tokio::test]
async fn test_rns_only_routing() {
    let node_a = create_node(NodeConfig {
        libp2p: true,
        rns: true,
        ..Default::default()
    }).await;

    let node_b = create_node(NodeConfig {
        libp2p: false,
        rns: true,
        ..Default::default()
    }).await;

    // Wait for discovery
    tokio::time::sleep(Duration::from_secs(10)).await;

    // Send message from A to B
    let result = node_a.send_direct_message(
        &UnifiedRecipient::Identity(node_b.identity_id.clone()),
        b"hello".to_vec(),
    ).await;

    assert!(result.is_ok());

    // Verify Thompson Sampling recorded the outcome
    let selector = node_a.transport_selector.read().await;
    let profile = selector.get_profile(TransportKind::Reticulum);
    assert!(profile.success_count > 0);
}
```

#### 6.2.3 Disaster Scenario Test

```rust
/// Test: disaster scenario, Thompson Sampling selects RNS
///
/// Setup:
/// - Node A: libp2p + RNS
/// - Node B: libp2p + RNS
///
/// Steps:
/// 1. Initial: both transports healthy, Thompson Sampling picks libp2p (faster)
/// 2. Simulate: libp2p fails (network partition)
/// 3. Verify: disaster level changes to Congested
/// 4. Verify: Thompson Sampling shifts to RNS
/// 5. Verify: messages route via RNS
#[tokio::test]
async fn test_disaster_scenario_rns_selection() {
    let node_a = create_node(NodeConfig {
        libp2p: true,
        rns: true,
        ..Default::default()
    }).await;

    let node_b = create_node(NodeConfig {
        libp2p: true,
        rns: true,
        ..Default::default()
    }).await;

    // Wait for initial discovery
    tokio::time::sleep(Duration::from_secs(10)).await;

    // Verify initial: libp2p is preferred
    let decision = node_a.transport_router.route(
        &PeerQuery::Identity(node_b.identity_id.clone()),
        MessagePriority::Normal,
    );
    assert!(matches!(decision, RoutingDecision::LibP2p { .. }));

    // Simulate libp2p failure
    node_a.simulate_transport_failure(TransportKind::Quic).await;

    // Wait for disaster detection
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Verify disaster level changed
    let level = node_a.disaster_level.read().await;
    assert_eq!(*level, DisasterLevel::Congested);

    // Verify routing shifted to RNS
    let decision = node_a.transport_router.route(
        &PeerQuery::Identity(node_b.identity_id.clone()),
        MessagePriority::Normal,
    );
    assert!(matches!(decision, RoutingDecision::Reticulum { .. }));
}
```

#### 6.2.4 Rapid Peer Churn Test

```rust
/// Test: rapid peer churn, verify table consistency
///
/// Setup:
/// - 10 nodes joining and leaving rapidly
///
/// Expected:
/// 1. Peer table stays consistent (no duplicates, no stale entries)
/// 2. GC removes stale entries
/// 3. New peers are properly registered
#[tokio::test]
async fn test_rapid_peer_churn() {
    let hub = create_node(NodeConfig {
        libp2p: true,
        rns: true,
        max_peers: 100,
        ..Default::default()
    }).await;

    // Spawn 10 nodes that join and leave
    let mut handles = Vec::new();
    for i in 0..10 {
        let handle = tokio::spawn(async move {
            let node = create_node(NodeConfig {
                libp2p: true,
                rns: false,
                ..Default::default()
            }).await;

            // Stay for a random duration
            let duration = Duration::from_millis(500 + (i * 100) as u64);
            tokio::time::sleep(duration).await;
            // Node drops here (simulates leaving)
        });
        handles.push(handle);
    }

    // Wait for all nodes to leave
    for handle in handles {
        handle.await.unwrap();
    }

    // Wait for GC
    tokio::time::sleep(Duration::from_secs(5)).await;

    // Verify table is clean
    let table = hub.peer_table.read().await;
    assert_eq!(table.peer_count(), 0);  // all peers should be GC'd
}
```

### 6.3 Stress Tests

#### 6.3.1 High Concurrency Test

```rust
/// Test: high concurrency, verify no race conditions
///
/// - 100 concurrent upsert operations
/// - 100 concurrent lookup operations
/// - 100 concurrent GC operations
///
/// Expected: no panics, no deadlocks, consistent state
#[tokio::test]
async fn test_high_concurrency() {
    let table = Arc::new(RwLock::new(
        UnifiedPeerTable::new(10000, Duration::from_secs(3600))
    ));

    let mut handles = Vec::new();

    // Spawn 100 upsert tasks
    for i in 0..100 {
        let table = table.clone();
        handles.push(tokio::spawn(async move {
            let seed = format!("seed {}", i);
            let identity = IdentityId::from_master_seed(seed.as_bytes());
            let mut table = table.write().await;
            table.upsert(
                identity,
                TransportKind::Quic,
                TransportAddr::LibP2p("/ip4/127.0.0.1/tcp/44444".parse().unwrap()),
            );
        }));
    }

    // Spawn 100 lookup tasks
    for i in 0..100 {
        let table = table.clone();
        handles.push(tokio::spawn(async move {
            let seed = format!("seed {}", i);
            let identity = IdentityId::from_master_seed(seed.as_bytes());
            let table = table.read().await;
            let _ = table.lookup(&PeerQuery::Identity(identity));
        }));
    }

    // Spawn 10 GC tasks
    for _ in 0..10 {
        let table = table.clone();
        handles.push(tokio::spawn(async move {
            let mut table = table.write().await;
            let _ = table.gc_stale();
        }));
    }

    // Wait for all tasks
    for handle in handles {
        handle.await.unwrap();
    }

    // Verify table is in valid state
    let table = table.read().await;
    assert!(table.peer_count() <= 10000);
}
```

### 6.4 Test Environment Setup

All integration tests require a test environment with:

1. **Multiple nodes**: At least 3 nodes for cross-transport testing
2. **Mock transports**: Ability to simulate transport failures
3. **Time control**: Fast-forward time for GC and timeout testing
4. **Network simulation**: Ability to simulate network partitions

```rust
/// Test harness for multi-node testing
struct TestHarness {
    nodes: Vec<TestNode>,
}

struct TestNode {
    identity_id: IdentityId,
    peer_table: Arc<RwLock<UnifiedPeerTable>>,
    transport_router: Arc<RwLock<TransportRouter>>,
    transport_selector: Arc<RwLock<TransportSelector>>,
    disaster_level: Arc<RwLock<DisasterLevel>>,
}

impl TestHarness {
    /// Create a test harness with N nodes
    async fn new(node_count: usize) -> Self;

    /// Simulate transport failure for a specific node
    async fn simulate_failure(&self, node_idx: usize, transport: TransportKind);

    /// Restore transport for a specific node
    async fn restore_transport(&self, node_idx: usize, transport: TransportKind);

    /// Get peer table for a specific node
    fn get_peer_table(&self, node_idx: usize) -> &Arc<RwLock<UnifiedPeerTable>>;
}
```

---

## 7. Reference Projects

The following projects inform the GhostWire symbiotic architecture design:

### 7.1 radicle-reticulum

**Source**: `git.hswro.org/mab122/radicle-reticulum`

**Relevance**: TCP to RNS bridge using RNS.Buffer for stream data and native RNS Packets for small messages. Demonstrates that bridging at the application layer (not transport layer) is viable and practical.

**Key insight**: The bridge does not modify either protocol. It translates at the application layer, keeping each transport native. This validates GhostWire's Principle 4 (bridge at the application layer).

### 7.2 Qortal (markqvist discussion #290)

**Source**: GitHub discussion on RNS repository

**Relevance**: Authoritative answer from RNS creator (markqvist) on how to bridge TCP and RNS. Confirms that gateway mode is the correct approach for cross-transport discovery.

**Key insight**: Gateway-mode nodes discover unknown paths on behalf of clients. This is exactly what GhostWire needs for radio-to-internet bridging.

### 7.3 lunarcore AddressTranslator

**Source**: lunarcore project

**Relevance**: Single public key to derived per-network hashes. Demonstrates deterministic identity derivation across multiple networks from a single seed.

**Key insight**: One master seed can deterministically derive unique identifiers for each transport/network. This validates GhostWire's Principle 3 (shared identity layer).

### 7.4 Briar

**Source**: briarproject.org

**Relevance**: One identity, per-transport plugins, BTP (Briar Transport Protocol) security above all. Demonstrates that a single identity can span multiple transports with transport-specific security.

**Key insight**: Transport plugins are independent; the identity layer is shared. Security is enforced at the transport level, not the identity level. This informs GhostWire's approach to per-transport key derivation.

### 7.5 libp2p pubsubPeerDiscovery

**Source**: libp2p ecosystem

**Relevance**: Gossipsub topic for cross-transport peer discovery. Demonstrates that pub/sub can be used for peer discovery across heterogeneous transports.

**Key insight**: A dedicated gossipsub topic (`ghostwire/announce`) enables cross-transport discovery without modifying the underlying transports. This validates GhostWire's gossipsub announce design.

### 7.6 RNS-over-MeshCore

**Source**: MeshCore project

**Relevance**: Custom RNS Interface bridging to another mesh protocol. Demonstrates that RNS can be extended with custom interfaces for specific use cases.

**Key insight**: RNS's interface model is extensible. GhostWire can add a custom interface for libp2p bridging without modifying RNS core.

---

## 8. Glossary

### Core Terms

**RNS / Reticulum**: The Reticulum Network Stack. An overlay routing stack for constrained networks. Provides transport-agnostic routing, encryption, and authentication. Handles radio links, store-and-forward, and heterogeneous network segments.

**AutoInterface**: RNS zero-config IPv6 multicast peer discovery over local Ethernet/WiFi. Uses UDP port 29716 (discovery) and 42671 (data transport). No IP infrastructure needed. Uses link-local IPv6 (fe80::/10) for multicast discovery.

**Gateway Mode (gw)**: RNS interface mode that runs full operations plus discovers unknown paths on behalf of clients. The cross-transport bridge mode for GhostWire.

**Boundary Mode**: RNS interface mode marking segment edges between heterogeneous networks (e.g., LoRa to Internet). Controls announce propagation between segments.

**libp2p**: Modular peer-to-peer networking stack. Provides transport protocols (QUIC, TCP, WebSocket), peer discovery (mDNS, Kademlia DHT), pub/sub messaging (gossipsub), and peer identification (Identify protocol).

**gossipsub**: libp2p's pub/sub protocol. Used for message broadcasting across the mesh. GhostWire uses a dedicated topic (`ghostwire/announce`) for cross-transport peer discovery.

**Kademlia**: libp2p's DHT protocol. Used for content routing and peer lookup. GhostWire extends Kademlia to store records for RNS-only peers.

**Sphinx**: Encryption protocol for anonymous routing. Used by both libp2p (in GhostWire) and RNS for message encryption.

### Architecture Terms

**Unified Peer Table**: The central data structure mapping one peer identity to multiple transport addresses. Enables cross-transport routing and discovery.

**Cross-Pollinate Discovery**: When one transport's discoveries feed into another's routing knowledge. E.g., a peer discovered via mDNS is automatically registered with RNS.

**Transport Bridging**: Routing messages through the best available transport for a specific peer. Uses Thompson Sampling to select the optimal transport.

**Thompson Sampling**: Bayesian bandit algorithm for transport selection. Maintains a Beta distribution for each transport, samples from distributions to select the best transport.

**IdentityId**: The canonical 32-byte identity derived from `master_seed` via BLAKE3. Deterministically derives per-transport identifiers (PeerId, AddressHash, DID).

**UnifiedRecipient**: An enum representing a message recipient that can be a PeerId, AddressHash, or IdentityId. Enables `send_direct_message` to accept any identifier type.

**TransportRouter**: The component that routes messages to peers across multiple transports. Uses Thompson Sampling and disaster level to select the optimal transport.

**MessagePriority**: The priority level of a message (Normal, Important, Critical). Critical messages use dual routing (both transports simultaneously).

### Disaster Terms

**DisasterLevel**: The current state of transport health (Normal, Congested, Isolation, Blackout). Determines which transports are available and their priority order.

**DisasterPolicy**: Configuration mapping disaster levels to transport priority lists. Determines which transports are preferred at each disaster level.

**DisasterDetector**: Component that monitors transport health and determines the current disaster level.

### Transport Terms

**TransportKind**: Enumeration of available transports (Quic, StealthTcp, Mesh, Bluetooth, WiFi, Reticulum, Blackhole).

**WeightProfile**: Thompson Sampling parameters for a specific transport. Includes base success rate, latency factor, loss factor, and recovery bonus.

**RoutingDecision**: The result of transport selection for a specific peer. Can be LibP2p, Reticulum, Dual, or Unreachable.

**TransportAddress**: Per-transport address with quality metrics (RTT, success rate, success/failure counts).

### Identity Terms

**master_seed**: The root secret from which all transport-specific keys are derived. Stored securely, never transmitted.

**IdentityId**: 32-byte canonical identity derived from `master_seed` via BLAKE3.

**PeerId**: libp2p peer identifier derived from an ed25519 public key.

**AddressHash**: RNS 16-byte identifier derived from the transport key.

**DID**: Decentralized Identifier in the format `did:ghost:<hex>`.

---

*End of Document*
