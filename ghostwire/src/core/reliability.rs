// Copyright (C) 2025-2026 Michael Irungu
// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Reliability Layer — ACK, Retry, TTL Enforcement
//!
//! Provides message delivery guarantees on top of the mesh transport.
//!
//! ## Components
//!
//! - **AckTracker**: Tracks pending message ACKs with configurable timeout
//! - **RetryQueue**: Exponential backoff retry for unACKed messages
//! - **TTL enforcement**: Drop messages that exceed hop limit or expiry
//!
//! ## Design
//!
//! Uses existing `DashMap` (concurrent) and `Instant`-based timing.
//! No new dependencies. No async runtime dependency — pure data structures.
//!
//! ```text
//! Message sent → AckTracker.track(id) → RetryQueue.enqueue(id)
//!                     ↓
//!              On ACK received → AckTracker.acknowledge(id) → RetryQueue.remove(id)
//!                     ↓ (timeout)
//!              RetryQueue.should_retry(id) → true → retransmit
//!                     ↓ (max retries exceeded)
//!              Message dropped, event emitted
//! ```

use dashmap::DashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};
use uuid::Uuid;

// ── Constants ─────────────────────────────────────────────────────────────────

/// Default ACK timeout before first retry attempt
pub const DEFAULT_ACK_TIMEOUT: Duration = Duration::from_secs(5);

/// Default initial retry delay (exponential backoff starts here)
pub const DEFAULT_INITIAL_RETRY_DELAY: Duration = Duration::from_secs(1);

/// Default maximum retry delay (backoff caps at this)
pub const DEFAULT_MAX_RETRY_DELAY: Duration = Duration::from_secs(30);

/// Default maximum retry attempts before dropping
pub const DEFAULT_MAX_RETRIES: u8 = 5;

/// Default backoff multiplier
pub const DEFAULT_BACKOFF_MULTIPLIER: f64 = 2.0;

/// Maximum hop count before message is dropped (prevents infinite relay loops)
pub const MAX_HOPS: u8 = 8;

/// Default message TTL in seconds (1 hour)
pub const DEFAULT_MESSAGE_TTL: u64 = 3600;

/// Maximum number of pending ACKs tracked simultaneously
pub const DEFAULT_MAX_PENDING_ACKS: usize = 1000;

pub fn current_unix_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

// ── Configuration ─────────────────────────────────────────────────────────────

/// Configuration for the reliability layer
#[derive(Debug, Clone)]
pub struct ReliabilityConfig {
    /// How long to wait for an ACK before first retry
    pub ack_timeout: Duration,
    /// Initial delay between retries
    pub initial_retry_delay: Duration,
    /// Maximum delay between retries (backoff cap)
    pub max_retry_delay: Duration,
    /// Maximum number of retry attempts
    pub max_retries: u8,
    /// Exponential backoff multiplier
    pub backoff_multiplier: f64,
    /// Maximum pending ACKs (prevents memory exhaustion)
    pub max_pending_acks: usize,
}

impl Default for ReliabilityConfig {
    fn default() -> Self {
        Self {
            ack_timeout: DEFAULT_ACK_TIMEOUT,
            initial_retry_delay: DEFAULT_INITIAL_RETRY_DELAY,
            max_retry_delay: DEFAULT_MAX_RETRY_DELAY,
            max_retries: DEFAULT_MAX_RETRIES,
            backoff_multiplier: DEFAULT_BACKOFF_MULTIPLIER,
            max_pending_acks: DEFAULT_MAX_PENDING_ACKS,
        }
    }
}

// ── ACK Tracker ───────────────────────────────────────────────────────────────

/// Tracks messages awaiting acknowledgment
struct PendingAck {
    sent_at: Instant,
    attempt: u8,
    next_retry: Instant,
    current_delay: Duration,
}

/// Tracks pending ACKs with configurable timeout.
///
/// Thread-safe via DashMap. No async required.
pub struct AckTracker {
    pending: DashMap<Uuid, PendingAck>,
    config: ReliabilityConfig,
    /// Total ACKs received (for metrics)
    acks_received: AtomicUsize,
    /// Total messages timed out (for metrics)
    timeouts: AtomicUsize,
}

impl AckTracker {
    pub fn new(config: ReliabilityConfig) -> Self {
        Self {
            pending: DashMap::with_capacity(config.max_pending_acks.min(256)),
            config,
            acks_received: AtomicUsize::new(0),
            timeouts: AtomicUsize::new(0),
        }
    }

