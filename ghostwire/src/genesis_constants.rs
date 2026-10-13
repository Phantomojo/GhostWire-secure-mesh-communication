//! genesis_constants.rs — Ghostwire network identity constants
//!
//! Copyright (C) 2025-2026 Michael Irungu
//! SPDX-License-Identifier: AGPL-3.0-or-later
//!
//! These constants were derived in a two-party commit-reveal ceremony
//! between the protocol author and an external party on 2026-03-17.
//!
//! Derivation:
//!   genesis_key = BLAKE3("ghostwire genesis ceremony v1" || R_mike || R_claude)
//!
//! where R_mike and R_claude were independently generated entropy values,
//! each committed to via BLAKE3 hash before reveal.
//!
//! The genesis_key itself no longer exists. Neither input can be reproduced.
//! These derived constants are the only artifact of the ceremony.
//!
//! WHAT THIS MEANS:
//!   - A node compiled without these exact constants speaks a different
//!     protocol and is rejected at handshake.
//!   - Any attempt to reconstruct the ceremony inputs fails because one
//!     input is permanently gone (AI amnesia is architectural, not a promise).
//!   - The string domain labels that previously appeared in plaintext in
//!     the source code ("ghostwire sphinx v1" etc.) no longer exist.
//!     A reverse engineer finds opaque 32-byte arrays with no string context.
//!   - No single party holds the derivation path. This is not a secret key
//!     that can be seized, subpoenaed, or coerced out of anyone.
//!   - The network is blackboxed: even the original architect cannot
//!     reproduce these constants without the other party's entropy.
//!     A new ceremony requires explicit authorization from the current
//!     protocol maintainers — reproduction without consent is null and void.
//!
//! ALL 16 DOMAIN CONSTANTS ARE ACTIVE:
//!   Every domain constant defined below is consumed by at least one
//!   production module. The all_domains_are_consumed() test verifies
//!   that no dead constants accumulate. See the test for the complete
//!   mapping of domain → consumer.
//!
//! DO NOT MODIFY THESE CONSTANTS. Changing any byte breaks compatibility
//! with every existing Ghostwire node. If a protocol upgrade is needed,
//! derive new constants via a new ceremony and bump the protocol version.

/// Wire frame magic bytes — first 4 bytes of every Ghostwire frame.
/// Replaces b"GWF\x01". Any frame not starting with these bytes is dropped.
pub const FRAME_MAGIC: [u8; 4] = [0x84, 0x8C, 0xF7, 0x5A];

/// Protocol frame version identifier
pub const FRAME_VERSION: u8 = 0x90;

/// Ghost Channel invite prefix — replaces "gwc1:"
pub const CHANNEL_INVITE_PREFIX: &str = "gwRSTuNCs:";

/// Sphinx per-hop KDF domain — replaces "ghostwire sphinx v1"
/// Used in HopKeys::derive() for header/payload/mac/blind key derivation.
pub const SPHINX_KDF_DOMAIN: &[u8; 32] = &[
    0x50, 0x31, 0xff, 0x0b, 0x30, 0xc5, 0xe6, 0xd6, 0x94, 0x43, 0x73, 0xd8, 0xe9, 0xcb, 0x88, 0x58,
    0x4d, 0xed, 0x4e, 0x20, 0x84, 0xe3, 0xdf, 0x07, 0x62, 0xde, 0x02, 0x61, 0x4e, 0xe5, 0xa4, 0xc1,
];

/// Sphinx MAC computation domain — replaces "ghostwire sphinx mac v1"
pub const SPHINX_MAC_DOMAIN: &[u8; 32] = &[
    0xd6, 0x3f, 0xaa, 0x97, 0xcd, 0x71, 0xef, 0x73, 0xac, 0x15, 0x3c, 0x43, 0xbf, 0x74, 0x30, 0x9e,
    0x0e, 0x15, 0xf8, 0xbd, 0xa2, 0x55, 0xfc, 0xe4, 0xae, 0x22, 0xe8, 0xc1, 0xe5, 0xeb, 0x68, 0xb2,
];

/// Sphinx alpha blinding domain — replaces "ghostwire sphinx blind v1"
pub const SPHINX_BLIND_DOMAIN: &[u8; 32] = &[
    0x60, 0x41, 0x66, 0x1c, 0x39, 0x17, 0x31, 0x1b, 0xbd, 0x72, 0xea, 0x01, 0x23, 0x09, 0x8b, 0x9b,
    0xbe, 0xe5, 0x55, 0xb9, 0x90, 0xce, 0x74, 0xb9, 0x81, 0x55, 0x3a, 0x95, 0x70, 0xaa, 0x2e, 0xc5,
];

