# Custom vs libp2p — Decision Matrix

**Date**: July 28, 2026

---

## Verdict Summary

| # | Feature | Verdict | Effort | Why |
|---|---------|---------|--------|-----|
| 1 | **Wire peer scoring** | **DO NOW** | +8 lines, 1 file | Code already written in skademlia.rs, just needs wiring. Anti-Sybil for free. |
| 2 | **Add connection limits** | **DO NOW** | +8 lines, 2 files | Zero risk, prevents resource exhaustion. libp2p-native. |
| 3 | **Add relay server** | **DO LATER** | +40 lines, 1 file | Only when you need relay infrastructure. Low risk but costs bandwidth. |
| 4 | **Replace custom ping** | **KEEP CUSTOM** | Would save -113 lines | Custom ping tests full gossipsub path + provides traffic obfuscation. libp2p ping only tests transport layer. |

---

## 1. Wire Peer Scoring → DO NOW

### Current State
- `hardened_peer_score_params()` and `hardened_peer_score_thresholds()` exist in `skademlia.rs` (L373-408)
- NOT wired — `mesh.rs:294` uses `Gossipsub::new()` (no scoring)
- Comment says: "peer scoring can be re-added after first live test"

### The Change (mesh.rs L293-298)
```rust
// BEFORE:
let gossipsub = Gossipsub::new(
    MessageAuthenticity::Signed(local_key.clone()),
    gossipsub_config,
)?;

// AFTER:
use crate::core::routing::skademlia::{
    hardened_peer_score_params, hardened_peer_score_thresholds,
};
let peer_score = Some((
    hardened_peer_score_params(),
    hardened_peer_score_thresholds(),
));
let gossipsub = Gossipsub::new_with_subscription_filter_and_transform(
    MessageAuthenticity::Signed(local_key.clone()),
    gossipsub_config,
    peer_score,
    libp2p::gossipsub::AllowAllSubscriptionFilter,
    libp2p::gossipsub::IdentityTransform,
)?;
```

### What This Buys You
- **Sybil resistance**: IP colocation penalty (-5.0 weight, threshold 3 peers/subnet)
- **Spam defense**: Invalid message penalty (-100.0 weight)
- **Mesh quality**: Peers must deliver messages to stay in mesh
- **Graylisting**: Score < -80 = stopped receiving gossip
- **Score thresholds**: gossip=-10, publish=-50, graylist=-80, opportunistic_graft=5

### Risk: LOW
Thresholds are lenient. Won't disrupt a live network. Memory overhead: ~50 bytes/peer.

---

## 2. Add Connection Limits → DO NOW

### Current State
- No connection limits enforced
- `peer_count_cache: Arc<AtomicUsize>` tracks count but doesn't enforce
- No per-peer, per-IP, or total connection caps

### The Change
```toml
# Cargo.toml — add to libp2p features:
"connection-limits"
```

```rust
// mesh.rs MeshBehaviour struct — add field:
connection_limits: libp2p::connection_limits::Behaviour,

// Constructor — initialize:
let limits = libp2p::connection_limits::ConnectionLimits::default()
    .with_max_established_incoming(Some(200))
    .with_max_established_outgoing(Some(200))
    .with_max_established_per_peer(Some(2))
    .with_max_pending_incoming(Some(32))
    .with_max_pending_outgoing(Some(32));
let connection_limits = libp2p::connection_limits::Behaviour::new(limits);
```

### What This Buys You
- Prevents resource exhaustion from connection floods
- Per-peer limit stops single peer from consuming all slots
- Silently rejects excess connections before any event fires
- Runtime-mutable limits (adjust without restart)
- Bypass list for trusted peers (bootstrap nodes)

### Risk: NEGLIGIBLE
Zero interaction with existing event handlers. Silent rejection means no code changes to connection handling.

---

## 3. Add Relay Server → DO LATER

### Current State
- Only relay CLIENT (line 185, transport at line 260)
- No relay SERVER behaviour
- No relay events handled

### What Would Change
- Add `relay_server: libp2p::relay::server::Behaviour` to MeshBehaviour
- Initialize with config (max 128 reservations, 16 circuits, 128KB/circuit, 2min duration)
- Handle 2-3 events (reservation accepted, circuit opened/closed)
- ~40 lines added

### When To Do It
- When you have stable nodes that can serve as relay infrastructure
- When you need NAT traversal for nodes that can't use DCUtR
- When the network is large enough that relay diversity matters

### Bandwidth Cost
16 circuits × 128KB × 2 directions = ~4MB worst case per circuit lifetime. Manageable for well-connected nodes.

---

## 4. Replace Custom Ping → KEEP CUSTOM

### Why Not Replace
| Aspect | Custom (WirePing/WirePong) | libp2p Ping |
|--------|---------------------------|-------------|
| Tests | Full gossipsub → frame → decode pipeline | Transport-level TCP only |
| Obfuscation | Same frame pipeline as real messages | Separate protocol (identifiable) |
| Interval | 2 seconds (fine-grained) | 15 seconds default |
| RTT tracking | Via peer_health (rolling avg, packet loss) | Event-based only |
| Dead peer detection | 30s timeout (is_alive flag) | No auto-disconnect |
| Lines saved | -113 (would remove) | +10 (would add) |

### The Key Insight
GhostWire's custom ping tests whether the **application path** works — can you gossipsub publish → frame decode → handle? libp2p ping only tests whether the **TCP connection** is alive. A connection can be alive but gossipsub broken. Custom ping catches that.

### Traffic Obfuscation
Custom pings go through the same FrameKind::Ping path as cover traffic and real messages. An observer can't distinguish pings from data. libp2p ping uses `/ipfs/ping/1.0.0` protocol — clearly identifiable on the wire.

### Verdict
The -113 lines saved aren't worth losing application-layer path validation and traffic obfuscation. Keep custom.

---

## Implementation Order

1. **Peer scoring** (8 lines, mesh.rs) — 10 minutes
2. **Connection limits** (8 lines, Cargo.toml + mesh.rs) — 10 minutes
3. **Relay server** (40 lines, mesh.rs) — when needed
4. **Custom ping** — keep as-is

Total immediate work: ~16 lines in 2 files, ~20 minutes.
