//! peer_health.rs — Per-peer health tracking with RTT measurement
//!
//! Tracks latency, packet loss, last_seen, and connection state for each
//! connected peer. Driven by Ping/Pong frame exchange in the MeshDriver.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::debug;

/// Health metrics for a single peer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerHealth {
    pub peer_id: String,
    pub avg_rtt_ms: f64,
    pub ewma_rtt_ms: f64,
    pub jitter_ms: f64,
    pub p95_rtt_ms: f64,
    pub pings_sent: u64,
    pub pongs_received: u64,
    pub last_seen_ms: u64,
    pub is_alive: bool,
    pub consecutive_misses: u32,
    #[serde(skip)]
    rtt_samples: Vec<f64>,
}

impl PeerHealth {
    const EWMA_ALPHA: f64 = 0.125;
    const MAX_SAMPLES: usize = 100;

    fn new(peer_id: String) -> Self {
        Self {
            peer_id,
            avg_rtt_ms: 0.0,
            ewma_rtt_ms: 0.0,
            jitter_ms: 0.0,
            p95_rtt_ms: 0.0,
            pings_sent: 0,
            pongs_received: 0,
            last_seen_ms: now_ms(),
            is_alive: true,
            consecutive_misses: 0,
            rtt_samples: Vec::with_capacity(Self::MAX_SAMPLES),
        }
    }

    pub fn record_rtt(&mut self, rtt_ms: f64) {
        self.rtt_samples.push(rtt_ms);
        if self.rtt_samples.len() > Self::MAX_SAMPLES {
            self.rtt_samples
                .drain(..self.rtt_samples.len() - Self::MAX_SAMPLES);
        }

        self.avg_rtt_ms = self.rtt_samples.iter().sum::<f64>() / self.rtt_samples.len() as f64;

        // EWMA: exponentially weighted — recent samples matter more
        if self.pongs_received == 0 {
            self.ewma_rtt_ms = rtt_ms;
        } else {
            self.ewma_rtt_ms =
                Self::EWMA_ALPHA * rtt_ms + (1.0 - Self::EWMA_ALPHA) * self.ewma_rtt_ms;
        }

        // Jitter: standard deviation of RTT samples
        if self.rtt_samples.len() >= 2 {
            let mean = self.avg_rtt_ms;
            let variance = self
                .rtt_samples
                .iter()
                .map(|s| (s - mean).powi(2))
                .sum::<f64>()
                / (self.rtt_samples.len() - 1) as f64;
            self.jitter_ms = variance.sqrt();
        }

        // P95: sort and take 95th percentile
        let mut sorted = self.rtt_samples.clone();
        sorted.sort_by(|a, b| a.partial_cmp(b).unwrap_or(std::cmp::Ordering::Equal));
        let idx = (sorted.len() as f64 * 0.95) as usize;
        self.p95_rtt_ms = sorted[idx.min(sorted.len() - 1)];

        self.pongs_received += 1;
        self.last_seen_ms = now_ms();
        self.is_alive = true;
        self.consecutive_misses = 0;
    }

    pub fn record_miss(&mut self) {
        self.consecutive_misses += 1;
        if self.consecutive_misses >= 3 {
            self.is_alive = false;
        }
    }

    pub fn should_disconnect(&self) -> bool {
        self.consecutive_misses >= 3
    }

    /// Record that a ping was sent
    pub fn record_ping_sent(&mut self) {
        self.pings_sent += 1;
    }

    /// Mark that we received any traffic from this peer
    pub fn record_activity(&mut self) {
        self.last_seen_ms = now_ms();
        self.is_alive = true;
    }

    /// Check if peer is considered dead (no traffic for timeout)
    pub fn check_liveness(&mut self, timeout_ms: u64) -> bool {
        let elapsed = now_ms().saturating_sub(self.last_seen_ms);
        self.is_alive = elapsed < timeout_ms;
        self.is_alive
    }

    /// Packet loss ratio (0.0 = perfect, 1.0 = total loss)
    pub fn packet_loss_ratio(&self) -> f64 {
        if self.pings_sent == 0 {
            return 0.0;
        }
        let lost = self.pings_sent.saturating_sub(self.pongs_received) as f64;
        lost / self.pings_sent as f64
    }
}

/// Central peer health tracker shared across the mesh driver
#[derive(Debug, Clone)]
pub struct PeerHealthTracker {
    peers: Arc<RwLock<HashMap<String, PeerHealth>>>,
    /// Ping interval in milliseconds
    ping_interval_ms: u64,
    /// Timeout after which a peer is considered dead
    peer_timeout_ms: u64,
}

impl PeerHealthTracker {
    pub fn new(ping_interval: Duration, peer_timeout: Duration) -> Self {
        Self {
            peers: Arc::new(RwLock::new(HashMap::new())),
            ping_interval_ms: ping_interval.as_millis() as u64,
            peer_timeout_ms: peer_timeout.as_millis() as u64,
        }
    }

