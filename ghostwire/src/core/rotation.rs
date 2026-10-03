//! Identity rotation and revocation.
//!
//! ## The problem
//!
//! GhostWire had **no rotation mechanism at all** — zero occurrences of `rotate`
//! anywhere under `src/core/identity/`. If the ed25519 key leaked, the node was
//! compromised permanently:
//!
//!   * The DID is `SHA256(pubkey)`, so a new key means a new DID. Nothing
//!     regenerates it.
//!   * The PeerId follows the key, so it changes too.
//!   * `identity.json` holds the encrypted keypair; nothing replaces it.
//!   * **Every peer that ever learned your address still holds it.** Under
//!     Option B anyone you have paired with has your PeerId on their device, and
//!     without a revocation a revoked key stays live at every one of them.
//!
//! That last point is what makes this more than a convenience. Rotation without
//! revocation is a rename, not a remediation.
//!
//! ## Reuse, not reinvention
//!
//! A revocation is structurally the same object as a pairing record: a signed,
//! canonical, verifiable statement about an identity. So it reuses
//! [`PairingRecord`]'s signing form, verification path and error taxonomy rather
//! than introducing a second signature scheme. One crypto path, audited once.
//!
//! ## The property
//!
//! A revocation is verifiable by anyone holding the revoked key — which is the
//! point. A peer that stored your old address can check "is this identity dead?"
//! with no directory, no revocation server, and no network call. That is what
//! decentralised revocation requires, and it is what Option B's "no central
//! authority" claim needs in order to actually hold.

use crate::core::pairing::{PairingError, PairingRecord};
use serde::{Deserialize, Serialize};

/// Version tag for the revocation format.
pub const REVOCATION_VERSION: u8 = 1;

/// A signed statement that an identity is no longer valid.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Revocation {
    pub version: u8,
    /// The DID being revoked. After this, a peer must refuse it.
    pub did: String,
    /// The PeerId that DID mapped to, so a holder of a stale address can match
    /// a revocation against what they stored without recomputing the DID.
    pub peer_id: String,
    /// Unix seconds when the compromise was noticed.
    pub revoked_at: u64,
    /// Optional human-readable cause: "key compromise", "routine rotation",
    /// "device lost". Audit value only; never used for enforcement.
    #[serde(default)]
    pub reason: String,
    /// Signature by the revoked key itself.
    ///
    /// Self-signed, which raises the obvious question: why does a self-signed
    /// statement count? Because the alternative is worse. An attacker who has
    /// stolen a key can also sign, so a self-signature cannot *prevent* abuse —
    /// but it lets every honest holder of the old address learn the truth without
    /// trusting a third party. That is a real improvement over a key that stays
    /// live forever, and it is the only form that needs no central authority.
    ///
    /// The stronger guarantee — proving revocation by someone OTHER than the
    /// owner — would need a CA or a transparency log, which Option B rules out.
    /// Stated plainly rather than implied.
    pub signature: Vec<u8>,
}

/// Canonical bytes signed by a revocation. Length-prefixed, for the same reason
/// as pairing records: a signed form that depends on field order would silently
/// invalidate on reordering.
fn signing_bytes(did: &str, peer_id: &str, revoked_at: u64) -> Vec<u8> {
    let mut buf = vec![REVOCATION_VERSION];
    for field in [did.to_string(), peer_id.to_string(), revoked_at.to_string()] {
        buf.extend_from_slice(&(field.len() as u32).to_be_bytes());
        buf.extend_from_slice(field.as_bytes());
    }
    buf
}

impl Revocation {
    /// Create a revocation signed by the compromised key.
    ///
    /// `keypair` must be the OLD key — the one being revoked. A revocation
    /// signed by the new key would be self-serving and unverifiable by anyone
    /// holding only the old identity.
    pub fn create(
        keypair: &libp2p::identity::Keypair,
        revoked_at: u64,
        reason: &str,
    ) -> Self {
        let did = crate::core::deref::did_from_keypair(keypair);
        let peer_id = libp2p::PeerId::from(keypair.public()).to_string();
        let signature = keypair
            .sign(&signing_bytes(&did, &peer_id, revoked_at))
            .expect("libp2p Keypair::sign is infallible for ed25519");

        Revocation {
            version: REVOCATION_VERSION,
            did,
            peer_id,
            revoked_at,
            reason: reason.to_string(),
            signature,
        }
    }

    pub fn signing_bytes(&self) -> Vec<u8> {
        signing_bytes(&self.did, &self.peer_id, self.revoked_at)
    }

    /// Verify this revocation against the identity it names.
    ///
    /// Checks that the `peer_id` really does hash to the `did`, and that the
    /// signature verifies against that key. Deliberately does NOT check
    /// recency: a revocation is valid forever. Someone compromised last year is
    /// still compromised, and an expiry would let an attacker simply wait.
    pub fn verify(&self) -> Result<(), PairingError> {
        if self.version != REVOCATION_VERSION {
            return Err(PairingError::Malformed(format!(
                "unsupported revocation version {}",
                self.version
            )));
        }
        let peer = self
            .peer_id
            .parse::<libp2p::PeerId>()
            .map_err(|_| PairingError::Malformed("peer_id is not a valid PeerId".into()))?;
        if !crate::core::deref::peer_id_matches_did(&peer, &self.did) {
            return Err(PairingError::IdentityMismatch);
        }
        let public = libp2p::identity::PublicKey::try_decode_protobuf(peer.to_bytes().as_ref())
            .map_err(|_| PairingError::Malformed("peer_id is not a decodable public key".into()))?;
        if !public.verify(&self.signing_bytes(), &self.signature) {
            return Err(PairingError::BadSignature);
        }
        Ok(())
    }

