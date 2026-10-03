//! Delivery-rate harness for GhostWire mesh.
//! Spins up 2-3 isolated MeshManager instances, sends N messages between them,
//! counts successful deliveries (via receive-side confirmation), and reports delivery %.
//!
//! Uses isolated test ports (9397/4301/8381, 9497/4401/8481, 9597/4501/8581).
//! Never touches production ports: API 9097, mesh 4001, QUIC 8081.
//!
//! Usage: cargo test delivery_rate -- --nocapture
//!
//! The harness exercises: ACK/NACK frames, HLC ordering, Bloom dedup, TTL, PeerHealthTracker, ratchet.

use ghostwire::core::discovery_directory::DiscoveryDirectory;
use ghostwire::core::encryption::Encryption;
use ghostwire::core::encryption::EncryptionManager;
use ghostwire::core::identity::Identity;
use ghostwire::core::mesh::MeshManager;
use ghostwire::core::message::Message;
use ghostwire::core::persistence_manager::PersistenceManager;
use ghostwire::core::reliability::ReliabilityLayer;
use ghostwire::core::security::{SecurityConfig, SecurityManager};
use ghostwire::core::store::SecureMessageStore;
use ghostwire::core::username_discovery::UsernameDiscovery;
use libp2p::PeerId;
use std::sync::{Arc, OnceLock};
use tokio::sync::RwLock;
use tokio::time::{sleep, Duration};

static TEST_HOME_LOCK: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();

async fn test_home_guard() -> tokio::sync::MutexGuard<'static, ()> {
    TEST_HOME_LOCK
        .get_or_init(|| tokio::sync::Mutex::new(()))
        .lock()
        .await
}

/// Build a minimal MeshManager for testing on a given port.
/// Returns (MeshManager, PeerId, listen_multiaddr).
async fn build_test_mesh_manager(
    port: u16,
    profile: Option<String>,
) -> anyhow::Result<(MeshManager, PeerId, libp2p::Multiaddr)> {
    // Use a unique temp directory for each node to avoid DB lock conflicts.
    //
    // The counter matters as much as the port: sled takes an exclusive lock on
    // `vault/ghost_default/db`, and a previous MeshManager on the same port may
    // still hold it when the next cycle rebuilds the node (the test runs
    // A->B then B->A on the same two ports). Keying the directory on the port
    // alone made the second cycle fail with
    // "could not acquire lock ... WouldBlock".
    static HOME_SEQ: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let seq = HOME_SEQ.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
    let temp_dir = std::env::temp_dir().join(format!(
        "ghostwire_test_{}_{}_{}",
        port,
        std::process::id(),
        seq
    ));
    let _ = std::fs::create_dir_all(&temp_dir);
    std::env::set_var("GHOSTWIRE_HOME", temp_dir.to_str().unwrap());

    let identity = Arc::new(
        Identity::new(profile).map_err(|e| anyhow::anyhow!("Identity creation failed: {}", e))?,
    );
    let keypair = identity
        .keypair()
        .ok_or_else(|| anyhow::anyhow!("Identity is locked"))?;
    let local_peer_id = PeerId::from(keypair.public());

    let encryption_manager = Arc::new(
        EncryptionManager::new()
            .map_err(|e| anyhow::anyhow!("EncryptionManager creation failed: {}", e))?,
    );

    let security_config = SecurityConfig::default();
    let _security_manager = Arc::new(SecurityManager::new(security_config));

    let persistence = Arc::new(PersistenceManager::new(None)?);
    let encryption = Arc::new(
        Encryption::new().map_err(|e| anyhow::anyhow!("Encryption creation failed: {}", e))?,
    );
    let message_store = Arc::new(SecureMessageStore::new(persistence, encryption));

    let messages: Arc<RwLock<Vec<Message>>> = Arc::new(RwLock::new(Vec::new()));

    let username_discovery = Arc::new(UsernameDiscovery::new(
        local_peer_id,
        identity.get_metadata().nickname,
    ));

    let mut mesh_manager = MeshManager::new(
        identity.clone(),
        encryption_manager,
        messages.clone(),
        message_store,
        vec![], // no bootstrap nodes -- we'll dial directly
        username_discovery,
        None, // no event bus
        None, // no handshake secret
        Arc::new(ReliabilityLayer::with_defaults()),
        Arc::new(DiscoveryDirectory::new()),
        0, // quic_port: 0 disables the QUIC listener for these tests
        // Tests use the default security policy. This is now an explicit
        // argument rather than a hardcoded default inside MeshManager::new,
        // so every call site states the policy it runs under.
        ghostwire::core::security_v2::SecurityConfig::default(),
    )
    .await?;

    let listen_addr: libp2p::Multiaddr = format!("/ip4/127.0.0.1/tcp/{}", port).parse()?;
    mesh_manager.start(listen_addr.clone()).await?;

    // Give the swarm a moment to bind
    sleep(Duration::from_millis(200)).await;

    Ok((mesh_manager, local_peer_id, listen_addr))
}

