# Delay-Tolerant Networking Specification

**GIP:** Pending
**Version:** 0.2.0-draft
**Date:** April 12, 2026

---

## Overview

This specification defines the Delay-Tolerant Networking (DTN) protocols used by GhostWire for communication in intermittently connected environments. DTN enables message delivery when end-to-end connectivity is not continuously available — messages are stored, carried, and forwarded when opportunities arise. This is essential for mesh networks in rural areas, disaster zones, and mobile scenarios.

GhostWire implements two DTN routing strategies: Spray-and-Wait for initial distribution and PRoPHET epidemic routing for intelligent forwarding based on delivery predictability.

---

## Bundle Protocol

### DtnBundle Format

All DTN messages are encapsulated in bundles.

```rust
pub struct DtnBundle {
    pub bundle_id: [u8; 32],          // BLAKE3(serialized_bundle_header)
    pub source: [u8; 32],             // Source peer ID (BLAKE3 of pubkey)
    pub destination: [u8; 32],        // Destination peer ID
    pub payload: Vec<u8>,             // Encrypted application message
    pub ttl: u64,                     // Time-to-live in seconds
    pub max_copies: u8,               // Maximum number of copies to distribute
    pub priority: BundlePriority,     // Message priority level
    pub created_at: u64,              // Unix timestamp (HLC)
    pub hop_count: u8,                // Number of hops traversed
    pub previous_hops: Vec<[u8; 32]>, // Previous carrier nodes (for PRoPHET)
    pub sync_bloom: Option<[u8; 128]>, // SyncBloom for deduplication (1024-bit)
}

pub enum BundlePriority {
    Bulk,          // Lowest priority (background sync, large files)
    Normal,        // Default priority (regular messages)
    Expedited,     // High priority (urgent messages)
    Critical,      // Highest priority (distress alerts, panic mode)
}
```

### Bundle ID

The bundle ID is computed as:
```
bundle_id = BLAKE3(source || destination || created_at || payload_hash, output_len=32)
```

This ensures uniqueness even for duplicate payloads sent to the same recipient.

### TTL and Expiry

| Priority | Default TTL | Max TTL |
|----------|------------|---------|
| Bulk | 24 hours | 7 days |
| Normal | 12 hours | 72 hours |
| Expedited | 4 hours | 24 hours |
| Critical | 1 hour | 12 hours |

**Expiry check:** A bundle is expired when `current_time - created_at > ttl`. Expired bundles are silently discarded and removed from the buffer.

### Bundle Size Limits

| Property | Value |
|----------|-------|
| **Maximum payload size** | 64 KB per bundle |
| **Maximum bundle size** | 65 KB (payload + headers) |
| **Fragmentation** | Messages >64 KB are split into multiple bundles with a fragment header |

**Fragmentation:**
```rust
pub struct FragmentHeader {
    pub total_fragments: u16,
    pub fragment_index: u16,
    pub original_bundle_id: [u8; 32],
    pub fragment_size: u32,
}
```

Fragments are reassembled at the destination. Missing fragments cause the entire message to be discarded after TTL expiry.

---

## Spray-and-Wait Routing

Spray-and-Wait is a two-phase DTN routing protocol.

### Phase 1: Spray

The source node distributes up to `max_copies` copies of the bundle to encountered peers.

```
spray(bundle, max_copies):
    copies_remaining = max_copies
    for each encountered peer p:
        if copies_remaining > 0 and p does not have bundle:
            send_copy(bundle, p)
            copies_remaining -= 1
        if copies_remaining == 0:
            break
```

**Copy distribution strategies:**

| Strategy | Description |
|----------|-------------|
| **Binary spray** | Split copies in half at each encounter (L = L/2 to self, L/2 to peer) |
| **Direct spray** | Send one copy per encounter until all copies distributed |
| **Predictive spray** | Send copies to peers with highest PRoPHET predictability to destination |

GhostWire uses **predictive spray** by default (combining Spray-and-Wait with PRoPHET), falling back to binary spray when PRoPHET state is unavailable.

### Phase 2: Wait

After distributing all copies, the source node waits. Any carrier node (including the source) that encounters the destination delivers the bundle.

```
wait(bundle, destination):
    if current_node == destination:
        deliver(bundle)
    for each encountered peer p:
        if p == destination:
            deliver(bundle)
            return
        // Do not forward further (wait phase)
```

### Spray Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| **max_copies** | 3 | Number of copies to distribute (configurable per bundle) |
| **Min copies** | 1 | At least one copy is always attempted |
| **Max copies** | 10 | Upper limit to prevent network flooding |