    /// Is this identity revoked, according to `revocations`?
    ///
    /// Only count revocations that verify. An unverifiable entry is ignored
    /// rather than obeyed — otherwise anyone could revoke anyone by injecting a
    /// garbage record, which is a denial-of-service on the whole network.
    pub fn is_revoked(did: &str, revocations: &[Revocation]) -> bool {
        revocations
            .iter()
            .filter(|r| r.verify().is_ok())
            .any(|r| r.did == did)
    }
}

/// What rotating an identity changes, so callers do not miss a piece.
///
/// Both identifiers change. That is the point of rotating, and it is also why
/// peers holding a stale address must be told — hence a revocation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RotationResult {
    pub old_did: String,
    pub old_peer_id: String,
    pub new_did: String,
    pub new_peer_id: String,
    /// Publish so peers that stored the old address learn it is dead.
    pub revocation: Revocation,
}

/// Compute what a rotation produces, without performing it.
///
/// Separated from the destructive part so the whole transformation is testable
/// and reviewable before anything on disk is replaced.
pub fn plan_rotation(
    old_keypair: &libp2p::identity::Keypair,
    new_keypair: &libp2p::identity::Keypair,
    revoked_at: u64,
    reason: &str,
) -> RotationResult {
    RotationResult {
        old_did: crate::core::deref::did_from_keypair(old_keypair),
        old_peer_id: libp2p::PeerId::from(old_keypair.public()).to_string(),
        new_did: crate::core::deref::did_from_keypair(new_keypair),
        new_peer_id: libp2p::PeerId::from(new_keypair.public()).to_string(),
        revocation: Revocation::create(old_keypair, revoked_at, reason),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use libp2p::identity::Keypair;
    use libp2p::PeerId;

    fn kp() -> Keypair {
        Keypair::generate_ed25519()
    }

    const NOW: u64 = 1_800_000_000;

    #[test]
    fn a_fresh_revocation_verifies() {
        assert!(Revocation::create(&kp(), NOW, "key compromise").verify().is_ok());
    }

    #[test]
    fn a_revocation_survives_a_json_round_trip() {
        let r = Revocation::create(&kp(), NOW, "routine");
        let back: Revocation = serde_json::from_str(&serde_json::to_string(&r).unwrap()).unwrap();
        assert_eq!(r, back);
        assert!(back.verify().is_ok());
    }

    #[test]
    fn a_tampered_timestamp_is_rejected() {
        let mut r = Revocation::create(&kp(), NOW, "x");
        r.revoked_at = 0;
        assert_eq!(r.verify(), Err(PairingError::BadSignature));
    }

    #[test]
    fn a_swapped_peer_id_is_caught() {
        let mut r = Revocation::create(&kp(), NOW, "x");
        r.peer_id = PeerId::from(kp().public()).to_string();
        assert_eq!(r.verify(), Err(PairingError::IdentityMismatch));
    }

    #[test]
    fn rotation_changes_both_identifiers() {
        let old = kp();
        let new = kp();
        let plan = plan_rotation(&old, &new, NOW, "compromise");

        assert_ne!(plan.old_did, plan.new_did, "rotation must change the DID");
        assert_ne!(
            plan.old_peer_id, plan.new_peer_id,
            "rotation must change the PeerId"
        );
        // The revocation is signed by the OLD key, so holders of the old
        // identity can verify it.
        assert!(plan.revocation.verify().is_ok());
        assert_eq!(plan.revocation.did, plan.old_did);
    }

    #[test]
    fn a_revoked_identity_is_detected() {
        let old = kp();
        let plan = plan_rotation(&old, &kp(), NOW, "compromise");
        assert!(Revocation::is_revoked(&plan.old_did, &[plan.revocation.clone()]));
        assert!(
            !Revocation::is_revoked(&plan.new_did, &[plan.revocation]),
            "the NEW identity must not be treated as revoked"
        );
    }

    #[test]
    fn a_forged_revocation_does_not_revoke_anyone() {
        // The denial-of-service guard: anyone could inject garbage to revoke
        // arbitrary DIDs. Unverifiable entries must be ignored, not obeyed.
        let victim = crate::core::deref::did_from_keypair(&kp());
        let forged = Revocation {
            version: REVOCATION_VERSION,
            did: victim.clone(),
            peer_id: PeerId::from(kp().public()).to_string(),
            revoked_at: NOW,
            reason: "hostile".into(),
            signature: vec![1, 2, 3],
        };
        assert!(forged.verify().is_err(), "forgery must not verify");
        assert!(
            !Revocation::is_revoked(&victim, &[forged]),
            "an unverifiable revocation must not be honoured — that would let \
             anyone revoke anyone"
        );
    }

    #[test]
    fn a_reason_cannot_alter_the_verification() {
        // `reason` is audit-only. Changing it must not break the signature —
        // otherwise it is not really optional metadata.
        let mut r = Revocation::create(&kp(), NOW, "key compromise");
        r.reason = "something else entirely".into();
        assert!(r.verify().is_ok());
    }

    #[test]
    fn revocation_does_not_expire() {
        // A revocation is valid forever. An attacker with a stolen key must not
        // be able to wait out an expiry.
        let r = Revocation::create(&kp(), NOW, "x");
        assert!(r.verify().is_ok(), "verification must not depend on time");
    }
}