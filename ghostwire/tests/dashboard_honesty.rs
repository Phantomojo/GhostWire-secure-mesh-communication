//! Honesty tests for `GET /api/v2/security/dashboard` (src/v2_routes.rs).
//!
//! The P0 honesty audit found `encryption_active: true` hardcoded at
//! v2_routes.rs:53 — a locked node with no key material loaded reported
//! encryption as active. Every endpoint must report truth or an honest error.
//!
//! Source-level assertions (comment-stripped, like tests/persistence_honesty.rs)
//! because the defect is which value is wired into the response struct, not
//! observable without deliberately locking a profile against a live server.

use std::fs;

/// Read a source file relative to the crate root, with comments stripped.
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

/// Isolate the `get_security_dashboard` body so assertions elsewhere in the
/// file cannot satisfy (or fail) them for the wrong reason.
fn dashboard_body() -> String {
    let file = src("src/v2_routes.rs");
    let start = file
        .find("pub async fn get_security_dashboard")
        .expect("get_security_dashboard not found");
    let body = &file[start..];
    let end = body
        .find("pub struct SecurityDashboard")
        .expect("SecurityDashboard struct not found");
    body[..end].to_string()
}

#[test]
fn security_dashboard_never_hardcodes_encryption_active_true() {
    let body = dashboard_body();
    assert!(
        !body.contains("encryption_active: true"),
        "GET /api/v2/security/dashboard still hardcodes `encryption_active: \
         true`. A locked node with no key material must not report encryption \
         as active — derive the value from the encryption manager's lock state."
    );
}

#[test]
fn security_dashboard_derives_encryption_from_lock_state() {
    let body = dashboard_body();
    assert!(
        body.contains("encryption_active: !") && body.contains("is_locked()"),
        "GET /api/v2/security/dashboard must derive `encryption_active` from \
         `EncryptionManager::is_locked()` (fails closed on a poisoned mutex), \
         not from a literal. Expected `encryption_active: !...is_locked()` in \
         the handler body."
    );
}
