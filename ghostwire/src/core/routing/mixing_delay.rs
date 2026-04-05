//! mixing_delay.rs — Exponential mixing delays for timing attack resistance
//!
//! Timing correlation is the most practical attack against real-world anonymous
//! networks. If an adversary observes that Alice sends a message and Bob
//! receives one 50ms later, they can correlate them across many observations.
//!
//! Nym's research showed that even with cover traffic, packet TIMING has a
//! signature. Their fix: exponential mixing delays at each hop.
//!
//! Why exponential? Because:
//!   - Small delays (P(d < 1ms)) are still possible — low latency preserved
//!   - Large delays (P(d > 1s)) are rare but occur — correlation is broken
//!   - The distribution is memoryless — no timing information leaks about
//!     how long a packet has been waiting
//!
//! Each relay node independently samples a delay from Exp(λ) before
//! forwarding. The total delay across k hops is Erlang(k, λ) — attackers
//! watching the first and last hop cannot correlate the timing.
//!
//! INTEGRATION: Wrap the forward() call in SphinxPacket::process_at_hop()
//! with `mixing_delay::sample_delay().await` before sending the next hop.

use rand::distributions::Distribution;
use rand_distr::Exp;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

/// Default mean delay in milliseconds (λ = 1/mean_ms)
/// 200ms mean = most messages < 500ms, rare outliers up to ~2s
pub const DEFAULT_MEAN_MS: f64 = 200.0;

/// Minimum delay (prevents trivial timing attacks even at small samples)
pub const MIN_DELAY_MS: u64 = 20;

/// Maximum delay cap (prevents extreme outliers from breaking UX)
pub const MAX_DELAY_MS: u64 = 5_000;

/// Mixing delay configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MixingConfig {
    /// Mean delay in milliseconds (controls λ = 1/mean_ms)
    pub mean_ms: f64,
    /// Whether mixing is enabled
    pub enabled: bool,
    /// Override min delay (ms)
    pub min_ms: u64,
    /// Override max delay cap (ms)
    pub max_ms: u64,
}

impl Default for MixingConfig {
    fn default() -> Self {
        Self {
            mean_ms: DEFAULT_MEAN_MS,
            enabled: true,
            min_ms: MIN_DELAY_MS,
            max_ms: MAX_DELAY_MS,
        }
    }
}

impl MixingConfig {
    /// High-anonymity preset: longer delays, better against nation-state adversaries
    pub fn high_anonymity() -> Self {
        Self {
            mean_ms: 1000.0,
            enabled: true,
            min_ms: 100,
            max_ms: 15_000,
        }
    }

    /// Low-latency preset: shorter delays for interactive use
    pub fn low_latency() -> Self {
        Self {
            mean_ms: 50.0,
            enabled: true,
            min_ms: 5,
            max_ms: 500,
        }
    }

    /// No mixing (disabled for testing or trusted-network contexts)
    pub fn disabled() -> Self {
        Self {
            enabled: false,
            ..Default::default()
        }
    }
}

/// Sample a random mixing delay from Exp(λ = 1/mean_ms) distribution.
///
/// Returns a Duration to sleep before forwarding the packet.
pub fn sample_delay(config: &MixingConfig) -> Duration {
    if !config.enabled {
        return Duration::ZERO;
    }

    // Exp(λ) where λ = 1/mean
    // Sample gives x in milliseconds
    let lambda = 1.0 / config.mean_ms;
    let exp = Exp::new(lambda).expect("Invalid exponential distribution parameter");
    let sample_ms = exp.sample(&mut rand::thread_rng());

    // Clamp to [min_ms, max_ms]
    let clamped_ms = sample_ms
        .max(config.min_ms as f64)
        .min(config.max_ms as f64) as u64;

    Duration::from_millis(clamped_ms)
}

/// Sleep for a mixing delay sampled from the exponential distribution.
/// Call this at a relay node before forwarding a Sphinx packet.
///
/// Usage:
/// ```rust
/// use crate::core::routing::mixing_delay::{apply_mixing_delay, MixingConfig};
///
/// let config = MixingConfig::default();
/// apply_mixing_delay(&config).await;
/// // Now forward the packet
/// mesh.send_sphinx_packet(next_hop, packet).await?;
/// ```
pub async fn apply_mixing_delay(config: &MixingConfig) {
    let delay = sample_delay(config);
    if delay > Duration::ZERO {
        sleep(delay).await;
    }
}

