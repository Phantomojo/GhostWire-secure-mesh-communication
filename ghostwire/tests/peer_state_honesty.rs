#![allow(dead_code)]
// A peer must not be reported offline while a connection is demonstrably up.
fn src(p: &str) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(root.join(p)).unwrap_or_default()
}

#[test]
fn connection_close_only_marks_offline_when_last_connection_goes() {
    let m = src("src/core/mesh.rs");
    // Regression: the handler destructured ConnectionClosed with `..`, ignoring
    // num_established, and set is_online = false on EVERY close. libp2p keeps
    // several connections per peer, so one closing declared the peer offline
    // while others were healthy -- /api/v2/peers reported Disconnected during 8
    // consecutive successful pings (30-230ms).
    assert!(
        m.contains("num_established"),
        "ConnectionClosed must read num_established; ignoring it marks a \
         still-connected peer offline."
    );
    assert!(
        m.contains("if *num_established == 0"),
        "only the last connection close may set is_online = false."
    );
}

#[test]
fn peers_list_reconciles_social_graph_with_live_topology() {
    let pc = src("src/controllers/peer_controller.rs");
    assert!(
        pc.contains("nodes.get(&p.node_id)"),
        "social-graph peers must be reconciled against the live topology."
    );
    assert!(
        pc.contains("unwrap_or(p.is_online)"),
        "absent from topology -> fall back to the social-graph value rather \
         than asserting offline on no evidence."
    );
}
