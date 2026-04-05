# GhostWire — Working Memory
# Maintained by: AI agents (update after every session)
# Purpose: Cross-session continuity — what actually happened, not what was planned

---

## Current State (as of 2026-04-04, Session 3 — GNN TRAINING IN PROGRESS)

### Build Status
- ✅ `cargo build` — SUCCESS, ZERO errors
- ✅ `cargo check` — ZERO errors
- All 4 AI BLOCKERS: ✅ RESOLVED
- All 14 pre-existing errors: ✅ FIXED

### AI Module — Complete Architecture
```
Layer 1 (every routing decision, <1ms):
  OnnxRouter → ghostwire_routing.onnx (LightGBM, R²=0.9544)
  → Core::predict_delivery_probability(&LinkFeatures) → Option<f32>

Layer 2 (every 30s, 50-200ms):
  GemmaClient → Ollama → gemma4:e2b → mesh topology analysis

Layer 3 (every 5 min, offline) — IN PROGRESS:
  GNN (Graph Attention Network) — training on Lightning AI T4
  → Quick run: 69.2% val accuracy (1K samples, 20 epochs)
  → Full run: 10K samples, 100 epochs — RUNNING (~2 hours)

Layer 4 (always available, fallback):
  PRoPHET formula in ai::routing_classifier::fallback

Security (parallel):
  IsolationForestModel → Core::score_anomaly(&[f32]) → Option<AnomalyResult>
```

### GNN Training Status
- Studio: `ghostwire-gnn` on Lightning AI (T4 GPU, 15.6GB VRAM)
- Script: `/teamspace/studios/this_studio/train_gnn.py`
- Quick validation: 69.2% val acc, 56.5% test acc (1K samples, 20 epochs)
- Full training: 10K samples, 100 epochs — RUNNING
- Checkpoint: `models/gnn_routing/gnn_routing_best.pt` (3.1MB, updating)
- ONNX export: Needs fix (forward() signature issue with num_candidates/edge_mask)

### Key Fixes Applied to GNN Script
1. OUTPUT_DIR: Changed from `Path(__file__).parent.parent` to `Path("models/gnn_routing")`
2. `total_mem` → `total_memory` (PyTorch API change)
3. PyG DataLoader → torch.utils.data.DataLoader (PyG's DataLoader wraps custom collate)
4. GATConv forward: Rewrote for per-graph processing (GATConv needs 2D input)
5. Router input dim: `hidden_dim * 2` → `hidden_dim * 3` (src+dst+cand)
6. Device fix: `num_candidates.to(device)` for mask comparison

### What Doesn't Exist Yet
- GNN ONNX export (forward signature needs fixing for export)
- Real-world dataset collection
- LiteLLM proxy setup

---

## Resume Point

GNN training is running on Lightning AI Studio. Check status:
```python
from lightning_sdk import Studio
import os
os.environ['LIGHTNING_USER_ID'] = '1e36d0ef-707e-4312-a790-19e867b9c20a'
os.environ['LIGHTNING_API_KEY'] = '<current-key>'
os.environ['LIGHTNING_USERNAME'] = 'phantomojo'
s = Studio(name='ghostwire-gnn', teamspace='financial-llm-training-project', user='phantomojo')
s.run('ls -lh /teamspace/studios/this_studio/models/gnn_routing/')
```

After training completes:
1. Fix ONNX export (add export-friendly forward method)
2. Download model to local `models/gnn_routing.onnx`
3. Wire into Rust for periodic routing table optimization