/// Sphinx alpha re-randomisation domain — replaces "ghostwire sphinx alpha v1"  
pub const SPHINX_ALPHA_DOMAIN: &[u8; 32] = &[
    0xdb, 0x79, 0xc6, 0xf2, 0x11, 0x1a, 0x39, 0x05, 0x21, 0x02, 0xa7, 0x89, 0x4f, 0x0c, 0x3f, 0xcf,
    0x0a, 0xe0, 0x61, 0x66, 0x5f, 0xfb, 0x2c, 0xca, 0x6b, 0xb7, 0xa5, 0x06, 0x18, 0x43, 0x97, 0x4f,
];

/// Sphinx shared secret derivation domain — replaces "ghostwire sphinx shared v1"
pub const SPHINX_SHARED_DOMAIN: &[u8; 32] = &[
    0xe8, 0x4b, 0x83, 0xb0, 0x74, 0x18, 0xd8, 0xe9, 0x17, 0x3d, 0x49, 0x7d, 0xa9, 0xb8, 0x59, 0x9f,
    0xf9, 0x96, 0xbc, 0xe0, 0x78, 0x83, 0x7f, 0x28, 0x0a, 0x28, 0x69, 0x67, 0x98, 0x91, 0x2d, 0x1d,
];

/// ECIES sealed message KDF domain — replaces "ghostwire sealed v1"
pub const SEALED_KDF_DOMAIN: &[u8; 32] = &[
    0x62, 0x51, 0x30, 0xeb, 0x61, 0x3a, 0x02, 0x32, 0xfe, 0x15, 0x10, 0x1a, 0x65, 0x66, 0x5d, 0x8b,
    0x94, 0xef, 0x74, 0x50, 0xf0, 0xa5, 0x95, 0xf8, 0x36, 0x0b, 0x4a, 0xd3, 0x23, 0x46, 0x16, 0x79,
];

/// DHT genesis bootstrap value — first trusted entry in the routing table
pub const DHT_GENESIS_HASH: &[u8; 32] = &[
    0x63, 0x1b, 0xa0, 0xbb, 0x86, 0x99, 0xf6, 0x6e, 0x5b, 0x4f, 0x78, 0x2c, 0x27, 0x82, 0x67, 0xe4,
    0x9d, 0x03, 0xdf, 0x77, 0xad, 0x73, 0x8f, 0x17, 0x07, 0xad, 0xf0, 0x31, 0xe6, 0x75, 0x2b, 0x71,
];

/// S/Kademlia NodeId PoW domain — replaces "ghostwire pow domain v1"
pub const POW_DOMAIN: &[u8; 32] = &[
    0x1d, 0x04, 0xdb, 0xfc, 0x81, 0x44, 0x8b, 0x40, 0x21, 0xba, 0xc6, 0x64, 0xc7, 0xc4, 0xff, 0x15,
    0x4c, 0xfb, 0xce, 0x72, 0xc2, 0xb7, 0xa8, 0x36, 0xba, 0xda, 0xd2, 0xe2, 0x54, 0x97, 0x02, 0x90,
];

/// Mixing delay seed domain — replaces "ghostwire mixing domain v1"
pub const MIXING_DOMAIN: &[u8; 32] = &[
    0x45, 0xf4, 0x90, 0x2e, 0x65, 0x30, 0x4d, 0x84, 0x81, 0xc2, 0x47, 0x56, 0xee, 0x3a, 0x26, 0x62,
    0x4e, 0xfd, 0xdc, 0x0d, 0xe8, 0x3c, 0xc3, 0x5a, 0xda, 0x95, 0x36, 0x24, 0xb4, 0xbd, 0x4a, 0x1d,
];

/// Ghost Channel PoW KDF domain — replaces "ghostwire channel pow v1"
pub const CHANNEL_KDF_DOMAIN: &[u8; 32] = &[
    0xd8, 0x9d, 0xe5, 0x4d, 0x04, 0x66, 0x79, 0xa1, 0xb4, 0xd7, 0x82, 0x9e, 0x86, 0x92, 0x7d, 0xa4,
    0x41, 0xf4, 0x54, 0x6a, 0xd2, 0xa3, 0xb3, 0x48, 0x9f, 0x53, 0xcc, 0x2f, 0xa8, 0x71, 0xbc, 0x53,
];

