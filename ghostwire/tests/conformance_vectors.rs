//! Execute the conformance vectors.
//!
//! # The point of this file
//!
//! `spec/vectors.json` is only useful if something checks it. This is that
//! something: it regenerates every normative encoding and compares it to the
//! committed vector.
//!
//! So the failure mode is inverted from an ordinary test suite. Instead of "the
//! code changed and the test broke", it is "the protocol changed and CI said so".
//! That is the difference between an application that implements a protocol and
//! an application that is becoming one.
//!
//! # What a failure means
//!
//! A failure here is NOT necessarily a bug — it is a PROTOCOL CHANGE. Someone must
//! decide whether the encoding moved deliberately (and the spec version goes up)
//! or accidentally (and the code is wrong). `spec/vectors.json` must be updated
//! in the same commit as the change that caused it, so the diff shows both
//! together and review can see the intent.
//!
//! # What is deliberately absent
//!
//! No vector for the libp2p PeerId derivation. That is libp2p's specification,
//! not GhostWire's, and a vector for it would imply we own it. It is covered
//! indirectly: `did-from-public-key` records the DID a given key produces, which
//! is the part GhostWire *does* define.
//!
//! No vector for anything still implementation-defined. A vector set that
//! enshrines a defect is worse than none, because the defect becomes the
//! normative behaviour by reference.

use ghostwire::core::deref::did_from_keypair;
use ghostwire::core::discovery_bridge::{AddressExchange, DiscoveryAnnounce};
use ghostwire::core::identity_pairing::device_pairing::PairingCode;
use ghostwire::core::pairing::PairingRecord;
use ghostwire::core::rotation::Revocation;
use libp2p::identity::Keypair;
use serde_json::Value;
use std::path::Path;

const VECTORS: &str = include_str!("../spec/vectors.json");

/// The fixed instant every vector is pinned to. Real time would make the set
/// irreproducible the moment it was published.
const T0: u64 = 1_800_000_000;

fn load() -> Vec<Value> {
    serde_json::from_str::<Value>(VECTORS).expect("vectors.json is valid JSON")["vectors"]
        .as_array()
        .expect("vectors is an array")
        .clone()
}

fn find<'a>(vs: &'a [Value], name: &str) -> &'a Value {
    vs.iter()
        .find(|v| v["name"] == name)
        .unwrap_or_else(|| panic!("vector '{name}' is missing from the set"))
}

fn unhex(s: &str) -> Vec<u8> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16).expect("valid hex"))
        .collect()
}

fn tohex(b: &[u8]) -> String {
    b.iter().map(|x| format!("{x:02x}")).collect()
}

/// The fixed key every vector derives from.
/// The fixed secret every vector derives from.
fn fixed_secret() -> [u8; 32] {
    [
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0a, 0x0b, 0x0c, 0x0d, 0x0e, 0x0f, 0x10,
        0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1b, 0x1c, 0x1d, 0x1e, 0x1f, 0x20,
    ]
}

fn fixed_key() -> (Keypair, String, String, Vec<u8>) {
    let secret = fixed_secret();
    let kp = Keypair::ed25519_from_bytes(secret).expect("valid ed25519 secret");
    let did = did_from_keypair(&kp);
    let pid = libp2p::PeerId::from(kp.public()).to_string();
    let proto = kp.public().encode_protobuf();
    (kp, did, pid, proto)
}

fn raw_key32(secret: &[u8; 32]) -> [u8; 32] {
    let kp = Keypair::ed25519_from_bytes(*secret).expect("valid");
    let proto = kp.public().encode_protobuf();
    proto[proto.len() - 32..].try_into().expect("32-byte key")
}

