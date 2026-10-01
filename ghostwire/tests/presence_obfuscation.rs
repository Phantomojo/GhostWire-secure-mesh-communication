//! Does the node actually randomise its presence announcements?
//!
//! World discovery used to publish on a fixed 60s timer and immediately on
//! every `NewListenAddr`. Both are fingerprintable: an observer who timestamps
//! the announces recovers an exact constant period, and a listener coming up
//! and publishing in the same tick makes node startup instantly visible.
//!
//! These tests are behavioural where they can be, and source-reading where the
//! behaviour sits inside the driver's event loop (which needs a live Swarm and
//! a real peer, so it is covered by the live rig rather than here).

use std::time::{Duration, Instant};

use ghostwire::core::network::presence_obfuscation::{
    apply_connect_jitter, generate_dummy_activity, DummyActivity, PresenceConfig,
};

#[tokio::test]
async fn connect_jitter_actually_delays_and_is_not_fixed() {
    let cfg = PresenceConfig {
        jitter_on_connect: true,
        connect_jitter_max_secs: 1,
        ..Default::default()
    };

    let mut delays = Vec::new();
    for _ in 0..6 {
        let t0 = Instant::now();
        apply_connect_jitter(&cfg).await;
        delays.push(t0.elapsed());
    }

    // At least one must be non-trivial, else the delay is not happening at all.
    assert!(
        delays.iter().any(|d| *d > Duration::from_millis(20)),
        "connect jitter never delayed anything: {delays:?}"
    );

    // And they must differ, else it is a constant and still fingerprintable.
    let distinct = delays
        .iter()
        .map(|d| d.as_millis() / 50)
        .collect::<std::collections::HashSet<_>>();
    assert!(
        distinct.len() > 1,
        "jitter is effectively constant across samples, which leaks the delay: {delays:?}"
    );

    // And it must respect its own ceiling.
    assert!(
        delays.iter().all(|d| *d < Duration::from_secs(3)),
        "jitter exceeded its configured maximum: {delays:?}"
    );
}

#[tokio::test]
async fn jitter_can_be_disabled_explicitly() {
    // An operator must be able to turn this off; if disabling does nothing the
    // setting is decorative.
    let cfg = PresenceConfig {
        jitter_on_connect: false,
        connect_jitter_max_secs: 30,
        ..Default::default()
    };
    let t0 = Instant::now();
    apply_connect_jitter(&cfg).await;
    assert!(
        t0.elapsed() < Duration::from_millis(50),
        "jitter_on_connect=false must skip the delay entirely, took {:?}",
        t0.elapsed()
    );
}

#[tokio::test]
async fn zero_max_delay_short_circuits() {
    // Guards the multiplication-by-zero case a caller could construct by hand.
    let cfg = PresenceConfig {
        jitter_on_connect: true,
        connect_jitter_max_secs: 0,
        ..Default::default()
    };
    let t0 = Instant::now();
    apply_connect_jitter(&cfg).await;
    assert!(t0.elapsed() < Duration::from_millis(50));
}

#[test]
fn default_policy_is_protective() {
    // If the defaults were relaxed, every node would announce on a metronome.
    let d = PresenceConfig::default();
    assert!(d.jitter_on_connect, "default must jitter announces");
    assert!(
        d.randomised_fetch,
        "default must fetch on a randomised schedule"
    );
    assert!(
        d.dummy_activity,
        "default must emit dummy activity, or idleness is still observable"
    );
    assert!(
        d.fetch_interval_max_secs > d.fetch_interval_min_secs,
        "fetch window must be a range; equal bounds is a fixed schedule"
    );
    assert!(
        d.connect_jitter_max_secs > 0,
        "a zero jitter ceiling means announcing immediately, the worst case"
    );
}

#[test]
fn dummy_activity_is_varied_not_a_single_fixed_shape() {
    // A single repeated dummy event is itself a signature. Sample and require
    // more than one kind.
    let mut kinds = std::collections::HashSet::new();
    for _ in 0..40 {
        kinds.insert(match generate_dummy_activity() {
            DummyActivity::DhtLookup { .. } => 0,
            DummyActivity::FakeHave { .. } => 1,
            DummyActivity::FakeWant { .. } => 2,
        });
    }
    assert!(
        kinds.len() > 1,
        "dummy activity only ever produces one event type, which is a signature \
         rather than cover: {kinds:?}"
    );
}