---

## PRoPHET Epidemic Routing

PRoPHET (Probabilistic Routing Protocol using History of Encounters and Transitivity) provides intelligent forwarding based on delivery predictability.

### Delivery Predictability Update

When node A encounters node B:

**Direct encounter:**
```
P(a,b) = P(a,b)_old + (1 - P(a,b)_old) × P_init
```

Where:
- `P(a,b)_old` is the current delivery predictability from A to B.
- `P_init` is the initial predictability constant (default: 0.75).

**Time decay:**
For each hour since the last encounter with B:
```
P(a,b) = P(a,b)_old × γ^(hours_elapsed)
```

Where `γ` (gamma) is the decay rate (default: 0.98 per hour).

**Transitivity:**
When A encounters B, A updates its predictability to all nodes C that B has encountered:
```
P(a,c)_new = P(a,c)_old + (1 - P(a,c)_old) × P(a,b) × P(b,c) × β
```

Where:
- `P(b,c)` is received from B during the encounter (exchange of predictability tables).
- `β` (beta) is the transitivity weight (default: 0.25).

### Forwarding Decision

When node A encounters node B and has bundles to forward:

```
for each bundle in buffer:
    if P(b, bundle.destination) > P(a, bundle.destination):
        // B has better chance of delivering to destination
        if bundle.copies_distributed < bundle.max_copies:
            send_copy(bundle, b)
            bundle.copies_distributed += 1
            bundle.previous_hops.push(a.peer_id)
```

**Acknowledgment:** When a bundle is delivered to its destination, an acknowledgment is sent back through the reverse path (if connectivity exists) or carried in the DTN for later delivery.

### PRoPHET State Exchange

During an encounter, peers exchange their PRoPHET state:

```rust
pub struct ProphetStateExchange {
    pub peer_id: [u8; 32],
    pub predictabilities: Vec<(PeerId, f64, u64)>,  // (target, predictability, last_update)
    pub timestamp: u64,
    pub signature: [u8; 64],  // Ed25519 signature
}
```

**State validation:**
- Verify the Ed25519 signature.
- Reject entries older than 24 hours (stale data).
- Apply transitivity updates for all shared entries.

---

## SyncBloom

SyncBloom is a probabilistic data structure for bundle deduplication during peer synchronization.

### Specification

| Property | Value |
|----------|-------|
| **Size** | 1024 bits (128 bytes) |
| **Hash functions** | 3 (BLAKE3 with different salts) |
| **False positive rate** | ~2.5% at 50% fill |
| **Capacity** | ~200 bundle IDs before significant false positive rate |

### Construction

```
sync_bloom = zeroed_bit_array(1024)

for each bundle_id in known_bundles:
    for i in 0..3:
        salt = BLAKE3("ghostwire/syncbloom/salt/" || i, output_len=32)
        hash = BLAKE3_keyed(salt, bundle_id, output_len=32)
        index = (hash as u64) % 1024
        sync_bloom[index] = 1
```

### Synchronization Protocol

When two peers encounter each other:

1. **Exchange SyncBlooms:** Each peer sends its SyncBloom to the other.
2. **Check for missing bundles:** Each peer checks its bundle IDs against the received SyncBloom.
3. **Request missing bundles:** If a bundle ID is not in the received SyncBloom (definitely missing), request it.
4. **Transfer bundles:** Missing bundles are transferred.
5. **Update SyncBloom:** Add newly received bundle IDs to the local SyncBloom.

**False positive handling:** If a peer requests a bundle that the other does not have (false positive), the request is silently ignored. The false positive rate is low enough that this does not significantly impact bandwidth.

**SyncBloom reset:** The SyncBloom is reset when it reaches 70% fill to maintain accuracy. A new SyncBloom is created and the old one is discarded.

---

## Buffer Management

### Buffer Configuration

| Property | Value |
|----------|-------|
| **Maximum buffer size** | 50 MB |
| **Eviction policy** | LRU by delivery predictability |
| **Per-bundle limit** | 64 KB (enforced at bundle creation) |
| **Critical bundle reservation** | 10% of buffer (5 MB) reserved for Critical priority |

### Eviction Algorithm

When the buffer exceeds 50 MB:

