use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Traffic Morphing configuration for DPI evasion.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MorphingConfig {
    /// Enable random packet padding.
    pub enable_padding: bool,
    /// Minimum packet size to pad to.
    pub min_packet_size: usize,
    /// Maximum additional random padding.
    pub max_padding: usize,
    /// Enable timing chaff (random delays).
    pub enable_timing_chaff: bool,
    /// Maximum random delay in milliseconds.
    pub max_chaff_delay_ms: u64,
}

impl Default for MorphingConfig {
    fn default() -> Self {
        Self {
            enable_padding: true,
            min_packet_size: 512,
            max_padding: 256,
            enable_timing_chaff: true,
            max_chaff_delay_ms: 100,
        }
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

        let mut rng = rand::thread_rng();
        let target_size = (self.config.min_packet_size)
            .max(payload.len() + rng.gen_range(0..self.config.max_padding));

        if payload.len() < target_size {
            let padding_len = target_size - payload.len();
            let padding: Vec<u8> = (0..padding_len).map(|_| rng.gen()).collect();
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
            let mut rng = rand::thread_rng();
            rng.gen_range(0..self.config.max_chaff_delay_ms)
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
