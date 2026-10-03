//! The RNS identity must be per-node, persisted securely, and its write errors
//! must not be swallowed.
//!
//! Three distinct defects in `core/reticulum.rs`, all in the same ~40-line
//! persistence block:
//
//! 1. **PLAINTEXT AT REST.** `std::fs::write(&identity_path,
//!    id.to_private_key_bytes())` writes the RNS private key in the clear. The
//!    file contains the raw key bytes — anyone who reads it has the node's RNS
//!    identity and can impersonate it. `zeroize`/`SecretString` had zero hits in
//!    the file, so this was the only persistence path available.
//
//! 2. **SWALLOWED WRITE ERRORS.** Both writes use `let _ = ...`. If the write
//!    fails, the node logs "persisting to …", continues with an in-memory
//!    identity, and silently gets a NEW address hash on the next boot. That is
//!    silent identity loss, and it surfaces as an unreachable-peer bug that
//!    names the network rather than the failed write.
//
//! 3. **`GHOSTWIRE_HOME` BYPASSED.** This is the one the audit did not report.
//!    The path came from `dirs::config_dir()` — the ONLY use of that call in
//!    the entire file — while everything else in GhostWire resolves its home
//!    through `GHOSTWIRE_HOME` (see `daemon::ghostwire_home`). Consequence: two
//!    nodes with separate `GHOSTWIRE_HOME` values on ONE machine still shared a
//!    single `rns_identity.bin`, so they presented the SAME RNS identity to the
//!    network. Any two-node test that isolates homes to keep nodes independent
//!    was silently not independent on the RNS transport.
//!
//! `GHOSTWIRE_HOME` is load-bearing for this project's multi-node testing, and
//! an identity collision would make RNS results untrustworthy — so this is a
//! test-integrity defect as much as a security one.

use std::fs;

fn source() -> String {
    fs::read_to_string("src/core/reticulum.rs").expect("src/core/reticulum.rs must be readable")
}

#[test]
fn rns_identity_respects_ghostwire_home() {
    let src = source();

    // The defect: this path came from `dirs::config_dir()`, which ignores
    // GHOSTWIRE_HOME. Two nodes with separate homes on one machine then shared
    // a single rns_identity.bin and presented the same RNS identity to the
    // network.
    //
    // Matched by LINES THAT USE IT, not by substring. The fix's explanatory
    // comment necessarily names the call it removed, so a bare `contains` fails
    // against correct code — the test would be certifying that nobody documents
    // their own bug fixes.
    let uses_config_dir = src
        .lines()
        .any(|l| !l.trim_start().starts_with("//") && l.contains("dirs::config_dir()"));
    assert!(
        !uses_config_dir,
        "reticulum.rs must not resolve the RNS identity path via \
         dirs::config_dir(). That ignores GHOSTWIRE_HOME, so two nodes with \
         separate homes on one machine share a single rns_identity.bin and \
         present the same RNS identity to the network."
    );

    // The fix delegates to the one canonical resolver rather than reimplementing
    // the lookup. Checking for the literal env var name would be wrong: the
    // corrected code never mentions GHOSTWIRE_HOME itself, it calls the shared
    // helper that owns that decision. Asserting the delegation is what actually
    // prevents the bypass from coming back.
    assert!(
        src.contains("crate::daemon::ghostwire_home()"),
        "reticulum.rs must resolve the RNS identity path through \
         crate::daemon::ghostwire_home(), the canonical GHOSTWIRE_HOME-aware \
         resolver."
    );

    // And that resolver must genuinely honour the env var.
    let daemon = fs::read_to_string("src/daemon.rs").expect("src/daemon.rs readable");
    let home_at = daemon
        .find("pub fn ghostwire_home()")
        .expect("ghostwire_home must exist and be public");
    let body = &daemon[home_at..(home_at + 400).min(daemon.len())];
    assert!(
        body.contains("GHOSTWIRE_HOME"),
        "the canonical resolver must read GHOSTWIRE_HOME"
    );
}

#[test]
fn rns_identity_is_not_written_in_plaintext() {
    let src = source();
    assert!(
        !src.contains("to_private_key_bytes())"),
        "the RNS private key must not be written to disk verbatim. It is the \
         raw key: anyone who reads the file can impersonate the node's RNS \
         identity. Encrypt it at rest, or reuse the project's existing secure \
         key storage."
    );
}

#[test]
fn rns_identity_write_errors_are_not_swallowed() {
    let src = source();
    assert!(
        !src.contains("let _ = std::fs::write(&identity_path"),
        "the RNS identity write must not discard its error with `let _ =`. A \
         failed write means the node runs this session with an identity it \
         cannot reuse, and silently gets a different address hash on the next \
         boot. Propagate the error or fail loudly."
    );
    assert!(
        !src.contains("let _ = std::fs::create_dir_all"),
        "directory creation for the RNS identity must not discard its error"
    );
}

// UNREACHABLE — read this before acting on the other three.
//
// While fixing the plaintext write, the enclosing signature turned out to be:
//
//     pub async fn new(interfaces_ini: &str, node_name: &str, master_seed: Option<[u8; 32]>)
//
// and its ONLY caller passes `Some(transport_seed)`, where
//
//     transport_seed = identity.derive_transport_key("reticulum")
//                    = SHA256(public_key || "reticulum")
//
// So `master_seed` is always `Some`, the `None` arm never executes, and the
// plaintext write is DEAD CODE. No RNS private key has ever been written to
// disk in the clear by this path, because this path does not run.
//
// That is a much better outcome than "it was fixed" — and it is also a trap:
// the code still LOOKS like it writes a key in plaintext, so the next reader
// (and the parallel audit that reported it) concludes there is a live
// vulnerability. There is not one. There is a misleading branch.
//
// The correct fix for the remaining three assertions is therefore NOT to
// encrypt anything. It is to make the unreachable state either
// reachable-and-safe or deleted. Until someone decides which, these tests
// document what is true and fail loudly if the code ever starts running again.

// -----------------------------------------------------------------------------

/// The identity file must never be readable as raw key bytes by anything but
/// this process. This asserts the property structurally — the persisted blob is
/// wrapped, not bare — rather than trying to execute the persistence path,
/// which needs a live RNS transport.
#[test]
fn the_persisted_identity_is_wrapped_not_bare() {
    let src = source();

    // Scope to the persistence region. The first version searched the whole
    // file and passed for the wrong reason: `encrypt` appears only in doc
    // comments and in an unrelated `encryption_errors` counter field, so a
    // keyword search found "encryption" with no protection step anywhere near
    // the write. A source-grep assertion must be anchored to the region it is
    // about, or it certifies prose.
    let region_start = src
        .find("rns_identity")
        .expect("the rns_identity path must exist");
    let region = &src[region_start..(region_start + 3000).min(src.len())];

    let has_protection = region.contains("seal")
        || region.contains("encrypt")
        || region.contains("EncryptedStore")
        || region.contains("secure_store")
        || region.contains("keyslot")
        || region.contains("passphrase");

    assert!(
        has_protection,
        "no seal/encrypt/passphrase step appears near the RNS identity write, so \
         the raw key bytes are what reach disk."
    );
}