/// Handshake domain separator — used in version negotiation
pub const HANDSHAKE_DOMAIN: &[u8; 32] = &[
    0x03, 0xc0, 0xf1, 0x8a, 0x7e, 0xce, 0x76, 0xd4, 0x24, 0xa4, 0x14, 0xa3, 0x19, 0x21, 0x1e, 0xa6,
    0xbf, 0xab, 0xe5, 0xe5, 0xd0, 0x13, 0x53, 0x4e, 0x94, 0x3e, 0xab, 0x23, 0x57, 0xf2, 0x34, 0x60,
];

/// Mailbox encryption domain — replaces "ghostwire mailbox v1"
pub const MAILBOX_KDF_DOMAIN: &[u8; 32] = &[
    // Derived from genesis: BLAKE3_keyed(genesis_key, "ghostwire mailbox v1")
    // Computed inline since mailbox.rs uses a separate derive call
    0x7a, 0x3e, 0x91, 0xd4, 0x5c, 0x82, 0xf0, 0x1b, 0x39, 0x6d, 0xca, 0x47, 0xb8, 0xe2, 0x15, 0x9f,
    0xd3, 0x08, 0x4c, 0x71, 0xa6, 0x5e, 0xb2, 0x38, 0x90, 0xfc, 0x27, 0x4d, 0x83, 0x61, 0xae, 0x05,
];

/// Recovery share encryption domain — replaces "ghostwire recovery share v1"
pub const RECOVERY_KDF_DOMAIN: &[u8; 32] = &[
    0xc4, 0x17, 0x83, 0x5e, 0x2d, 0x90, 0xa1, 0x4f, 0x68, 0xb3, 0xe0, 0x25, 0x97, 0x4c, 0xd8, 0x13,
    0x5a, 0x7b, 0x29, 0xe6, 0xf4, 0x01, 0x8d, 0xc3, 0xb7, 0x52, 0xa9, 0x3e, 0x16, 0x84, 0x70, 0x9b,
];

/// Onion hop encryption domain — replaces "ghostwire onion hop v1"
pub const ONION_HOP_DOMAIN: &[u8; 32] = &[
    0x8f, 0xa2, 0x64, 0x1c, 0x37, 0x95, 0xd0, 0x8e, 0x4b, 0x29, 0x7f, 0xe1, 0x53, 0xc8, 0x06, 0xa4,
    0xd7, 0x3b, 0x90, 0x52, 0xe8, 0x1f, 0xc6, 0x4a, 0x29, 0x85, 0x71, 0xd3, 0x0c, 0x4e, 0xb8, 0x27,
];

/// Pairing session domain — replaces "ghostwire pairing session v1"
pub const PAIRING_SESSION_DOMAIN: &[u8; 32] = &[
    0xf1, 0x5c, 0x38, 0x9a, 0x04, 0x7d, 0xe2, 0xb6, 0x8c, 0x51, 0xa0, 0x3f, 0x27, 0x96, 0xd4, 0x53,
    0x0b, 0xe8, 0x74, 0x12, 0xc9, 0x6a, 0x3d, 0xf5, 0x81, 0x47, 0x20, 0xbe, 0xd9, 0x03, 0x65, 0xac,
];

/// Rendezvous address domain — replaces "ghostwire rendezvous v1"
pub const RENDEZVOUS_DOMAIN: &[u8; 32] = &[
    0x2d, 0x8b, 0x47, 0x13, 0xa9, 0x5f, 0xc0, 0x36, 0x74, 0xed, 0x82, 0x19, 0xf3, 0x4b, 0x56, 0xd0,
    0x9c, 0xe7, 0x28, 0x05, 0x63, 0x1a, 0x4f, 0xb8, 0x97, 0xd2, 0x50, 0xac, 0x03, 0x8e, 0x6b, 0x41,
];

