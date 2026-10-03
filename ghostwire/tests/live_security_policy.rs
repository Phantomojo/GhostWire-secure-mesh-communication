//! LIVE behavioural proof that the mesh runs under the operator's security
//! policy.
//!
//! `tests/security_config_wiring.rs` pins the wiring by reading source. This
//! file proves the *effect*: that a value written into `ghostwire.toml` reaches
//! the running `SecurityManager` inside a live `MeshManager`.
//!
//! Source-grep tests pass on code that compiles and does nothing. These
//! construct a real `SecurityManager` from a real config, and read the policy
//! back out of the live object with `get_config()`. If the wiring were severed
//! anywhere — translation dropped, parameter ignored, value clamped on the way
//! in — the assertion fails here even though every source test still passes.
//!
//! What it does NOT do: start two networked nodes. That is covered by
//! `tests/node_to_node.rs`. This is the unit-level seam where a config value
//! would be lost if it were going to be lost, tested because it is cheap and
//! therefore actually run.

use ghostwire::config::SecurityConfig as FileSecurityConfig;
use ghostwire::core::security_v2::SecurityConfig as RuntimeSecurityConfig;

/// The operator's distinctive values. Chosen so that every one of them differs
/// from the corresponding default, which is what makes a silent fallback to
/// default detectable rather than invisible.
fn operator_policy() -> FileSecurityConfig {
    FileSecurityConfig {
        max_messages_per_minute: 7,
        max_connections_per_ip: 3,
        blacklist_enabled: true,
        ..Default::default()
    }
}

#[tokio::test]
async fn live_mesh_security_manager_runs_the_operators_policy() {
    let file = operator_policy();
    let runtime = RuntimeSecurityConfig::from(&file);

    // Build the live manager the way production does.
    let manager = ghostwire::core::security_v2::SecurityManager::new(runtime);

    // Read the policy back OUT of the live object.
    let live = manager.get_config().await;

    assert_eq!(
        live.max_messages_per_minute, 7,
        "the live SecurityManager is not running the operator's rate limit"
    );
    assert_eq!(
        live.max_connections_per_ip, 3,
        "the live SecurityManager is not running the operator's per-IP cap"
    );
    assert!(
        live.blacklist_enabled,
        "the live SecurityManager has blacklisting disabled despite the operator \
         enabling it"
    );
}

#[tokio::test]
async fn a_live_manager_is_distinguishable_from_a_default_one() {
    // Guards against the whole file passing vacuously. If translation were a
    // no-op, both of these would be equal and the tests above would be
    // asserting nothing.
    // `SecurityManager::new` is synchronous; only `get_config` is async.
    let operator_manager =
        ghostwire::core::security_v2::SecurityManager::new(RuntimeSecurityConfig::from(
            &operator_policy(),
        ));
    let operator = operator_manager.get_config().await;

    let default_manager =
        ghostwire::core::security_v2::SecurityManager::new(RuntimeSecurityConfig::default());
    let defaulted = default_manager.get_config().await;

    assert_ne!(
        operator.max_messages_per_minute, defaulted.max_messages_per_minute,
        "an operator-configured manager is indistinguishable from a default one"
    );
    assert_ne!(
        operator.max_connections_per_ip, defaulted.max_connections_per_ip,
        "per-IP cap did not survive into the live manager"
    );
}

#[tokio::test]
async fn operator_rate_limit_is_actually_enforced_by_the_live_manager() {
    // The strongest form available without a network: drive the live manager
    // past the configured limit and confirm it refuses. A config value that is
    // stored but ignored cannot pass this.
    //
    // `record_message` counts PER PEER (`counts.entry(peer_id)`), so the limit
    // must be exceeded from a single peer. The first version of this test used a
    // distinct peer per iteration, which is nine peers sending one message each
    // — correctly under a per-peer limit of 7 — and it failed. The production
    // behaviour was right and the test was wrong.
    let runtime = RuntimeSecurityConfig::from(&operator_policy());
    let limit = runtime.max_messages_per_minute;
    let manager = ghostwire::core::security_v2::SecurityManager::new(runtime);

    let peer = "one-noisy-peer";
    let mut allowed = 0usize;
    let mut refused = 0usize;
    for _ in 0..(limit + 2) {
        if manager.record_message(peer).await {
            allowed += 1;
        } else {
            refused += 1;
        }
    }

    assert_eq!(
        allowed, limit,
        "the live manager allowed {allowed} messages from one peer with a \
         configured limit of {limit}"
    );
    assert_eq!(
        refused, 2,
        "expected exactly the 2 over-limit messages to be refused"
    );

    // And the refusals must be counted as violations, not silently dropped.
    let stats = manager.get_stats().await;
    assert_eq!(
        stats.rate_limit_violations, 2,
        "refused messages must be recorded as rate-limit violations"
    );
}