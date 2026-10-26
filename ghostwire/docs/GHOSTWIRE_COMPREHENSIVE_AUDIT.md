# GhostWire Comprehensive Project Audit

**Date:** 2026-10-27  
**Basis:** All insights from 2026-10-26 session (security/privacy/anonymity lens, compensating controls, dependency chain, armory metaphor, resilience > speed, scaffolding = design)

---

## 1. Unwired Code Audit (Scaffolding Inventory)

### Modules with ZERO external callers (pure scaffolding — no production code paths)

| Module | Status | Notes |
|---|---|---|
| `babel_routing` | ❌ Unwired | No callers outside self |
| `briar` | ❌ Unwired | Only self-references |
| `chunking` | ❌ Unwired | Only mentioned in comments |
| `crypto_profile` | ❌ Unwired | No callers |
| `db` | ❌ Unwired | No callers |
| `dht_signatures` | ❌ Unwired | Only genesis_constants comment |
| `encrypted_store` | ❌ Unwired | Only mesh_health references (no calls) |
| `medium_adapter` | ❌ Unwired | No callers |
| `oem_battery_guide` | ❌ Unwired | No callers |
| `qr` | ❌ Unwired | Only qr_string in controllers |
| `sealed_sender` | ❌ Unwired | No callers (created 2026-10-26, never wired) |
| `sync_protocol` | ❌ Unwired | Only genesis_constants comment |
| `transport_ext` | ❌ Unwired | No callers |
| `transport_router` | ❌ Unwired | Only peer_table comment |
| `typed_crypto` | ❌ Unwired | No callers |

**Total: 15 modules are pure scaffolding with no callers.**

### Modules with minimal callers (1 caller — fragile wiring)

| Module | Callers | Status |
|---|---|---|
| `double_ratchet` | 1 | ⚠️ Barely wired |
| `monitor` | 1 | ⚠️ Barely wired |
| `passphrase_keyslot` | 1 | ⚠️ Barely wired |
| `peer_table` | 1 | ⚠️ Barely wired |
| `power` | 1 | ⚠️ Barely wired (created 2026-10-26) |
| `rotation` | 1 | ⚠️ Barely wired |
| `secure_wipe` | 1 | ⚠️ Barely wired |
| `traffic_armor` | 1 | ⚠️ Barely wired |
| `transport_manager` | 1 | ⚠️ Barely wired |
| `transport_selector` | 1 | ⚠️ Barely wired (only transport_manager) |
| `updater_fixed` | 1 | ⚠️ Barely wired |

### Key wiring gaps confirmed

| Question | Answer |
|---|---|
| Is `transport_selector` used in send path? | ❌ NO — only transport_manager references it (and transport_manager itself has 1 caller) |
| Is `sealed_sender` called anywhere? | ❌ NO — zero callers |
| Is `store_incoming` called in production? | ❌ NO — only in tests (mailbox.rs) |
| Is `init_reticulum` called? | ✅ YES — from controllers and `initialize_mesh_after_unlock()` |
| Is `active_transport` consumed by send_message? | ❌ NO — only controllers (display) and mesh_health (display) |
| Does RNS carry traffic? | ❌ NO — receives via pump, but send path only uses libp2p because rns_addr is always empty |
| Is cross-transport discovery functional? | ❌ NO — `rns_addr = String::new()` hardcoded in handshake, so no RNS addresses are ever exchanged |
| Is BLE transport wired? | ❌ NO — BluetoothTransport exists but is never registered in TransportRegistry; uses custom UUIDs not NUS |
| Is WiFi transport implemented? | ❌ NO — only design docs exist |
| Are privacy tiers (L0-L3) implemented? | ❌ NO — docs/PRIVACY_TIERS.md exists, no code |

---

## 2. Security/Privacy/Anonymity Lens Audit

### Per-component evaluation

