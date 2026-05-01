# vodozemac crate documentation

## Overview

vodozemac is a Rust implementation of Olm and Megolm cryptographic ratchets, used for end-to-end encryption in Matrix.

## License

Apache-2.0

## Source

https://github.com/matrix-org/vodozemac

## Security

Audited by Least Authority - https://matrix.org/media/Least%20Authority%20-%20Matrix%20vodozemac%20Final%20Audit%20Report.pdf

## Technical Specifications

- Olm spec: https://gitlab.matrix.org/matrix-org/olm/-/blob/master/docs/olm.md
- Megolm spec: https://gitlab.matrix.org/matrix-org/olm/-/blob/master/docs/megolm.md

---

## Olm Module - Double Ratchet

### Account Struct

Manages all cryptographic keys used on a device.

An Olm Account consists of:

1. Ed25519 signing key pair (fingerprint)
2. Curve25519 sender key pair (identity key)
3. One-time key pairs
4. Fallback key pair

#### Creating a new account

```rust
use vodozemac::olm::Account;

let account = Account::new();
```

#### Getting identity keys

```rust
let identity_keys = account.identity_keys();
let ed25519_key = account.ed25519_key();
let curve25519_key = account.curve25519_key();
```

#### Creating outbound session

```rust
use vodozemac::olm::{Account, SessionConfig};

// Generate OTKs
account.generate_one_time_keys(5);
account.mark_keys_as_published();

// Create session with peer's keys
let mut session = account.create_outbound_session(
    SessionConfig::version_1(),
    peer_identity_key,
    peer_one_time_key
)?;
```

#### Creating inbound session

```rust
let result = account.create_inbound_session(
    SessionConfig::version_1(),
    our_identity_key,
    prekey_message
)?;
let session = result.session;
```

### Session Struct

Represents one end of an encrypted communication channel.

#### Encrypting messages

```rust
let encrypted = session.encrypt("Hello, secure world!")?;
// encrypted is OlmMessage::PreKey (first message) or OlmMessage::Normal
```

#### Decrypting messages

```rust
let plaintext = session.decrypt(&encrypted_message)?;
```

#### Pickling (serialization)

```rust
let pickle = session.pickle();
// Serialize to JSON, save to disk, etc.
```

---

## Megolm Module - Group Encryption

Megolm is an AES-based single ratchet for group conversations.

### Usage

```rust
use vodozemac::megolm::{GroupSession, InboundGroupSession};

// Sender creates session
let mut group_session = GroupSession::new();

// Encrypt for group
let message = group_session.encrypt("Hello group!")?;

// Receiver creates inbound session from session key
let inbound = InboundGroupSession::from_session_key(&session_key)?;
let plaintext = inbound.decrypt(&message)?;
```

---

## Key Types

### Curve25519PublicKey

Public key for X25519 key exchange.

### Ed25519PublicKey

Public key for Ed25519 digital signatures.

### Ed25519Signature

Digital signature that can verify message authenticity.

---

## Error Types

### SessionCreationError

Errors when creating Olm sessions:

- MismatchedIdentityKey
- MismatchedSessionConfig
- InvalidSignature
- LibraryError

### EncryptionError

Errors during message encryption.

### DecryptionError

Errors during message decryption.

---

## Feature Flags

### default

Basic Olm/Megolm support.

### libolm-compat

Support for reading libolm pickles.

### low-level-api

Advanced cryptographic functions (not recommended for general use).

---

## Pickling (Serialization)

Modern pickle format uses serde:

```rust
// Save account state
let pickle = account.pickle().encrypt(&pickle_key);
let json = serde_json::to_string(&pickle)?;

// Restore account state
let pickle: AccountPickle = serde_json::from_str(&json)?;
let account: Account = pickle.into();
```

---

## Dependencies

- aes
- chacha20poly1305
- curve25519-dalek
- ed25519-dalek
- getrandom
- hmac
- serde
- sha2
- x25519-dalek
- zeroize
