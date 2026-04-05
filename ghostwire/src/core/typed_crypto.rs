//! typed_crypto.rs — Veilid-inspired typed cryptographic keys
//!
//! Key lesson from Veilid: ALL cryptographic primitives are tagged with
//! a "crypto kind" identifier. This means you can upgrade the crypto system
//! (e.g. migrate from X25519 to ML-KEM) without breaking existing nodes.
//!
//! Veilid calls this VLD0 (their current crypto suite). We implement the
//! same versioning architecture so Ghostwire can do transparent migrations.
//!
//! Current suite (GW0):
//!   Auth:     Ed25519
//!   Exchange: X25519
//!   Encrypt:  ChaCha20-Poly1305
//!   Hash:     BLAKE3
//!   KDF:      Argon2id
//!   PQ:       ML-KEM-768 (hybrid with X25519)
//!
//! Future suite (GW1, when ready):
//!   Exchange: ML-KEM-1024 (standalone, post-quantum)
//!   Auth:     CRYSTALS-Dilithium (post-quantum signatures)

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fmt;

/// Crypto suite identifier — tags all keys and operations
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CryptoKind {
    /// Current production suite: X25519 + Ed25519 + ChaCha20 + BLAKE3
    GW0 = 0,
    /// Future: ML-KEM-1024 + Dilithium (post-quantum standalone)
    GW1 = 1,
}

impl CryptoKind {
    pub fn current() -> Self {
        Self::GW0
    }

    pub fn as_tag(&self) -> [u8; 4] {
        match self {
            Self::GW0 => *b"GW00",
            Self::GW1 => *b"GW01",
        }
    }

    pub fn from_tag(tag: &[u8; 4]) -> Option<Self> {
        match tag {
            b"GW00" => Some(Self::GW0),
            b"GW01" => Some(Self::GW1),
            _ => None,
        }
    }

    pub fn supports_pq(&self) -> bool {
        matches!(self, Self::GW0 | Self::GW1) // GW0 has hybrid PQ
    }
}

impl fmt::Display for CryptoKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::GW0 => write!(f, "GW0 (X25519+Ed25519+ChaCha20+BLAKE3)"),
            Self::GW1 => write!(f, "GW1 (ML-KEM-1024+Dilithium — future)"),
        }
    }
}

/// A typed public key — always carries its crypto kind
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedPublicKey {
    pub kind: CryptoKind,
    pub key: Vec<u8>,
}

impl TypedPublicKey {
    pub fn new_gw0_ed25519(bytes: Vec<u8>) -> Self {
        Self { kind: CryptoKind::GW0, key: bytes }
    }

    pub fn new_gw0_x25519(bytes: Vec<u8>) -> Self {
        Self { kind: CryptoKind::GW0, key: bytes }
    }

    /// Encode as wire format: [4-byte tag][4-byte len LE][key bytes]
    pub fn to_wire(&self) -> Vec<u8> {
        let tag = self.kind.as_tag();
        let mut wire = Vec::with_capacity(8 + self.key.len());
        wire.extend_from_slice(&tag);
        wire.extend_from_slice(&(self.key.len() as u32).to_le_bytes());
        wire.extend_from_slice(&self.key);
        wire
    }

    pub fn from_wire(bytes: &[u8]) -> Result<Self> {
        if bytes.len() < 8 {
            anyhow::bail!("Wire key too short: {} bytes", bytes.len());
        }
        let tag: [u8; 4] = bytes[..4].try_into()?;
        let kind = CryptoKind::from_tag(&tag)
            .ok_or_else(|| anyhow::anyhow!("Unknown crypto kind: {:?}", tag))?;
        let len = u32::from_le_bytes(bytes[4..8].try_into()?) as usize;
        if bytes.len() < 8 + len {
            anyhow::bail!("Wire key truncated: expected {} + 8 bytes", len);
        }
        Ok(Self {
            kind,
            key: bytes[8..8 + len].to_vec(),
        })
    }
}

/// A typed signature
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypedSignature {
    pub kind: CryptoKind,
    pub signature: Vec<u8>,
}

/// A typed hash
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct TypedHash {
    pub kind: CryptoKind,
    pub hash: [u8; 32],
}

