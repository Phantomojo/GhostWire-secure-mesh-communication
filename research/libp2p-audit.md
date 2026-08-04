# GhostWire libp2p Feature Audit

**Date**: July 28, 2026
**libp2p version**: 0.56 (Rust)

---

## 1. Cargo Features Enabled

| Feature | Status | Usage |
|---------|--------|-------|
| `tokio` | ✅ USED | Async runtime for SwarmBuilder |
| `macros` | ✅ USED | `#[derive(NetworkBehaviour)]` on MeshBehaviour |
| `tcp` | ✅ USED | Base transport with nodelay |
| `dns` | ✅ IMPLICIT | DNS multiaddr resolution (no explicit code reference) |
| `noise` | ✅ USED | Noise XX handshake for encryption |
| `yamux` | ✅ USED | Stream multiplexing |
| `kad` | ✅ USED | Kademlia DHT (MemoryStore, Server mode) |
| `gossipsub` | ✅ USED | Primary message transport |
| `mdns` | ✅ USED | Local peer discovery |
| `identify` | ✅ USED | Protocol/agent version exchange |
| `autonat` | ✅ USED | NAT type detection |
| `relay` | ✅ USED | Circuit relay client (NAT traversal) |
| `dcutr` | ✅ USED | Direct connection upgrade through relay |
| `ping` | ❌ ENABLED, NOT USED | GhostWire has custom ping/pong via gossipsub frames |
| `request-response` | ✅ USED | CBOR-based handshake + chat |
| `cbor` | ✅ USED | CBOR codec for request-response |

## 2. Transport Stack

```
tcp_transport (libp2p TCP + nodelay)
  .or_transport(relay_transport)         → circuit relay client
  .upgrade(Version::V1)
  .authenticate(noise::Config)           → Noise XX
  .multiplex(yamux::Config::default())   → Yamux
  .or_transport(webrtc_transport)        → libp2p-webrtc
  .boxed()
```

| Transport | Status | Notes |
|-----------|--------|-------|
| TCP | ✅ In stack | mesh.rs:256-257 |
| Noise | ✅ Wired | mesh.rs:275 |
| Yamux | ✅ Wired | mesh.rs:276 |
| Relay client | ✅ Wired | mesh.rs:260, 273 |
| WebRTC | ✅ Wired | mesh.rs:263-266, 277 |
| QUIC (standalone quinn) | ⚠️ Separate | NOT libp2p's QUIC |
| WebSocket | ❌ Not used | pluggable_transport.rs is raw WS, not libp2p's |

## 3. Behaviours (MeshBehaviour)

| Behaviour | Status | Notes |
|-----------|--------|-------|
| Gossipsub | ✅ Wired | Primary message transport |
| Kademlia | ✅ Wired | DHT routing + record storage |
| Identify | ✅ Wired | Peer identification |
| mDNS | ✅ Wired | LAN discovery |
| AutoNAT | ✅ Wired | NAT status detection |
| Relay client | ✅ Wired | Circuit relay |
| DCUtR | ✅ Wired | Direct upgrade through relay |
| Request-Response | ✅ Wired | CBOR handshake + chat |

## 4. Peer Discovery

| Mechanism | Status | Notes |
|-----------|--------|-------|
| mDNS | ✅ Active | Auto-discovers LAN peers |
| DHT Bootstrap | ⚠️ Minimal | `DEFAULT_BOOTSTRAPS = &[]` (empty) |
| Cloudflare Bootstrap | ✅ Active | Fetches peer multiaddrs from Worker |
| Identify | ✅ Active | Protocol exchange, adds peers to Kademlia |
| Kademlia routing | ✅ Active | get/put records, regional sharding |
| Username discovery | ✅ Active | DHT-based username→PeerId |

## 5. Gossipsub Configuration

| Parameter | Value |
|-----------|-------|
| ValidationMode | Strict |
| mesh_n | 2/8/12 (small/medium/large) |
| mesh_n_low | 1/4/8 |
| mesh_n_high | 4/12/20 |
| mesh_outbound_min | 1/3/4 |
| heartbeat_interval | 1s |
| history_length | 10 |
| history_gossip | 4 |
| MessageAuthenticity | Signed(local_key) |

**Peer scoring**: Functions exist in skademlia.rs but NOT wired into Gossipsub constructor.

## 6. Kademlia Configuration

| Parameter | Value |
|-----------|-------|
| Protocol name | `/ghostwire/kad/shard/global` |
| Store | MemoryStore (in-memory only) |
| Mode | Server |
| Record TTL | 24 hours |
| Quorum | Quorum::One |
| Provider records | NOT used |
| Persistent store | NOT used |

## 7. NAT Traversal

| Feature | Status |
|---------|--------|
| AutoNAT | ✅ Active |
| Relay client | ✅ Active |
| Relay server | ❌ Not configured |
| DCUtR | ✅ Active |

---

## 8. GAPS — libp2p Features NOT Used

| Feature | What It Does | Potential Value |
|---------|-------------|-----------------|
| **Relay server** | Act as relay for other nodes | Help NAT'd peers connect; earn reputation |
| **Kademlia provider records** | Content provider discovery | Find who has specific messages/content |
| **Persistent DHT store** | Survive restarts | Keep routing state across restarts |
| **libp2p QUIC** | Native QUIC transport | Lower latency than TCP+Noise+Yamux |
| **Ping behaviour** | Built-in health checks | Replace custom gossipsub-based ping |
| **Circuit relay v2 reservation** | Pre-book relay slots | More reliable NAT traversal |
| **Peer scoring (wired)** | Mesh quality enforcement | Anti-spam, anti-Sybil (code exists, not active) |
| **S/Kademlia PoW** | Sybil-resistant DHT | Harder to spin up fake nodes |
| **Diverse routing table** | IP-diverse peer selection | Better resistance to Eclipse attacks |
