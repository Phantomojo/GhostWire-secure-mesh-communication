---
name: ghostwire-ai-layer
description: >
  Use this skill when working on GhostWire's AI/ML integration layer:
  Gemma 4 client, ONNX inference, LightGBM routing, anomaly detection,
  trust scoring, or the four-layer routing architecture. Triggers on:
  "ai module", "gemma", "onnx", "routing classifier", "anomaly detector",
  "trust predictor", "LightGBM", "GNN", "PRoPHET wiring".
  Do NOT use for non-AI Rust modules or Python training scripts.
---

# GhostWire AI Layer — Agent Skill

## What This Skill Covers

The AI module lives at `src/core/ai/` and integrates into `src/core/mod.rs`.
This skill provides the precise context needed to work on it correctly.

## Module Structure

```
src/core/ai/
├── mod.rs              — module exports (gemma_client active, onnx commented out)
├── gemma_client.rs     — Ollama HTTP client for Gemma 4 (280 lines, complete)
├── routing_classifier.rs — ML routing classifier (exists, partially wired)
├── onnx_inference.rs   — ONNX runtime (BROKEN: ort import error, commented out)
├── anomaly_detector.rs — DUPLICATE of anomaly_detector.joblib — DELETE THIS
└── trust_predictor.rs  — DUPLICATE of reputation.rs — DELETE THIS
```

## The Right Way to Initialize GemmaClient

The current implementation is broken. The correct pattern:

```rust
// In Core struct definition (core/mod.rs)
pub struct Core {
    // ... existing fields ...
    pub gemma_client: Arc<RwLock<Option<GemmaClient>>>,
    pub anomaly_detector: Arc<IsolationForest>,  // loaded from .joblib via PyO3 or ONNX
}

// In Core::new()
gemma_client: Arc::new(RwLock::new(None)),  // initialized later

// In start_nervous_system()
let gemma_arc = Arc::clone(&self.gemma_client);
let model = self.config.features.gemma_model_tag(); // "gemma4:e2b" etc
tokio::spawn(async move {
    // Wait for system to stabilise
    tokio::time::sleep(Duration::from_secs(5)).await;
    let client = GemmaClient::new("http://localhost:11434", &model);
    if client.health_check().await {
        *gemma_arc.write().await = Some(client);
        tracing::info!("✅ Gemma 4 AI online: {}", model);
    } else {
        tracing::warn!("⚠️  Ollama not reachable — running AI-free mode");
    }

    // Periodic mesh analysis loop
    let mut interval = tokio::time::interval(Duration::from_secs(30));
    loop {
        interval.tick().await;
        if let Some(client) = gemma_arc.read().await.as_ref() {
            // collect telemetry and call analyze_mesh()
        }
    }
});
```

## Fixing onnx_inference.rs

The comment says `ort::Value` import fails. The ort crate (OnnxRuntime) changed
its API in v2.0. The likely fix:

```rust
// OLD (ort 1.x — broken)
use ort::Value;

// NEW (ort 2.x — correct)
use ort::session::SessionOutputs;
use ort::inputs;
// OR check the specific version in Cargo.toml and match docs.rs exactly
```

Before fixing: run `cargo metadata | grep '"ort"'` to find the exact version,
then look up that version's API on docs.rs/ort.

## Wiring anomaly_detector.joblib into Rust

The joblib file is a scikit-learn Isolation Forest. Options:
1. **Convert to ONNX** (recommended): `skl2onnx` in Python → load via ort
2. **PyO3 FFI**: call Python from Rust (adds complexity, avoid if possible)
3. **Re-implement in Rust**: overkill for Isolation Forest

Recommended path:
```bash
# In Python (run once, save the ONNX model)
pip install skl2onnx
python -c "
import joblib
from skl2onnx import convert_sklearn
from skl2onnx.common.data_types import FloatTensorType
clf = joblib.load('models/anomaly_detector.joblib')
initial_type = [('float_input', FloatTensorType([None, 7]))]
onx = convert_sklearn(clf, initial_type=initial_type)
with open('models/anomaly_detector.onnx', 'wb') as f:
    f.write(onx.SerializeToString())
"
# Then load models/anomaly_detector.onnx via onnx_inference.rs
```

## Verification Checklist After Any AI Module Change

```bash
cargo check -p ghostwire 2>&1 | head -30        # fast check
cargo build 2>&1 | grep "^error" | head -20     # full build errors only
cargo test ai:: 2>&1 | tail -20                 # AI tests only
cargo clippy -- -D warnings 2>&1 | head -30     # lint
```

All four must pass before declaring any AI module task complete.

## Ollama Verification

```bash
# Check if Ollama is running
curl -s http://localhost:11434/api/tags | python3 -m json.tool

# Check if gemma4:e2b is downloaded
ollama list | grep gemma4

# Test inference (takes 2-5 seconds on RTX 3050)
ollama run gemma4:e2b "Respond with: ONLINE" --nowordwrap
```

If Ollama is not running:
```bash
OLLAMA_HOST=0.0.0.0:11434 ollama serve &
```
