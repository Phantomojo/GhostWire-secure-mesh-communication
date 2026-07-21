<!--
  Copyright 2025-2026 Michael Irungu and GhostWire Contributors
  SPDX-License-Identifier: Apache-2.0

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.

  AI Training Restriction: This specification may not be used for training,
  fine-tuning, or otherwise developing machine learning models, large language
  models, or artificial intelligence systems without prior written permission
  from the copyright holder.
-->

# Identity Specification

**GIP:** Pending
**Version:** 0.2.0-draft
**Date:** April 12, 2026

---

## Overview

This specification defines the identity system used throughout GhostWire, including decentralized identifiers (DIDs), peer IDs, keypair management, local persistence, device pairing, Shamir Secret Recovery, coercion resistance, and contact card formats. GhostWire identities are self-sovereign: no central authority issues or revokes identities. Each peer generates and manages their own cryptographic identity.

---

## Decentralized Identifiers (DID)

### DID Format

GhostWire uses the DID method `did:ghost:` with Ed25519 public keys as the identifier.

```
did:ghost:<sha256_hex(ed25519_pubkey)>
```

| Component | Description |
|-----------|-------------|
| **Method** | `ghost` (GhostWire-specific DID method) |
| **Identifier** | SHA-256 hex encoding of the Ed25519 public key (64 hex characters) |

**Example:**
```
did:ghost:a1b2c3d4e5f6a7b8c9d0e1f2a3b4c5d6e7f8a9b0c1d2e3f4a5b6c7d8e9f0a1b2
```

**Rationale for SHA-256 over BLAKE3:** DIDs are often integrated with external systems that expect SHA-256. Using SHA-256 ensures compatibility with DID resolvers and verification tools outside the GhostWire ecosystem.

### DID Document

The DID document is stored locally and optionally published to the DHT:

```json
{
  "@context": "https://www.w3.org/ns/did/v1",
  "id": "did:ghost:<sha256_hex>",
  "verificationMethod": [{
    "id": "did:ghost:<sha256_hex>#keys-1",
    "type": "Ed25519VerificationKey2020",
    "controller": "did:ghost:<sha256_hex>",
    "publicKeyMultibase": "<base58btc_ed25519_pubkey>"
  }],
  "authentication": ["did:ghost:<sha256_hex>#keys-1"],
  "keyAgreement": ["did:ghost:<sha256_hex>#x25519-1"],
  "service": [{
    "id": "did:ghost:<sha256_hex>#ghostwire",
    "type": "GhostWireMessaging",
    "serviceEndpoint": "<multiaddr_or_dht_rendezvous>"
  }]
}
```

---

## Peer ID

### Peer ID Format

The Peer ID is the primary identifier used within the GhostWire protocol.

```
peer_id = BLAKE3(ed25519_pubkey, output_len=32)
peer_id_base58 = base58btc_encode(peer_id)
```

| Property | Value |
|----------|-------|
| **Hash function** | BLAKE3 |
| **Output length** | 32 bytes (256 bits) |
| **Encoding** | Base58BTC |
| **Encoded length** | ~44 characters |

**Example:** `4tK9mN2pQ7rS8vW1xY3zA5bC6dE7fG8hJ9kL0mN1pQ2r`

### Peer ID vs DID

| Property | Peer ID | DID |
|----------|---------|-----|
| **Hash** | BLAKE3 | SHA-256 |
| **Encoding** | Base58BTC | Hex |
| **Length** | ~44 chars | 72 chars (did:ghost: + 64 hex) |
| **Use case** | Internal protocol identifier | External DID resolution |
| **Reversible** | No (one-way hash) | No (one-way hash) |

Both identifiers are derived from the same Ed25519 public key and are deterministically linked.

---

## Identity Keypair

### Keypair Composition

Each GhostWire peer possesses an identity keypair consisting of:

| Component | Algorithm | Size | Purpose |
|-----------|-----------|------|---------|
| **Signing key** | Ed25519 | 32 bytes (private) + 32 bytes (public) | Message signing, identity verification |
| **Encryption key** | X25519 | 32 bytes (private) + 32 bytes (public) | Key exchange, encrypted communication |

**Total keypair size:** 128 bytes (64 bytes private + 64 bytes public).

### Key Generation

```
ed25519_keypair = Ed25519::generate()  // CSPRNG
x25519_keypair = X25519::from_ed25519(ed25519_keypair)  // Standard conversion
```

The X25519 keypair is derived from the Ed25519 keypair using the standard Edwards-to-Montgomery conversion, ensuring a single seed generates both keypairs.

### Keypair Serialization

```rust
pub struct IdentityKeypair {
    pub ed25519_private: [u8; 32],
    pub ed25519_public: [u8; 32],
    pub x25519_private: [u8; 32],
    pub x25519_public: [u8; 32],
}
```

**Protobuf encoding:** For Shamir Secret Sharing, the keypair is serialized using protobuf:
```
keypair_bytes = ed25519_keypair.to_protobuf_encoding()
```

---

