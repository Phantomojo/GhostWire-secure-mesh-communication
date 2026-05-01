# GhostWire — Dev.to Post Draft

**Title:** GhostWire: Decentralized Encrypted Mesh Networking with AI Routing — Built in Rust, Trained on Real Data, Made in Kenya

**Tags:** `#rust` `#opensource` `#ai` `#meshnetworking` `#privacy` `#decentralized` `#machinelearning` `#cybersecurity` `#africa` `#buildinpublic`

---

## The Problem

2.6 billion people lack reliable internet access. When disasters strike, cell towers fail, or communities are too remote for infrastructure — traditional communication breaks down. Precisely when coordination is most critical.

Existing solutions require centralized infrastructure, sacrifice privacy, or lack the resilience needed for adverse environments.

## The Solution

**GhostWire** is a peer-to-peer encrypted mesh communication platform that operates without any servers. No cloud. No ISPs. No cell towers. Just devices talking directly to each other, forming a self-organizing network that routes messages intelligently using machine learning trained on real-world mesh data.

### What Makes It Different

**AI-Powered Routing — Trained on Real Data, Not Simulations**

GhostWire is the first mesh platform to use machine learning trained on real-world community mesh data. Our models are trained on 7,931 samples from Barcelona's Guifi.net — one of the world's largest community mesh networks (63 nodes, 31 days of real mesh traffic).

The 4-layer AI system:

| Layer | Model | Purpose | Performance |
|-------|-------|---------|-------------|
| **L1** | LightGBM (ONNX) | Anomaly detection | AUC 1.0, 91μs inference |
| **L1b** | Isolation Forest | Unsupervised anomaly detection | ROC-AUC 0.918 |
| **L2** | Gemma 4 (Ollama) | Mesh topology analysis | ~15s structured output |
| **L3** | Graph Attention Network | Routing path optimization | 58.9% accuracy (improving) |
| **L4** | PRoPHET (Rust) | Delay-tolerant fallback | Always available |

**Production-Grade Security**

- XChaCha20-Poly1305 encryption with X25519 key exchange
- Forward secrecy via symmetric ratchet
- Shamir Secret Sharing for key recovery
- Isolation Forest anomaly detection for threat monitoring
- All encryption mandatory — no unencrypted mode

**Serverless by Design**

- Pure P2P via libp2p (GossipSub + Kademlia DHT + mDNS)
- No central point of failure
- No cloud dependency
- No internet required

### Current State

- **297,000+ lines of Rust** — 130 source files
- **37 Python scripts** — training, data processing, ONNX export
- **101 TypeScript/JS files** — web UI, build tooling
- **First cross-machine mesh ping** — April 5, 2026
- **Submitted to GCD4F 2026** — UNESCO IITE competition
- **NLnet NGI Zero application** — €15,000 grant pending
- **License:** AGPL-3.0-or-later (free forever)

### Quick Start

```bash
git clone https://github.com/Phantomojo/GhostWire-secure-mesh-communication.git
cd GhostWire-secure-mesh-communication/ghostwire
cargo run --release
```

The node starts on `http://127.0.0.1:8080`. Mesh networking activates with `--mesh-port`.

### Interactive TUI

```bash
cargo run --release -- tui
```

Terminal dashboard with peer list, message history, and compose — all in one screen.

### Why This Matters

Communication infrastructure in Africa is almost entirely built elsewhere — designed in California, London, Berlin, Beijing — for contexts that are not African contexts.

GhostWire is built in Nairobi, by someone who understands what it means to lack reliable connectivity. It is African-built infrastructure for African communities, designed for environments where power is intermittent, bandwidth is scarce, and users range from technical operators to community health workers with basic phones.

### What's Next

- **GNN routing improvements** — targeting 70-80% accuracy across diverse topologies
- **Delay-tolerant networking** — encrypted store-and-forward for offline message delivery
- **Android client** — Tauri-based proof-of-concept
- **Community deployment pilots** — real-world testing in Kenya

### Built in Public

Everything is open source, publicly auditable, and free. The core protocol will remain AGPL-3.0-or-later forever.

**Repository:** https://github.com/Phantomojo/GhostWire-secure-mesh-communication
**Website:** https://phantomojo.github.io/GhostWire-secure-mesh-communication/
**License:** AGPL-3.0-or-later

---

*Built in Nairobi, Kenya. 12 months of solo development. 297K lines of Rust. One mission: connecting the disconnected.*