fn src(p: &str) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(root.join(p)).unwrap_or_default()
}

#[test]
fn the_fixed_cadence_publish_is_jittered_at_both_leak_sites() {
    let m = src("src/core/mesh.rs");
    // Site 1: the periodic 60s world-discovery publish must randomise its gap.
    assert!(
        m.contains("apply_connect_jitter"),
        "the NewListenAddr publish must be delayed before announcing"
    );
    // Site 2: the interval itself must be reset per-tick, or the 60s period
    // survives and remains trivially measurable.
    assert!(
        m.contains("world_discovery_interval.reset_after"),
        "the periodic publish must restart its interval with jitter, otherwise \
         an exactly-60s cadence still leaks"
    );
    assert!(
        m.contains("presence: crate::core::network::presence_obfuscation::PresenceConfig"),
        "the driver must carry a presence policy"
    );
    // Guard against NEUTERING rather than deleting. An earlier version of this
    // test passed while the jitter was present but gated behind `if false`,
    // which is exactly the "looks wired, does nothing" failure this repo keeps
    // hitting.
    assert!(
        !m.contains("let jitter = if false"),
        "the periodic jitter must not be short-circuited to a constant"
    );
    assert!(
        m.contains("if self.presence.jitter_on_connect"),
        "the periodic jitter must be gated on the actual policy flag"
    );
}

#[test]
fn a_rebuilt_mesh_inherits_the_presence_policy() {
    // The supervisor rebuilds the transport. If a rebuild reverted to a default
    // that differed from the running policy, a crash would quietly degrade the
    // node's anonymity -- the exact class of bug this file is about.
    let m = src("src/core/mesh.rs");
    assert!(
        m.contains("presence: self.presence.clone()"),
        "each driver generation must inherit the transport's presence policy"
    );
}

#[test]
fn the_jitter_never_blocks_the_swarm_event_loop() {
    // This is a REGRESSION guard, and it exists because the first attempt at
    // this change broke delivery_rate. `apply_connect_jitter` was awaited
    // directly inside the `NewListenAddr` handler, which runs on the same
    // select! that must service the Swarm. Node A slept up to 30s in its event
    // handler; node B's dial was cancelled because `PeerCondition::Disconnected`
    // no longer held; both nodes then read as broken transports.
    //
    // So: the delay must be expressed as an armed DEADLINE that the select!
    // waits on, never as an awaited sleep inside a handler.
    let m = src("src/core/mesh.rs");

    assert!(
        m.contains("deferred_announce_at"),
        "a jittered announce must be an armed deadline on the driver's select!"
    );
    assert!(
        m.contains("sleep_until(at)"),
        "the deadline must be awaited by the select! itself, which is non-blocking"
    );

    // The connect-time path must arm the deadline, not publish inline.
    // The NewListenAddr handler must arm a deadline. Search within the
    // handler's own body: a bare field-name search can match the struct
    // definition or the DeferredAnnounce handler instead.
    let listener = m.find("World discovery listener available").unwrap_or(0);
    let handler = &m[listener..];
    let arm = handler
        .find("self.deferred_announce_at =")
        .unwrap_or(usize::MAX);
    assert!(
        arm < usize::MAX,
        "the NewListenAddr handler must arm a deadline rather than publishing \
         inline or awaiting a delay on the event loop"
    );
    // And it must not publish inline BEFORE arming, or the announce is still
    // instant and the jitter does nothing.
    let inline = handler
        .find("self.publish_world_discovery().await")
        .unwrap_or(usize::MAX);
    assert!(
        arm < inline,
        "the handler must arm the deadline BEFORE publishing, otherwise the \
         announce is still immediate and the jitter has no effect"
    );

    // And no `apply_connect_jitter(...).await` may sit in the event handler.
    assert!(
        !m.contains("apply_connect_jitter(&presence).await"),
        "awaiting the jitter inside the event handler blocks the Swarm loop;          arm a deadline instead"
    );

    // The periodic tick must re-arm its interval rather than sleep.
    assert!(
        !m.contains("tokio::time::sleep(std::time::Duration::from_millis(jitter)).await"),
        "the periodic publish must not sleep on the event loop"
    );
}
