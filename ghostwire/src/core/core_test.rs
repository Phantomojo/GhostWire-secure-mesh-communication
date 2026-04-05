use super::*;
use crate::config::Config;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::test]
async fn test_core_transport_switching() {
    let config = Config::default();
    let bootstrap_nodes = Vec::new();

    let core = Core::new(config, bootstrap_nodes, None, None)
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
async fn test_core_send_message_via_active_transport() {
    let config = Config::default();
    let core = Core::new(config, Vec::new(), None, None).await.unwrap();
    let core_arc = Arc::new(core);

    // Set to Dummy transport
    {
        let mut registry = core_arc.transport_registry.write().await;
        registry.register(crate::core::transport::TransportKind::Dummy);
    }
    core_arc.set_active_transport("dummy").await.unwrap();

    let message = Message::new(
        "sender".to_string(),
        "recipient".to_string(),
        "Hello World".to_string(),
    );

    // This should now succeed and reach the Dummy transport logic
    let result = core_arc.send_message(&message).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_core_clone_shares_active_transport() {
    let config = Config::default();
    let core = Core::new(config, Vec::new(), None, None).await.unwrap();

    // Initial core has None
    assert!(core.active_transport.read().await.is_none());

    let core_clone = core.clone();

    // Update transport in original
    {
        let mut active = core.active_transport.write().await;
        *active = Some(crate::core::transport::TransportKind::Dummy);
    }

    // Clone should see the change because of Arc<RwLock>
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
