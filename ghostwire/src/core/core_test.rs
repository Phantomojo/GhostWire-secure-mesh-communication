use super::*;
use crate::config::Config;
use std::sync::Arc;

/// Generate a unique profile name for each test to avoid DB lock conflicts
fn test_profile(name: &str) -> Option<String> {
    Some(format!("test_{}_{}", name, std::process::id()))
}

#[tokio::test]
async fn test_core_transport_switching() {
    let _home = crate::core::encryption::test_home();
    let config = Config::default();
    let bootstrap_nodes = Vec::new();
    let profile = test_profile("transport_switching");

    let core = Core::new(config, bootstrap_nodes, None, profile)
        .await
        .unwrap();
    let core_arc = Arc::new(core);

    {
        // NO transport is active at startup, and that is now the asserted contract
        // rather than an accident.
        //
        // This previously asserted `is_some()` because Core::new installed
        // StealthTCP as the default. StealthTCP was retired: it sent message
        // content as bare bincode with no encryption, and because the mesh arm
        // encrypts only into a CLONE, every mesh failure put the plaintext body on
        // the wire through this fallback.
        //
        // The correct behaviour for a node with no mesh and no Reticulum is to fail
        // loudly — `send_message` returns "NOT TRANSMITTED: no transport could carry
        // this message" — rather than to fall back to a transport that leaks.
        //
        // Asserting the absence here is deliberate: it makes reintroducing a
        // plaintext fallback a test failure rather than a security regression found
        // later.
        let transport = core_arc.active_transport.read().await;
        assert!(
            transport.is_none(),
            "no fallback transport must be installed at startup: the previous \
             default (StealthTCP) sends message content unencrypted"
        );
    }

    // 2. init_default_transport must NOT resurrect a leaking transport.
    //    It is still callable for operators who explicitly select one, so this
    //    asserts what it leaves in place rather than that it errors.
    core_arc.init_default_transport().await.ok();
    {
        let transport = core_arc.active_transport.read().await;
        if let Some(t) = transport.as_ref() {
            assert_ne!(
                t.name(),
                "stealth_tcp",
                "StealthTCP must never be auto-installed as a message transport"
            );
        }
    }

    // 3. Register and switch to Dummy transport
    {
        let mut registry = core_arc.transport_registry.write().await;
        registry.register(crate::core::transport::TransportKind::Dummy);
    }

    core_arc.set_active_transport("dummy").await.unwrap();
    {
        let transport = core_arc.active_transport.read().await;
        assert!(transport.is_some());
        assert_eq!(transport.as_ref().unwrap().name(), "dummy");
    }
}

#[tokio::test]
#[ignore = "pre-existing: Core::new blocks on mesh/swarm init — needs async init refactor"]
async fn test_core_send_message_via_active_transport() {
    let config = Config::default();
    let profile = test_profile("send_message");
    let core = Core::new(config, Vec::new(), None, profile).await.unwrap();
    let core_arc = Arc::new(core);

    let mut registry = core_arc.transport_registry.write().await;
    registry.register(crate::core::transport::TransportKind::Dummy);
    drop(registry);
    core_arc.set_active_transport("dummy").await.unwrap();

    let message = Message::new(
        "sender".to_string(),
        "recipient".to_string(),
        "Hello World".to_string(),
    );

    let result = core_arc.send_message(&message).await;
    assert!(result.is_ok());

    assert_eq!(core_arc.messages.read().await.len(), 1);
}

#[tokio::test]
async fn test_core_clone_shares_active_transport() {
    let _home = crate::core::encryption::test_home();
    let config = Config::default();
    let profile = test_profile("clone_shares");
    let core = Core::new(config, Vec::new(), None, profile).await.unwrap();

    // NO transport is active at startup now, and that is the contract.
    //
    // History of this assertion: it once asserted Some("quic"), which only passed
    // because the structurally dead QuicTransportProvider won an `is_none()` race
    // in Core::new. Then it asserted `is_some()` because Core::new installed
    // StealthTCP. Both defaults are gone: QuicTransportProvider stole the swarm's
    // UDP port and could never address a peer, and StealthTCP sends message
    // content unencrypted.
    //
    // This test previously poisoned a global ENV_LOCK when it failed, cascading
    // into 20 unrelated encryption failures. Asserting `is_none()` keeps the test
    // passing for the right reason, so it cannot poison anything again.
    let initial_name = core
        .active_transport
        .read()
        .await
        .as_ref()
        .map(|t| t.name().to_string());
    assert!(
        initial_name.is_none(),
        "no fallback transport may be installed at startup: every previous \
         default transmitted message content in the clear"
    );

    {
        let mut registry = core.transport_registry.write().await;
        registry.register(crate::core::transport::TransportKind::Dummy);
    }
    core.set_active_transport("dummy").await.unwrap();

    let core_clone = core.clone();

    assert_eq!(
        core_clone
            .active_transport
            .read()
            .await
            .as_ref()
            .unwrap()
            .name(),
        "dummy"
    );
}
