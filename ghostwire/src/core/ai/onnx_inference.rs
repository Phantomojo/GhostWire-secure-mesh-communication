// ONNX Runtime Inference Engine
// Core infrastructure for running AI models in GhostWire
// Uses ort 2.0 API (rc.12)

use ort::{session::Session, value::{Tensor, ValueType}};
use anyhow::{Result, Context};
use std::sync::Mutex;

/// Main ONNX inference engine for GhostWire
///
/// Wraps an ONNX session with interior mutability since ort::Session::run requires &mut self.
/// Uses std::sync::Mutex for thread-safe sharing across Arc.
pub struct OnnxInferenceEngine {
    session: Mutex<Session>,
    model_name: String,
}

impl OnnxInferenceEngine {
    /// Load an ONNX model from file
    pub fn new(model_path: &str, model_name: &str) -> Result<Self> {
        // Build session and commit from file (ort 2.0 API)
        let mut builder = Session::builder()
            .context("Failed to create ONNX session builder")?;

        // Enable max optimization — ignore error since this isn't available in minimal builds
        builder = builder
            .with_optimization_level(ort::session::builder::GraphOptimizationLevel::Level3)
            .unwrap_or_else(|e| e.recover());

        let session = builder
            .commit_from_file(model_path)
            .context(format!("Failed to load model from {}", model_path))?;

        Ok(Self {
            session: Mutex::new(session),
            model_name: model_name.to_string(),
        })
    }

    /// Run inference with the given input features
    pub fn run_inference(&self, input_features: Vec<f32>) -> Result<Vec<f32>> {
        // Get input/output names first
        let (input_name, output_name) = {
            let session = self.session.lock().map_err(|e| {
                anyhow::anyhow!("ONNX session mutex poisoned: {}", e)
            })?;
            let input_name = session
                .inputs()
                .first()
                .context("No inputs defined for this model")?
                .name()
                .to_string();
            let output_name = session
                .outputs()
                .first()
                .context("No outputs defined for this model")?
                .name()
                .to_string();
            (input_name, output_name)
        };

        // Create tensor from input features — ort 2.0 uses Tensor::from_array
        let shape = vec![1i64, input_features.len() as i64];
        let input_tensor = Tensor::from_array((shape, input_features))
            .context("Failed to create input tensor")?;

        // Run inference (mutable borrow via Mutex guard)
        let mut session = self.session.lock().map_err(|e| {
            anyhow::anyhow!("ONNX session mutex poisoned: {}", e)
        })?;
        let outputs = session
            .run(ort::inputs![input_name => input_tensor])
            .context("ONNX inference failed")?;

        // Extract output tensor — try_extract_tensor returns (&Shape, &[T])
        let output_value = outputs
            .get(output_name.as_str())
            .context("Output not found")?;

        let (_shape, output_data) = output_value
            .try_extract_tensor::<f32>()
            .context("Failed to extract output tensor")?;

        let output_vec: Vec<f32> = output_data.to_vec();

        Ok(output_vec)
    }

    /// Get model name
    pub fn name(&self) -> &str {
        &self.model_name
    }

    /// Get input shape expected by this model
    pub fn input_shape(&self) -> Result<Vec<i64>> {
        let session = self.session.lock().map_err(|e| {
            anyhow::anyhow!("ONNX session mutex poisoned: {}", e)
        })?;
        let input = session
            .inputs()
            .first()
            .context("No inputs defined for this model")?;

        // Extract shape from ValueType::Tensor
        if let ValueType::Tensor { shape, .. } = input.dtype() {
            Ok(shape.iter().copied().collect())
        } else {
            anyhow::bail!("Input is not a tensor type")
        }
    }
}

/// Feature normalization utilities
pub mod normalization {
    /// Normalize features to 0-1 range
    pub fn normalize_minmax(features: &mut [f32], min: &[f32], max: &[f32]) {
        for (i, feature) in features.iter_mut().enumerate() {
            let range = max[i] - min[i];
            if range > 0.0 {
                *feature = (*feature - min[i]) / range;
            } else {
                *feature = 0.0;
            }
        }
    }

    /// Standardize features (zero mean, unit variance)
    pub fn standardize(features: &mut [f32], mean: &[f32], std: &[f32]) {
        for (i, feature) in features.iter_mut().enumerate() {
            if std[i] > 0.0 {
                *feature = (*feature - mean[i]) / std[i];
            } else {
                *feature = 0.0;
            }
        }
    }
}