| Component | Strong at | Weak at | Compensating control | Covered? |
|---|---|---|---|---|
| **mDNS discovery** | Speed, zero-config LAN | S: broadcasts presence; P: reveals identity; A: none | RNS encrypted announces (design only) | ❌ NO — stealth mode not implemented |
| **RNS transport** | Resilience (radio/serial/TCP), works offline | S: no auth by default; P: announce leaks topology; A: no anonymity | Sealed sender (unwired), Sphinx (2-hop) | ⚠️ PARTIAL — sealed sender unwired |
| **Adaptive selector** | Resilience (picks best transport) | S: switch patterns leak; P: switch patterns leak; A: none | Redact switch logs (not done) | ❌ NO |
| **Store-and-forward** | Resilience (offline delivery) | P: intermediate nodes see metadata; S: no expiry verification | Sealed sender (unwired), padding (not implemented) | ❌ NO |
| **Sealed sender** | P: hides sender identity (X25519+HKDF+XChaCha20-Poly1305) | S: no AAD, no replay, no expiry; A: pseudonymous (BLAKE3 pubkey hint) | AAD+replay+expiry wiring (not done) | ❌ NO |
| **Sphinx routing** | A: multi-hop unlinkability | A: only 2 hops (too weak); P: real PeerId hop tokens leak path | Increase to 3+ hops + anonymous IDs (not done) | ❌ NO |
| **Cross-transport discovery** | Resilience (fallback) | P: RNS address sent as plaintext JSON; S: no auth | Encrypt RNS field in handshake (not done) | ❌ NO |
| **Handshake** | S: X25519 key exchange, signature verification | P: metadata (sender, recipient, timestamp visible); A: DID-PeerId link | Sealed sender (unwired) | ⚠️ PARTIAL — content encrypted, metadata not |
| **Encryption at rest** | S: user-keyed (passphrase) | S: state serialized without verified passphrase envelope; P: none | Verified passphrase envelope (not done) | ❌ NO |
| **mDNS announce** | Speed, zero-config | S: identity-announcing; P: broadcasts presence; A: none | Stealth mode toggle (not implemented) | ❌ NO |
| **libp2p gossipsub** | Speed, reliability | P: all peers receive encrypted payload; A: none | Sealed sender (unwired) | ⚠️ PARTIAL — content encrypted, metadata visible |
| **DTN bundles** | Resilience | P: destination hashes visible; S: no integrity verification beyond hash | Sealed sender + padding (not done) | ❌ NO |

### Compensating controls summary

**Exists:**
- Cross-transport discovery code (tries RNS first, falls back to libp2p) — but never triggered because rns_addr is always empty
- RNS receive pump — works, receives messages
- Handshake includes nickname field — works
- Encryption keys stored under both PeerId and identity ID — works
- Log redaction (truncate_id) — works
- PQC downgrade prevention (signature covers ciphertext + pq_key) — works
- Dev-mode auth bypass (loopback in all builds, RFC1918 debug-only) — works

**Missing (no compensating control exists):**
- Sealed sender not wired (no AAD, replay, expiry)
- Sphinx only 2 hops (needs 3+)
- Anonymous hop IDs (uses real PeerId)
- RNS address not exchanged in handshake (always empty)
- BLE/WiFi transport not implemented (design docs only)
- Privacy tiers (L0-L3) not implemented
- Stealth mode for mDNS not implemented
- Cover traffic not implemented
- Transport switch log redaction not done
- At-rest passphrase envelope not implemented
- vodozemac module not wired (custom X25519+ChaCha used)

---

## 3. Claims vs Reality Audit

### README.md claims

