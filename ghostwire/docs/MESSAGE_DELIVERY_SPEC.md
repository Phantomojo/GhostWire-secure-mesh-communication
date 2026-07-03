# GhostWire Message Delivery Specification

> **Version**: 1.0 | **Status**: Draft | **Last Updated**: 2026-07-01
> **Phase**: v1.0 Phase 1 (Message Reliability)

---

## 1. Overview

This spec defines message delivery guarantees: acknowledgment, ordering, retry, deduplication, and chunking.

---

## 2. Message Lifecycle

```
Sender                Network              Receiver
  │                     │                     │
  │─── Send Message ───▶│                     │
  │                     │─── Forward ────────▶│
  │                     │                     │
  │◀────── ACK ─────────│◀────── Process ─────│
  │                     │                     │
  │  [Retry if no ACK]  │                     │
```

---

## 3. Acknowledgment (ACK)

### 3.1 ACK Format

```rust
pub struct MessageAck {
    pub message_id: [u8; 32],    // SHA-256 of original message
    pub timestamp: u64,           // Unix timestamp (ms)
    pub status: AckStatus,        // Delivered | Failed | Pending
    pub signature: Ed25519Signature,
}

pub enum AckStatus {
    Delivered,
    Failed(AckFailureReason),
    Pending,
}
```

### 3.2 ACK Behavior

| Scenario | Action |
|----------|--------|
| ACK received | Mark message as delivered |
| ACK timeout (5s) | Retry with backoff |
| Max retries (5) | Mark as failed, notify sender |
| Duplicate ACK | Ignore (idempotent) |

### 3.3 Retry Policy

```rust
pub struct RetryPolicy {
    pub max_retries: u32,         // Default: 5
    pub base_delay_ms: u64,       // Default: 1000
    pub max_delay_ms: u64,        // Default: 30000
    pub backoff_multiplier: f64,  // Default: 2.0
    pub jitter: bool,             // Default: true
}
```

**Backoff Formula:**
```
delay = min(base_delay * multiplier^attempt + jitter, max_delay)
```

**Jitter:**
```rust
let jitter = OsRng.gen_range(0.0..0.1) * delay;
delay += jitter;
```

---

## 4. Message Ordering

### 4.1 Causal Ordering (Vector Clocks)

```rust
pub struct VectorClock {
    pub clock: HashMap<NodeId, u64>,
}

impl VectorClock {
    pub fn increment(&mut self, node: NodeId) {
        *self.clock.entry(node).or_insert(0) += 1;
    }

    pub fn merge(&mut self, other: &VectorClock) {
        for (node, counter) in &other.clock {
            let entry = self.clock.entry(*node).or_insert(0);
            *entry = (*entry).max(*counter);
        }
    }

    pub fn happens_before(&self, other: &VectorClock) -> bool {
        self.clock.iter().all(|(node, &count)| {
            other.clock.get(node).map_or(false, |&other_count| count <= other_count)
        }) && self != other
    }
}
```

### 4.2 Delivery Rules

| Rule | Description |
|------|-------------|
| **Causal delivery** | If A happened-before B, deliver A before B |
| **Concurrent messages** | Deliver in any stable order |
| **Gap detection** | Buffer messages if gap detected |
| **Gap timeout** | Deliver after 2s if gap not filled |

---

## 5. Deduplication

### 5.1 Bloom Filter

```rust
pub struct DeduplicationFilter {
    filter: BloomFilter,          // 1% false positive rate
    seen_messages: LruCache<[u8; 32], u64>,  // message_id → timestamp
}

impl DeduplicationFilter {
    pub fn is_duplicate(&mut self, message_id: &[u8; 32]) -> bool {
        if self.seen_messages.contains(message_id) {
            return true;
        }
        if self.filter.check(message_id) {
            // Possible duplicate — check LRU
            return self.seen_messages.contains(message_id);
        }
        // New message
        self.filter.insert(message_id);
        self.seen_messages.put(*message_id, now());
        false
    }
}
```

### 5.2 Configuration

| Parameter | Value | Rationale |
|-----------|-------|-----------|
| Bloom filter size | 1MB | ~1M messages at 1% FP |
| LRU cache size | 10,000 messages | ~24h of typical usage |
| TTL | 24 hours | Messages older than 24h are unique |

---

## 6. Message TTL

### 6.1 TTL Format

```rust
pub struct MessageTTL {
    pub created_at: u64,          // Unix timestamp (ms)
    pub ttl_ms: u64,              // Time-to-live in milliseconds
    pub hop_count: u8,            // Current hop count
    pub max_hops: u8,             // Maximum hops (default: 10)
}
```

### 6.2 TTL Rules

| Condition | Action |
|-----------|--------|
| `now > created_at + ttl_ms` | Drop message |
| `hop_count >= max_hops` | Drop message |
| `ttl_ms == 0` | Immediate delivery, no forwarding |
| `ttl_ms == u64::MAX` | Never expires |

### 6.3 Default TTLs

| Message Type | TTL | Max Hops |
|-------------|-----|----------|
| Text message | 24 hours | 10 |
| File transfer | 1 hour | 5 |
| System message | 1 hour | 3 |
| Discovery | 30 seconds | 2 |

---

## 7. Message Chunking

### 7.1 Chunk Format

```rust
pub struct MessageChunk {
    pub message_id: [u8; 32],     // Original message ID
    pub chunk_index: u32,         // 0-based index
    pub total_chunks: u32,        // Total number of chunks
    pub chunk_data: Vec<u8>,      // Chunk payload (max 64KB)
    pub checksum: [u8; 32],       // SHA-256 of chunk
}
```

### 7.2 Chunking Rules

| Rule | Value |
|------|-------|
| Max chunk size | 64 KB |
| Max message size | 10 MB (160 chunks) |
| Chunk timeout | 30 seconds per chunk |
| Missing chunk action | Request retransmission |

### 7.3 Reassembly

```rust
pub struct MessageReassembler {
    chunks: HashMap<u32, MessageChunk>,  // index → chunk
    total_chunks: u32,
    received_at: u64,
}

impl MessageReassembler {
    pub fn is_complete(&self) -> bool {
        self.chunks.len() == self.total_chunks as usize
    }

    pub fn reassemble(&self) -> Option<Vec<u8>> {
        if !self.is_complete() {
            return None;
        }
        let mut data = Vec::new();
        for i in 0..self.total_chunks {
            data.extend_from_slice(&self.chunks[&i].chunk_data);
        }
        Some(data)
    }
}
```

---

## 8. Error Handling

| Error | Recovery |
|-------|----------|
| ACK timeout | Retry with backoff |
| Chunk missing | Request retransmission |
| Reassembly timeout (60s) | Drop partial message, notify |
| Checksum mismatch | Request retransmission |
| Duplicate detected | Ignore silently |

---

## 9. Performance Targets

| Metric | Target |
|--------|--------|
| ACK latency | <100ms (local), <500ms (relay) |
| Delivery guarantee | 99.9% (within 5 retries) |
| Dedup false positive rate | <1% |
| Chunk reassembly time | <1 second |
| Max message size | 10 MB |

---

## 10. References

- [GhostWire Threat Model](./THREAT_MODEL.md)
- [GhostWire Layer Audit](~/Downloads/GhostWire_Layer_Audit_v1.md)
- [RFC 3161 — Internet X.509 PKI Timestamp Protocol](https://datatracker.ietf.org/doc/html/rfc3161)
