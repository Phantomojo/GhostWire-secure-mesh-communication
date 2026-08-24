# GhostWire Protocol Specification

**Version:** 0.2.0-draft
**Status:** Living Specification
**Date:** April 12, 2026

---

## Overview

The GhostWire Protocol is an open, decentralized networking protocol for resilient peer-to-peer communication. It provides encrypted messaging, anonymous routing, delay-tolerant networking, and AI-powered path selection — operating without any central servers, cloud infrastructure, or trusted third parties.

GhostWire is designed to function in adversarial environments: censorship, network partition, infrastructure failure, and physical coercion.

---

## Protocol Layers

```
┌─────────────────────────────────────────────────────────┐
│  APPLICATION LAYER                                      │
│  Messaging  │  Knowledge Ark  │  Ghost Channels  │ Files │
├─────────────────────────────────────────────────────────┤
│  API LAYER                                              │
│  REST (Axum)  │  WebSocket  │  CLI  │  TUI              │
├─────────────────────────────────────────────────────────┤
│  PROTOCOL SERVICES                                      │
│  BlockStore/Bitswap │  Mailbox (offline) │ Social Graph  │
│  Sphinx Onion Router │  HLC (causal ordering)            │
│  Device Pairing │  Shamir Recovery │  Coercion Resistance│
├─────────────────────────────────────────────────────────┤
│  AI/ML ROUTING                                          │
│  L1: LightGBM → L2: Gemma 4 → L3: GNN → L4: PRoPHET    │
├─────────────────────────────────────────────────────────┤
│  CRYPTOGRAPHY                                           │
│  XChaCha20-Poly1305 │ X25519 │ Ed25519 │ ML-KEM-768    │
│  BLAKE3 │ Argon2 │ Forward Secrecy Ratchet              │
├─────────────────────────────────────────────────────────┤
│  PLUGGABLE TRANSPORTS                                   │
│  StealthTCP │ WebRTC │ BLE │ LoRa │ Reticulum │ obfs4   │
│  WebSocket Camouflage │ Shadowsocks AEAD │ TLS Camo     │
├─────────────────────────────────────────────────────────┤
│  MESH NETWORKING                                        │
│  libp2p GossipSub │ Kademlia DHT │ mDNS                 │
│  DTN (Store-Carry-Forward) │ PRoPHET Epidemic Routing   │
└─────────────────────────────────────────────────────────┘
```

---

## Layer Specifications

Each layer is specified independently. Implementations MAY support any subset of layers while remaining protocol-compatible.

| Spec | Document | Status |
|------|----------|--------|
| **Wire Format** | [`WIRE_FORMAT.md`](./WIRE_FORMAT.md) | Draft |
| **Cryptography** | [`CRYPTO.md`](./CRYPTO.md) | Draft |
| **Sphinx Routing** | [`SPHINX.md`](./SPHINX.md) | Draft |
| **Peer Discovery** | [`DISCOVERY.md`](./DISCOVERY.md) | Draft |
| **AI Routing** | [`ROUTING.md`](./ROUTING.md) | Draft |
| **Delay-Tolerant Networking** | [`DTN.md`](./DTN.md) | Draft |
| **Identity** | [`IDENTITY.md`](./IDENTITY.md) | Draft |
| **Social Protocol** | [`SOCIAL.md`](./SOCIAL.md) | Draft |
| **Pluggable Transports** | [`TRANSPORT.md`](./TRANSPORT.md) | Draft |

---

## Reference Implementation

The canonical reference implementation is written in Rust and maintained at:
- **Repository:** https://codeberg.org/PHANTOMOJO/GhostWire-secure-mesh-communication
- **Crate:** `ghostwire` (published to crates.io)
- **License:** AGPL-3.0-or-later (application) / MIT+Apache-2.0 (library crates)

### Extracted Libraries

| Crate | Description | crates.io |
|-------|-------------|-----------|
| `sphinx-rs` | Sphinx onion routing with SURBs | ✅ Published |
| `ghostwire-dtn` | Delay-tolerant networking (PRoPHET) | 🚧 Next |
| `hlc-rs` | Hybrid Logical Clocks | 🚧 Planned |
| `trust-store` | Web of Trust + TOFU | 🚧 Planned |

---

## Protocol Evolution

GhostWire uses **GhostWire Implementation Proposals (GIPs)** to evolve the protocol. See [`GOVERNANCE.md`](../GOVERNANCE.md) for the proposal process.

---

## Comparison to Other Protocols

| Feature | GhostWire | libp2p | Matrix | Nostr | I2P |
|---------|-----------|--------|--------|-------|-----|
| **P2P mesh networking** | ✅ | ✅ | ❌ (federated) | ❌ (relay-based) | ✅ |
| **Built-in encryption** | ✅ (XChaCha20-Poly1305) | ✅ (Noise) | ✅ (Olm/Megolm) | ✅ (NIP-44) | ✅ (Garlic) |
| **Anonymous routing** | ✅ (Sphinx onion) | ❌ | ❌ | ❌ | ✅ (Garlic) |
| **Delay-tolerant networking** | ✅ (PRoPHET) | ❌ | ❌ | ❌ | ❌ |
| **AI-powered routing** | ✅ (4-layer ML) | ❌ | ❌ | ❌ | ❌ |
| **Post-quantum crypto** | ✅ (ML-KEM-768) | ❌ | ❌ | ❌ | ❌ |
| **Multi-transport** | ✅ (TCP/WebRTC/BLE/LoRa) | ✅ (TCP/QUIC/WebRTC) | ❌ | ❌ | ✅ |
| **Offline messaging** | ✅ (Mailbox + DTN) | ❌ | ✅ (homeserver) | ❌ (relay) | ❌ |
| **Censorship resistance** | ✅ (4 pluggable transports) | ❌ | ❌ | ❌ | ✅ |
| **Coercion resistance** | ✅ (Duress PIN, panic wipe) | ❌ | ❌ | ❌ | ❌ |

---

## License

This specification is licensed under **CC-BY-4.0**. Implementations may be licensed independently.