## Local Persistence

### Encrypted Storage

The identity keypair is stored locally in an encrypted database.

| Property | Value |
|----------|-------|
| **Encryption** | AES-256-GCM |
| **Key derivation** | Argon2id (64 MiB memory, 3 iterations, 4 lanes) |
| **Database** | SQLite (encrypted via SQLCipher or equivalent) |
| **Key storage** | mlock'd memory (prevents swap to disk) |

### Encryption Process

```
1. User enters PIN/passphrase
2. salt = read_salt_from_database_header()
3. encryption_key = Argon2id(pin, salt, memory=64MB, iterations=3, lanes=4, output=32)
4. database = AES-256-GCM_decrypt(encryption_key, encrypted_database)
5. keypair = deserialize(database.identity_table)
6. zeroize(pin)  // Clear PIN from memory
```

### Lock State

The database is locked (encrypted in memory) when:
- The application is not running.
- The user explicitly locks the application.
- Panic mode is triggered.
- The dead man's switch timeout expires.

**Auto-lock timeout:** Configurable (default: 5 minutes of inactivity).

---

## Device Pairing

Device pairing enables a single GhostWire identity to be used across multiple devices.

### Pairing Methods

| Method | Description | Use Case |
|--------|-------------|----------|
| **QR code** | Scan QR code displayed on existing device | Same room, camera available |
| **Manual code** | Enter 16-character pairing code | No camera, or remote pairing |
| **LAN discovery** | mDNS-based automatic discovery | Same local network |
| **DHT rendezvous** | Meet at a DHT rendezvous point | Different networks |

### Pairing Protocol

1. **Existing device generates pairing code:**
   ```
   pairing_code = random_alphanumeric(16, charset="ABCDEFGHJKLMNPQRSTUVWXYZ23456789")
   formatted = XXXX-XXXX-XXXX-XXXX
   session_key = BLAKE3(pairing_code, output_len=32)
   ```

2. **Existing device displays QR code:** QR code contains the pairing code and the device's connection address.

3. **New device enters/scans pairing code:** Derives the same session key.

4. **Both devices connect:** Via LAN (mDNS) or DHT rendezvous.

5. **Mutual authentication:** Exchange identity public keys encrypted with the session key.

6. **Key transfer:** The existing device encrypts the identity keypair with the session key and sends it to the new device.

7. **Verification:** Both devices display the 6-digit safety number for user verification.

8. **Confirmation:** User confirms safety numbers match; pairing is complete.

### Multi-Device Sync

After pairing, devices sync:
- Contact list (peer IDs and display names).
- Social graph connections (WoT state).
- Message history (for messages received while the other device was offline).
- PRoPHET routing state.

**Conflict resolution:** Messages are merged using Hybrid Logical Clocks (HLC) for causal ordering. Conflicting social state changes are resolved by timestamp (latest wins).

---

## Shamir Secret Recovery

Shamir Secret Sharing enables recovery of the identity keypair without a central server.

### Scheme Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| **Total shares (T)** | 5 | Total number of recovery shares created |
| **Threshold (N)** | 3 | Minimum shares needed to reconstruct |
| **Scheme** | N-of-T Shamir Secret Sharing | Over a large prime field |
| **Encryption** | XChaCha20-Poly1305 | Each share encrypted with recovery password |

### Share Generation

```
1. keypair_bytes = serialize(identity_keypair)  // Protobuf encoding
2. shares = shamir_split(keypair_bytes, threshold=N, total=T)
3. For each share i:
   nonce = random_bytes(24)
   encrypted_share = XChaCha20-Poly1305_encrypt(
       key = argon2id(recovery_password, salt_i),
       nonce = nonce,
       plaintext = share_i
   )
   store(encrypted_share, nonce, salt_i)
```

### Share Distribution

Encrypted shares are distributed to:
- **Guardians:** Trusted contacts who hold shares on behalf of the user.
- **DHT:** Shares stored at rendezvous points derived from the user's public key.
- **Local backup:** Encrypted file stored on USB drive or printed as QR codes.

### Recovery Process

```
1. User provides recovery_password
2. Collect N encrypted shares from guardians/DHT/backup
3. For each share:
   key = argon2id(recovery_password, salt_i)
   share_i = XChaCha20-Poly1305_decrypt(key, nonce_i, encrypted_share_i)
4. keypair_bytes = shamir_reconstruct(shares[0..N-1])
5. identity_keypair = deserialize(keypair_bytes)
6. Generate revocation certificate for old keypair (if compromised)
7. Generate new keypair and re-establish connections
```

**Security property:** Fewer than N shares reveal no information about the secret (information-theoretic security). An attacker must compromise at least N guardians or storage locations.

---

## Coercion Resistance

### Duress PIN

Users configure two PINs:

| PIN | Function |
|-----|----------|
| **Primary PIN** | Unlocks the real identity keypair and all data |
| **Duress PIN** | Unlocks a decoy identity with independent, plausible data |

