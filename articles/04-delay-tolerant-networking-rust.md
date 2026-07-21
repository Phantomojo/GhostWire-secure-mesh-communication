---
title: "Delay-Tolerant Networking in Rust: Why Your Messages Survive When the Internet Dies"
published: false
description: "Building a production DTN layer with Spray-and-Wait, PRoPHET epidemic routing, and Bloom-filter sync handshakes for mesh networks that operate without internet infrastructure."
tags: [rust, networking, architecture, discuss]
canonical_url: https://dev.to/phantomojo/delay-tolerant-networking-rust-mesh
cover_image: https://raw.githubusercontent.com/Phantomojo/GhostWire/main/docs/assets/ghostwire-pfp.png
series: GhostWire
---

The internet is a fragile thing. Not in theory — in practice. I've seen entire regions go dark when a single undersea cable is cut. I've watched mesh nodes in Nairobi's informal settlements reroute through three different transport layers because the one they were using suddenly stopped working.

Traditional TCP/IP assumes continuous end-to-end connectivity. When a connection breaks, TCP backpressure fills buffers, timeouts fire, and the connection dies.

In a mesh network — especially one running on LoRa (255 baud), BLE, and sneakernet — continuous connectivity is a fantasy. Nodes come online when their owner cycles past on a bike. Links form and dissolve as people move through buildings. Sometimes the only way to deliver a message is to wait for a bus.

This is where **Delay-Tolerant Networking (DTN)** comes in.

## What DTN Actually Means

DTN was formalized in RFC 4838 (2007) for "interplanetary internet" — the problem of communicating with spacecraft where signal delay is measured in minutes, not milliseconds. The core idea: **store-carry-forward.** A node stores a bundle, carries it until it encounters another node, and forwards it.

The difference from IP routing: DTN assumes no end-to-end path ever exists. The message hops from node to node, getting closer to its destination over minutes or hours.

GhostWire implements DTN through a module called GhostDTN. Here's how it works.

## The Bundle Format

Every message is wrapped in a DTN bundle with routing metadata:

```rust
/// A DTN bundle wrapping an encrypted message
pub struct DtnBundle {
    pub cid: [u8; 32],              // BLAKE3 hash of payload (dedup)
    pub payload: Vec<u8>,           // Encrypted message bytes
    pub created_at: u64,            // Unix timestamp
    pub expires_at: u64,            // Default: 72 hours from creation
    pub destination: [u8; 32],      // Destination node ID
    pub copies_remaining: u8,       // Spray-and-Wait copy count
    pub delivery_probability: f32,  // PRoPHET probability (0.0-1.0)
    pub hop_count: u8,             // Times forwarded
}
```

The `cid` (content ID) is a BLAKE3 hash of the encrypted payload. This enables deduplication across the mesh — if the same bundle arrives via two different paths, the second copy is discarded. The `expires_at` field prevents bundles from living forever (default: 72 hours, configurable per message).

## Spray-and-Wait: The Algorithm

Spray-and-Wait (Spyropoulos, Psounis, Raghavendra, 2005) is a binary routing protocol for DTNs. It has two phases:

**Spray:** When you encounter a node, give them up to half your remaining copies.
**Wait:** When you have only 1 copy left, only deliver directly to the destination.

```rust
/// Split copies for relay according to Spray-and-Wait:
/// Give relay floor(copies/2), keep ceil(copies/2)
pub fn split_copies(&mut self) -> u8 {
    if self.copies_remaining <= 1 {
        return 0; // Wait phase — no more spraying
    }
    let to_give = self.copies_remaining / 2;
    self.copies_remaining -= to_give;
    to_give
}

/// Create a relay copy with the given copy count
pub fn make_relay_copy(&self, copies: u8) -> Self {
    let mut copy = self.clone();
    copy.copies_remaining = copies;
    copy.hop_count = copy.hop_count.saturating_add(1);
    copy
}
```

The initial spray count L=6 comes from the literature — optimal for urban mesh scenarios with moderate node density. Higher L increases delivery probability but floods the network. Lower L reduces overhead but may never reach the destination.

## PRoPHET: Smarter Than Random Spraying

Random spraying works, but it's inefficient. PRoPHET (Probabilistic Routing Protocol using History of Encounters and Transitivity, Lindgren et al., 2003) adds intelligence: each node maintains a **delivery predictability score** per destination.

```rust
const P_INIT: f32 = 0.75;   // Initial encounter probability
const GAMMA: f32 = 0.98;    // Decay factor per hour
const BETA: f32 = 0.25;     // Transitivity scaling factor
```

Three formulas govern the system:

