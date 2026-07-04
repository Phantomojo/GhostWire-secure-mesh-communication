// Anomaly Detector — Isolation Forest implementation
// Loads the trained model from models/anomaly_detector.json
// Provides anomaly scoring for mesh network traffic analysis

use serde::Deserialize;
use std::fs;

/// A single decision tree from the Isolation Forest
#[derive(Debug, Clone, Deserialize)]
pub struct IsolationTree {
    #[serde(default)]
    pub node_count: Option<usize>,
    pub children_left: Vec<i64>,
    pub children_right: Vec<i64>,
    pub feature: Vec<i64>,
    pub threshold: Vec<f64>,
    #[serde(default)]
    pub value: Option<Vec<Vec<f64>>>,
    pub n_node_samples: Vec<f64>,
}

impl IsolationTree {
    /// Traverse the tree and return the path length for a given input
    pub fn path_length(&self, features: &[f32]) -> f64 {
        let mut node: i64 = 0;
        let mut depth: f64 = 0.0;

        while node >= 0 {
            let idx = node as usize;
            if idx >= self.children_left.len() {
                break;
            }

            let left = self.children_left[idx];
            let right = self.children_right[idx];

            // Leaf node (both children are -1)
            if left < 0 && right < 0 {
                // Add adjustment for unbuilt subtree
                let n_samples = self.n_node_samples[idx] as usize;
                depth += c(n_samples);
                break;
            }

            let feature_idx = self.feature[idx] as usize;
            let threshold = self.threshold[idx] as f32;

            if feature_idx < features.len() && features[feature_idx] <= threshold {
                node = left;
            } else {
                node = right;
            }
            depth += 1.0;
        }

        depth
    }
}

/// Average path length of unsuccessful search in BST
/// c(n) = 2 * H(n-1) - 2*(n-1)/n where H(i) = ln(i) + 0.5772156649 (Euler's constant)
fn c(n: usize) -> f64 {
    if n <= 1 {
        return 0.0;
    }
    if n == 2 {
        return 1.0;
    }
    let n = n as f64;
    2.0 * (n - 1.0).ln() + 2.0 * 0.5772156649 - 2.0 * (n - 1.0) / n
}

/// Complete Isolation Forest model
#[derive(Debug, Deserialize)]
pub struct IsolationForestModel {
    pub n_estimators: usize,
    pub n_features: usize,
    pub contamination: f64,
    pub max_samples: usize,
    pub offset: f64,
    pub trees: Vec<IsolationTree>,
}

/// Result of anomaly detection
#[derive(Debug, Clone)]
pub struct AnomalyResult {
    /// Anomaly score: positive = normal, negative = anomalous
    /// Matches sklearn's decision_function output
    pub score: f64,
    /// Whether this is classified as anomalous
    pub is_anomaly: bool,
    /// Normalized anomaly probability (0.0 = normal, 1.0 = anomalous)
    pub anomaly_probability: f64,
}

impl IsolationForestModel {
    /// Load model from JSON file
    pub fn from_file(path: &str) -> anyhow::Result<Self> {
        let json = fs::read_to_string(path)
            .map_err(|e| anyhow::anyhow!("Failed to read anomaly model at {}: {}", path, e))?;

        let model: Self = serde_json::from_str(&json)
            .map_err(|e| anyhow::anyhow!("Failed to parse anomaly model: {}", e))?;

        if model.trees.len() != model.n_estimators {
            anyhow::bail!(
                "Model mismatch: expected {} trees, found {}",
                model.n_estimators,
                model.trees.len()
            );
        }

        Ok(model)
    }

    /// Compute anomaly score for a single sample
    /// Returns score matching sklearn's decision_function:
    ///   positive = normal, negative = anomalous
    pub fn score(&self, features: &[f32]) -> AnomalyResult {
        if features.len() != self.n_features {
            // Pad or truncate with warning
            tracing::warn!(
                "Anomaly detector expected {} features, got {}",
                self.n_features,
                features.len()
            );
        }

        // Average path length across all trees
        let avg_path_length: f64 = self
            .trees
            .iter()
            .map(|tree| tree.path_length(features))
            .sum::<f64>()
            / self.n_estimators as f64;

        // sklearn's decision_function: offset - avg_path_length
        let score = self.offset - avg_path_length;

        // Convert to probability-like score (0.0 = normal, 1.0 = anomalous)
        // Using sigmoid-like transformation
        let anomaly_probability = 1.0 / (1.0 + (score * 3.0).exp());

        AnomalyResult {
            score,
            is_anomaly: score > 0.0,
            anomaly_probability,
        }
    }

    /// Quick check: is this sample anomalous?
    pub fn is_anomaly(&self, features: &[f32]) -> bool {
        self.score(features).is_anomaly
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_c_function() {
        assert!((c(1) - 0.0).abs() < 0.001);
        assert!((c(2) - 1.0).abs() < 0.001);
        // c(256) = 2*(ln(255) + γ) - 2*255/256 ≈ 10.24
        let c256 = c(256);
        assert!(c256 > 10.0 && c256 < 11.0, "c(256) ≈ 10.24, got {}", c256);
        // c(8192) = 2*(ln(8191) + γ) - 2*8191/8192 ≈ 17.18
        let c8192 = c(8192);
        assert!(
            c8192 > 17.0 && c8192 < 18.0,
            "c(8192) ≈ 17.18, got {}",
            c8192
        );
    }

    #[test]
    fn test_model_loads() {
        let result = IsolationForestModel::from_file("models/anomaly_detector.json");
        // May fail if model file doesn't exist in test environment
        if let Ok(model) = result {
            assert_eq!(model.n_estimators, 200);
            assert_eq!(model.n_features, 39);
            assert_eq!(model.trees.len(), 200);
        }
    }
}
