//! Regression tests for the silent-persistence and plaintext-fallback defects
//! found by running a real two-machine message send on 2026-09-28.
//!
//! Observed live: `POST /api/v2/messages/send` returned
//!
//! ```text
//! 200 OK  {"status":"accepted","delivery":"receipt_pending"}
//! ```
//!
//! while the message was never stored anywhere, and the only evidence was
//!
//! ```text
//! ERROR ghostwire::core: ⚠️ Persistence failure: persistence is locked
//! ```
//!
//! And `Core::send_direct_message` had a second, worse variant: when the
//! encrypted write failed it fell back to `save_message_plain`, writing a
//! direct message to disk **in the clear**, traced only at `debug` level.
//!
//! These tests pin the corrected behaviour. They are source-level assertions
//! because the defects are about which call is made on an error path, which is
//! not otherwise observable without a profile deliberately left locked.

use std::fs;

/// Read a source file relative to the crate root, with comments stripped.
///
/// Comments are removed because the fixes are documented in prose right next to
/// the code they change — including the phrase `save_message_plain` in the
/// comment explaining why it is gone. Matching a comment would make the test
/// fail for the right reason in the wrong place.
fn src(path: &str) -> String {
    let root = env!("CARGO_MANIFEST_DIR");
    let raw = fs::read_to_string(format!("{root}/{path}"))
        .unwrap_or_else(|e| panic!("could not read {path}: {e}"));
    raw.lines()
        .map(|l| match l.find("//") {
            Some(i) => &l[..i],
            None => l,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn direct_message_never_falls_back_to_plaintext_storage() {
    let core = src("src/core/mod.rs");

    // Isolate send_direct_message so a match elsewhere in the file cannot
    // satisfy (or fail) this assertion for the wrong reason.
    let start = core
        .find("pub async fn send_direct_message")
        .expect("send_direct_message not found");
    let body = &core[start..];
    let end = body.find("\n    pub async fn ").unwrap_or(body.len());
    let body = &body[..end];

    assert!(
        !body.contains("save_message_plain"),
        "send_direct_message must not fall back to plaintext storage. A direct \
         message that fails to encrypt must lose its local copy, not be written \
         in the clear. Found a save_message_plain call in the body."
    );
}

#[test]
fn direct_message_propagates_a_persistence_failure() {
    let core = src("src/core/mod.rs");
    let start = core
        .find("pub async fn send_direct_message")
        .expect("send_direct_message not found");
    let body = &core[start..];
    let end = body.find("\n    pub async fn ").unwrap_or(body.len());
    let body = &body[..end];

    // The write must not be discarded with a bare `tracing::error!` and `Ok(())`.
    assert!(
        !body.contains("tracing::error!(\"⚠️ Persistence failure"),
        "send_direct_message still logs and swallows a persistence failure; \
         the caller is told `accepted` and no copy exists."
    );
    assert!(
        body.contains("save_message(message).map_err"),
        "send_direct_message should return the persistence error to the caller."
    );
}

#[test]
fn broadcast_path_still_returns_a_persistence_failure() {
    let core = src("src/core/mod.rs");
    let start = core
        .find("pub async fn send_message")
        .expect("send_message not found");
    let body = &core[start..];
    let end = body.find("\n    pub async fn ").unwrap_or(body.len());
    let body = &body[..end];

    assert!(
        !body.contains("Persistence failure for broadcast"),
        "the broadcast path still swallows persistence failures."
    );
}

#[test]
fn profile_created_at_is_not_the_current_time() {
    let identity = src("src/controllers/identity.rs");

    // This bug was `created_at: chrono::Utc::now().to_rfc3339()`, evaluated on
    // every request, so two GETs a minute apart disagreed about when the
    // profile was created.
    assert!(
        !identity.contains("created_at: chrono::Utc::now()"),
        "created_at must be read from stored state, not evaluated per request."
    );
}

#[test]
fn profile_locked_reflects_persistence_not_the_identity_lock() {
    let identity = src("src/controllers/identity.rs");

    // `Core::is_locked()` is the identity subsystem; it stayed `false` while
    // every persistence write failed. The field callers act on must be the
    // one that actually blocks writes.
    assert!(
        identity.contains("persistence_is_unlocked"),
        "the `locked` field must report persistence readiness; the identity \
         lock is a different subsystem and can be open while writes fail."
    );
    assert!(
        identity.contains("identity_locked"),
        "the identity lock should be surfaced under its own name rather than \
         conflated with persistence."
    );
}

#[test]
fn persistence_locked_maps_to_service_unavailable() {
    let types = src("src/api_v2_types.rs");
    assert!(
        types.contains("PERSISTENCE_LOCKED"),
        "a locked profile should be 503, not 500: the node is healthy and the \
         caller just needs to unlock."
    );
}

#[test]
fn dev_profile_opens_machine_keyed_storage() {
    let core = src("src/core/mod.rs");
    assert!(
        core.contains("GHOSTWIRE_DEV_PROFILE"),
        "a dev node must be selectable so tests have a node that can actually \
         store messages without a human unlocking a passphrase."
    );
    assert!(
        core.contains("new_machine_mode_with_base_dir"),
        "dev mode should open machine-keyed persistence, not the locked profile."
    );
    // Dev mode must be opt-in and never the default.
    let main_rs = src("src/main.rs");
    let default_is_locked =
        core.contains("Arc::new(persistence_manager::PersistenceManager::new_locked(");
    assert!(
        default_is_locked,
        "the non-dev path must still open a locked profile; a real identity \
         should not silently become machine-keyed."
    );
    assert!(
        main_rs.contains("dev-profile") || main_rs.contains("dev_profile"),
        "dev mode should be reachable from the CLI."
    );
}

#[test]
fn dev_profile_refuses_a_passphrase_protected_profile() {
    let pm = src("src/core/persistence_manager.rs");
    // A profile created with a passphrase must not be silently downgraded to
    // machine-keyed storage just because the env var is set.
    assert!(
        pm.contains("cannot be opened in machine mode")
            || pm.contains("cannot be opened") && pm.contains("machine mode"),
        "machine mode must refuse a user-mode profile rather than ignoring its \
         passphrase protection."
    );
}
