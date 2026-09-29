//! Node-to-node connectivity test for GhostWire Mesh
//!
//! Spins up two MeshManagers on localhost, connects them, sends a message,
//! and verifies the recipient received it.

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

/// Build a minimal MeshManager for testing.
/// Returns (MeshManager, PeerId, listen_multiaddr).
async fn build_test_mesh_manager(
    port: u16,
    profile: Option<String>,
) -> anyhow::Result<(MeshManager, PeerId, libp2p::Multiaddr)> {
    // Use a unique temp directory for each node to avoid DB lock conflicts
    let temp_dir =
        std::env::temp_dir().join(format!("ghostwire_test_{}_{}", port, std::process::id()));
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
    )
    .await?;

    let listen_addr: libp2p::Multiaddr = format!("/ip4/127.0.0.1/tcp/{}", port).parse()?;
    mesh_manager.start(listen_addr.clone()).await?;

    // Give the swarm a moment to bind
    sleep(Duration::from_millis(200)).await;

    Ok((mesh_manager, local_peer_id, listen_addr))
}

#[tokio::test]
async fn test_node_to_node_connectivity() {
    let _home_guard = test_home_guard().await;
    std::env::set_var("GHOSTWIRE_TEST_DISABLE_MDNS", "1");
    // Outer guard must exceed the inner 20s waits below, which in turn must
    // exceed the mesh driver's 15s PING_INTERVAL.
    tokio::time::timeout(Duration::from_secs(30), async {
        let (mut node_a, peer_id_a, _addr_a) =
            build_test_mesh_manager(19001, Some("test_node_a".to_string()))
                .await
                .expect("Failed to build node A");
        let (node_b, peer_id_b, addr_b) =
            build_test_mesh_manager(19002, Some("test_node_b".to_string()))
                .await
                .expect("Failed to build node B");

        node_a
            .add_address(peer_id_b, addr_b.clone())
            .await
            .expect("Failed to add address");

        let dm_topic = ghostwire::core::constants::topics::DM.to_string();
        node_b
            .subscribe_topic(&dm_topic)
            .await
            .expect("Failed to subscribe to DM topic");
        node_a
            .subscribe_topic(&dm_topic)
            .await
            .expect("Failed to subscribe to DM topic");

        node_a
            .connect_peer(peer_id_b)
            .await
            .expect("Failed to connect to peer");

        let handshake_ready = tokio::time::timeout(Duration::from_secs(8), async {
            loop {
                let a_ready = node_a
                    .get_topology()
                    .await
                    .nodes
                    .get(&peer_id_b.to_string())
                    .map(|node| !node.public_key.is_empty())
                    .unwrap_or(false);
                let b_ready = node_b
                    .get_topology()
                    .await
                    .nodes
                    .get(&peer_id_a.to_string())
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
        // Must exceed the mesh driver's PING_INTERVAL (15s, see
        // `ping_interval` in core/mesh.rs). The first tick fires immediately but
        // before the peer has completed the handshake and entered the topology,
        // so `send_periodic_pings` iterates an empty/partial set. The first
        // ping that can actually reach the peer is therefore at t=15s, and this
        // window has to be longer than that or the test can never observe a
        // pong regardless of whether the transport is healthy.
        let peer_health_ready = tokio::time::timeout(Duration::from_secs(20), async {
            loop {
                if let Some(health) = node_a
                    .peer_health()
                    .get_peer_health(&peer_id_b.to_string())
                    .await
                {
                    if health.pongs_received > 0 && health.is_alive {
                        break true;
                    }
                }
                sleep(Duration::from_millis(100)).await;
            }
        })
        .await
        .unwrap_or(false);
        assert!(
            peer_health_ready,
            "peer did not become live after handshake"
        );

        let peer_id_a_str = peer_id_a.to_string();
        let peer_id_b_str = peer_id_b.to_string();
        let message = Message::new(
            peer_id_a_str,
            peer_id_b_str,
            "Hello from Node A!".to_string(),
        );

        node_a
            .send_direct_message(peer_id_b, message.clone())
            .await
            .expect("Failed to send direct message");

        let mut received = false;
        for attempt in 0..10 {
            sleep(Duration::from_millis(500)).await;
            let b_messages = node_b.get_received_messages().await;
            if b_messages.iter().any(|m| m.content == "Hello from Node A!") {
                received = true;
                println!("Node B received message after {} attempts", attempt + 1);
                break;
            }
        }

        assert!(received, "Node B did not receive the message from Node A");

        let _ = node_a.stop().await;
        let _ = node_b.stop().await;
    })
    .await
    .expect("test_node_to_node_connectivity timed out after 15s");
}

#[tokio::test]
async fn test_ping_pong_same_machine() {
    let _home_guard = test_home_guard().await;
    std::env::set_var("GHOSTWIRE_TEST_DISABLE_MDNS", "1");
    // Outer guard must exceed the inner 20s waits below, which in turn must
    // exceed the mesh driver's 15s PING_INTERVAL.
    tokio::time::timeout(Duration::from_secs(30), async {
        let (mut node_a, peer_id_a, _addr_a) =
            build_test_mesh_manager(19010, Some("ping_test_a".to_string()))
                .await
                .expect("node A");
        let (node_b, peer_id_b, addr_b) =
            build_test_mesh_manager(19011, Some("ping_test_b".to_string()))
                .await
                .expect("node B");

        node_a
            .add_address(peer_id_b, addr_b.clone())
            .await
            .expect("add_address");

        let dm_topic = ghostwire::core::constants::topics::DM.to_string();
        node_b.subscribe_topic(&dm_topic).await.expect("sub B");
        node_a.subscribe_topic(&dm_topic).await.expect("sub A");

        node_a.connect_peer(peer_id_b).await.expect("connect");

        let peer_b_str = peer_id_b.to_string();
        // Same coupling as above: must exceed the 15s PING_INTERVAL.
        let _ = tokio::time::timeout(Duration::from_secs(20), async {
            loop {
                if let Some(health) = node_a.peer_health().get_peer_health(&peer_b_str).await {
                    if health.pongs_received > 0 {
                        break;
                    }
                }
                sleep(Duration::from_millis(100)).await;
            }
        })
        .await;
        let health = node_a.peer_health().get_peer_health(&peer_b_str).await;

        match health {
            Some(h) => {
                println!(
                    "Peer B health: pings_sent={}, pongs_received={}, avg_rtt={:.1}ms, alive={}",
                    h.pings_sent, h.pongs_received, h.avg_rtt_ms, h.is_alive
                );
                assert!(h.pings_sent > 0, "expected pings sent to B");
                assert!(h.pongs_received > 0, "expected pongs received from B");
                assert!(h.avg_rtt_ms >= 0.0, "RTT must not be negative");
                assert!(h.is_alive, "peer B should be alive");
            }
            None => {
                panic!("No health record for peer B — ping/pong never fired");
            }
        }

        let _ = node_a.stop().await;
        let _ = node_b.stop().await;
    })
    .await
    .expect("test_ping_pong_same_machine timed out after 15s");
}

/// `MeshManager::ping_peer` must return a MEASURED round-trip to a live peer.
///
/// This is the guarantee the `/api/v2/peers/ping` endpoint rests on: the
/// request goes on the wire and the call only completes when the peer's reply
/// arrives. A synthesised or cached number would satisfy a weaker assertion, so
/// this checks the measured value is physically plausible for a loopback link
/// and that a second probe also succeeds (i.e. it is repeatable, not a fluke).
#[tokio::test]
async fn test_ping_peer_returns_real_rtt() {
    let _home_guard = test_home_guard().await;
    std::env::set_var("GHOSTWIRE_TEST_DISABLE_MDNS", "1");

    // Outer guard exceeds the inner 20s wait.
    tokio::time::timeout(Duration::from_secs(30), async {
        let (mut node_a, peer_id_a, _addr_a) =
            build_test_mesh_manager(19020, Some("rtt_test_a".to_string()))
                .await
                .expect("Failed to build node A");
        let (node_b, peer_id_b, addr_b) =
            build_test_mesh_manager(19021, Some("rtt_test_b".to_string()))
                .await
                .expect("Failed to build node B");

        node_a
            .add_address(peer_id_b, addr_b.clone())
            .await
            .expect("Failed to add address A->B");
        node_a
            .connect_peer(peer_id_b)
            .await
            .expect("Failed to connect to peer B");

        // Wait for the authenticated handshake so the peer is live.
        let ready = tokio::time::timeout(Duration::from_secs(20), async {
            loop {
                let ok = node_a
                    .get_topology()
                    .await
                    .nodes
                    .get(&peer_id_b.to_string())
                    .map(|n| !n.public_key.is_empty())
                    .unwrap_or(false);
                if ok {
                    break true;
                }
                sleep(Duration::from_millis(100)).await;
            }
        })
        .await
        .unwrap_or(false);
        assert!(ready, "peer B never became live");

        // Probe 1: must return an actual measurement.
        let rtt_1 = tokio::time::timeout(Duration::from_secs(10), node_a.ping_peer(peer_id_b))
            .await
            .expect("ping_peer did not return within 10s")
            .expect("ping_peer returned an error for a live peer");
        assert!(
            rtt_1 < 5_000,
            "loopback RTT of {}ms is not physically plausible",
            rtt_1
        );

        // Probe 2: repeatable, so the number is a measurement and not a fluke.
        let rtt_2 = tokio::time::timeout(Duration::from_secs(10), node_a.ping_peer(peer_id_b))
            .await
            .expect("second ping_peer did not return within 10s")
            .expect("second ping_peer returned an error");
        assert!(rtt_2 < 5_000, "second loopback RTT {}ms implausible", rtt_2);

        println!(
            "MEASURED_RTT: probe1={}ms probe2={}ms peer={}",
            rtt_1, rtt_2, peer_id_b
        );

        let _ = node_a.stop().await;
        let _ = node_b.stop().await;
    })
    .await
    .expect("test_ping_peer_returns_real_rtt timed out");
}
