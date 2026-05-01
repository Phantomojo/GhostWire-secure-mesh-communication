# libp2p Common Patterns for GhostWire

> [!NOTE]
> libp2p 0.56.x as of May 2026

## Mesh Networking with Gossipsub and Kademlia

In GhostWire, we combine Gossipsub for message propagation and Kademlia for peer discovery.

### 1. Peer Discovery (Kademlia)
- Use `KademliaConfig` to set the replication factor and query timeout.
- Peers should periodically perform a random walk to discover new nodes.
- Bootstrap nodes are critical for initial connection.

### 2. Message Propagation (Gossipsub)
- Topics should be hashed or namespaced (e.g., `ghostwire/v1/messages`).
- Use `MessageAuthenticity::Signed` to ensure only authorized nodes can publish.
- Configure `GossipsubConfig` with `heartbeat_interval` tailored for mesh latency.

### 3. Swarm Behaviour
- Combine multiple protocols using a custom `NetworkBehaviour` struct.
- Use `#[derive(NetworkBehaviour)]` for automatic event delegation.

```rust
#[derive(NetworkBehaviour)]
pub struct GhostWireBehaviour {
    pub gossipsub: Gossipsub,
    pub kademlia: Kademlia<MemoryStore>,
    pub identify: Identify,
    pub ping: Ping,
}
```