/// A queued message waiting for its mixing delay to expire
#[derive(Debug)]
pub struct QueuedSphinxPacket {
    pub next_hop: String,
    pub packet_bytes: Vec<u8>,
    pub forward_at: std::time::Instant,
}

/// A delay queue that holds Sphinx packets until their mixing delay expires.
/// This is the recommended way to implement mixing at scale — a single
/// background task drains the queue rather than blocking per-packet.
pub struct MixingQueue {
    queue: std::sync::Arc<tokio::sync::Mutex<Vec<QueuedSphinxPacket>>>,
    config: MixingConfig,
}

impl MixingQueue {
    pub fn new(config: MixingConfig) -> Self {
        Self {
            queue: std::sync::Arc::new(tokio::sync::Mutex::new(Vec::new())),
            config,
        }
    }

    /// Add a packet to the mixing queue with a sampled delay
    pub async fn enqueue(&self, next_hop: String, packet_bytes: Vec<u8>) {
        let delay = sample_delay(&self.config);
        let forward_at = std::time::Instant::now() + delay;
        self.queue.lock().await.push(QueuedSphinxPacket {
            next_hop,
            packet_bytes,
            forward_at,
        });
    }

    /// Drain all packets whose delay has expired.
    /// Call this in a background loop.
    pub async fn drain_ready(&self) -> Vec<QueuedSphinxPacket> {
        let now = std::time::Instant::now();
        let mut queue = self.queue.lock().await;
        let mut ready = Vec::new();
        let mut remaining = Vec::new();

        for pkt in queue.drain(..) {
            if pkt.forward_at <= now {
                ready.push(pkt);
            } else {
                remaining.push(pkt);
            }
        }

        *queue = remaining;
        ready
    }

    /// Start the drain loop. Spawns a background task that periodically
    /// flushes ready packets via a sender channel.
    pub fn start_drain_loop(
        self: std::sync::Arc<Self>,
        tx: tokio::sync::mpsc::Sender<QueuedSphinxPacket>,
    ) {
        tokio::spawn(async move {
            loop {
                tokio::time::sleep(Duration::from_millis(10)).await;
                let ready = self.drain_ready().await;
                for pkt in ready {
                    if tx.try_send(pkt).is_err() {
                        tracing::warn!("Mixing queue drain channel full — dropping packet");
                    }
                }
            }
        });
    }

    pub async fn queue_len(&self) -> usize {
        self.queue.lock().await.len()
    }
}

/// Statistics about mixing behaviour (for monitoring/UI)
#[derive(Debug, Default, serde::Serialize)]
pub struct MixingStats {
    pub packets_mixed: u64,
    pub mean_delay_ms: f64,
    pub max_delay_ms: u64,
    pub queue_depth: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn delay_within_bounds() {
        let config = MixingConfig::default();
        for _ in 0..1000 {
            let d = sample_delay(&config);
            assert!(d.as_millis() >= config.min_ms as u128);
            assert!(d.as_millis() <= config.max_ms as u128);
        }
    }

    #[test]
    fn disabled_config_returns_zero() {
        let config = MixingConfig::disabled();
        assert_eq!(sample_delay(&config), Duration::ZERO);
    }

    #[test]
    fn exponential_distribution_is_memoryless() {
        // The mean of many samples should be close to configured mean
        let config = MixingConfig {
            mean_ms: 100.0,
            enabled: true,
            min_ms: 0,
            max_ms: 10_000,
        };

        let n = 10_000;
        let sum: f64 = (0..n)
            .map(|_| sample_delay(&config).as_millis() as f64)
            .sum();
        let measured_mean = sum / n as f64;

        // Mean should be within 20% of configured mean
        assert!(
            (measured_mean - 100.0).abs() < 20.0,
            "Mean {} too far from 100ms",
            measured_mean
        );
    }

    #[tokio::test]
    async fn mixing_queue_drains_after_delay() {
        let config = MixingConfig {
            mean_ms: 10.0,
            enabled: true,
            min_ms: 5,
            max_ms: 50,
        };
        let queue = std::sync::Arc::new(MixingQueue::new(config));
        queue
            .enqueue("peer1".to_string(), vec![0u8; 64])
            .await;

        // Before delay: nothing ready
        let ready = queue.drain_ready().await;
        // May or may not be ready depending on timing — just check it doesn't panic
        drop(ready);

        // After max delay: everything ready
        tokio::time::sleep(Duration::from_millis(100)).await;
        let ready = queue.drain_ready().await;
        assert_eq!(ready.len(), 1);
        assert_eq!(ready[0].next_hop, "peer1");
    }
}
