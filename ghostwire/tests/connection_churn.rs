#![allow(dead_code)]
// Parallel connections per peer must be bounded, not left to libp2p pruning.
fn src(p: &str) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(root.join(p)).unwrap_or_default()
}

#[test]
fn swarm_caps_parallel_connections_per_peer() {
    let m = src("src/core/mesh.rs");
    // Four discovery sources (mdns, kad_provider, kad_closest,
    // world_discovery) each dial a peer's addresses, and dial_peer_if_needed
    // only suppresses when already-connected or already-dialling. The swarm
    // settled at 3-4 parallel connections to one peer and pruned the surplus
    // every 60s, which is the connection churn seen live.
    assert!(
        m.contains("connection_limits"),
        "the swarm must cap parallel connections per peer; without it the \
         discovery sources each open their own connection."
    );
    assert!(
        m.contains("with_max_established(Some(2))"),
        "cap parallel connections per peer. Must be 2, not 1: a cap of 1 makes \
         libp2p-request-response 0.29 trip its internal debug assert at \
         src/lib.rs:678 (connections.is_empty() vs remaining_established == 0) \
         and panic the node -- observed live on the Elitebook."
    );
    assert!(
        m.contains("with_max_pending_outgoing(Some("),
        "pending dials must be bounded too, or a discovery burst queues \
         unbounded dials."
    );
}

#[test]
fn connection_limit_uses_an_already_enabled_feature() {
    // The fix must not add a dependency: `memory-connection-limits` is already
    // in Cargo.toml's libp2p feature list.
    let cargo = src("Cargo.toml");
    assert!(
        cargo.contains("memory-connection-limits"),
        "expected the libp2p memory-connection-limits feature to be enabled."
    );
}