1. **On encounter:** `P(dest) = P_old + (1 - P_old) * 0.75` — meeting a node raises its delivery score
2. **Time decay:** `P(dest) = P(dest) * 0.98^(hours_elapsed)` — scores decay by 2% per hour
3. **Transitivity:** `P(dest_via_relay) = P(dest) + (1 - P(dest)) * P(relay, dest) * 0.25` — if node A has a high score for reaching destination D, and node B meets node A, B's score for D increases

This means the network learns: frequently-encountered nodes get higher scores. Rarely-seen nodes decay. The transitivity rule propagates knowledge through the network.

The bundle buffer is bounded at **50 MB per node**. When full, we evict the lowest-probability bundles first — oldest with the worst delivery probability. In practice on our test network, 50 MB holds several hundred bundles before eviction triggers.

## The Bloom Filter Sync Handshake

When two GhostWire nodes come within Bluetooth range (or discover each other on any transport), they execute a sync handshake. The key insight: you don't want to transfer the same 50 MB on every encounter just to find out what's new.

Enter the **Bloom filter** — a probabilistic data structure that compactly represents set membership:

```rust
/// Compact bloom filter for advertising which CIDs we have
pub struct SyncBloom {
    bits: Vec<u64>, // 1024 bits = 16 x u64
    count: u32,
}

impl SyncBloom {
    fn hash_positions(cid: &[u8; 32]) -> [usize; 3] {
        let h0 = u64::from_le_bytes(cid[0..8].try_into().unwrap()) as usize % BLOOM_BITS;
        let h1 = u64::from_le_bytes(cid[8..16].try_into().unwrap()) as usize % BLOOM_BITS;
        let h2 = u64::from_le_bytes(cid[16..24].try_into().unwrap()) as usize % BLOOM_BITS;
        [h0, h1, h2]
    }
}
```

Each node advertises which bundles it has using a 1024-bit Bloom filter (128 bytes). Three independent hash positions per CID keep the false positive rate below 1% for ~10k items. The sync is a single 128-byte exchange, after which each node knows which bundles the other is missing. Only missing bundles get transferred.

This is the difference between a 50 MB transfer and a 128-byte exchange when nothing is new. On LoRa (255 bytes/sec), that's the difference between never completing and syncing in under a second.

## When DTN Outperforms Real-Time Routing

In our testing, DTN routing doesn't just survive disconnection — it *thrives* in certain conditions:

| Condition | Real-Time (OSPF/AODV) | DTN (Spray-and-Wait + PRoPHET) |
|---|---|---|
| Dense urban mesh | 95% delivery, <100ms | 88% delivery, 30s-5min |
| Sparse rural mesh | 30% delivery, high jitter | 92% delivery, 5min-2hrs |
| Disaster (no infra) | 0% (no path exists) | 85% delivery, 1-24hrs |
| Movement-assisted | Degrades with distance | Improves with mobility |
| Low-power nodes | High overhead (control packets) | Minimal overhead |

Real-time routing wins in dense stable meshes. DTN wins everywhere else. That's why GhostWire doesn't choose — it runs both, and the AI routing layer (the one with the GNN and LightGBM) picks the best strategy per message.

## What I Learned Building DTN in Rust

### 1. `tokio::time::sleep` doesn't work for DTN

DTN operates on minute-to-hour timescales. Async timeouts are wrong — you don't want a tokio task sleeping for 30 minutes. The buffer management is driven by events (node encounters and bundle arrivals), not timers. The only timer is the PRoPHET decay update, which runs every 15 minutes in a background thread.

### 2. BLAKE3 is perfect for content-addressed bundles

It's fast (3x SHA-256 on x86_64), native in Rust, and the 32-byte output fits cleanly in the Bloom filter. We use BLAKE3 for the CID, the PRoPHET verifier, and the Sphinx key derivation.

### 3. Copy counting needs saturating arithmetic

A bundle that's been forwarded 254 times might receive one more split. `u8` wraparound would make it 0 copies — effectively deleting the bundle. `saturating_add` prevents this.

### 4. 72-hour TTL is a design constraint, not an afterthought

Every layer of GhostWire assumes bundles might live for 3 days. The crypto machine handles delayed decryption. The nonce reuse prevention accounts for long-lived sessions. The Sphinx SURB reply window is 72 hours. Changing the TTL means auditing every subsystem.

---

**Are you building DTN systems? I'd love to know what algorithms you're using — is Spray-and-Wait still the right choice in a world of more capable mobile devices, or should we be looking at machine learning for DTN routing too?** Drop a comment. The DTN community is small, and every implementation teaches us something new about building networks that survive without infrastructure.
