# GhostWire Transport Implementation Plan

**Date**: 2026-07-07
**Status**: Research Complete — Ready for Implementation

---

## Executive Summary

GhostWire currently has **one working transport** (libp2p mesh) and four stubs. This plan maps every transport to concrete Rust libraries, integration points, and effort estimates based on deep research across 37+ anonymous networks, 15+ Rust transport libraries, and the existing codebase.

**Key finding**: The Rust ecosystem has mature enough primitives for ALL target transports. No C FFI needed for WiFi Direct (use `pim-wifidirect`), BLE (use `blew`), or internet transports (use `quinn`, `craftls`, `shadowsocks-rs`).

---

## Current State (from codebase analysis)

| Transport | File | LOC | Status | What Works |
|-----------|------|-----|--------|------------|
| libp2p Mesh | `mesh.rs` | 1825 | ✅ **PRODUCTION** | GossipSub + Kademlia + mDNS + WebRTC |
| StealthTCP | `stealth_tcp.rs` | 504 | ⚠️ **MOCK** | HMAC handshake, rate limiting — send path is `Ok(())` |
| WebRTC | `transport_ext/webrtc_transport.rs` | 313 | ⚠️ **PARTIAL** | STUN probing, fragmenter — no actual send/receive |
| Pluggable | `transport_pluggable/pluggable_transport.rs` | 398 | ⚠️ **PARTIAL** | WebSocket upgrade + Shadowsocks AEAD — not wired to mesh |
| Bluetooth | `bluetooth.rs` | 63 | ❌ **SKELETON** | Scanning only, `send_ble_data` is no-op |
| WiFi Direct | (none) | 0 | ❌ **MISSING** | Does not exist |
| SimTransport | `sim_transport.rs` | 82 | ✅ **TEST** | UDP simulation for dev/test |

**Transport trait interface** (from `transport.rs`):
```rust
pub enum TransportKind {
    Mesh(Arc<RwLock<MeshManager>>),
    Dummy,
    StealthTCP(Arc<StealthTCPProvider>),
    Bluetooth(Arc<RwLock<BluetoothTransport>>),
    Simulated(Arc<RwLock<SimTransport>>),
}

// Each must implement:
//   send_message(&self, message: &Message) -> Result<()>
//   receive_message(&self) -> Result<Option<Message>>
//   name(&self) -> &'static str
//   description(&self) -> &'static str
```

**Integration point**: `TransportRegistry` in `transport.rs` — register transports at startup, dispatch by name.

---

## Transport 1: WiFi Direct

### Libraries to Use

| Library | Version | Purpose | LOC Estimate |
|---------|---------|---------|--------------|
| **`pim-wifidirect`** | 0.1.5 | WiFi Direct P2P via wpa_cli | ~400-600 |
| `tokio` | 1.51 | Async runtime | existing |
| `tokio-util` | 0.7 | CancellationToken | existing |

**Why `pim-wifidirect`**: Only working Rust WiFi Direct crate. Handles full P2P lifecycle: discovery → connect → group formation → IP resolution. Uses `wpa_cli` subprocess (simplest approach, works on any Linux with wpa_supplicant).

**What it provides**:
- `WpaCliController` — wraps `wpa_cli -i <iface> <cmd>`
- `WifiDirectDiscovery` — async discovery loop with peer polling
- `WifiDirectGroup` — P2P group state (GO/GC role, IP resolution)
- `CancellationToken` for clean shutdown

### Implementation Plan

```
ghostwire/src/core/transport_ext/wifi_direct_transport.rs (NEW, ~500 LOC)

1. WifiDirectTransport struct wrapping pim-wifidirect
2. Discovery phase: WifiDirectDiscovery::new() → peer_rx channel
3. Connection phase: on peer found, form P2P group
4. Data phase: TCP socket over P2P group interface
5. Integration: register in TransportKind::WiFiDirect variant
```

### What to Borrow

