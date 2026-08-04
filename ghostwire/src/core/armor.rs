use serde::{Deserialize, Serialize};

/// Device capability tier — drives padding aggressiveness and cover traffic rate.
///
/// Higher tiers = more padding + slower cover = better privacy but more overhead.
/// Lower tiers = less padding + faster cover = less privacy but battery/CPU friendly.
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[derive(Default)]
pub enum DeviceTier {
    /// Desktop, laptop on AC, server. Aggressive padding, slow cover.
    High,
    /// Laptop on battery, modern phone. Moderate padding, moderate cover.
    #[default]
    Medium,
    /// IoT, old phone, constrained device. Light padding, fast cover.
    Low,
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
