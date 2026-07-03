# GhostWire Cryptography Implementation

This document provides a comprehensive overview of the cryptographic foundation used in the GhostWire project. Our approach prioritizing robustness, performance, and future-proof design, ensuring end-to-end security for messages, identities, and node communication.

## Core Principles

-   **Zero-Trust:** Every node and message must cryptographically prove its authenticity.
-   **Defense-in-Depth:** Multiple layers of encryption to prevent single points of failure.
-   **Forward Secrecy:** Protecting past communications even if long-term keys are compromised.
-   **Resilience:** Fail-open defaults at the application layer while enforcing strict cryptography at the transport layer.

## Identity and Key Management

### Cryptographic Identities (Ed25519)
GhostWire identities are rooted in **Ed25519** public-key cryptography. This provides fast, highly secure digital signatures ideal for the fast-paced peer-to-peer network environment.

-   **Key Generation:** When an `Identity` is created (in `core_identity.rs`), an Ed25519 keypair is generated using system-level secure entropy sources via `libp2p::identity::Keypair::generate_ed25519`.
-   **Decentralized Identifiers (DIDs):** The public key is hashed using SHA-256 to create a globally unique fingerprint (DID) formatted as `did:ghost:<sha256_hash_hex>`.
-   **Profile Security:**
    -   Keypairs are maintained securely in memory.
    -   When saved to disk (`identity.json`), the keypair is encrypted with the user's password using standard symmetric encryption (detailed below).
    -   A locked `IdentityState` clears sensitive key material from active memory.

### Symmetric Encryption (XChaCha20-Poly1305)
For persistent data at rest and symmetric message payloads, we utilize Authenticated Encryption with Associated Data (AEAD) using **XChaCha20-Poly1305**. This cipher was chosen over AES-GCM to prevent nonce reuse vulnerabilities and provide performance benefits on devices lacking hardware AES acceleration.

-   **Identity Storage:** When saving an identity, a symmetric key is derived from the user's password. The Ed25519 keypair is then encrypted using this derived key via XChaCha20-Poly1305.

### Key Derivation (Argon2id)
We use **Argon2id** (the winner of the Password Hashing Competition) to derive strong symmetric keys from user passwords. Argon2id defends against both dictionary attacks and side-channel threats.

-   **Process:** When a password is provided (e.g., during profile setup or unlock), a persistent salt is used alongside Argon2id's memory-hard configurations to calculate the XChaCha20 encryption/decryption key.

## Transportation Security

### libp2p NOISE Protocol
GhostWire's core transport layer operates on the battle-tested `libp2p` networking stack. All peer-to-peer connections are automatically upgraded to use the **NOISE protocol framework**.

-   **Authentication:** Nodes authenticate each other using their Ed25519 public keys during the handshake, ensuring connections only ever occur between verified identities.
-   **Encryption in Transit:** The NOISE protocol establishes an encrypted tunnel for all Muxed streams (like Gossipsub), guaranteeing data confidentiality over untrusted internet connections.
-   **Perfect Forward Secrecy:** Temporary session keys are negotiated via Diffie-Hellman exchanges (e.g., X25519), ensuring that the compromise of a long-term identity key will not compromise past sessions.

## Message Integrity and Gossipsub

### Structural Integrity
Our primary message bus relies on `libp2p::gossipsub`. GhostWire configures Gossipsub defensively to prevent network spam and spoofed messages.

-   **Strict Message Authentication:** `MessageAuthenticity::Signed` requires every broadcasted message to contain a valid Ed25519 signature from its author. Nodes silently drop messages that fail signature verification.
-   **Content-Addressed Identifiers:** Our `MessageId` function hashes the message source peer ID, the payload, and the topic using `sha256`. This guarantees that network message IDs are globally deterministic and immutable.

### Application-Layer Encryption
Future extensions to GhostWire (like Direct Messaging) will build strictly on this foundation. P2P Direct Messages will be wrapped using additional layers of public-key encryption (e.g., ECIES) before being broadcasted, so only the recipient holding the respective private key can read the payload content.

## Dependencies

-   `libp2p::identity`: Core keypair and signing primitives.
-   `ring`: Underlying cryptographic primitives (SHA-256 for hashes, secure PRNG).
-   `chacha20poly1305`: Authenticated symmetric encryption for local storage.
-   `argon2`: Memory-hard key derivation.
