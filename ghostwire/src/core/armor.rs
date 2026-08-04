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

    /// Get the number of data shards
    pub fn data_shard_count(&self) -> usize {
        self.data_shards
    }

    /// Get the number of parity shards
    pub fn parity_shard_count(&self) -> usize {
        self.parity_shards
    }

    /// Get total number of shards (data + parity)
    pub fn total_shards(&self) -> usize {
        self.data_shards + self.parity_shards
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_device_tier_default() {
        let tier = DeviceTier::default();
        assert_eq!(tier, DeviceTier::Medium);
    }

    #[test]
    fn test_device_tier_high_params() {
        let params = DeviceTier::High.params();
        assert_eq!(params.0, 1024); // min_packet_size
        assert_eq!(params.1, 512);  // max_padding
        assert_eq!(params.2, 200);  // max_chaff_delay_ms
        assert_eq!(params.3, 5_000); // cover_min_ms
        assert_eq!(params.4, 15_000); // cover_max_ms
    }

    #[test]
    fn test_device_tier_medium_params() {
        let params = DeviceTier::Medium.params();
        assert_eq!(params.0, 512);
        assert_eq!(params.1, 256);
        assert_eq!(params.2, 100);
        assert_eq!(params.3, 3_000);
        assert_eq!(params.4, 12_000);
    }

    #[test]
    fn test_device_tier_low_params() {
        let params = DeviceTier::Low.params();
        assert_eq!(params.0, 256);
        assert_eq!(params.1, 128);
        assert_eq!(params.2, 50);
        assert_eq!(params.3, 1_000);
        assert_eq!(params.4, 5_000);
    }

    #[test]
    fn test_device_tier_equality() {
        assert_eq!(DeviceTier::High, DeviceTier::High);
        assert_eq!(DeviceTier::Medium, DeviceTier::Medium);
        assert_eq!(DeviceTier::Low, DeviceTier::Low);
        assert_ne!(DeviceTier::High, DeviceTier::Medium);
        assert_ne!(DeviceTier::Medium, DeviceTier::Low);
    }

    #[test]
    fn test_device_tier_serialize() {
        let tier = DeviceTier::High;
        let serialized = serde_json::to_string(&tier).unwrap();
        assert_eq!(serialized, "\"High\"");
        
        let deserialized: DeviceTier = serde_json::from_str(&serialized).unwrap();
        assert_eq!(deserialized, DeviceTier::High);
    }

    #[test]
    fn test_erasure_armor_new() {
        let armor = ErasureArmor::new(3, 2);
        assert_eq!(armor.data_shard_count(), 3);
        assert_eq!(armor.parity_shard_count(), 2);
        assert_eq!(armor.total_shards(), 5);
    }

    #[test]
    fn test_erasure_armor_encode_produces_correct_shard_count() {
        let armor = ErasureArmor::new(3, 2);
        let data = b"Hello, GhostWire!";
        let shards = armor.encode(data);
        
        // Should produce (data_shards + parity_shards) shards
        assert_eq!(shards.len(), 5);
        
        // Each shard should contain the full data
        for shard in &shards {
            assert_eq!(shard.as_slice(), data);
        }
    }

    #[test]
    fn test_erasure_armor_reconstruct_with_sufficient_shards() {
        let armor = ErasureArmor::new(3, 2);
        let original_data = b"Test data for reconstruction";
        
        let shards = armor.encode(original_data);
        
        // Provide all shards as Some
        let shard_options: Vec<Option<Vec<u8>>> = shards.into_iter().map(Some).collect();
        let reconstructed = armor.reconstruct(shard_options);
        
        assert!(reconstructed.is_some());
        assert_eq!(reconstructed.unwrap().as_slice(), original_data);
    }

    #[test]
    fn test_erasure_armor_reconstruct_with_insufficient_shards() {
        let armor = ErasureArmor::new(3, 2);
        let data = b"Data that cannot be reconstructed";
        
        // Provide fewer than data_shards shards
        let shard_options = vec![
            Some(data.to_vec()),
            None, // missing
            None, // missing
        ];
        
        let reconstructed = armor.reconstruct(shard_options);
        assert!(reconstructed.is_none());
    }

    #[test]
    fn test_erasure_armor_reconstruct_with_some_failed_shards() {
        let armor = ErasureArmor::new(2, 1);
        let original = b"Partial failure test data";
        
        let shards = armor.encode(original);
        
        // Simulate one shard failure
        let mut shard_options: Vec<Option<Vec<u8>>> = shards.into_iter().map(Some).collect();
        shard_options[1] = None; // One shard failed
        
        // Should still be able to reconstruct with 2 data shards
        let reconstructed = armor.reconstruct(shard_options);
        assert!(reconstructed.is_some());
        assert_eq!(reconstructed.unwrap().as_slice(), original);
    }

    #[test]
    fn test_erasure_armor_reconstruct_with_all_none() {
        let armor = ErasureArmor::new(3, 2);
        let shard_options = vec![None, None, None, None, None];
        let reconstructed = armor.reconstruct(shard_options);
        assert!(reconstructed.is_none());
    }

    #[test]
    fn test_erasure_armor_empty_data() {
        let armor = ErasureArmor::new(2, 1);
        let shards = armor.encode(b"");
        assert_eq!(shards.len(), 3);
        
        let reconstructed = armor.reconstruct(shards.into_iter().map(Some).collect());
        assert!(reconstructed.is_some());
        assert_eq!(reconstructed.unwrap(), b"");
    }

    #[test]
    fn test_erasure_armor_military_grade_config() {
        // Military grade: 3 data, 2 parity
        let armor = ErasureArmor::new(3, 2);
        assert_eq!(armor.data_shard_count(), 3);
        assert_eq!(armor.parity_shard_count(), 2);
        assert_eq!(armor.total_shards(), 5);
        
        // Can tolerate losing up to 2 shards (parity count)
        let data = b"Military grade message";
        let shards: Vec<Option<Vec<u8>>> = armor.encode(data)
            .into_iter()
            .take(3) // Only provide 3 of 5 shards (2 lost)
            .map(Some)
            .collect();
        
        let reconstructed = armor.reconstruct(shards);
        assert!(reconstructed.is_some());
        assert_eq!(reconstructed.unwrap().as_slice(), data);
    }

    #[test]
    fn test_device_tier_debug_format() {
        let tier = DeviceTier::High;
        let debug_str = format!("{:?}", tier);
        assert_eq!(debug_str, "High");
    }
}
