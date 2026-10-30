# GhostWire Wire Protocol Specification

> **Version**: 0.1-draft | **Status**: Draft | **Date**: 2026-10-28
> **Line of record**: `rescue/gw-push/ghostwire` (branch `rescue/sep-disk-tree`)
> **Companion docs**: `GHOSTWIRE_CRYPTOGRAPHIC_SECURITY_MODEL.md`, `THREAT_MODEL.md`, `MESSAGE_DELIVERY_SPEC.md`, `GHOSTWIRE_IFAC_DISTRIBUTION_DESIGN.md`, `GHOSTWIRE_PRIVATE_RNS_DESIGN.md`, `RNS_BLE_WIFI_INTERFACE_DESIGN.md`, `RESILIENCE_SPEED_AUDIT.md`, `MATH_PHYSICS_AUDIT.md`
> **Gap closed**: ledger listed "no standalone wire spec doc" as a main gap (remediation audit 2026-10-02).

---

## 1. Scope

This document is the **standalone wire protocol specification** for GhostWire: the on-the-wire formats, encryption layers, routing headers, DTN bundle format, transport framing, discovery, and the resilience policy that binds them. It is the single source of truth a third party needs to interoperate or implement a compatible node. It does **not** redefine delivery guarantees (see `MESSAGE_DELIVERY_SPEC.md`) or the threat model (see `THREAT_MODEL.md`); it references them.

**Design invariants (from `GHOSTWIRE_ARCHITECTURE_LESSONS.md`):** Security, Privacy, Anonymity, Anti-censorship (and now Resilience, per `PRINCIPLES_AUDIT.md`) are evaluated at every layer below. No layer may weaken a lens of a higher layer.

---

## 2. Layering model

```
┌─────────────────────────────────────────────────────┐
│  Application / WebUI / Tauri client                  │
├─────────────────────────────────────────────────────┤
│  Message layer  (sealed envelope, delivery token)     │  §4
├─────────────────────────────────────────────────────┤
│  Crypto layer   (ratchet, PQC, IFAC gating)          │  §3, §5
├─────────────────────────────────────────────────────┤
│  Routing layer  (L1–L4 cascade, Sphinx, SURB)        │  §6
├─────────────────────────────────────────────────────┤
│  DTN layer      (bundles, TTL, priority, shards)    │  §7
├─────────────────────────────────────────────────────┤
│  Transport layer (libp2p TCP/QUIC, RNS, BLE, WiFi)  │  §8
├─────────────────────────────────────────────────────┤
│  Radio / link   (LoRa/BLE/WiFi/serial, link budget) │  §8.4, MATH_PHYSICS §2
└─────────────────────────────────────────────────────┘
```

Resilience policy (§10) is a cross-cutting gate that runs **before** transport selection: censorship removes transports from the feasible set.

---

## 3. Identity & key material

### 3.1 Hybrid identity key (1248 bytes)
Every node holds one hybrid identity key, stored on both sides of a pairing:
```
[ Ed25519  32 ][ X25519  32 ][ ML-KEM-768 pk  1184 ] = 1248 bytes
```
- **Ed25519** — signing, identity, message authentication.
- **X25519** — classical ECDH for forward-secret key agreement.
- **ML-KEM-768** (pqcrypto-mlkem, PQClean reference = FIPS 203 final) — post-quantum key encapsulation.
- Entropy source: `OsRng`. No nonce reuse. User-keyed path additionally uses Argon2id.

Verified live: two machines over Tailscale exchanged identical 1248-byte hybrid keys (remediation audit 2026-10-02).

### 3.2 IFAC (Interface Access Control) seed
- Stored at `~/.ghostwire/ifac_seed`.
- Gates which peers may join a GhostWire-private network. Public RNS peers are bridged but cannot see GhostWire peers without a valid IFAC.
- Distribution design: `GHOSTWIRE_IFAC_DISTRIBUTION_DESIGN.md` (QR/certificate-based, not QR-only, to support non-colocated users).
- **Math gap (P0):** IFAC root secret should be split via Shamir (k,n) threshold sharing with rotation by re-sharing, so no single node compromise breaks the network. See `MATH_PHYSICS_AUDIT.md` §3.3.

### 3.3 Key agreement
- Anonymous intro uses SURB-based delivery; **gap (Briar audit):** no key commitment verification → key-substitution possible. Recommendation: add HKDF key commitment (16-byte truncation of pubkey) + mutual HMAC confirmation after SURB delivery.
- Sealed sender outer envelope (§4) moves sender identity inside the encrypted payload.

---

## 4. Message envelope (sealed sender)

Following Signal's sealed-sender design, adapted for GhostWire:

