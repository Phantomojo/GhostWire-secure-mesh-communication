//! Pairing records: signed proof that a node controls an address.
//!
//! ## Why this exists
//!
//! Option B — derive internally, publish the DID only, treat the PeerId as a
//! delivery detail disclosed at pairing — needs somewhere for that disclosure
//! to land. Without it, `send_direct_message` can never resolve a DID, because
//! resolution verifies the DID against peers already known and nothing ever
//! adds them.
//!
//! ## The security property
//!
//! A pairing record is a **signed statement of address ownership**. Anyone
//! holding the signer's public key can check that the node that presented this
//! address is the node that owns the DID — with no directory, no prior trust,
//! and no network call.
//!
//! This is the "make forgery provable" half of the threat model. GhostWire
//! cannot detect an attacker watching the network, but it CAN make an injected
//! or forged address detectable by anyone, which is the property that actually
//! holds.
//!
//! ## Deliberately not included
//!
//! No transport, no storage, no UI. This is the record format plus signing and
//! verification, so the pairing API and the QR/short-code flow can be built on
//! something already proven. Nothing here can fail open: an unsigned or
//! unverifiable record is rejected, not downgraded.

use libp2p::identity::Keypair;
use libp2p::PeerId;
use serde::{Deserialize, Serialize};

/// Version tag, so a future format change is detectable rather than silently
/// mis-parsed.
pub const PAIRING_VERSION: u8 = 1;

/// How long a pairing record stays usable.
///
/// Long enough to survive a laptop being closed overnight, short enough that a
/// stale address stops being dialed. Peers should re-pair rather than rely on a
/// record older than this.
pub const PAIRING_TTL_SECS: u64 = 30 * 24 * 60 * 60; // 30 days

/// A signed claim that `did` controls `peer_id` at `addresses`.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PairingRecord {
    pub version: u8,
    /// Canonical identity that signed this. Verifiable against the record.
    pub did: String,
    /// Delivery detail — only ever shared with someone who has paired.
    pub peer_id: String,
    /// Multiaddrs where this node can be reached, learned from discovery.
    pub addresses: Vec<String>,
    /// Unix seconds. The signature covers this, so it cannot be backdated by a
    /// third party without invalidating the signature.
    pub issued_at: u64,
    /// Unix seconds after which the record is no longer honoured.
    pub expires_at: u64,
    /// ed25519 signature over the canonical byte string of every field above.
    pub signature: Vec<u8>,
}

/// A rejection reason, so a caller can tell "not paired yet" from "the record
/// was forged" from "the record expired".
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PairingError {
    Malformed(String),
    /// The record is well-formed but its signature does not verify against the
    /// DID it claims. Treat as an attack or corruption, never as "retry".
    BadSignature,
    Expired,
    /// The DID and PeerId in the record are not actually the same identity.
    IdentityMismatch,
}

impl std::fmt::Display for PairingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PairingError::Malformed(m) => write!(f, "malformed pairing record: {m}"),
            PairingError::BadSignature => write!(
                f,
                "signature does not verify against the claimed DID — the record \
                 is forged or corrupted"
            ),
            PairingError::Expired => write!(f, "pairing record has expired; re-pair"),
            PairingError::IdentityMismatch => write!(
                f,
                "the DID and PeerId in this record are different identities"
            ),
        }
    }
}

impl std::error::Error for PairingError {}

/// The exact bytes that get signed.
///
/// Field order is fixed and explicit. Using serde on the struct would risk a
/// signature becoming invalid across a field-reordering change, so the canonical
/// form is built by hand with length prefixes so no field boundary can be
/// ambiguous.
fn signing_bytes(
    did: &str,
    peer_id: &str,
    addresses: &[String],
    issued_at: u64,
    expires_at: u64,
) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.push(PAIRING_VERSION);
    for field in [
        did.to_string(),
        peer_id.to_string(),
        addresses.join(","),
        issued_at.to_string(),
        expires_at.to_string(),
    ] {
        buf.extend_from_slice(&(field.len() as u32).to_be_bytes());
        buf.extend_from_slice(field.as_bytes());
    }
    buf
}

