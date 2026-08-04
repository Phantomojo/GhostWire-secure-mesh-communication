# External Project Learnings — TWIN, IOTA, Fognet

**Created:** 2026-08-02  
**Purpose:** Patterns GhostWire can adopt from TWIN, IOTA Identity, and Fognet  
**Scope:** GhostWire self-improvement (NOT integration with these projects)

---

## Context

GhostWire is a Rust-based mesh communication platform built on libp2p with sphinx onion routing, content-addressed knowledge storage, and a reputation system. Research into three external projects revealed concrete patterns GhostWire can adopt to improve its own capabilities.

**Projects researched:**
- **TWIN** (Trade Worldwide Information Network) — IOTA Foundation's trust, identity, and data-sharing framework
- **IOTA Identity / Notarization** — DID documents, credential revocation, hash anchoring on IOTA Rebased
- **Fognet** — BLE mesh with IOTA flash-channel micro-payments (Evanfeenstra, 2018-2019)

---

## 1. Wire TrustStore into Routing Decisions

**Gap:** GhostWire's `reputation.rs` (402L) is fully implemented — TOFU + WoT trust scoring, attestation verification, block/verify — but **never consulted in routing or connection decisions**. Only used for health telemetry (`mesh_health.rs` reads `blocked_count()` and `average_trust_score()`).

**Pattern from TWIN:** DSpaceConnector re-verifies a VC bearer token **on every protocol call**. No session trust. Every `negotiateAgreement()` call carries a trust payload that's verified fresh.

**Adoption:**
- Feed `trust_store.trust_score` into `mesh.rs` routing decisions as a feature weight
- The AI classifier in `routing_classifier.rs` already has a `trust_score` field — nothing feeds it
- One integration call at decision time closes the biggest open HIGH issue (AGENTS.md)

**Effort:** Small. **Impact:** High.

---

## 2. Pluggable Verifier Chain for Identity Checking

**Gap:** `core_identity.rs` (849L) does Ed25519 verify + X3DH key agreement + group key check in one monolithic function. Adding new identity types requires modifying the core.

**Pattern from TWIN:** `ITrustComponent.verify()` runs a chain of pluggable verifiers against an opaque payload. Any verifier returning `true` succeeds. `JwtVerifiableCredentialVerifier` checks JWT format → expiry → DID resolution → signature → revocation. `IdentityAllowDenyVerifier` is a pure ACL gate. New verifiers are registered via factory.

**Adoption:**
- Define a `TrustVerifier` trait in Rust
- Split current identity verification into: `DeviceKeyVerifier`, `GroupKeyVerifier`, `AttestationVerifier`
- Same logic, extensible — new identity types get a new verifier, no core changes

**Effort:** Medium. **Impact:** Medium — extensibility foundation.

---

## 3. Compact Denylist via Revocation Bitmap

**Gap:** GhostWire has no denylist mechanism beyond `reputation.block_node()` which sets a flag. No compact representation for mesh-wide distribution.

**Pattern from IOTA Identity:** `RevocationBitmap2022` stores a compressed RoaringBitmap (32k credentials in ~4KB) as a DID document service. One bit per revoked credential. Zlib-compressed, base64-embedded in the endpoint URL.

**Adoption:**
- Add a `denylist_bitmap` field to `CoreIdentity` serialized in the identity document
- A gateway publishes a compact denylist inside its own identity
- Peers resolve it off-chain, check locally — no per-check transactions
- 10k banned peers costs ~4KB of storage

**Effort:** Small. **Impact:** Medium — mesh security.

---

## 4. BlockStore File Transfer API

**Gap:** `blockstore.rs` (556L) is fully implemented — content-addressed storage, wantlist, per-peer debt ledger, Bitswap message types, GC — but has **zero API exposure**. No `store_file`/`retrieve_file`/`want_manifest` callers outside the module. Only `block_count()` stat consumed by `mesh_health.rs`.

**Pattern from Fognet:** Flash channel metering: `spend(packetCount)` after each page load. Pay-per-byte. 18-byte BLE packet framing with `<*>cmd` header and `<^>` terminator.

