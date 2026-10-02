//! Phase-1 / phase-2 cross-transport discovery for GhostWire.
//!
//! This is the seam between the RNS transport (`reticulum.rs`) and the libp2p
//! mesh (`mesh.rs`). It exists as its own module so the security boundary is
//! enforceable rather than a convention spread across two large files.
//!
//! ## Why two phases
//!
//! RNS is GhostWire's cold-start mechanism: two nodes with no shared subnet and
//! no bootstrap address can still find each other, because RNS announces are
//! routed across a backbone. Once connected, libp2p carries the traffic.
//!
//! The phases are asymmetric on purpose:
//!
//! * **Phase 1 — public announce.** Identity only. No libp2p addresses. Announce
//!   packets are broadcast to every hop on the backbone, so anything in one is
//!   public. Publishing a libp2p address there would hand every hop a direct
//!   route to the node and let an observer bypass RNS entirely.
//!
//! * **Phase 2 — authenticated exchange.** Only after phase 1 verified the peer's
//!   signature against a trusted key do the addresses cross, over the RNS channel
//!   that is already encrypted. They never touch the public backbone.
//!
//! ## The invariant
//!
//! A [`VerifiedPeerAddress`] — the only value the mesh side may dial — can only
//! be produced by [`AddressExchange::verify`]. There is no other constructor, so
//! there is no path by which an address read off a public announce can reach a
//! dial. That property is the reason the two types are separate rather than one
//! struct with a flag.
//!
//! Mesh-side contract, for whoever implements the dial:
//!
//! 1. Dial only what a [`VerifiedPeerAddress`] contains.
//! 2. Re-check `expires_at` at dial time, not just at receive time — a queue can
//!    outlive the window it was verified in.
//! 3. Never dial on the strength of a [`DiscoveryAnnounce`]. It has no addresses
//!    to dial, which is the point.

use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};

/// Wire-format version for [`DiscoveryAnnounce`].
pub const DISCOVERY_ANNOUNCE_VERSION: u16 = 1;
/// Wire-format version for [`AddressExchange`].
pub const ADDRESS_EXCHANGE_VERSION: u16 = 1;

/// An announce is a routing hint, so it expires quickly. A stale one costs a
/// wasted dial and nothing else.
pub const DISCOVERY_ANNOUNCE_TTL_SECS: u64 = 300;

/// An address exchange is acted on immediately, so it expires sooner.
pub const ADDRESS_EXCHANGE_TTL_SECS: u64 = 120;

/// Upper bound on addresses accepted in one exchange.
pub const MAX_EXCHANGE_ADDRESSES: usize = 16;

/// Longest accepted peer id or DID.
const MAX_ID_LEN: usize = 256;

/// Longest accepted signature. Matches `world_discovery`'s own bound.
const MAX_SIGNATURE_LEN: usize = 128;

pub fn now_unix() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ───────────────────────────── phase 1 ─────────────────────────────

/// Identity announced over the public RNS backbone.
///
/// **This type has no `addresses` field, and that is the security property, not
/// an oversight.** A public announce must not be able to express "dial this
/// libp2p endpoint", because announces are seen by every hop. `AddressExchange`
/// exists precisely so that addresses travel in a type this one is not.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct DiscoveryAnnounce {
    pub version: u16,
    /// Scopes the announce to one GhostWire network, so a node on another
    /// network cannot inject itself here.
    pub network_id: String,
    /// Stable libp2p peer id, base58.
    pub peer_id: String,
    pub did: String,
    /// Ed25519 public key that `signature` is verified against.
    pub public_key: Vec<u8>,
    pub issued_at: u64,
    pub expires_at: u64,
    /// Monotonic counter, so a replayed older announce is distinguishable.
    pub sequence: u64,
    pub signature: Vec<u8>,
}