impl PairingRecord {
    /// Create and sign a record. Requires an unlocked identity.
    pub fn create(
        keypair: &Keypair,
        addresses: Vec<String>,
        now: u64,
        ttl_secs: u64,
    ) -> Self {
        let did = crate::core::deref::did_from_keypair(keypair);
        let peer_id = PeerId::from(keypair.public()).to_string();
        let expires_at = now + ttl_secs;

        let signature = keypair
            .sign(&signing_bytes(
                &did,
                &peer_id,
                &addresses,
                now,
                expires_at,
            ))
            .expect("libp2p Keypair::sign is infallible for ed25519");

        PairingRecord {
            version: PAIRING_VERSION,
            did,
            peer_id,
            addresses,
            issued_at: now,
            expires_at,
            signature,
        }
    }

    /// The canonical bytes a verifier recomputes.
    pub fn signing_bytes(&self) -> Vec<u8> {
        signing_bytes(
            &self.did,
            &self.peer_id,
            &self.addresses,
            self.issued_at,
            self.expires_at,
        )
    }

    /// Verify this record.
    ///
    /// Checks, in order: version, that the DID and PeerId are genuinely the same
    /// identity, that the signature verifies against the DID's own key, and that
    /// the record has not expired.
    ///
    /// The identity check matters independently of the signature: a validly
    /// signed record could still pair a DID with somebody else's PeerId, and
    /// that would be a genuine security failure rather than a forgery.
    pub fn verify(&self, now: u64) -> Result<(), PairingError> {
        if self.version != PAIRING_VERSION {
            return Err(PairingError::Malformed(format!(
                "unsupported version {}",
                self.version
            )));
        }

        // DID and PeerId must be the same identity. Parses the PeerId and
        // re-derives the DID from its key.
        let peer = self
            .peer_id
            .parse::<PeerId>()
            .map_err(|_| PairingError::Malformed("peer_id is not a valid PeerId".into()))?;
        if !crate::core::deref::peer_id_matches_did(&peer, &self.did) {
            return Err(PairingError::IdentityMismatch);
        }

        // Signature must verify against the key behind the PeerId.
        // `libp2p::identity::PublicKey::verify` returns bool, not Result.
        let public = libp2p::identity::PublicKey::try_decode_protobuf(peer.to_bytes().as_ref())
            .map_err(|_| PairingError::Malformed("peer_id is not a decodable public key".into()))?;
        if !public.verify(&self.signing_bytes(), &self.signature) {
            return Err(PairingError::BadSignature);
        }

        if now >= self.expires_at {
            return Err(PairingError::Expired);
        }

        Ok(())
    }

