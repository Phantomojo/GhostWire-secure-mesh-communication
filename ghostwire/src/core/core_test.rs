use super::*;
use crate::config::Config;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Generate a unique profile name for each test to avoid DB lock conflicts
fn test_profile(name: &str) -> Option<String> {
    Some(format!("test_{}_{}", name, std::process::id()))
}

#[tokio::test]
async fn test_core_transport_switching() {
    let config = Config::default();
    let bootstrap_nodes = Vec::new();
    let profile = test_profile("transport_switching");

    let core = Core::new(config, bootstrap_nodes, None, profile)
        .await
        .unwrap();
    let core_arc = Arc::new(core);

    // 1. Initial state should be None (or Simulated if sim_addr was provided, but here it's None)
    {
        let transport = core_arc.active_transport.read().await;
        assert!(transport.is_none());
    }

    // 2. Initialize default transport
    core_arc.init_default_transport().await.unwrap();
    {
        let transport = core_arc.active_transport.read().await;
        assert!(transport.is_some());
        assert_eq!(transport.as_ref().unwrap().name(), "stealth_tcp");
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
    let config = Config::default();
    let profile = test_profile("clone_shares");
    let core = Core::new(config, Vec::new(), None, profile).await.unwrap();

    // Initial core has None
    assert!(core.active_transport.read().await.is_none());

    // Register Dummy transport first
    {
        let mut registry = core.transport_registry.write().await;
        registry.register(crate::core::transport::TransportKind::Dummy);
    }
    core.set_active_transport("dummy").await.unwrap();

    // Clone should share the same Arc<RwLock> references
    let core_clone = core.clone();

    // Clone should see the transport because of Arc<RwLock> sharing
    assert!(core_clone.active_transport.read().await.is_some());
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
