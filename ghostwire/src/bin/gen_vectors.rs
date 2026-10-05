//! Generate the GhostWire conformance vector set.
//!
//! # Why this exists
//!
//! The research on what makes a protocol a protocol produced a clear ranking, and
//! the top item was: **a machine-readable vector file plus an executor**. It is
//! the only artifact that makes every other artifact testable.
//!
//! Without it, "does your implementation agree with mine?" is answered by reading
//! two pieces of prose and hoping. With it, the answer is a command.
//!
//! This is also the concrete lesson from Reticulum. Four-plus third parties wrote
//! competing byte-level specifications against different version targets and none
//! of them interoperate. They were not short of documentation; they had no vectors.
//!
//! # What is covered
//!
//! Only encodings that are NORMATIVE — meaning a second implementation must
//! reproduce them byte for byte:
//!
//! 1. DID derivation from an ed25519 public key
//! 2. Canonical signing form for `DiscoveryAnnounce`
//! 3. Canonical signing form for `AddressExchange`
//! 4. Canonical signing form for `PairingRecord`
//! 5. Canonical signing form for `Revocation`
//! 6. The base32 pairing code (`encode`/`decode` must be inverses)
//!
//! Deliberately NOT included: anything wire-framing that is still
//! implementation-defined, and the libp2p PeerId derivation, which is libp2p's
//! specification rather than GhostWire's. A vector set that enshrines a defect is
//! worse than none, so anything not yet settled stays out.
//!
//! # Running
//!
//! ```text
//! cargo run --bin gen-vectors > vectors.json
//! cargo test --test conformance_vectors   # executor
//! ```
//!
//! The executor must FAIL if the implementation changes a normative encoding. That
//! is the point: a change in output is a visible, deliberate protocol change rather
//! than a silent drift.

use ghostwire::core::deref::did_from_keypair;
use ghostwire::core::discovery_bridge::{AddressExchange, DiscoveryAnnounce};
use ghostwire::core::pairing::PairingRecord;
use ghostwire::core::rotation::Revocation;
use libp2p::identity::Keypair;
use std::io::Write;

/// `DiscoveryAnnounce::new` wants a bare `[u8; 32]`, not the protobuf encoding.
fn pk_bytes32(secret: &[u8; 32]) -> [u8; 32] {
    let pk = Keypair::ed25519_from_bytes(*secret)
        .expect("valid key")
        .public()
        .clone();
    // Ed25519 protobuf is the 32-byte key preceded by a short type prefix.
    let proto = pk.encode_protobuf();
    proto[proto.len() - 32..]
        .try_into()
        .expect("ed25519 protobuf carries a 32-byte key")
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|b| format!("{b:02x}")).collect()
}

fn vector(name: &str, description: &str, inputs: serde_json::Value, outputs: serde_json::Value) -> serde_json::Value {
    serde_json::json!({
        "name": name,
        "description": description,
        "inputs": inputs,
        "outputs": outputs,
    })
}