- **From PIM**: `pim-wifidirect` crate is the transport layer itself — use directly
- **From I2P**: SSU2's UDP-based transport design for fallback when TCP over P2P fails
- **From Meshtastic**: TUN interface approach for transparent routing

### Effort: **2-3 days** (integration, not new protocol)

---

## Transport 2: Bluetooth Low Energy

### Libraries to Use

| Library | Version | Purpose | LOC Estimate |
|---------|---------|---------|--------------|
| **`blew`** | latest | Central + Peripheral + L2CAP | ~400-600 |
| `bluetooth_mesh` | latest | SIG Mesh protocol (optional) | ~200 if needed |
| `tokio` | 1.51 | Async runtime | existing |

**Why `blew`**: Cross-platform (Linux/macOS/Android/iOS), supports both central AND peripheral simultaneously (required for mesh), L2CAP CoC for high-throughput data transport. Used by `iroh-ble-transport`.

**Alternative**: If SIG Mesh compliance needed, use `bluetooth_mesh` crate for protocol + `blew` for radio driver.

### Implementation Plan

```
ghostwire/src/core/bluetooth.rs (REWRITE, ~500 LOC)

1. Replace btleplug (client-only) with blew (central+peripheral)
2. L2CAP CoC for data transport (200-500 KB/s vs GATT's 20-50 KB/s)
3. GATT service for discovery/handshake (UUID: custom GhostWire service)
4. Advertising for peer discovery (manufacturer data = node ID + capabilities)
5. Routing: flood-based with TTL (like Poholos) or mesh routing table
6. Integration: register in TransportKind::Bluetooth variant
```

### What to Borrow

- **From Poholos**: Advertising-frame mesh design (no connections needed for discovery)
- **From peat-btle**: CRDT-based state sync for mesh topology
- **From Meshtastic**: GATT service design (ToRadio/FromRadio pattern)
- **From I2P**: Tunnel-based routing over BLE links

### Effort: **3-5 days** (BLE mesh protocol is non-trivial)

---

## Transport 3: Stealth TCP (Internet Direct)

### Libraries to Use

| Library | Version | Purpose | LOC Estimate |
|---------|---------|---------|--------------|
| **`craftls`** | latest | TLS with browser fingerprint parroting | ~200 wiring |
| `tokio::net` | existing | TCP sockets | existing |
| `shadowsocks-rs` | 1.24.0 | AEAD encryption (pluggable) | existing crate |
| `tungstenite` | 0.29 | WebSocket for WebTunnel | ~150 wiring |

**Why `craftls`**: Pure Rust, drops into `rustls` API, provides Chrome/Firefox/Safari ClientHello fingerprints. Defeats JA3/JA4 DPI fingerprinting. No C dependencies.

### Implementation Plan

```
ghostwire/src/core/stealth_tcp.rs (FIX, ~200 LOC changes)

1. Replace mock send_path with actual TCP + craftls TLS wrapping
2. Add browser fingerprint config (Chrome 108 default)
3. Wire HMAC handshake over real TLS connection
4. Add connection pooling (currently per-connection, no reuse)
5. Integration: already registered as TransportKind::StealthTCP
```

### What to Borrow

- **From signal-tlsd**: Inner/outer TLS pattern for domain fronting
- **From I2P NTCP2**: Noise protocol framework handshake (more secure than HMAC)
- **From Tor obfs4**: Probing resistance (send padding after handshake)
- **From craftls docs**: `.with_fingerprint(CHROME_108.builder())` API

### Effort: **1-2 days** (mostly wiring, not new protocol)

---

## Transport 4: WebRTC (Internet Direct)

### Libraries to Use

| Library | Version | Purpose | LOC Estimate |
|---------|---------|---------|--------------|
| `libp2p-webrtc` | existing | WebRTC via libp2p | ~300 wiring |
| `webrtc-rs` | latest | Alternative WebRTC stack | if libp2p-webrtc insufficient |

**Current state**: STUN probing works, fragmenter/reassembler works, but actual data channel send/receive not wired.

### Implementation Plan

