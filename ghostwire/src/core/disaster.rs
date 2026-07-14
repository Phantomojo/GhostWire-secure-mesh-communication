use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, info};

/// Real-time network health metrics for GhostWire.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisasterMetrics {
    /// Average round-trip time in milliseconds.
    pub latency_ms: u32,
    /// Ratio of lost packets (0.0 to 1.0).
    pub packet_loss_ratio: f32,
    /// Number of reachable peers in the mesh.
    pub active_peers: u32,
    /// Whether the public internet is reachable (Stealth TCP / Infrastructure).
    pub is_internet_reachable: bool,
    /// Timestamp of the last metric update.
    pub last_update: u64,
}

impl Default for DisasterMetrics {
    fn default() -> Self {
        Self {
            latency_ms: 0,
            packet_loss_ratio: 0.0,
            active_peers: 0,
            is_internet_reachable: true,
            last_update: SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}

/// Levels of network degradation and disaster.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DisasterLevel {
    /// Optimal operation.
    Normal = 0,
    /// High latency or packet loss, but infrastructure is up.
    Congested = 1,
    /// Public internet is unreachable, but local mesh is active.
    Isolation = 2,
    /// Total blackout of the primary medium.
    Blackout = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum TransportType {
    Mesh,
    Quic,
    StealthTcp,
    WiFi,
    Bluetooth,
    LoRa,
}

impl TransportType {
    pub fn name(&self) -> &'static str {
        match self {
            TransportType::Mesh => "mesh",
            TransportType::Quic => "quic",
            TransportType::StealthTcp => "stealth_tcp",
            TransportType::WiFi => "wifi",
            TransportType::Bluetooth => "bluetooth",
            TransportType::LoRa => "lora",
        }
    }
}

/// Logic for evaluating metrics into a disaster level with anti-thrashing hysteresis.
pub struct DisasterManager {
    latency_threshold: u32,
    loss_threshold: f32,
    isolation_peer_count: u32,

    // Hysteresis State
    current_level: DisasterLevel,
    stability_counter: u32,
    required_stable_samples: u32,
}

impl DisasterManager {
    pub fn new(latency: u32, loss: f32, peers: u32, stable_samples: u32) -> Self {
        Self {
            latency_threshold: latency,
            loss_threshold: loss,
            isolation_peer_count: peers,
            current_level: DisasterLevel::Normal,
            stability_counter: 0,
            required_stable_samples: stable_samples,
        }
    }

    /// Evaluates metrics and returns the NEW level, considering hysteresis.
    pub fn evaluate(&mut self, metrics: &DisasterMetrics) -> DisasterLevel {
        let instantaneous_level = self.calculate_instant_level(metrics);

        // Rule 1: Fast Adaptation (If things get worse, move IMMEDIATELY)
        if instantaneous_level > self.current_level {
            self.current_level = instantaneous_level;
            self.stability_counter = 0; // Reset stability, we are in trouble
            return self.current_level;
        }

        // Rule 2: Stable Recovery (If things get better, WAIT until they stay better)
        if instantaneous_level < self.current_level {
            self.stability_counter += 1;
            if self.stability_counter >= self.required_stable_samples {
                info!(
                    "NETWORK STABILIZED: Recovering from {:?} to {:?}",
                    self.current_level, instantaneous_level
                );
                self.current_level = instantaneous_level;
                self.stability_counter = 0;
            } else {
                debug!(
                    "Network appears better ({:?}), but waiting for stability ({} / {})",
                    instantaneous_level, self.stability_counter, self.required_stable_samples
                );
            }
        } else {
            // Level is the same as current, reset stability counter
            self.stability_counter = 0;
        }

        self.current_level
    }

    fn calculate_instant_level(&self, metrics: &DisasterMetrics) -> DisasterLevel {
        if !metrics.is_internet_reachable && metrics.active_peers < self.isolation_peer_count {
            return DisasterLevel::Blackout;
        }

        if !metrics.is_internet_reachable {
            return DisasterLevel::Isolation;
        }

        if metrics.latency_ms > self.latency_threshold
            || metrics.packet_loss_ratio > self.loss_threshold
        {
            return DisasterLevel::Congested;
        }

        DisasterLevel::Normal
    }
}

/// Fallback policy based on disaster level.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisasterPolicy {
    pub normal_priority: Vec<TransportType>,
    pub congested_priority: Vec<TransportType>,
    pub isolation_priority: Vec<TransportType>,
    pub blackout_priority: Vec<TransportType>,
}

impl Default for DisasterPolicy {
    fn default() -> Self {
        Self {
            normal_priority: vec![TransportType::Quic, TransportType::StealthTcp, TransportType::Mesh],
            congested_priority: vec![TransportType::Mesh, TransportType::WiFi],
            isolation_priority: vec![TransportType::WiFi, TransportType::Bluetooth],
            blackout_priority: vec![TransportType::LoRa],
        }
    }
}

impl DisasterPolicy {
    pub fn get_recommended_transport(&self, level: DisasterLevel) -> Option<TransportType> {
        match level {
            DisasterLevel::Normal => self.normal_priority.first().cloned(),
            DisasterLevel::Congested => self.congested_priority.first().cloned(),
            DisasterLevel::Isolation => self.isolation_priority.first().cloned(),
            DisasterLevel::Blackout => self.blackout_priority.first().cloned(),
        }
    }
}