```
┌──────────────────────────────────────────────┐
│ Outer (unencrypted, minimal)                 │
│  - recipient routing hint (opaque token)      │
│  - delivery token                            │
│  - ciphertext (inner envelope)               │
├──────────────────────────────────────────────┤
│ Inner (encrypted to recipient)               │
│  - sender_id                                 │
│  - timestamp                                 │
│  - message type                              │
│  - payload (application data)                │
│  - vector clock (causal ordering)            │
└──────────────────────────────────────────────┘
```

- **Delivery tokens** map to contact gating (who may deliver to whom).
- **Sender certs** (optional, for verified senders) require IFAC membership; for anonymous senders the sender_id is a one-time ephemeral key.
- Consequence: observers see only recipient routing hint + size; sender identity is hidden unless the recipient decrypts.

Spec reference: `docs/spec/SEALED_SENDER.md`.

---

## 5. Encryption pipeline

Two modes, selected by message class and user preference (see §10):

| Mode | Use | Properties |
|---|---|---|
| **User-keyed** | Authenticated conversations | Argon2id on user passphrase; `/api/identity/setup` persists 13KB state; forward secrecy via double ratchet |
| **Anonymous** | Unlinkable delivery | Ephemeral keys, SURB reply, no long-term identity linkage |

- Forward secrecy via `double_ratchet.rs` (X25519 + ML-KEM ratchet).
- Compression (`compression.rs`) applied before encryption; chunking (`chunking.rs`) after.
- **Gap:** `activate_user_keyed` (encryption.rs:321) has no live caller outside internal + test; wiring is part of the message-path remediation.

---

## 6. Routing layer

### 6.1 Cascade (ROUTING.md L1–L4)
1. **L1 LightGBM** — fast path prediction.
2. **L2 Gemma** — local LLM fallback.
3. **L3 GNN** — graph-based routing.
4. **L4 PRoPHET** — DTN probabilistic fallback.

### 6.2 Anonymous routing
- **Sphinx** onion routing (currently 2-hop min — flagged weak; recommend ≥3 hops, padding to fixed size).
- **SURB** for anonymous replies.
- Cover traffic: `CoverTrafficScheduler` is live via `TrafficArmor` constructor side effect (mesh.rs:783–786, 1618) publishing `FrameKind::CoverTraffic` on the gossipsub DM topic. Underscore binding does not imply discard.

### 6.3 Transport selection
- `transport_selector.rs`: DORS taxonomy + Thompson Sampling, outputs a transport **and** (future) coding rate.
- Feasible set is pruned by the anti-censorship gate (§10) before scoring.

### 6.4 Discovery & rendezvous
- Kademlia DHT (`/ghostwire/kad/`), gossipsub (`/meshsub/1.1.0`), protocol id `rust-libp2p/0.47.0`.
- Discovery bridge + directory (`discovery_bridge.rs`, `discovery_directory.rs`).
- Bridge mode: GhostWire-private RNS (IFAC-gated) vs public RNS peers; standardize transport codes for zoning/filtering (MeshCore lesson).

---

## 7. DTN bundle format

```
DTN Bundle {
  bundle_id:     [u8; 32]       // SHA-256 of canonical content
  created_at:    u64            // ms
  ttl_ms:        u64            // 0 = immediate, MAX = never expires
  priority:      BundlePriority  // Critical/High/Normal/Low
  hop_count:     u8
  max_hops:      u8             // default 10
  source:        NodeId?        // omitted in anonymous mode
  destination:   RoutingHint    // opaque token (sealed sender)
  payload:       Vec<u8>        // encrypted inner envelope or erasure shard
  erasure:       Option<ErasureHeader>  // §7.1
  signature:     Ed25519Signature?  // omitted for anonymous
}
```

### 7.1 Erasure coding (P0, MATH_PHYSICS §3.1)
- For unreliable paths, a bundle may be split into `n` shards where any `k` reconstruct the original (Reed-Solomon for small blocks; LT/Raptor for large; COPE-style XOR as a lightweight fallback).
- Shard header: `{ bundle_id, shard_index, total_shards, k_threshold, checksum }`.
- Replaces whole-bundle retransmission on lossy radio links; interacts with DTN TTL (shards inherit parent TTL).

### 7.2 Default TTLs
Per `MESSAGE_DELIVERY_SPEC.md` §6.3: Text 24h/10 hops, File 1h/5, System 1h/3, Discovery 30s/2.

---

## 8. Transport framing

### 8.1 libp2p (TCP + QUIC)
- Mesh transport, peer discovery, protocol ids above. QUIC for NAT traversal; TCP fallback.

