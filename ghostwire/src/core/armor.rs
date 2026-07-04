use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Device capability tier — drives padding aggressiveness and cover traffic rate.
///
/// Higher tiers = more padding + slower cover = better privacy but more overhead.
/// Lower tiers = less padding + faster cover = less privacy but battery/CPU friendly.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum DeviceTier {
    /// Desktop, laptop on AC, server. Aggressive padding, slow cover.
    High,
    /// Laptop on battery, modern phone. Moderate padding, moderate cover.
    Medium,
    /// IoT, old phone, constrained device. Light padding, fast cover.
    Low,
}

impl Default for DeviceTier {
    fn default() -> Self {
        Self::Medium
    }
}

impl DeviceTier {
    /// (min_packet_size, max_padding, max_chaff_delay_ms, cover_min_ms, cover_max_ms)
    pub fn params(self) -> (usize, usize, u64, u64, u64) {
        match self {
            Self::High => (1024, 512, 200, 5_000, 15_000),
            Self::Medium => (512, 256, 100, 3_000, 12_000),
            Self::Low => (256, 128, 50, 1_000, 5_000),
        }
    }

    /// Auto-detect tier from system. Conservative default: Medium.
    pub fn detect() -> Self {
        #[cfg(target_os = "linux")]
        {
            if std::path::Path::new("/sys/class/power_supply/AC/online").exists() {
                let on_ac = std::fs::read_to_string("/sys/class/power_supply/AC/online")
                    .map(|s| s.trim() == "1")
                    .unwrap_or(false);
                if on_ac {
                    return Self::High;
                }
            }
        }
        Self::Medium
    }
}

/// Traffic Morphing configuration for DPI evasion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorphingConfig {
    pub enable_padding: bool,
    pub min_packet_size: usize,
    pub max_padding: usize,
    pub enable_timing_chaff: bool,
    pub max_chaff_delay_ms: u64,
    #[serde(default)]
    pub tier: DeviceTier,
}

impl Default for MorphingConfig {
    fn default() -> Self {
        let tier = DeviceTier::Medium;
        let (min_packet_size, max_padding, max_chaff_delay_ms, _, _) = tier.params();
        Self {
            enable_padding: true,
            min_packet_size,
            max_padding,
            enable_timing_chaff: true,
            max_chaff_delay_ms,
            tier,
        }
    }
}

impl MorphingConfig {
    pub fn from_tier(tier: DeviceTier) -> Self {
        let (min_packet_size, max_padding, max_chaff_delay_ms, _, _) = tier.params();
        Self {
            enable_padding: true,
            min_packet_size,
            max_padding,
            enable_timing_chaff: true,
            max_chaff_delay_ms,
            tier,
        }
    }

    /// Cover traffic interval range for this tier.
    pub fn cover_range_ms(&self) -> (u64, u64) {
        let (_, _, _, min, max) = self.tier.params();
        (min, max)
    }
}

/// Military-grade "Armor" for morphing traffic and defeating DPI.
pub struct TrafficArmor {
    config: MorphingConfig,
}

impl TrafficArmor {
    pub fn new(config: MorphingConfig) -> Self {
        Self { config }
    }

    /// Morphs a payload by adding padding and noise.
    pub fn morph_payload(&self, mut payload: Vec<u8>) -> Vec<u8> {
        if !self.config.enable_padding {
            return payload;
        }

        let mut rng = rand::rng();
        let target_size = (self.config.min_packet_size)
            .max(payload.len() + rng.random_range(0..self.config.max_padding));

        if payload.len() < target_size {
            let padding_len = target_size - payload.len();
            let padding: Vec<u8> = (0..padding_len).map(|_| rng.random()).collect();
            payload.extend_from_slice(&padding);
        }

        payload
    }

    /// Applies timing chaff to the current execution.
    pub async fn apply_timing_chaff(&self) {
        if !self.config.enable_timing_chaff {
            return;
        }

        let delay_ms = {
            let mut rng = rand::rng();
            rng.random_range(0..self.config.max_chaff_delay_ms)
        };
        tokio::time::sleep(Duration::from_millis(delay_ms)).await;
    }
}

/// Erasure Coding (Fountain Code) interface for high-interference links (LoRa/Radio).
pub struct ErasureArmor {
    data_shards: usize,
    parity_shards: usize,
}

impl ErasureArmor {
    pub fn new(data: usize, parity: usize) -> Self {
        Self {
            data_shards: data,
            parity_shards: parity,
        }
    }

    /// Encodes data into multiple shards.
    /// In a real implementation, this would use Reed-Solomon or RaptorQ.
    pub fn encode(&self, data: &[u8]) -> Vec<Vec<u8>> {
        // Reed-Solomon FEC can be added here for forward error correction
        // For now, we simulate by creating redundant copies or simple XOR parity
        let mut shards = Vec::new();
        for _ in 0..(self.data_shards + self.parity_shards) {
            shards.push(data.to_vec());
        }
        shards
    }

    /// Reconstructs data from available shards.
    pub fn reconstruct(&self, shards: Vec<Option<Vec<u8>>>) -> Option<Vec<u8>> {
        // If we have at least 'data_shards', we can reconstruct
        let successful_shards: Vec<Vec<u8>> = shards.into_iter().flatten().collect();
        if successful_shards.len() >= self.data_shards {
            return successful_shards.first().cloned();
        }
        None
    }
}
