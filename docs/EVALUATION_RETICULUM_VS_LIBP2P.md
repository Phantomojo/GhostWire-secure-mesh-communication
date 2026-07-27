# Evaluation: Reticulum vs libp2p for GhostWire

## Executive Summary

**Recommendation: Reticulum as complementary transport for LoRa/DTN scenarios, NOT as primary transport replacement.**

libp2p is the right primary transport for GhostWire — it provides mature NAT traversal, multiplexing, browser support, and a thriving ecosystem. Reticulum fills a specific gap: ultra-low-bandwidth LoRa hardware that libp2p cannot reach. The integration should be via the `ProtocolAdapter` trait, making Reticulum a bridge alongside Matrix, Meshtastic, and Briar — not a competing transport.

**Blocker: Reticulum's anti-AI license clause** prevents packaging in Debian/F-Droid. This may be a hard constraint for a censorship-circumvention tool targeting distributions hostile to such restrictions.

---

## 1. Protocol Layer Comparison

| Dimension | libp2p (GhostWire current) | Reticulum |
|-----------|---------------------------|-----------|
| **OSI Layer** | L4+ (Transport) | L3+ (Networking) |
| **Connection model** | Swarm-based, connection-oriented | Connectionless, packet-based |
| **Multiplexing** | Yamux (stream multiplexing) | None needed (single-channel) |
| **NAT traversal** | Circuit Relay, DCUtR, AutoNAT | None (designed for direct-link networks) |
| **Browser support** | Yes (WebRTC) | No |
| **Peer discovery** | Kademlia DHT, mDNS, Identify | Announce-based (path tables) |
| **Minimum bandwidth** | ~100 bps (TCP) | ~5 bps (LoRa) |
| **Max hops** | Unlimited (relay) | 128 |
| **Packet size** | MTU-dependent (64KB typical) | 250 bytes (LoRa MTU) |

### Verdict
libp2p excels at internet-scale networking with NAT traversal and browser support. Reticulum excels at constrained hardware with minimal bandwidth. They serve different niches — complementary, not competing.

---

## 2. Cryptography Comparison

| Property | GhostWire (libp2p + Sphinx) | Reticulum |
|----------|---------------------------|-----------|
| **Key exchange** | X25519 (Noise XX) | X25519 (BLAKE2b KDF) |
| **Signatures** | Ed25519 | Ed25519 |
| **Symmetric** | XChaCha20-Poly1305 | AES-256-CBC + HMAC-SHA256 |
| **KDF** | HKDF-SHA256 | HKDF (via BLAKE2b) |
| **Onion routing** | Sphinx (full mix-net) | Layered encrypt (2-hop only) |
| **Post-quantum** | ML-KEM-768 ✅ | None ❌ |
| **Forward secrecy** | Double Ratchet ✅ | Per-session keys (partial) |
| **Replay protection** | Nonce tracking ✅ | Not documented |

### Verdict
GhostWire's crypto is strictly superior — post-quantum, forward secrecy, proper replay protection. Reticulum's crypto is classical-only and weaker (AES-CBC instead of AEAD). **Any Reticulum integration must wrap Reticulum's transport in GhostWire's E2E encryption** — never trust Reticulum's crypto layer.

---

## 3. Routing Comparison

| Property | GhostWire (Sphinx + GossipSub) | Reticulum |
|----------|-------------------------------|-----------|
| **Algorithm** | Sphinx mix-net + GossipSub pub/sub | Announce-based path tables |
| **Anonymity** | Full mix-net ( unlinkable) | Metadata visible to intermediate nodes |
| **Path selection** | AI-enhanced (GNN router) | Announce propagation |
| **Latency** | Higher (mixing delays) | Lower (direct paths) |
| **Bandwidth overhead** | Higher (padding, SURBs) | Minimal |
| **Self-healing** | Kademlia + GossipSub | Re-announce on failure |

### Verdict
GhostWire's Sphinx routing provides strong anonymity but at higher latency and bandwidth cost. Reticulum's path tables are efficient but leak metadata. For LoRa scenarios where bandwidth is the binding constraint, Reticulum's lightweight routing may be the only viable option — but GhostWire should still layer Sphinx on top when bandwidth permits.

---

## 4. Hardware & LoRa Support

| Property | GhostWire | Reticulum |
|----------|-----------|-----------|
| **LoRa firmware** | Not implemented | RNode firmware (ESP32, nRF52) |
| **Supported boards** | None | 15+ (Heltec, TTGO, LILYGO, etc.) |
| **BLE** | Feature-gated, partial | Via RNode |
| **WiFi Direct** | Not implemented | Not implemented |
| **Range** | Internet-dependent | 1-10km (LoRa) |
| **Power efficiency** | Standard | Optimized for battery |

### Verdict
This is Reticulum's killer advantage. GhostWire has no LoRa support. Reticulum's RNode firmware is production-ready and supports 15+ boards. For the OTF proposal's hardware testing (ESP32, LoRa, smartphones), Reticulum is the only viable path to actual LoRa mesh networking.

