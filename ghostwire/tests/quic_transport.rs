#![allow(dead_code)]
// QUIC must be a real mesh transport, and must not be able to kill the node.
fn src(p: &str) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(root.join(p)).unwrap_or_default()
}

#[test]
fn quic_is_in_the_libp2p_swarm_not_only_the_registry() {
    let m = src("src/core/mesh.rs");
    // QUIC used to exist ONLY in the GhostWire TransportRegistry, where the
    // mesh branch in Core::send_direct_message returns before active_transport
    // is ever read. So a QUIC transport could never carry a peer message, even
    // though the node bound a UDP port and the transport manager logged
    // "Switching to Quic".
    assert!(
        m.contains("libp2p::quic::tokio::Transport::new"),
        "QUIC must be built into the libp2p swarm transport stack, not just the registry, \
         or it can never carry peer traffic."
    );
    assert!(
        m.contains("or_transport(quic_boxed)"),
        "the QUIC transport must be joined into the swarm's transport chain."
    );
    assert!(
        m.contains("quic-v1"),
        "the QUIC listener must be a real /quic-v1 multiaddr so peers can dial it."
    );
}

#[test]
fn quic_bind_failure_degrades_instead_of_killing_the_node() {
    let m = src("src/core/mesh.rs");
    // A fatal QUIC bind took down Core::new and cascaded into 17 unrelated
    // tests ("env lock poisoned") whenever udp/8081 was already held -- two
    // nodes on one host, or a parallel test run. QUIC is an ADDITIONAL path;
    // TCP/relay/WebSocket/WebRTC all still work without it, so a bind failure
    // must not take the node offline.
    assert!(
        !m.contains("QUIC failed to bind udp/"),
        "a QUIC bind failure must NOT be propagated as an error; it should log a \
         warning and continue on the other transports."
    );
    assert!(
        m.contains("degraded, not down"),
        "keep the note explaining that QUIC is additive, so a bind failure degrades \
         the node instead of failing it."
    );
}

#[test]
fn quic_port_is_threaded_from_config() {
    let mesh = src("src/core/mesh.rs");
    let core = src("src/core/mod.rs");
    // The mesh never received the configured port before, which is part of why
    // QUIC could not be a real transport.
    assert!(
        mesh.contains("quic_port: u16"),
        "MeshManager::new must accept the QUIC port."
    );
    assert!(
        core.contains("server.quic_port"),
        "the configured port must be passed into MeshManager::new."
    );
}

#[test]
fn quic_is_disabled_by_a_zero_port() {
    let m = src("src/core/mesh.rs");
    // Tests pass quic_port = 0 to opt out entirely rather than fighting over a
    // fixed UDP port.
    assert!(
        m.contains("if quic_port > 0 {"),
        "a zero quic_port must disable the QUIC listener so parallel test runs do not \
         contend for the same UDP port."
    );
}
