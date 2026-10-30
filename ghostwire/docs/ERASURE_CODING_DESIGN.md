# Erasure & Fountain Coding — Design Note for GhostWire DTN

## Purpose

Replace AIMD retransmission with erasure/fountain coding for DTN bundles.
Directly addresses delivery reliability on lossy radio/DTN paths.

## Problem

GhostWire's current reliability: AIMD retransmits whole bundles on loss.
On lossy radio/DTN paths, this is inefficient and slow. A single lost
packet requires retransmitting the entire bundle.

## Solution: Erasure Coding

Split each bundle into k data shards + (n-k) parity shards. Any k of n
shards can reconstruct the original bundle. No per-packet ACKs needed.

### Options

| Scheme | Pros | Cons | Best For |
|--------|------|------|----------|
| Reed-Solomon | Simple, well-understood, exact recovery | Fixed rate, O(n²) encode/decode | Small bundles, known loss rate |
| LT (Luby Transform) | Rateless, any k of n, simple | Suboptimal for small n, needs large blocks | Large bundles, unknown loss rate |
| Raptor | Linear-time decode, near-optimal | Complex, patent concerns | High-throughput, large bundles |
| COPE-XOR | Simple, low overhead | Limited gain (5-70%), pollution risk | Simple relay scenarios |

### GhostWire's Bundle Sizes

GhostWire messages are typically < 10KB. DTN bundles may be larger (up to
64KB with mailbox). For small bundles:
- Reed-Solomon is likely best (simple, exact recovery)
- LT needs large block sizes for near-optimal rates
- Raptor is overkill for small messages

### Recommendation

**Phase 1**: Reed-Solomon (k,n) with k = message size / shard size, n = k + parity
- Shard size: 256 bytes (fits in single RNS packet)
- Parity: 25% (n = 1.25 * k)
- Any k of n shards reconstruct the original

**Phase 2**: Evaluate LT/Raptor if bundle sizes grow or loss rates are high

### Implementation Plan

1. Create `src/core/erasure.rs` with Reed-Solomon encode/decode
2. Add `shard_bundle()` and `reconstruct_bundle()` functions
3. Wire into DTN send path (shard before sending)
4. Wire into DTN receive path (reconstruct on receipt)
5. Add shard tracking (which shards received, which missing)
6. Add timeout for incomplete reconstruction

### Integration with Transport Selector

The transport selector should choose:
- High-quality links: no erasure coding (direct send)
- Medium-quality links: Reed-Solomon (25% parity)
- Low-quality links: Reed-Solomon (50% parity) or fountain codes
- DTN paths: maximum parity (store-and-forward with erasure)

### References

- Reed-Solomon: Berlekamp-Massey algorithm
- Fountain codes: MacKay 2005, Luby 2002 LT, Shokrollahi 2006 Raptor
- Network coding: Ahlswede et al. 2000, COPE (Katti et al.)
- GhostWire DTN: docs/DTN.md
EOF
echo "created"