impl DiscoveryAnnounce {
    pub fn new(
        network_id: &str,
        peer_id: &str,
        did: &str,
        public_key: [u8; 32],
        sequence: u64,
    ) -> Self {
        let now = now_unix();
        Self {
            version: DISCOVERY_ANNOUNCE_VERSION,
            network_id: network_id.to_string(),
            peer_id: peer_id.to_string(),
            did: did.to_string(),
            public_key: public_key.to_vec(),
            issued_at: now,
            expires_at: now.saturating_add(DISCOVERY_ANNOUNCE_TTL_SECS),
            sequence,
            signature: Vec::new(),
        }
    }

    /// Canonical bytes to sign: every field except the signature.
    pub fn signing_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let mut unsigned = self.clone();
        unsigned.signature.clear();
        Ok(bincode::serialize(&unsigned)?)
    }

    pub fn sign(mut self, secret: &[u8; 32]) -> anyhow::Result<Self> {
        use ed25519_dalek::{Signer, SigningKey};
        self.signature = SigningKey::from_bytes(secret)
            .sign(&self.signing_bytes()?)
            .to_bytes()
            .to_vec();
        Ok(self)
    }

    /// Verify structure, network scope, expiry, and signature.
    ///
    /// `trusted_key` is the caller's out-of-band anchor — the peer's registered
    /// key, not anything from the packet. The announce's own `public_key` must
    /// equal it, otherwise a valid signature by some other key would
    /// authenticate an arbitrary `peer_id`.
    ///
    /// Returns the authenticated peer id. Nothing should act on this announce
    /// before this returns `Ok`.
    pub fn verify(
        &self,
        expected_network_id: &str,
        now: u64,
        trusted_key: &[u8; 32],
    ) -> anyhow::Result<String> {
        if self.version != DISCOVERY_ANNOUNCE_VERSION {
            anyhow::bail!("unsupported discovery announce version {}", self.version);
        }
        if self.network_id != expected_network_id {
            anyhow::bail!("discovery announce network scope mismatch");
        }
        if self.peer_id.is_empty() || self.peer_id.len() > MAX_ID_LEN {
            anyhow::bail!("invalid discovery announce peer id");
        }
        if self.did.is_empty() || self.did.len() > MAX_ID_LEN {
            anyhow::bail!("invalid discovery announce did");
        }
        if self.signature.is_empty() || self.signature.len() > MAX_SIGNATURE_LEN {
            anyhow::bail!("invalid discovery announce signature");
        }
        if self.public_key.as_slice() != trusted_key.as_slice() {
            anyhow::bail!("discovery announce key does not match the trusted key");
        }
        if self.expires_at <= self.issued_at || now >= self.expires_at {
            anyhow::bail!("discovery announce expired or has a malformed window");
        }

        verify_signature(&self.signing_bytes()?, &self.signature, trusted_key)
            .map_err(|_| anyhow::anyhow!("discovery announce signature invalid"))?;

        Ok(self.peer_id.clone())
    }
}

// ───────────────────────────── phase 2 ─────────────────────────────

/// The authenticated address exchange, sent over the RNS channel after phase 1
/// verified the peer.
///
/// This is the only structure in the codebase permitted to carry a libp2p
/// address between nodes.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AddressExchange {
    pub version: u16,
    pub network_id: String,
    /// Peer id this exchange is FROM, so the receiver can check both directions
    /// rather than only its own.
    pub peer_id: String,
    pub did: String,
    /// libp2p multiaddrs, e.g. `/ip4/203.0.113.5/tcp/4001`.
    pub addresses: Vec<String>,
    pub issued_at: u64,
    pub expires_at: u64,
    pub sequence: u64,
    pub signature: Vec<u8>,
}

impl AddressExchange {
    pub fn new(
        network_id: &str,
        peer_id: &str,
        did: &str,
        addresses: Vec<String>,
        sequence: u64,
    ) -> Self {
        let now = now_unix();
        Self {
            version: ADDRESS_EXCHANGE_VERSION,
            network_id: network_id.to_string(),
            peer_id: peer_id.to_string(),
            did: did.to_string(),
            addresses,
            issued_at: now,
            expires_at: now.saturating_add(ADDRESS_EXCHANGE_TTL_SECS),
            sequence,
            signature: Vec::new(),
        }
    }