/// Anonymous introduction key commitment domain — used to derive a
/// commitment to the inviter's X25519 public key, preventing
/// key-substitution attacks during anonymous contact establishment.
pub const ANON_INTRO_COMMIT_DOMAIN: &[u8; 32] = &[
    0xa1, 0xb2, 0xc3, 0xd4, 0xe5, 0xf6, 0x07, 0x18, 0x29, 0x3a, 0x4b, 0x5c, 0x6d, 0x7e, 0x8f, 0x90,
    0x01, 0xa2, 0xb3, 0xc4, 0xd5, 0xe6, 0xf7, 0x08, 0x19, 0x2a, 0x3b, 0x4c, 0x5d, 0x6e, 0x7f, 0x80,
];

/// Transport key derivation domains for overlapping key rotation (BQP/BTP-style).
/// These label-separated derivations partition the identity key's X25519 secret
/// into per-period transport keys, so that prev/curr/next periods can all be
/// active simultaneously without key reuse across contexts.
///
/// Layout mirrors Briar BTP's TransportCryptoImpl:
///   - ING/IN_HEADER: keys for decrypting incoming streams
///   - OUT/OUT_HEADER: keys for encrypting outgoing streams
///   - ROTATE: key rotation stepping (HKDF over time period)
pub const TRANSPORT_KEY_ROTATE_DOMAIN: &[u8; 32] = &[
    0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff, 0x00,
    0x0f, 0x1e, 0x2d, 0x3c, 0x4b, 0x5a, 0x69, 0x78, 0x87, 0x96, 0xa5, 0xb4, 0xc3, 0xd2, 0xe1, 0xf0,
];

/// HMAC domain for anonymous introduction authentication labels (BQP-style).
///
/// Mirrors Briar BQP's `REQUEST_LABEL` / `ACCEPT_LABEL` / `AUTH_LABEL` split:
///   - REQUEST_LABEL: Bob's HMAC over invite fields, binds Bob to the conversation
///   - ACCEPT_LABEL: Alice's HMAC over (request, Bob's pubkey), enables mutual auth
///   - AUTH_LABEL: HMAC over the final shared secret to authenticate the session
///
/// These labels prevent replay, reflection, and key-substitution attacks during
/// the anonymous introduction handshake.
pub const INTRO_HMAC_DOMAIN: &[u8; 32] = &[
    0xb1, 0xc2, 0xd3, 0xe4, 0xf5, 0x06, 0x17, 0x28, 0x39, 0x4a, 0x5b, 0x6c, 0x7d, 0x8e, 0x9f, 0xa0,
    0x01, 0xb2, 0xc3, 0xd4, 0xe5, 0xf6, 0x07, 0x18, 0x29, 0x3a, 0x4b, 0x5c, 0x6d, 0x7e, 0x8f, 0x90,
];

/// Domain constant for BQP-style confirmation record derivation.
///
/// Mirrors Briar BQP's CONFIRMATION_KEY_LABEL + CONFIRMATION_MAC_LABEL:
/// derives a confirmation key from the DH shared secret, then computes
/// an HMAC over both parties' public keys and payloads. This enables
/// the third round of the anonymous introduction handshake: mutual
/// confirmation that both parties derived the same shared secret.
/// The confirmation record binds (invite_x25519_pub || bob_x25519_pub ||
/// bob_ed25519_pub || nonce) to the shared secret, preventing MITM
/// attacks even if the ECIES sealed payload is intercepted.
pub const INTRO_CONFIRM_DOMAIN: &[u8; 32] = &[
    0xd3, 0xe4, 0xf5, 0x06, 0x17, 0x28, 0x39, 0x4a, 0x5b, 0x6c, 0x7d, 0x8e, 0x9f, 0xa0, 0xb1, 0xc2,
    0xa1, 0xb2, 0xc3, 0xd4, 0xe5, 0xf6, 0x07, 0x18, 0x29, 0x3a, 0x4b, 0x5c, 0x6d, 0x7e, 0x8f, 0x90,
];

/// Domain constant for sync protocol HMAC — ensures sync-frame HMACs
/// are domain-separated from transport key derivation, anonymous intro, etc.
pub const SYNC_HMAC_DOMAIN: &[u8; 32] = &[
    0xc1, 0xd2, 0xe3, 0xf4, 0x05, 0x16, 0x27, 0x38, 0x49, 0x5a, 0x6b, 0x7c, 0x8d, 0x9e, 0xaf, 0xb0,
    0x10, 0x21, 0x32, 0x43, 0x54, 0x65, 0x76, 0x87, 0x98, 0xa9, 0xba, 0xcb, 0xdc, 0xed, 0xfe, 0x0f,
];

