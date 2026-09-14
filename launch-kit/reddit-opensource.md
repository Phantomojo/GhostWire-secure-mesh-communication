# Reddit r/opensource — Project Announcement

## Post Title
GhostWire: Decentralized mesh communication for when the internet goes dark (Rust, AGPL-3.0)

## Post Body

Hey r/opensource,

I'm building **GhostWire** — a peer-to-peer encrypted mesh communication
platform that works without any central servers. It's AGPL-3.0 licensed and
we're actively looking for contributors.

## What it solves

When disasters hit, cell towers fail. When governments censor, ISPs comply.
When communities are remote, nobody builds infrastructure. GhostWire exists
for all three scenarios.

## Technical highlights

- **Rust** (Tokio + Axum + libp2p)
- **7 transport layers** with automatic fallback (WiFi, Bluetooth, LoRa,
  WebRTC, TCP, Reticulum, Briar)
- **AI-powered routing** — LightGBM + GNN trained on real GuifiSants mesh data
- **End-to-end encryption** — AES-256-GCM, X25519, post-quantum ready
- **Delay-tolerant** — messages queue and deliver when peers reconnect
- **Runs on a Raspberry Pi**

## Looking for contributors

We need help with:
- libp2p networking improvements
- GNN model retraining (targeting 85-95% accuracy)
- Security audit
- React/TypeScript frontend
- Documentation

Good first issues are labeled on GitHub.

## Links
- GitHub: https://github.com/Phantomojo/GhostWire-secure-mesh-communication
- Website: https://phantomojo.github.io/GhostWire-secure-mesh-communication/
- Docs: https://github.com/Phantomojo/GhostWire-secure-mesh-communication/tree/main/docs

Happy to answer any questions!