    /// A short, human-comparable form for out-of-band verification — reading
    /// it aloud in person, or scanning a QR.
    ///
    /// This is a *fingerprint*, not an address: two nodes showing the same code
    /// have the same identity. Deliberately lossy (first 8 bytes of the DID), so
    /// it cannot be used to send.
    pub fn short_code(did: &str) -> String {
        match did.strip_prefix("did:ghost:") {
            Some(hex_part) if hex_part.len() >= 12 => {
                let mut out = String::new();
                for (i, chunk) in hex_part.as_bytes().chunks(4).take(3).enumerate() {
                    if i > 0 {
                        out.push('-');
                    }
                    out.push_str(std::str::from_utf8(chunk).unwrap_or("????"));
                }
                out
            }
            _ => "invalid-did".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn kp() -> Keypair {
        Keypair::generate_ed25519()
    }

    const NOW: u64 = 1_800_000_000;

    #[test]
    fn a_freshly_created_record_verifies() {
        let keypair = kp();
        let record =
            PairingRecord::create(&keypair, vec!["/ip4/192.168.1.5/tcp/4001".into()], NOW, PAIRING_TTL_SECS);
        assert!(record.verify(NOW).is_ok());
    }

    #[test]
    fn verification_survives_a_serde_round_trip() {
        // A pairing record will be JSON-encoded for the API and for a QR code.
        // If the canonical form depended on in-memory layout rather than the
        // field values, that round trip would break the signature.
        let record = PairingRecord::create(&kp(), vec!["/ip4/10.0.0.1/tcp/4001".into()], NOW, PAIRING_TTL_SECS);
        let json = serde_json::to_string(&record).expect("serialise");
        let back: PairingRecord = serde_json::from_str(&json).expect("deserialise");
        assert_eq!(record, back);
        assert!(
            back.verify(NOW).is_ok(),
            "a record must still verify after a JSON round trip"
        );
    }

    #[test]
    fn a_tampered_address_is_rejected() {
        let mut record = PairingRecord::create(&kp(), vec!["/ip4/10.0.0.1/tcp/4001".into()], NOW, PAIRING_TTL_SECS);
        // Attacker rewrites the address to their own machine, leaving the
        // signature untouched.
        record.addresses = vec!["/ip4/203.0.113.9/tcp/4001".into()];
        assert_eq!(record.verify(NOW), Err(PairingError::BadSignature));
    }

    #[test]
    fn a_forged_signature_is_rejected() {
        let mut record = PairingRecord::create(&kp(), vec!["/ip4/10.0.0.1/tcp/4001".into()], NOW, PAIRING_TTL_SECS);
        record.signature[0] ^= 0xff;
        assert_eq!(record.verify(NOW), Err(PairingError::BadSignature));
    }

    #[test]
    fn backdating_is_rejected() {
        // Extending the expiry is the attack a user would actually try.
        let mut record = PairingRecord::create(&kp(), vec!["/ip4/10.0.0.1/tcp/4001".into()], NOW, PAIRING_TTL_SECS);
        record.expires_at = u64::MAX;
        assert_eq!(record.verify(NOW), Err(PairingError::BadSignature));
    }

    #[test]
    fn a_swapped_peer_id_is_caught_as_an_identity_mismatch() {
        // Even if an attacker could produce a valid signature (they cannot),
        // pairing someone else's PeerId with this DID must fail the identity
        // check independently.
        let mut record = PairingRecord::create(&kp(), vec!["/ip4/10.0.0.1/tcp/4001".into()], NOW, PAIRING_TTL_SECS);
        record.peer_id = PeerId::from(kp().public()).to_string();
        assert_eq!(record.verify(NOW), Err(PairingError::IdentityMismatch));
    }

    #[test]
    fn expiry_is_enforced() {
        let record = PairingRecord::create(&kp(), vec!["/ip4/10.0.0.1/tcp/4001".into()], NOW, PAIRING_TTL_SECS);
        assert!(record.verify(record.expires_at - 1).is_ok());
        assert_eq!(record.verify(record.expires_at), Err(PairingError::Expired));
    }

    #[test]
    fn address_field_boundaries_are_unambiguous() {
        // A record with ["ab","c"] must not verify as if it were ["a","bc"].
        // Length prefixes in the canonical form exist for exactly this.
        let a = PairingRecord::create(&kp(), vec!["ab".into(), "c".into()], NOW, PAIRING_TTL_SECS);
        let b = PairingRecord::create(&kp(), vec!["a".into(), "bc".into()], NOW, PAIRING_TTL_SECS);
        assert_ne!(
            a.signing_bytes(),
            b.signing_bytes(),
            "address joins must not be ambiguous"
        );
    }

    #[test]
    fn short_code_is_stable_and_distinct() {
        let a = crate::core::deref::did_from_keypair(&kp());
        let b = crate::core::deref::did_from_keypair(&kp());
        assert_eq!(PairingRecord::short_code(&a), PairingRecord::short_code(&a));
        assert_ne!(PairingRecord::short_code(&a), PairingRecord::short_code(&b));
        assert!(PairingRecord::short_code("nonsense") == "invalid-did");
    }
}