# 30-Day libp2p Deep Dive Curriculum

**Version:** 1.0  
**Time Commitment:** 1-2 hours/day  
**Level:** Intermediate to Advanced

---

## Overview

This structured learning path takes you from libp2p basics to advanced mesh networking concepts, with hands-on exercises using GhostWire.

---

## Week 1: Foundations (Days 1-7)

### Day 1: P2P Basics
**Topics:**
- What is peer-to-peer networking?
- Client-server vs P2P architectures
- libp2p history and ecosystem (IPFS, Ethereum 2.0, Filecoin)

**Reading:**
- [libp2p Documentation](https://docs.libp2p.io/)
- `.qwen/library/whitepapers/libp2p/Kademlia_DHT.pdf`

**Exercise:**
```bash
# Explore GhostWire's libp2p dependencies
cd ghostwire
grep "libp2p" Cargo.toml
```

**Duration:** 1 hour

---

### Day 2: libp2p Architecture
**Topics:**
- Modular architecture (transport, multiplexing, protocols)
- The libp2p stack diagram
- Protocol negotiation (multistream-select)

**Reading:**
- [libp2p Architecture Overview](https://docs.libp2p.io/concepts/fundamentals/architecture/)

**Exercise:**
```bash
# Identify libp2p components in GhostWire
grep -rn "libp2p::" ghostwire/src/core/mesh.rs | head -20
```

**Duration:** 1.5 hours

---

### Day 3: Peer IDs and Cryptography
**Topics:**
- Peer ID generation (Blake2b hash of public key)
- Ed25519 keypairs for peer identity
- Multiaddr format

**Reading:**
- `.qwen/library/cheat-sheets/crypto-cheat-sheet.md`
- [libp2p Peer ID Spec](https://github.com/libp2p/specs/blob/master/peer-ids/peer-ids.md)

**Exercise:**
```rust
// Generate a peer ID in Rust
use libp2p::identity::Keypair;
let keypair = Keypair::generate_ed25519();
let peer_id = keypair.public().to_peer_id();
println!("Peer ID: {}", peer_id);
```

**Duration:** 1.5 hours

---

### Day 4: Transport Layer
**Topics:**
- TCP, UDP, WebRTC transports
- TLS/Noise encryption
- Custom transports (GhostWire's stealth TCP)

**Reading:**
- [libp2p Transports](https://docs.libp2p.io/concepts/transport/)
- `ghostwire/src/core/stealth_tcp.rs`

**Exercise:**
```bash
# Compare standard TCP vs stealth TCP
cat ghostwire/src/core/stealth_tcp.rs | head -50
```

**Duration:** 1.5 hours

---

### Day 5: Multiplexing (Yamux)
**Topics:**
- Why multiplexing?
- Yamux protocol
- Stream management

**Reading:**
- [Yamux Spec](https://github.com/hashicorp/yamux/blob/master/spec.md)

**Exercise:**
```bash
# Find Yamux configuration in GhostWire
grep -rn "yamux\|Yamux" ghostwire/src/
```

**Duration:** 1 hour

---

### Day 6: Noise Protocol
**Topics:**
- Noise handshake (XX pattern)
- X25519 key exchange
- Authenticated encryption

**Reading:**
- [Noise Protocol Framework](https://noiseprotocol.org/)
- `.qwen/library/whitepapers/libp2p/Noise_Protocol.html`

**Exercise:**
```bash
# Analyze Noise configuration
grep -A 10 "noise" ghostwire/src/core/mesh.rs
```

**Duration:** 1.5 hours

---

### Day 7: Week 1 Review + Hands-on
**Activities:**
- Review all concepts
- Build GhostWire with debug logging
- Start a single node and observe logs

**Exercise:**
```bash
cd ghostwire
RUST_LOG=debug cargo run -- --port 8080 --mesh-port 4001 &
tail -f /tmp/ghostwire-*.log
```

**Duration:** 2 hours

---

## Week 2: Kademlia DHT (Days 8-14)

### Day 8: DHT Fundamentals
**Topics:**
- What is a Distributed Hash Table?
- Key-value storage across peers
- Consistency models

**Reading:**
- [Kademlia Paper](https://pdos.csail.mit.edu/~petar/papers/maymounkov-kademlia-lncs.pdf)

**Duration:** 2 hours

---

### Day 9: XOR Distance Metric
**Topics:**
- XOR as distance metric
- Properties (symmetry, triangle inequality)
- Bucket organization

**Exercise:**
```python
# Calculate XOR distance
def xor_distance(a, b):
    return int(a, 16) ^ int(b, 16)

# Example
peer_a = "QmPeer1..."
peer_b = "QmPeer2..."
distance = xor_distance(peer_a, peer_b)
```

**Duration:** 1.5 hours

---

### Day 10: K-Buckets
**Topics:**
- Routing table structure
- Bucket ranges
- Eviction policies

**Reading:**
- `docs/protocol-knowledge/PROTOCOL_DEEP_DIVE.md`

**Exercise:**
```bash
# Check routing table size
curl "http://localhost:9097/api/v2/peers" | jq '.data | length'
```

**Duration:** 1.5 hours

---

### Day 11: Kademlia RPC
**Topics:**
- FIND_NODE
- FIND_VALUE
- ADD_PROVIDER / GET_PROVIDERS
- PUT_VALUE / GET_VALUE

**Reading:**
- [Kademlia Spec](https://github.com/libp2p/specs/blob/master/kad-dht/README.md)

**Duration:** 2 hours

---

### Day 12: Provider Records
**Topics:**
- Content addressing
- Provider announcements
- Record replication

**Exercise:**
```bash
# Watch provider records in action
RUST_LOG=libp2p-kad=debug ./ghostwire --mesh &
```

**Duration:** 1.5 hours

---

### Day 13: Bootstrap Process
**Topics:**
- Bootstrap nodes
- Initial peer discovery
- Routing table population

**Reading:**
- `ghostwire.toml.example` - bootstrap configuration

**Exercise:**
```bash
# Configure bootstrap peers
cat ghostwire.toml.example | grep -A 5 bootstrap
```

**Duration:** 1.5 hours

---

### Day 14: Week 2 Review + DHT Analysis
**Activities:**
- Run DHT analysis skill
- Examine routing table
- Query for providers

**Exercise:**
```bash
skill: "libp2p-dht" --routing-table
```

**Duration:** 2 hours

---

## Week 3: Gossipsub (Days 15-21)

### Day 15: Pubsub Basics
**Topics:**
- Publish-subscribe pattern
- Floodsub vs Gossipsub
- Topics and subscriptions

**Reading:**
- [Gossipsub Spec](https://github.com/libp2p/specs/blob/master/pubsub/gossipsub/gossipsub-v1.1.md)

**Duration:** 2 hours

---

### Day 16: Mesh Topology
**Topics:**
- Mesh degree (D=6)
- Watermarks (D_lo, D_hi)
- Peer selection

**Exercise:**
```bash
# Monitor mesh peers
curl "http://localhost:9097/api/v2/mesh/topics" | jq '.'
```

**Duration:** 1.5 hours

---

### Day 17: Message Propagation
**Topics:**
- Heartbeat mechanism
- Gossip factor
- Message caching

**Duration:** 2 hours

---

### Day 18: Peer Scoring
**Topics:**
- Behavior penalties
- Message delivery scores
- Graylisting/blacklisting

**Reading:**
- [Gossipsub Score Spec](https://github.com/libp2p/specs/blob/master/pubsub/gossipsub/gossipsub-v1.1.md#peer-scoring)

**Duration:** 2 hours

---

### Day 19: GhostWire Topics
**Topics:**
- `/ghostwire/messages`
- `/ghostwire/events`
- `/ghostwire/discovery`

**Exercise:**
```bash
# List subscribed topics
curl "http://localhost:9097/api/v2/mesh/topics"
```

**Duration:** 1.5 hours

---

### Day 20: Message Validation
**Topics:**
- Message ID functions
- Signature verification
- Duplicate detection

**Duration:** 1.5 hours

---

### Day 21: Week 3 Review + Gossip Analysis
**Activities:**
- Run gossipsub monitoring
- Analyze message propagation
- Check peer scores

**Exercise:**
```bash
skill: "libp2p-dht" --gossip-topics
```

**Duration:** 2 hours

---

## Week 4: Advanced Topics (Days 22-30)

### Day 22: mDNS Discovery
**Topics:**
- Zero-config discovery
- DNS-SD (DNS Service Discovery)
- Local network announcements

**Exercise:**
```bash
# Browse mDNS services
avahi-browse -a | grep -i ghostwire
```

**Duration:** 1.5 hours

---

### Day 23: Identify Protocol
**Topics:**
- Peer info exchange
- Protocol negotiation
- Agent version

**Duration:** 1.5 hours

---

### Day 24: AutoNAT
**Topics:**
- NAT detection
- Reachability status
- Public vs Private

**Duration:** 1.5 hours

---

### Day 25: Relay v2
**Topics:**
- Circuit relay
- Hole punching
- NAT traversal

**Reading:**
- [Relay Spec](https://github.com/libp2p/specs/blob/master/relay/README.md)

**Duration:** 2 hours

---

### Day 26: DCUtR
**Topics:**
- Direct Connection Upgrade through Relay
- Simultaneous connect
- Port mapping

**Duration:** 2 hours

---

### Day 27: Two-Environment Testing
**Activities:**
- Setup Tailscale
- Deploy to stratsytem
- Test cross-env discovery

**Exercise:**
```bash
skill: "two-env-test" --test-discovery
```

**Duration:** 2 hours

---

### Day 28: Performance Tuning
**Topics:**
- Connection limits
- Rate limiting
- Resource management

**Exercise:**
```bash
# Monitor connection stats
skill: "libp2p-dht" --health
```

**Duration:** 2 hours

---

### Day 29: Security Considerations
**Topics:**
- Sybil attacks
- Eclipse attacks
- Mitigation strategies

**Reading:**
- `docs/SECURITY.md`

**Duration:** 2 hours

---

### Day 30: Final Project
**Activities:**
- Build a complete mesh network
- Send messages across peers
- Document learnings

**Exercise:**
```bash
# Full mesh test
skill: "mesh-test" --nodes 3
```

**Duration:** 3 hours

---

## Assessment

### Knowledge Check
After completing the curriculum, you should be able to:

- [ ] Explain libp2p architecture
- [ ] Calculate XOR distance between peer IDs
- [ ] Configure Kademlia routing
- [ ] Debug Gossipsub mesh issues
- [ ] Setup two-environment testing
- [ ] Analyze mesh health metrics

### Next Steps
- Contribute to GhostWire
- Explore protocol bridges (Briar, Matrix)
- Study advanced cryptography (PQC, ZK)

---

## Resources

### Primary
- [libp2p Documentation](https://docs.libp2p.io/)
- `.qwen/library/` - Offline books and papers
- `docs/protocol-knowledge/PROTOCOL_DEEP_DIVE.md`

### Communities
- libp2p Discord
- IPFS Discord
- r/rust (Reddit)

---

**Created:** March 2026  
**Maintained By:** GhostWire Development Team  
**License:** CC-BY-SA-4.0