**Adoption:**
- Expose `store_file`/`retrieve_file`/`want_manifest` through axum routes (`/v2/files/*`)
- Wire the WantList into mesh events (Bitswap message type already defined)
- Use Fognet's frame protocol as reference for chunked transfer over constrained transports

**Effort:** Medium. **Impact:** High — unlocks file transfer.

---

## 5. Content Access Policies (Lightweight ODRL)

**Gap:** Knowledge ark entries have no access control. Anyone with the CID can read.

**Pattern from TWIN:** ODRL policies as signed JSON-LD objects. PEP enforces at serve time. Closed world: no applicable permission = deny. JSONPath field-level targeting for fine-grained control.

**Adoption (minimal):**
- Store a simple policy object alongside content in the ark (content-addressed)
- At serve time: `permission` → allow, `prohibition` → deny, no match → deny
- Start with just `read` permission + `allowed_peers` list
- Skip full DSP negotiation layer — just the PAP/PDP/PEP slice

**Effort:** Medium. **Impact:** Medium — data sovereignty.

---

## 6. CID Anchoring with Version Counter

**Gap:** Knowledge ark has no versioning or tamper-evidence. No way to verify "this is the latest version of X".

**Pattern from IOTA Notarization:** `Notarization<D>` stores data + `state_version_count` in a shared object. Verification: fetch object, confirm count matches claim, compare hash. Anti-stale via monotonic counter.

**Adoption:**
- Anchor the latest CID of each content category in a local signed record
- `state_version_count` prevents stale-data replay
- For offline mesh: local signed record, not IOTA mainnet — same anti-stale pattern, different storage

**Effort:** Small. **Impact:** Low — tamper-evidence.

---

## 7. Resource-Constrained Device Support

**Gap:** GhostWire assumes full libp2p nodes. No special handling for ESP32, phones on low battery, or IoT devices with limited CPU/RAM.

### 7.1 ESP32 Hardware Reality (from libp2p feasibility research)

**Core finding: rust-libp2p cannot compile for ESP32.** No `no_std` target exists, tokio only supports desktop/WASM, and a minimal swarm needs ~MB of RAM vs ~148KB free on ESP32 with WiFi+BLE active.

**ESP32 constraints (hard numbers):**
- **RAM**: 520KB SRAM, ~390KB heap; WiFi+BLE costs ~205KB, leaving **~148KB free**
- **Radio options** (from Espressif official comparison):
  - ESP-NOW: <0.5 Mbps, <100 nodes, <150m range, 250B payload (v1) / 1470B (v2), 20 unicast peers
  - BLE mesh: <1 Kbps, theoretical 32767 nodes (tested ~100), 50m range — **control-plane only**
  - ESP WiFi Mesh: up to 20 Mbps, <100 nodes, 6 layers
- **No existing libp2p-on-ESP32 project** — only a dead fork (0 stars, no actual code)
- **What works**: `esp-hal` + `esp-wifi` 0.14/0.15 (pre-1.0, requires `unstable` feature) + Embassy for bare-metal async WiFi/BLE/ESP-NOW

**Proof that the hard part is solvable**: `Amperstrand/microfips` runs `no_std` Noise_IK/XK handshakes + full framing protocol on ESP32-D0WD with Embassy — crypto on ESP32 is a solved problem. The blocker is libp2p machinery, not crypto.

### 7.2 Proven Patterns from Constrained Mesh Projects

**Meshtastic (13 device roles, not binary full/leaf):**
- Role enum: `CLIENT`, `CLIENT_MUTE`, `ROUTER`, `ROUTER_CLIENT`, `REPEATER`, `TRACKER`, `SENSOR`, `TAK`, `CLIENT_HIDDEN`, `LOST_AND_FOUND`, `TAK_TRACKER`, `ROUTER_LATE`, `CLIENT_BASE`
- Each role is a **4-dimensional profile**: forwarding duty (6 modes: ALL, LOCAL_ONLY, KNOWN_ONLY, CORE_PORTNUMS_ONLY, NONE, ALL_SKIP_DECODING), power behavior (sleep policy), storage tier, messagability (`is_unmessagable` flag)
- Role defaults set via single switch (`installRoleDefaults`) — not scattered config
- Security: ROUTER role restricted to authenticated firmware builds only
- Storage: `WarmNodeStore` packs role into 4 bits, hot/warm tiers for flash-limited devices
- Hop floors: reporting roles (TRACKER ≥2 hops, SENSOR ≥1) stay reachable regardless of network density