| Claim | README says | Reality | Verdict |
|---|---|---|---|
| P2P mesh networking (libp2p) | ✅ Working | Implemented, working on both machines | ✅ Accurate |
| End-to-end encryption (XChaCha20-Poly1305) | ✅ Working | Implemented with X25519 key exchange | ✅ Accurate |
| Key exchange (Ed25519 + X25519) | ✅ Working | Implemented | ✅ Accurate |
| Forward secrecy (symmetric ratchet) | ✅ Working | Double ratchet module exists but barely wired (1 caller) | ⚠️ Overstated |
| Message delivery ACKs/NACKs | ✅ Working | delivery_status field added to Message + UI, backend not wired | ⚠️ Overstated |
| HLC timestamps + message ordering | ✅ Working | Implemented (oldest-first sort fixed) | ✅ Accurate |
| Peer health monitoring (ping/pong + RTT) | ✅ Working | Implemented | ✅ Accurate |
| Peer discovery (mDNS + DHT + peer exchange) | ✅ Working | mDNS works, DHT partially, peer exchange exists | ⚠️ Partially accurate |
| Decentralized bootstrap (seed nodes) | ✅ Working | Design exists, implementation partial | ⚠️ Overstated |
| Anomaly detection (Isolation Forest) | ✅ Working | Module exists, wired into Core (optional), model file may not exist | ⚠️ Overstated (model file missing) |
| AI routing (LightGBM + PRoPHET fallback) | ✅ Working | Modules exist, wired into Core, but ONNX model file missing; LightGBM not actually used | ⚠️ Overstated |
| Terminal UI (Ratatui) | ✅ Working | Exists | ✅ Accurate |
| Web API (Axum + JWT auth) | ✅ Working | Implemented (JWT via dev token, auth bypass in debug) | ✅ Accurate |
| Shamir secret recovery | ✅ Working | Wired from controllers, appears functional | ✅ Accurate |
| Post-quantum key encapsulation (ML-KEM) | ✅ Working | ML-KEM-768 IS integrated in encryption.rs | ✅ Accurate (surprising) |
| X25519 small-order point defense | ✅ Working | Implemented | ✅ Accurate |
| Two-machine integration test | [ ] | Not done | ✅ Accurate (correctly unchecked) |
| Wire mesh event handlers to topology map | [ ] | Partially done | ✅ Accurate |
| DTN epidemic routing integration | [ ] | Not done | ✅ Accurate |
| Sphinx mixnet wiring | [ ] | Not done | ✅ Accurate |
| Full Double Ratchet with DH steps | [ ] | Not done | ✅ Accurate |

### ARCHITECTURE.md claims

| Claim | ARCHITECTURE says | Reality | Verdict |
|---|---|---|---|
| Transport: 5/10 partial | libp2p + Reticulum work; Obfs4 not implemented; Iroh/BT not in tree | Accurate | ✅ Accurate |
| Reticulum: initialized, standby, doesn't carry traffic | Accurate | Accurate | ✅ Accurate |
| Bluetooth BLE: NOT IMPLEMENTED | Accurate | BluetoothTransport exists as stub (scanning only, no data transfer wired) | ⚠️ Slightly understated (stub exists) |
| Sphinx: fixed 2252B packets | Accurate | Module exists, 2-hop min | ✅ Accurate |

### GHOSTWIRE_TRUTH_LEDGER.md claims

| Claim | Truth ledger says | Reality | Verdict |
|---|---|---|---|
| RNS: initialized, not primary, libp2p carries all traffic | Accurate | Accurate (confirmed by code: rns_addr always empty) | ✅ Accurate |
| PQ: hybrid (X25519 + ML-KEM-768) in code, unverified | Accurate | ML-KEM-768 IS in encryption.rs | ✅ Accurate |
| Sphinx: prototype/limited integration | Accurate | 2-hop min, real PeerId hop tokens | ✅ Accurate |
| "No logs/no traces": False as absolute claim | Accurate | System has logs, metrics, persistence | ✅ Accurate |
| Binary integrity: disabled unless configured | Accurate | Hash hooks exist but key config unset | ✅ Accurate |
| Signed updates: not release-ready | Accurate | Zero placeholder key | ✅ Accurate |
| Nine live transports: overstated | Accurate | Only libp2p + RNS (standby) actually functional | ✅ Accurate |

**The truth ledger is the most honest document. It correctly downgrades claims.**

### spec/README.md claims

