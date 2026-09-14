# Reddit Posts — GhostWire

## 1. r/selfhosted
**Title:** GhostWire — Decentralized mesh networking with on-device AI routing (Rust, no cloud)

**Body:**
Hey r/selfhosted,

I've been building GhostWire — a decentralized P2P mesh network in Rust that runs entirely on your own hardware. No cloud dependencies, no central servers.

What makes it different from other mesh projects:

**AI routing that actually works on edge hardware.** We trained a LightGBM model on real GuifiSants mesh data (7,931 nodes, 2,387 features) and exported it to ONNX. Inference runs in ~77μs on a Raspberry Pi. The AI doesn't replace the routing protocol — it augments PRoPHET (pure Rust fallback) with anomaly detection. If a node is degrading, the model flags it before it fails.

**4-layer decision chain:**
1. LightGBM ONNX (~77μs) — fast path anomaly detection
2. Gemma 4 via Ollama — advisory topology analysis
3. GNN (PyTorch) — offline graph optimization
4. PRoPHET — always-on fallback, zero dependencies

**Security-first design:**
- AES-256-GCM + Ed25519/X25519 hybrid encryption
- vodozemac (Signal Protocol) for double ratchet
- Sphinx onion routing (with real X25519 curve math, not hash-based)
- Shamir secret sharing for identity recovery
- Binary hardening, frame size caps, replay protection

**Protocol bridges:** Matrix, Meshtastic, Reticulum (LoRa)

It's AGPL-3.0 and compiles clean. We just achieved our first cross-machine ping between two nodes (~18ms latency).

Looking for feedback from people who've self-hosted mesh networks or run community networks. Where do our assumptions break? What would you need to actually deploy this?

Repo: https://github.com/Phantomojo/GhostWire-secure-mesh-communication

---

## 2. r/rust
**Title:** Building a decentralized mesh network in Rust — lessons from wiring 59 modules, fixing AEAD truncation, and training ONNX models

**Body:**
Hey r/rust,

I've been building GhostWire — a P2P anonymous mesh network in Rust. Wanted to share some technical lessons learned, because several of them were painful.

**The AEAD truncation bug that broke multi-hop routing:**
We were using ChaCha20Poly1305 for Sphinx header encryption. Each hop's AEAD decryption stripped 16 bytes of the inner layer, making multi-hop routing impossible. The fix was migrating to ChaCha20 stream cipher mode (XOR with keystream). The tests are still `#[ignore]`d because we need curve25519-dalek for proper ephemeral key blinding.

**Wiring 59 core modules without losing your mind:**
The `Core` struct has ~40 fields: Identity, Encryption, MeshManager, SecureMessageStore, TrustStore, BlockStore, KnowledgeArk, SocialGraph, GhostChannel, PanicManager, DtnStore, PairingManager, plus AI components. The trick was `core_wiring.rs` — a separate file that documents every dependency before it goes into `Core::new()`.

**ONNX inference in Rust that actually works:**
We trained LightGBM on real GuifiSants mesh data (7,931 samples × 2,387 features), exported to ONNX, and load it with the `ort` crate. ~77μs inference. The key insight: don't try to run a 2B-parameter model for routing decisions. Use a tiny model for the hot path, bigger models for advisory.

**Binary hardening:**
- `mlock()` for sensitive memory regions
- Compile-time signing key injection via `build.rs`
- Runtime key loading from env vars (no hardcoded zeros)
- Frame size caps (64KB) to prevent OOM attacks
- LRU dedup instead of HashSet + clear() to close replay windows

**Dependencies we're proud of:** libp2p 0.56, vodozemac (Signal Protocol), chacha20poly1305, ort (ONNX), sled (embedded DB), axum (web server).

**Dependencies that gave us grief:** jsonwebtoken version pinning, ort 2.0.0-rc.4 API changes, libp2p-webrtc alpha.

Happy to answer questions about any of the above. Code is AGPL-3.0.

---

## 3. r/opensource
**Title:** GhostWire: A decentralized mesh network that runs on Raspberry Pis with on-device AI routing

**Body:**
Hey r/opensource,

I'm building GhostWire — a decentralized P2P mesh network in Rust (AGPL-3.0). The goal: secure communication that works when the internet doesn't.

**Why this matters:**
Most mesh network projects simulate their AI routing models on synthetic data. We trained ours on real GuifiSants data from 7,931 actual mesh nodes in Catalonia. The model detects node degradation in ~77μs on a Raspberry Pi — fast enough to reroute before a node fails during an emergency.

**What it does:**
- End-to-end encrypted messaging (AES-256-GCM + Signal Protocol double ratchet)
- Self-healing mesh with libp2p gossipsub + Kademlia DHT
- AI routing that augments (not replaces) a working PRoPHET protocol
- Protocol bridges for Matrix, Meshtastic, and LoRa
- Shamir secret sharing for identity recovery
- Works entirely offline — no cloud dependencies