**Berty (mobile nodes drop DHT entirely):**
- Node types: `peer` (full), `replication` (stores encrypted data without decrypting), `relay` (TURN-like), `bootstrap` (DHT seed), `rdvp` (rendezvous)
- Mobile peers: disable mDNS, DHT random-walk, listeners — **outbound-only, dial-only**
- Sacrifices: discovery (must know rdvp), availability (needs replication nodes), addressability (relay-only)
- Replication nodes store encrypted group history — can't decrypt, only forward

**Briar (storage-constrained sync):**
- BSP (Bramble Synchronisation Protocol): bounded offer lists, client-controlled deletion
- Mailbox pattern: external always-on buffer for leaf devices — owner's phone stays offline
- LPN/friend model from BLE Mesh: power-constrained nodes offload storage+forwarding to a "friend"

**libp2p circuit relay (bootstrap, not data path):**
- 128 max reservations, 2-min circuit duration, 128KiB data limit per circuit
- Designed for hole-punching bootstrap (dcutr), not sustained leaf traffic
- For sustained leaf connectivity: need Berty-style persistent relays or reservation renewal

**Academic (BT Mesh + libp2p split, Sensors 2025):**
- Delegate peer discovery to the constrained radio (BLE advertising)
- Keep libp2p only at routing/connection layer on gateway-class device
- Measured: ~8s latency, 95-99% PDR — viable when host is gateway, radio does discovery

### 7.3 GhostWire Adoption Plan

**Device capability model — role enum, not boolean:**

