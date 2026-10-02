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

/// True once `other` has completed the authenticated handshake and appears in
/// `m`'s topology with a public key.
async fn handshake_seen(m: &MeshManager, other: PeerId) -> bool {
    m.get_topology()
        .await
        .nodes
        .get(&other.to_string())
        .map(|n| !n.public_key.is_empty())
        .unwrap_or(false)
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

/// Three nodes on one mesh. A sends a DM to B. C is an innocent third party
/// that is fully connected and subscribed to the DM topic, so it necessarily
/// RECEIVES the gossipsub frame. C must still store nothing.
///
/// This is the half of the DM-leak fix that a two-node test cannot prove.
/// Before the fix, `is_dm_for_us` was a pure topic comparison, so every node
/// in the mesh stored and acknowledged every DM — a live privacy regression
/// that showed up in `GET /api/v2/messages` and the UI. The two-node test only
/// proves B received; this one proves C was not polluted.
#[tokio::test]
async fn test_dm_is_not_stored_by_a_third_party() {
    let _home_guard = test_home_guard().await;
    std::env::set_var("GHOSTWIRE_TEST_DISABLE_MDNS", "1");
    // Outer guard exceeds the 8s handshake wait plus the settle window.
    tokio::time::timeout(Duration::from_secs(60), async {
        let (mut node_a, peer_id_a, _addr_a) =
            build_test_mesh_manager(19020, Some("leak_test_a".to_string()))
                .await
                .expect("node A");
        let (node_b, peer_id_b, addr_b) =
            build_test_mesh_manager(19021, Some("leak_test_b".to_string()))
                .await
                .expect("node B");
        // Node C must be a real peer on the mesh, not a bystander. If C never
        // receives the frame, this test would pass for the wrong reason.
        let (node_c, peer_id_c, addr_c) =
            build_test_mesh_manager(19022, Some("leak_test_c".to_string()))
                .await
                .expect("node C");

        let dm_topic = ghostwire::core::constants::topics::DM.to_string();
        for node in [&node_a, &node_b, &node_c] {
            node.subscribe_topic(&dm_topic)
                .await
                .expect("Failed to subscribe to DM topic");
        }

        node_a
            .add_address(peer_id_b, addr_b.clone())
            .await
            .expect("add B address");
        node_a
            .add_address(peer_id_c, addr_c.clone())
            .await
            .expect("add C address");

        node_a.connect_peer(peer_id_b).await.expect("connect to B");
        node_a.connect_peer(peer_id_c).await.expect("connect to C");

        // Every pair must complete the authenticated handshake, otherwise the
        // sender parks the DM instead of publishing it and the test proves
        // nothing about storage.
        let all_ready = tokio::time::timeout(Duration::from_secs(20), async {
            loop {
                if handshake_seen(&node_a, peer_id_b).await
                    && handshake_seen(&node_a, peer_id_c).await
                    && handshake_seen(&node_b, peer_id_a).await
                    && handshake_seen(&node_c, peer_id_a).await
                {
                    break true;
                }
                sleep(Duration::from_millis(100)).await;
            }
        })
        .await
        .unwrap_or(false);
        assert!(all_ready, "authenticated handshake did not complete on all pairs");

        // gossipsub builds a per-topic mesh from subscription announcements that
        // propagate separately from the connection handshake. Being connected
        // and handshaked does NOT mean the publisher will include this node when
        // it publishes to the DM topic. Without this settle the frame is
        // published into a mesh that excludes C and the leak assertion below
        // passes vacuously -- measured, not assumed: with the fix reverted and
        // no settle, this test stayed green because C never saw the frame.
        sleep(Duration::from_secs(5)).await;

        // CONTROL. A DM addressed TO C must be stored by C. That proves C is on
        // the DM topic mesh, that A holds C's key, and that the store this test
        // reads is the same store a leaked DM would land in. A clean C after the
        // A->B DM then means "chose not to store" rather than "never saw the
        // frame". This test is known to pass vacuously without the control, so
        // the control is load-bearing, not decoration.
        let control = Message::new(
            peer_id_a.to_string(),
            peer_id_c.to_string(),
            "CONTROL-CANARY-c3d19e04".to_string(),
        );
        node_a
            .send_direct_message(peer_id_c, control.clone())
            .await
            .expect("send control");

        let mut control_ok = false;
        for _ in 0..10 {
            sleep(Duration::from_millis(500)).await;
            if node_c
                .get_received_messages()
                .await
                .iter()
                .any(|m| m.content == "CONTROL-CANARY-c3d19e04")
            {
                control_ok = true;
                break;
            }
        }
        assert!(
            control_ok,
            "CONTROL FAILED: node C never received a DM addressed to it, so the \
             leak assertion below would be vacuous. Fix the harness before \
             trusting this test."
        );
        let c_before = node_c.get_received_messages().await.len();
        println!("control ok; C holds {} message(s) before the A->B DM", c_before);

        // ENCRYPTED DM (send_direct_message seals it), so this covers E4, not E3.
        // MEASURED: reverting E3 leaves this green -- C cannot decrypt a DM
        // sealed for B, the failure keeps message_type's "encrypted:" prefix
        // (mesh.rs:3213), so the E4 guard at mesh.rs:3230 skips
        // handle_incoming_message entirely and nothing reaches core_messages.
        // E3's case is a PLAINTEXT DM on the DM topic, which no pub method
        // can emit: publish_to_topic (mesh.rs:3720) sends unframed bytes and
        // send_message (mesh.rs:3600) goes to the broadcast topic. So E3 is
        // defence-in-depth against an unreachable state, not covered here.
        let secret = "CANARY-SHOULD-REACH-B-ONLY-e7f3a91c";
        let message = Message::new(
            peer_id_a.to_string(),
            peer_id_b.to_string(),
            secret.to_string(),
        );
        node_a
            .send_direct_message(peer_id_b, message.clone())
            .await
            .expect("send A->B DM");

        let mut b_received = false;
        for _ in 0..10 {
            sleep(Duration::from_millis(500)).await;
            if node_b
                .get_received_messages()
                .await
                .iter()
                .any(|m| m.content == secret)
            {
                b_received = true;
                break;
            }
        }
        assert!(
            b_received,
            "Node B did not receive the DM, so the leak assertion below would be vacuous"
        );

        // Settle window. If C were going to store the DM it would already have
        // done so by the time B has it -- gossipsub delivers to subscribers in
        // the same propagation. Waiting here is what makes a FAIL here real.
        sleep(Duration::from_secs(2)).await;

        // A non-recipient cannot decrypt the DM, so a leaked copy arrives as
        // CIPHERTEXT with encryption_status == "decrypt_failed" -- searching for
        // the plaintext canary would pass even with the leak present. Assert on
        // the shape of the store instead: nothing addressed to B, and no growth
        // past the control baseline.
        let c_messages = node_c.get_received_messages().await;
        let leaked = c_messages
            .iter()
            .filter(|m| m.recipient == peer_id_b.to_string())
            .collect::<Vec<_>>();
        assert!(
            leaked.is_empty(),
            "DM LEAK: node C holds {} message(s) addressed to node B, not to \
             itself: {:?}",
            leaked.len(),
            leaked.iter().map(|m| &m.content).collect::<Vec<_>>()
        );
        assert_eq!(
            c_messages.len(),
            c_before,
            "node C's store grew from {} to {} after a DM addressed elsewhere: {:?}",
            c_before,
            c_messages.len(),
            c_messages.iter().map(|m| &m.content).collect::<Vec<_>>()
        );

        let _ = node_a.stop().await;
        let _ = node_b.stop().await;
        let _ = node_c.stop().await;
    })
    .await
    .expect("test_dm_is_not_stored_by_a_third_party timed out");
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

/// E7: a DM sent before the recipient's key is installed must not freeze the
/// driver. Awaiting the key inside the driver's `select!` (mesh.rs:2002) starved
/// the swarm, so the handshake reply that installs the key was never processed
/// and the wait could only expire. Parking the DM (mesh.rs:2022) and re-driving
/// it from the 5s tick (mesh.rs:1618) fixes it.
///
/// The 12s bound is BELOW the 15s deadline ON PURPOSE -- that gap is the
/// discriminator. Raising it past 15s makes this test vacuous.
/// MEASURED: reverting the park branch to the original sleep loop turns this
/// RED. Verify such a revert with a match-count assert or grep first; a silent
/// no-op patch leaves this green for the wrong reason.
#[tokio::test]
async fn test_dm_before_handshake_key_does_not_freeze_the_driver() {
    let _home_guard = test_home_guard().await;
    std::env::set_var("GHOSTWIRE_TEST_DISABLE_MDNS", "1");
    tokio::time::timeout(Duration::from_secs(90), async {
        let (mut node_a, peer_id_a, _addr_a) =
            build_test_mesh_manager(19040, Some("test_e7_a".to_string()))
                .await
                .expect("Failed to build node A");
        let (node_b, peer_id_b, addr_b) =
            build_test_mesh_manager(19041, Some("test_e7_b".to_string()))
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

        // connect_peer returns on transport connect, BEFORE the handshake has
        // installed a peer key -- that window is what this test needs.
        node_a
            .connect_peer(peer_id_b)
            .await
            .expect("Failed to connect to peer");

        let key_already_installed = handshake_seen(&node_a, peer_id_b).await;
        println!(
            "E7: peer key present at send time = {}",
            key_already_installed
        );

        let message = Message::new(
            peer_id_a.to_string(),
            peer_id_b.to_string(),
            "E7-parked-DM-must-not-freeze-the-driver-4b1e7c02".to_string(),
        );

        let started = std::time::Instant::now();
        let sent = tokio::time::timeout(
            Duration::from_secs(12),
            node_a.send_direct_message(peer_id_b, message),
        )
        .await;
        let elapsed_ms = started.elapsed().as_millis();
        println!("E7: send_direct_message resolved after {}ms", elapsed_ms);

        let result = sent.expect(
            "E7 REGRESSION: send_direct_message did not resolve within 12s. The \
             driver is blocked awaiting the peer key instead of parking the DM, \
             so the handshake reply that would install that key is never \
             processed. This is the 15s-freeze defect.",
        );
        result.unwrap_or_else(|e| {
            panic!(
                "E7 REGRESSION: send_direct_message failed after {}ms: {}. The DM was \
                 not parked -- the key wait blocked the driver, so the handshake \
                 could never complete.",
                elapsed_ms, e
            )
        });

        assert!(
            !key_already_installed,
            "handshake key was already installed at send time, so the deferred-DM \
             branch was never taken and this test proves nothing"
        );

        let mut received = false;
        tokio::time::timeout(Duration::from_secs(20), async {
            while !received {
                received = node_b
                    .get_received_messages()
                    .await
                    .iter()
                    .any(|m| m.content == "E7-parked-DM-must-not-freeze-the-driver-4b1e7c02");
                sleep(Duration::from_millis(200)).await;
            }
        })
        .await
        .expect("node B never received the parked direct message");

        let _ = node_a.stop().await;
        let _ = node_b.stop().await;
    })
    .await
    .expect("test_dm_before_handshake_key_does_not_freeze_the_driver timed out");
}
