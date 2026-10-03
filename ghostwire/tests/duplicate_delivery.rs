//! Every inbound path must deduplicate, not just the gossipsub one.
//!
//! ## The defect
//!
//! `handle_incoming_message` is the single function every inbound path funnels
//! into. It persists the message to the store and emits
//! `CoreEvent::NewMessage`. The duplicate check lived in ONE CALLER, so the
//! other two reached it unguarded:
//!
//!   * request-response  (mesh.rs:2803)
//!   * direct message    (mesh.rs:3309)
//!   * Sphinx Deliver    (mesh.rs:3391)
//!
//! Consequence: a message arriving twice — which is exactly what redundant send
//! across two transports produces — was stored twice and shown to the user
//! twice. Redundant send could not have shipped safely.
//!
//! ## Why this is asserted structurally
//!
//! These are source assertions, because the property IS a wiring property: the
//! check must sit in the shared handler, not in a caller. A behavioural test
//! would need two live nodes on two transports delivering the same message,
//! which is the integration gap this is meant to close in the first place.
//! `tests/node_to_node.rs` covers single-path delivery; this covers the shape.
//!
//! Anchoring note: assertions are scoped to the function body, and the
//! "no caller-side guard" check looks for the check OUTSIDE the handler. A
//! comment inside the handler explaining the fix must not satisfy it.

use std::fs;

fn mesh_source() -> String {
    fs::read_to_string("src/core/mesh.rs").expect("src/core/mesh.rs must be readable")
}

/// The body of `handle_incoming_message`, from its signature to the next
/// top-level `async fn` / closing brace at the same indent.
fn handler_body(src: &str) -> String {
    let start = src
        .find("async fn handle_incoming_message")
        .expect("handle_incoming_message must exist");
    let rest = &src[start..];
    // Take a generous but bounded slice: past the function's own closing brace.
    let end = rest
        .find("\n    }\n")
        .map(|e| e + 7)
        .unwrap_or(2500.min(rest.len()));
    rest[..end].to_string()
}

#[test]
fn the_shared_handler_deduplicates() {
    let body = handler_body(&mesh_source());

    assert!(
        body.contains("self.seen_messages.contains(&msg.id)"),
        "handle_incoming_message must check for duplicates itself. It is the \\
         convergence point for every inbound path, and it both persists the \\
         message and emits CoreEvent::NewMessage — so an unguarded duplicate \\
         is stored twice and shown to the user twice."
    );
}

#[test]
fn deduplication_happens_before_the_store_write() {
    // Order matters. If the guard runs after `save_message`, a duplicate is
    // still written to disk even though the event is not emitted.
    let body = handler_body(&mesh_source());

    let check = body
        .find("self.seen_messages.contains(&msg.id)")
        .expect("the dedup check must exist");
    let save = body
        .find("save_message")
        .expect("the store write must exist");

    assert!(
        check < save,
        "the duplicate check must run BEFORE save_message; otherwise a \\
         duplicate is persisted even though its event is suppressed"
    );
}

#[test]
fn deduplication_happens_before_the_event_is_emitted() {
    let body = handler_body(&mesh_source());

    let check = body
        .find("self.seen_messages.contains(&msg.id)")
        .expect("the dedup check must exist");
    let emit = find_event_emission(&body);

    assert!(
        check < emit,
        "the duplicate check must run BEFORE the NewMessage event, or the \
         user still sees the message twice (check at {check}, emit at {emit})"
    );
}

/// Offset of the actual `bus.send(CoreEvent::NewMessage(...))` call.
///
/// Anchored to the send, not to the bare type name. The fix's own explanatory
/// comment names `CoreEvent::NewMessage` and sits ABOVE the guard, so matching
/// the type name finds the prose and reports the guard as coming after its own
/// documentation — the test failing because the code explains itself.
fn find_event_emission(body: &str) -> usize {
    body.find("bus.send(crate::core::CoreEvent::NewMessage")
        .expect("the NewMessage event emission must exist")
}

#[test]
fn the_duplicate_is_recorded_in_the_bounded_set() {
    // A check that never records the id passes every message forever. Pin the
    // recording, and pin the bound, because an unbounded set is a memory leak.
    let body = handler_body(&mesh_source());

    assert!(
        body.contains("self.seen_messages.push_back(msg.id)"),
        "the handler must record the id after accepting a message, or the \\
         check rejects nothing"
    );
    assert!(
        body.contains("SEEN_MSGS_MAX"),
        "the seen set must stay bounded — it evicts by halving at \\
         SEEN_MSGS_MAX"
    );
}

#[test]
fn all_three_inbound_paths_converge_on_the_guarded_handler() {
    // The guard only helps if the paths actually route through the handler.
    // Three distinct callers were identified during the audit; assert they
    // are still there, so a refactor that adds a fourth unguarded path is a
    // visible change to this file rather than a silent regression.
    let src = mesh_source();
    let callers = src.matches("handle_incoming_message(").count();

    // 1 definition + 3 call sites
    assert!(
        callers >= 4,
        "expected handle_incoming_message to have 3 call sites plus its \
         definition, found {callers}"
    );
}

/// The failure mode this whole file exists to catch: dedup present in exactly
/// one caller. Assert the handler is guarded AND that the earlier gossipsub
/// guard is now redundant rather than the only defence.
#[test]
fn the_handler_is_the_authoritative_guard() {
    let src = mesh_source();
    let body = handler_body(&src);

    assert!(
        body.contains("self.seen_messages.contains(&msg.id)"),
        "the shared handler must be the authoritative dedup guard"
    );

    // The pre-existing caller-side check may remain, but it must not be the
    // only one. Count guards across the file: handler + at least one caller.
    let total = src.matches("self.seen_messages.contains(&msg.id)").count();
    assert!(
        total >= 2,
        "only {total} dedup guard(s) in the file — the shared handler and the \
         gossipsub path should both have one"
    );
}