| Role | Forwarding | Power | Storage | Dialable | Identity |
|------|-----------|-------|---------|----------|----------|
| `Full` | Full DHT + gossip | Always on | Full node store | Yes (listeners) | Standard |
| `Relay` | CORE_PORTNUMS_ONLY | Low-current/solar | Warm tier | Yes (relay reservation) | Restricted (authenticated builds) |
| `Leaf` | LOCAL_ONLY (never relays) | Sleep-heavy (PowerFSM) | Minimal (own groups only) | No (outbound only) | Simplified |
| `Sensor` | LOCAL_ONLY | Ultra-low (duty cycle) | Read-only | No | Ephemeral |
| `Replicator` | Forward encrypted blocks | Always on | Full (can't decrypt) | Yes | Service identity |

**Gateway-bridged architecture (proven pattern):**
1. **ESP32 side**: bare-metal `esp-hal` + `esp-wifi` + Embassy, ESP-NOW or BLE for device-to-device mesh
2. **Bridge**: ESP32s with WiFi uplink run gateway process that terminates tiny device mesh and speaks full libp2p to GhostWire
3. This mirrors Meshtastic (LoRa mesh + MQTT bridge) and microfips (no_std Noise node + serial/BLE bridge)

**Operation gating by capability:**
- Leaf: send/receive only, no forwarding, no storage responsibility
- Relay: authenticated builds only (Meshtastic pattern), rate-limited reservations
- Full: standard libp2p node with all responsibilities

**Storage tiers (Meshtastic WarmNodeStore pattern):**
- Hot: active peer metadata (in RAM)
- Warm: evicted peer records with 4-bit role field (in flash/limited storage)
- Leaf keeps only its own groups; relay keeps routing table; full keeps everything

**Serialized packet writes for constrained transports (Fognet pattern):**
- Packets written sequentially to BLE/ESP-NOW to avoid buffer overrun
- `packets.reduce((prev,val) => prev.then(() => write(val)), Promise.resolve())`

**Effort:** Large (device capability model + gateway bridge + role-based defaults). **Impact:** High — enables IoT/ESP32 integration, battery-powered devices, sensor networks.

---

## 8. Re-entrancy Guards on Async Handlers

**Gap:** No protection against duplicate in-flight requests on slow radio paths.

**Pattern from Fognet:** Every async action checks `if(!this.state.gettingBalance)` before starting, sets a boolean flag, resets on completion. Prevents double-submit.

**Adoption:**
- Add `Mutex<bool>` or `AtomicBool` guard on libp2p request/response handlers
- Especially for sphinx packet construction (CPU-bound) and mesh message sends (network-bound)
- One guard per operation

**Effort:** Small. **Impact:** Low — prevents edge-case races.

---

## 9. Knowledge Ark Signature Verification

**Gap:** `knowledge_ark.rs` (569L) has a `signature: Option<Vec<u8>>` field that's always `None` — never signed, never verified. `import_entry` verifies CID only, not signature.

**Pattern from IOTA Identity:** VCs are signed with issuer's Ed25519 key. Verification resolves the issuer DID document on-ledger and checks signature + revocation.

**Adoption:**
- Sign ark entries with the creator's Ed25519 key (already in `CoreIdentity`)
- Verify by checking signature against creator's public key in identity document
- Add verification in `import_entry` alongside existing CID check

**Effort:** Small. **Impact:** Medium — closes dead code path.

---

## 10. Federated Content Discovery

**Gap:** Ark has no discovery mechanism beyond direct search. No way to find "what content exists in this mesh section".

**Pattern from TWIN Federated Catalogue:** DCAT 3 datasets, publisher-owned entries, owner-only writes, publisher-grouped queries. Verified identity ⇒ ownership.

**Adoption:**
- Publisher-owned registry pattern maps to GhostWire's DHT
- Content-addressed entries, owner-signed, no central broker
- "Verified identity ⇒ ownership" rule prevents spoofed entries
- Pubsub records keyed by owner with write-auth by verified identity

**Effort:** Medium. **Impact:** Medium — content discovery.

---

## Priority Matrix

| Priority | Pattern | Effort | Impact |
|---|---|---|---|
| **P0** | Wire TrustStore into routing | Small | High |
| **P0** | BlockStore file transfer API | Medium | High |
| **P0** | Resource-constrained device support (§7) | Large | High — enables ESP32, sensors, battery devices |
| **P1** | Ark signature verification | Small | Medium |
| **P1** | TrustVerifier chain trait | Medium | Medium |
| **P1** | Compact denylist bitmap | Small | Medium |
| **P2** | Content access policies | Medium | Medium |
| **P2** | CID anchoring with version counter | Small | Low |
| **P2** | Federated content discovery | Medium | Medium |
| **P3** | Re-entrancy guards | Small | Low |

---

## What NOT to Adopt

- **IOTA Rebased storage deposits** — GhostWire content lives in mesh DHT, not on-chain. Only anchor CIDs.
- **TWIN's full DSP negotiation** — Overkill for mesh. Adopt PAP/PDP/PEP slice without protocol state machine.
- **Fognet's Elastic Beanstalk deployment** — Useless for mesh. Transport-role separation is valuable; PaaS deployment is not.
- **IOTA Gas Station** — Only relevant if GhostWire anchors anything on IOTA mainnet.
- **Flash channel micro-payments** — GhostWire doesn't need token incentives; reputation-based trust is sufficient.

---

## Key Takeaway

**Two biggest wins, evidence-backed:**

1. **Wire TrustStore into routing** — GhostWire already has the trust scoring code, the routing classifier, and the mesh decision point — they just aren't connected. One integration task, massive security improvement.

2. **Resource-constrained device support is architecturally real, not speculative.** The research proves:
   - rust-libp2p **cannot** compile for ESP32 (no `no_std`, tokio-only, ~MB runtime vs ~148KB free RAM)
   - The proven architecture is **gateway-bridged**: ESP32 runs bare-metal ESP-NOW/BLE mesh, a bridge process terminates the device mesh and speaks full libp2p
   - Meshtastic's 13-role model (not binary full/leaf) is the right abstraction — 4 dimensions: forwarding duty, power policy, storage tier, dialability
   - Berty proves mobile nodes can drop DHT entirely; Briar proves bounded sync works for storage-constrained devices
   - The crypto layer works on ESP32 (microfips proves no_std Noise handshakes) — the blocker is libp2p machinery, not cryptography
   - A role enum with per-role defaults (`installRoleDefaults` pattern) is the single design decision that enables the whole feature

**What NOT to adopt:** IOTA storage deposits, TWIN's full DSP negotiation, Fognet's PaaS deployment, flash channel micro-payments. Only the transport-role separation, serialized packet writes, and operation-gating patterns are valuable.
