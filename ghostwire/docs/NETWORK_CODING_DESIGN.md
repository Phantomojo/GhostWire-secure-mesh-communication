# Network Coding — Design for GhostWire Mesh

## Purpose

Replace store-and-forward with network coding (XOR/RLNC) for mesh
broadcasts. Improves throughput by 5-70% in relay scenarios by allowing
intermediate nodes to mix packets instead of just forwarding.

## Problem

GhostWire's current relay: intermediate nodes forward whole packets.
This is inefficient when multiple flows share paths. Network coding allows
intermediate nodes to XOR/mix packets, reducing retransmissions.

## Options

| Scheme | Pros | Cons | Best For |
|--------|------|------|----------|
| COPE-XOR | Simple, low overhead | Limited gain (5-70%), pollution risk | Simple relay scenarios |
| RLNC (Random Linear) | Higher gain, no coordination | Higher complexity, pollution risk | Mesh with multiple flows |
| BATS (Batched) | Scalable, fountain-like | Complex, needs large batches | Large file transfers |
| Sparse coding | Low overhead, good for small packets | Less gain than RLNC | Small message mesh |

## GhostWire's Use Case

GhostWire messages are small (< 10KB). Network coding is most beneficial
for:
- Broadcast messages (gossipsub already handles this, but coding could reduce redundancy)
- DTN bundles (store-and-forward could use coding for partial recovery)
- Multi-path routing (Sphinx could use coding for path diversity)

## Recommendation

**Phase 1**: XOR coding for broadcast redundancy
- Sender sends original + coded packets (XOR of pairs)
- Receivers can recover from any single loss
- Low overhead, simple implementation

**Phase 2**: Evaluate RLNC for DTN bundles
- RLNC allows partial recovery from any subset of coded packets
- Better for high-loss radio paths
- Needs larger block sizes

## Implementation Plan

1. Create `src/core/network_coding.rs`
2. Implement XOR encode/decode for pairs of messages
3. Wire into gossipsub publish (send coded packets alongside originals)
4. Wire into gossipsub receive (decode coded packets)
5. Add coding gain metrics to mesh health

## References

- Ahlswede et al. 2000: "Network Information Flow"
- Katti et al. 2006: "XORs in the Air: Practical Wireless Network Coding"
- Ho et al. 2006: "A Random Linear Network Coding Approach"
- Chou et al. 2003: "Practical Network Coding"
EOF
echo "created"