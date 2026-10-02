//! Honesty pins for `POST /api/v2/messages/broadcast`.
//!
//! Observed defect (source-inspected 2026-10-02, fallback-queue item):
//! when `Core::send_message` returned `Err`, the handler still answered
//! HTTP 200 with `status: "BROADCAST_QUEUED"` / `delivery: "queued"` and
//! logged "Broadcast message sent" — but no code queues a failed broadcast:
//! the mesh `BroadcastMessage` handler only encodes and publishes, and the
//! retry loop (`handle_retries`) retransmits solely ids present in
//! `pending_messages`, into which only the direct-message path inserts
//! (`mesh.rs:2089`). "Queued" promised delivery that can never happen.
//!
//! These are source-level assertions for the same reason as
//! `tests/persistence_honesty.rs`: the defect is which branch answers the
//! caller, not otherwise observable without deliberately breaking transport.

use std::fs;

/// Read a source file relative to the crate root, with comments stripped.
///
/// Comments are stripped because the fix is documented in prose right next to
/// the code it changes, including the phrase `BROADCAST_QUEUED` explaining
/// what was removed.
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

/// Extract the body of `pub async fn broadcast(` up to the next method.
fn broadcast_body() -> String {
    let src = src("src/controllers/messaging.rs");
    let start = src
        .find("pub async fn broadcast(")
        .expect("broadcast handler not found in messaging.rs");
    let body = &src[start..];
    let end = body.find("\n    pub async fn ").unwrap_or(body.len());
    body[..end].to_string()
}

#[test]
fn broadcast_failure_is_never_reported_as_queued() {
    let body = broadcast_body();

    assert!(
        !body.contains("BROADCAST_QUEUED"),
        "a failed broadcast must not answer `BROADCAST_QUEUED`: nothing queues \
         it (the mesh retry loop only retransmits ids inserted into \
         pending_messages, which only the DM path does). The caller would wait \
         for delivery that can never happen."
    );
    assert!(
        !body.contains("\"queued\""),
        "`delivery: queued` promises a background retry that does not exist \
         for broadcasts."
    );
}

#[test]
fn broadcast_failure_returns_an_error_response() {
    let body = broadcast_body();

    // The handler must branch on the send result and turn the Err arm into
    // an error response, not fall through to ApiV2Response::success.
    assert!(
        body.contains("match send_result"),
        "broadcast must match on send_result; pre-fix code used \
         `if send_result.is_ok()` twice and answered success either way."
    );
    assert!(
        body.contains("Err(e)"),
        "the Err arm of the send result must be handled explicitly."
    );
    assert!(
        body.contains("error_response"),
        "the Err arm must return an error response, not HTTP 200."
    );
}

#[test]
fn broadcast_sent_log_only_runs_on_success() {
    let body = broadcast_body();

    // Pre-fix, `tracing::info!("Broadcast message sent to {} peers", ...)` ran
    // unconditionally. Post-fix it must sit inside the Ok arm of the match,
    // which the structural test above pins; here we pin that no "sent" info
    // log remains outside a match by requiring the success log line to appear
    // after `Ok(` within the match.
    let match_pos = body
        .find("match send_result")
        .expect("match on send_result missing");
    let sent_pos = body
        .find("Broadcast message sent to")
        .expect("the success log line was deleted entirely; it belongs in the Ok arm");
    let ok_arm_pos = body[match_pos..]
        .find("Ok(")
        .map(|p| match_pos + p)
        .expect("Ok arm missing in match on send_result");
    assert!(
        sent_pos > ok_arm_pos,
        "the 'Broadcast message sent' log must live inside the Ok arm, not \
         before the match where the Err path would also emit it."
    );
}