```
ghostwire/src/core/transport_ext/webrtc_transport.rs (FIX, ~300 LOC)

1. Wire WebRTC data channel to mesh.rs event loop
2. Handle ICE candidate exchange via existing Kademlia DHT
3. Fragment/reassemble GhostWire frames over data channels
4. Add TURN relay fallback for symmetric NAT
5. Integration: already partially registered
```

### What to Borrow

- **From Tor Snowflake**: WebRTC as censorship-resistant transport (same approach)
- **From I2P SSU2**: UDP hole-punching for NAT traversal
- **From existing GhostWire**: The fragmenter/reassembler already works — just wire it

### Effort: **2-3 days** (plumbing, not new protocol)

---

## Transport 5: Pluggable Transport (Shadowsocks/WSS)

### Libraries to Use

| Library | Version | Purpose | LOC Estimate |
|---------|---------|---------|--------------|
| `shadowsocks-rs` | 1.24.0 | AEAD-encrypted proxy | existing crate |
| `tokio-tungstenite` | 0.29 | WebSocket transport | existing crate |

**Current state**: WebSocket upgrade + Shadowsocks AEAD encrypt/decrypt passes tests but not integrated into mesh.rs.

### Implementation Plan

```
ghostwire/src/core/transport_pluggable/pluggable_transport.rs (FIX, ~200 LOC)

1. Wire pluggable transport into mesh.rs send/receive path
2. Add Shadowsocks server mode (currently client-only)
3. Add WebSocket Camo mode (disguise as normal web traffic)
4. Integration: register as TransportKind::Pluggable variant
```

### What to Borrow

- **From I2P**: Plugin architecture for swappable transports
- **From Tor**: PT managed transport protocol (lyrebird pattern)
- **From shadowsocks-rs**: SIP003 plugin chaining

### Effort: **1-2 days** (integration, protocol already works)

---

## Translation Opportunities (C/Go → Rust)

### Already Done by Others

| Project | Source Lang | Rust Status | Can Borrow |
|---------|-------------|-------------|------------|
| **Arti** (Tor) | C → Rust | ✅ Complete | Transport patterns, error handling |
| **i2pd** | C++ | ❌ No Rust port | NTCP2/SSU2 protocol specs |
| **Emissary** | New in Rust | ✅ Early | I2P router architecture |
| **shadowsocks-rs** | C → Rust | ✅ Complete | AEAD crypto, relay logic |
| **Quinn** | Go QUIC → Rust | ✅ Complete | QUIC transport layer |
| **obfs4-rs** | Go → Rust | ⚠️ Alpha | Obfuscation protocol |

### Small Enough to Rewrite (<2000 LOC)

| Component | Source | LOC | Difficulty |
|-----------|--------|-----|------------|
| **I2P NTCP2 handshake** | Java/C++ | ~800 | Medium (Noise framework) |
| **I2P SSU2 session** | Java | ~1200 | Medium (UDP + crypto) |
| **Reticulum transport** | Python | ~1500 | Easy (Python→Rust) |
| **Poholos BLE mesh** | Rust | ~1000 | Already Rust |
| **obfs4 protocol** | Go | ~1500 | Medium (alpha exists) |

### Must Wrap via FFI (Too Complex to Rewrite)

| Component | Source | LOC | Why FFI |
|-----------|--------|-----|---------|
| **wpa_supplicant** | C | ~500K | P2P state machine, WPS, DHCP |
| **BlueZ** | C | ~200K | Full BLE stack |
| **OpenSSL/BoringSSL** | C | ~500K | Use craftls instead |
| **Xray/V2Ray** | Go | ~100K | REALITY/VMess protocols |

---

## Recommended Implementation Order

### Phase 1: Wire What Exists (Week 1)
1. **Fix StealthTCP send path** — 1-2 days, highest impact
2. **Wire WebRTC data channels** — 2-3 days, enables internet transport
3. **Integrate pluggable transport** — 1-2 days, enables censorship resistance

