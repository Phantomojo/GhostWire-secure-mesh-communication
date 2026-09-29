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
