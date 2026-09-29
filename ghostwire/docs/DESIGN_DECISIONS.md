# GhostWire Design Decisions

Why each choice was made, not just what was chosen.

---

## 1. libp2p (P2P Framework)

**Decision:** Build on libp2p v0.56 with Rust bindings.

**Alternatives considered:**
- Custom protocol over raw TCP/UDP
- Yggdrasil (written in Go, not embeddable in Rust)
- go-libp2p (Go, not Rust-native)

**Why libp2p:**
- libp2p started as Ethereum's network layer and has been battle-tested across IPFS, Filecoin, Polkadot, and dozens of production networks. The attack surface is well-understood.
- The Rust implementation (`libp2p` crate) gives native async/await integration with Tokio, zero-cost abstractions, and no FFI overhead.
- Built-in support for the exact primitives GhostWire needs: Noise encryption, mDNS local discovery, Kademlia DHT, Gossipsub pubsub, NAT traversal (AutoNAT, DCUtR), relay circuits, and peer identification.
- The pluggable transport architecture means GhostWire can run over TCP, QUIC, WebRTC, Bluetooth, or any custom transport without rewriting the networking stack.
- Feature flags (`tcp`, `noise`, `yamux`, `kad`, `gossipsub`, `mdns`, `identify`, `autonat`, `relay`, `dcutr`, `ping`, `request-response`, `cbor`) let you compile only what you need.

**What libp2p didn't cover (built custom):**
- Stealth TCP with traffic morphing (`stealth_tcp.rs`)
- Obfs4 DPI evasion (`obfs4_transport.rs`)
- DTN epidemic routing (`dtn/epidemic.rs`)
- Sphinx onion routing (`sphinx-rs` crate)
- AI-enhanced routing decisions

**Trade-off:** libp2p adds ~2MB to binary size and pulls in a large dependency tree. The alternative (custom protocol) would mean reimplementing Noise handshakes, multiplexing, DHT, and NAT traversal from scratch. That's months of work with a worse security surface.

---

## 2. XChaCha20-Poly1305 (Message Encryption)

**Decision:** XChaCha20-Poly1305 for symmetric encryption of messages and storage.

**Alternatives considered:**
- AES-256-GCM (hardware-accelerated on x86)
- ChaCha20-Poly1305 (96-bit nonce)
- AES-256-CBC + HMAC (legacy, non-AEAD)

**Why XChaCha20-Poly1305:**
- The 192-bit nonce (vs 96-bit for standard ChaCha20-Poly1305) makes nonce reuse astronomically unlikely, even with random nonce generation. For a mesh network where nodes generate thousands of messages without central coordination, this safety margin matters.
- No hardware AES acceleration on ARM Cortex-M devices, Raspberry Pi, older laptops, or many embedded boards that GhostWire targets. ChaCha20 performs consistently across all platforms.
- The `chacha20poly1305` crate (v0.10) provides a clean AEAD API with constant-time comparison built in.
- AES-GCM is also present in Cargo.toml (`aes-gcm >= 0.10.3`) for compatibility with legacy key material and specific CVE-patched requirements, but XChaCha20-Poly1305 is the primary cipher.

**Where it's used (confirmed in code):**
- `controllers/ws_handler.rs`: WebSocket message encryption
- `core/double_ratchet.rs`: Double Ratchet session encryption
- `core/store.rs`: Encrypted message storage at rest

**Trade-off:** AES-GCM would be faster on x86 with AES-NI. XChaCha20-Poly1305 trades ~10-15% throughput on x86 for universal performance and a 192-bit nonce that prevents catastrophic nonce reuse.

---

## 3. Ed25519 + X25519 (Key Pairs)

**Decision:** Ed25519 for signatures, X25519 for key agreement.

**Alternatives considered:**
- ECDSA (secp256k1, P-256)
- RSA-2048/4096
- Dilithium (post-quantum signatures)

**Why Ed25519 + X25519:**
- Ed25519 gives 128-bit security with 32-byte public keys and 64-byte signatures. ECDSA needs 64-byte public keys and variable-length signatures. RSA needs 256+ byte keys.
- `ed25519-dalek` (v2.0) and `x25519-dalek` (v2.0) are pure Rust, audited, zero-dependency crates from the same dalek family. No FFI, no C bindings, no OpenSSL.
- X25519 provides Diffie-Hellman key agreement for the Noise protocol handshake and session key establishment.
- The `KeyType` enum in `encryption.rs` supports `Ed25519`, `X25519`, `Hybrid` (both), `PostQuantum` (Kyber), and `Ultimate` (Ed25519 + X25519 + Kyber). The system is designed to evolve.

