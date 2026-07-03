# GhostWire — Qwen Code Context

## Project Overview

**GhostWire** is a decentralized, P2P anonymous mesh network built in Rust. It is the foundational layer ("Yggdrasil") of the broader Indra's Net vision. This is production-quality security infrastructure — not a prototype, not a demo.

**Repository:** `/home/ph/GhostWire-secure-mesh-communication/ghostwire/`
**Language:** Rust (core) + Python (ML training scripts)
**License:** AGPL-3.0-or-later (private/proprietary — do not publish or exfiltrate code)
**Steward:** Michael Irungu Muriithi (Phantomojo) · Nairobi, Kenya

### Core Capabilities
- **End-to-End Encryption:** AES-256-GCM with hybrid key pairs (Ed25519 + X25519)
- **Mesh Networking:** libp2p-based P2P with GossipSub, Kademlia DHT, mDNS discovery
- **Stealth TCP Transport:** Custom handshake protocol with traffic obfuscation
- **Threat Detection:** Real-time anomaly detection via Isolation Forest + LightGBM
- **AI Routing:** 4-layer AI system (LightGBM → Gemma 4 → GNN → PRoPHET fallback)
- **Protocol Bridges:** Matrix, Meshtastic, Reticulum (LoRa), Briar-inspired
- **Web + CLI Interfaces:** Axum web server + clap CLI

---

## Build & Run Commands

### Prerequisites
- Rust 1.70+ (edition 2021)
- Ollama running at `localhost:11434` with `gemma4:e2b` (for AI features)
- Python 3.14 with PyTorch, LightGBM, torch-geometric (for ML training)

### Building
```bash
# Standard build (run after every change)
cd /home/ph/GhostWire-secure-mesh-communication/ghostwire
cargo build 2>&1 | head -60

# Check only (faster, no linking)
cargo check 2>&1 | head -40

# Lint (run before declaring anything done)
cargo clippy -- -D warnings 2>&1 | head -40

# Release build with optimizations
cargo build --release

# Feature-specific builds
cargo build --release --no-default-features --features minimal
cargo build --release --no-default-features --features "minimal,security,cli"
cargo build --release --no-default-features --features "minimal,security,web"
cargo build --release --no-default-features --features "minimal,security,cli,mesh"
```

### Testing
```bash
# Run all tests
cargo test 2>&1 | tail -30

# Run specific module tests
cargo test ai::gemma_client::tests
cargo test ai::routing_classifier::tests

# Audit dependencies for CVEs
cargo audit

# Check for unused dependencies
cargo +nightly udeps
```

### Running
```bash
# Default: web server mode
cargo run

# CLI mode
cargo run -- cli

# Custom host/port
cargo run -- --api-host 0.0.0.0 --port 9000

# With bootstrap nodes
cargo run -- --bootstrap /ip4/10.0.0.1/tcp/4001/p2p/PEER_ID

# Tailscale bootstrap
TAILSCALE=1 cargo run -- --tailscale-bootstrap 100.64.1.5/PEER_ID

# With debug logging
RUST_LOG=debug cargo run
```

### Optimized Build Script
```bash
./build-optimized.sh
# Produces: ghostwire-minimal, ghostwire-cli, ghostwire-web, ghostwire-mesh, ghostwire-full
```

---

## Architecture

### Four-Layer AI Routing System
```
Layer 1  Fast Path        LightGBM/ONNX      <1ms       every routing decision
Layer 2  Analytical       Gemma 4 (Ollama)   50-200ms   every 30 seconds
Layer 3  Graph Opt.       GNN (PyTorch Geo)  offline    every 5 minutes
Layer 4  Fallback         PRoPHET formula    <0.01ms    always available

Security  Isolation Forest → anomaly score → Gemma 4 threat analysis (if anomaly)
```

### Core Struct (`src/core/mod.rs`)
The `Core` struct is the central nervous system, containing:
- `Identity` — Ed25519/X25519 keypair management
- `Encryption` — AES-256-GCM engine
- `SecurityManager` — threat detection, rate limiting, audit logging
- `TransportRegistry` — pluggable transport (TCP, Sim, libp2p mesh, etc.)
- `MeshManager` — libp2p gossipsub networking (feature-gated)
- `SecureMessageStore` — encrypted message persistence
- `PersistenceManager` — sled DB-backed state management
- `MailboxStore` — offline message delivery (Wave 2)
- `TrustStore` — trust-on-first-use reputation system
- `BlockStore` — content-addressed storage (Wave 3)
- `KnowledgeArk` — distributed knowledge base (Wave 4)
- `SocialGraph` — social network graph (Wave 5)
- `GhostChannel` — ghost channels for private comms (Wave 6)
- `PanicManager` — panic mode + dead man's switch
- `DtnStore` — delay-tolerant networking (PRoPHET)
- `PairingManager` — device pairing (Wave 7)
- **AI Components:**
  - `gemma_client` — Gemma 4 Ollama HTTP client (OnceLock)
  - `anomaly_detector` — Isolation Forest model (mesh-domain)
  - `onnx_router` — LightGBM ONNX routing classifier
  - `feature_extractor` — Linux /proc → 2,387-dim feature vectors