### Phase 2: New Transports (Week 2)
4. **WiFi Direct via pim-wifidirect** — 2-3 days, enables proximity mesh
5. **BLE via blew** — 3-5 days, enables offline mesh

### Phase 3: Harden (Week 3)
6. **Add craftls fingerprint parroting** — 1 day, defeats JA3/JA4
7. **Add Shadowsocks server mode** — 1 day, enables relay nodes
8. **Add WebTunnel mode** — 1 day, WebSocket camouflage

**Total estimated effort**: 12-18 days for all transports

---

## Dependencies to Add

```toml
# WiFi Direct
pim-wifidirect = "0.1"

# BLE (replace btleplug)
blew = "0.1"  # cross-platform central+peripheral+L2CAP

# TLS fingerprint parroting
craftls = "0.1"  # or whatever version is current

# Already in Cargo.toml (verify versions):
# libp2p = "0.56"
# tokio = "1.51"
# shadowsocks-rs = check version
# tungstenite = "0.29"
```

---

## Risk Assessment

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| `pim-wifidirect` unmaintained | Medium | Low | Fork, it's <1000 LOC |
| `blew` L2CAP unavailable on some platforms | High | Medium | GATT fallback (slower but works) |
| `craftls` lags behind upstream rustls | Medium | Low | Pin version, update manually |
| WiFi Direct requires root | High | Low | Document requirement, add capabilities |
| BLE range too short for mesh | Medium | Medium | Coded PHY for 300m+ (peat-btle approach) |

---

## Bonus: C/Go → Rust Translation Findings

### Reticulum — Use `rns-rs` Instead of Rewriting

**`rns-rs`** (lelloman/rns-rs) is a faithful Rust port of Reticulum with **900+ interop tests** validated against Python. Includes Direct Link NAT hole punching and WASM hooks.

**Action**: Replace `reticulum.rs` bridge (426 lines) with `rns-rs` as a library dependency. Saves ~3200 lines of Python logic you'd otherwise write.

### i2pd NTCP2/SSU2 — Extract Frame Format

Both use `Noise_XK_25519_ChaChaPoly_SHA256`. Extractable:
- NTCP2 frame format (~500 LOC): SipHash-obfuscated lengths + ChaCha20-Poly1305 blocks
- Noise XK handshake (~300 LOC): Use `snow` crate + i2pd-specific extensions
- SSU2 reliability layer (~400 LOC): ACK window + retransmit timer

**Total**: ~1200 LOC as `ghostwire-transport-i2p` crate.

### Arti Patterns to Steal

1. **Weak<T> for circular deps**: `TransportManager` ↔ `MeshManager` cycle → use `Weak<T>`
2. **Reactor per I/O direction**: Split `stealth_tcp.rs` into `ReadReactor`/`WriteReactor`
3. **Crate decomposition**: 24+ single-responsibility crates

### Meshtastic PhoneAPI — Extract Client Protocol

PhoneAPI is transport-agnostic (TCP/BLE/Serial/HTTP). Extract ~500 LOC state machine + protobuf definitions for device management.

### GNUnet MQ — Priority Envelopes

```rust
struct MqEnvelope {
    priority: Priority,  // Background/Default/High/Critical
    cork: bool,
    reliable: bool,
    body: Vec<u8>,
}
```

~50 LOC for prioritized DTN vs real-time traffic.

---

## Sources

- I2P ecosystem: 5 implementations, Kimwolf attack analysis, NTCP2/SSU2 specs
- Tor ecosystem: Arti v2.5.0, pluggable transports, Snowflake
- 37 alternative networks across 7 tiers (from PRIVACY_NETWORKS_RESEARCH.md)
- btleplug internals, blew, bluer, bluetooth_mesh crate analysis
- pim-wifidirect source code review
- craftls API documentation
- shadowsocks-rs feature analysis
- GhostWire codebase: all transport files read and analyzed
- C/Go translation research: i2pd NTCP2/SSU2, Arti architecture, rns-rs, Meshtastic PhoneAPI, GNUnet CADET/MQ
