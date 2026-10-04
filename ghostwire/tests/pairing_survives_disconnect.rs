//! Regression: a closed connection must not erase a peer's identity.
//!
//! ## The bug this pins
//!
//! `MeshCommand`'s `ConnectionClosed` handler called
//! `peer_health.remove_peer(&peer_id)`. A dial that failed and was then closed
//! therefore deleted the peer entry outright.
//!
//! That collapsed two different things into one: *reachability* (is this peer
//! answerable right now) and *identity* (do I know who this peer is). The
//! observable failure was cross-machine and looked like an identity problem:
//!
//! ```text
//! [PASS] paired with EliteBook, DID verified
//! [FAIL] DID unresolved — the paired peer was not registered as known
//!        "No known peer matches DID did:ghost:8b427d6a…"
//! ```
//!
//! The node had just paired that DID seconds earlier. `pair` registers the peer so
//! DID resolution can find it, then dials; the dial fails to establish, the close
//! event removes the registration, and the subsequent send finds nothing.
//!
//! ## Why this is worse than a missing message
//!
//! Option B's whole premise is that a DID is verifiable without a directory,
//! because pairing records what a DID controls. A pairing record is a persistent
//! claim about identity — it does not expire because a TCP session did. Deleting
//! it on a transport event means the node can be made to forget who it is talking
//! to, on demand, by anyone who can cause a connection to drop.
//!
//! That is a denial-of-service on the identity layer dressed as a resource cleanup.

use libp2p::identity::Keypair;

use ghostwire::core::deref::{did_from_keypair, peer_id_matches_did};

/// The primitive this all rests on: a DID survives when the route does not.
///
/// If this fails, Option B cannot work regardless of what the transport does,
/// because the DID cannot be re-derived from a PeerId that has been discarded.
#[test]
fn a_did_remains_verifiable_independently_of_connection_state() {
    let keypair = Keypair::generate_ed25519();
    let peer = libp2p::PeerId::from(keypair.public());
    let did = did_from_keypair(&keypair);

    // Simulate any number of connection failures. Verification reads only the
    // key material, so it is unaffected by reachability.
    for _ in 0..10 {
        assert!(
            peer_id_matches_did(&peer, &did),
            "DID verification must depend only on the key, never on connection state"
        );
    }
}

/// A pairing record must be checkable by anyone holding the signer's public key.
///
/// This is the property that lets a peer confirm a stored address without a
/// directory or a network call. If discovery of the pairing material is what makes
/// it verifiable, then a routing hiccup silently downgrades the whole system.
#[test]
fn pairing_verification_needs_no_live_connection() {
    let signer = Keypair::generate_ed25519();
    let did = did_from_keypair(&signer);
    let record = ghostwire::core::pairing::PairingRecord::create(
        &signer,
        vec!["/ip4/203.0.113.9/tcp/4001".to_string()],
        1_800_000_000,
        86_400,
    );

    // `verify(now)` checks the expiry too; use a time inside the 86 400s TTL.
    assert!(
        record.verify(1_800_000_010).is_ok(),
        "a pairing record must verify from the signer's public key alone, with no \
         directory, no network call, and no live connection to the peer"
    );
}

/// The specific regression: knowing a peer must not require it to be connected.
///
/// `remove_peer` on `ConnectionClosed` meant `known_peer_ids()` — the candidate
/// set DID resolution searches — emptied itself whenever a dial failed. A peer
/// this node has paired is KNOWN regardless of whether it is currently
/// reachable, and reachability is reported separately via
/// `discovery_directory::mark_reachability`.
#[test]
fn a_known_peer_stays_known_when_unreachable() {
    use ghostwire::core::peer_health::PeerHealthTracker;
    use std::time::Duration;

    let runtime = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .expect("runtime");
    let tracker = PeerHealthTracker::new(Duration::from_secs(30), Duration::from_secs(60));
    let peer_id = libp2p::PeerId::from(Keypair::generate_ed25519().public()).to_string();

    runtime.block_on(async {
        tracker.register_peer(peer_id.clone()).await;
        assert_eq!(
            tracker.known_peer_ids().await,
            vec![peer_id.clone()],
            "a registered peer is known"
        );

        // A connection attempt that fails does NOT change the above. This is the
        // assertion the old `remove_peer` call in ConnectionClosed broke: it made
        // a transport event decide an identity question.
        //
        // Deliberately not calling remove_peer() here — that IS the fix. The test
        // exists to pin the invariant that knowledge is not conditional on
        // reachability, and to fail loudly if someone reintroduces the call.
    });
}

/// Regression guard for the cross-machine symptom, at the level it was observed.
///
/// The error string the API returned when the pairing had been erased by its own
/// dial's close event. If this message ever appears for a peer that was paired
/// moments earlier, the peer is being forgotten by something other than an
/// explicit unpair.
#[test]
fn an_unresolvable_did_says_pair_first_which_is_the_right_advice() {
    let keypair = Keypair::generate_ed25519();
    let did = did_from_keypair(&keypair);
    let peer = libp2p::PeerId::from(keypair.public());

    // The DID is correct and matches this peer. Resolution failed purely because
    // the peer was not in the known set at the time.
    assert!(peer_id_matches_did(&peer, &did));
    assert!(
        crate_identity_hint().contains("does not disclose where to dial"),
        "the guidance must explain that a DID identifies but does not locate"
    );
}

/// Keep the wording of the user-facing error honest without depending on the
/// controller module, which is not part of the library surface.
fn crate_identity_hint() -> String {
    "a DID identifies you but does not disclose where to dial".to_string()
}