**Why not ECDSA:** ECDSA signatures are deterministic but require careful nonce handling. Ed25519 is deterministic by design, eliminating the nonce-reuse catastrophe class entirely. ECDSA also has known timing side channels in naive implementations.

**Why not RSA:** RSA-2048 keys are 256 bytes. Ed25519 keys are 32 bytes. In a mesh network where every peer exchange includes key material, bandwidth matters. RSA verification is also slower.

**Trade-off:** Ed25519 is not FIPS 140-2 approved (as of 2025). If regulatory compliance is needed, P-256 ECDSA would be the fallback. GhostWire chose security properties over compliance checkboxes.

---

## 4. ML-KEM (Post-Quantum Key Encapsulation)

**Decision:** ML-KEM (Kyber768) via `pqcrypto-mlkem` (v0.1.1) for hybrid key exchange.

**Why it was added:**
- Shor's algorithm on a sufficiently large quantum computer breaks X25519 and Ed25519. While large-scale quantum computers don't exist today, encrypted mesh traffic can be harvested and decrypted later ("harvest now, decrypt later").
- ML-KEM is the NIST standard (FIPS 203) for post-quantum key encapsulation. Kyber768 is the recommended parameter set (192-bit classical security, 128-bit quantum security).
- GhostWire uses a hybrid approach: X25519 key exchange + ML-KEM encapsulation. If ML-KEM is broken, X25519 still protects. If X25519 is broken by quantum, ML-KEM still protects.

**Where it shows up:**
- `pqcrypto-mlkem` and `pqcrypto-traits` in Cargo.toml
- `EncryptedMessage.pq_key: Option<Vec<u8>>` in `encryption.rs` carries the Kyber-encapsulated key alongside the classical ciphertext

**Trade-off:** ML-KEM adds ~1KB to each handshake and ~10% latency on constrained devices. The hybrid approach means you pay both costs, but the security guarantee is strictly stronger than either alone.

---

## 5. Pluggable Transport (TCP, BLE, Obfs4, QUIC)

**Decision:** Transport-agnostic architecture via `TransportKind` enum and `TransportRegistry`.

**Alternatives considered:**
- Fixed TCP transport only
- Compile-time feature flags for transport selection

**Why pluggable:**
- GhostWire targets scenarios ranging from "normal internet" (TCP/QUIC) to "censored network" (Obfs4) to "no infrastructure" (BLE mesh). No single transport covers all use cases.
- The `TransportKind` enum in `transport.rs` lists: `Mesh` (libp2p), `Dummy`, `StealthTCP`, `Bluetooth`, `Simulated`.
- The `AutoTransportSwitcher` in `transport_manager.rs` monitors network health (latency, packet loss, peer count) and automatically switches transports when conditions degrade. This is the "nervous system" concept.
- `libp2p-webrtc` (v0.9.0-alpha.1) is included for browser-to-node connections without relays.

**Transport breakdown:**
| Transport | Purpose | Status |
|-----------|---------|--------|
| libp2p TCP | Standard P2P mesh | Working |
| libp2p WebRTC | Browser clients | Working (alpha) |
| BLE (bluer) | Local mesh, no internet | Feature-gated |
| Stealth TCP | Traffic morphing, DPI evasion | Working |
| Obfs4 | Tor-grade obfuscation | **NOT IMPLEMENTED** — no handshake, no framing, not in the transport chain. The old placeholder even set the "public key" to the secret key. |
| Simulated | Hardware testing (BT/WiFi) | Working |

**Trade-off:** Multiple transports increase code surface and testing matrix. The alternative (one transport) means failing in censorship or no-infrastructure scenarios. GhostWire chose coverage over simplicity.

---

## 6. Sphinx Packets (Privacy Routing)

**Decision:** Sphinx onion routing via the `sphinx-rs` crate (v0.1).

**Why Sphinx:**
- Sphinx is the packet format used by the Nym mixnet. It provides source hiding, payload encryption at each hop, and SURBs (Single-Use Reply Blocks) for anonymous replies.
- Unlike Tor's onion format, Sphinx has fixed-size packets, which prevents traffic analysis based on packet size. Every packet looks the same regardless of how many hops it traverses.
- SURBs let a recipient reply without knowing the sender's address. The sender embeds a SURB in their message; the recipient uses it to route a reply through the mixnet.
- The `sphinx-rs` crate is published by the Nym project and is independently maintained from GhostWire, which is good for security audit surface.

**Trade-off:** Sphinx adds latency (each hop decrypts and forwards) and bandwidth overhead (fixed-size packets mean padding). For real-time chat, this matters. GhostWire uses Sphinx for high-privacy messages and direct mesh routing for speed.

