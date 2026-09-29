#![allow(dead_code)]
// The swarm driver can die and take the whole mesh with it. These tests pin the
// supervision and the liveness reporting that make that failure visible instead
// of leaving a node that looks healthy but cannot reach a peer.
fn src(p: &str) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(root.join(p)).unwrap_or_default()
}

#[test]
fn mesh_driver_is_supervised_so_a_panic_is_not_silent() {
    let m = src("src/core/mesh.rs");
    // The driver owns the Swarm and is consumed by run(), so a panic leaves no
    // mesh and nothing can rebuild it in place. Observed live: the node kept
    // serving the API and logging autonomic lines while being completely
    // unable to reach a peer.
    assert!(
        m.contains("catch_unwind"),
        "the swarm driver must be wrapped in catch_unwind so its death is attributable"
    );
    assert!(
        m.contains("Mesh driver PANICKED"),
        "a driver panic must be logged loudly rather than swallowed"
    );
    assert!(
        m.contains("has no mesh connectivity") || m.contains("NO mesh connectivity"),
        "log that the node is still serving the API but has no mesh"
    );
}

#[test]
fn mesh_liveness_is_tracked_and_marked_dead_on_every_exit_path() {
    let m = src("src/core/mesh.rs");
    assert!(
        m.contains("pub fn is_mesh_alive"),
        "callers need a way to ask whether the mesh is still running"
    );
    assert!(
        m.contains("driver_alive.store(false"),
        "the liveness flag must be cleared when the driver stops, on both the \
         clean-exit and panic paths"
    );
    // The flag has to be cleared AFTER the match so it covers both arms.
    let store = m.find("driver_alive.store(false").unwrap_or(0);
    let match_pos = m
        .find("match futures_util::FutureExt::catch_unwind")
        .unwrap_or(0);
    assert!(
        store > match_pos,
        "the flag must be cleared after the catch_unwind match, not before it, \
         or the panic path would be missed"
    );
}

#[test]
fn connection_bookkeeping_is_the_documented_cause_of_the_driver_panic() {
    let m = src("src/core/mesh.rs");
    // Keep the reason for the supervision attached to the code. Upstream
    // libp2p-request-response 0.29.0 has, in on_connection_closed:
    //     debug_assert_eq!(connections.is_empty(), remaining_established == 0);
    // and two non-debug `.expect()` calls beside it. A connection timeout makes
    // that bookkeeping disagree, which is how the driver was killed. The
    // supervision above is the mitigation; this is the why.
    assert!(
        m.contains("libp2p-request-response"),
        "document which upstream behaviour panics on connection teardown, so the \
         supervision is not mistaken for unrelated defensive code"
    );
}

// ─────────────────────────────────────────────────────────────────────────────
// Behavioural tests for the RECOVERY path.
//
// The three tests above pin the wiring by reading the source, which is how the
// previous commit (2508adf) could only prove the failure was *reported*. The
// supervisor's whole point is that the mesh comes back, so these drive
// `supervise_mesh` directly with fake drivers and a fake rebuild closure: no
// Swarm, no network, no timing luck. If recovery breaks, these go red.
// ─────────────────────────────────────────────────────────────────────────────

use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use ghostwire::core::mesh::{supervise_mesh, MESH_REBUILD_ATTEMPTS};

type RebuildResult =
    std::pin::Pin<Box<dyn std::future::Future<Output = anyhow::Result<()>> + Send>>;

/// A driver that panics at once, like the real `libp2p-request-response`
/// connection-bookkeeping assert did after a connection timeout.
fn panicking_driver() -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
    Box::pin(async { panic!("simulated mesh driver panic") })
}

/// The "exited unexpectedly" path, as opposed to a panic.
fn exiting_driver() -> std::pin::Pin<Box<dyn std::future::Future<Output = ()> + Send>> {
    Box::pin(async {})
}

#[tokio::test]
async fn a_panicking_driver_is_rebuilt_and_liveness_is_restored() {
    let alive = Arc::new(AtomicBool::new(true));
    let stopping = Arc::new(AtomicBool::new(false));
    let rebuilds = Arc::new(AtomicUsize::new(0));
    let counter = rebuilds.clone();

    supervise_mesh(
        panicking_driver(),
        move || {
            let counter = counter.clone();
            Box::pin(async move {
                counter.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }) as RebuildResult
        },
        alive.clone(),
        stopping,
        3,
        Duration::from_millis(1),
    )
    .await;

    assert_eq!(
        rebuilds.load(Ordering::SeqCst),
        1,
        "a dead driver must trigger exactly one rebuild"
    );
    assert!(
        alive.load(Ordering::SeqCst),
        "liveness must be restored once a replacement driver is running; \
         otherwise the node keeps reporting degraded after it has recovered"
    );
}

