// GhostWire AI/ML Module
// ONNX Runtime integration + Gemma 4 local AI for mesh network intelligence
//
// Architecture (4-layer fallback chain):
//   Layer 1: OnnxRouter (LightGBM, 2,387 features) — anomaly detection (<1ms)
//   Layer 2: GemmaClient (Gemma 4 via Ollama/LiteRT) — mesh analysis (50-200ms)
//   Layer 3: GNN (PyTorch Geo) — routing optimization (offline, every 5 min)
//   Layer 4: PRoPHET (epidemic.rs) — always-on fallback (<0.01ms)
//
// Additional:
//   FunctionGemma — on-device tool calling (270M params, 32K context)
//   LiteRT-LM — native on-device inference backend (no Ollama required)
//   IsolationForest — anomaly detection from trained model
//   FeatureExtractor — live mesh telemetry to 2,387-dim vectors

pub mod routing_classifier;
pub mod gemma_client;
pub mod onnx_inference;
pub mod anomaly_detector;
pub mod feature_extractor;
pub mod function_calling;
pub mod litert_backend;

pub use routing_classifier::*;
pub use gemma_client::*;
pub use onnx_inference::*;
pub use anomaly_detector::*;
pub use feature_extractor::*;
pub use function_calling::*;
pub use litert_backend::*;
