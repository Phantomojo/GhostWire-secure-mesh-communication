//! hlc.rs — Hybrid Logical Clocks for distributed event ordering
//!
//! HLC combines physical time (NTP) with logical counters to provide
//! causally correct ordering without requiring perfectly synced clocks.
//!
//! Algorithm:
//!   - On local event: advance physical time, reset or increment counter
//!   - On remote event: merge with remote HLC, advance counter
//!   - Comparison: (physical_time, logical_counter, node_id) gives total order
//!
//! References:
//!   - "Hybrid Logical Clocks" by Kulkarni et al.
//!   - Signal Protocol message numbering (for chain-level ordering)
//!   - Matrix origin_server_ts (for human-readable timestamps)

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
    /// Uses a simple hash to get a 16-bit identifier.
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
/// Thread-safe via atomic operations. Each node maintains its own HLC state
/// and updates it on every local and remote event.
pub struct HybridLogicalClock {
    /// Current physical time (ms since epoch)
    physical_time: AtomicU64,
    /// Current logical counter
    logical_counter: AtomicU32,
    /// This node's compact ID
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

    /// Generate a timestamp for a local event (e.g., sending a message).
    ///
    /// Algorithm:
    ///   now = current physical time
    ///   if now > local.physical_time:
    ///       local.physical_time = now
    ///       local.logical_counter = 0
    ///   else:
    ///       local.logical_counter += 1
    ///   return {local.physical_time, local.logical_counter, local.node_id}
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
    ///
    /// Algorithm:
    ///   now = current physical time
    ///   if remote.physical_time > local.physical_time:
    ///       local.physical_time = remote.physical_time
    ///       local.logical_counter = 0
    ///   elif remote.physical_time == local.physical_time:
    ///       local.logical_counter = max(local.logical_counter, remote.logical_counter)
    ///   local.logical_counter += 1
    ///   local.physical_time = max(local.physical_time, now)
    ///   return {local.physical_time, local.logical_counter, local.node_id}
    pub fn receive(&self, remote: &HlcTimestamp) -> HlcTimestamp {
        let now = current_time_ms();

        // Use a loop for atomic compare-and-swap to ensure consistency
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

            // Try to atomically update both fields
            let pt_result = self.physical_time.compare_exchange_weak(
                local_pt,
                final_pt,
                AtomicOrdering::Relaxed,
                AtomicOrdering::Relaxed,
            );

            if pt_result.is_ok() {
                self.logical_counter.store(final_lc, AtomicOrdering::Relaxed);
                return HlcTimestamp {
                    physical_time: final_pt,
                    logical_counter: final_lc,
                    node_id: self.node_id,
                };
            }
            // If CAS failed, retry with new values
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
fn current_time_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// Message reorder buffer.
///
/// Holds messages that arrive out of order and delivers them in HLC order.
/// Messages are held for up to `max_delay_ms` milliseconds to allow
/// earlier messages to arrive before delivering.
pub struct MessageReorderBuffer<T> {
    /// Buffered messages waiting to be delivered
    buffer: Vec<(HlcTimestamp, T)>,
    /// Maximum delay in milliseconds before delivering a message
    max_delay_ms: u64,
    /// The HLC timestamp of the last delivered message
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

    /// Add a message to the buffer. Returns messages that are ready for delivery
    /// (sorted by HLC timestamp).
    pub fn add(&mut self, timestamp: HlcTimestamp, message: T) -> Vec<(HlcTimestamp, T)> {
        self.buffer.push((timestamp, message));

        // Sort buffer by HLC timestamp
        self.buffer.sort_by(|a, b| a.0.cmp(&b.0));

        // Find messages that are ready for delivery
        // A message is ready if:
        // 1. It's after the last delivered message
        // 2. Its physical time is older than (now - max_delay_ms)
        //    OR it's the next expected message in order
        let now = current_time_ms();
        let cutoff = now.saturating_sub(self.max_delay_ms);

        let mut ready = Vec::new();
        let mut remaining = Vec::new();

        for (ts, msg) in self.buffer.drain(..) {
            if ts > self.last_delivered && (ts.physical_time <= cutoff || ready.is_empty()) {
                ready.push((ts, msg));
                // Update last_delivered to the latest delivered timestamp
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
    /// Use this when shutting down or when you can't wait any longer.
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

        // Simulate receiving a message from the future
        let remote = HlcTimestamp {
            physical_time: current_time_ms() + 1000,
            logical_counter: 0,
            node_id: 2,
        };

        let local = hlc.receive(&remote);

        // Local clock should have advanced to at least the remote's physical time
        assert!(local.physical_time >= remote.physical_time);
    }

    #[test]
    fn test_hlc_causal_ordering() {
        let hlc_a = HybridLogicalClock::new(1);
        let hlc_b = HybridLogicalClock::new(2);

        // A sends a message
        let ts_a = hlc_a.send();

        // B receives A's message
        let ts_b = hlc_b.receive(&ts_a);

        // B sends a reply
        let ts_b_reply = hlc_b.send();

        // A receives B's reply
        let ts_a_reply = hlc_a.receive(&ts_b_reply);

        // Causal order: A's send < B's receive < B's send < A's receive
        assert!(ts_a < ts_b);
        assert!(ts_b < ts_b_reply);
        assert!(ts_b_reply < ts_a_reply);
    }

    #[test]
    fn test_hlc_concurrent_events() {
        let hlc_a = HybridLogicalClock::new(1);
        let hlc_b = HybridLogicalClock::new(2);

        // Both send at roughly the same time
        let ts_a = hlc_a.send();
        let ts_b = hlc_b.send();

        // They should be ordered (total order), even if concurrent
        assert!(ts_a != ts_b || ts_a.node_id != ts_b.node_id);
        // One must be less than the other (total order via node_id tiebreaker)
        assert!(ts_a < ts_b || ts_b < ts_a);
    }

    #[test]
    fn test_reorder_buffer_delivers_in_order() {
        let mut buf = MessageReorderBuffer::<String>::new(100);

        // Add messages out of order (message 3 arrives before 1 and 2)
        let ts3 = HlcTimestamp {
            physical_time: 1000,
            logical_counter: 2,
            node_id: 1,
        };
        let ts1 = HlcTimestamp {
            physical_time: 1000,
            logical_counter: 0,
            node_id: 1,
        };
        let ts2 = HlcTimestamp {
            physical_time: 1000,
            logical_counter: 1,
            node_id: 1,
        };

        // Add message 3 first (out of order)
        let ready = buf.add(ts3, "msg3".to_string());
        assert!(ready.is_empty()); // Should buffer, not deliver yet

        // Add message 1
        let ready = buf.add(ts1, "msg1".to_string());
        assert!(!ready.is_empty()); // Should deliver msg1

        // Add message 2
        let ready = buf.add(ts2, "msg2".to_string());
        assert!(!ready.is_empty()); // Should deliver msg2 (and possibly msg3)
    }

    #[test]
    fn test_reorder_buffer_flush() {
        let mut buf = MessageReorderBuffer::<String>::new(10000);

        let ts1 = HlcTimestamp {
            physical_time: 1000,
            logical_counter: 0,
            node_id: 1,
        };
        let ts2 = HlcTimestamp {
            physical_time: 1000,
            logical_counter: 1,
            node_id: 1,
        };

        buf.add(ts2, "msg2".to_string());
        buf.add(ts1, "msg1".to_string());

        let flushed = buf.flush();
        assert_eq!(flushed.len(), 2);
        assert_eq!(flushed[0].1, "msg1");
        assert_eq!(flushed[1].1, "msg2");
    }

    #[test]
    fn test_hlc_node_id_from_peer_id() {
        let id1 = HlcTimestamp::node_id_from_peer_id("12D3KooWPeer1");
        let id2 = HlcTimestamp::node_id_from_peer_id("12D3KooWPeer2");
        let id1_again = HlcTimestamp::node_id_from_peer_id("12D3KooWPeer1");

        assert_eq!(id1, id1_again); // Same peer → same node ID
        assert_ne!(id1, id2); // Different peers → different node IDs (with high probability)
    }
}
