# Cryptography Specification

**GIP:** Pending
**Version:** 0.2.0-draft
**Date:** April 12, 2026

---

## Overview

This specification defines the cryptographic primitives, constructions, and key management protocols used throughout GhostWire. All peer-to-peer communication is encrypted by default; there is no plaintext mode. GhostWire employs a defense-in-depth strategy combining well-vetted primitives (XChaCha20-Poly1305, X25519, Ed25519, BLAKE3) with post-quantum readiness (ML-KEM-768) and coercion resistance (duress PIN with independent namespaces).

---

## Cryptographic Primitives

### AEAD: XChaCha20-Poly1305

XChaCha20-Poly1305 is the primary authenticated encryption with associated data (AEAD) construction used for all message encryption.

| Property | Value |
|----------|-------|
| **Cipher** | XChaCha20 (extended-nonce ChaCha20) |
| **MAC** | Poly1305 one-time authenticator |
| **Key size** | 256 bits (32 bytes) |
| **Nonce size** | 192 bits (24 bytes) |
| **Tag size** | 128 bits (16 bytes) |
| **Max message size** | 256 GB per nonce |
| **Standard** | RFC 8439 (ChaCha20-Poly1305), extended nonce per libsodium convention |

**Usage:**
- All application-layer messages are encrypted with XChaCha20-Poly1305 before framing.
- The nonce MUST be unique per message under a given key. Nonces are generated via the symmetric ratchet (see Forward Secrecy below) or via CSPRNG for one-shot operations.
- Associated data (AAD) includes the sender and recipient public keys to bind ciphertext to the communication context.

**Nonce construction for direct messages:**
```
nonce = ratchet_next(send_chain)  // 24 bytes from symmetric ratchet
```

**Nonce construction for stored data (local persistence, DHT values):**
```
nonce = random_bytes(24)  // CSPRNG (getrandom syscall)
```

### Key Exchange: X25519

X25519 (Curve25519 Diffie-Hellman) is the primary key exchange mechanism.

| Property | Value |
|----------|-------|
| **Curve** | Curve25519 (Montgomery form) |
| **Function** | X25519 scalar multiplication |
| **Public key size** | 32 bytes (u-coordinate) |
| **Shared secret size** | 32 bytes |
| **Standard** | RFC 7748 |

**Usage:**
- Ephemeral X25519 keypairs are generated for each Sphinx hop and each session handshake.
- Long-term X25519 keys are derived from the identity Ed25519 keypair (see Identity spec).
- The shared secret is fed into a KDF (HKDF-BLAKE3 or Argon2id) to derive session keys.

**Clamping:** X25519 scalar clamping is applied per RFC 7748:
```
scalar[0] &= 248
scalar[31] &= 127
scalar[31] |= 64
```

### Signatures: Ed25519

Ed25519 is used for all digital signatures: message authentication, peer identity binding, and social protocol attestations.

| Property | Value |
|----------|-------|
| **Curve** | Ed25519 (twisted Edwards form of Curve25519) |
| **Hash** | SHA-512 (internal to Ed25519) |
| **Public key size** | 32 bytes |
| **Signature size** | 64 bytes |
| **Standard** | RFC 8032 |

**Usage:**
- Every peer possesses a long-term Ed25519 identity keypair.
- All ConnectRequests, social attestations, and protocol messages requiring non-repudiation are signed with Ed25519.
- The Ed25519 public key is the peer's primary identity (see Identity spec).

**Key conversion:** Ed25519 and X25519 keys are interoperable via the standard conversion:
```
x25519_public = edwards_to_montgomery(ed25519_public)
```
This allows a single keypair to serve both signature and key exchange functions.

### Hashing: BLAKE3

BLAKE3 is the primary hash function throughout GhostWire.