#[tokio::test]
async fn a_clean_driver_exit_also_triggers_a_rebuild() {
    let alive = Arc::new(AtomicBool::new(true));
    let stopping = Arc::new(AtomicBool::new(false));
    let rebuilds = Arc::new(AtomicUsize::new(0));
    let counter = rebuilds.clone();

    supervise_mesh(
        exiting_driver(),
        move || {
            let counter = counter.clone();
            Box::pin(async move {
                counter.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }) as RebuildResult
        },
        alive.clone(),
        stopping,
        3,
        Duration::from_millis(1),
    )
    .await;

    assert_eq!(rebuilds.load(Ordering::SeqCst), 1);
    assert!(alive.load(Ordering::SeqCst));
}

#[tokio::test]
async fn liveness_is_cleared_while_a_rebuild_is_in_flight() {
    // The whole failure 2508adf was written to stop is a node that reports
    // "healthy" while it has no driver. So the gap must be OBSERVABLE, not
    // merely implied by the ordering in the source.
    let alive = Arc::new(AtomicBool::new(true));
    let stopping = Arc::new(AtomicBool::new(false));
    let seen_during_rebuild = Arc::new(AtomicBool::new(true));

    let probe_flag = alive.clone();
    let probe = seen_during_rebuild.clone();
    supervise_mesh(
        exiting_driver(),
        move || {
            let probe = probe.clone();
            let probe_flag = probe_flag.clone();
            Box::pin(async move {
                // Sampled from inside the rebuild, before it reports success.
                probe.store(probe_flag.load(Ordering::SeqCst), Ordering::SeqCst);
                Ok(())
            }) as RebuildResult
        },
        alive.clone(),
        stopping,
        3,
        Duration::from_millis(1),
    )
    .await;

    assert!(
        !seen_during_rebuild.load(Ordering::SeqCst),
        "liveness must read false while a rebuild is pending, so the API reports \
         degraded instead of a mesh that is not there"
    );
    assert!(
        alive.load(Ordering::SeqCst),
        "and must read true again once the replacement is running"
    );
}

#[tokio::test]
async fn shutdown_does_not_resurrect_the_mesh() {
    // `stop()` sets `stopping` BEFORE sending Shutdown precisely so the
    // supervisor does not race to rebuild the mesh it was told to tear down.
    let alive = Arc::new(AtomicBool::new(true));
    let stopping = Arc::new(AtomicBool::new(true));
    let rebuilds = Arc::new(AtomicUsize::new(0));
    let counter = rebuilds.clone();

    supervise_mesh(
        exiting_driver(),
        move || {
            let counter = counter.clone();
            Box::pin(async move {
                counter.fetch_add(1, Ordering::SeqCst);
                Ok(())
            }) as RebuildResult
        },
        alive.clone(),
        stopping,
        3,
        Duration::from_millis(1),
    )
    .await;

    assert_eq!(
        rebuilds.load(Ordering::SeqCst),
        0,
        "a deliberate shutdown must NOT trigger a rebuild"
    );
    assert!(
        !alive.load(Ordering::SeqCst),
        "a stopped mesh must not report itself alive"
    );
}

#[tokio::test]
async fn a_never_healing_rebuild_gives_up_instead_of_spinning() {
    let alive = Arc::new(AtomicBool::new(true));
    let stopping = Arc::new(AtomicBool::new(false));
    let attempts = Arc::new(AtomicUsize::new(0));
    let counter = attempts.clone();

    let started = std::time::Instant::now();
    supervise_mesh(
        exiting_driver(),
        move || {
            let counter = counter.clone();
            Box::pin(async move {
                counter.fetch_add(1, Ordering::SeqCst);
                Err(anyhow::anyhow!("simulated rebuild failure"))
            }) as RebuildResult
        },
        alive.clone(),
        stopping,
        3,
        Duration::from_millis(1),
    )
    .await;

    assert_eq!(
        attempts.load(Ordering::SeqCst),
        3,
        "retries must stop at the budget rather than looping forever"
    );
    assert!(
        !alive.load(Ordering::SeqCst),
        "after the budget is exhausted the node must not claim a working mesh"
    );
    assert!(
        started.elapsed() < Duration::from_secs(20),
        "backoff must be finite; a spin loop would hang this test"
    );
}

#[test]
fn the_production_retry_budget_is_bounded() {
    // The shipped constant, not a test-supplied one. An unbounded budget would
    // make the give-up path unreachable in production while every behavioural
    // test above still passed, because they pass their own small budget.
    assert!(
        (1..=20).contains(&MESH_REBUILD_ATTEMPTS),
        "MESH_REBUILD_ATTEMPTS must be small and finite, got {}",
        MESH_REBUILD_ATTEMPTS
    );
}
