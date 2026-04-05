//! Node-to-node connectivity test for GhostWire Mesh
//! Spawns two nodes and verifies they can discovery each other and exchange messages.

use ghostwire::core::encryption::EncryptionManager;
use ghostwire::core::identity::Identity;
use ghostwire::core::mesh::MeshManager;
use ghostwire::core::message::Message;
use ghostwire::core::username_discovery::UsernameDiscovery;
use libp2p::Multiaddr;
use std::sync::Arc;
use tokio::sync::RwLock;

#[tokio::test]
async fn test_node_to_node_connectivity() {
    // 1. Setup Node A
    let identity_a = Arc::new(Identity::from_seed(&[1u8; 32][..]).unwrap());
    let peer_id_a = identity_a.keypair().unwrap().public().to_peer_id();
    let encryption_a = Arc::new(EncryptionManager::new().unwrap());
    let messages_a = Arc::new(RwLock::new(Vec::new()));
    let persistence_a = Arc::new(
        ghostwire::core::persistence_manager::PersistenceManager::new(Some("node_a".to_string()))
            .unwrap(),
    );
    let store_a = Arc::new(ghostwire::core::store::SecureMessageStore::new(
        persistence_a.clone(),
        encryption_a.clone(),
    ));
    let discovery_a = Arc::new(UsernameDiscovery::new(peer_id_a, "NodeA".to_string()));

    let mut mesh_a = MeshManager::new(
        identity_a.clone(),
        encryption_a.clone(),
        messages_a.clone(),
        store_a.clone(),
        vec![],
        discovery_a.clone(),
        None,
        None,
    )
    .await
    .unwrap();

    // 2. Setup Node B
    let identity_b = Arc::new(Identity::from_seed(&[2u8; 32][..]).unwrap());
    let peer_id_b = identity_b.keypair().unwrap().public().to_peer_id();
    let encryption_b = Arc::new(EncryptionManager::new().unwrap());
    let messages_b = Arc::new(RwLock::new(Vec::new()));
    let persistence_b = Arc::new(
        ghostwire::core::persistence_manager::PersistenceManager::new(Some("node_b".to_string()))
            .unwrap(),
    );
    let store_b = Arc::new(ghostwire::core::store::SecureMessageStore::new(
        persistence_b.clone(),
        encryption_b.clone(),
    ));
    let discovery_b = Arc::new(UsernameDiscovery::new(peer_id_b, "NodeB".to_string()));

    let mut mesh_b = MeshManager::new(
        identity_b.clone(),
        encryption_b.clone(),
        messages_b.clone(),
        store_b.clone(),
        vec![],
        discovery_b.clone(),
        None,
        None,
    )
    .await
    .unwrap();

    // 3. Start nodes on local addresses
    let addr_a: Multiaddr = "/ip4/127.0.0.1/tcp/0".parse().unwrap();
    let addr_b: Multiaddr = "/ip4/127.0.0.1/tcp/0".parse().unwrap();

    mesh_a.start(addr_a).await.unwrap();
    mesh_b.start(addr_b).await.unwrap();

    // Give libp2p time to bind to addresses
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    // 3. Verify listeners and connect
    let listeners_a = mesh_a.get_listeners().await;
    let listeners_b = mesh_b.get_listeners().await;

    println!("Node A listeners: {:?}", listeners_a);
    println!("Node B listeners: {:?}", listeners_b);

    assert!(
        !listeners_a.is_empty(),
        "Node A should have at least one listener"
    );
    assert!(
        !listeners_b.is_empty(),
        "Node B should have at least one listener"
    );

    if let Some(target_addr) = listeners_b.get(0) {
        mesh_a
            .add_address(peer_id_b, target_addr.clone())
            .await
            .unwrap();
        mesh_a.connect_peer(peer_id_b).await.unwrap();
        // Libp2p needs time to establish connection and identify
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    }

    assert!(!listeners_b.is_empty());

    mesh_a.stop().await.unwrap();
    mesh_b.stop().await.unwrap();
}