---

## 7. PRoPHET for DTN Routing

**Decision:** PRoPHET (Probabilistic Routing Protocol using History of Transmissions and Inter-Contact Times) as the DTN routing foundation.

**Why PRoPHET:**
- PRoPHET is the standard algorithm for delay-tolerant networks where nodes intermittently connect. It was designed for exactly the scenario GhostWire's BLE mesh creates: nodes come into range, exchange data, move apart.
- The implementation in `dtn/epidemic.rs` follows the original paper (Lindgren et al., 2003) with Spray-and-Wait (Spyropoulos et al., 2005) for copy bounded forwarding.
- PRoPHET's delivery predictability table (`P(dest)`) uses three mechanisms: encounter update (`P_init = 0.75`), time decay (`gamma = 0.98` per hour), and transitivity (`beta = 0.25`).
- Bloom filters (`SyncBloom`, 1024 bits) make the sync handshake efficient: nodes advertise what they have and need without revealing content.

**Parameters (from code):**
```
P_INIT = 0.75     # Initial encounter probability
GAMMA = 0.98      # Decay per hour
BETA = 0.25       # Transitivity scaling
MAX_BUFFER = 50MB  # Per-node bundle buffer
BLOOM_BITS = 1024  # Sync filter size
```

**Why not epidemic flooding:** Pure flooding wastes bandwidth on constrained BLE links. Spray-and-Wait bounds the number of copies (`L/2` copies in spray phase), preventing buffer bloat while maintaining delivery probability.

**Trade-off:** PRoPHET requires nodes to encounter each other to update delivery predictability. In a static network with no mobility, it degrades to direct delivery. The AI routing layers (below) address this.

---

## 8. 4-Layer AI Routing

**Decision:** Four-layer fallback chain for routing decisions.

**Layer 1: LightGBM/ONNX Fast Path**
- Model: `ghostwire_routing.onnx` (LightGBM, 2,387 features from Guifi.net mesh data)
- Inference: ORT (ONNX Runtime) via the `ort` crate (v2.0.0-rc.4)
- Latency: <1ms per decision
- Purpose: Anomaly detection and fast routing classification

**Layer 2: Gemma 4 Analytical**
- Client: HTTP to local Ollama (`gemma4:e2b` model)
- Latency: 50-200ms per analysis
- Purpose: Mesh topology analysis, relay optimization, threat assessment
- Runs every 30 seconds

**Layer 3: GNN (Graph Attention Network)**
- Model: `scoring_head.onnx` + `node_embeddings.json`
- Trained on real Guifi.net topology data
- Latency: ~91 microseconds
- Purpose: Graph-aware next-hop routing that understands network structure
- Runs every 5 minutes

**Layer 4: PRoPHET Fallback**
- Always available, no model required
- Latency: <0.01ms
- Purpose: When ML layers fail, PRoPHET's delivery predictability table still works

**Why four layers:**
- No single model handles all scenarios. LightGBM is fast but ignorant of graph structure. GNN understands topology but needs pre-computed embeddings. Gemma provides natural language analysis but is slow. PRoPHET is instant but doesn't optimize.
- The fallback chain means the system degrades gracefully: if the ONNX model fails to load, PRoPHET takes over. If Gemma is unavailable, the ONNX model still runs. If GNN embeddings aren't loaded, PRoPHET handles routing.
- The anomaly detection layer (Isolation Forest trained on NSL-KDD/Guifi.net data) feeds into Gemma for threat analysis, creating a security-aware routing system.

**Trade-off:** Four layers add complexity and model loading time. The alternative (PRoPHET alone) works but doesn't optimize for real-world mesh conditions. GhostWire chose intelligent routing over simplicity.

---

## 9. sled (Storage Engine)

**Decision:** sled (v0.34) as the primary embedded database.

**Alternatives considered:**
- SQLite (via `rusqlite`)
- RocksDB (via `rust-rocksdb`)
- LMDB (via `lmdb-rs`)

**Why sled:**
- Pure Rust, no C dependencies. Compiles anywhere Rust compiles, including WASM targets and embedded systems without a C toolchain.
- MVCC (multi-version concurrency control) with lock-free reads. Multiple mesh event handlers can read simultaneously without contention.
- The `PersistenceManager` in `persistence_manager.rs` wraps sled with a tree-based schema: `store_secure("messages", key, data)` encrypts-then-stores.
- Sled's `open()` is a single function call. SQLite requires `rusqlite` + `libsqlite3-sys` which means compiling C code.