---

## 5. License Analysis (Updated July 2026)

### 5.1 Python Reticulum License

The Reticulum License is a modified MIT with two extra conditions:

> 1. The Software shall not be used in any kind of system which includes amongst its functions the ability to purposefully do harm to human beings.
> 2. The Software shall not be used, directly or indirectly, in the creation of an artificial intelligence, machine learning or language model training dataset, including but not limited to any use that contributes to the training or development of such a model or algorithm.

**What it actually restricts** (per maintainer markqvist in GitHub discussions #790 and #1062):
- Training LLMs/ML models on Reticulum source code
- Corporations scraping code into proprietary training datasets

**What it does NOT restrict** (maintainer's own words):
- Using Reticulum as a library/dependency — linking is fine
- Sending any data over the network (AI-related or not)
- Using Copilot/Cursor to write your project
- Academic research, personal experimentation
- ML-based spam filters running on Reticulum nodes

**The real blocker is packaging, not usage:**
- **Debian**: License violates DFSG #6 (No Discrimination Against Fields of Endeavor) → `non-free` at best
- **F-Droid**: Won't accept it (Sideband already blocked)
- **NixOS**: May drop pre-built binaries

### 5.2 Rust Implementations — License Comparison

| Project | License | Anti-AI? | LoRa Support | Completeness | Risk |
|---|---|---|---|---|---|
| **Beechat/Reticulum-rs** | **MIT** | **No** | ❌ No Serial/KISS/RNode | v0.1.0, early | Needs LoRa ifaces built |
| **lelloman/rns-rs** | Reticulum License | **Yes** | ✅ Full (Serial/KISS/RNode/AX.25) | Most complete, 900+ tests | Anti-AI clause applies |
| **ratspeak/rsReticulum** | AGPL-3.0 | No (copyleft) | ✅ Full (Serial/KISS/RNode/BLE) | v1.0.1, active | AGPL forces open-source |
| **FreeTAKTeam/LXMF-rs** | EPL-2.0 | No | Partial | v0.9.5 | EPL-2.0 weak copyleft |
| **Python Reticulum** | Reticulum License | **Yes** | ✅ Full | Mature, reference | Anti-AI clause applies |

**Key finding**: Beechat's reticulum-rs is **plain MIT** — no anti-AI clause. It's a clean-room reimplementation, not a fork of the Python code. The Reticulum protocol itself is **public domain** (dedicated by markqvist on day one).

### 5.3 Workaround Paths

**Path A: Beechat/Reticulum-rs (MIT) — cleanest license**
- Use TCP/UDP transport now, implement RNode interface when mature
- Risk: v0.1.0, 52 open issues, LoRa not yet supported

**Path B: Python FFI (Reticulum License) — fastest to working LoRa**
- Call Python Reticulum via PyO3/maturin FFI for RNode firmware access
- Risk: Python dependency, same license issues for packaging

**Path C: Protocol-only reimplementation (MIT) — cleanest long-term**
- Reticulum protocol is public domain → reimplement under MIT using only the spec
- Risk: Significant engineering effort (months)

**Path D: Direct LoRa bypass — no Reticulum at all**
- Implement LoRa directly via ESP32 BLE/WiFi, skip Reticulum entirely
- Risk: Must build LoRa mesh stack from scratch

### 5.4 Verdict

**For GhostWire's Apache-2.0 distribution goals**, the Python Reticulum license is a packaging blocker but not a usage blocker. The cleanest path is:

1. **Short-term**: Use Beechat/Reticulum-rs (MIT) for TCP/UDP. Accept no LoRa yet.
2. **Medium-term**: Contribute RNode interface to Beechat's repo, or fork and add it.
3. **Long-term**: If Beechat matures, it becomes the standard Rust Reticulum. If not, a clean protocol reimplementation under MIT is viable.

**Avoid**: lelloman/rns-rs (anti-AI clause), ratspeak/rsReticulum (AGPL), Python FFI (dependency bloat unless LoRa is urgent).

---

## 6. Integration Architecture

### Current GhostWire Transport Stack

```
Core
├── TransportRegistry (HashMap<String, TransportKind>)
│   ├── Mesh (libp2p Swarm) ← PRIMARY
│   ├── Quic
│   ├── StealthTCP
│   ├── Bluetooth
│   └── Simulated
├── AdapterRegistry (HashMap<String, Box<dyn ProtocolAdapter>>)
│   ├── Matrix
│   ├── Meshtastic
│   └── Briar
├── AutoTransportSwitcher (DisasterManager → TransportType)
└── ReticulumManager ← STUB, not integrated
```

### Proposed Integration: Reticulum as ProtocolAdapter

```
Core
├── TransportRegistry (unchanged)
├── AdapterRegistry
│   ├── Matrix
│   ├── Meshtastic
│   ├── Briar
│   └── ReticulumAdapter ← NEW
├── AutoTransportSwitcher
│   └── DisasterPolicy: Reticulum → fallback when LoRa-only
└── ReticulumManager ← INITIALIZED when config.reticulum_enabled
```

**Why ProtocolAdapter, not TransportKind:**
1. ProtocolAdapter is designed for protocol bridges — Reticulum is a bridge, not a replacement
2. It gives Reticulum access to `send_message()`/`receive_message()` without modifying the transport stack
3. It integrates with the adapter registry's message dispatch path
4. It keeps libp2p as the primary transport (NAT traversal, browsers, internet scale)

---

## 7. Implementation Plan

### Phase 1: Reticulum Adapter (1-2 weeks)

**Goal:** Wire existing `ReticulumManager` into `AdapterRegistry` as a `ProtocolAdapter`.

**Files to modify:**
- `ghostwire/src/core/adapter_registry.rs` — Implement `ProtocolAdapter` for `ReticulumManager`
- `ghostwire/src/core/reticulum.rs` — Fix integration points (currently standalone)
- `ghostwire/src/core/mod.rs` — Initialize `reticulum_manager` when config enabled
- `config/ghostwire.toml.example` — Document `reticulum_enabled = true`

**Success criteria:**
- `reticulum_enabled = true` in config → ReticulumAdapter registered
- Messages can be sent/received via Reticulum transport
- Auto-switcher can recommend Reticulum when appropriate

### Phase 2: LoRa Hardware Integration (2-4 weeks)

**Goal:** Connect GhostWire to actual LoRa hardware via Reticulum's RNode firmware.

**Options:**
1. **Use reticulum-rs** (Rust, TCP/UDP only) — needs LoRa extension
2. **Use Python Reticulum** via FFI — bridges to RNode firmware
3. **Direct LoRa** via ESP32 BLE/WiFi — bypass Reticulum entirely

**Recommendation:** Option 2 (Python FFI) for fastest path to working LoRa. Option 3 (direct LoRa) for long-term independence from Reticulum's license.

**Files to create:**
- `ghostwire/src/core/transport_ext/reticulum_transport.rs` — RNode bridge
- `ghostwire/src/core/hardware/loRa.rs` — Direct LoRa (long-term)

### Phase 3: DTN Integration (1-2 weeks)

**Goal:** Use Reticulum's path tables for DTN delivery in LoRa scenarios.

**Rationale:** GhostWire's DTN (Spray-and-Wait + PRoPHET) works over libp2p. Reticulum's path tables could provide a complementary delivery mechanism for LoRa where libp2p's overhead is prohibitive.

**Integration point:** `ghostwire/src/core/dtn/epidemic.rs` — add Reticulum as a delivery backend alongside GossipSub.

### Phase 4: Auto-Switching (1 week)

**Goal:** DisasterManager recommends Reticulum when:
- Hardware is LoRa-capable (ESP32/nRF52 detected)
- Bandwidth < 1 kbps
- Peers are on same LoRa network
- Internet is unavailable (censorship scenario)

**Files to modify:**
- `ghostwire/src/core/transport_manager.rs` — Add Reticulum to disaster policy
- `ghostwire/src/core/transport.rs` — Add `Reticulum` to `TransportType` enum

---

## 8. Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Reticulum license blocks distribution | High | Critical | Use reticulum-rs (different license?) or direct LoRa |
| Reticulum's crypto too weak | Medium | High | Always wrap in GhostWire's E2E encryption |
| LoRa bandwidth too low for Sphinx | High | Medium | Use Reticulum routing when bandwidth < threshold, Sphinx when > threshold |
| reticulum-rs incomplete | High | Medium | Fall back to Python FFI or direct LoRa |
| Anti-AI clause interpreted broadly | Medium | High | Legal review before integration |

---

## 9. Decision Matrix

| Criterion | Weight | libp2p only | + Reticulum adapter | + Direct LoRa |
|-----------|--------|-------------|--------------------|--------------------|
| LoRa hardware support | 30% | 0/10 | 8/10 | 9/10 |
| License compatibility | 25% | 10/10 | 5/10 | 10/10 |
| Crypto strength | 20% | 10/10 | 8/10 | 10/10 |
| Implementation effort | 15% | 10/10 | 6/10 | 4/10 |
| Community/ecosystem | 10% | 10/10 | 7/10 | 5/10 |
| **Weighted score** | | **8.5/10** | **6.7/10** | **7.8/10** |

### Final Recommendation

**Short-term (OTF proposal):** Use Reticulum adapter for LoRa hardware demos. Accept the license limitation for now — it blocks packaging but not usage.

**Long-term (post-OTF):** Implement direct LoRa transport via ESP32 BLE/WiFi. This gives GhostWire LoRa capability without Reticulum's license baggage. The Reticulum adapter becomes a reference implementation for the direct LoRa transport.

**For the OTF proposal:** Frame Reticulum integration as "hardware validation" — proving GhostWire's architecture works on constrained devices. The license limitation is acceptable for a research/education project, but should be noted as a risk in the proposal.
