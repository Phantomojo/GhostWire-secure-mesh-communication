//! Node-to-node connectivity test for GhostWire Mesh
//!
//! NOTE: This test requires full mesh networking setup (libp2p swarm, etc.)
//! and is disabled until the mesh infrastructure is testable in isolation.
//!
//! To re-enable: implement a test swarm with mock transports.

#[ignore = "Requires full mesh infrastructure setup"]
#[tokio::test]
async fn test_node_to_node_connectivity() {
    // TODO: Implement with test swarm
    panic!("Not yet implemented — requires mesh test infrastructure");
}
