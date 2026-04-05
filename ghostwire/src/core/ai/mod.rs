// GhostWire AI/ML Module
// ONNX Runtime integration + Gemma 4 local AI for mesh network intelligence

pub mod routing_classifier;
pub mod gemma_client;
pub mod onnx_inference;
pub mod anomaly_detector;
pub mod feature_extractor;

pub use routing_classifier::*;
pub use gemma_client::*;
pub use onnx_inference::*;
pub use anomaly_detector::*;
pub use feature_extractor::*;
