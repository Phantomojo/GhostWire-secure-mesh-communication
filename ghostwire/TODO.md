# GhostWire — TODO

**Last Updated:** June 24, 2026
**Status:** Phase 1+2 complete. 25 compile errors resolved. `cargo build` passes. 203/203 tests pass.

---

## ✅ Completed

### 1. Key Exchange Protocol ✅
- Proper `FrameKind::KeyExchange` frames implemented
- Backwards compatibility with legacy handshakes
- No more `+enc:` hack in Identify agent version

### 2. Forward Secrecy (Symmetric Ratchet) ✅
- Symmetric ratchet initialized on key exchange
- Chain key derived from ECDH shared secret
- Ratchet aliased under both PeerId and identity UUID for reliable decryption

### 3. Delivery ACKs/NACKs ✅
- ACK frame sent on successful decryption
- NACK frame sent on decryption failure
- Frame types wired in mesh handler

### 4. Message Ordering (HLC) ✅
- Hybrid Logical Clocks implemented
- Messages timestamped with `{physical_time, logical_counter, node_id}`
- Message reorder buffer with 2-second window
- Causally correct ordering across nodes

### 5. Decentralized Bootstrap ✅
- `FrameKind::PeerExchange` for peer discovery
- Periodic peer exchange every 30 seconds
- Deduplication via `known_peers` HashMap
- Hardcoded seed nodes (Omen + Stratos)

### 6. Terminal User Interface (TUI) ✅
- Interactive TUI built with Ratatui
- 4 tabs: Dashboard, Peers, Messages, Compose
- Message input with recipient and message fields
- Real-time status bar and error display
- Run with: `ghostwire tui`

### 7. Phase 1: TUI & Backend Integration ✅ (2026-06-24)
- TUI `get_messages()` wired to `/api/v2/messages` endpoint
- Auth token support with `authenticated_get`/`authenticated_post`
- `peer_count` field name mismatch fixed with `active_peers` fallback
- PeerController returns real peer data from mesh
- `/api/send` removed from auth whitelist (requires authentication now)
- v2 send/broadcast routes uncommented and active
- Messages load on contact Enter key in TUI
- Structured `tracing::warn!` on all TUI daemon error paths
- `refresh_data()` concurrent via `tokio::join!`
- Live AI status via `get_ai_status_summary()`