    /// Start tracking a message for ACK
    pub fn track(&self, message_id: Uuid) {
        // Don't exceed max pending — drop oldest if needed
        if self.pending.len() >= self.config.max_pending_acks {
            // Find and remove the oldest entry
            if let Some(oldest_id) = self
                .pending
                .iter()
                .min_by_key(|entry| entry.sent_at)
                .map(|entry| *entry.key())
            {
                self.pending.remove(&oldest_id);
            }
        }

        let now = Instant::now();
        self.pending.insert(
            message_id,
            PendingAck {
                sent_at: now,
                attempt: 0,
                next_retry: now + self.config.ack_timeout,
                current_delay: self.config.initial_retry_delay,
            },
        );
    }

    /// Acknowledge a message — returns true if it was pending
    pub fn acknowledge(&self, message_id: &Uuid) -> bool {
        let removed = self.pending.remove(message_id).is_some();
        if removed {
            self.acks_received.fetch_add(1, Ordering::Relaxed);
        }
        removed
    }

    /// Stop tracking a message without recording a delivery acknowledgement.
    ///
    /// Used when a local policy rejects dispatch after tracking began. This
    /// prevents a privacy-policy rejection from later becoming a retransmit.
    pub fn cancel(&self, message_id: &Uuid) -> bool {
        self.pending.remove(message_id).is_some()
    }

    /// Check which messages have timed out and need retry
    ///
    /// Returns message IDs that are past their next_retry time.
    /// Automatically increments attempt count and computes next backoff.
    pub fn timed_out(&self) -> Vec<Uuid> {
        let now = Instant::now();
        let mut to_retry = Vec::new();

        self.pending.retain(|&id, entry| {
            if now >= entry.next_retry {
                entry.attempt += 1;

                if entry.attempt > self.config.max_retries {
                    // Max retries exceeded — drop the message
                    self.timeouts.fetch_add(1, Ordering::Relaxed);
                    return false;
                }

                // Compute next backoff: delay * multiplier, capped at max
                let new_delay = Duration::from_secs_f64(
                    (entry.current_delay.as_secs_f64() * self.config.backoff_multiplier)
                        .min(self.config.max_retry_delay.as_secs_f64()),
                );
                entry.current_delay = new_delay;
                entry.next_retry = now + new_delay;

                to_retry.push(id);
            }
            true // Keep in pending
        });

        to_retry
    }

    /// Check if a specific message is still pending
    pub fn is_pending(&self, message_id: &Uuid) -> bool {
        self.pending.contains_key(message_id)
    }

    /// Get number of pending ACKs
    pub fn pending_count(&self) -> usize {
        self.pending.len()
    }

    /// Get metrics
    pub fn stats(&self) -> AckTrackerStats {
        AckTrackerStats {
            pending: self.pending.len(),
            acks_received: self.acks_received.load(Ordering::Relaxed),
            timeouts: self.timeouts.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone)]
pub struct AckTrackerStats {
    pub pending: usize,
    pub acks_received: usize,
    pub timeouts: usize,
}

// ── Retry Queue ───────────────────────────────────────────────────────────────

/// Messages pending retry, keyed by message ID
struct RetryEntry {
    attempt: u8,
    next_retry: Instant,
    current_delay: Duration,
}

/// Exponential backoff retry queue for unACKed messages.
///
/// Works in tandem with AckTracker. When AckTracker reports timeout,
/// the retry queue determines if/when to retransmit.
pub struct RetryQueue {
    entries: DashMap<Uuid, RetryEntry>,
    config: ReliabilityConfig,
    /// Total retried messages
    retried: AtomicUsize,
    /// Total dropped (exceeded max retries)
    dropped: AtomicUsize,
}

impl RetryQueue {
    pub fn new(config: ReliabilityConfig) -> Self {
        Self {
            entries: DashMap::new(),
            config,
            retried: AtomicUsize::new(0),
            dropped: AtomicUsize::new(0),
        }
    }

    /// Enqueue a message for retry tracking
    pub fn enqueue(&self, message_id: Uuid) {
        let now = Instant::now();
        self.entries.insert(
            message_id,
            RetryEntry {
                attempt: 0,
                next_retry: now + self.config.ack_timeout,
                current_delay: self.config.initial_retry_delay,
            },
        );
    }