impl TypedHash {
    pub fn blake3(data: &[u8]) -> Self {
        Self {
            kind: CryptoKind::GW0,
            hash: *blake3::hash(data).as_bytes(),
        }
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.hash)
    }
}

/// A Ghostwire node ID — derived from the Ed25519 public key via BLAKE3
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct NodeId {
    pub kind: CryptoKind,
    /// 32-byte BLAKE3 hash of the Ed25519 public key
    id: [u8; 32],
}

impl NodeId {
    pub fn from_ed25519_pub(pub_bytes: &[u8]) -> Self {
        let hash = blake3::hash(pub_bytes);
        Self {
            kind: CryptoKind::GW0,
            id: *hash.as_bytes(),
        }
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.id
    }

    pub fn to_hex(&self) -> String {
        hex::encode(self.id)
    }

    /// DID representation: did:ghost:<kind>:<hex>
    pub fn to_did(&self) -> String {
        format!("did:ghost:{}:{}", self.kind.as_tag().iter()
            .map(|b| *b as char)
            .collect::<String>()
            .to_lowercase(), self.to_hex())
    }

    /// XOR distance to another node (for Kademlia-style routing)
    pub fn xor_distance(&self, other: &NodeId) -> [u8; 32] {
        let mut dist = [0u8; 32];
        for i in 0..32 {
            dist[i] = self.id[i] ^ other.id[i];
        }
        dist
    }

    /// How many leading zero bits (used in Kademlia bucket selection)
    pub fn common_prefix_len(&self, other: &NodeId) -> u32 {
        let dist = self.xor_distance(other);
        let mut count = 0u32;
        for byte in dist {
            if byte == 0 {
                count += 8;
            } else {
                count += byte.leading_zeros();
                break;
            }
        }
        count
    }
}

impl fmt::Display for NodeId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}:{}", 
            self.kind.as_tag().iter().map(|b| *b as char).collect::<String>(),
            &self.to_hex()[..16]
        )
    }
}

/// Cryptographic key bundle for a Ghostwire identity
#[derive(Debug, Serialize, Deserialize)]
pub struct KeyBundle {
    pub kind: CryptoKind,
    pub node_id: NodeId,
    pub ed25519_pub: TypedPublicKey,  // For signing
    pub x25519_pub: TypedPublicKey,   // For encryption / DH
    /// Optional ML-KEM public key for post-quantum hybrid
    pub mlkem_pub: Option<TypedPublicKey>,
    pub created_at: u64,
}

impl KeyBundle {
    pub fn verify_self_consistent(&self) -> bool {
        let derived_id = NodeId::from_ed25519_pub(&self.ed25519_pub.key);
        derived_id == self.node_id
    }
}

// ── Cargo.toml deps ──────────────────────────────────────────────────────────
// blake3 = "1.5"  (already needed by mailbox and traffic_armor)
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn node_id_from_pub_key() {
        let pub_key = vec![0xAAu8; 32];
        let id = NodeId::from_ed25519_pub(&pub_key);
        assert_eq!(id.kind, CryptoKind::GW0);
        let did = id.to_did();
        assert!(did.starts_with("did:ghost:gw00:"));
    }

    #[test]
    fn typed_key_wire_roundtrip() {
        let key = TypedPublicKey::new_gw0_ed25519(vec![0xBBu8; 32]);
        let wire = key.to_wire();
        let recovered = TypedPublicKey::from_wire(&wire).unwrap();
        assert_eq!(recovered.key, key.key);
        assert_eq!(recovered.kind, key.kind);
    }

    #[test]
    fn xor_distance_same_is_zero() {
        let pub_key = vec![0xCC; 32];
        let id = NodeId::from_ed25519_pub(&pub_key);
        let dist = id.xor_distance(&id);
        assert_eq!(dist, [0u8; 32]);
    }

    #[test]
    fn common_prefix_len_same_is_256() {
        let pub_key = vec![0xDD; 32];
        let id = NodeId::from_ed25519_pub(&pub_key);
        assert_eq!(id.common_prefix_len(&id), 256);
    }

    #[test]
    fn crypto_kind_tag_roundtrip() {
        for kind in [CryptoKind::GW0, CryptoKind::GW1] {
            let tag = kind.as_tag();
            let recovered = CryptoKind::from_tag(&tag).unwrap();
            assert_eq!(recovered, kind);
        }
    }
}
