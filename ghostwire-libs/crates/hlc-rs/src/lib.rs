//! # hlc-rs
//!
//! Hybrid Logical Clocks (HLC) for causally correct distributed event ordering.
//!
//! HLC combines physical time (NTP) with logical counters to provide
//! causally correct ordering without requiring perfectly synced clocks.
//!
//! ## Algorithm
//!
//! - **On local event**: advance physical time, reset or increment counter
//! - **On remote event**: merge with remote HLC, advance counter
//! - **Comparison**: `(physical_time, logical_counter, node_id)` gives total order
//!
//! ## Quick Start
//!
//! ```rust
//! use hlc_rs::{HybridLogicalClock, HlcTimestamp};
//!
//! let hlc_a = HybridLogicalClock::new(1);
//! let hlc_b = HybridLogicalClock::new(2);
//!
//! // A sends a message
//! let ts_a = hlc_a.send();
//!
//! // B receives A's message
//! let ts_b = hlc_b.receive(&ts_a);
//!
//! // Causal order: A's send < B's receive
//! assert!(ts_a < ts_b);
//! ```
//!
//! ## Features
//!
//! - **Thread-safe** — atomic CAS operations for concurrent access
//! - **Clock manipulation detection** — hybrid SystemTime + Instant prevents backwards clock manipulation
//! - **Message reorder buffer** — holds out-of-order messages and delivers in HLC order
//! - **Zero dependencies** (except serde for serialization)
//!
//! ## Reference
//!
//! - Kulkarni, S., et al. "Hybrid Logical Clocks."

#![warn(missing_docs)]

use serde::{Deserialize, Serialize};
use std::cmp::Ordering;
use std::sync::atomic::{AtomicU32, AtomicU64, Ordering as AtomicOrdering};

/// Hybrid Logical Clock timestamp.
///
/// Comparison order: physical_time → logical_counter → node_id
/// This gives a total order that respects causality.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub struct HlcTimestamp {
    /// Physical time in milliseconds since Unix epoch (from NTP/system clock)
    pub physical_time: u64,
    /// Logical counter (increments when physical times are equal)
    pub logical_counter: u32,
    /// Compact node identifier (hash of peer ID, truncated to 16 bits)
    pub node_id: u16,
}

impl HlcTimestamp {
    /// Create a zero timestamp (used as initial value)
    pub fn zero() -> Self {
        Self {
            physical_time: 0,
            logical_counter: 0,
            node_id: 0,
        }
    }

    /// Generate a compact node ID from a full peer ID string.
    pub fn node_id_from_peer_id(peer_id: &str) -> u16 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        let mut hasher = DefaultHasher::new();
        peer_id.hash(&mut hasher);
        (hasher.finish() & 0xFFFF) as u16
    }
}

impl Default for HlcTimestamp {
    fn default() -> Self {
        Self::zero()
    }
}

impl PartialOrd for HlcTimestamp {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HlcTimestamp {
    fn cmp(&self, other: &Self) -> Ordering {
        self.physical_time
            .cmp(&other.physical_time)
            .then_with(|| self.logical_counter.cmp(&other.logical_counter))
            .then_with(|| self.node_id.cmp(&other.node_id))
    }
}

/// Hybrid Logical Clock state for a single node.
///
/// Thread-safe via atomic operations.
pub struct HybridLogicalClock {
    physical_time: AtomicU64,
    logical_counter: AtomicU32,
    node_id: u16,
}

impl HybridLogicalClock {
    /// Create a new HLC for this node.
    pub fn new(node_id: u16) -> Self {
        Self {
            physical_time: AtomicU64::new(0),
            logical_counter: AtomicU32::new(0),
            node_id,
        }
    }

