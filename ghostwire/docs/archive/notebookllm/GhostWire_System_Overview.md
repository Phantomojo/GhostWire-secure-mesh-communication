# GhostWire System Architecture & Design Overview

## 1. High-Level Architecture

GhostWire is designed as a modular, secure mesh networking application split into two primary layers:

1.  **Core Library (`ghostwire` crate)**: Handles all networking, encryption, identity, and security logic.
2.  **Interfaces**:
    *   **Web API (Axum)**: Exposes core functionality via REST endpoints (v1 and v2) for the frontend or external apps.
    *   **CLI/TUI**: A terminal-based user interface for direct interaction without a browser.

### Component Relationship
`User` <-> `Interface (Web/TUI)` <-> `Core::App State` <-> `Security Manager` <-> `Encryption Manager` <-> `Mesh Transport`

## 2. Core Components

### A. Encryption Manager (`src/core/encryption.rs`)
-   **Hybrid Encryption**: Uses **Ed25519** for digital signatures (identity verification) and **X25519** for key agreement (ECDH).
-   **Session Keys**: Derived using HKDF (HMAC-based Key Derivation Function) from the shared secret.
-   **Protocol**:
    1.  Alice generates ephemeral X25519 key pair.
    2.  Alice performs ECDH with Bob's static public key.
    3.  Shared secret is used to derive a session key.
    4.  Messages are encrypted using **AES-256-GCM** with this session key.
    5.  Signatures are verified using Ed25519 public keys.

### B. Security Manager (`src/core/security.rs`)
-   **Threat Detection**: Monitors traffic patterns for anomalies (e.g., rapid connection attempts, malformed packets).
-   **Scoring System**: Assigns trust scores to peers.
    -   *Good behavior* (verified signature, stable connection) increases score.
    -   *Bad behavior* (invalid signature, rate limit breach) decreases score.
-   **Rate Limiting**: Uses a sliding window algorithm to enforce message/connection limits per IP.
-   **Blacklisting**: Automatically blocks IPs that exceed threat thresholds.

### C. Mesh Transport (`src/core/mesh.rs` & `stealth_tcp.rs`)
-   **Stealth TCP**: A custom transport layer designed to look like benign traffic (HTTPS/TLS) to evade DPI (Deep Packet Inspection).
-   **Handshake**: Randomizes handshake timing and packet sizes to avoid fingerprinting.
-   **Discovery**: Uses MDNS for local discovery and a Kademlia DHT for global peer finding.

## 3. Data Flow Example: Sending a Message

1.  **API Call**: User POSTs to `/api/v2/send_message`.
2.  **Validation**: Web handler validates input and checks authentication.
3.  **Security Check**: `SecurityManager` verifies the sender isn't rate-limited.
4.  **Encryption**: `EncryptionManager` looks up the recipient's public key, derives a session key (if not cached), encrypts the payload (AES-256-GCM), and signs it (Ed25519).
5.  **Transport**: `MeshManager` routes the encrypted packet via the active transport (StealthTCP) to the peer.
6.  **Stats**: `SecurityManager` increments `messages_processed` stats.

## 4. Current Project State (as of latest commit)
-   **API v2**: Fully implemented for Profile, Peers, and Security Dashboard.
-   **Encryption**: Unit tests passing. *Note: Verification currently uses a deterministic stub for session key derivation in tests due to temporary lack of persistent X25519 storage.*
-   **Integration**: Basic connectivity verified; multi-node simulation pending.
