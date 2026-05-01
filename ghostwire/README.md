<div align="center">

<img src="../docs/assets/ghostwire-pfp.png" alt="GhostWire Logo" width="150" />

# GhostWire

> **Decentralized encrypted mesh networking. Run it on any machine — no servers, no cloud, no middlemen.**

[![Rust](https://img.shields.io/badge/Rust-1.70+-orange.svg)](https://www.rust-lang.org/)
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
│ 100.88.x.x  │   XChaCha20-Poly   │ 100.105.x.x │
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
cd GhostWire
cargo run --release
```

The node starts on `http://127.0.0.1:8080`. API is live. Mesh networking activates when you specify `--mesh-port`.

### Interactive TUI

```bash
cargo run --release -- tui
```

Terminal interface with dashboard, peer list, message history, and compose — all in one screen.

| Key   | Action          |
| ----- | --------------- |
| `Tab` | Switch tabs     |
| `i`   | Compose message |
| `q`   | Quit            |

## What Actually Works

| Feature                                     | Status     |
| ------------------------------------------- | ---------- |
| P2P mesh networking (libp2p)                | ✅ Working |
| End-to-end encryption (XChaCha20-Poly1305)  | ✅ Working |
| Key exchange (Ed25519 + X25519)             | ✅ Working |
| Forward secrecy (vodozemac Double Ratchet)  | ✅ Working |
| Message delivery ACKs/NACKs                 | ✅ Working |
| HLC timestamps + message ordering           | ✅ Working |
| Peer discovery (mDNS + DHT + peer exchange) | ✅ Working |
| Decentralized bootstrap (seed nodes)        | ✅ Working |
| Anomaly detection (Isolation Forest)        | ✅ Working |
| AI routing (LightGBM + PRoPHET fallback)    | ✅ Working |
| Terminal UI (Ratatui)                       | ✅ Working |
| Web API (Axum)                              | ✅ Working |
| Stealth TCP transport                       | ✅ Working |
| Shamir secret recovery                      | ✅ Working |

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
- **Forward Secrecy:** vodozemac Olm/Megolm Double Ratchet (Signal protocol, Trail of Bits audited)
- **Identity:** Ed25519 signing + X25519 encryption keypairs
- **Recovery:** Shamir Secret Sharing with encrypted shares
- **Threat Detection:** Isolation Forest anomaly scoring

### Core Structure

```
src/
├── core/
│   ├── ai/              — AI/ML (LightGBM, Gemma 4, GNN, PRoPHET)
│   ├── identity/        — Key generation, recovery, pairing
│   ├── network/         — libp2p mesh management
│   ├── dtn/             — Delay-tolerant networking (PRoPHET)
│   ├── encryption.rs    — XChaCha20-Poly1305 engine
│   ├── mesh.rs          — GossipSub + Kademlia
│   ├── stealth_tcp.rs   — Obfuscated TCP transport
│   ├── security_v2.rs   — Rate limiting, audit logging
│   ├── hlc.rs           — Hybrid Logical Clocks
│   ├── crypto_machine.rs — vodozemac Olm/Megolm Double Ratchet
│   ├── crypto_processor.rs — Crypto I/O driver
│   ├── routing/         — Sphinx, SKademlia, Private Routing
│   └── wire_protocol.rs — Binary frame protocol
├── cli/                 — Command-line interface
├── tui/                 — Terminal user interface
└── web.rs               — Axum HTTP API
```

## Configuration

```toml
# ~/.config/ghostwire/config.toml
[server]
host = "127.0.0.1"
port = 8080
web_enabled = true

[network]
mesh_enabled = true
max_peers = 50
```

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

## Project Stats

- **120 commits** since April 2025
- **55 commits** in April 2026 alone (active development)
- **791 clones** in the last 14 days
- **0 dependencies on external services** — runs entirely offline

## Roadmap

- [ ] Fix rapid message loss (gossipsub batching)
- [ ] Full Double Ratchet with DH steps
- [ ] Real-time peer list in TUI
- [ ] Message history persistence in TUI
- [ ] Mobile clients (Android/iOS)
- [ ] Voice over mesh (VoIP)

## License

AGPL-3.0-or-later — free to use, modify, and distribute. If you run a modified version as a network service, you must make the source available.

## Built By

**Phantomojo** — Nairobi, Kenya

Decentralized. Encrypted. Unstoppable.