### 8.2 Reticulum (RNS)
- Python reference + Rust port (`reticulum.rs`). Interface design: `RNS_BLE_WIFI_INTERFACE_DESIGN.md`.
- IFAC gating for GhostWire-private sub-mesh; public RNS bridged.
- **Gap:** RNS does not carry traffic yet (remediation audit). BLE/WiFi interfaces not coded (`bluetooth.rs` is a 63-line scaffold with false-success `send_ble_data`).

### 8.3 BLE (planned)
- NUS UUIDs `6e400001/0002/0003`, KISS framing, MTU 500 logical with chunked writes, central+peripheral lifecycle, integration via `ReticulumTransport::`.
- Companion nodes do not repeat by default; bounded client repeating opt-in (MeshCore lesson).

### 8.4 WiFi / serial
- WiFi TCP/UDP/Direct; serial KISS bridge for host applications (PC/phone driving a radio node). KISS protocol spec: MeshCore `docs/kiss_modem_protocol.md` (reference).

### 8.5 Link budget (physics)
- All radio transports must declare a link budget + propagation model + fading margin per `MATH_PHYSICS_AUDIT.md` §2 before "range/meshing" claims are made. Coding rate is a transport-selector output (§6.3), not a fixed constant.

---

## 9. Metadata minimization & anti-analysis

- Padding to fixed sizes; timing chaff; cover traffic (`FrameKind::CoverTraffic`).
- Speed optimizations are metadata leaks (padding/timing-chaff/hops/cover) — anti-censorship gate runs before speed term (RESILIENCE_SPEED_AUDIT.md).
- No plaintext sender on the wire (sealed envelope).
- Differential privacy (ε-budget) for any aggregate telemetry (MATH_PHYSICS §3.6).

---

## 10. Resilience policy (cross-cutting)

From `RESILIENCE_SPEED_AUDIT.md`:

```
U(a) = R(a) − λ(s,p,c)·S(a) − Σ w_k·C_k(a)
λ(s,p,c) = λ_base(s) · κ_pref(p) · κ_msg(c)
```

- **Inputs:** action set A, environmental state s (Disaster Level), user preference p, message class c.
- **Anti-censorship is a hard feasibility gate**, not a weight: censorship removes transports from A (fast transports become infeasible/fingerprintable) before scoring.
- λ low when internet-up/uncensored (speed ≈ resilience); high under censorship/isolation; ∞/lexicographic in blackout.
- Unifies Disaster Levels, DTN priority/TTL, and the L1→L4 cascade under one objective.

---

## 11. Versioning & extensibility

- Protocol version field in outer envelope; unknown fields ignored (forward compat).
- V2 protocol discussion (MeshCore) covers path hashes, new encryption specs, sub-meshes, transport codes for zoning, dynamic coding rate, multiple virtual nodes, LZW compression, round-trip manual paths. GhostWire tracks these but applies stricter anonymity/PQC constraints; do not inherit crypto wholesale.
- IFAC distribution versioned independently (GHOSTWIRE_IFAC_DISTRIBUTION_DESIGN.md).

---

## 12. What is NOT specified here (deliberate)

- Delivery guarantees, ACK/retry/dedup/ordering/chunking → `MESSAGE_DELIVERY_SPEC.md`.
- Threat model & compensating controls → `THREAT_MODEL.md`, `COMPENSATING_CONTROLS_AUDIT.md`.
- Deployment, config, API → `DEPLOYMENT.md`, `CONFIGURATION.md`, `API.md`.
- Formal verification artifacts → future TLA+/Tamarin specs (MATH_PHYSICS §3.2).

---

## 13. References

- `GHOSTWIRE_CRYPTOGRAPHIC_SECURITY_MODEL.md`, `THREAT_MODEL.md`
- `MESSAGE_DELIVERY_SPEC.md`
- `GHOSTWIRE_IFAC_DISTRIBUTION_DESIGN.md`, `GHOSTWIRE_PRIVATE_RNS_DESIGN.md`
- `RNS_BLE_WIFI_INTERFACE_DESIGN.md`
- `RESILIENCE_SPEED_AUDIT.md`, `PRINCIPLES_AUDIT.md`, `COMPENSATING_CONTROLS_AUDIT.md`
- `GHOSTWIRE_ARCHITECTURE_LESSONS.md`
- `MATH_PHYSICS_AUDIT.md`
- Briar deep-dive (2026-10-08): HKDF labels, BQP commitment, BSP OFFER/REQUEST/PAYLOAD/ACK
- Sealed sender research: `docs/spec/SEALED_SENDER.md`
- Obfuscation research: `docs/OBFUSCATION_TRAFFIC_ANALYSIS_RESEARCH.md`
- MeshCore: github.com/meshcore-dev/MeshCore (KISS protocol, V2 discussion)