| Claim | Spec says | Reality | Verdict |
|---|---|---|---|
| DRAFT protocol spec, version 1, unreleased | Accurate | spec/README.md exists with DID derivation, pairing codes, conformance vectors | ✅ Accurate |
| Conformance vectors (tests/conformance_vectors.rs + spec/vectors.json) | Exists | Files exist | ✅ Accurate |
| "No anonymity vs global passive observer" (honest non-goal) | Accurate | Correctly documented | ✅ Accurate |
| "DID can be linked to PeerId (correlation problem)" | Accurate | Known defect, documented | ✅ Accurate |
| Two-phase discovery (DiscoveryAnnounce public, AddressExchange authenticated) | Design | Implementation partial | ⚠️ Partially implemented |

---

## 4. Transport & Discovery Architecture Audit

### What's registered in TransportRegistry

| Transport | Registered? | Carries traffic? | Notes |
|---|---|---|---|
| `mesh` (libp2p) | ✅ Always | ✅ YES — all broadcast + DM | Primary and only functional transport |
| `reticulum` (RNS) | ✅ After unlock | ❌ NO — receive pump works, but send never triggered (rns_addr always empty) | Standby |
| `bluetooth` | ❌ NO | ❌ NO | BluetoothTransport exists but never registered |
| `simulated` | ❌ NO (not in default build) | ❌ NO | Test-only |
| WiFi | ❌ NO | ❌ NO | Design docs only |

### What the selector thinks is available

`eligible_selector_transports()` returns: `["mesh", "bluetooth", "reticulum"]` (if registered). But only `mesh` and `reticulum` are ever registered. Bluetooth is never registered.

### What the send path actually uses

`send_message()` → `self.transport.broadcast_message()` → libp2p gossipsub. Always. No transport selection. No RNS. No BLE. No WiFi.

Cross-transport discovery code exists in `mod.rs` (tries RNS first if peer has reticulum_address), but `reticulum_address` is never populated because `rns_addr = String::new()` in handshake.

### The gap

| What should happen | What actually happens |
|---|---|
| Handshake exchanges RNS address | `rns_addr = String::new()` — always empty |
| Topology stores RNS address | `node.reticulum_address = Some(addr)` — never set (addr always empty) |
| Cross-transport tries RNS first | Code exists but never triggers (no addresses) |
| RNS send_message called | Never called (no addresses to route to) |
| RNS receive pump works | ✅ Yes — receives and stores messages |
| BLE transport carries traffic | ❌ No — stub only, never registered |
| WiFi transport carries traffic | ❌ No — not implemented |

**Bottom line: The system is 100% libp2p. RNS, BLE, and WiFi are scaffolding. Cross-transport discovery is dead code.**

---

## 5. Dependency Chain Status

```
Layer 1 (Transport):     ❌ NOT DONE — libp2p only; RNS standby; BLE/WiFi absent
    ↓
Layer 2 (Routing):       ⚠️ PARTIAL — Sphinx exists (2-hop, unwired); DTN exists (unwired); RNS receive works
    ↓
Layer 3 (Privacy):       ❌ NOT DONE — sealed sender unwired; metadata leaks; no privacy tiers
    ↓
Layer 4 (Crypto):        ✅ PARTIAL — PQ actually integrated (surprising); vodozemac unwired
    ↓
Layer 5 (UX):            ⚠️ PARTIAL — broadcast/DM separation done; delivery_status field added; receipts not wired; unread/typing/search/attachments/offline queue not done
    ↓
Layer 6 (Quality):       ❌ NOT DONE — 724 unwraps, 11 panics, unbounded collections, transport_router_rns test fails
```

**Critical path:** Layer 1 must be completed before Layer 2 can be tested. Layer 2 must be completed before Layer 3. But wiring can happen in parallel.

---

## 6. What's Actually Working (Honest List)

