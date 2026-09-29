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
fn rns_recipients_are_routed_to_rns_instead_of_failing_a_peerid_parse() {
    let m = src("src/core/mod.rs");
    // This USED to assert the defect: send_direct_message only tried
    // `recipient.parse::<PeerId>()` and returned "Invalid P2P Recipient ID"
    // for everything else, so a valid Reticulum address could not even be
    // expressed. Now the recipient shape decides the transport.
    assert!(
        m.contains("is_reticulum_address(&recipient)"),
        "send_direct_message must route a Reticulum address to RNS"
    );
    assert!(
        m.contains("is a Reticulum address but the RNS transport is not running"),
        "a Reticulum recipient with no RNS transport must say so, not fall \
         through to a PeerId parse error"
    );
}

#[test]
fn rns_receives_through_a_pump_instead_of_filling_an_undrained_channel() {
    let m = src("src/core/mod.rs");
    // ReticulumManager::receive_message had ZERO callers: the rx task filled a
    // 256-slot mpsc that nobody read, so RNS could buffer but never receive.
    assert!(
        m.contains("rns_rx_pump"),
        "a receive pump must exist and be cancellable, or RNS cannot receive"
    );
    assert!(
        m.contains("receive_message().await"),
        "the pump must actually drain the RNS receive channel"
    );
    // And it must not downgrade a private message to plaintext.
    assert!(
        m.contains("store.save_message(&msg)"),
        "inbound RNS messages must be persisted encrypted via save_message, \
         which fails closed on a locked profile"
    );
    assert!(
        !m.contains("save_message_plain(&msg)"),
        "never fall back to plaintext storage for an inbound RNS message"
    );
    assert!(
        m.contains("handle.abort()"),
        "stopping RNS must stop the pump rather than leak a task per cycle"
    );
}

#[test]
fn rns_routing_check_is_strict_about_length() {
    let r = src("src/core/reticulum.rs");
    // parse_address_hash accepts ANY hex of ANY length, so using it to pick a
    // transport would truncate a PeerId into a bogus 16-byte destination and
    // encrypt a private message to a key nobody holds.
    assert!(
        r.contains("ADDRESS_HASH_SIZE * 2"),
        "the RNS address check must require the exact Reticulum address length"
    );
    assert!(
        r.contains("is_ascii_hexdigit"),
        "the check must also reject non-hex input of the right length"
    );
}
