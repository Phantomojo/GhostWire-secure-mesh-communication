use serde::{Deserialize, Serialize};
use std::time::{SystemTime, UNIX_EPOCH};
use tracing::{debug, info};

pub const CAPABILITY_TCP: u16 = 0x0001;
pub const CAPABILITY_QUIC: u16 = 0x0002;
pub const CAPABILITY_MESH: u16 = 0x0004;
pub const CAPABILITY_WIFI: u16 = 0x0008;
pub const CAPABILITY_BLUETOOTH: u16 = 0x0010;
pub const CAPABILITY_LORA: u16 = 0x0020;
pub const CAPABILITY_COMPRESSION: u16 = 0x0040;
pub const CAPABILITY_RELAY: u16 = 0x0080;
pub const CAPABILITY_RETICULUM: u16 = 0x0100;

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
    Reticulum,
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
            TransportType::Reticulum => "reticulum",
        }
    }

    pub fn capability_flag(&self) -> u16 {
        match self {
            TransportType::StealthTcp => CAPABILITY_TCP,
            TransportType::Quic => CAPABILITY_QUIC,
            TransportType::Mesh => CAPABILITY_MESH,
            TransportType::WiFi => CAPABILITY_WIFI,
            TransportType::Bluetooth => CAPABILITY_BLUETOOTH,
            TransportType::LoRa => CAPABILITY_LORA,
            TransportType::Reticulum => CAPABILITY_RETICULUM,
        }
    }

    pub fn supports(remote_capabilities: u16) -> Vec<TransportType> {
        let mut supported = Vec::new();
        let flags = [
            (CAPABILITY_TCP, TransportType::StealthTcp),
            (CAPABILITY_QUIC, TransportType::Quic),
            (CAPABILITY_MESH, TransportType::Mesh),
            (CAPABILITY_WIFI, TransportType::WiFi),
            (CAPABILITY_BLUETOOTH, TransportType::Bluetooth),
            (CAPABILITY_LORA, TransportType::LoRa),
            (CAPABILITY_RETICULUM, TransportType::Reticulum),
        ];
        for (flag, transport) in flags {
            if remote_capabilities & flag != 0 {
                supported.push(transport);
            }
        }
        supported
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disaster_metrics_default() {
        let metrics = DisasterMetrics::default();
        assert_eq!(metrics.latency_ms, 0);
        assert_eq!(metrics.packet_loss_ratio, 0.0);
        assert_eq!(metrics.active_peers, 0);
        assert!(metrics.is_internet_reachable);
        assert!(metrics.last_update > 0);
    }

    #[test]
    fn test_disaster_metrics_serialization() {
        let metrics = DisasterMetrics {
            latency_ms: 100,
            packet_loss_ratio: 0.05,
            active_peers: 5,
            is_internet_reachable: true,
            last_update: 1234567890,
        };
        
        let json = serde_json::to_string(&metrics).unwrap();
        let deserialized: DisasterMetrics = serde_json::from_str(&json).unwrap();
        
        assert_eq!(deserialized.latency_ms, 100);
        assert_eq!(deserialized.packet_loss_ratio, 0.05);
        assert_eq!(deserialized.active_peers, 5);
    }

    #[test]
    fn test_disaster_level_ordering() {
        assert!(DisasterLevel::Normal < DisasterLevel::Congested);
        assert!(DisasterLevel::Congested < DisasterLevel::Isolation);
        assert!(DisasterLevel::Isolation < DisasterLevel::Blackout);
        assert!(DisasterLevel::Blackout > DisasterLevel::Normal);
    }

    #[test]
    fn test_disaster_level_equality() {
        assert_eq!(DisasterLevel::Normal, DisasterLevel::Normal);
        assert_eq!(DisasterLevel::Blackout, DisasterLevel::Blackout);
        assert_ne!(DisasterLevel::Normal, DisasterLevel::Blackout);
    }

    #[test]
    fn test_disaster_level_debug() {
        let level = DisasterLevel::Congested;
        assert_eq!(format!("{:?}", level), "Congested");
    }

    #[test]
    fn test_transport_type_names() {
        assert_eq!(TransportType::Mesh.name(), "mesh");
        assert_eq!(TransportType::Quic.name(), "quic");
        assert_eq!(TransportType::StealthTcp.name(), "stealth_tcp");
        assert_eq!(TransportType::WiFi.name(), "wifi");
        assert_eq!(TransportType::Bluetooth.name(), "bluetooth");
        assert_eq!(TransportType::LoRa.name(), "lora");
    }

    #[test]
    fn test_transport_type_equality() {
        assert_eq!(TransportType::Mesh, TransportType::Mesh);
        assert_eq!(TransportType::LoRa, TransportType::LoRa);
        assert_ne!(TransportType::WiFi, TransportType::Bluetooth);
    }

    #[test]
    fn test_transport_type_serialization() {
        let tt = TransportType::StealthTcp;
        let json = serde_json::to_string(&tt).unwrap();
        let deserialized: TransportType = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, TransportType::StealthTcp);
    }

    #[test]
    fn test_disaster_manager_new() {
        let mut manager = DisasterManager::new(200, 0.1, 3, 5);
        // Manager should be in Normal state initially
        assert_eq!(manager.evaluate(&DisasterMetrics::default()), DisasterLevel::Normal);
    }

    #[test]
    fn test_disaster_manager_normal_conditions() {
        let mut manager = DisasterManager::new(200, 0.1, 3, 3);
        let metrics = DisasterMetrics {
            latency_ms: 50,
            packet_loss_ratio: 0.01,
            active_peers: 5,
            is_internet_reachable: true,
            last_update: 0,
        };
        assert_eq!(manager.evaluate(&metrics), DisasterLevel::Normal);
    }

    #[test]
    fn test_disaster_manager_high_latency() {
        let mut manager = DisasterManager::new(100, 0.1, 3, 3);
        let metrics = DisasterMetrics {
            latency_ms: 200, // Above threshold
            packet_loss_ratio: 0.01,
            active_peers: 5,
            is_internet_reachable: true,
            last_update: 0,
        };
        assert_eq!(manager.evaluate(&metrics), DisasterLevel::Congested);
    }

    #[test]
    fn test_disaster_manager_high_packet_loss() {
        let mut manager = DisasterManager::new(200, 0.05, 3, 3);
        let metrics = DisasterMetrics {
            latency_ms: 50,
            packet_loss_ratio: 0.1, // Above threshold
            active_peers: 5,
            is_internet_reachable: true,
            last_update: 0,
        };
        assert_eq!(manager.evaluate(&metrics), DisasterLevel::Congested);
    }

    #[test]
    fn test_disaster_manager_isolation() {
        let mut manager = DisasterManager::new(200, 0.1, 3, 3);
        let metrics = DisasterMetrics {
            latency_ms: 50,
            packet_loss_ratio: 0.01,
            active_peers: 5,
            is_internet_reachable: false,
            last_update: 0,
        };
        assert_eq!(manager.evaluate(&metrics), DisasterLevel::Isolation);
    }

    #[test]
    fn test_disaster_manager_blackout() {
        let mut manager = DisasterManager::new(200, 0.1, 3, 3);
        let metrics = DisasterMetrics {
            latency_ms: 50,
            packet_loss_ratio: 0.01,
            active_peers: 1, // Below isolation threshold
            is_internet_reachable: false,
            last_update: 0,
        };
        assert_eq!(manager.evaluate(&metrics), DisasterLevel::Blackout);
    }

    #[test]
    fn test_disaster_manager_fast_escalation() {
        // When things get worse, it should escalate immediately (no hysteresis)
        let mut manager = DisasterManager::new(100, 0.1, 3, 5);
        
        // Start with normal
        let normal_metrics = DisasterMetrics {
            latency_ms: 50,
            packet_loss_ratio: 0.01,
            active_peers: 5,
            is_internet_reachable: true,
            last_update: 0,
        };
        assert_eq!(manager.evaluate(&normal_metrics), DisasterLevel::Normal);
        
        // Escalate immediately to congested
        let congested_metrics = DisasterMetrics {
            latency_ms: 200,
            packet_loss_ratio: 0.01,
            active_peers: 5,
            is_internet_reachable: true,
            last_update: 0,
        };
        assert_eq!(manager.evaluate(&congested_metrics), DisasterLevel::Congested);
    }

    #[test]
    fn test_disaster_manager_hysteresis_recovery() {
        // When things get better, it should wait for stability
        let mut manager = DisasterManager::new(100, 0.1, 3, 3);
        
        // Start with congested
        let congested_metrics = DisasterMetrics {
            latency_ms: 200,
            packet_loss_ratio: 0.01,
            active_peers: 5,
            is_internet_reachable: true,
            last_update: 0,
        };
        assert_eq!(manager.evaluate(&congested_metrics), DisasterLevel::Congested);
        
        // First recovery check - should still be congested (waiting for stability)
        let normal_metrics = DisasterMetrics {
            latency_ms: 50,
            packet_loss_ratio: 0.01,
            active_peers: 5,
            is_internet_reachable: true,
            last_update: 0,
        };
        
        // With required_stable_samples=3, should need multiple consecutive normal readings
        let _ = manager.evaluate(&normal_metrics);
        let _ = manager.evaluate(&normal_metrics);
        assert_eq!(manager.evaluate(&normal_metrics), DisasterLevel::Normal);
    }

    #[test]
    fn test_disaster_policy_default() {
        let policy = DisasterPolicy::default();
        
        assert!(!policy.normal_priority.is_empty());
        assert!(!policy.congested_priority.is_empty());
        assert!(!policy.isolation_priority.is_empty());
        assert!(!policy.blackout_priority.is_empty());
    }

    #[test]
    fn test_disaster_policy_recommendations() {
        let policy = DisasterPolicy::default();
        
        assert_eq!(
            policy.get_recommended_transport(DisasterLevel::Normal),
            Some(TransportType::Quic)
        );
        assert_eq!(
            policy.get_recommended_transport(DisasterLevel::Congested),
            Some(TransportType::Mesh)
        );
        assert_eq!(
            policy.get_recommended_transport(DisasterLevel::Isolation),
            Some(TransportType::WiFi)
        );
        assert_eq!(
            policy.get_recommended_transport(DisasterLevel::Blackout),
            Some(TransportType::LoRa)
        );
    }

    #[test]
    fn test_disaster_policy_priority_order() {
        let policy = DisasterPolicy::default();
        
        // Normal: Quic > StealthTcp > Mesh
        assert!(policy.normal_priority[0] == TransportType::Quic);
        assert!(policy.normal_priority.len() >= 3);
        
        // Blackout fallback: LoRa only
        assert_eq!(policy.blackout_priority.len(), 1);
        assert_eq!(policy.blackout_priority[0], TransportType::LoRa);
    }

    #[test]
    fn test_disaster_metrics_clone() {
        let metrics = DisasterMetrics::default();
        let cloned = metrics.clone();
        assert_eq!(metrics.latency_ms, cloned.latency_ms);
        assert_eq!(metrics.is_internet_reachable, cloned.is_internet_reachable);
    }

    #[test]
    fn test_disaster_level_serialization() {
        let level = DisasterLevel::Isolation;
        let json = serde_json::to_string(&level).unwrap();
        let deserialized: DisasterLevel = serde_json::from_str(&json).unwrap();
        assert_eq!(deserialized, DisasterLevel::Isolation);
    }
}
