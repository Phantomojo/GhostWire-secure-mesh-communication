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
    /// Rolling average RTT in milliseconds
    pub avg_rtt_ms: f64,
    /// Number of pings sent
    pub pings_sent: u64,
    /// Number of pongs received
    pub pongs_received: u64,
    /// Last time we received any traffic from this peer (epoch millis)
    pub last_seen_ms: u64,
    /// Whether we consider this peer currently alive
    pub is_alive: bool,
    /// Recent RTT samples (capped at 60 entries)
    #[serde(skip)]
    rtt_samples: Vec<f64>,
}

impl PeerHealth {
    fn new(peer_id: String) -> Self {
        Self {
            peer_id,
            avg_rtt_ms: 0.0,
            pings_sent: 0,
            pongs_received: 0,
            last_seen_ms: now_ms(),
            is_alive: true,
            rtt_samples: Vec::with_capacity(60),
        }
    }

    /// Record an RTT sample and update rolling average
    pub fn record_rtt(&mut self, rtt_ms: f64) {
        self.rtt_samples.push(rtt_ms);
        // Keep last 60 samples (≈ 2 minutes at 2s interval)
        if self.rtt_samples.len() > 60 {
            self.rtt_samples.drain(..self.rtt_samples.len() - 60);
        }
        self.avg_rtt_ms = self.rtt_samples.iter().sum::<f64>() / self.rtt_samples.len() as f64;
        self.pongs_received += 1;
        self.last_seen_ms = now_ms();
        self.is_alive = true;
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
}