**Key derivation:**
```
primary_namespace_key = Argon2id(primary_pin, salt_primary, memory=64MB, iterations=3)
duress_namespace_key = Argon2id(duress_pin, salt_duress, memory=64MB, iterations=3)
```

**Independent namespaces:** The primary and duress namespaces are fully separated:
- Different Ed25519 + X25519 keypairs.
- Different peer IDs and DIDs.
- Different contact lists and message histories.
- Different DHT rendezvous points.

**Constant-time derivation:** Both PIN derivations use identical parameters and constant-time comparison to prevent timing side-channel attacks:

```rust
fn verify_pin_constant_time(input: &[u8], stored_hash: &[u8]) -> bool {
    let input_hash = argon2id_hash(input, /* same params as stored */);
    subtle::ConstantTimeEq::ct_eq(&input_hash, stored_hash).into()
}
```

### Panic Mode

Panic mode provides rapid data protection:

1. **Distress alert:** Send encrypted alert to trusted guardians via Sphinx route.
2. **Zeroize:** Overwrite in-memory key material using `zeroize` crate (mlock'd memory).
3. **Re-lock:** Re-encrypt local database with a fresh random key.

**Trigger:** User-configurable gesture/shortcut or panic PIN entry.

### Dead Man's Switch

| Property | Value |
|----------|-------|
| **Heartbeat interval** | 24 hours (configurable) |
| **Grace period** | 72 hours (configurable) |
| **Action on timeout** | Notify guardians; guardians may initiate Shamir recovery |

**Heartbeat mechanism:**
- The client sends an encrypted heartbeat message to each guardian every 24 hours.
- If a guardian does not receive a heartbeat within the grace period, they are authorized to initiate recovery.
- The user can reset the heartbeat timer at any time by sending a manual heartbeat.

---

## Contact Cards

### Format

Contact cards are shared as `gw2:` codes (see Discovery spec).

```
gw2:<base58(pubkey_32bytes || checksum_4bytes || version_1byte)>
```

### QR Code Encoding

Contact cards can be encoded as QR codes:
- **Error correction:** Level M (15% recovery capability).
- **Minimum version:** QR version 4 (for ~44 character base58 string).
- **Content format:** Plain text `gw2:...` string.

### Contact Card Metadata

Optionally, contact cards can include metadata:

```rust
pub struct ContactCard {
    pub pubkey: [u8; 32],              // Ed25519 public key
    pub display_name: Option<String>,  // User-chosen name
    pub avatar_hash: Option<[u8; 32]>, // BLAKE3 hash of avatar image
    pub personal_message: Option<String>, // Short status message
    pub signature: [u8; 64],           // Ed25519 signature over card
}
```

---

## Security Properties

| Property | Mechanism |
|----------|-----------|
| **Identity uniqueness** | Cryptographic keypair (2^256 key space) |
| **Identity binding** | Ed25519 signatures on all identity documents |
| **Local data protection** | AES-256-GCM + Argon2id |
| **Recovery security** | Shamir Secret Sharing (N-of-T, information-theoretic) |
| **Share confidentiality** | XChaCha20-Poly1305 encryption of each share |
| **Coercion resistance** | Duress PIN with independent namespaces |
| **Timing attack resistance** | Constant-time PIN comparison |
| **Memory safety** | zeroize crate, mlock'd memory |
| **Multi-device security** | Session key encryption + safety number verification |

### Threat Model

| Adversary | Capability | Protection |
|-----------|-----------|------------|
| **Device seizure** | Physical access to device | Encrypted database; duress PIN provides plausible deniability |
| **Coercion** | Forces user to unlock device | Duress PIN unlocks decoy identity; real data inaccessible |
| **Guardian compromise** | Fewer than N guardians collude | Shamir Secret Sharing reveals nothing with <N shares |
| **Recovery password guessing** | Offline brute-force attack | Argon2id (64 MiB, 3 iterations) makes guessing computationally expensive |
| **Memory forensics** | Reads RAM after unlock | zeroize + mlock prevents key material in swap or core dumps |
| **Dead man's switch abuse** | Prevents user from sending heartbeat | Grace period (72h) gives user time to act; guardians notified only after timeout |

---

## Reference Implementation

See the following modules in the GhostWire reference implementation:

| Module | Path |
|--------|------|
| Identity keypair | `src/core/identity/keypair.rs` |
| DID generation | `src/core/identity/did.rs` |
| Peer ID | `src/core/identity/peer_id.rs` |
| Local persistence | `src/core/identity/storage.rs` |
| Device pairing | `src/core/discovery/pairing.rs` |
| Shamir recovery | `src/core/identity/recovery.rs` |
| Coercion resistance | `src/core/coercion_resistance.rs` |
| Contact cards | `src/core/identity/contact_card.rs` |
| Dead man's switch | `src/core/identity/dead_mans_switch.rs` |
---

> **Lawful Use Only:** GhostWire is designed for lawful communication in connectivity-constrained environments. Users are responsible for complying with all applicable laws and regulations in their jurisdiction. The developers assume no liability for misuse of this software.

---