### Functional (tested on two machines)
1. ✅ libp2p mesh (TCP+QUIC) — connects, discovers peers via mDNS
2. ✅ Peer naming — handshake includes nickname, both machines show correct names
3. ✅ E2E encrypted DMs — X25519 + AES-GCM, keys stored under both PeerId and identity ID
4. ✅ Broadcast messages — gossipsub BROADCAST_TOPIC, plaintext (by design)
5. ✅ Broadcast/DM separation — backend filters correctly, frontend filters correctly
6. ✅ Message ordering — oldest-first (fixed)
7. ✅ Profile creation/unlock — user-keyed (passphrase), machine key disabled
8. ✅ Dev-mode auth bypass — loopback in all builds, RFC1918 debug-only
9. ✅ Rate limiting — relaxed in dev (1000 req/s), strict in prod
10. ✅ RNS initialization — `init_reticulum()` called, receive pump running
11. ✅ RNS receive — pump receives and stores messages
12. ✅ Adaptive gossipsub mesh_n — (6, 1, 12, 2) for 2-peer networks
13. ✅ Cross-transport discovery code — exists, tries RNS first (but never triggered)
14. ✅ PowerMode enum — created, wired into Core
15. ✅ Mailbox purge — 12h expiry, wired into event loop
16. ✅ Log redaction — truncate_id helper, applied to 12+ log statements
17. ✅ PQC downgrade prevention — signature covers ciphertext + pq_key
18. ✅ ML-KEM-768 hybrid encryption — IS integrated in encryption.rs
19. ✅ Shamir secret recovery — wired from controllers
20. ✅ AI modules — wired into Core (gnn_router, onnx_router, anomaly_detector, feature_extractor)

### Partially working (exists but incomplete)
1. ⚠️ RNS transport — initialized, receives, but never sends (no addresses exchanged)
2. ⚠️ Cross-transport discovery — code exists, never triggered
3. ⚠️ Delivery receipts — field added + UI icons, backend status updates not wired
4. ⚠️ Store-and-forward — mailbox purge works, store_incoming not called
5. ⚠️ Double ratchet — module exists, barely wired (1 caller)
6. ⚠️ AI routing — modules wired, but model files (ONNX, JSON) may not exist
7. ⚠️ Anomaly detection — module wired, but model file may not exist

### Not working at all
1. ❌ BLE transport — stub only, never registered
2. ❌ WiFi transport — design docs only
3. ❌ Sealed sender — unwired
4. ❌ Sphinx routing — unwired (2-hop min)
5. ❌ Privacy tiers (L0-L3) — not implemented
6. ❌ Stealth mode (mDNS) — not implemented
7. ❌ Cover traffic — not implemented
8. ❌ Unread tracking — not implemented
9. ❌ Typing indicators — not implemented
10. ❌ Message search — not implemented
11. ❌ Attachments — not implemented
12. ❌ Offline queue — not implemented
13. ❌ Message reactions — not implemented
14. ❌ Read receipts (backend) — not wired
15. ❌ At-rest passphrase envelope — not implemented
16. ❌ vodozemac — unwired (custom AEAD used)
17. ❌ 15 modules — pure scaffolding

---

## 7. Key Findings (What the audit reveals)

### Finding 1: The system is 100% libp2p
Despite RNS being initialized and having a working receive pump, no traffic ever goes through RNS because handshake never exchanges RNS addresses (`rns_addr = String::new()`). Cross-transport discovery code is dead code. The system's actual transport is libp2p gossipsub, full stop.

### Finding 2: 15 modules are pure scaffolding
Out of 69 `pub mod` declarations in `src/core/mod.rs`, 15 have zero external callers. These are not "partially implemented" — they are completely unwired. They represent design decisions, not working code.

### Finding 3: The truth ledger is the most honest document
The truth ledger correctly downgrades every claim. It says RNS is standby (correct), Sphinx is prototype (correct), PQ is implemented in code but unverified (correct — ML-KEM-768 IS in encryption.rs), "no logs/no traces" is false (correct). The truth ledger should be trusted over README and ARCHITECTURE.md.

### Finding 4: README overstates several features
"Forward secrecy (symmetric ratchet) ✅ Working" — double ratchet is barely wired (1 caller). "Message delivery ACKs/NACKs ✅ Working" — backend not wired. "Anomaly detection (Isolation Forest) ✅ Working" — model file may not exist. "AI routing (LightGBM + PRoPHET fallback) ✅ Working" — ONNX model file missing, LightGBM not actually used. "Decentralized bootstrap (seed nodes) ✅ Working" — implementation partial.

