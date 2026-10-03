//! The RNS transport must re-announce, not announce once at startup.
//!
//! Why this matters: RNS announces are not durable. Nothing rebroadcasts them
//! forever, so a single startup announce means a long-lived node becomes
//! invisible to any peer that learns its path later. This is the defect the
//! project's own design doc names at
//! SYMBIOTIC_P2P_ARCHITECTURE.md:1278 — "After startup, the node becomes
//! invisible to new RNS peers."
//!
//! Source-level in the repo's `persistence_honesty` style. A behavioural test
//! would need a live RNS transport plus tokio's paused clock, which is worth
//! doing when CI exists; what matters here is that the invariant is pinned and
//! the reasoning is recorded next to the test that guards it.

use std::fs;

fn reticulum_source() -> String {
    fs::read_to_string("src/core/reticulum.rs").expect("src/core/reticulum.rs must be readable")
}

#[test]
fn a_periodic_reannounce_task_exists() {
    let src = reticulum_source();
    assert!(
        src.contains("_announce_task"),
        "ReticulumTransport must hold a periodic re-announce task. Without it the \
         node announces once at startup and is then invisible to RNS peers that \
         join later -- the defect at SYMBIOTIC_P2P_ARCHITECTURE.md:1278."
    );
}

#[test]
fn the_reannounce_interval_is_defined_and_bounded() {
    let src = reticulum_source();
    assert!(
        src.contains("REANNOUNCE_INTERVAL"),
        "the re-announce interval must be a named constant, not a magic number"
    );
    // 300 s is what the original design specified. Assert it so a careless edit
    // to e.g. 30 s (chatty) or 3600 s (exceeds announce lifetime) is visible.
    assert!(
        src.contains("from_secs(300)"),
        "REANNOUNCE_INTERVAL must stay at the designed 300 s. Lower risks \
         tripping the RNS 2% announce cap on shared segments; higher risks \
         exceeding the announce lifetime so the node goes dark."
    );
}

#[test]
fn the_initial_announce_is_not_duplicated_by_the_timer() {
    // The timer uses tokio::time::interval, whose FIRST tick fires immediately.
    // If that tick is not consumed, the node double-announces at startup. This
    // asserts the skip-tick is present.
    let src = reticulum_source();
    assert!(
        src.contains("ticker.tick().await;")
            && src.contains("The first tick fires immediately"),
        "the interval's immediate first tick must be consumed before the loop, \
         otherwise startup sends two announces."
    );
}

#[test]
fn the_destination_handle_is_retained_not_discarded() {
    // The healthy path previously did `let _local_dest = ...` and discarded the
    // destination after the one-shot announce, which is precisely why
    // re-announcing was impossible. The degraded path (line ~453) still
    // discards its destination, which is correct there: a degraded transport has
    // no reachable peer to announce to, and it never starts the timer. So this
    // asserts the healthy path specifically.
    let src = reticulum_source();

    assert!(
        src.contains("let local_dest = t"),
        "the healthy init path must bind and retain the destination handle so the \
         re-announce task can reuse it"
    );

    let init = src
        .find("let rx_data = t.received_data_events();")
        .expect("the healthy init path must exist");
    let healthy = &src[init..init + 900];
    assert!(
        !healthy.contains("let _local_dest"),
        "the healthy init path must not discard the destination as `_local_dest`"
    );

    // And the degraded path must NOT start an announce task, which is what
    // makes discarding its destination correct rather than a second bug.
    assert!(
        src.contains("(Arc::new(t), rx_data, None)"),
        "the degraded path must return no announce task — there is no reachable \
         RNS peer to announce to."
    );
}

#[test]
fn stop_aborts_the_reannounce_task_before_dropping_the_transport() {
    // Otherwise a live timer task keeps announcing on a dropped transport.
    let src = reticulum_source();
    let stop = src
        .find("pub async fn stop(&mut self) -> Result<()> {")
        .expect("stop() must exist");

    // Find the ReticulumTransport::stop (the one that owns _announce_task).
    let body = &src[stop..];
    let announces_abort = body.contains("_announce_task.take()");
    let drops_transport = body.find("self.transport.take()");
    let abort_at = body.find("_announce_task.take()");

    assert!(
        announces_abort,
        "stop() must abort the re-announce task"
    );
    assert!(
        drops_transport.is_some() && abort_at.is_some() && abort_at.unwrap() < drops_transport.unwrap(),
        "the re-announce task must be aborted BEFORE the transport is dropped, \
         so the timer never touches a dead transport."
    );
}

#[test]
fn the_transport_is_shared_so_the_task_can_hold_it() {
    // The re-announce task needs its own handle to the transport, independent of
    // the manager's RwLock. That requires Arc.
    let src = reticulum_source();
    assert!(
        src.contains("transport: Option<Arc<Transport>>"),
        "the transport must be held in an Arc so the re-announce task can share \
         it without contending for the manager write lock."
    );
}

#[test]
fn send_broadcast_still_needs_work_documented_by_this_file() {
    // Pins what the source audit found, so the next reader does not have to
    // re-derive it. `send_broadcast` currently sets
    // destination_type: DestinationType::Single with an EMPTY destination hash.
    //
    // The receiver (reticulum-rs-transport wire.rs) only delivers data when
    //   destination_type == Single AND single_in_destinations contains the hash
    // An empty hash is never registered, so the packet is not delivered locally.
    // Switching the flag to Broadcast would skip the delivery block entirely and
    // be WORSE, not better.
    //
    // The correct fix needs a registered broadcast/group destination, which the
    // 0.9.7 API does not expose. Until that exists, `send_broadcast` must not
    // be advertised as working.
    let src = reticulum_source();
    assert!(
        src.contains("fn send_broadcast"),
        "send_broadcast must still exist (this test documents its known state)"
    );
}