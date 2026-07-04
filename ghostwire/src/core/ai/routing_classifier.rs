// Smart PRoPHET Routing Classifier
// ML-enhanced routing decisions for mesh networks
//
// Architecture (4-layer fallback chain):
//   Layer 1: OnnxRouter (LightGBM, 2,387 features) — anomaly detection
//   Layer 4: PRoPHET formula — always-on fallback for routing

use crate::core::ai::OnnxRouter;
use anyhow::{Context, Result};
use libp2p::PeerId;

/// Features for routing decision
#[derive(Clone, Debug, Default)]
pub struct RoutingFeatures {
    pub rssi: i16,          // Signal strength (-100 to 0 dBm)
    pub latency_ms: u64,    // Round-trip latency
    pub packet_loss: f32,   // Packet loss rate (0.0-1.0)
    pub trust_score: f32,   // Peer trust score (0.0-1.0)
    pub hop_count: u8,      // Number of hops to destination
    pub battery_level: f32, // Battery level (0.0-1.0, 1.0 for powered nodes)
    pub time_of_day: u8,    // Hour of day (0-23)
}

/// Routing decision from ML model
#[derive(Clone, Debug)]
pub struct RoutingDecision {
    pub selected_peer: PeerId,
    pub confidence: f32,
    pub alternative_peers: Vec<(PeerId, f32)>,
}

/// Smart PRoPHET routing classifier
///
/// Uses PRoPHET formula for routing decisions,
/// with OnnxRouter available for anomaly detection on mesh state.
pub struct RoutingClassifier {
    pub router: OnnxRouter,
}

impl RoutingClassifier {
    /// Create a new routing classifier with the LightGBM ONNX model
    pub fn new(model_path: &str) -> Result<Self> {
        let router = OnnxRouter::new(model_path).context("Failed to load ONNX routing model")?;

        Ok(Self { router })
    }

    /// Select best next-hop peer for destination using ML prediction
    ///
    /// Uses PRoPHET formula for routing (the ONNX model is for anomaly detection
    /// with 2,387 features from Guifi.net mesh data).
    pub fn select_next_hop(
        &self,
        candidates: Vec<(PeerId, RoutingFeatures)>,
    ) -> Result<RoutingDecision> {
        if candidates.is_empty() {
            anyhow::bail!("No routing candidates available");
        }

        // Score each candidate using PRoPHET formula
        let mut scored_candidates = Vec::new();
        for (peer_id, features) in &candidates {
            let confidence = fallback::calculate_delivery_predictivity(
                features.rssi,
                features.latency_ms,
                features.packet_loss,
                features.trust_score,
            ) as f32;

            scored_candidates.push((peer_id.clone(), confidence));
        }

        // Sort by confidence descending
        scored_candidates
            .sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        // Select best
        let (selected_peer, confidence) = scored_candidates
            .first()
            .context("No candidates after scoring")?
            .clone();

        // Get top alternatives (up to 3)
        let alternatives: Vec<_> = scored_candidates.into_iter().skip(1).take(3).collect();

        Ok(RoutingDecision {
            selected_peer,
            confidence,
            alternative_peers: alternatives,
        })
    }

    /// Check mesh state for anomalies using the ONNX LightGBM model
    ///
    /// Takes a 2,387-dimensional feature vector from the mesh monitoring system.
    /// Returns anomaly probability (0.0 = normal, 1.0 = anomaly detected).
    pub fn check_anomaly(&self, features: &[f32]) -> Option<f32> {
        self.router.predict(features)
    }
}

/// Fallback routing when ML model is unavailable
pub mod fallback {
    use super::*;

    /// Simple formula-based PRoPHET routing
    pub fn calculate_delivery_predictivity(
        rssi: i16,
        latency_ms: u64,
        packet_loss: f32,
        trust_score: f32,
    ) -> f64 {
        // PRoPHET formula: P(a,b) = P(a,b)_old + (1 - P(a,b)_old) * P_init

        // Calculate base score from features
        let rssi_score = ((rssi as f64 + 100.0) / 100.0).max(0.0);
        let latency_score = (1.0 - (latency_ms as f64 / 1000.0)).max(0.0);
        let loss_score = 1.0 - packet_loss as f64;

        // Weighted combination
        let base_score =
            rssi_score * 0.3 + latency_score * 0.3 + loss_score * 0.2 + trust_score as f64 * 0.2;

        base_score
    }

    /// Select best peer using formula-based approach
    pub fn select_next_hop_formula(candidates: Vec<(PeerId, RoutingFeatures)>) -> PeerId {
        let first_candidate = candidates.first().map(|(p, _)| p.clone());
        candidates
            .into_iter()
            .max_by(|a, b| {
                let score_a = calculate_delivery_predictivity(
                    a.1.rssi,
                    a.1.latency_ms,
                    a.1.packet_loss,
                    a.1.trust_score,
                );
                let score_b = calculate_delivery_predictivity(
                    b.1.rssi,
                    b.1.latency_ms,
                    b.1.packet_loss,
                    b.1.trust_score,
                );
                score_a
                    .partial_cmp(&score_b)
                    .unwrap_or(std::cmp::Ordering::Equal)
            })
            .map(|(peer, _)| peer)
            .or(first_candidate)
            .expect("No candidates provided")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fallback_routing() {
        let peer1 = PeerId::random();
        let peer2 = PeerId::random();

        let candidates = vec![
            (
                peer1.clone(),
                RoutingFeatures {
                    rssi: -30, // Excellent signal
                    latency_ms: 50,
                    packet_loss: 0.01,
                    trust_score: 0.9,
                    hop_count: 1,
                    battery_level: 1.0,
                    time_of_day: 12,
                },
            ),
            (
                peer2.clone(),
                RoutingFeatures {
                    rssi: -80, // Poor signal
                    latency_ms: 300,
                    packet_loss: 0.3,
                    trust_score: 0.5,
                    hop_count: 5,
                    battery_level: 0.3,
                    time_of_day: 12,
                },
            ),
        ];

        let best = fallback::select_next_hop_formula(candidates);
        assert_eq!(best, peer1); // Should select better peer
    }

    #[test]
    fn test_prphet_formula() {
        // Excellent link: RSSI=-30, latency=50ms, loss=1%, trust=0.9
        // rssi=0.7, latency=0.95, loss=0.99 → score ≈ 0.873
        let score_good = fallback::calculate_delivery_predictivity(-30, 50, 0.01, 0.9);
        // Poor link: RSSI=-80, latency=300ms, loss=30%, trust=0.5
        // rssi=0.2, latency=0.7, loss=0.7 → score ≈ 0.51
        let score_poor = fallback::calculate_delivery_predictivity(-80, 300, 0.3, 0.5);

        assert!(
            score_good > score_poor,
            "Good link should score higher (good={}, poor={})",
            score_good,
            score_poor
        );
        assert!(
            score_good > 0.8,
            "Good link should be > 0.8, got {}",
            score_good
        );
        assert!(
            score_poor < 0.6,
            "Poor link should be < 0.6, got {}",
            score_poor
        );
    }
}