### Finding 5: The compensating controls pattern is not yet implemented
The user's insight about compensating controls (each feature's weakness covered by another's strength) is a design principle, not yet code. No sealed sender is wired. No Sphinx multi-hop is wired. No privacy tiers exist. The "armory" exists as modules but none are loaded.

### Finding 6: The scaffolding IS the design — but it's not software yet
The user's insight that "scaffolding is design" is correct for architecture. But the audit reveals that the distance between "design" and "software" is large. 15 modules have zero callers. 11 more have only 1 caller. The RNS receive pump works but send doesn't. Cross-transport discovery exists but is never triggered. The system is a well-designed prototype, not a working mesh.

### Finding 7: ML-KEM-768 is actually integrated (contrary to model doc)
The truth ledger and model doc said PQ was "future profile" but the audit confirms ML-KEM-768 IS integrated in encryption.rs with HKDF-SHA256 mixing and downgrade prevention. This is one of the few features that's more implemented than documented.

---

## 8. Recommended Priority Order (from dependency chain)

### Immediate (Layer 1 — Transport)
1. **Fix handshake to exchange RNS address** — `rns_addr` should be `self.reticulum_manager`'s local address, not `String::new()`
2. **Wire RNS into send path** — cross-transport discovery already exists, just needs addresses
3. **Verify RNS message delivery end-to-end** — send from Omen, receive on Elitebook via RNS
4. **Implement BLE transport** — KISS framing over GATT NUS, using design docs
5. **Implement WiFi transport** — TCP port 4242, HDLC framing, mDNS discovery

### Next (Layer 2 — Routing)
6. **Wire Sphinx multi-hop** — increase to 3+ hops, anonymous hop IDs
7. **Wire DTN store-and-forward** — call `store_incoming` on receive
8. **Wire sealed sender** — AAD binding, replay protection, expiry
9. **Wire transport_selector into send path** — consume active_transport

### Then (Layer 3 — Privacy)
10. **Implement privacy tiers (L0-L3)** — connect to transport selector
11. **Implement stealth mode for mDNS** — toggle to hide presence
12. **Implement cover traffic** — for traffic analysis resistance
13. **Redact transport switch logs** — prevent metadata leaks

### Then (Layer 4 — Crypto)
14. **Wire vodozemac** — replace custom X25519+ChaCha with proper Double Ratchet
15. **Implement at-rest passphrase envelope** — verified encryption

### Then (Layer 5 — UX)
16. **Wire delivery receipts backend** — update delivery_status on ACK
17. **Implement unread tracking** — per-conversation read/unread
18. **Implement typing indicators** — real-time presence
19. **Implement message search** — search within conversations
20. **Implement attachments** — file/image sharing
21. **Implement offline queue** — messages queued when peer offline

### Finally (Layer 6 — Quality)
22. **Replace 724 unwraps** — with proper error handling
23. **Bound collections** — prevent DoS
24. **Fix transport_router_rns test** — TransportKind::Quic variant
25. **Clippy -D warnings** — fix all warnings

---

## 9. The Bottom Line

GhostWire is a **well-designed prototype with 69 modules, 15 of which are completely unwired, and a transport layer that is 100% libp2p despite having RNS, BLE, and WiFi scaffolding.** The architecture is sound. The design docs are thorough. The truth ledger is honest. But the distance between "design" and "working software" is the entire project's remaining work.

The user's insights are correct:
- The scaffolding IS the design (but it's not software yet)
- Security/privacy/anonymity is a lens (but it's not applied to code yet)
- Compensating controls are the answer (but they're not wired yet)
- RNS should be primary (but it's not even carrying traffic)
- Resilience > speed (but the system only has one transport)
- The armory exists (but no weapons are loaded)

The next step is Layer 1: make RNS actually carry traffic by fixing the handshake to exchange RNS addresses. Everything else depends on this.