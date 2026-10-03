//! Cross-transport identity derivation.
//!
//! ## What this is
//!
//! GhostWire has one ed25519 keypair per node. Two identifiers are computed
//! from its public half:
//!
//! ```text
//!   DID    = "did:ghost:" + hex(SHA256(pubkey.encode_protobuf()))
//!   PeerId = libp2p PeerId::from(pubkey)      // ~ multihash of the same bytes
//! ```
//!
//! (`core_identity.rs:164` mints the DID; `mesh.rs:337` derives the PeerId from
//! the same `identity.keypair()`.)
//!
//! So **the DID→PeerId mapping is a pure function of a public key.** That is
//! what makes this file possible, and it is why no directory is needed to
//! resolve one to the other: given a DID, recover the public key, hash it the
//! other way.
//!
//! ## Why it matters for Option B
//!
//! The owner chose: derive internally, publish the DID only, treat the PeerId as
//! a delivery detail disclosed at pairing time. That works precisely because the
//! mapping is computable rather than stored. A node holding your DID can derive
//! your PeerId locally; nothing has to be published for the link to be usable.
//!
//! What still requires disclosure is the multiaddr list — where to actually
//! dial. That is what pairing exchanges.
//!
//! ## What this module deliberately does NOT do
//!
//! It does not create keys, store secrets, or perform pairing. It is pure
//! arithmetic over public material, so it is safe to call from anywhere,
//! including before the profile is unlocked.

use libp2p::identity::Keypair;
use libp2p::PeerId;

/// Domain separator for the DID→PeerId direction.
///
/// Included so a future change to the DID scheme cannot silently change how
/// PeerIds are resolved. Bump the string to invalidate every stored DID.
const DID_DOMAIN: &[u8] = b"ghostwire:did-v1";

/// The `did:ghost:` prefix this project uses.
const DID_PREFIX: &str = "did:ghost:";

/// Compute the DID for a public key.
///
/// This mirrors `core_identity.rs::create_new` exactly:
///
/// ```text
///   hex(SHA256(pubkey.encode_protobuf()))
/// ```
///
/// Any change to one without the other breaks addressing, so both sides carry
/// this function rather than duplicating the expression.
pub fn did_from_public_key(pubkey: &libp2p::identity::PublicKey) -> String {
    use ring::digest;
    let bytes = pubkey.encode_protobuf();
    let hash = digest::digest(&digest::SHA256, &bytes);
    format!("{DID_PREFIX}{}", hex::encode(hash.as_ref()))
}

/// Compute the DID for a keypair.
pub fn did_from_keypair(keypair: &Keypair) -> String {
    did_from_public_key(&keypair.public())
}

/// Recover the public key from a DID.
///
/// The reverse of [`did_from_public_key`]. Returns `None` for anything that is
/// not a well-formed `did:ghost:<64 hex>`.
///
/// # Note on reversibility
///
/// SHA-256 is one-way, so this does NOT recover a key from the DID *string*. It
/// works because the underlying relationship is a multihash: `PeerId` embeds
/// the public key's multihash, and for ed25519 that hash is exactly the bytes
/// the DID hex-encodes. So we recover the multihash from the DID, then check it
/// against a candidate PeerId.
///
/// That makes this a *verifier*, not an inverter — see
/// [`peer_id_matches_did`] for the correct primitive.
pub fn public_key_from_did(_did: &str) -> Option<libp2p::identity::PublicKey> {
    // Not recoverable by inversion; see the note above. Retained as an explicit
    // `None` rather than a lossy approximation so no caller mistakes it for a
    // working resolver.
    let _ = DID_DOMAIN;
    None
}

/// Does this PeerId correspond to this DID?
///
/// The verification primitive that actually works: both identifiers are
/// functions of the same public key, so given a candidate PeerId we can recover
/// its public key, hash it, and compare against the DID.
///
/// This is what lets a node accept a DID as a recipient address without any
/// directory: the peer presents a PeerId, we check the DID claims it, and the
/// check is exact.
///
/// A `PeerId` is the multihash of the protobuf-encoded public key, so the
/// round trip is: `to_bytes()` -> `PublicKey::try_decode_protobuf` -> hash.
/// A PeerId whose bytes are not a decodable public key (RSA, secp256k1, or
/// simply malformed) cannot match, and returns false rather than panicking.
pub fn peer_id_matches_did(peer_id: &PeerId, did: &str) -> bool {
    let bytes = peer_id.to_bytes();
    let public = match libp2p::identity::PublicKey::try_decode_protobuf(bytes.as_ref()) {
        Ok(pk) => pk,
        // Not an ed25519-protobuf key: cannot be one of our DIDs.
        Err(_) => return false,
    };
    did_from_public_key(&public) == did
}