    /// Mark a retry attempt — returns true if should retry, false if exceeded
    pub fn record_attempt(&self, message_id: &Uuid) -> bool {
        if let Some(mut entry) = self.entries.get_mut(message_id) {
            entry.attempt += 1;

            if entry.attempt > self.config.max_retries {
                self.dropped.fetch_add(1, Ordering::Relaxed);
                return false;
            }

            // Exponential backoff
            let new_delay = Duration::from_secs_f64(
                (entry.current_delay.as_secs_f64() * self.config.backoff_multiplier)
                    .min(self.config.max_retry_delay.as_secs_f64()),
            );
            entry.current_delay = new_delay;
            entry.next_retry = Instant::now() + new_delay;

            self.retried.fetch_add(1, Ordering::Relaxed);
            true
        } else {
            false
        }
    }

    pub fn remove(&self, message_id: &Uuid) {
        self.entries.remove(message_id);
    }

    /// Get retry queue size
    pub fn len(&self) -> usize {
        self.entries.len()
    }

    /// Get metrics
    pub fn stats(&self) -> RetryQueueStats {
        RetryQueueStats {
            queued: self.entries.len(),
            retried: self.retried.load(Ordering::Relaxed),
            dropped: self.dropped.load(Ordering::Relaxed),
        }
    }
}

#[derive(Debug, Clone)]
pub struct RetryQueueStats {
    pub queued: usize,
    pub retried: usize,
    pub dropped: usize,
}

// ── TTL Enforcement ───────────────────────────────────────────────────────────

/// Check if a message should be dropped based on TTL or hop count.
///
/// Returns `true` if the message should be relayed (not dropped).
pub fn should_relay(ttl: u64, hops: u8) -> bool {
    // Drop if TTL expired
    if ttl == 0 {
        return false;
    }
    // Drop if max hops exceeded
    if hops >= MAX_HOPS {
        return false;
    }
    true
}

/// Increment hop count and decrement TTL for relay.
///
/// Returns new (hops, ttl) or None if message should be dropped.
pub fn prepare_for_relay(hops: u8, ttl: u64) -> Option<(u8, u64)> {
    if !should_relay(ttl, hops) {
        return None;
    }
    Some((hops + 1, ttl.saturating_sub(1)))
}

// ── Combined Reliability Layer ────────────────────────────────────────────────

/// Combined reliability layer: ACK tracking + retry + dedup integration.
///
/// Uses the existing `MessageDeduplicator` from `micro_optimizations`
/// for dedup (not duplicated here).
pub struct ReliabilityLayer {
    pub ack_tracker: AckTracker,
    pub retry_queue: RetryQueue,
    config: ReliabilityConfig,
}

impl ReliabilityLayer {
    pub fn new(config: ReliabilityConfig) -> Self {
        Self {
            ack_tracker: AckTracker::new(config.clone()),
            retry_queue: RetryQueue::new(config.clone()),
            config,
        }
    }

    pub fn with_defaults() -> Self {
        Self::new(ReliabilityConfig::default())
    }

    /// Called when a message is sent — start tracking ACK
    pub fn on_message_sent(&self, message_id: Uuid) {
        self.ack_tracker.track(message_id);
        self.retry_queue.enqueue(message_id);
    }

    /// Called when an ACK is received — stop tracking
    pub fn on_ack_received(&self, message_id: &Uuid) -> bool {
        self.ack_tracker.acknowledge(message_id);
        self.retry_queue.remove(message_id);
        true
    }

    /// Cancel delivery tracking without treating the message as acknowledged.
    pub fn cancel_pending(&self, message_id: &Uuid) -> bool {
        let removed_ack = self.ack_tracker.cancel(message_id);
        self.retry_queue.remove(message_id);
        removed_ack
    }

    /// Called periodically (e.g., every second) to check for timeouts
    ///
    /// Returns message IDs that need retransmission
    pub fn check_timeouts(&self) -> Vec<Uuid> {
        self.ack_tracker.timed_out()
    }

    /// Get combined reliability stats
    pub fn stats(&self) -> ReliabilityStats {
        ReliabilityStats {
            ack: self.ack_tracker.stats(),
            retry: self.retry_queue.stats(),
        }
    }

