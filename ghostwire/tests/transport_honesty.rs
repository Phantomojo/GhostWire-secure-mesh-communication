#![allow(dead_code)]
// A transport that cannot move a byte but reports itself healthy is worse than
// one that is obviously missing. These tests pin the honesty of the RNS and
// WebRTC reporting.
fn src(p: &str) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(root.join(p)).unwrap_or_default()
}

#[test]
fn rns_does_not_count_config_text_as_a_bound_interface() {
    let r = src("src/core/reticulum.rs");
    // This used to be:
    //     interfaces_ini.lines().filter(|l| l.starts_with("interface ")).count()
    // The DEFAULT config is "interface 0 = AutoInterface", so a default node
    // reported 1 interface. But the only spawn loop read `target ` lines, which
    // the default config does not have, so it actually spawned ZERO.
    assert!(
        !r.contains(r#"filter(|line| line.trim_start().starts_with("interface "))"#),
        "interfaces_configured must not be derived from config text; count real spawns"
    );
    assert!(
        r.contains("interfaces_configured += 1"),
        "the counter must be incremented at the point of a real spawn"
    );
}

#[test]
fn rns_auto_interface_actually_spawns_something() {
    let r = src("src/core/reticulum.rs");
    // "AutoInterface" is what the default config names. It was parsed and then
    // discarded as `let _rns_config`, so the one interface the default config
    // promised was never constructed anywhere in src/.
    // Anchor to real code, not the comments that explain the old bug by name:
    // a discarded binding starts the line, whereas the comments mention it
    // mid-sentence inside backticks.
    let discarded = r
        .lines()
        .any(|l| l.trim_start().starts_with("let _rns_config"));
    assert!(
        !discarded,
        "the parsed RNS config must not be discarded; AutoInterface has to be honoured"
    );
    assert!(
        r.contains("AutoInterface") && r.contains("spawn_multicast_udp_with_status"),
        "AutoInterface must map to a real spawned interface, not be parsed and dropped"
    );
}

#[test]
fn rns_warns_when_it_has_an_identity_but_no_interfaces() {
    let r = src("src/core/reticulum.rs");
    assert!(
        r.contains("configured zero interfaces"),
        "an RNS transport with zero interfaces can reach no peer and must say so, \
         because mesh_health.rs otherwise reports 'ready' from this number"
    );
}

#[test]
fn webrtc_is_not_silently_joined_while_being_undialable() {
    let m = src("src/core/mesh.rs");
    // libp2p-webrtc's dial path borrows a udp_mux from an EXISTING listener:
    //     self.listeners.iter().next().ok_or(Error::NoListeners)?
    // Nothing in src/ ever called listen_on for webrtc, so every dial failed
    // there regardless of address. The transport sat in the or_transport chain
    // looking supported.
    assert!(
        m.contains("GHOSTWIRE_WEBRTC"),
        "WebRTC must be an explicit operator opt-in, not unconditionally joined"
    );
    assert!(
        m.contains("NoListeners") || m.contains("no listener is"),
        "document that WebRTC cannot dial without a listener, so it is not \
         mistaken for a working fallback"
    );
    // And it must only be joined when actually enabled.
    assert!(
        m.contains("match webrtc_boxed"),
        "the transport chain must join WebRTC only when it is enabled"
    );
}

#[test]
fn rns_send_path_is_still_known_to_be_unreachable() {
    // This is a RECORD of a known defect, not a claim it is fixed. Core's
    // send_message returns from the mesh branch before active_transport is read,
    // so TransportKind::Reticulum can never be selected for a peer message, and
    // send_direct_message demands a libp2p PeerId, which an RNS address hash is
    // not. Keep this pinned so the gap stays visible until it is closed.
    let m = src("src/core/mod.rs");
    let mesh_return = m.find("return mgr.send_message(message).await;");
    let active_read = m.find("self.active_transport.read().await.as_ref()");
    assert!(
        mesh_return.is_some() && active_read > mesh_return,
        "send_message must return from the mesh branch before active_transport is \
         consulted -- that ordering is why RNS is currently unreachable"
    );
    assert!(
        m.contains("Mesh network not available"),
        "send_direct_message has no transport fallback at all, only this error"
    );
}