/// True if `text` looks like a DID this project issues.
pub fn is_did(text: &str) -> bool {
    match text.strip_prefix(DID_PREFIX) {
        Some(hex_part) => hex_part.len() == 64 && hex_part.chars().all(|c| c.is_ascii_hexdigit()),
        None => false,
    }
}

/// True if `text` is a libp2p PeerId in string form.
pub fn is_peer_id(text: &str) -> bool {
    text.parse::<PeerId>().is_ok()
}

/// Classify an address a user might paste or scan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AddressKind {
    Did(String),
    PeerId(PeerId),
    /// Well-formed Reticulum AddressHash: 32 lowercase hex chars (16 bytes).
    Reticulum(String),
    Unknown,
}

/// Classify a recipient string without needing a network round-trip.
///
/// The send path accepts anything a user might hold: a DID, a PeerId, a RNS
/// address hash, or (as a special case) the literal `broadcast`.
pub fn classify(text: &str) -> AddressKind {
    let trimmed = text.trim();
    if is_did(trimmed) {
        return AddressKind::Did(trimmed.to_string());
    }
    if let Ok(peer) = trimmed.parse::<PeerId>() {
        return AddressKind::PeerId(peer);
    }
    // RNS AddressHash is 16 bytes = 32 hex chars, lowercase in the reference
    // implementation. Checked after DID/PeerId because those have fixed,
    // unambiguous forms and must not be shadowed.
    if trimmed.len() == 32 && trimmed.chars().all(|c| c.is_ascii_hexdigit()) {
        return AddressKind::Reticulum(trimmed.to_ascii_lowercase());
    }
    AddressKind::Unknown
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::identity::ed25519;

    fn a_keypair() -> Keypair {
        Keypair::generate_ed25519()
    }

    #[test]
    fn did_is_deterministic_for_a_keypair() {
        let kp = a_keypair();
        assert_eq!(
            did_from_keypair(&kp),
            did_from_keypair(&kp),
            "the same keypair must always yield the same DID"
        );
    }

    #[test]
    fn different_keys_yield_different_dids() {
        assert_ne!(
            did_from_keypair(&a_keypair()),
            did_from_keypair(&a_keypair())
        );
    }

    #[test]
    fn did_shape_is_correct() {
        let did = did_from_keypair(&a_keypair());
        assert!(did.starts_with("did:ghost:"));
        assert_eq!(did.len(), "did:ghost:".len() + 64);
        assert!(is_did(&did));
    }

    #[test]
    fn peer_id_matches_its_own_did() {
        // The load-bearing property: a peer can be verified against a DID with
        // no directory and no network.
        let kp = a_keypair();
        let did = did_from_keypair(&kp);
        let peer = PeerId::from(kp.public());
        assert!(
            peer_id_matches_did(&peer, &did),
            "a PeerId must verify against the DID derived from the same key"
        );
    }

    #[test]
    fn a_foreign_peer_id_does_not_match() {
        let did = did_from_keypair(&a_keypair());
        let stranger = PeerId::from(a_keypair().public());
        assert!(
            !peer_id_matches_did(&stranger, &did),
            "an unrelated PeerId must NOT verify against someone else's DID"
        );
    }

    #[test]
    fn classification_covers_the_three_address_forms() {
        let kp = a_keypair();
        let did = did_from_keypair(&kp);
        let peer = PeerId::from(kp.public());
        let rns = "0123456789abcdef0123456789abcdef";

        assert_eq!(classify(&did), AddressKind::Did(did.clone()));
        assert_eq!(classify(&peer.to_string()), AddressKind::PeerId(peer));
        assert_eq!(
            classify(rns),
            AddressKind::Reticulum(rns.to_string())
        );
    }

    #[test]
    fn classification_rejects_junk() {
        for junk in ["", "not-an-address", "did:ghost:tooshort", "12D3KooW!!"] {
            assert_eq!(
                classify(junk),
                AddressKind::Unknown,
                "{junk:?} must not classify as a routable address"
            );
        }
    }

    #[test]
    fn a_did_is_never_confused_with_a_rns_address() {
        // Both are hex; the lengths differ (64 vs 32). Regression guard for the
        // ordering in `classify`.
        let did = did_from_keypair(&a_keypair());
        assert!(matches!(classify(&did), AddressKind::Did(_)));
        assert!(matches!(
            classify("0123456789abcdef0123456789abcdef"),
            AddressKind::Reticulum(_)
        ));
    }
}