    pub fn config(&self) -> &ReliabilityConfig {
        &self.config
    }
}

#[derive(Debug, Clone)]
pub struct ReliabilityStats {
    pub ack: AckTrackerStats,
    pub retry: RetryQueueStats,
}

// ── Tests ─────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ack_tracker_basic() {
        let tracker = AckTracker::new(ReliabilityConfig::default());
        let id = Uuid::new_v4();

        tracker.track(id);
        assert!(tracker.is_pending(&id));
        assert_eq!(tracker.pending_count(), 1);

        assert!(tracker.acknowledge(&id));
        assert!(!tracker.is_pending(&id));
        assert_eq!(tracker.pending_count(), 0);
    }

    #[test]
    fn test_ack_tracker_unknown_id() {
        let tracker = AckTracker::new(ReliabilityConfig::default());
        let id = Uuid::new_v4();

        // Acknowledging unknown ID returns false
        assert!(!tracker.acknowledge(&id));
    }

    #[test]
    fn test_retry_queue_exponential_backoff() {
        let config = ReliabilityConfig {
            initial_retry_delay: Duration::from_secs(1),
            backoff_multiplier: 2.0,
            max_retry_delay: Duration::from_secs(10),
            max_retries: 3,
            ..Default::default()
        };
        let queue = RetryQueue::new(config);
        let id = Uuid::new_v4();

        queue.enqueue(id);
        assert!(queue.record_attempt(&id)); // attempt 1
        assert!(queue.record_attempt(&id)); // attempt 2
        assert!(queue.record_attempt(&id)); // attempt 3
        assert!(!queue.record_attempt(&id)); // exceeded
    }

    #[test]
    fn test_ttl_enforcement() {
        assert!(!should_relay(0, 0)); // TTL=0 → drop
        assert!(should_relay(3600, 0)); // TTL fresh, hops=0 → relay
        assert!(should_relay(3600, 7)); // TTL fresh, hops=7 → relay
        assert!(!should_relay(3600, 8)); // MAX_HOPS exceeded → drop
        assert!(should_relay(1, 0)); // TTL=1 → relay (not expired yet)
    }

    #[test]
    fn test_prepare_for_relay() {
        assert_eq!(prepare_for_relay(0, 3600), Some((1, 3599)));
        assert_eq!(prepare_for_relay(7, 3600), Some((8, 3599)));
        assert_eq!(prepare_for_relay(8, 3600), None); // Max hops
        assert_eq!(prepare_for_relay(0, 0), None); // TTL expired
    }

    #[test]
    fn test_reliability_layer_lifecycle() {
        let layer = ReliabilityLayer::with_defaults();
        let id = Uuid::new_v4();

        // Send message
        layer.on_message_sent(id);
        assert!(layer.ack_tracker.is_pending(&id));

        // Receive ACK
        layer.on_ack_received(&id);
        assert!(!layer.ack_tracker.is_pending(&id));

        let stats = layer.stats();
        assert_eq!(stats.ack.acks_received, 1);
    }

    #[test]
    fn test_cancel_pending_does_not_count_as_acknowledgement() {
        let layer = ReliabilityLayer::with_defaults();
        let id = Uuid::new_v4();
        layer.on_message_sent(id);

        assert!(layer.cancel_pending(&id));
        assert!(!layer.ack_tracker.is_pending(&id));
        assert_eq!(layer.stats().ack.acks_received, 0);
        assert_eq!(layer.stats().retry.queued, 0);
    }

    #[test]
    fn test_ack_tracker_max_pending() {
        let config = ReliabilityConfig {
            max_pending_acks: 3,
            ..Default::default()
        };
        let tracker = AckTracker::new(config);

        let id1 = Uuid::new_v4();
        let id2 = Uuid::new_v4();
        let id3 = Uuid::new_v4();
        let id4 = Uuid::new_v4();

        tracker.track(id1);
        tracker.track(id2);
        tracker.track(id3);
        tracker.track(id4); // Should evict oldest

        // At most 3 pending (oldest evicted)
        assert!(tracker.pending_count() <= 3);
    }

    #[test]
    fn test_retry_queue_remove() {
        let queue = RetryQueue::new(ReliabilityConfig::default());
        let id = Uuid::new_v4();

        queue.enqueue(id);
        assert_eq!(queue.len(), 1);

        queue.remove(&id);
        assert_eq!(queue.len(), 0);
    }
}