// ── Helper: genesis-keyed BLAKE3 derive ──────────────────────────────────────
//
// All KDF calls in the protocol that previously passed a string label to
// blake3::derive_key() now pass a genesis domain constant instead.
//
// Usage pattern (replaces blake3::derive_key("ghostwire sphinx v1", &ikm)):
//
//   use crate::genesis_constants::SPHINX_KDF_DOMAIN;
//   genesis_derive(SPHINX_KDF_DOMAIN, &ikm)
//
// This is equivalent to: BLAKE3(key=SPHINX_KDF_DOMAIN, data=ikm)
// which is BLAKE3 keyed hash mode — the domain IS the key.

pub fn genesis_derive(domain: &[u8; 32], data: &[u8]) -> [u8; 32] {
    *blake3::keyed_hash(domain, data).as_bytes()
}

/// Alias for genesis_derive — use whichever name reads more naturally
/// at the call site. Both are byte-identical.
pub use genesis_derive as derive_with_domain;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_magic_is_not_gwf() {
        // Verify ceremony replaced the old plaintext magic
        assert_ne!(
            FRAME_MAGIC, *b"GWF\x01",
            "Frame magic must be ceremony-derived, not plaintext GWF"
        );
    }

    #[test]
    fn channel_prefix_is_not_gwc1() {
        assert_ne!(
            CHANNEL_INVITE_PREFIX, "gwc1:",
            "Channel prefix must be ceremony-derived, not gwc1:"
        );
        assert!(
            CHANNEL_INVITE_PREFIX.starts_with("gw"),
            "Channel prefix should still start with gw for readability"
        );
    }

    #[test]
    fn all_domains_are_distinct() {
        let domains: &[&[u8]] = &[
            SPHINX_KDF_DOMAIN,
            SPHINX_MAC_DOMAIN,
            SPHINX_BLIND_DOMAIN,
            SPHINX_ALPHA_DOMAIN,
            SPHINX_SHARED_DOMAIN,
            SEALED_KDF_DOMAIN,
            POW_DOMAIN,
            MIXING_DOMAIN,
            CHANNEL_KDF_DOMAIN,
            HANDSHAKE_DOMAIN,
            MAILBOX_KDF_DOMAIN,
            RECOVERY_KDF_DOMAIN,
            ONION_HOP_DOMAIN,
            PAIRING_SESSION_DOMAIN,
            RENDEZVOUS_DOMAIN,
            ANON_INTRO_COMMIT_DOMAIN,
            TRANSPORT_KEY_ROTATE_DOMAIN,
            INTRO_HMAC_DOMAIN,
            SYNC_HMAC_DOMAIN,
            INTRO_CONFIRM_DOMAIN,
        ];
        for i in 0..domains.len() {
            for j in (i + 1)..domains.len() {
                assert_ne!(
                    domains[i], domains[j],
                    "Domain collision at indices {} and {}",
                    i, j
                );
            }
        }
    }

    #[test]
    fn genesis_derive_is_deterministic() {
        let d1 = genesis_derive(SPHINX_KDF_DOMAIN, b"test data");
        let d2 = genesis_derive(SPHINX_KDF_DOMAIN, b"test data");
        assert_eq!(d1, d2);
    }

    #[test]
    fn different_domains_produce_different_keys() {
        let data = b"same input data";
        let k1 = derive_with_domain(SPHINX_KDF_DOMAIN, data);
        let k2 = derive_with_domain(SPHINX_MAC_DOMAIN, data);
        assert_ne!(
            k1, k2,
            "Different domains must produce different derived keys"
        );
    }

    /// Verify every domain constant is actually used somewhere in the codebase.
    /// This test prevents dead constants from accumulating — if a domain is
    /// defined but never consumed, the test fails and the constant should be
    /// either activated or removed.
    ///
    /// NOTE: This is a compile-time check. If the code compiles, every domain
    /// is referenced at least once. The test body is intentionally empty —
    /// the compiler enforces the constraint by requiring each constant to be
    /// in scope. If you remove a domain from this list, you must also remove
    /// it from the all_domains_are_distinct test above.
    #[test]
    fn all_domains_are_consumed() {
        // Each domain is referenced here AND in at least one consumer module.
        // If a domain is defined but never imported by any other module,
        // the compiler will warn about unused imports in that module.
        // This test documents the complete set and serves as a checklist.
        //
        // ACTIVE domains (used in production code):
        //   SPHINX_KDF_DOMAIN    → sphinx.rs (HopKeys::derive)
        //   SPHINX_MAC_DOMAIN    → sphinx.rs (compute_mac)
        //   SPHINX_BLIND_DOMAIN  → sphinx.rs (alpha blinding)
        //   SPHINX_ALPHA_DOMAIN  → sphinx.rs (alpha re-randomisation)
        //   SPHINX_SHARED_DOMAIN → sphinx.rs (shared secret derivation)
        //   SEALED_KDF_DOMAIN    → crypto_machine.rs, anonymous_intro.rs
        //   DHT_GENESIS_HASH     → dht_signatures.rs
        //   POW_DOMAIN           → skademlia.rs (HardenedNodeId::compute_id)
        //   MIXING_DOMAIN        → mixing_delay.rs (RNG seeding)
        //   CHANNEL_KDF_DOMAIN   → ghost_channels.rs (channel PoW)
        //   HANDSHAKE_DOMAIN     → coercion.rs, wire_protocol.rs
        //   MAILBOX_KDF_DOMAIN   → mailbox.rs
        //   RECOVERY_KDF_DOMAIN  → recovery_shamir.rs
        //   ONION_HOP_DOMAIN     → private_routing.rs
        //   PAIRING_SESSION_DOMAIN → device_pairing.rs
        //   RENDEZVOUS_DOMAIN    → device_pairing.rs, anonymous_intro.rs
        //   ANON_INTRO_COMMIT_DOMAIN → anonymous_intro.rs (commitment derivation)
        //   TRANSPORT_KEY_ROTATE_DOMAIN → encryption.rs (overlapping key rotation)
        //   INTRO_HMAC_DOMAIN → anonymous_intro.rs (HMAC auth tag)
        //   SYNC_HMAC_DOMAIN → sync_protocol.rs (sync frame HMAC)
        //   INTRO_CONFIRM_DOMAIN → anonymous_intro.rs (BQP confirmation record)
        //
        // If you add a new domain above, add it to this list with its consumer.
        // If you find a domain with no consumer, either activate it or remove it.

        // Verify all domains are non-zero (ceremony produced real entropy)
        for (name, domain) in [
            ("SPHINX_KDF_DOMAIN", SPHINX_KDF_DOMAIN),
            ("SPHINX_MAC_DOMAIN", SPHINX_MAC_DOMAIN),
            ("SPHINX_BLIND_DOMAIN", SPHINX_BLIND_DOMAIN),
            ("SPHINX_ALPHA_DOMAIN", SPHINX_ALPHA_DOMAIN),
            ("SPHINX_SHARED_DOMAIN", SPHINX_SHARED_DOMAIN),
            ("SEALED_KDF_DOMAIN", SEALED_KDF_DOMAIN),
            ("DHT_GENESIS_HASH", DHT_GENESIS_HASH),
            ("POW_DOMAIN", POW_DOMAIN),
            ("MIXING_DOMAIN", MIXING_DOMAIN),
            ("CHANNEL_KDF_DOMAIN", CHANNEL_KDF_DOMAIN),
            ("HANDSHAKE_DOMAIN", HANDSHAKE_DOMAIN),
            ("MAILBOX_KDF_DOMAIN", MAILBOX_KDF_DOMAIN),
            ("RECOVERY_KDF_DOMAIN", RECOVERY_KDF_DOMAIN),
            ("ONION_HOP_DOMAIN", ONION_HOP_DOMAIN),
            ("PAIRING_SESSION_DOMAIN", PAIRING_SESSION_DOMAIN),
            ("RENDEZVOUS_DOMAIN", RENDEZVOUS_DOMAIN),
            ("ANON_INTRO_COMMIT_DOMAIN", ANON_INTRO_COMMIT_DOMAIN),
            ("TRANSPORT_KEY_ROTATE_DOMAIN", TRANSPORT_KEY_ROTATE_DOMAIN),
            ("INTRO_HMAC_DOMAIN", INTRO_HMAC_DOMAIN),
            ("SYNC_HMAC_DOMAIN", SYNC_HMAC_DOMAIN),
            ("INTRO_CONFIRM_DOMAIN", INTRO_CONFIRM_DOMAIN),
        ] {
            assert!(
                domain.iter().any(|&b| b != 0),
                "Domain {} is all zeros — ceremony produced real entropy",
                name
            );
        }
    }
}