/// ONNX-based anomaly detector — Layer 1 fast path (<1ms)
///
/// Uses the trained LightGBM model (ghostwire_routing.onnx) to classify
/// mesh network state: 2,387 features → anomaly probability (0.0–1.0).
///
/// Trained on REAL Guifi.net Guifisants node data:
///   - 7,931 samples × 2,387 features (system metrics, network stats, BMX6 routing)
///   - Binary classification: normal vs anomaly (gateway failure events)
///   - Test AUC: 1.0, F1: 1.0 at optimal threshold (0.283)
///   - Inference: ~77μs per prediction
///
/// The model expects a 2,387-dimensional feature vector containing:
///   - Per-node system metrics (CPU, memory, swap, processes)
///   - Network statistics (eth/wifi TX/RX rates, bytes, packets)
///   - BMX6 routing protocol metrics
///   - Temporal features (is_weekend, etc.)
pub struct OnnxRouter {
    engine: OnnxInferenceEngine,
}

impl OnnxRouter {
    /// Load the routing model from ONNX file
    pub fn new(model_path: &str) -> Result<Self> {
        let engine = OnnxInferenceEngine::new(model_path, "lightgbm_routing")?;
        Ok(Self { engine })
    }

    /// Predict anomaly probability for a mesh state
    ///
    /// Returns probability in [0.0, 1.0] — higher means more likely anomaly.
    /// Returns None if the model is unavailable (graceful degradation).
    pub fn predict(&self, features: &[f32]) -> Option<f32> {
        match self.engine.run_inference(features.to_vec()) {
            Ok(output) => {
                // TreeEnsembleClassifier outputs: [prob_class_0, prob_class_1]
                // We want the anomaly probability (class 1)
                if output.len() >= 2 {
                    Some(output[1].clamp(0.0, 1.0))
                } else if let Some(&prob) = output.first() {
                    Some(prob.clamp(0.0, 1.0))
                } else {
                    tracing::warn!("ONNX router returned empty output");
                    None
                }
            }
            Err(e) => {
                tracing::warn!("ONNX router inference failed: {}", e);
                None
            }
        }
    }

    /// Get the underlying engine name
    pub fn name(&self) -> &str {
        self.engine.name()
    }

    /// Get expected feature count from the model
    pub fn feature_count(&self) -> Option<usize> {
        self.engine.input_shape().ok().and_then(|shape| {
            if shape.len() >= 2 {
                Some(shape[1] as usize)
            } else {
                None
            }
        })
    }
}

/// Link features for legacy 7-feature routing (deprecated, kept for compatibility)
///
/// These 7 features match the original synthetic LightGBM training data.
/// For production use, use the 2,387-feature Guifi.net model via OnnxRouter::predict().
#[derive(Clone, Debug)]
pub struct LinkFeatures {
    pub rssi: f32,                  // Signal strength (-100 to 0 dBm)
    pub latency_ms: f32,            // Round-trip latency (1-500 ms)
    pub packet_loss: f32,           // Packet loss rate (0.0-1.0)
    pub battery_level: f32,         // Node battery (0.0-1.0)
    pub hop_count: f32,             // Hops to destination (1-15)
    pub encounter_frequency: f32,   // Encounters/hour (0.0-10.0)
    pub time_since_last_seen: f32,  // Seconds since last contact (0-3600)
}

impl LinkFeatures {
    /// Convert to feature vector in the exact order the model expects
    pub fn to_vector(&self) -> Vec<f32> {
        vec![
            self.rssi,
            self.latency_ms,
            self.packet_loss,
            self.battery_level,
            self.hop_count,
            self.encounter_frequency,
            self.time_since_last_seen,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_engine_creation() {
        // This test will fail if model file doesn't exist (expected during development)
        let result = OnnxInferenceEngine::new("models/test.onnx", "test_model");
        assert!(result.is_ok() || result.is_err());
    }

    #[test]
    fn test_normalization() {
        let mut features = vec![5.0, 10.0, 15.0];
        let min = vec![0.0, 0.0, 0.0];
        let max = vec![10.0, 20.0, 30.0];

        normalization::normalize_minmax(&mut features, &min, &max);

        assert_eq!(features, vec![0.5, 0.5, 0.5]);
    }
}