### 8. Phase 2: Compile Error Cleanup ✅ (2026-06-24)
- 25 compile errors → 0 errors
- `rand` 0.9 → 0.8 (fixed 14 errors: rand + Distribution compat)
- BLE routes commented out, bluetooth gated with `#[cfg(feature = "bluetooth")]`
- `anyhow!` macro imported in `coercion.rs`
- `DtnBundle` import path fixed (`dtn::epidemic` → `dtn::bundle`)
- `ring::hkdf::Prk::expand` API updated for ring 0.17
- `publish_to_topic` replaced with logging stub (method doesn't exist on MeshManager)
- Missing `threshold` arg added to `recover_identity_secret`
- Build verified: 203/203 tests pass, 68 warnings (all pre-existing)

---

## 🔴 Critical (Must Do Before Public Release)

### 1. Fix Key Exchange Protocol
**Problem:** Encryption keys are smuggled in the libp2p Identify agent version string. It works but it's ugly and fragile.

**What needs to happen:**
- Design a proper key exchange message type (FrameKind::KeyExchange)
- Send encryption public keys as a dedicated frame on connection established
- Remove the `+enc:` hack from the Identify agent version
- Verify that both nodes can still exchange encrypted messages after the change

**Estimated effort:** 2-3 days  
**Files:** `src/core/mesh.rs`, `src/core/wire_protocol.rs`

### 2. Implement Forward Secrecy
**Problem:** Static X25519 keys mean that if a node's private key is compromised, all past messages can be decrypted.

**What needs to happen:**
- Implement the Double Ratchet protocol (or a simplified variant)
- Generate ephemeral X25519 keypairs per message batch
- Store ratchet state in the encryption manager
- Handle out-of-order message delivery (messages arrive before earlier ones)

**Estimated effort:** 1-2 weeks  
**Files:** `src/core/encryption.rs`, `src/core/message.rs`  
**Reference:** Signal Protocol specification, https://signal.org/docs/specifications/doubleratchet/

### 3. Build a Minimal Web UI
**Problem:** The API works but there's no frontend. Users need a way to interact with GhostWire without curl.

**What needs to happen:**
- Build a simple React/Vue chat interface
- Connect to the WebSocket API for real-time messages
- Show peer list, connection status, and message history
- Handle authentication (token input on first load)
- Keep it minimal — one page, no routing, no state management library

**Estimated effort:** 1 week  
**Files:** `webui/` (currently empty)  
**Tech:** React + Bootstrap (or vanilla JS if you want to keep it simple)

### 4. Wire MailboxStore to DHT for Offline Delivery
**Problem:** Messages for offline peers are sealed locally but not published to the DHT. If both nodes are offline simultaneously, messages don't queue.

**What needs to happen:**
- Publish sealed mailbox envelopes to Kademlia DHT under recipient's peer ID
- Implement periodic DHT polling to check for new messages
- Add message expiry (TTL) to prevent DHT bloat
- Handle DHT record conflicts (multiple nodes publishing for the same recipient)

**Estimated effort:** 3-5 days  
**Files:** `src/core/mailbox.rs`, `src/core/mesh.rs`, `src/core/mod.rs`

---

## 🟡 Important (Should Do)

### 5. Add Comprehensive Tests
**Problem:** 178 tests for 35K LOC is ~1 test per 196 lines. Signal has ~1 test per 10 lines.

**What needs to happen:**
- Integration tests for the full message flow (send → encrypt → transmit → decrypt → receive)
- Property-based tests for encryption (roundtrip, nonce uniqueness, key derivation)
- Fuzz testing for the wire protocol parser
- Mock tests for the mesh driver (simulate peer connections without libp2p)
- Test the key exchange protocol specifically

**Estimated effort:** 1-2 weeks  
**Files:** `tests/`, `src/core/*/tests.rs`

### 6. Implement Proper Signature Verification
**Problem:** Signature verification is currently skipped because the sender's identity ID doesn't match their libp2p PeerId. AEAD provides authentication, but signatures should still verify.

**What needs to happen:**
- Unify the key storage format so that the sender's identity ID maps to their Ed25519 public key
- Or: sign messages with the libp2p identity keypair instead of a separate keypair
- Re-enable signature verification in `decrypt_message()`

**Estimated effort:** 2-3 days  
**Files:** `src/core/encryption.rs`, `src/core/mesh.rs`

### 7. Add Message Ordering
**Problem:** Messages can arrive out of order due to multi-hop relay and gossipsub propagation. There's no mechanism to reorder them.

**What needs to happen:**
- Use the `sequence_num` field (already added to Message) for ordering
- Buffer out-of-order messages and deliver them in sequence
- Add a timeout for missing messages (don't wait forever)

**Estimated effort:** 2-3 days  
**Files:** `src/core/message.rs`, `src/core/mesh.rs`, `src/core/store.rs`

### 8. Add Delivery ACKs with Retry
**Problem:** Messages are fire-and-forget. If a message is lost in transit, the sender never knows.

**What needs to happen:**
- Send `FrameKind::Ack` on successful message receipt (already defined, partially wired)
- Track pending ACKs with a timeout
- Retry messages that don't receive an ACK within the timeout
- Implement exponential backoff for retries

**Estimated effort:** 3-5 days  
**Files:** `src/core/mesh.rs`, `src/core/wire_protocol.rs`, `src/core/mod.rs`

---

## 🟢 Nice to Have

### 9. WebRTC Transport for Browser Connectivity
**Problem:** WebRTC transport exists but uses alpha-quality libp2p-webrtc. Needs stabilization.

**Estimated effort:** 1 week  
**Files:** `src/core/mesh.rs`

### 10. LoRa/Reticulum Bridge
**Problem:** Protocol is defined but no hardware integration.

**Estimated effort:** 2-4 weeks (requires hardware)  
**Files:** `src/core/reticulum.rs`

### 11. Matrix Bridge
**Problem:** Feature flag exists but implementation is stub.

**Estimated effort:** 1 week  
**Files:** `src/core/matrix_bridge.rs`

### 12. Tor Transport
**Problem:** Not implemented yet. Would provide additional anonymity for nodes that need it.

**Estimated effort:** 1 week  
**Files:** New module `src/core/tor_transport.rs`

### 13. Mobile Apps (Android/iOS)
**Problem:** No mobile clients. Tauri could work for a cross-platform app.

**Estimated effort:** 2-4 weeks  
**Tech:** Tauri + React Native or Flutter

### 14. DNS-over-HTTPS Bootstrap
**Problem:** Bootstrap nodes are hardcoded. A DoH-based bootstrap would allow dynamic discovery.

**Estimated effort:** 2-3 days  
**Files:** `src/core/mesh.rs`

### 15. Performance Optimization
**Problem:** Binary is 37 MB. Could be smaller with more aggressive stripping and LTO.

**What needs to happen:**
- Enable full LTO (`lto = "fat"`)
- Enable `panic = "abort"` (already done)
- Strip debug symbols (already done)
- Consider `opt-level = "z"` for size optimization

**Estimated effort:** 1 day
**Files:** `Cargo.toml`

### 16. Fix Rapid Message Loss (Gossipsub Batching)
**Problem:** When sending multiple messages rapidly, gossipsub drops some messages because the mesh isn't fully formed or can't keep up.

**What needs to happen:**
- Implement message batching — queue messages and send in batches
- Add message acknowledgment with retry on timeout
- Implement delivery confirmation with exponential backoff
- Consider adding a reliable transport layer on top of gossipsub

**Estimated effort:** 1-2 weeks
**Files:** `src/core/mesh.rs`, `src/core/mod.rs`

---

## 📋 Done (Completed)

- [x] P2P mesh networking with libp2p
- [x] End-to-end encrypted messaging (XChaCha20-Poly1305)
- [x] Key exchange via dedicated FrameKind::KeyExchange
- [x] Peer discovery (mDNS + bootstrap + peer exchange)
- [x] Ping/health monitoring with RTT measurement
- [x] Symmetric ratchet for forward secrecy
- [x] Delivery ACKs/NACKs
- [x] Hybrid Logical Clocks for message ordering
- [x] Decentralized bootstrap with peer exchange
- [x] Terminal User Interface (TUI) with Ratatui
- [x] Web UI (minimal)
- [x] MailboxStore DHT publishing
- [x] HLC timestamp on all messages
- [x] Ratchet aliasing under identity UUID for reliable decryption
- [x] Identity system with auto-unlock
- [x] Stealth TCP transport
- [x] Transport switching (AutoTransportSwitcher)
- [x] Anomaly detection (Isolation Forest)
- [x] CLI interface
- [x] REST API with authentication
- [x] WebSocket auth (was CRITICAL vulnerability — fixed)
- [x] Hardcoded default password (was CRITICAL vulnerability — fixed)
- [x] encrypt() panic (was HIGH vulnerability — fixed)
- [x] Username path traversal (was MEDIUM vulnerability — fixed)
- [x] JWT algorithm restriction (was MEDIUM vulnerability — fixed)
- [x] Identity lock on restart (was MEDIUM vulnerability — fixed)
- [x] Sequence numbers on messages
- [x] ACK/NACK frame types defined
- [x] MailboxStore for offline message sealing
- [x] Live A↔B encrypted messaging test (Omen ↔ Stratos)
- [x] Phase 1: TUI & Backend Integration (11 fixes — 2026-06-24)
- [x] Phase 2: Compile Error Cleanup (25 errors → 0 — 2026-06-24)

---

## 🎯 Next Phase Priorities

Phase 1 (TUI fixes) and Phase 2 (compile errors) are done. Phase 3 is next:

1. **Wire mesh event handlers** — Populate topology map (KB1 from Phase 1)
2. **Update peer_count_cache** — On ConnectionEstablished/ConnectionClosed (KB2)
3. **Two-machine integration test** — Omen ↔ Stratos end-to-end
4. **DTN epidemic routing integration** — Wire PRoPHET formula into live mesh
5. **Sphinx mixnet wiring** — Anonymous routing layer
6. **P0 security** — EXPECTED_SCORING_HASH, key zeroization, secure wipe
7. **Build minimal web UI** (Critical #3) — Users need a way to interact.
8. **Wire mailbox to DHT** (Critical #4) — Offline delivery matters.
9. **Add tests** (Important #5) — Confidence in correctness.
10. **Forward secrecy** (Critical #2) — This is a security requirement, not a feature.
