# Hacker News — Show HN Post

## URL to submit
https://github.com/Phantomojo/GhostWire-secure-mesh-communication

## Title (choose one)
Show HN: GhostWire – Decentralized mesh communication in Rust
Show HN: GhostWire – Encrypted P2P messaging that works without internet
Show HN: GhostWire – Serverless mesh networking with AI-powered routing

## Post Body (first comment)
GhostWire is a peer-to-peer encrypted communication platform that works without
any central servers. It's built for when traditional infrastructure fails —
disasters, remote areas, censorship, or network shutdowns.

Key technical details:
• Written in Rust (Tokio + Axum + libp2p)
• 7 transport layers: WiFi Direct, Bluetooth LE, LoRa, WebRTC, TCP/IP,
  Reticulum, Briar — auto-fallback when one is unavailable
• End-to-end encryption: AES-256-GCM + X25519 key exchange + post-quantum
  ML-KEM-768 ready
• AI-powered routing: LightGBM anomaly detector (AUC 1.0, 76.7μs inference,
  ONNX exported) + GNN trained on 7,931 real GuifiSants mesh samples
• Delay-tolerant: Messages queue when peers are offline, deliver on reconnect
• Sybil-resistant: Proof-of-work node admission + reputation system

The project is AGPL-3.0 licensed and part of our GCD4F 2026 competition
submission (Global Climate Data for Future — AI for Society track).

We're actively looking for contributors, especially in:
• Rust backend development (libp2p, networking)
• AI/ML model training (GNN routing optimization)
• Security auditing
• Frontend (React/TypeScript)

Live site: https://phantomojo.github.io/GhostWire-secure-mesh-communication/
GitHub: https://github.com/Phantomojo/GhostWire-secure-mesh-communication
Docs: https://github.com/Phantomojo/GhostWire-secure-mesh-communication/tree/main/docs

Happy to answer any questions about the architecture, crypto, or mesh networking!

## When to post
Tuesday-Thursday, 7:00-9:00 AM Pacific Time
(Avoid Monday mornings and Friday afternoons)

## Tips
• Be in the comments for the first 2 hours
• Answer every technical question honestly
• If someone points out a flaw, acknowledge it and explain your plan
• Don't upvote your own post
• Don't ask for upvotes