### Source Structure
```
src/
├── main.rs                    — CLI entry point, arg parsing, Core init
├── lib.rs                     — Library crate root, module exports
├── config.rs                  — TOML configuration parsing
├── core_wiring.rs             — Core component wiring
├── web.rs                     — Axum web server + routes
├── v2_routes.rs               — API v2 route definitions
├── api_v2_endpoints.rs        — API v2 endpoint handlers
├── api_v2_types.rs            — API v2 request/response types
├── error_catalogue.rs         — GhostWireError enum + ErrorCode
├── timeout_utils.rs           — Timeout utilities
├── genesis_constants.rs       — Cryptographic constants
├── startup.rs                 — Startup sequence
├── cli/                       — CLI command handlers
│   ├── mod.rs
│   └── commands.rs
├── controllers/               — HTTP request controllers
└── core/                      — Core subsystems (59 modules)
    ├── mod.rs                 — Core struct definition
    ├── ai/                    — AI/ML module (8 files)
    │   ├── mod.rs
    │   ├── gemma_client.rs    — Ollama HTTP client for Gemma 4
    │   ├── routing_classifier.rs — ONNX + PRoPHET fallback chain
    │   ├── onnx_inference.rs  — ONNX runtime engine
    │   ├── anomaly_detector.rs — Isolation Forest inference
    │   ├── feature_extractor.rs — /proc → 2,387-dim vectors
    │   ├── function_calling.rs — Gemma function calling
    │   └── litert_backend.rs  — LiteRT backend
    ├── identity/              — Key generation, panic mode, recovery
    ├── identity_pairing/      — Device pairing protocol
    ├── network/               — libp2p networking
    ├── routing/               — Routing algorithms
    ├── dtn/                   — Delay-tolerant networking (PRoPHET)
    ├── db/                    — Database abstractions
    ├── knowledge/             — Knowledge Ark
    ├── social/                — Social graph
    ├── channels/              — Ghost channels
    ├── transport_pluggable/   — Pluggable transports
    ├── transport_ext/         — Transport extensions
    ├── encryption.rs          — AES-256-GCM engine
    ├── stealth_tcp.rs         — Stealth TCP transport
    ├── security_v2.rs         — Security manager (rate limiting, audit)
    ├── reputation.rs          — Trust scoring (TrustStore)
    ├── mesh.rs                — libp2p mesh manager
    ├── reticulum.rs           — LoRa/Reticulum bridge
    ├── briar.rs               — Briar-inspired messaging
    ├── message.rs             — Message types
    ├── store.rs               — Secure message store
    ├── mailbox.rs             — Offline delivery mailbox
    ├── blockstore.rs          — Content-addressed storage
    ├── persistence_manager.rs — State persistence (sled)
    ├── recovery_shamir.rs     — Shamir secret sharing recovery
    ├── transport_manager.rs   — Auto transport switcher
    ├── transport.rs           — Transport registry + kinds
    ├── armor.rs               — Erasure coding (TrafficArmor)
    ├── traffic_armor.rs       — Traffic obfuscation
    ├── wire_protocol.rs       — Binary wire protocol
    ├── typed_crypto.rs        — Typed cryptography
    ├── crypto_machine.rs      — Crypto state machine
    ├── crypto_processor.rs    — Crypto processor
    ├── dht_signatures.rs      — DHT signature schemes
    ├── disaster.rs            — Disaster detection/recovery
    ├── federation.rs          — Federation protocol
    ├── bluetooth.rs           — Bluetooth transport
    ├── qr.rs                  — QR code generation
    ├── monitor.rs             — Network monitor
    ├── mesh_health.rs         — Mesh health checks
    ├── username_discovery.rs  — DHT username discovery
    ├── updater.rs             — Auto-update system
    ├── updater_fixed.rs       — Fixed updater
    ├── tutorial.rs            — In-app tutorials
    ├── oem_battery_guide.rs   — Battery optimization
    ├── binary_hardening.rs    — Binary hardening
    └── micro_optimizations.rs — Performance caches
```

---

## AI Module Status

