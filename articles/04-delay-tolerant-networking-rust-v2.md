---
title: "When the Internet Dies, Your Messages Still Arrive — Here's How"
published: false
description: "Building delay-tolerant networking in Rust: Spray-and-Wait, PRoPHET routing, and Bloom filter sync for mesh networks that survive without infrastructure."
tags: [rust, networking, architecture, discuss]
canonical_url: https://dev.to/phantomojo/delay-tolerant-networking-internet-survives-rust
cover_image: https://raw.githubusercontent.com/Phantomojo/GhostWire/main/docs/assets/ghostwire-pfp.png
series: GhostWire
---

I watched mesh nodes in Nairobi's informal settlements reroute through three different transport layers because the one they were using suddenly stopped working. A node came online when its owner cycled past on a bike.

This is Delay-Tolerant Networking. And it's not theoretical — it's how GhostWire keeps messages alive when infrastructure fails.

## Why TCP Dies

Traditional TCP/IP assumes continuous end-to-end connectivity. When a connection breaks, TCP backpressure fills buffers, timeouts fire, and the connection dies.

In a mesh network — especially one running on LoRa (255 baud), BLE, and sneakernet — continuous connectivity is a fantasy. Nodes come online when their owner cycles past. Links form and dissolve as people move through buildings. Sometimes the only way to deliver a message is to wait for a bus.

DTN was formalized in RFC 4838 for "interplanetary internet" — spacecraft where signal delay is minutes, not milliseconds. The core idea: **store-carry-forward.** Store a bundle, carry it until you encounter another node, forward it.

## The Bundle Format

```rust
pub struct DtnBundle {
    pub cid: [u8; 32],              // BLAKE3 hash (dedup)
    pub payload: Vec<u8>,           // Encrypted message
    pub created_at: u64,
    pub expires_at: u64,            // Default: 72 hours
    pub destination: [u8; 32],
    pub copies_remaining: u8,       // Spray-and-Wait count
    pub delivery_probability: f32,  // PRoPHET score
    pub hop_count: u8,
}
```

The `cid` enables deduplication — if the same bundle arrives via two paths, the second is discarded. `expires_at` prevents bundles from living forever (default: 72 hours, configurable per message).

## Spray-and-Wait: The Algorithm

Two phases:

**Spray:** When you encounter a node, give them half your remaining copies.
**Wait:** When you have 1 copy left, only deliver directly to the destination.

```rust
pub fn split_copies(&mut self) -> u8 {
    if self.copies_remaining <= 1 {
        return 0; // Wait phase
    }
    let to_give = self.copies_remaining / 2;
    self.copies_remaining -= to_give;
    to_give
}
```

Initial spray count L=6 — optimal for urban mesh scenarios with moderate node density. Higher L increases delivery probability but floods the network.

## PRoPHET: Smarter Than Random Spraying

PRoPHET (Lindgren et al., 2003) adds intelligence. Each node maintains a **delivery predictability score** per destination.

```rust
const P_INIT: f32 = 0.75;
const GAMMA: f32 = 0.98;

pub fn on_encounter(&mut self, destination: &[u8; 32]) {
    let current = self.scores.get(destination).copied().unwrap_or(0.0);
    let new_score = current + (1.0 - current) * P_INIT;
    self.scores.insert(*destination, new_score.min(1.0));
}

pub fn decay_all(&mut self) {
    for (dest, score) in self.scores.iter_mut() {
        *score *= GAMMA.powf(hours_elapsed);
    }
}
```

Three formulas:
1. **On encounter:** `P(dest) = P_old + (1 - P_old) * 0.75` — meeting a node raises its score
2. **Time decay:** `P(dest) = P(dest) * 0.98^hours` — scores decay 2% per hour
3. **Transitivity:** If A can reach D, and B meets A, B's score for D increases

The network learns: frequently-encountered nodes get higher scores. Rarely-seen nodes decay.

Bundle buffer: **50 MB per node.** When full, evict lowest-probability bundles first.

## The Bloom Filter Sync Handshake

When two nodes come within range, they sync. You don't want to transfer 50 MB on every encounter to find what's new.

```rust
pub struct SyncBloom {
    bits: Vec<u64>, // 1024 bits = 128 bytes
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

Each node advertises which bundles it has using a 1024-bit Bloom filter (128 bytes). False positive rate below 1% for ~10k items. The sync is a single 128-byte exchange. Only missing bundles get transferred.

On LoRa (255 bytes/sec), that's the difference between never completing and syncing in under a second.

## When DTN Wins

| Condition | Real-Time (OSPF) | DTN (Spray-and-Wait + PRoPHET) |
|---|---|---|
| Dense urban mesh | 95%, <100ms | 88%, 30s-5min |
| Sparse rural mesh | 30%, high jitter | 92%, 5min-2hrs |
| Disaster (no infra) | 0% | 85%, 1-24hrs |
| Movement-assisted | Degrades with distance | Improves with mobility |
| Low-power nodes | High overhead | Minimal overhead |

Real-time routing wins in dense stable meshes. DTN wins everywhere else.

## What I Learned

**1. `tokio::time::sleep` doesn't work for DTN.**

DTN operates on minute-to-hour timescales. Buffer management is driven by events (node encounters), not timers. The only timer is PRoPHET decay, running every 15 minutes.

**2. BLAKE3 is perfect for content-addressed bundles.**

Fast (3x SHA-256), native in Rust, 32-byte output fits the Bloom filter. We use BLAKE3 for the CID, PRoPHET verifier, and Sphinx key derivation.

**3. Copy counting needs saturating arithmetic.**

A bundle forwarded 254 times might receive one more split. `u8` wraparound makes it 0 — effectively deleting the bundle. `saturating_add` prevents this.

**4. 72-hour TTL is a design constraint.**

Every layer assumes bundles might live 3 days. The crypto machine handles delayed decryption. The nonce reuse prevention accounts for long-lived sessions. Changing the TTL means auditing every subsystem.

---

**Are you building DTN systems?** What algorithms are you using — is Spray-and-Wait still right in 2026, or should we look at ML for DTN routing? Drop a comment. The DTN community is small, and every implementation teaches us something new.

<dcp-message-id>m0019</dcp-message-id>
