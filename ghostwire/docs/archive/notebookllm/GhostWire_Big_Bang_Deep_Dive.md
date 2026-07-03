# ⚛️ The Big Bang: Layer-by-Layer Reconstruction

The "Big Bang" of GhostWire wasn't just an empty repository. It was a structured explosion of security and networking primitives. Here is the reconstruction of the first layers added on **April 30, 2025**.

## 🧬 Layer 1: The Core Infrastructure (`9918a692`)

At the moment of creation, GhostWire was already a functional CLI tool written in Rust.

### 1. The Network Layer (`ghostwire/src/core/networking.rs`)
- **Engine**: `libp2p v0.43`.
- **Protocol**: `/ghostwire/1.0.0` using a Request/Response model.
- **Discovery**: `mDNS` for local peer discovery.
- **Mechanism**: A `Swarm` managing peer events, listening on all interfaces (`0.0.0.0`) on random ports.
- **Role**: This was the "nervous system" allowing any instance to talk to another without a central server.

### 2. The Cryptographic Layer (`ghostwire/src/core/encryption.rs`)
- **Engine**: `sodiumoxide` (wrappers for `libsodium`).
- **Symmetric**: `secretbox` (XSalsa20-Poly1305) for message encryption.
- **Asymmetric**: `sign::gen_keypair` (Ed25519) for identity and signatures.
- **Logic**: Every message was nonced and sealed before even hitting the network layer.
- **Role**: This was the "armor," ensuring that even if a message was intercepted, it was unreadable and authenticated.

### 3. The Command Layer (`ghostwire/src/cli/commands.rs`)
- **Action**: The `Whisper` command.
- **Flow**: 
    1. Initialize Encryption.
    2. Spawn Networking in a background `tokio` task.
    3. Manually parse a PeerID.
    4. Encrypt the string.
    5. Send via the `libp2p` swarm.
- **Role**: This was the "brain," translating user intent into network packets.

### 4. The Data Vision (`ghostwire/README.md`)
- **Original Goal**: "A secure, decentralized platform for threat intelligence sharing and private messaging."
- **Planned Modules**: `ioc/` (Indicators of Compromise), `trust/` (Reputation), and `tor_integration/`.

## 🛠️ Comparison: Then vs. Now (February 2026)

| Layer          | Big Bang (April 2025)        | Node Cluster V2 (Dec 2025)        | Mobile & Modernization (Feb 2026)     |
| :------------- | :---------------------- | :-------------------------------- | :------------------------------------ |
| **Encryption** | Sodiumoxide (Secretbox) | AES-256-GCM + X25519 Hybrid       | + HKDF-SHA256 + ML-KEM (PQC)          |
| **Networking** | Basic mDNS Discovery    | Kademlia DHT + Mesh Topology      | libp2p v0.56 + WebRTC + AutoNAT       |
| **UI**         | CLI Only (`Whisper`)    | Tactical HUD (React + WebUI)      | + Tauri Mobile App (Android)          |
| **TUI**        | ❌ None                | Full TUI (1,400+ lines)           | ❌ Deprecated (removed)                |
| **Security**   | Signing/Encryption      | PoW Sybil Defense + Rate Limiting | Key Rotation + Revocation Lists       |
| **License**    | MIT                     | AGPL-3.0                          | AGPL-3.0 (unchanged)                  |
| **Testing**    | ❌ None                | Basic unit tests                  | Integration tests + Hurl API suite    |
| **Codebase**   | ~5K lines              | ~175K lines (with archives)       | ~10K lines (85% reduction)            |

## 🛰️ Key Takeaway
The "Big Bang" was remarkably mature. GhostWire didn't start as a "Hello World"; it started with **libp2p** and **NaCl** (Networking and Cryptography) fully integrated. The layers added since have been about scaling, visualization, and hardening that core "Big Bang" energy.
