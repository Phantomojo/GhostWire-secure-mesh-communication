# Reddit r/rust — Project Showcase

## Post Title
Built a decentralized mesh communication platform in Rust — looking for contributors

## Post Body

Hey r/rust,

I've been building **GhostWire** — a peer-to-peer encrypted mesh communication
platform written entirely in Rust. It's designed for when traditional
infrastructure fails: disasters, remote areas, censorship, or network shutdowns.

## What it does

GhostWire lets devices communicate directly without any central servers. Messages
hop node-to-node through whatever path is available — WiFi Direct, Bluetooth LE,
LoRa radio, WebRTC, TCP/IP, Reticulum, or Briar. If one transport goes down, it
automatically falls back to another.

## Tech stack

- **Networking:** libp2p with Kademlia DHT and Gossipsub
- **Runtime:** Tokio async
- **HTTP API:** Axum
- **Encryption:** AES-256-GCM, Ed25519/X25519, post-quantum ML-KEM-768 (planned)
- **AI routing:** LightGBM anomaly detector (ONNX exported, 76.7μs inference) +
  GNN trained on real GuifiSants mesh data (7,931 samples, 63 nodes, 31 days)
- **DTN:** PRoPHET protocol for delay-tolerant networking
- **Sybil resistance:** Proof-of-work node admission + TrustStore reputation

## What I'm proud of

- The AI routing actually works — trained on real community mesh network data
  from Barcelona's GuifiSants, not simulated data
- 7 transport layers with automatic fallback
- The whole thing runs on a Raspberry Pi
- Messages queue and deliver when peers reconnect (delay-tolerant)

## What I need help with

I'm a solo dev (with a team of 7 students for GCD4F 2026) and could really use
help with:

- **libp2p networking** — improving peer discovery and relay traversal
- **GNN model training** — retraining with edge features for better routing
  accuracy (currently at 58.9%, targeting 85-95%)
- **Security audit** — the crypto is solid but we haven't had an independent
  third-party review yet
- **Frontend** — the dashboard is functional but could use a proper React UI

## Links

- GitHub: https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication
- Website: https://phantomojo.github.io/GhostWire-secure-mesh-communication/
- Docs: https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication/tree/main/docs

The repo has `good first issue` labels on beginner-friendly tasks and a detailed
CONTRIBUTING.md. Happy to answer any Rust questions!

## When to post
Tuesday-Thursday, 10:00 AM - 2:00 PM UTC
Check the weekly "What's everyone working on" thread as an alternative

## Tips
• r/rust prefers technical depth over marketing
• Be ready to discuss Rust-specific decisions (why Tokio over async-std, etc.)
• Share your Cargo.toml dependencies if asked
• Acknowledge what you don't know yet
