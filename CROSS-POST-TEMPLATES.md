# Cross-Post Templates for GhostWire

## Hacker News

### Title Options (pick one):
1. "GhostWire: Open-source P2P mesh with post-quantum encryption and AI routing"
2. "Show HN: Decentralized mesh network that survives internet shutdowns"
3. "I built a P2P mesh that routes around censorship using 9 transports"

### Post Body:
```
GhostWire is an open-source P2P mesh networking protocol built in Rust. It provides encrypted messaging, delay-tolerant delivery, and AI-powered routing without any central servers.

Key features:
- Post-quantum ML-KEM-768 key exchange (NIST FIPS 205)
- Sphinx onion routing (same as Nym/Lightning)
- 9 transports: TCP, WebRTC, BLE, LoRa, Reticulum, obfs4, WebSocket, Shadowsocks, TLS
- 4-layer AI routing: LightGBM → Gemma 4 → GNN → PRoPHET
- Duress PIN with decoy profiles for coercion resistance
- Works offline via store-carry-forward (DTN)

Built by a team at Open University of Kenya, submitted to GCD4F 2026 (UNESCO) under "AI for Society."

GitHub: https://github.com/Phantomojo/GhostWire-secure-mesh-communication
Docs: https://www.ghostwire.cc/

Looking for feedback on the architecture and contributions.
```

### Subreddits:
- r/rust
- r/privacy
- r/netsec
- r/selfhosted
- r/P2P

---

## Reddit Posts

### r/rust
**Title:** "GhostWire: P2P mesh networking in Rust with post-quantum crypto and AI routing"

**Body:**
```
Hey r/rust! I've been building GhostWire — a decentralized mesh networking protocol in Rust. It uses libp2p for P2P, XChaCha20-Poly1305 for encryption, ML-KEM-768 for post-quantum key exchange, and a 4-layer AI routing stack.

The project started as part of GCD4F 2026 (UNESCO competition) and grew into a full protocol with 9 transports, Sphinx onion routing, and delay-tolerant networking.

Would love feedback from the Rust community on:
- The crypto stack (XChaCha20 + ML-KEM-768 hybrid)
- The libp2p integration patterns
- The AI routing pipeline (LightGBM → GNN → PRoPHET)

GitHub: https://github.com/Phantomojo/GhostWire-secure-mesh-communication
```

### r/privacy
**Title:** "GhostWire: P2P mesh that works without internet, with duress PIN and decoy profiles"

**Body:**
```
GhostWire is a decentralized P2P mesh that works across 9 transport layers including LoRa, BLE, and sneakernet. It's designed for when infrastructure fails — internet shutdowns, disasters, censorship.

Key privacy features:
- Sphinx onion routing (hides sender/receiver)
- Duress PIN that opens a decoy profile
- Post-quantum ML-KEM-768 (protects against future quantum attacks)
- No central servers, no logs, no metadata collection
- Delay-tolerant: messages deliver even when peers are offline

Built by a team in Nairobi, Kenya — where internet shutdowns during protests are historical fact.

GitHub: https://github.com/Phantomojo/GhostWire-secure-mesh-communication
```

### r/netsec
**Title:** "GhostWire: P2P mesh with post-quantum ML-KEM-768 and coercion resistance"

**Body:**
```
GhostWire is an open-source P2P mesh protocol with a focus on security:

**Crypto stack:**
- XChaCha20-Poly1305 AEAD for session encryption
- X25519 + ML-KEM-768 hybrid key exchange (NIST FIPS 205)
- Sphinx onion routing with SURBs
- Symmetric ratchet for forward secrecy

**Coercion resistance:**
- Duress PIN opens a cryptographically independent decoy profile
- Panic wipe destroys keys on command
- Dead man's switch for time-based key destruction

**Threat model:**
- Network surveillance: LOW (XChaCha20 + Sphinx)
- Active MitM: LOW (X25519 + Ed25519 + ML-KEM-768)
- Sybil attacks: MEDIUM (PoW + TrustStore)
- Physical seizure: HIGH (device seizure risk)

Internal audit complete (February 2026), A+ rating. Seeking third-party audit.

GitHub: https://github.com/Phantomojo/GhostWire-secure-mesh-communication
```

### r/selfhosted
**Title:** "GhostWire: Self-hosted P2P mesh that survives internet outages"

**Body:**
```
GhostWire is a self-hosted P2P mesh networking protocol. Run a node on any machine, connect to others directly — no central server required.

**Quick start:**
```bash
git clone https://github.com/Phantomojo/GhostWire-secure-mesh-communication.git
cd GhostWire-secure-mesh-communication/ghostwire
cargo run --release
# Node starts on http://127.0.0.1:8080
```

**Features:**
- 9 transports (TCP, WebRTC, BLE, LoRa, etc.)
- End-to-end encryption (XChaCha20-Poly1305)
- Delay-tolerant messaging (works when peers are offline)
- AI-powered routing
- Post-quantum ready (ML-KEM-768)

**Use cases:**
- Disaster response communication
- Privacy-focused messaging
- Off-grid mesh networks
- Censorship resistance

GitHub: https://github.com/Phantomojo/GhostWire-secure-mesh-communication
Docs: https://www.ghostwire.cc/
```
