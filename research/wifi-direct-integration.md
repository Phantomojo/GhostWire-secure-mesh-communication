# WiFi Direct (P2P) Transport Integration — Research Synthesis

**Date**: 2026-07-12  
**Repo**: `52f10b0fc8077304ef53c166347f4e997a636b56`  
**Author**: GhostWire Team  
**License**: AGPL-3.0-or-later (research notes are project-internal)

---

## Table of Contents

1. [Crate Recommendation](#1-crate-recommendation)
2. [API Surface](#2-api-surface)
3. [Architecture: How It Fits GhostWire](#3-architecture-how-it-fits-ghostwire)
4. [Integration Plan](#4-integration-plan)
5. [Platform Support](#5-platform-support)
6. [Dependency Impact](#6-dependency-impact)
7. [Estimated Effort](#7-estimated-effort)
8. [Risks & Mitigations](#8-risks--mitigations)

---

## 1. Crate Recommendation

**Winner**: `pim-wifidirect = "0.1.5"` — MIT license, actively maintained (4 releases in 12 days), ~3K SLoC.

### Why not the alternatives

| Crate | Verdict | Reason |
|-------|---------|--------|
| `wpa-ctrl 0.4.0` | ❌ Too low-level | Raw wpa_supplicant control interface — you'd need to reimplement P2P group negotiation, DHCP waiting, ARP polling, role assignment. pim-wifidirect already does all of this. |
| `wifi-ctrl` (2024) | ❌ Stale | Last updated 2024, no P2P-specific features. |
| DIY wpa_cli subprocess | ❌ Boilerplate | pim-wifidirect is exactly this, but tested and maintained. |

### Why pim-wifidirect wins

- **Complete P2P lifecycle**: discovery → connect → group formation → IP assignment → `SocketAddr` emission
- **Async/tokio native**: uses `tokio::process::Command` for wpa_cli, `mpsc` channels, `CancellationToken` — matches GhostWire's runtime exactly
- **Clean separation**: it's a *peer-finding layer*, not a transport — exactly what GhostWire needs
- **BSD/macOS fallback**: uses Bonjour DNS-SD on macOS (graceful degradation)
- **Active author**: Rfluid/Astervia, 4 revs in 12 days, responsive maintainer
- **No dependency conflicts**: pim-core's deps (blake3, bytes, serde, sha2, thiserror, toml, tracing) are all already in GhostWire or completely compatible

---

## 2. API Surface

### Core Types

```rust
// ── Discovery (main entry point) ──────────────────────────

pub struct WifiDirectDiscovery { /* ... */ }

impl WifiDirectDiscovery {
    /// Create. Returns (service, receiver) — receiver yields SocketAddr per formed group.
    pub fn new(
        node_name: impl Into<String>,
        config: WifiDirectConfig,
        listen_port: u16,
    ) -> (Self, mpsc::Receiver<SocketAddr>);

    /// Register per-peer event sink (optional, for cleanup tracking).
    pub fn set_peer_event_tx(&mut self, tx: mpsc::Sender<WfdPeerEvent>);

    /// Run the discovery loop until cancelled.
    pub async fn run(self, cancel: CancellationToken);
}

// ── Configuration (from pim-core) ─────────────────────────

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct WifiDirectConfig {
    pub enabled: bool,              // default: false (opt-in)
    pub interface: String,          // e.g. "wlan0"
    pub go_intent: u8,              // 0-15, default: 7 (neutral)
    pub listen_channel: u8,         // default: 6
    pub op_channel: u8,             // default: 6
    pub connect_method: String,     // "pbc" or "pin:<8-digit-pin>"
    pub peer_cleanup: PeerCleanupConfig,
}

// ── Low-level (available but not needed for integration) ──

pub struct WpaCliController { /* thin async wrapper around wpa_cli */ }
pub struct WifiDirectGroup { pub group_iface, pub role, pub own_ip, pub peer_ip }
pub enum GroupRole { Go, Gc }
pub const GO_INTERFACE_IP: Ipv4Addr = Ipv4Addr::new(192, 168, 49, 1);

// ── Error type ────────────────────────────────────────────

pub enum WifiDirectError {
    WpaCli(String),
    Bonjour(String),
    GroupFormation(String),
    Io(std::io::Error),
}
```

### Usage Pattern (from lib.rs + tests/basics.rs)

```rust
use pim_wifidirect::WifiDirectDiscovery;
use pim_core::WifiDirectConfig;
use tokio_util::sync::CancellationToken;

let config = WifiDirectConfig {
    interface: "wlan0".into(),
    connect_method: "pbc".into(),
    ..Default::default()
};

let (discovery, mut peer_rx) =
    WifiDirectDiscovery::new("ghostwire-node", config, 9100);

let cancel = CancellationToken::new();
tokio::spawn(discovery.run(cancel.clone()));

// peer_rx yields SocketAddr for every successfully formed P2P group
while let Some(addr) = peer_rx.recv().await {
    // addr = "192.168.49.x:9100"
    swarm.dial(addr.to_string().parse()?)?;
}
```

---

## 3. Architecture: How It Fits GhostWire

```
┌──────────────────────────────────────────────────┐
│                   GhostWire                       │
│                                                   │
│  ┌──────────────────────────────────────────┐     │
│  │           MeshTransport (mesh.rs)          │     │
│  │  ┌────────┐  ┌─────────┐  ┌──────────┐  │     │
│  │  │  TCP   │  │  Relay  │  │ WebRTC   │  │     │
│  │  │+Noise+ │  │ (NAT)   │  │ (browser)│  │     │
│  │  │ Yamux  │  │         │  │          │  │     │
│  │  └────────┘  └─────────┘  └──────────┘  │     │
│  └──────────────────────────────────────────┘     │
│                          ▲                        │
│                          │ dial via IP:port       │
│  ┌──────────────────────────────────────────┐     │
│  │   WfdPeerSource (NEW — wfd_transport.rs)  │     │
│  │   Spawns WifiDirectDiscovery::run()        │     │
│  │   Feeds SocketAddrs into connection mgr    │     │
│  └──────────────────────────────────────────┘     │
│                          ▲                        │
│                   p2p_find / p2p_connect           │
│                          │                        │
│  ┌──────────────────────────────────────────┐     │
│  │  wpa_supplicant (CONFIG_P2P=y)            │     │
│  │  Kernel: nl80211 + P2P-capable WiFi chip  │     │
│  └──────────────────────────────────────────┘     │
└──────────────────────────────────────────────────┘
```

**Key insight**: pim-wifidirect is NOT a transport — it's a **peer-discovery provider** that works at the same level as mDNS. mDNS discovers peers on the LAN; WiFi Direct discovers peers within radio range. Both produce `Multiaddr` values that feed into the same TCP-based transport stack.

---

## 4. Integration Plan

### Step 1: Add dependency

```toml
# ghostwire/Cargo.toml
[dependencies]
pim-wifidirect = "0.1.5"
# pim-core is pulled in transitively
```

### Step 2: Create `ghostwire/src/core/wfd_transport.rs`

A single new file (~100 lines). Responsibilities:

```rust
pub struct WfdPeerSource {
    peer_tx: mpsc::Sender<SocketAddr>,
    cancel: CancellationToken,
}

impl WfdPeerSource {
    /// Start WiFi Direct discovery. Returns a handle that can be cancelled.
    pub fn start(
        config: WifiDirectConfig,
        listen_port: u16,
        node_name: &str,
    ) -> (Self, mpsc::Receiver<SocketAddr>);

    /// Stop discovery.
    pub async fn shutdown(self);
}
```

The `mpsc::Receiver<SocketAddr>` is consumed wherever the swarm event loop lives — likely in `MeshTransport::run()`. On each received address, call `swarm.dial(addr)`.

### Step 3: Wire into `MeshTransport` (mesh.rs)

In `MeshTransport::new()` or the `MeshManager` constructor:

```rust
#[cfg(target_os = "linux")]
if config.wifi_direct.enabled {
    let (wfd_source, mut wfd_rx) = WfdPeerSource::start(
        config.wifi_direct.clone(),
        mesh_port,  // reuse mesh listen port
        "ghostwire-node",
    );
    // Store wfd_source for shutdown
    // Spawn receiver task:
    tokio::spawn(async move {
        while let Some(addr) = wfd_rx.recv().await {
            let multiaddr: Multiaddr = format!("/ip4/{}/tcp/{}",
                addr.ip(), addr.port()).parse().unwrap();
            if let Err(e) = swarm.dial(multiaddr) {
                tracing::warn!("WFD dial failed: {e}");
            }
        }
    });
}
```

### Step 4: `TransportKind` variant (optional)

If WiFi Direct needs to be exposed via the `TransportKind` dispatch in `transport.rs` for the message routing layer:

```rust
pub enum TransportKind {
    // ... existing variants
    #[cfg(target_os = "linux")]
    WiFiDirect(Arc<WfdPeerSource>),
}
```

This is *probably unnecessary* — WFD is a discovery mechanism, not a message transport. The `TransportKind` enum handles message sending; WFD just feeds addresses into the existing mesh transport.

### Step 5: Feature gate

```toml
[features]
default = []
wifi-direct = ["pim-wifidirect"]
```

Guard the module with `#[cfg(feature = "wifi-direct")]` and `#[cfg(target_os = "linux")]`.

---

## 5. Platform Support

| Platform | Status | Mechanism |
|----------|--------|-----------|
| **Linux** | ✅ Full | `wpa_supplicant` P2P via `wpa_cli` subprocess. Requires `CONFIG_P2P=y`, nl80211 kernel, P2P-capable chip (ath9k, mt76, iwlwifi with certain firmware). |
| **macOS** | ⚠️ Bonjour fallback | Uses `async-dnssd` for DNS-SD discovery. No actual WiFi Direct — same-device-subnet Bonjour only. Included in pim-wifidirect via `#[cfg(target_os = "macos")]`. |
| **Android** | 🔄 Phase B | lib.rs has a comment noting: "Phase B replaces the Linux wpa_cli backend with a JNI bridge to Java WifiP2pManager." Not implemented yet. |
| **Windows** | ❌ | No plan. WiFi Direct on Windows requires WinRT APIs. Not in pim-wifidirect's scope. |

**GhostWire's immediate use case**: Linux x86_64 (Omen 16, StratOS machines). This is where it matters for local mesh testing.

---

## 6. Dependency Impact

### Added to `Cargo.toml`

```toml
pim-wifidirect = "0.1.5"
```

### Pulled in transitively

```
pim-core v0.1.10
├── blake3 v1.x        ✓ already in GhostWire style (uses blake2b_simd)
├── bytes v1.x         ✓ already in GhostWire
├── serde v1.x         ✓ already in GhostWire
├── sha2 v0.10         ⚠️ new (tiny, pure Rust, widely used)
├── thiserror v2.x     ✓ already in GhostWire
├── toml v1.1          ✓ already in GhostWire (but v0.9 vs v1.1 — check compat)
└── tracing v0.1       ✓ already in GhostWire
```

**Note**: pim-core uses `toml v1.1` while GhostWire uses `toml v0.9`. Cargo's resolver can handle both (they're separate incompatible versions). No build conflict, but worth noting for uniformity.

**Total added**: ~10-15 new crate compilations. Incremental build impact: negligible (~30s on first build).

---

## 7. Estimated Effort

| Task | Effort | Complexity |
|------|--------|------------|
| Add dependency & feature flag | 5 min | Trivial |
| `wfd_transport.rs` module | 1 hr | Low — standard task spawn pattern |
| Wire into `MeshTransport::new()` | 15 min | Low — add spawn + receiver |
| Wire shutdown via `CancellationToken` | 15 min | Low — already used pattern |
| Config plumbing (CLI args / config file) | 30 min | Medium — depends on config system |
| Linux test (two real machines) | 2 hr | Medium — hardware setup + debug |
| **Total** | **~4 hrs** | |

---

## 8. Risks & Mitigations

| Risk | Impact | Mitigation |
|------|--------|------------|
| `wpa_supplicant` not compiled with P2P | Silent failure | pim-wifidirect logs warn + returns early. Check `wpa_cli p2p_find` before enabling. |
| WiFi chip doesn't support P2P | Silent failure | Detect via `iw list` for `P2P` in supported interface modes. Add preflight check. |
| GO/GC negotiation fails | Lost discovery cycle | pim-wifidirect retries on next poll (2s interval). Logged at warn level. |
| Interface name not `wlan0` | Config error | Make `interface` configurable (already is in `WifiDirectConfig`). |
| pim-core toml version conflict | Build error | Cargo handles separate minor versions fine. Verify with `cargo tree`. |
| pim-wifidirect breaks on future update | Runtime regression | Pin to `"0.1"`. Audit on upgrade. |
| macOS Bonjour finds localhost | False positive | Bonjour fallback is opt-in and clearly documented. Only relevant for macOS dev. |

---

## Appendix: Comparison Table

| Criterion | pim-wifidirect 0.1.5 | wpa-ctrl 0.4.0 | DIY wpa_cli |
|-----------|---------------------|----------------|-------------|
| P2P discovery | ✅ Built-in | ❌ Raw only | ❌ You build it |
| Group negotiation | ✅ Built-in | ❌ Raw only | ❌ You build it |
| IP assignment | ✅ DHCP wait + ARP | ❌ | ❌ You build it |
| Async runtime | ✅ tokio | ✅ tokio | ✅ tokio |
| Error handling | ✅ thiserror | ✅ thiserror | ⚠️ You define it |
| Tests | ✅ Parsing + unit | ✅ Some | ⚠️ You write them |
| Active maintenance | ✅ 2026-05 | ⚠️ 2025-11 | ❌ You own it |
| License | MIT | MIT | Your code |
| SLoC savings | ~300 lines saved | ~200 lines saved | 0 (full DIY) |
| GhostWire compat | ✅ Full | ✅ Full | ✅ Full |