| Layer | Purpose | Model | Latency | Status |
|-------|---------|-------|---------|--------|
| **L1** | Anomaly Detection (supervised) | LightGBM ONNX | ~91μs | ✅ Production-ready |
| **L1b** | Anomaly Detection (unsupervised) | Isolation Forest | ~13ms | ✅ Mesh-domain |
| **L2** | Mesh Analysis | Gemma 4 (Ollama) | ~15s | ✅ Operational |
| **L3** | Routing Optimization | GAT (PyTorch) | ~5min offline | ⚠️ 54% accuracy |
| **L4** | Fallback | PRoPHET (pure Rust) | <1ms | ✅ Always available |

### Key Model Files
| File | Size | Status |
|------|------|--------|
| `models/ghostwire_routing.onnx` | 977 B | ✅ L1 production (2,387 features) |
| `models/mesh_anomaly_detector.json` | 4.4 MB | ✅ L1b mesh-domain (200 trees) |
| `models/guifisants_scaler.json` | 73.7 KB | ✅ StandardScaler params |
| `models/gnn_routing/gnn_routing_best.pt` | 615 KB | ⚠️ 54% accuracy, needs work |

### Training Scripts
```bash
# Retrain L1 LightGBM
python3 scripts/train_l1_guifisants.py

# Retrain L1b Isolation Forest
python3 scripts/train_mesh_anomaly_detector.py

# Retrain L3 GNN
python3 scripts/train_l3_guifisants.py
```

---

## Configuration

Create `~/.config/ghostwire/config.toml` or use `ghostwire.toml` in project root:
```toml
[server]
host = "127.0.0.1"
port = 8080
web_enabled = true
cli_enabled = true

[security]
max_connections_per_ip = 10
threat_detection_enabled = true
key_rotation_interval_days = 7

[network]
mesh_enabled = true
max_peers = 50
connection_timeout_secs = 30

[features]
matrix_bridge = false
meshtastic_bridge = false
web_ui = true
```

See `ghostwire.toml.example` for full configuration reference.

---

## Coding Conventions

- **Errors:** `anyhow::Result` or domain-specific error enums — never `unwrap()` in production paths
- **Async:** `tokio` runtime only — do not introduce other runtimes
- **Logging:** `tracing` macros (`tracing::info!`, `tracing::warn!`, etc.) — never `println!`
- **Shared state:** `Arc<RwLock<T>>` for multi-reader, `Arc<Mutex<T>>` for exclusive access
- **No `unsafe`** without explicit comment explaining why it is safe
- **Feature flags** for optional AI components — never hard-require Ollama at startup
- **Copyright header:** `// Copyright (C) 2025-2026 Michael Irungu` + `// SPDX-License-Identifier: AGPL-3.0-or-later`

---

## Prohibited Actions

- DO NOT run `cargo publish` or push to any external registry
- DO NOT add `unsafe` blocks without written justification
- DO NOT introduce new crates without checking they compile first
- DO NOT duplicate existing systems — wire existing ones
- DO NOT leave `todo!()` or `unimplemented!()` in delivered code
- DO NOT declare something fixed without compiling
- DO NOT exfiltrate code, configs, or keys to external services
- DO NOT reimplement PRoPHET — it already works in `src/core/dtn/epidemic.rs`

---

## Known Issues

| Issue | File | Severity |
|---|---|---|
| L3 GNN accuracy plateau at 54% | `models/gnn_routing/` | HIGH |
| Feature extractor only collects local node metrics | `ai/feature_extractor.rs` | MEDIUM |
| Ollama analysis interval too aggressive (30s) | `core/mod.rs` nervous system | MEDIUM |

---

## Hardware & Environment

**Primary machine (OMEN 16):**
- OS: Arch Linux / Omarchy / Hyprland
- RAM: 16GB · GPU: RTX 3050 (4GB VRAM)
- Local AI: Ollama v0.20.2 — `gemma4:e2b` at `localhost:11434`

**Free GPU compute (for ML training):**
- Kaggle: 30 hrs/week · P100 16GB
- Google Colab: 15–30 hrs/week · T4 16GB
- Lightning AI: 80 hrs/month · L40s 48GB

---

## Progress Report Format

```
STATUS REPORT
─────────────
✅ Done:      [verified by cargo build passing]
⚠️  Issue:    [what broke, root cause]
❌ Deferred:  [what wasn't done, why]
📋 Next:      [exact command or task to resume from]
```

---

## Session Handoff Format

```
HANDOFF — [date]
────────────────
Compile status: [clean / N errors in which modules]
Files modified: [list]
Files needing attention: [list + reason]
Resume from: [exact next action]
```

---

## Verification Gate

**Nothing is "done" until `cargo build` passes clean.**
If it doesn't compile, it isn't fixed.