```
evict():
    while buffer_size > 50MB:
        // Find the bundle with lowest delivery predictability
        victim = argmin_{b in buffer} P(self, b.destination)
        
        // Never evict Critical priority bundles if buffer > 45MB
        if victim.priority == Critical and buffer_size > 45MB:
            victim = argmin_{b in buffer, b.priority != Critical} P(self, b.destination)
        
        remove(victim)
        log("Evicted bundle", bundle_id=victim.bundle_id, reason="buffer_full")
```

**Additional eviction factors (tie-breaking):**
1. Oldest `created_at` timestamp (evict oldest first).
2. Highest `hop_count` (evict bundles that have traveled furthest).
3. Largest payload size (evict largest bundles to free more space).

### Buffer Status

The buffer status is exposed to the AI routing layer for feature extraction:

```rust
pub struct BufferStatus {
    pub total_size: u64,           // Current buffer size in bytes
    pub bundle_count: u64,         // Number of bundles in buffer
    pub oldest_bundle_age: u64,    // Age of oldest bundle in seconds
    pub eviction_count: u64,       // Total bundles evicted (lifetime)
    pub priority_distribution: HashMap<BundlePriority, u64>,  // Count per priority
}
```

---

## Bluetooth Sync Handshake

Bluetooth Low Energy (BLE) provides opportunistic sync between nearby devices.

### BLE Service

| Property | Value |
|----------|-------|
| **Service UUID** | `0x4757` ("GW" for GhostWire) |
| **Characteristic UUID** | `0x4758` (bundle exchange) |
| **MTU** | 512 bytes (negotiated up to 4096 bytes) |
| **Advertising interval** | 1 second (when screen is on) |

### Sync Handshake

```
1. Device A advertises GhostWire BLE service
2. Device B scans and discovers A
3. B connects to A
4. MTU negotiation: both devices agree on maximum transfer unit
5. SyncBloom exchange: both devices send their SyncBlooms
6. Bundle request: each device requests bundles it's missing
7. Bundle transfer: bundles are transferred in priority order (Critical > Expedited > Normal > Bulk)
8. Acknowledgment: receiving device acknowledges each bundle
9. Disconnect: connection is closed after transfer or timeout (30 seconds)
```

**Power management:**
- BLE advertising is disabled when battery < 10% (user-configurable).
- Sync is limited to 30 seconds per encounter to conserve battery.
- Large bundles (>4 KB) are deferred to WiFi/cellular when available.

**Security:**
- All bundles transferred over BLE are encrypted (XChaCha20-Poly1305).
- BLE connection is authenticated using the peer's Ed25519 public key.
- SyncBloom does not leak bundle contents (only bundle IDs, which are BLAKE3 hashes).

---

## Security Properties

| Property | Mechanism |
|----------|-----------|
| **Bundle confidentiality** | XChaCha20-Poly1305 encryption (payload is already encrypted at application layer) |
| **Bundle integrity** | BLAKE3 bundle ID; tampered bundles fail ID verification |
| **Replay resistance** | Unique bundle IDs; expired bundles discarded |
| **DoS resistance** | Buffer size limits (50 MB); per-bundle size limits (64 KB) |
| **Sybil resistance** | PRoPHET state signed with Ed25519; fake peers cannot forge valid state |
| **Privacy** | SyncBloom only reveals bundle IDs (hashes), not contents |

### Threat Model

| Adversary | Capability | Protection |
|-----------|-----------|------------|
| **Bundle injection** | Injects fake bundles into the network | Bundle ID verification; signature validation on PRoPHET state |
| **Buffer flooding** | Sends many bundles to exhaust buffer | Per-peer rate limiting; buffer eviction policy |
| **SyncBloom analysis** | Analyzes SyncBloom to infer communication patterns | Bundle IDs are BLAKE3 hashes; without the source/destination, inference is limited |
| **BLE eavesdropping** | Monitors BLE sync traffic | All bundles encrypted; BLE authenticated |
| **PRoPHET manipulation** | Reports fake predictability to attract traffic | Ed25519 signatures on PRoPHET state; Web of Trust scores factor into forwarding decisions |

---

## Reference Implementation

See the following modules in the GhostWire reference implementation:

| Module | Path |
|--------|------|
| DtnBundle | `src/core/dtn/bundle.rs` |
| Spray-and-Wait | `src/core/dtn/spray_and_wait.rs` |
| PRoPHET routing | `src/core/dtn/prophet.rs` |
| SyncBloom | `src/core/dtn/sync_bloom.rs` |
| Buffer management | `src/core/dtn/buffer.rs` |
| BLE sync | `src/core/dtn/ble_sync.rs` |
| Bundle fragmentation | `src/core/dtn/fragmentation.rs` |