**Current status:**
- ✅ Compiles clean (cargo check passes)
- ✅ First cross-machine ping achieved (~18ms between two nodes)
- ✅ Direct P2P messaging with key exchange
- ✅ Channel-based encrypted group messaging
- ⚠️ Web UI needs frontend scaffolding
- ⚠️ Sphinx multi-hop needs curve25519-dalek integration

**Looking for:**
- Contributors (Rust, networking, cryptography)
- Testers with Raspberry Pis or Android devices
- Feedback from community network operators
- People who've deployed mesh networks in the real world

The code is at https://github.com/Phantomojo/GhostWire-secure-mesh-communication. No contribution is too small — docs, tests, bug reports, all welcome.

---

## 4. r/privacy
**Title:** Building an offline-capable encrypted mesh network — threat model and design decisions

**Body:**
Hey r/privacy,

I've been building GhostWire — a decentralized mesh network designed for privacy-first communication. I want to share our threat model and get feedback from this community, because privacy is hard to get right.

**Threat model:**
- Network surveillance (traffic analysis, packet inspection)
- Man-in-the-middle attacks
- Metadata analysis (timing, patterns)
- Identity correlation across sessions
- Node compromise (physical access to hardware)

**What we do:**
- **Encryption in transit AND at rest:** AES-256-GCM for messages, XChaCha20-Poly1305 for storage
- **Sphinx onion routing:** Each relay node only knows the previous and next hop. Real X25519 curve math for unlinkability (not hash-based fake rerandomization)
- **Ephemeral identities:** Temporary identities with automatic key rotation
- **No central servers:** libp2p P2P with mDNS local discovery + Kademlia DHT
- **Traffic analysis resistance:** Cover traffic frames during idle periods, random padding on all messages
- **Panic mode:** Dead man's switch that wipes identity keys on duress

**What we DON'T do (yet):**
- Full Sphinx multi-hop (AEAD truncation bug, working on it)
- Mix network timing (planned)
- Plausible deniability for stored messages (planned)

**Honest limitations:**
- Broadcast messages on the mesh topic are encrypted in transit (Noise protocol) but the content is readable by anyone on the topic
- Channel messages need member-only encryption (working on it)
- Instance admins in a federated setup could theoretically see metadata

**Questions for this community:**
1. Is our threat model missing anything critical?
2. For emergency communication, what privacy guarantees matter most?
3. Would you trust a mesh network you could audit (AGPL-3.0, all source visible)?

Code: https://github.com/Phantomojo/GhostWire-secure-mesh-communication

---

## 5. r/networking
**Title:** Real-world mesh routing: training AI models on GuifiSants data instead of synthetic topologies

**Body:**
Hey r/networking,

I'm working on GhostWire — a decentralized mesh network in Rust — and I want to share something that changed how I think about routing: training on real data instead of simulations.

Most mesh/AI routing papers use synthetic topologies. We used GuifiSants data from 7,931 real nodes in Catalonia's community network. The difference is stark.

**What real data taught us:**
- Node CPU load correlates with routing table instability (high CPU = flaky next-hop decisions)
- WiFi interface TX/RX rates are better predictors of link quality than RSSI alone
- A single feature split in the LightGBM model captured most anomaly signals — the real network has very strong degradation patterns
- Betweenness centrality as a node feature improved GNN accuracy more than any other single feature

**The routing architecture:**
- Layer 1: LightGBM ONNX (~77μs) — takes 2,387 features from /proc, outputs anomaly probability
- Layer 2: Gemma 4 (Ollama) — topology analysis, human-readable recommendations
- Layer 3: GAT (PyTorch Geometric) — graph optimization, runs offline
- Layer 4: PRoPHET — pure Rust, always available, no model dependency

**Protocol details:**
- libp2p gossipsub for message propagation
- Kademlia DHT for peer discovery
- mDNS for zero-config local discovery
- Bincode wire format (50-70% smaller than JSON)
- Frame size cap at 64KB (OOM prevention)
- Replay window (64-bit sliding window per sender)

**Question for network engineers:** In your experience, what routing metrics actually predict link failures before they happen? We're collecting features from /proc/stat, /proc/meminfo, /proc/net/dev — are we missing anything?

---

## Posting Strategy

**Order to post (spread over 2-3 days to avoid spam filters):**
1. Day 1: r/rust (most technical, your home turf)
2. Day 2: r/selfhosted + r/opensource
3. Day 3: r/privacy + r/networking

**Tips:**
- Don't post all at once — Reddit's spam filter will catch it
- Engage with comments within the first hour (boosts visibility)
- Cross-post the best-performing one to 1-2 more subs
- Reply to every comment, even critical ones