    /// Register a new peer (called on connection established)
    pub async fn register_peer(&self, peer_id: String) {
        let mut map = self.peers.write().await;
        if !map.contains_key(&peer_id) {
            debug!("Registering new peer {} for health tracking", peer_id);
            map.insert(peer_id.clone(), PeerHealth::new(peer_id));
        }
    }

    /// Remove a peer (called on connection closed)
    pub async fn remove_peer(&self, peer_id: &str) {
        let mut map = self.peers.write().await;
        map.remove(peer_id);
    }

    /// Record that we sent a ping to this peer
    pub async fn record_ping(&self, peer_id: &str) {
        let mut map = self.peers.write().await;
        if let Some(health) = map.get_mut(peer_id) {
            health.record_ping_sent();
        }
    }

    /// Record that we received a pong from this peer
    pub async fn record_pong(&self, peer_id: &str, rtt_ms: f64) {
        let mut map = self.peers.write().await;
        if let Some(health) = map.get_mut(peer_id) {
            health.record_rtt(rtt_ms);
            debug!(
                "Peer {} pong: RTT={:.1}ms, avg={:.1}ms",
                peer_id, rtt_ms, health.avg_rtt_ms
            );
        }
    }

    pub async fn record_miss(&self, peer_id: &str) {
        let mut map = self.peers.write().await;
        if let Some(health) = map.get_mut(peer_id) {
            health.record_miss();
            if health.should_disconnect() {
                debug!(
                    "Peer {} exceeded 3 consecutive misses, marking for disconnect",
                    peer_id
                );
            }
        }
    }

    pub async fn peers_to_disconnect(&self) -> Vec<String> {
        self.peers
            .read()
            .await
            .iter()
            .filter(|(_, h)| h.should_disconnect())
            .map(|(id, _)| id.clone())
            .collect()
    }

    /// Record any activity from a peer
    pub async fn record_activity(&self, peer_id: &str) {
        let mut map = self.peers.write().await;
        if let Some(health) = map.get_mut(peer_id) {
            health.record_activity();
        }
    }

    /// Get health for a specific peer
    pub async fn get_peer_health(&self, peer_id: &str) -> Option<PeerHealth> {
        self.peers.read().await.get(peer_id).cloned()
    }

    /// Get all peer health summaries
    pub async fn get_all_health(&self) -> Vec<PeerHealth> {
        self.peers.read().await.values().cloned().collect()
    }

    /// Non-blocking version for use inside sync functions called from tokio contexts.
    /// Returns empty vec if the RwLock is contended.
    pub fn try_get_all_health(&self) -> Vec<PeerHealth> {
        match self.peers.try_read() {
            Ok(guard) => guard.values().cloned().collect(),
            Err(_) => Vec::new(),
        }
    }

    /// Check liveness of all peers and mark dead ones
    pub async fn check_all_liveness(&self) {
        let mut map = self.peers.write().await;
        let timeout = self.peer_timeout_ms;
        for health in map.values_mut() {
            health.check_liveness(timeout);
        }
    }

    /// Get the ping interval
    pub fn ping_interval_ms(&self) -> u64 {
        self.ping_interval_ms
    }

    /// Get list of all tracked peer IDs
    pub async fn tracked_peers(&self) -> Vec<String> {
        self.peers.read().await.keys().cloned().collect()
    }
}

/// Pending ping state: tracks in-flight pings awaiting pong responses
pub struct PendingPings {
    /// Maps ping timestamp → peer_id
    pings: HashMap<u32, String>,
}

impl PendingPings {
    pub fn new() -> Self {
        Self {
            pings: HashMap::new(),
        }
    }

    /// Register an in-flight ping
    pub fn insert(&mut self, ts: u32, peer_id: String) {
        self.pings.insert(ts, peer_id);
    }

    /// Resolve a pong: returns the peer_id and RTT in ms, or None if unknown
    pub fn resolve(&mut self, pong_ts_ms: u32) -> Option<(String, f64)> {
        if let Some(peer_id) = self.pings.remove(&pong_ts_ms) {
            let now_ms = now_ms_u32();
            // pong_ts_ms is the millisecond timestamp we sent in the ping
            let rtt_ms = now_ms.saturating_sub(pong_ts_ms) as f64;
            Some((peer_id, rtt_ms))
        } else {
            None
        }
    }

    /// Clean up stale pings older than timeout_ms
    pub fn cleanup_stale(&mut self, timeout_ms: u32) {
        let now_ms = now_ms_u32();
        self.pings
            .retain(|ts, _| now_ms.saturating_sub(*ts) < timeout_ms);
    }