fn main() {
    let mut vectors = Vec::new();

    // ---------------------------------------------------------------- identity
    // A FIXED key, not a random one: vectors must be reproducible.
    let secret: [u8; 32] = [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
        0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
    ];
    let keypair = Keypair::ed25519_from_bytes(secret).expect("valid ed25519 secret");
    let did = did_from_keypair(&keypair);
    let peer_id = libp2p::PeerId::from(keypair.public()).to_string();
    let pk_bytes = keypair.public().encode_protobuf();

    vectors.push(vector(
        "did-from-public-key",
        "did:ghost:<lowercase hex SHA-256 of the ed25519 public key's protobuf encoding>. \
         A DID is a VERIFICATION anchor, never something a PeerId can be inverted from.",
        serde_json::json!({
            "public_key_protobuf_hex": hex(&pk_bytes),
        }),
        serde_json::json!({
            "did": did,
            "peer_id": peer_id,
        }),
    ));

    // ------------------------------------------------------- pairing record
    let pairing = PairingRecord::create(
        &keypair,
        vec!["/ip4/203.0.113.5/tcp/4001".to_string()],
        1_800_000_000,
        86_400,
    );
    vectors.push(vector(
        "pairing-record-signing-bytes",
        "Length-prefixed canonical form: version byte, then each field prefixed with its \
         u32 big-endian length. Unambiguous under concatenation.",
        serde_json::json!({
            "version": pairing.version,
            "did": pairing.did,
            "peer_id": pairing.peer_id,
            "addresses": pairing.addresses,
            "issued_at": pairing.issued_at,
            "expires_at": pairing.expires_at,
        }),
        serde_json::json!({
            "signing_bytes_hex": hex(&pairing.signing_bytes()),
            "signature_hex": hex(&pairing.signature),
        }),
    ));

    // ------------------------------------------------------------- revocation
    let revocation = Revocation::create(&keypair, 1_800_000_000, "key compromise");
    vectors.push(vector(
        "revocation-signing-bytes",
        "Self-signed by the revoked key. Revocations never expire: a key compromised last \
         year is still compromised, and a TTL would let an attacker wait it out.",
        serde_json::json!({
            "version": revocation.version,
            "did": revocation.did,
            "peer_id": revocation.peer_id,
            "revoked_at": revocation.revoked_at,
            "reason": revocation.reason,
        }),
        serde_json::json!({
            "signing_bytes_hex": hex(&revocation.signing_bytes()),
            "signature_hex": hex(&revocation.signature),
        }),
    ));

    // ----------------------------------------------------- discovery announce
    // `new()` stamps issued_at from the system clock, which would make the vector
    // irreproducible. Pin it to the same fixed instant used everywhere else.
    let announce_pk = pk_bytes32(&secret);
    let mut announce =
        DiscoveryAnnounce::new("ghostwire-main", &peer_id, &did, announce_pk, 7);
    announce.issued_at = 1_800_000_000;
    announce.expires_at = 1_800_003_600;
    let announce = announce.sign(&secret).expect("sign announce");
    vectors.push(vector(
        "discovery-announce-signing-bytes",
        "Domain-separated canonical form. MUST NOT contain any address: a public announce \
         must not be able to express 'dial this endpoint', which is what phase 1 withholds.",
        serde_json::json!({
            "version": announce.version,
            "network_id": announce.network_id,
            "peer_id": announce.peer_id,
            "did": announce.did,
            "public_key_hex": hex(&announce.public_key),
            "issued_at": announce.issued_at,
            "expires_at": announce.expires_at,
            "sequence": announce.sequence,
        }),
        serde_json::json!({
            "signing_bytes_hex": hex(&announce.signing_bytes().expect("bytes")),
            "signature_hex": hex(&announce.signature),
        }),
    ));

    // ------------------------------------------------------ address exchange
    let mut exchange = AddressExchange::new(
        "ghostwire-main",
        &peer_id,
        &did,
        vec!["/ip4/203.0.113.5/tcp/4001".to_string()],
        9,
    );
    exchange.issued_at = 1_800_000_000;
    exchange.expires_at = 1_800_003_600;
    let exchange = exchange.sign(&secret).expect("sign exchange");
    vectors.push(vector(
        "address-exchange-signing-bytes",
        "Same canonical shape as the announce but a DIFFERENT domain separator, so a signed \
         public announce cannot be replayed as an authenticated address exchange.",
        serde_json::json!({
            "version": exchange.version,
            "network_id": exchange.network_id,
            "peer_id": exchange.peer_id,
            "did": exchange.did,
            "addresses": exchange.addresses,
            "issued_at": exchange.issued_at,
            "expires_at": exchange.expires_at,
            "sequence": exchange.sequence,
        }),
        serde_json::json!({
            "signing_bytes_hex": hex(&exchange.signing_bytes().expect("bytes")),
            "signature_hex": hex(&exchange.signature),
        }),
    ));

    // ------------------------------------------------------------- ambiguity
    // The property that makes the length prefixes necessary, as a fixed pair.
    let mut amb_a = AddressExchange::new(
        "ghostwire-main", &peer_id, &did,
        vec!["ab".into(), "c".into()], 11,
    );
    amb_a.issued_at = 1_800_000_000;
    amb_a.expires_at = 1_800_003_600;
    let mut amb_b = AddressExchange::new(
        "ghostwire-main", &peer_id, &did,
        vec!["a".into(), "bc".into()], 11,
    );
    amb_b.issued_at = 1_800_000_000;
    amb_b.expires_at = 1_800_003_600;
    vectors.push(vector(
        "address-boundary-unambiguity",
        "[\"ab\",\"c\"] and [\"a\",\"bc\"] must produce different signing bytes. Without \
         explicit length prefixes these would concatenate identically, and an attacker \
         could shift bytes across a field boundary to sign different content.",
        serde_json::json!({
            "case_a_addresses": ["ab", "c"],
            "case_b_addresses": ["a", "bc"],
            // issued_at/expires_at MUST be pinned here too. AddressExchange::new
            // stamps them from the clock, so without this the generator and the
            // executor run a second apart and the vectors never match — which is
            // a broken vector set, not a protocol change.
            "issued_at": 1_800_000_000,
            "expires_at": 1_800_003_600,
        }),
        serde_json::json!({
            "case_a_signing_bytes_hex": hex(&amb_a.signing_bytes().expect("bytes")),
            "case_b_signing_bytes_hex": hex(&amb_b.signing_bytes().expect("bytes")),
            "distinct": true,
        }),
    ));

    // --------------------------------------------------------------- pairing code
    // The base32 round trip, on fixed bytes so it is reproducible.
    let code_entropy: [u8; 16] = [
        0x00, 0x11, 0x22, 0x33, 0x44, 0x55, 0x66, 0x77, 0x88, 0x99, 0xaa, 0xbb, 0xcc, 0xdd, 0xee, 0xff,
    ];
    let code = ghostwire::core::identity_pairing::device_pairing::PairingCode::from_entropy_for_vectors(
        code_entropy,
    );
    let code_text = code.display.clone();
    let retyped = ghostwire::core::identity_pairing::device_pairing::PairingCode::from_string(&code_text)
        .expect("decode the code");
    vectors.push(vector(
        "pairing-code-roundtrip",
        "16 bytes of entropy -> base32 (RFC 4648 alphabet) -> 26 characters grouped \
         4-4-4-4-4-4-4. encode() and decode() MUST be inverses, and both sides of a typed \
         pairing MUST derive the same safety number. This is what makes pairing possible \
         without meeting in person.",
        serde_json::json!({
            "entropy_hex": hex(&code_entropy),
        }),
        serde_json::json!({
            "display": code_text,
            "safety_number": code.safety_number(),
            "retyped_safety_number": retyped.safety_number(),
            "round_trip_is_exact": code.safety_number() == retyped.safety_number(),
        }),
    ));

    // ----------------------------------------------------------------- output
    let doc = serde_json::json!({
        "spec": "GhostWire",
        "version": 1,
        "note": "Normative encodings only. An implementation MUST reproduce these byte for \
                 byte. Changing one is a protocol version bump, not a refactor.",
        "vectors": vectors,
    });

    let mut out = std::io::stdout().lock();
    serde_json::to_writer_pretty(&mut out, &doc).expect("serialise vectors");
    writeln!(out).ok();
}