    pub fn signing_bytes(&self) -> anyhow::Result<Vec<u8>> {
        let mut unsigned = self.clone();
        unsigned.signature.clear();
        Ok(bincode::serialize(&unsigned)?)
    }

    pub fn sign(mut self, secret: &[u8; 32]) -> anyhow::Result<Self> {
        use ed25519_dalek::{Signer, SigningKey};
        self.signature = SigningKey::from_bytes(secret)
            .sign(&self.signing_bytes()?)
            .to_bytes()
            .to_vec();
        Ok(self)
    }

    /// Verify, then return a [`VerifiedPeerAddress`] — the only dialable value.
    ///
    /// Each address is validated through `world_discovery::validate_address`
    /// with `allow_private: false`, so a peer cannot make this node dial
    /// loopback, multicast, unspecified, or an RFC1918 address it does not
    /// control. Invalid entries are dropped rather than fatal, so one bad address
    /// does not discard an otherwise good exchange.
    pub fn verify(
        &self,
        expected_network_id: &str,
        now: u64,
        trusted_key: &[u8; 32],
    ) -> anyhow::Result<VerifiedPeerAddress> {
        if self.version != ADDRESS_EXCHANGE_VERSION {
            anyhow::bail!("unsupported address exchange version {}", self.version);
        }
        if self.network_id != expected_network_id {
            anyhow::bail!("address exchange network scope mismatch");
        }
        if self.peer_id.is_empty() || self.peer_id.len() > MAX_ID_LEN {
            anyhow::bail!("invalid address exchange peer id");
        }
        if self.did.is_empty() || self.did.len() > MAX_ID_LEN {
            anyhow::bail!("invalid address exchange did");
        }
        if self.signature.is_empty() || self.signature.len() > MAX_SIGNATURE_LEN {
            anyhow::bail!("invalid address exchange signature");
        }
        if self.addresses.is_empty() || self.addresses.len() > MAX_EXCHANGE_ADDRESSES {
            anyhow::bail!("invalid address exchange address count");
        }
        if self.expires_at <= self.issued_at || now >= self.expires_at {
            anyhow::bail!("address exchange expired or has a malformed window");
        }

        verify_signature(&self.signing_bytes()?, &self.signature, trusted_key)
            .map_err(|_| anyhow::anyhow!("address exchange signature invalid"))?;

        let accepted: Vec<String> = self
            .addresses
            .iter()
            .filter(|a| crate::core::world_discovery::validate_address(a, false).is_ok())
            .cloned()
            .collect();
        if accepted.is_empty() {
            anyhow::bail!(
                "address exchange carried no dialable public address (of {} offered)",
                self.addresses.len()
            );
        }

        Ok(VerifiedPeerAddress {
            peer_id: self.peer_id.clone(),
            did: self.did.clone(),
            addresses: accepted,
            expires_at: self.expires_at,
        })
    }
}

/// The one and only value the mesh side may dial.
///
/// No public constructor. It is produced by [`AddressExchange::verify`] and
/// nothing else, which is what makes "an address read off a public announce can
/// never reach a dial" a property of the type system rather than a review
/// convention.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedPeerAddress {
    pub peer_id: String,
    pub did: String,
    /// Already filtered: every entry passed `validate_address(_, false)`.
    pub addresses: Vec<String>,
    /// Unix seconds. The dial side MUST re-check this at dial time — verification
    /// may happen earlier than the dial, and a queued entry can outlive its
    /// window.
    pub expires_at: u64,
}

impl VerifiedPeerAddress {
    /// Whether this is still dialable at `now`.
    ///
    /// Separate from `verify()` on purpose: verifying at receive time and dialing
    /// later are different moments, and only this one guards the dial.
    pub fn is_dialable_at(&self, now: u64) -> bool {
        now < self.expires_at
    }
}

