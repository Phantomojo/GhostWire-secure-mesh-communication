---
name: ghostwire-ml-training
description: >
  Use when working on GhostWire's ML model training, dataset preparation,
  LightGBM/XGBoost routing models, GNN training, ONNX export, or free GPU
  compute workflows. Triggers on: "train", "LightGBM", "XGBoost", "GNN",
  "ONNX export", "Kaggle", "Colab", "Lightning AI", "101K dataset",
  "routing accuracy", "HRM model", "PyTorch Geometric".
---

# GhostWire ML Training Skill

## The Problem Statement

Current HRM routing model accuracy: **2.69%**
Root cause: Text classification transformer treating float network features as token IDs.
This is architecturally wrong. The correct model:
- Input: 7 numerical features (rssi, latency_ms, packet_loss, battery_level, hop_count, encounter_frequency, time_since_last_seen)
- Output: delivery probability (0.0–1.0)
- Model type: LightGBM regression
- Expected accuracy: 85–95%

## Free GPU Resources (use these in order)

| Platform | GPU | VRAM | Hours | Session | Credit Card? |
|---|---|---|---|---|---|
| Kaggle | P100 | 16GB | 30/wk | 12hr | No |
| Google Colab | T4 | 16GB | 15–30/wk | 12hr | No |
| Lightning AI | L40s | 48GB | 80/mo | 4hr | No |

Strategy: Kaggle for main training (30hr/wk), Colab for quick experiments.
Lightning AI for heavy runs (GNN, future Gemma fine-tuning).

## Dataset Preparation

The 101K dataset is currently in token format (wrong). Must convert:

```python
# Kaggle notebook — Dataset Conversion
import pandas as pd
import numpy as np

# Load existing dataset (adjust path for Kaggle)
df = pd.read_csv('/kaggle/input/ghostwire/routing_dataset.csv')

# The 7 features we actually need
FEATURES = [
    'rssi',              # signal strength (float, typically -100 to 0 dBm)
    'latency_ms',        # round-trip latency (float, ms)
    'packet_loss',       # packet loss rate (float, 0.0 to 1.0)
    'battery_level',     # node battery (float, 0.0 to 1.0)
    'hop_count',         # number of hops (int)
    'encounter_frequency', # how often nodes meet (float, encounters/hour)
    'time_since_last_seen', # seconds since last contact (float)
]

TARGET = 'delivery_probability'  # 0.0 to 1.0

X = df[FEATURES]
y = df[TARGET]

print(f"Dataset shape: {X.shape}")
print(f"Target range: {y.min():.3f} – {y.max():.3f}")
print(f"Target mean: {y.mean():.3f}")
```

## LightGBM Training Script

```python
# Kaggle notebook — LightGBM Training
import lightgbm as lgb
from sklearn.model_selection import train_test_split
from sklearn.metrics import mean_squared_error
import numpy as np

X_train, X_val, y_train, y_val = train_test_split(X, y, test_size=0.2, random_state=42)

params = {
    'objective': 'regression',
    'metric': 'rmse',
    'num_leaves': 127,
    'learning_rate': 0.05,
    'feature_fraction': 0.9,
    'bagging_fraction': 0.8,
    'bagging_freq': 5,
    'n_estimators': 1000,
    'early_stopping_rounds': 50,
    'verbose': -1,
}

train_data = lgb.Dataset(X_train, label=y_train)
val_data = lgb.Dataset(X_val, label=y_val)

model = lgb.train(
    params,
    train_data,
    valid_sets=[val_data],
    callbacks=[lgb.log_evaluation(100)]
)

preds = model.predict(X_val)
rmse = np.sqrt(mean_squared_error(y_val, preds))
print(f"Validation RMSE: {rmse:.4f}")
print(f"Approximate accuracy proxy: {(1 - rmse):.1%}")

# Save model
model.save_model('ghostwire_routing_lgbm.txt')
```

## ONNX Export

```python
# Convert LightGBM to ONNX for Rust inference
from skl2onnx import convert_sklearn
from skl2onnx.common.data_types import FloatTensorType
import lightgbm as lgb
from onnxmltools import convert_lightgbm
from onnxmltools.utils import save_model

model = lgb.Booster(model_file='ghostwire_routing_lgbm.txt')

# onnxmltools approach for LightGBM
onnx_model = convert_lightgbm(
    model,
    initial_types=[('input', FloatTensorType([None, 7]))],
    target_opset=12
)
save_model(onnx_model, 'ghostwire_routing.onnx')
print(f"ONNX model saved. Size: {os.path.getsize('ghostwire_routing.onnx') / 1024:.1f} KB")
# Expected: ~200–500 KB (vs 7.9MB for the broken HRM model)
```

## Loading ONNX in Rust (onnx_inference.rs)

```rust
use ort::{Environment, Session, SessionBuilder, Value};
use ndarray::Array2;

pub struct OnnxRouter {
    session: Session,
}

impl OnnxRouter {
    pub fn new(model_path: &str) -> anyhow::Result<Self> {
        let environment = Environment::builder()
            .with_name("ghostwire_routing")
            .build()?
            .into_arc();
        
        let session = SessionBuilder::new(&environment)?
            .with_model_from_file(model_path)?;
        
        Ok(Self { session })
    }

    pub fn predict(&self, features: &RoutingFeatures) -> anyhow::Result<f32> {
        let input = Array2::from_shape_vec(
            (1, 7),
            vec![
                features.rssi,
                features.latency_ms,
                features.packet_loss,
                features.battery_level,
                features.hop_count as f32,
                features.encounter_frequency,
                features.time_since_last_seen,
            ]
        )?;
        
        let outputs = self.session.run(ort::inputs!["input" => input.view()]?)?;
        let probability = outputs["output"].try_extract_tensor::<f32>()?
            .view()
            .iter()
            .next()
            .copied()
            .unwrap_or(0.5);
        
        Ok(probability.clamp(0.0, 1.0))
    }
}
```

## GNN Training (Phase 3 — after LightGBM is working)

```bash
# On Lightning AI (48GB VRAM — needed for PyTorch Geometric)
pip install torch-geometric torch-scatter torch-sparse

python gnn_routing.py \
    --data ghostwire_mesh_graphs.pkl \
    --epochs 200 \
    --hidden-dim 128 \
    --num-heads 4 \
    --output gnn_routing.onnx
```

The GNN handles topology-aware routing (which nodes are connected to which).
LightGBM handles per-link delivery probability.
They work together, not in competition.

## Validation Before Committing a Model

```python
# Minimum validation before any model goes into production
assert rmse < 0.15, f"RMSE too high: {rmse}"
assert (preds >= 0).all() and (preds <= 1).all(), "Predictions out of [0,1] range"
assert model.predict(X_val[:1]).shape == (1,), "Wrong output shape"

# Latency test
import time
start = time.perf_counter()
for _ in range(1000):
    model.predict(X_val[:1])
latency_ms = (time.perf_counter() - start)
assert latency_ms < 1.0, f"Inference too slow: {latency_ms:.3f}ms per call"
print(f"✅ Inference latency: {latency_ms:.3f}ms per 1000 calls ({latency_ms/1000*1000:.3f}μs each)")
```