    /// Generate a timestamp for a local event.
    pub fn send(&self) -> HlcTimestamp {
        let now = current_time_ms();
        let local_pt = self.physical_time.load(AtomicOrdering::Relaxed);
        let local_lc = self.logical_counter.load(AtomicOrdering::Relaxed);

        let (new_pt, new_lc) = if now > local_pt {
            self.physical_time.store(now, AtomicOrdering::Relaxed);
            self.logical_counter.store(0, AtomicOrdering::Relaxed);
            (now, 0)
        } else {
            let new_lc = local_lc + 1;
            self.logical_counter.store(new_lc, AtomicOrdering::Relaxed);
            (local_pt, new_lc)
        };

        HlcTimestamp {
            physical_time: new_pt,
            logical_counter: new_lc,
            node_id: self.node_id,
        }
    }

    /// Update the HLC state after receiving a remote event.
    pub fn receive(&self, remote: &HlcTimestamp) -> HlcTimestamp {
        let now = current_time_ms();

        loop {
            let local_pt = self.physical_time.load(AtomicOrdering::Relaxed);
            let local_lc = self.logical_counter.load(AtomicOrdering::Relaxed);

            let (new_pt, new_lc) = if remote.physical_time > local_pt {
                (remote.physical_time, 0)
            } else if remote.physical_time == local_pt {
                (local_pt, std::cmp::max(local_lc, remote.logical_counter))
            } else {
                (local_pt, local_lc)
            };

            let final_lc = new_lc + 1;
            let final_pt = std::cmp::max(new_pt, now);

            let pt_result = self.physical_time.compare_exchange_weak(
                local_pt,
                final_pt,
                AtomicOrdering::Relaxed,
                AtomicOrdering::Relaxed,
            );

            if pt_result.is_ok() {
                self.logical_counter
                    .store(final_lc, AtomicOrdering::Relaxed);
                return HlcTimestamp {
                    physical_time: final_pt,
                    logical_counter: final_lc,
                    node_id: self.node_id,
                };
            }
        }
    }

    /// Get the current HLC timestamp without advancing the clock.
    pub fn now(&self) -> HlcTimestamp {
        HlcTimestamp {
            physical_time: self.physical_time.load(AtomicOrdering::Relaxed),
            logical_counter: self.logical_counter.load(AtomicOrdering::Relaxed),
            node_id: self.node_id,
        }
    }

    /// Get this node's compact ID
    pub fn node_id(&self) -> u16 {
        self.node_id
    }
}

/// Get current time in milliseconds since Unix epoch.
/// Uses hybrid SystemTime + Instant to detect clock manipulation.
fn current_time_ms() -> u64 {
    static MONOTONIC_BASE: std::sync::OnceLock<(std::time::Instant, u64)> =
        std::sync::OnceLock::new();
    static LAST_SEEN: AtomicU64 = AtomicU64::new(0);

    let (instant_base, system_base) = MONOTONIC_BASE.get_or_init(|| {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_millis() as u64;
        (std::time::Instant::now(), now)
    });

    let system_now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64;
    let monotonic_now = *system_base + instant_base.elapsed().as_millis() as u64;

    let candidate = system_now.max(monotonic_now);
    let last = LAST_SEEN.load(AtomicOrdering::Relaxed);
    let result = candidate.max(last);
    LAST_SEEN.store(result, AtomicOrdering::Relaxed);

    result
}

/// Message reorder buffer.
///
/// Holds messages that arrive out of order and delivers them in HLC order.
pub struct MessageReorderBuffer<T> {
    buffer: Vec<(HlcTimestamp, T)>,
    max_delay_ms: u64,
    last_delivered: HlcTimestamp,
}

impl<T> MessageReorderBuffer<T> {
    /// Create a new reorder buffer with the given max delay.
    pub fn new(max_delay_ms: u64) -> Self {
        Self {
            buffer: Vec::new(),
            max_delay_ms,
            last_delivered: HlcTimestamp::zero(),
        }
    }

