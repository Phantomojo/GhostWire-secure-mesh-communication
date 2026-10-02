//! Compile-time proof that `VerifiedPeerAddress` cannot be forged.
//!
//! The security property is that a libp2p address can only reach a dial by
//! passing `AddressExchange::verify`, which filters every address through
//! `world_discovery::validate_address(_, allow_private: false)`. If the struct
//! were constructible from outside its own module, that filter could be skipped
//! entirely and a caller could dial `192.168.1.50` with a forged, never-expiring
//! value.
//!
//! This is a COMPILE-FAIL test, so it lives in its own integration test target:
//! `VerifiedPeerAddress::forge()` must NOT compile, and the only way to assert
//! that is for the attempt to sit in a file that is built on every `cargo test`
//! run while being excluded from the crate's own build.
//!
//! The `FORGE_ATTEMPT` gate exists so this file compiles to an empty test when
//! the attempt is disabled. Enabling it (`--features forge-attempt`) must FAIL to
//! compile — that failure is the assertion. CI runs it deliberately expecting a
//! non-zero exit.
//!
//! Provenance: found and demonstrated by opencode, who compiled a forged value
//! before `#[non_exhaustive]` was added.

/// The test below documents the property. The forge attempt is compiled only
/// under the `forge-attempt` feature; see the module docs.
#[cfg(not(feature = "forge-attempt"))]
#[test]
fn verified_peer_address_is_not_constructible_from_outside() {
    // The real proof is the compile failure under `--features forge-attempt`,
    // which this file also contains. Run both:
    //   cargo test --test forge_attempt                      -> passes
    //   cargo build --tests --features forge-attempt          -> MUST fail
    //     error[E0639]: cannot create non-exhaustive struct using struct expression
    //
    // This runtime half only pins the attribute so a reviewer reading the suite
    // sees the invariant and its rationale without opening another file. It is
    // deliberately secondary: a source-text check can pass while the attribute
    // sits on the wrong item, which is why the E0639 check above is the real
    // assertion and this is only documentation.
    let source = include_str!("../src/core/discovery_bridge.rs");
    assert!(
        source.contains("#[non_exhaustive]"),
        "VerifiedPeerAddress must be #[non_exhaustive]; see this file's module docs"
    );
}

/// The forge attempt. Under `--features forge-attempt` this MUST NOT COMPILE.
/// If it ever starts compiling, the security boundary has been broken.
///
/// Run deliberately:
///   cargo test --test forge_attempt --features forge-attempt
/// and expect a COMPILE ERROR. A success here is a failure of the property.
#[cfg(feature = "forge-attempt")]
#[allow(dead_code)]
fn attempt_to_forge_a_dialable_address_from_outside_the_module() {
    let forged = ghostwire::core::discovery_bridge::VerifiedPeerAddress {
        peer_id: "12D3KooWForgedPeerId".to_string(),
        did: "did:key:forged".to_string(),
        addresses: vec!["/ip4/192.168.1.50/tcp/4001".to_string()],
        expires_at: u64::MAX,
    };
    assert!(forged.is_dialable_at(0));
}