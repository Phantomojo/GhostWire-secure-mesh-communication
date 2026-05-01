# GhostWire Master Todo List
# This file persists across sessions. Update it whenever todos change.
# Format: [STATUS] ID: Description
# Statuses: [ ] pending, [x] completed, [-] in_progress, [!] blocked

# ═══════════════════════════════════════════════════════════
# PHASE 1: Fix What's Broken (High Priority)
# ═══════════════════════════════════════════════════════════

[x] 1: BLE Encryption - Wrap data in XChaCha20-Poly1305 before BLE write
[x] 2: BLE Central Mode - Implement connect_peer() with adapter.connect_device()
[x] 3: BLE Transport Registration - Register in TransportRegistry for message flow
[-] 4: Build TUI - Fix API endpoints, auth token, message loading, contact selection
[x] 4a: TUI - Fix message sending endpoint (/api/v2/messages/send)
[x] 4b: TUI - Fix peer list (wire PeerController to actual peer data)
[x] 4c: TUI - Multi-perspective analysis complete - 10 perspectives analyzed
[x] 4d: TUI - Fix Unicode cursor handling (panic on non-ASCII input)
[x] 4e: TUI - Add conversation auto-scroll
[x] 4f: TUI - Add disaster level indicator
[ ] 4g: TUI - Add security dashboard view
[ ] 4h: TUI - Add per-peer latency display
[ ] 4i: TUI - Add key fingerprint display
[x] 4j: TUI - Add offline mode indicator

# ═══════════════════════════════════════════════════════════
# PHASE 2: Stability & Testing (Medium Priority)
# ═══════════════════════════════════════════════════════════

[ ] 5: 24-Hour Stability Test - Monitor memory, DB size, connections overnight
[ ] 6: 3-Node Mesh Test - Test DHT scaling, multi-hop routing, broadcast
[ ] 7: Security Penetration Test - Brute force, replay, MITM, BLE eavesdrop
[ ] 8: Network Partition Test - Split-brain behavior, reconnection

# ═══════════════════════════════════════════════════════════
# PHASE 3: AI Enhancement + Extras (Lower Priority)
# ═══════════════════════════════════════════════════════════

[ ] 9: Train ONNX Routing Model - Run train_l1_guifisants.py, export to ONNX
[ ] 10: Fix Feature Scaler - Debug format mismatch, retrain or convert
[ ] 11: Install Ollama + Gemma 4 - Install Ollama, pull gemma4:e2b, verify L2
[ ] 12: WiFi Direct Implementation - wpa_supplicant D-Bus integration
[ ] 13: Bluetooth MITM Protection - Identity verification over BLE
[ ] 14: Matrix Bridge - Interoperability with Matrix network
[ ] 15: Meshtastic Bridge - LoRa integration
[ ] 16: Message Encryption in Transit - Fix first-message plaintext issue

# ═══════════════════════════════════════════════════════════
# COMPLETED (from previous sessions)
# ═══════════════════════════════════════════════════════════

[x] C1: Fix encryption key exchange race condition (4 bugs: handshake type, broadcast topic, mesh delay, message queue)
[x] C2: BLE GATT server full implementation (bluer crate, TX/RX characteristics, fragmentation, reassembly)
[x] C3: WiFi Direct module created (placeholder + complete implementation plan)
[x] C4: GNN model exported from PyTorch to ONNX (2.5 MB)
[x] C5: Anomaly detector models created and deployed (10 trees, 39 features)
[x] C6: FunctionGemma handlers wired (all 8 connected to real subsystems)
[x] C7: AI config toggle added (ai_enabled flag in FeatureFlags)
[x] C8: LiteRT-LM dead code cleaned up (714 → 188 lines)
[x] C9: Rate limiter fixed (60→200 req/s, 30→500 burst)
[x] C10: Test suite: 48/48 passing (100%)
[x] C11: Identity auto-unlock via keyfile
[x] C12: Peer list API wired to actual connected peers
[x] C13: Recovery controller wired (send shares, list shares, reconstruct key)
[x] C14: Network controller wired (transport mode, x25519 pub key, contact card signature)
[x] C15: V2 routes wired (ws_count, sybil_rejections)
[x] C16: TUI daemon client wired (unread_count, get_messages)
[x] C17: Core mesh topology wired (get_mesh_topology)
[x] C18: DTN epidemic cids_we_need wired

# ═══════════════════════════════════════════════════════════
# SCORECARD (updated each session)
# ═══════════════════════════════════════════════════════════
# Overall: 8.4/10 (up from 8.2/10)
# Tests: 48/48 (100%)
# AI: L1 ✅ loaded, L2 ⚠️ missing, L3 ✅ loaded, L4 ✅ fallback
# BLE: GATT server ✅, central mode ✅, encryption ✅, registration ✅
# BLE API: /status ✅, /connect ✅, /send ❌ (axum Handler Send bound issue)
# WiFi Direct: module ✅, implementation ❌
# Web UI: ❌ TS errors block build (12+ errors)
# Last updated: 2026-04-09