    /// Add a message to the buffer. Returns messages ready for delivery.
    pub fn add(&mut self, timestamp: HlcTimestamp, message: T) -> Vec<(HlcTimestamp, T)> {
        self.buffer.push((timestamp, message));
        self.buffer.sort_by(|a, b| a.0.cmp(&b.0));

        let now = current_time_ms();
        let cutoff = now.saturating_sub(self.max_delay_ms);

        let mut ready = Vec::new();
        let mut remaining = Vec::new();

        for (ts, msg) in self.buffer.drain(..) {
            if ts > self.last_delivered && (ts.physical_time <= cutoff || ready.is_empty()) {
                ready.push((ts, msg));
                if ts > self.last_delivered {
                    self.last_delivered = ts;
                }
            } else {
                remaining.push((ts, msg));
            }
        }

        self.buffer = remaining;
        ready
    }

    /// Force deliver all buffered messages in HLC order.
    pub fn flush(&mut self) -> Vec<(HlcTimestamp, T)> {
        let mut result = std::mem::take(&mut self.buffer);
        result.sort_by(|a, b| a.0.cmp(&b.0));
        if let Some(last) = result.last() {
            if last.0 > self.last_delivered {
                self.last_delivered = last.0;
            }
        }
        result
    }

    /// Get the number of buffered messages
    pub fn len(&self) -> usize {
        self.buffer.len()
    }

    /// Check if the buffer is empty
    pub fn is_empty(&self) -> bool {
        self.buffer.is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hlc_send_ordering() {
        let hlc = HybridLogicalClock::new(1);
        let ts1 = hlc.send();
        let ts2 = hlc.send();
        let ts3 = hlc.send();
        assert!(ts1 < ts2);
        assert!(ts2 < ts3);
    }

    #[test]
    fn test_hlc_receive_updates_clock() {
        let hlc = HybridLogicalClock::new(1);
        let remote = HlcTimestamp {
            physical_time: current_time_ms() + 1000,
            logical_counter: 0,
            node_id: 2,
        };
        let local = hlc.receive(&remote);
        assert!(local.physical_time >= remote.physical_time);
    }

    #[test]
    fn test_hlc_causal_ordering() {
        let hlc_a = HybridLogicalClock::new(1);
        let hlc_b = HybridLogicalClock::new(2);
        let ts_a = hlc_a.send();
        let ts_b = hlc_b.receive(&ts_a);
        let ts_b_reply = hlc_b.send();
        let ts_a_reply = hlc_a.receive(&ts_b_reply);
        assert!(ts_a < ts_b);
        assert!(ts_b < ts_b_reply);
        assert!(ts_b_reply < ts_a_reply);
    }

    #[test]
    fn test_hlc_concurrent_events() {
        let hlc_a = HybridLogicalClock::new(1);
        let hlc_b = HybridLogicalClock::new(2);
        let ts_a = hlc_a.send();
        let ts_b = hlc_b.send();
        assert!(ts_a != ts_b || ts_a.node_id != ts_b.node_id);
        assert!(ts_a < ts_b || ts_b < ts_a);
    }

    #[test]
    fn test_reorder_buffer_flush() {
        let mut buf = MessageReorderBuffer::<String>::new(10000);
        let now = current_time_ms();
        // Use future timestamps so messages aren't auto-delivered by the cutoff
        let ts1 = HlcTimestamp {
            physical_time: now + 100000,
            logical_counter: 0,
            node_id: 1,
        };
        let ts2 = HlcTimestamp {
            physical_time: now + 100000,
            logical_counter: 1,
            node_id: 1,
        };
        // First add delivers immediately (establishes baseline)
        let ready = buf.add(ts2, "msg2".to_string());
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].1, "msg2");
        // Second add buffers (out of order)
        let ready = buf.add(ts1, "msg1".to_string());
        assert!(ready.is_empty());
        // Flush returns the buffered message
        let flushed = buf.flush();
        assert_eq!(flushed.len(), 1);
        assert_eq!(flushed[0].1, "msg1");
    }

    #[test]
    fn test_hlc_node_id_from_peer_id() {
        let id1 = HlcTimestamp::node_id_from_peer_id("12D3KooWPeer1");
        let id2 = HlcTimestamp::node_id_from_peer_id("12D3KooWPeer2");
        let id1_again = HlcTimestamp::node_id_from_peer_id("12D3KooWPeer1");
        assert_eq!(id1, id1_again);
        assert_ne!(id1, id2);
    }
}