/// Run a delivery count from sender_port to receiver_port.
/// This function builds fresh nodes, connects them, sends `count` messages,
/// and returns (delivered, total_sent).
async fn run_delivery_cycle(sender_port: u16, receiver_port: u16, count: usize) -> (usize, usize) {
    // Build sender and receiver fresh
    let (mut sender, sender_psid, sender_addr) =
        build_test_mesh_manager(sender_port, Some("delivery_sender".to_string()))
            .await
            .expect("Failed to build sender");
    let (mut receiver, receiver_psid, receiver_addr) =
        build_test_mesh_manager(receiver_port, Some("delivery_receiver".to_string()))
            .await
            .expect("Failed to build receiver");

    // Add cross-addresses so they can find each other.
    // NOTE: these must be the real peer IDs returned by build_test_mesh_manager.
    // A placeholder PublicKey::new([0u8; 32]) would address a peer that does not exist.
    sender
        .add_address(receiver_psid.clone(), receiver_addr.clone())
        .await
        .expect("Failed to add address sender->receiver");
    receiver
        .add_address(sender_psid.clone(), sender_addr.clone())
        .await
        .expect("Failed to add address receiver->sender");

    let dm_topic = ghostwire::core::constants::topics::DM.to_string();
    receiver
        .subscribe_topic(&dm_topic)
        .await
        .expect("Failed to subscribe to DM topic");
    sender
        .subscribe_topic(&dm_topic)
        .await
        .expect("Failed to subscribe to DM topic");

    sender
        .connect_peer(receiver_psid.clone())
        .await
        .expect("Failed to connect to peer");

    // Wait for handshake
    let handshake_ready = tokio::time::timeout(Duration::from_secs(8), async {
        loop {
            let a_ready = sender
                .get_topology()
                .await
                .nodes
                .get(&receiver_psid.to_string())
                .map(|node| !node.public_key.is_empty())
                .unwrap_or(false);
            let b_ready = receiver
                .get_topology()
                .await
                .nodes
                .get(&sender_psid.to_string())
                .map(|node| !node.public_key.is_empty())
                .unwrap_or(false);
            if a_ready && b_ready {
                break true;
            }
            sleep(Duration::from_millis(100)).await;
        }
    })
    .await
    .unwrap_or(false);
    assert!(handshake_ready, "authenticated handshake did not complete");

    let mut delivered = 0;
    for i in 0..count {
        let msg_content = format!("delivery_harness_msg_{}", i);
        let message = Message::new(
            sender_psid.to_string(),
            receiver_psid.to_string(),
            msg_content.clone(),
        );

        sender
            .send_direct_message(receiver_psid.clone(), message.clone())
            .await
            .expect("Failed to send direct message");

        // Check if receiver got it
        sleep(Duration::from_millis(100)).await;
        let b_messages = receiver.get_received_messages().await;
        if b_messages.iter().any(|m| m.content == msg_content) {
            delivered += 1;
            if i < 3 || i >= count - 3 {
                println!("  msg {}: delivered", i);
            }
        } else if i < 3 {
            println!("  msg {}: NOT delivered", i);
        }
    }

    (delivered, count)
}

#[tokio::test]
async fn test_delivery_rate_two_nodes() {
    let _home_guard = test_home_guard().await;
    std::env::set_var("GHOSTWIRE_TEST_DISABLE_MDNS", "1");

    let count = 100;

    // Use isolated test ports: 9397, 9497
    let (delivered_a_b, total_a_b) = run_delivery_cycle(9397, 9497, count).await;
    let (delivered_b_a, total_b_a) = run_delivery_cycle(9497, 9397, count).await;

    let total_delivered = delivered_a_b + delivered_b_a;
    let total_sent = total_a_b + total_b_a;
    let pct = if total_sent > 0 {
        (total_delivered as f64 / total_sent as f64) * 100.0
    } else {
        0.0
    };

    println!("\n=== Delivery Rate Summary ===");
    println!(
        "Messages sent A->B: {} / delivered: {}",
        total_a_b, delivered_a_b
    );
    println!(
        "Messages sent B->A: {} / delivered: {}",
        total_b_a, delivered_b_a
    );
    println!(
        "Total sent: {} / total delivered: {}",
        total_sent, total_delivered
    );
    println!(
        "Delivery rate: {}/{} = {:.2}%",
        total_delivered, total_sent, pct
    );

    // Verify at least some messages get through
    assert!(
        total_delivered > 0,
        "No messages delivered at all - harness may be broken"
    );

    // This assert ensures we report a real number on the funding call
    eprintln!("DELIVERY_RATE: {:.2}%", pct);
}

#[tokio::test]
async fn test_delivery_rate_three_nodes() {
    let _home_guard = test_home_guard().await;
    std::env::set_var("GHOSTWIRE_TEST_DISABLE_MDNS", "1");

    let count = 50;

    // Use isolated test ports: 9397, 9497, 9597
    let (delivered_a_b, total_a_b) = run_delivery_cycle(9397, 9497, count).await;
    let (delivered_b_c, total_b_c) = run_delivery_cycle(9497, 9597, count).await;

    let total_delivered = delivered_a_b + delivered_b_c;
    let total_sent = total_a_b + total_b_c;
    let pct = if total_sent > 0 {
        (total_delivered as f64 / total_sent as f64) * 100.0
    } else {
        0.0
    };

    println!("\n=== 3-Node Delivery Rate Summary ===");
    println!(
        "Messages sent A->B: {} / delivered: {}",
        total_a_b, delivered_a_b
    );
    println!(
        "Messages sent B->C: {} / delivered: {}",
        total_b_c, delivered_b_c
    );
    println!(
        "Total sent: {} / total delivered: {}",
        total_sent, total_delivered
    );
    println!(
        "Delivery rate: {}/{} = {:.2}%",
        total_delivered, total_sent, pct
    );

    assert!(total_delivered > 0, "No messages delivered in 3-node test");

    // This assert ensures we report a real number on the funding call
    eprintln!("DELIVERY_RATE_3NODE: {:.2}%", pct);
}