| Property | Value |
|----------|-------|
| **Output size** | Configurable (default: 256 bits / 32 bytes) |
| **Input size** | Unlimited |
| **Type** | Merkle-Damgard / tree hash |
| **Standard** | [BLAKE3 specification](https://github.com/BLAKE3-team/BLAKE3-specs) |

**Usage:**
- Peer ID derivation: `peer_id = BLAKE3(public_key, output_len=32)`
- Bundle ID (DTN): `bundle_id = BLAKE3(serialized_bundle, output_len=32)`
- Rendezvous point computation: `rendezvous = BLAKE3(pubkey || "rendezvous")[:16]`
- Session key derivation during device pairing
- Content addressing in BlockStore/Bitswap
- Integrity verification of downloaded models and configuration

**Keyed mode:** BLAKE3 keyed mode is used as a PRF where a dedicated key is required:
```
prf_output = BLAKE3_keyed(key, message, output_len=32)
```

### Key Derivation: Argon2id

Argon2id is used for password-based key derivation and memory-hard operations.

| Property | Value |
|----------|-------|
| **Variant** | Argon2id (hybrid memory-hard) |
| **Memory cost** | 64 MiB (65536 KiB) |
| **Time cost** | 3 iterations |
| **Parallelism** | 4 lanes (or number of available cores, capped at 4) |
| **Output length** | 32 bytes (256 bits) |
| **Standard** | RFC 9106 |

**Usage:**
- Local database encryption: user passphrase → Argon2id → AES-256-GCM key
- Device pairing session key derivation from short display codes
- Recovery share encryption (combined with XChaCha20-Poly1305)
- Duress PIN key derivation (see Coercion Resistance below)

**Parameter rationale:** 64 MiB memory cost provides strong GPU/ASIC resistance while remaining feasible on mobile and embedded devices (minimum 256 MiB RAM recommended). 3 iterations balance security against denial-of-service via repeated KDF calls.

---

## Post-Quantum Cryptography

### ML-KEM-768 (Kyber-768)

ML-KEM-768 (Module-Lattice-Based Key-Encapsulation Mechanism) provides post-quantum security for key exchange.

| Property | Value |
|----------|-------|
| **Algorithm** | ML-KEM-768 (formerly CRYSTALS-Kyber-768) |
| **Security level** | NIST Level 3 (equivalent to ~192-bit classical) |
| **Public key size** | 1184 bytes |
| **Ciphertext size** | 1088 bytes |
| **Shared secret size** | 32 bytes |
| **Standard** | FIPS 203 (August 2024) |

**Hybrid construction:** GhostWire uses a hybrid X25519 + ML-KEM-768 key exchange. The final session key is derived from both shared secrets:

```
shared_secret_x25519 = X25519(ephemeral_privkey, peer_pubkey)
(ciphertext, shared_secret_pq) = ML-KEM-768.Encaps(peer_pq_pubkey)
session_key = HKDF-BLAKE3(
    salt = random_bytes(32),
    ikm = shared_secret_x25519 || shared_secret_pq,
    info = "ghostwire/hybrid-session-key/v2",
    length = 32
)
```

**Security property:** The hybrid construction is secure if either X25519 or ML-KEM-768 remains unbroken. A quantum computer capable of breaking X25519 (via Shor's algorithm) cannot break ML-KEM-768, and vice versa.

**Transport:** ML-KEM-768 public keys and ciphertexts are transmitted in the `pq_key` field of the `EncryptedMessage` wire format (see WIRE_FORMAT.md).

**Fallback:** If a peer does not support ML-KEM-768, the connection falls back to X25519-only key exchange with a warning logged. The `pq_key` field is set to `None`.

---

## Forward Secrecy: Symmetric Ratchet

GhostWire implements a symmetric ratchet to provide forward secrecy for ongoing conversations.

### Ratchet Construction

```
state = (send_chain: Chain, recv_chain: Chain)

Chain = (key: [u8; 32], step: u64)

ratchet_next(chain):
    (key, nonce) = HKDF-BLAKE3(
        salt = chain.key,
        ikm = chain.key,
        info = "ghostwire/ratchet/v2",
        length = 64  // 32 bytes key + 32 bytes nonce material
    )
    chain.key = key[:32]
    nonce = key[32:56]  // 24 bytes for XChaCha20
    chain.step += 1
    return nonce
```

### Ratchet Initialization

The ratchet is initialized from the X25519 shared secret:

```
initial_key = HKDF-BLAKE3(
    salt = random_bytes(32),
    ikm = x25519_shared_secret,
    info = "ghostwire/ratchet-init/v2",
    length = 32
)
send_chain = Chain { key: initial_key, step: 0 }
recv_chain = Chain { key: initial_key, step: 0 }
```

### Security Properties

- **Forward secrecy:** Compromising the current ratchet key does not reveal past message keys (HKDF is one-way).
- **Post-compromise security:** After a key rotation event (see Key Management), the ratchet is re-initialized from a fresh shared secret, healing from prior compromise.
- **Message ordering:** The `step` counter provides implicit message ordering. Out-of-order messages can be decrypted if their nonce was already generated; the receiver buffers nonces for expected future steps (default window: 64 steps).

### Ratchet Sync

For bidirectional communication, each peer maintains independent send and receive chains. The send chain of peer A corresponds to the receive chain of peer B, and vice versa. No ratchet state is transmitted over the wire; only the initial key exchange material is sent.

---

## Key Management

### Trust-On-First-Use (TOFU)

GhostWire uses TOFU for initial peer key verification:

1. On first contact with a peer, their Ed25519 public key is recorded and pinned.
2. The key fingerprint (BLAKE3 hash, base58btc-encoded) is displayed to the user as a "safety number."
3. Subsequent connections verify the peer's key against the pinned value.
4. If the key differs, the connection is rejected and the user is alerted to a potential MITM attack.

**TOFU bypass:** Users may manually verify and trust a key fingerprint out-of-band (e.g., in person, via voice call) before first contact, upgrading from TOFU to explicit trust.

### Key Rotation

Keys are rotated under the following conditions:

| Event | Action |
|-------|--------|
| **Session expiry** | New ephemeral X25519 keypair generated |
| **Ratchet exhaustion** | Re-initialize ratchet from fresh X25519 handshake (threshold: 2^48 messages per chain) |
| **Peer reconnection** | Fresh X25519 handshake; ratchet re-initialized |
| **Suspected compromise** | Full identity keypair rotation (requires Shamir recovery or re-pairing) |
| **Scheduled rotation** | Optional periodic rotation (default: disabled; user-configurable) |

### Key Revocation

If a peer's identity keypair is compromised:

1. The peer (or their guardian, via Shamir recovery) generates a revocation certificate signed with the old key.
2. The revocation is published to the DHT and gossiped to connected peers.
3. Peers receiving the revocation mark the old key as revoked and reject any messages signed with it.
4. The peer generates a new identity keypair and re-establishes connections.

**Revocation certificate format:**
```rust
pub struct RevocationCertificate {
    pub revoked_pubkey: [u8; 32],
    pub reason: RevocationReason,
    pub timestamp: u64,
    pub signature: [u8; 64],  // Signed with revoked_privkey
}

pub enum RevocationReason {
    KeyCompromise,
    DeviceLoss,
    Coercion,
    UserRequest,
}
```

---

## Coercion Resistance

GhostWire provides mechanisms to resist physical coercion (e.g., border checks, extortion, forced device access).

### Duress PIN

Users configure two PINs/passphrases:

| PIN Type | Function |
|----------|----------|
| **Primary PIN** | Unlocks the real identity keypair and all data |
| **Duress PIN** | Unlocks a decoy identity with independent, plausible data |

**Derivation:**
```
primary_key = Argon2id(primary_pin, salt, memory=64MB, iterations=3)
duress_key = Argon2id(duress_pin, salt, memory=64MB, iterations=3)
```

Both derivations use the same salt and parameters but produce independent keypairs. The duress keypair is a fully functional Ed25519 + X25519 identity with its own:
- Peer ID and DID
- Contact list (decoy contacts, generated or user-populated)
- Message history (decoy conversations)
- Social graph (plausible but fake connections)

**Constant-time comparison:** PIN verification uses constant-time comparison to prevent timing side-channel attacks that could distinguish primary from duress PINs:

```rust
fn verify_pin(input: &[u8], stored_hash: &[u8], salt: &[u8]) -> bool {
    let input_hash = argon2id_hash(input, salt);
    subtle::ConstantTimeEq::ct_eq(&input_hash, stored_hash).into()
}
```

**Namespace separation:** The primary and duress namespaces are fully independent. No data from the primary namespace is accessible when unlocked with the duress PIN. The duress namespace is indistinguishable from a real identity under forensic analysis.

### Panic Mode

Panic mode provides rapid data protection under duress:

1. **Distress alert:** Optionally sends an encrypted alert to trusted contacts (guardians) via Sphinx route.
2. **Zeroize:** Overwrites in-memory key material with zeros using `zeroize` crate (mlock'd memory).
3. **Re-lock:** Re-encrypts local database with a fresh random key; the user must re-enter their PIN to unlock.

**Trigger:** Panic mode is triggered by a dedicated gesture/shortcut (user-configurable) or by entering a specific "panic PIN" at the unlock screen.

### Dead Man's Switch

A heartbeat-based dead man's switch provides additional protection:

- The client sends an encrypted heartbeat every 24 hours (configurable).
- If the heartbeat is not acknowledged (user is unavailable), trusted guardians are notified.
- After a configurable grace period (default: 72 hours), guardians can initiate Shamir recovery to revoke the user's keys.

---

## Security Properties Summary

| Property | Mechanism |
|----------|-----------|
| **Confidentiality** | XChaCha20-Poly1305 AEAD |
| **Integrity** | Poly1305 MAC (128-bit tag) |
| **Authentication** | Ed25519 signatures + TOFU |
| **Forward secrecy** | Symmetric ratchet (HKDF-BLAKE3 chain) |
| **Post-quantum security** | ML-KEM-768 hybrid key exchange |
| **Password hardening** | Argon2id (64 MiB, 3 iterations) |
| **Coercion resistance** | Duress PIN with independent namespaces |
| **Timing attack resistance** | Constant-time comparison for PIN verification |
| **Memory safety** | zeroize crate for key material, mlock'd memory |
| **Key revocation** | Signed revocation certificates, DHT publication |

---

## Reference Implementation

See the following modules in the GhostWire reference implementation:

| Module | Path |
|--------|------|
| AEAD encryption | `src/core/crypto/aead.rs` |
| Key exchange | `src/core/crypto/kex.rs` |
| Signatures | `src/core/crypto/signature.rs` |
| Symmetric ratchet | `src/core/crypto/ratchet.rs` |
| Argon2id KDF | `src/core/crypto/kdf.rs` |
| ML-KEM-768 hybrid | `src/core/crypto/pq_crypto.rs` |
| Coercion resistance | `src/core/coercion_resistance.rs` |
| Recovery manager | `src/core/identity/recovery.rs` |