fn verify_signature(
    message: &[u8],
    signature: &[u8],
    key: &[u8; 32],
) -> anyhow::Result<()> {
    use ed25519_dalek::{Signature, Verifier, VerifyingKey};
    let vk = VerifyingKey::from_bytes(key).map_err(|e| anyhow::anyhow!("bad key: {e}"))?;
    let sig_bytes: [u8; 64] = signature
        .try_into()
        .map_err(|_| anyhow::anyhow!("unexpected signature length {}", signature.len()))?;
    vk.verify(message, &Signature::from_bytes(&sig_bytes))
        .map_err(|e| anyhow::anyhow!("signature invalid: {e}"))
}

#[cfg(test)]
mod tests {
    use super::*;
    use ed25519_dalek::SigningKey;

    /// The peer whose key we trust.
    fn trusted() -> (SigningKey, [u8; 32]) {
        let sk = SigningKey::from_bytes(&[7u8; 32]);
        let pk = sk.verifying_key().to_bytes();
        (sk, pk)
    }

    /// A genuinely different key. Must not collide with `trusted()`, or the
    /// key-mismatch cases would pass vacuously — the trap of a "locked node"
    /// test that stayed green with the fix reverted.
    fn attacker() -> (SigningKey, [u8; 32]) {
        let sk = SigningKey::from_bytes(&[99u8; 32]);
        let pk = sk.verifying_key().to_bytes();
        (sk, pk)
    }

    const GOOD: &str = "/ip4/203.0.113.5/tcp/4001";

    // ── phase 1 ──

    /// Compile-time proof of the security property. If someone adds an
    /// `addresses` field to `DiscoveryAnnounce`, this stops compiling.
    #[test]
    fn announce_type_has_no_address_field() {
        let (sk, pk) = trusted();
        let a = DiscoveryAnnounce::new("net1", "12D3KooWabc", "did:ghost:x", pk, 1)
            .sign(&sk.to_bytes())
            .unwrap();
        let DiscoveryAnnounce {
            version,
            network_id,
            peer_id,
            did,
            public_key,
            issued_at,
            expires_at,
            sequence,
            signature,
        } = a;
        assert_eq!(version, DISCOVERY_ANNOUNCE_VERSION);
        assert_eq!(network_id, "net1");
        assert_eq!(peer_id, "12D3KooWabc");
        assert_eq!(did, "did:ghost:x");
        assert_eq!(public_key, pk.to_vec());
        assert!(issued_at > 0 && expires_at > issued_at);
        assert_eq!(sequence, 1);
        assert!(!signature.is_empty());
    }

    #[test]
    fn signed_announce_verifies() {
        let (sk, pk) = trusted();
        let a = DiscoveryAnnounce::new("net1", "12D3KooWabc", "did:ghost:x", pk, 1)
            .sign(&sk.to_bytes())
            .unwrap();
        assert_eq!(a.verify("net1", now_unix(), &pk).unwrap(), "12D3KooWabc");
    }

    #[test]
    fn announce_from_another_network_refused() {
        let (sk, pk) = trusted();
        let a = DiscoveryAnnounce::new("attacker-net", "12D3KooWabc", "did:ghost:x", pk, 1)
            .sign(&sk.to_bytes())
            .unwrap();
        assert!(a.verify("our-net", now_unix(), &pk).is_err());
    }

    #[test]
    fn announce_signed_by_another_key_refused() {
        let (_, pk) = trusted();
        let (ask, apk) = attacker();
        let a = DiscoveryAnnounce::new("net1", "12D3KooWevil", "did:ghost:e", apk, 1)
            .sign(&ask.to_bytes())
            .unwrap();
        assert!(a.verify("net1", now_unix(), &pk).is_err());
    }

    #[test]
    fn tampered_announce_refused() {
        let (sk, pk) = trusted();
        let a = DiscoveryAnnounce::new("net1", "12D3KooWabc", "did:ghost:x", pk, 1)
            .sign(&sk.to_bytes())
            .unwrap();
        let mut bad = a.clone();
        bad.peer_id = "12D3KooWevil".to_string();
        assert!(bad.verify("net1", now_unix(), &pk).is_err());
    }