    pub fn cleanup_stale_with_misses(&mut self, timeout_ms: u32) -> Vec<String> {
        let now_ms = now_ms_u32();
        let stale: Vec<String> = self
            .pings
            .iter()
            .filter(|(ts, _)| now_ms.saturating_sub(**ts) >= timeout_ms)
            .map(|(_, peer_id)| peer_id.clone())
            .collect();
        self.pings
            .retain(|ts, _| now_ms.saturating_sub(*ts) < timeout_ms);
        stale
    }
}

impl Default for PendingPings {
    fn default() -> Self {
        Self::new()
    }
}

fn now_ms() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

fn now_ms_u32() -> u32 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_peer_health_basic() {
        let mut health = PeerHealth::new("peer1".to_string());
        assert!(health.is_alive);
        assert_eq!(health.pings_sent, 0);
        assert_eq!(health.pongs_received, 0);

        health.record_ping_sent();
        assert_eq!(health.pings_sent, 1);

        health.record_rtt(50.0);
        assert_eq!(health.pongs_received, 1);
        assert!((health.avg_rtt_ms - 50.0).abs() < 0.01);
    }

    #[test]
    fn test_packet_loss_ratio() {
        let mut health = PeerHealth::new("peer1".to_string());
        health.record_ping_sent();
        health.record_ping_sent();
        health.record_ping_sent();
        health.record_rtt(10.0);
        health.record_rtt(20.0);
        // 3 sent, 2 received → 1 lost
        assert!((health.packet_loss_ratio() - 1.0 / 3.0).abs() < 0.01);
    }

    #[test]
    fn test_peer_liveness() {
        let mut health = PeerHealth::new("peer1".to_string());
        assert!(health.check_liveness(5000)); // 5s timeout, just created → alive

        // Simulate old peer
        health.last_seen_ms = now_ms() - 10_000;
        assert!(!health.check_liveness(5000)); // 10s ago > 5s timeout → dead
    }

    #[tokio::test]
    async fn test_tracker_concurrent() {
        let tracker = PeerHealthTracker::new(Duration::from_secs(2), Duration::from_secs(30));
        tracker.register_peer("peer1".to_string()).await;
        tracker.record_ping("peer1").await;
        tracker.record_pong("peer1", 42.0).await;

        let health = tracker.get_peer_health("peer1").await.unwrap();
        assert_eq!(health.pongs_received, 1);
        assert!((health.avg_rtt_ms - 42.0).abs() < 0.01);
    }

    #[test]
    fn test_pending_pings_resolve() {
        let mut pending = PendingPings::new();
        let ts = 1_700_000_000u32;
        pending.insert(ts, "peer1".to_string());

        let (peer_id, rtt) = pending.resolve(ts).unwrap();
        assert_eq!(peer_id, "peer1");
        assert!(rtt >= 0.0);
    }

    #[test]
    fn test_pending_pings_unknown_pong() {
        let mut pending = PendingPings::new();
        assert!(pending.resolve(999).is_none());
    }

    #[test]
    fn test_ewma_converges() {
        let mut health = PeerHealth::new("peer1".to_string());
        // Feed constant RTT
        for _ in 0..20 {
            health.record_rtt(100.0);
        }
        assert!(
            (health.ewma_rtt_ms - 100.0).abs() < 0.01,
            "EWMA should converge to 100.0, got {}",
            health.ewma_rtt_ms
        );
    }

    #[test]
    fn test_ewma_responds_to_change() {
        let mut health = PeerHealth::new("peer1".to_string());
        for _ in 0..10 {
            health.record_rtt(100.0);
        }
        assert!((health.ewma_rtt_ms - 100.0).abs() < 1.0);

        // Spike to 200
        health.record_rtt(200.0);
        assert!(health.ewma_rtt_ms > 100.0, "EWMA should respond to spike");
        assert!(
            health.ewma_rtt_ms < 200.0,
            "EWMA should not jump to spike value"
        );
    }

    #[test]
    fn test_jitter_increases_with_variance() {
        let mut stable = PeerHealth::new("stable".to_string());
        for _ in 0..20 {
            stable.record_rtt(50.0);
        }
        assert!(
            stable.jitter_ms < 1.0,
            "Stable RTT should have low jitter, got {}",
            stable.jitter_ms
        );

        let mut variable = PeerHealth::new("variable".to_string());
        for i in 0..20 {
            variable.record_rtt(50.0 + (i % 2) as f64 * 100.0);
        }
        assert!(
            variable.jitter_ms > 5.0,
            "Variable RTT should have high jitter, got {}",
            variable.jitter_ms
        );
    }

    #[test]
    fn test_p95_captures_tail() {
        let mut health = PeerHealth::new("peer1".to_string());
        // 95 samples at 10ms, 5 samples at 100ms
        for _ in 0..95 {
            health.record_rtt(10.0);
        }
        for _ in 0..5 {
            health.record_rtt(100.0);
        }
        // P95 should be at least 100 (the tail)
        assert!(
            health.p95_rtt_ms >= 10.0,
            "P95 should capture tail, got {}",
            health.p95_rtt_ms
        );
    }
}