/// A vector that changes is a PROTOCOL change, so the message says so rather than
/// describing a test failure.
macro_rules! assert_vector {
    ($vector:expr, $name:expr, $actual:expr, $expected_path:expr) => {{
        let expected = &$vector["outputs"][$expected_path];
        let expected_s = expected.as_str().unwrap_or("");
        assert_eq!(
            $actual, expected_s,
            "VECTOR '{}' FAILED.\n\
             A normative encoding changed. This is a PROTOCOL change, not just a \
             refactor: decide deliberately whether it should move, bump the spec \
             version, and update spec/vectors.json in the SAME commit.",
            $name
        );
    }};
}

#[test]
fn did_from_public_key_matches_the_vector() {
    let vs = load();
    let v = find(&vs, "did-from-public-key");
    let (_kp, did, pid, proto) = fixed_key();

    assert_eq!(tohex(&proto), v["inputs"]["public_key_protobuf_hex"].as_str().unwrap());
    assert_vector!(v, "did-from-public-key", did.as_str(), "did");
    assert_vector!(v, "did-from-public-key", pid.as_str(), "peer_id");
}

#[test]
fn pairing_record_signing_bytes_match_the_vector() {
    let vs = load();
    let v = find(&vs, "pairing-record-signing-bytes");
    let (kp, _, _, _) = fixed_key();

    let rec = PairingRecord::create(
        &kp,
        vec![v["inputs"]["addresses"][0].as_str().unwrap().to_string()],
        v["inputs"]["issued_at"].as_u64().unwrap(),
        (v["inputs"]["expires_at"].as_u64().unwrap()) - v["inputs"]["issued_at"].as_u64().unwrap(),
    );

    assert_vector!(v, "pairing-record-signing-bytes", tohex(&rec.signing_bytes()), "signing_bytes_hex");
    assert_vector!(v, "pairing-record-signing-bytes", tohex(&rec.signature), "signature_hex");
}

#[test]
fn revocation_signing_bytes_match_the_vector() {
    let vs = load();
    let v = find(&vs, "revocation-signing-bytes");
    let (kp, _, _, _) = fixed_key();

    let rev = Revocation::create(
        &kp,
        v["inputs"]["revoked_at"].as_u64().unwrap(),
        v["inputs"]["reason"].as_str().unwrap(),
    );

    assert_vector!(v, "revocation-signing-bytes", tohex(&rev.signing_bytes()), "signing_bytes_hex");
    assert_vector!(v, "revocation-signing-bytes", tohex(&rev.signature), "signature_hex");
}

#[test]
fn discovery_announce_signing_bytes_match_the_vector() {
    let vs = load();
    let v = find(&vs, "discovery-announce-signing-bytes");
    let (kp, did, pid, _) = fixed_key();
    let secret = fixed_secret();

    let mut a = DiscoveryAnnounce::new(
        v["inputs"]["network_id"].as_str().unwrap(),
        &pid,
        &did,
        raw_key32(&secret),
        v["inputs"]["sequence"].as_u64().unwrap(),
    );
    a.issued_at = v["inputs"]["issued_at"].as_u64().unwrap();
    a.expires_at = v["inputs"]["expires_at"].as_u64().unwrap();
    let a = a.sign(&secret).expect("sign");

    assert_vector!(v, "discovery-announce-signing-bytes", tohex(&a.signing_bytes().unwrap()), "signing_bytes_hex");
    assert_vector!(v, "discovery-announce-signing-bytes", tohex(&a.signature), "signature_hex");
}

#[test]
fn address_exchange_signing_bytes_match_the_vector() {
    let vs = load();
    let v = find(&vs, "address-exchange-signing-bytes");
    let (kp, did, pid, _) = fixed_key();
    let secret = fixed_secret();

    let addrs: Vec<String> = v["inputs"]["addresses"]
        .as_array()
        .unwrap()
        .iter()
        .map(|s| s.as_str().unwrap().to_string())
        .collect();
    let mut e = AddressExchange::new(
        v["inputs"]["network_id"].as_str().unwrap(),
        &pid,
        &did,
        addrs,
        v["inputs"]["sequence"].as_u64().unwrap(),
    );
    e.issued_at = v["inputs"]["issued_at"].as_u64().unwrap();
    e.expires_at = v["inputs"]["expires_at"].as_u64().unwrap();
    let e = e.sign(&secret).expect("sign");

    assert_vector!(v, "address-exchange-signing-bytes", tohex(&e.signing_bytes().unwrap()), "signing_bytes_hex");
    assert_vector!(v, "address-exchange-signing-bytes", tohex(&e.signature), "signature_hex");
}