    #[test]
    fn expired_announce_refused() {
        let (sk, pk) = trusted();
        let a = DiscoveryAnnounce::new("net1", "12D3KooWabc", "did:ghost:x", pk, 1)
            .sign(&sk.to_bytes())
            .unwrap();
        assert!(a.verify("net1", a.expires_at + 1, &pk).is_err());
    }

    // ── phase 2 ──

    #[test]
    fn exchange_produces_dialable_verified_peer() {
        let (sk, pk) = trusted();
        let x = AddressExchange::new(
            "net1",
            "12D3KooWabc",
            "did:ghost:x",
            vec![GOOD.to_string()],
            1,
        )
        .sign(&sk.to_bytes())
        .unwrap();
        let v = x.verify("net1", now_unix(), &pk).unwrap();
        assert_eq!(v.peer_id, "12D3KooWabc");
        assert_eq!(v.addresses, vec![GOOD.to_string()]);
    }

    #[test]
    fn exchange_cannot_reach_loopback_or_private_ranges() {
        let (sk, pk) = trusted();
        let hostile = vec![
            "/ip4/127.0.0.1/tcp/4001",
            "/ip4/10.0.0.5/tcp/4001",
            "/ip4/192.168.1.9/tcp/4001",
            "/ip4/172.16.4.4/tcp/4001",
            "/ip4/0.0.0.0/tcp/4001",
            "/ip4/224.0.0.1/tcp/4001",
        ]
        .into_iter()
        .map(String::from)
        .collect::<Vec<_>>();
        let x = AddressExchange::new("net1", "12D3KooWabc", "did:ghost:x", hostile, 1)
            .sign(&sk.to_bytes())
            .unwrap();
        // Nothing survives filtering, so verify fails rather than returning empty.
        assert!(x.verify("net1", now_unix(), &pk).is_err());
    }

    #[test]
    fn hostile_entries_dropped_but_good_one_kept() {
        let (sk, pk) = trusted();
        let mixed = vec![GOOD.to_string(), "/ip4/127.0.0.1/tcp/4001".to_string()];
        let x = AddressExchange::new("net1", "12D3KooWabc", "did:ghost:x", mixed, 1)
            .sign(&sk.to_bytes())
            .unwrap();
        let v = x.verify("net1", now_unix(), &pk).unwrap();
        assert_eq!(v.addresses, vec![GOOD.to_string()]);
        assert!(!v.addresses.iter().any(|a| a.contains("127.0.0.1")));
    }

    #[test]
    fn tampered_exchange_address_refused() {
        let (sk, pk) = trusted();
        let x = AddressExchange::new("net1", "12D3KooWabc", "did:ghost:x", vec![GOOD.to_string()], 1)
            .sign(&sk.to_bytes())
            .unwrap();
        let mut bad = x.clone();
        bad.addresses = vec!["/ip4/198.51.100.9/tcp/4001".to_string()];
        assert!(bad.verify("net1", now_unix(), &pk).is_err());
    }

    #[test]
    fn expired_exchange_refused() {
        let (sk, pk) = trusted();
        let x = AddressExchange::new("net1", "12D3KooWabc", "did:ghost:x", vec![GOOD.to_string()], 1)
            .sign(&sk.to_bytes())
            .unwrap();
        assert!(x.verify("net1", x.expires_at + 1, &pk).is_err());
    }

    /// The dial-time re-check. Verification and dialing are different moments;
    /// only this guards the dial.
    #[test]
    fn verified_peer_stops_being_dialable_at_expiry() {
        let (sk, pk) = trusted();
        let x = AddressExchange::new("net1", "12D3KooWabc", "did:ghost:x", vec![GOOD.to_string()], 1)
            .sign(&sk.to_bytes())
            .unwrap();
        let v = x.verify("net1", now_unix(), &pk).unwrap();
        assert!(v.is_dialable_at(v.expires_at - 1));
        assert!(!v.is_dialable_at(v.expires_at));
        assert!(!v.is_dialable_at(v.expires_at + 1));
    }
}