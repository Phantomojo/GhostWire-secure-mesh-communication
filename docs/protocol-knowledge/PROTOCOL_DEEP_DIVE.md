# Protocol Knowledge Base

**Version:** 1.0  
**Last Updated:** March 2026

---

## Overview

This document provides deep technical knowledge about the protocols used in GhostWire, including implementation details, wire formats, and best practices.

---

## Table of Contents

1. [libp2p Stack](#libp2p-stack)
2. [Cryptography](#cryptography)
3. [Stealth Transport](#stealth-transport)
4. [Mesh Networking](#mesh-networking)
5. [Protocol Design Patterns](#protocol-design-patterns)

---

## libp2p Stack

### Kademlia DHT

#### Key Concepts

**Peer ID Generation:**
```
PeerID = Blake2b-256(PublicKey)
Length: 32 bytes (base58btc encoded for display)
```

**XOR Distance Metric:**
```
distance(A, B) = SHA256(A) XOR SHA256(B)

Properties:
- Symmetric: distance(A,B) = distance(B,A)
- Triangle inequality: distance(A,C) ≤ distance(A,B) + distance(B,C)
- Locality: Close IDs have small XOR distance
```

**K-Bucket Structure:**
```
Bucket i contains peers where:
- Distance is in range [2^i, 2^(i+1))
- Maximum 20 peers per bucket (k=20)
- Buckets organized by distance from local node

Bucket 0: [1, 2)
Bucket 1: [2, 4)
Bucket 2: [4, 8)
...
Bucket 255: [2^255, 2^256)
```

**RPC Messages:**

```rust
// Wire format (protobuf)
message KademliaRPC {
    oneof request {
        FIND_NODE request_find_node = 1;
        FIND_VALUE request_find_value = 2;
        ADD_PROVIDER request_add_provider = 3;
        GET_PROVIDERS request_get_providers = 4;
        PUT_VALUE request_put_value = 5;
        GET_VALUE request_get_value = 6;
    }
    
    oneof response {
        FIND_NODE_RESPONSE response_find_node = 11;
        FIND_VALUE_RESPONSE response_find_value = 12;
        ADD_PROVIDER_RESPONSE response_add_provider = 13;
        GET_PROVIDERS_RESPONSE response_get_providers = 14;
        PUT_VALUE_RESPONSE response_put_value = 15;
        GET_VALUE_RESPONSE response_get_value = 16;
    }
}

message PeerInfo {
    bytes id = 1;           // Peer ID
    repeated bytes multiaddrs = 2;
}

message FIND_NODE {
    bytes key = 1;          // Target Peer ID
}

message FIND_NODE_RESPONSE {
    repeated PeerInfo closer_peers = 1;
}
```

**Query Algorithm:**
```
1. Start with α=3 closest peers from local routing table
2. Send FIND_NODE to all α peers in parallel
3. Collect responses, merge closer peers
4. Select α closest unqueried peers
5. Repeat until no closer peers found
6. Return k closest peers discovered
```

**GhostWire Implementation:**
```rust
// ghostwire/src/core/mesh.rs
use libp2p::kad::{
    Kademlia,
    KademliaConfig,
    KademliaEvent,
    QueryResult,
    store::MemoryStore,
};

fn build_kademlia(local_peer_id: PeerId) -> Kademlia<MemoryStore> {
    let mut config = KademliaConfig::default();
    config.set_replication_factor(NonZeroUsize::new(20).unwrap());
    config.set_query_timeout(Duration::from_secs(60));
    config.set_parallelism(NonZeroUsize::new(3).unwrap());
    
    let store = MemoryStore::new(local_peer_id);
    Kademlia::new(local_peer_id, store, config)
}
```

---

### Gossipsub

#### Configuration Parameters

```rust
// Optimal values for GhostWire
pub struct GossipsubConfig {
    // Mesh topology
    pub d: usize = 6,              // Mesh degree (connections)
    pub d_lo: usize = 4,           // Low watermark
    pub d_hi: usize = 12,          // High watermark
    
    // Gossip propagation
    pub gossip_factor: f64 = 0.25, // Forward to 25% of peers
    pub heartbeat_interval: Duration = Duration::from_secs(1),
    
    // Message cache
    pub history_length: usize = 5,  // Heartbeats to remember
    pub history_gossip: usize = 3,  // Heartbeats to gossip
    
    // Validation
    pub validate_messages: bool = true,
    pub message_id_fn: fn(&Message) -> MessageId,
}
```

**Topic Schema:**

```rust
// GhostWire topics
pub const TOPIC_MESSAGES: &str = "/ghostwire/messages/1.0.0";
pub const TOPIC_EVENTS: &str = "/ghostwire/events/1.0.0";
pub const TOPIC_DISCOVERY: &str = "/ghostwire/discovery/1.0.0";

// Message wire format
pub struct GossipsubMessage {
    pub source: PeerId,
    pub sequence_number: u64,
    pub data: Vec<u8>,  // Encrypted payload
    pub timestamp: u64,
    pub signature: Signature,
}
```

**Peer Scoring:**

```rust
// GhostWire peer scoring
pub struct PeerScoreParams {
    pub ip_colocation_factor_weight: f64 = -0.5,
    pub ip_colocation_factor_threshold: f64 = 5.0,
    
    pub behaviour_penalty_weight: f64 = -0.2,
    pub behaviour_penalty_threshold: f64 = 10.0,
    
    pub message_deliveries_weight: f64 = 0.5,
    pub message_deliveries_decay: f64 = 0.9,
    
    pub app_specific_score: f64 = 0.0,
}
```

---

### Noise Protocol

**Handshake Flow:**

```
Initiator (I)                    Responder (R)
    |                              |
    |--------- Handshake --------->|
    |  (Noise_XX_25519_AES_GCM)    |
    |                              |
    | 1. e (ephemeral pubkey)      |
    |----------------------------->|
    |                              |
    | 2. e, s (ephemeral + static) |
    |<-----------------------------|
    |                              |
    | 3. s (static pubkey)         |
    |----------------------------->|
    |                              |
    |<====== Encrypted Traffic ===>|
```

**Implementation:**
```rust
use libp2p::noise::{
    Config,
    Keypair,
    X25519Spec,
};

fn build_noise_config() -> Config<X25519Spec> {
    let local_key = Keypair::generate();
    Config::new(libp2p::noise::Authenticating::Both)
        .with_dh_keys(local_key)
}
```

---

## Cryptography

### Key Derivation

**Argon2id Parameters:**
```rust
// GhostWire password-based key derivation
use argon2::{
    Argon2,
    Params,
    Version,
};

pub fn derive_key(password: &str, salt: &[u8]) -> Result<[u8; 32]> {
    let params = Params::new(
        65536,  // 64 MB memory
        3,      // 3 iterations
        4,      // 4 parallel lanes
        Some(32), // 32-byte output
        Version::V0x13,
    )?;
    
    let argon2 = Argon2::new(Algorithm::Argon2id, Version::V0x13, params);
    let mut key = [0u8; 32];
    argon2.hash_password_into(password.as_bytes(), salt, &mut key)?;
    Ok(key)
}
```

**PBKDF2 Parameters:**
```rust
use pbkdf2::{pbkdf2_hmac, Params};
use sha2::Sha256;

pub fn derive_key_pbkdf2(password: &str, salt: &[u8]) -> [u8; 32] {
    let mut key = [0u8; 32];
    pbkdf2_hmac::<Sha256>(
        password.as_bytes(),
        salt,
        100_000,  // 100k iterations
        &mut key,
    );
    key
}
```

### Encryption Patterns

**AES-GCM:**
```rust
use aes_gcm::{
    Aes256Gcm,
    Key, Nonce,
    aead::{Aead, KeyInit, OsRng},
};

pub fn encrypt_aes_gcm(plaintext: &[u8], key: &[u8; 32]) -> Result<(Vec<u8>, Nonce<Aes256Gcm>)> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng);
    
    let ciphertext = cipher.encrypt(&nonce, plaintext)?;
    Ok((ciphertext, nonce))
}

pub fn decrypt_aes_gcm(
    ciphertext: &[u8],
    key: &[u8; 32],
    nonce: &Nonce<Aes256Gcm>
) -> Result<Vec<u8>> {
    let cipher = Aes256Gcm::new(Key::<Aes256Gcm>::from_slice(key));
    Ok(cipher.decrypt(nonce, ciphertext)?)
}
```

**ChaCha20-Poly1305:**
```rust
use chacha20poly1305::{
    ChaCha20Poly1305,
    Key, Nonce,
    aead::{Aead, KeyInit, OsRng},
};

pub fn encrypt_chacha20(plaintext: &[u8], key: &[u8; 32]) -> Result<(Vec<u8>, Nonce<ChaCha20Poly1305>)> {
    let cipher = ChaCha20Poly1305::new(Key::<ChaCha20Poly1305>::from_slice(key));
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    
    let ciphertext = cipher.encrypt(&nonce, plaintext)?;
    Ok((ciphertext, nonce))
}
```

### Post-Quantum Cryptography

**ML-KEM (Kyber) Hybrid:**
```rust
use pqcrypto_mlkem::kyber768;

pub fn hybrid_kem_encrypt(
    classical_pubkey: &[u8],
    pqc_pubkey: &[u8],
    plaintext: &[u8]
) -> Result<(Vec<u8>, Vec<u8>)> {
    // Classical encryption (X25519)
    let classical_ct = x25519_encrypt(classical_pubkey, plaintext)?;
    
    // PQC encryption (ML-KEM-768)
    let pqc_ct = kyber768::encapsulate(pqc_pubkey)?;
    
    Ok((classical_ct, pqc_ct.0))
}
```

---

## Stealth Transport

### TCP Handshake Obfuscation

**Standard TCP Handshake:**
```
Normal: [SYN] -> [SYN-ACK] -> [ACK]
GhostWire: Same (indistinguishable at TCP level)
```

**Application Layer Obfuscation:**
```rust
// First packet appears as generic traffic
pub struct StealthHandshake {
    magic: [u8; 4],      // Looks like random data
    version: u8,         // Protocol version
    flags: u8,           // Feature flags
    peer_id_hash: [u8; 16], // Truncated hash of PeerID
    nonce: [u8; 32],     // Random nonce
    signature: [u8; 64], // Ed25519 signature
}

impl StealthHandshake {
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(120);
        buf.extend_from_slice(&self.magic);
        buf.push(self.version);
        buf.push(self.flags);
        buf.extend_from_slice(&self.peer_id_hash);
        buf.extend_from_slice(&self.nonce);
        buf.extend_from_slice(&self.signature);
        buf
    }
}
```

---

## Mesh Networking

### Peer Discovery Flow

```
1. mDNS Broadcast (local network)
   └──> Announce: "_ghostwire._tcp"
   └──> Include: PeerID, Port, Capabilities

2. Kademlia Bootstrap
   └──> Connect to known bootstrap nodes
   └──> Query for peers

3. Identify Protocol
   └──> Exchange protocol lists
   └──> Share agent version

4. AutoNAT Check
   └──> Verify public reachability
   └──> Determine NAT type

5. Relay Discovery
   └──> Find relay nodes if behind NAT
   └──> Reserve relay slot
```

---

## Protocol Design Patterns

### Message Envelope Pattern

```rust
// Generic message envelope
pub struct MessageEnvelope<T> {
    pub version: u8,
    pub timestamp: u64,
    pub sender: PeerId,
    pub recipient: Option<PeerId>,
    pub payload: T,
    pub signature: Signature,
}

impl<T: Serialize> MessageEnvelope<T> {
    pub fn sign(self, private_key: &SigningKey) -> Result<Self> {
        let payload_bytes = bincode::serialize(&self.payload)?;
        let signature = private_key.sign(&payload_bytes);
        Ok(Self {
            signature,
            ..self
        })
    }
}
```

### Request-Response Pattern

```rust
// Protocol request/response
#[derive(Serialize, Deserialize)]
pub enum ProtocolRequest {
    GetProfile { peer_id: PeerId },
    SendMessage { recipient: PeerId, content: String },
    ListPeers { page: u32, per_page: u32 },
}

#[derive(Serialize, Deserialize)]
pub enum ProtocolResponse {
    Profile(ProfileData),
    Ack { message_id: String },
    PeerList(Vec<PeerInfo>),
    Error { code: u16, message: String },
}
```

---

## Related Documents

- [ARCHITECTURE.md](../ARCHITECTURE.md)
- [SECURITY.md](../SECURITY.md)
- [GLOBAL_KNOWLEDGE_SOURCES.md](../GLOBAL_KNOWLEDGE_SOURCES.md)

---

**Maintained By:** GhostWire Protocol Team  
**License:** AGPL-3.0-or-later