#[test]
fn address_boundary_unambiguity_holds() {
    let vs = load();
    let v = find(&vs, "address-boundary-unambiguity");
    let (kp, did, pid, _) = fixed_key();

    let mk = |addrs: Vec<&str>| {
        let mut e = AddressExchange::new(
            "ghostwire-main",
            &pid,
            &did,
            addrs.into_iter().map(String::from).collect(),
            11,
        );
        e.issued_at = v["inputs"]["issued_at"].as_u64().unwrap();
        e.expires_at = v["inputs"]["expires_at"].as_u64().unwrap();
        e
    };
    let a = mk(vec!["ab", "c"]);
    let b = mk(vec!["a", "bc"]);

    assert_vector!(v, "address-boundary-unambiguity", tohex(&a.signing_bytes().unwrap()), "case_a_signing_bytes_hex");
    assert_vector!(v, "address-boundary-unambiguity", tohex(&b.signing_bytes().unwrap()), "case_b_signing_bytes_hex");

    assert_ne!(
        a.signing_bytes().unwrap(),
        b.signing_bytes().unwrap(),
        "[\"ab\",\"c\"] and [\"a\",\"bc\"] MUST differ. Without explicit length \
         prefixes an attacker can shift bytes across a field boundary and produce \
         a valid signature over different content."
    );
}

#[test]
fn pairing_code_roundtrip_matches_the_vector() {
    let vs = load();
    let v = find(&vs, "pairing-code-roundtrip");

    let entropy: [u8; 16] = unhex(v["inputs"]["entropy_hex"].as_str().unwrap())
        .try_into()
        .expect("16 bytes");
    let code = PairingCode::from_entropy_for_vectors(entropy);

    assert_vector!(v, "pairing-code-roundtrip", code.display.as_str(), "display");
    assert_vector!(v, "pairing-code-roundtrip", code.safety_number().as_str(), "safety_number");

    // The property that matters: typing the code into the other side must
    // reproduce the same safety number. This is what makes pairing work without
    // meeting in person, and it was broken until the base32 round trip was fixed.
    let retyped = PairingCode::from_string(&code.display).expect("decode");
    assert_eq!(
        code.safety_number(),
        retyped.safety_number(),
        "a typed pairing code must reconstruct the same safety number on both \
         sides. If this fails, pairing cannot complete at any distance."
    );
    assert_eq!(
        code.display, retyped.display,
        "encode() and decode() must be inverses"
    );
}

/// The set itself must stay honest: every vector has the required shape.
#[test]
fn every_vector_is_well_formed() {
    let vs = load();
    assert!(!vs.is_empty(), "the vector set must not be empty");
    for v in &vs {
        for field in ["name", "description", "inputs", "outputs"] {
            assert!(
                v.get(field).is_some(),
                "vector '{}' is missing the required field '{field}'",
                v["name"]
            );
        }
        assert!(
            !v["description"].as_str().unwrap().is_empty(),
            "vector '{}' must say what it pins and why",
            v["name"]
        );
        assert!(
            !v["outputs"].as_object().unwrap().is_empty(),
            "vector '{}' must assert at least one output",
            v["name"]
        );
    }
}

/// The vectors file must be present and parseable at build time, not only when a
/// test happens to run. `include_str!` above already guarantees the file exists.
#[test]
fn the_vector_file_is_committed() {
    assert!(
        Path::new("spec/vectors.json").exists() || VECTORS.len() > 2,
        "spec/vectors.json must be part of the repository — it is the artifact \
         other implementations conform to"
    );
    assert!(T0 > 0, "the pinned instant must be fixed");
}
