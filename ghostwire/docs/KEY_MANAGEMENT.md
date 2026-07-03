# GhostWire Key Management Guide

> **Version**: 1.1 | **Status**: Draft | **Last Updated**: 2026-07-03
> **Standard**: OWASP Key Management Cheat Sheet + NIST SP 800-57

---

## 1. Overview

GhostWire uses Ed25519 for signing and X25519 for key agreement. Keys are generated, stored, rotated, and destroyed following OWASP/NIST guidelines.

---

## 2. Key Types

| Key Type | Algorithm | Purpose | Lifetime |
|----------|-----------|---------|----------|
| **Signing Key** | Ed25519 | Identity verification, message signing | Long-term (rotated quarterly) |
| **Encryption Key** | X25519 | Key agreement (ECDH) | Long-term (rotated quarterly) |
| **Session Key** | AES-256 | Message encryption | Per-session (forward secrecy) |
| **Biscuit Token** | HMAC + DTL | API authentication | Per-session (attenuated) |

---

## 3. Key Lifecycle

### 3.1 Generation

| Step | Method | Requirements |
|------|--------|-------------|
| Entropy source | OS CSPRNG (`/dev/urandom`, `getrandom`) | ≥256 bits entropy |
| Key generation | `ed25519-dalek`, `x25519-dalek` | FIPS 140-2 compliant module |
| Validation | Verify key is on curve, not weak | Reject zero/low-order points |

```rust
// Generated via:
let signing_key = SigningKey::generate(&mut OsRng);
let encryption_key = EphemeralSecret::random_from_rng(OsRng);
```

**Small-Order Point Defense (CVE-2025-62514):**
After X25519 Diffie-Hellman, the shared secret is checked for all-zero output. If the DH result is entirely zero bytes, the key exchange is rejected. This prevents small-class attacks where crafted public keys produce degenerate shared secrets.

```rust
// In encryption.rs — after diffie_hellman()
if shared_secret.as_bytes().iter().all(|&b| b == 0) {
    return Err(anyhow::anyhow!("X25519: degenerate shared secret (all zeros)"));
}
```

### 3.2 Storage

| Storage Type | Protection | Location |
|-------------|------------|----------|
| **Private Keys** | AES-256-GCM encrypted | `~/.ghostwire/keys/` |
| **Public Keys** | plaintext | Distributed to peers |
| **Session Keys** | In-memory only | Zeroized on drop |
| **Biscuit Tokens** | In-memory only | Expire on session end |

**Storage Rules:**
- Private keys encrypted at rest with device-specific key
- Session keys never written to disk
- Keys zeroized (`zeroize` crate) on drop
- No key material in logs, error messages, or core dumps

### 3.3 Distribution

| Method | Trust Model | Verification |
|--------|-------------|-------------|
| **Direct Exchange** | Out-of-band verification | QR code + fingerprint |
| **DHT** | Trust-on-first-use (TOFU) | Signature verification |
| **Social Graph** | Transitive trust | Trust score propagation |

### 3.4 Rotation

| Key Type | Rotation Interval | Trigger |
|----------|------------------|---------|
| Signing Key | 90 days | Calendar + manual |
| Encryption Key | 90 days | Calendar + manual |
| Session Key | Per-session | New connection |
| Biscuit Token | Per-session | New authentication |

**Rotation Process:**
1. Generate new key pair
2. Sign new public key with old signing key (transition proof)
3. Distribute to connected peers
4. Keep old key for verification of in-flight messages (24h grace)
5. Securely erase old key

### 3.5 Revocation

| Method | Propagation | Reliability |
|--------|-------------|-------------|
| **Gossip CRL** | Peer-to-peer gossip | Medium (planned v1.0 Phase 2) |
| **Social Recovery** | Trusted contacts | High (planned v1.2) |
| **KERI** | Self-sovereign | High (planned v1.2) |

**Revocation Process:**
1. User reports key compromise
2. Generate revocation certificate (pre-signed)
3. Broadcast via gossip protocol
4. Peers update trust scores
5. Compromised key rejected by network

### 3.6 Destruction

| Method | When | Verification |
|--------|------|-------------|
| **Zeroize** | Session end, key rotation | Memory overwrite |
| **Secure Delete** | Key revocation | File overwrite + unlink |
| **Memory Drop** | Rust `Drop` trait | Automatic zeroize |

---

## 4. Cryptographic Primitives

| Purpose | Algorithm | Library | Security Level |
|---------|-----------|---------|---------------|
| **Signing** | Ed25519 | `ed25519-dalek` | 128-bit |
| **Key Agreement** | X25519 | `x25519-dalek` | 128-bit |
| **Symmetric Encryption** | AES-256-GCM | `aes-gcm` | 256-bit |
| **Hashing** | SHA-256 | `sha2` | 128-bit (collision) |
| **Key Derivation** | HKDF-SHA256 | `hkdf` | 128-bit |
| **Post-Quantum (hybrid)** | ML-KEM-768 | `ml-kem` | 192-bit (quantum) |

### 4.1 Key Derivation

```
Master Secret (X25519 ECDH)
    ↓ HKDF-SHA256
Session Key (AES-256)
    ↓ HKDF-SHA256
Message Key (AES-256, per-message)
```

### 4.2 Hybrid Mode (Post-Quantum)

When enabled, GhostWire uses hybrid key exchange:
- Classical: X25519 ECDH
- Post-quantum: ML-KEM-768 (Kyber)
- Combined: Both shared secrets concatenated → HKDF → Session Key

---

## 5. Security Properties

| Property | Guarantee | Limitation |
|----------|-----------|------------|
| **Forward Secrecy** | Per-session keys | Past messages safe if session key secure |
| **Break-in Recovery** | Key rotation + revocation | Gap between compromise and revocation |
| **Key Isolation** | Separate signing/encryption keys | Compromise of device exposes both |
| **Secure Wipe** | Zeroize on drop | OS may have swap/dump copies |

---

## 6. Threat Mitigations

| Threat | Mitigation | Status |
|--------|------------|--------|
| Key theft from disk | Encrypted storage | ✅ Implemented |
| Key theft from memory | Zeroize on drop | ✅ Implemented |
| Key compromise | Rotation + revocation | ⚠️ Partial (revocation planned) |
| Replay attack | Nonces + session keys | ✅ Implemented |
| Quantum attack | ML-KEM hybrid mode | ✅ Available (feature flag) |
| Weak key generation | OS CSPRNG | ✅ Implemented |

---

## 7. Compliance

| Standard | Requirement | Status |
|----------|-------------|--------|
| **NIST SP 800-57** | Key management lifecycle | ✅ Aligned |
| **OWASP** | Key storage best practices | ✅ Aligned |
| **FIPS 140-2** | Cryptographic module | ⚠️ Uses FIPS-eligible algorithms |
| **GDPR** | Data protection | ✅ Local-only storage |

---

## 8. References

- [OWASP Key Management Cheat Sheet](https://cheatsheetseries.owasp.org/cheatsheets/Key_Management_Cheat_Sheet.html)
- [NIST SP 800-57 Part 1](https://csrc.nist.gov/publications/detail/sp/800-57-part-1/rev-5/final)
- [GhostWire Threat Model](./THREAT_MODEL.md)
- [GhostWire Security Policy](../SECURITY.md)