**What sled doesn't do well:**
- Complex queries (JOINs, aggregations). GhostWire doesn't need them: it stores/retrieves by key.
- Large dataset performance (>100GB). GhostWire's data is per-node messages and keys, not a data warehouse.
- WAL-based crash recovery. Sled uses a copy-on-write B-tree with a separate flush thread.

**Why not SQLite:** SQLite is the most tested database in the world, but it requires a C compiler and has a complex concurrency model (WAL mode, journal modes). For a Rust mesh node that stores messages by key, sled is simpler.

**Why not RocksDB:** RocksDB pulls in ~500K lines of C++ code. The binary size and compile time penalty isn't worth it for GhostWire's access patterns.

**Trade-off:** Sled is less mature than SQLite or RocksDB. It has had breaking API changes across versions. But for GhostWire's needs (key-value storage with encryption), it's the right size.

---

## 10. Ratatui (TUI Framework)

**Decision:** Ratatui (v0.29) with crossterm (v0.29) for terminal UI.

**Alternatives considered:**
- `cursive` (higher-level, more batteries-included)
- `tui-rs` (predecessor to Ratatui, now unmaintained)

**Why Ratatui:**
- Ratatui is the actively maintained fork of `tui-rs`. The `tui-rs` crate was the standard but stopped receiving updates. Ratatui picked up where it left off with regular releases and a growing contributor base.
- Immediate-mode rendering: you rebuild the entire frame each tick. No widget tree, no event propagation complexity. You draw what you see.
- Crossterm as the backend works on Windows, Linux, and macOS without ncurses dependencies.
- The TUI is used for the GhostWire daemon client (`tui/daemon_client.rs`) showing connection status, peer list, message flow, and encryption status.

**Trade-off:** `cursive` provides a higher-level abstraction with built-in views (edit boxes, menus, dialogs). Ratatui is lower-level, meaning more boilerplate for complex layouts. GhostWire chose Ratatui for control over rendering and to avoid the `cursive` dependency tree.

---

## 11. Trade-offs Acknowledged

### Complexity vs Security
- The 4-layer AI routing system adds significant code surface. Each layer is independently useful, but the integration between them (feature extraction, topology building, model loading) creates failure modes that simpler systems don't have.
- Hybrid post-quantum key exchange (X25519 + ML-KEM) doubles the handshake overhead. The security guarantee is strictly stronger, but the cost is real on constrained devices.
- Sphinx packet encryption adds latency to every message routed through the mixnet.

### Performance vs Privacy
- Cover traffic (chaff frames sent every 30 seconds) prevents idle detection but consumes bandwidth and CPU.
- Traffic morphing in StealthTCP randomizes timing patterns, which adds jitter to real messages.
- Encrypted message storage means every message is encrypted-then-decrypted on read, doubling the crypto operations per message lifecycle.

### Maturity vs Ambition
- The ONNX routing model is loaded from a file that may not exist (`ghostwire_routing.onnx`). The system degrades to PRoPHET when it's missing, but the fallback is slower and less optimal.
- BLE transport is feature-gated and not fully implemented. The `bluer` crate (v0.17) supports BlueZ D-Bus, but the `ble_controller` module referenced in AGENTS.md isn't built yet.
- The Obfs4 transport is NOT implemented. There is no handshake, no framing and no socket wrapper, and it is not wired into the libp2p transport chain. `obfs4_enabled` is read only by a startup warning. Until that is built, GhostWire has NO transport obfuscation: TCP carries plaintext framed by Noise, and against a real DPI adversary its handshake is fingerprintable.
- The GNN router requires pre-computed node embeddings (`node_embeddings.json`) from Guifi.net training data. Without these files, it falls back to PRoPHET.

### Dependency Footprint
- The `Cargo.toml` lists 70+ direct dependencies. Some are heavy: `tokio` (full features), `libp2p` (with 13+ sub-features), `ort` (ONNX Runtime with binary download), `matrix-sdk` (optional).
- Release profile optimizes for binary size (`strip = true`, `lto = false`, `opt-level = 2`) but the debug binary is still 254MB.

### What Was Deliberately Simple
- The `PersistenceManager` uses sled's basic API: `open()`, `get_tree()`, `insert()`, `get()`. No schema migrations, no query planner, no WAL tuning. Good enough for key-value message storage.
- The `SecureMessageStore` encrypts-then-stores with a straightforward pipeline. No batch writes, no read-ahead caching, no connection pooling. sled handles concurrency internally.
- The `TransportRegistry` is a `HashMap<String, TransportKind>`. No dynamic discovery, no hot-swapping plugins. You register transports at startup and pick one.

---

*Generated from actual codebase analysis. Last updated: 2026-07-03.*
