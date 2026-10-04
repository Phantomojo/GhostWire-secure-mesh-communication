<div align="center">

<img src="../docs/assets/ghostwire-pfp.png" alt="GhostWire Logo" width="150" />

# GhostWire

> **Decentralized encrypted mesh networking. Run it on any machine — no servers, no cloud, no middlemen.**

[![Rust](https://img.shields.io/badge/Rust-1.80+-orange.svg)](https://www.rust-lang.org/)
[![Version](https://img.shields.io/badge/version-0.2.0-blue.svg)](CHANGELOG.md)
[![License](https://img.shields.io/badge/License-AGPL--3.0-green.svg)](LICENSE)
[![Build](https://img.shields.io/badge/build-passing-brightgreen.svg)](https://github.com/Phantomojo/GhostWire/actions)

</div>

---

## What Is This?

GhostWire is a **peer-to-peer encrypted mesh network** built in Rust. Two nodes discover each other, exchange keys, and send encrypted messages — no central server, no API keys, no accounts.

**It works right now.** Not a prototype. Not a demo. Live A↔B encrypted messaging between real machines.

```
┌─────────────┐     encrypted      ┌─────────────┐
│   Omen      │ ◄────────────────► │  Stratos    │
│ 100.88.x.x  │   XChaCha20-Poly   │  100.105.x.x│
│  Node A     │                    │  Node B     │
└─────────────┘                    └─────────────┘
      ▲                                  ▲
      │          libp2p mesh             │
      └──────── GossipSub + Kademlia ────┘
```

## Quick Start

Three commands. That's it.

```bash
git clone https://github.com/Phantomojo/GhostWire.git
cd GhostWire/ghostwire
cargo run --release
```

The node starts on `http://127.0.0.1:8080`. API is live. Mesh networking activates when you specify `--mesh-port`.

### Interactive TUI

```bash
cargo run --release -- tui
```

Terminal interface with dashboard, peer list, message history, and compose — all in one screen.

| Key | Action |
|-----|--------|
| `Tab` | Switch tabs |
| `i` | Compose message |
| `q` | Quit |

## What Actually Works

| Feature | Status |
|---------|--------|
| P2P mesh networking (libp2p) | ✅ Working |
| End-to-end encryption (XChaCha20-Poly1305) | ✅ Working |
| Key exchange (Ed25519 + X25519) | ✅ Working |
| Forward secrecy (symmetric ratchet) | ✅ Working |
| Message delivery ACKs/NACKs | ✅ Working |
| HLC timestamps + message ordering | ✅ Working |
| Peer health monitoring (ping/pong + RTT) | ✅ Working |
| Peer discovery (mDNS + DHT + peer exchange) | ✅ Working |
| Decentralized bootstrap (seed nodes) | ✅ Working |
| Anomaly detection (Isolation Forest) | ✅ Working |
| AI routing (LightGBM + PRoPHET fallback) | ✅ Working |
| Terminal UI (Ratatui) | ✅ Working |
| Web API (Axum + JWT auth) | ✅ Working |
| ~~Stealth TCP transport~~ | **REMOVED** — sent message content unencrypted; never provided traffic morphing or DPI evasion. See `docs/REMOVED_STEALTHTCP.md` |
| Shamir secret recovery | ✅ Working |
| Post-quantum key encapsulation (ML-KEM) | ✅ Working |
| X25519 small-order point defense | ✅ Working |

## Architecture

### Four-Layer AI Routing

```
Layer 1  Fast Path        LightGBM/ONNX      <1ms
Layer 2  Analytical       Gemma 4 (Ollama)   ~15s
Layer 3  Graph Opt.       GNN (PyTorch)      offline
Layer 4  Fallback         PRoPHET formula    <1ms
```

### Security Stack

- **Encryption:** XChaCha20-Poly1305 with X25519 key exchange
- **Forward Secrecy:** Symmetric ratchet with chain key aliasing
- **Identity:** Ed25519 signing + X25519 encryption keypairs
- **Post-Quantum:** ML-KEM (Kyber) key encapsulation
- **Key Validation:** X25519 small-order point rejection (CVE-2025-62514 defense)
- **Recovery:** Shamir Secret Sharing with encrypted shares
- **Threat Detection:** Isolation Forest anomaly scoring
- **Dependency Hardening:** Pinned minimum safe versions (CVE-2024-58262, CVE-2023-42811, CVE-2025-0805)

### Core Structure

```
src/
├── core/
│   ├── ai/              — AI/ML (LightGBM, Gemma 4, GNN, PRoPHET)
│   ├── identity/        — Key generation, recovery, pairing
│   ├── network/         — libp2p mesh management
│   ├── dtn/             — Delay-tolerant networking (PRoPHET)
│   ├── encryption.rs    — XChaCha20-Poly1305 engine
│   ├── mesh.rs          — GossipSub + Kademlia + health monitoring
│   ├── peer_health.rs   — Ping/pong, RTT tracking, liveness
│   ├── stealth_tcp.rs   — Obfuscated TCP transport
│   ├── security_v2.rs   — Rate limiting, audit logging
│   ├── hlc.rs           — Hybrid Logical Clocks
│   └── wire_protocol.rs — Binary frame protocol
├── cli/                 — Command-line interface
├── tui.rs               — Terminal user interface
└── web.rs               — Axum HTTP API
```

## Configuration

```bash
# Start with API on port 8080 (default) and mesh on port 4001
ghostwire --mesh-port 4001

# Bind API to all interfaces
ghostwire --api-host 0.0.0.0 --mesh-port 4001
```

See [docs/CONFIGURATION.md](docs/CONFIGURATION.md) for all options.

## CLI Commands

```bash
# Start node with mesh networking
ghostwire --mesh-port 4001

# Interactive TUI
ghostwire tui

# Send a message
ghostwire send --recipient <peer-id> --message "hello"

# Check peers
ghostwire peers

# View stats
ghostwire stats
```

## Development

```bash
# Fast check (no linking)
cargo check

# Debug build
cargo build

# Release build (optimized)
cargo build --release

# Tests
cargo test

# Lint
cargo clippy -- -D warnings
```

**Incremental builds are enabled.** After the first build, only changed files recompile (~30s). `cargo build --release` is slow because it disables incremental compilation for maximum optimization.

## Documentation

| Document | Description |
|----------|-------------|
| [docs/ARCHITECTURE.md](docs/ARCHITECTURE.md) | System architecture and layer overview |
| [docs/CONFIGURATION.md](docs/CONFIGURATION.md) | Config files, CLI args, environment variables |
| [docs/INSTALL.md](docs/INSTALL.md) | Installation from source, binaries, Docker |
| [docs/DEPLOYMENT.md](docs/DEPLOYMENT.md) | Production deployment guide |
| [docs/TROUBLESHOOTING.md](docs/TROUBLESHOOTING.md) | Common issues and fixes |
| [docs/DESIGN_DECISIONS.md](docs/DESIGN_DECISIONS.md) | Why we chose X over Y |
| [docs/THREAT_MODEL.md](docs/THREAT_MODEL.md) | STRIDE threat model |
| [docs/KEY_MANAGEMENT.md](docs/KEY_MANAGEMENT.md) | Key lifecycle and management |
| [SECURITY.md](SECURITY.md) | Security policy and open findings |
| [CONTRIBUTING.md](CONTRIBUTING.md) | Contribution guidelines |

## Project Stats

- **120+ commits** since April 2025
- **0 dependencies on external services** — runs entirely offline
- **~35K lines of Rust** across 60+ source modules
- **203 tests** passing

## Build Status

**Last verified**: 2026-07-03 — `cargo check` passes, 0 errors, 48 warnings (all pre-existing).

Recent work: Security hardening (CVE patching, X25519 defense, dependency pinning), peer health monitoring (ping/pong), USBGuard device management.

## Roadmap

- [x] TUI backend integration (Phase 1 — 2026-06-24)
- [x] Compile error cleanup (Phase 2 — 2026-06-24)
- [x] Security hardening (CVE fixes, version floors, X25519 defense — 2026-07-03)
- [x] Peer health monitoring (ping/pong, RTT tracking — 2026-07-03)
- [ ] Two-machine integration test (Omen ↔ Stratos end-to-end)
- [ ] Wire mesh event handlers to topology map
- [ ] DTN epidemic routing integration
- [ ] Sphinx mixnet wiring
- [ ] Fix rapid message loss (gossipsub batching)
- [ ] Full Double Ratchet with DH steps
- [ ] Mobile clients (Android/iOS)
- [ ] Voice over mesh (VoIP)

## License

AGPL-3.0-or-later — free to use, modify, and distribute. If you run a modified version as a network service, you must make the source available.

## Built By

**Phantomojo** — Nairobi, Kenya

Decentralized. Encrypted. Unstoppable.
