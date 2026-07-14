# GhostWire AI Routing - Qwen Handoff Document

**Generated:** 2026-04-03
**Project:** GhostWire Secure Mesh Communication
**Goal:** Improve AI routing model accuracy from 2.69% to 15-70%+ for GCD4F submission

---

## 🎯 IMMEDIATE CONTEXT

The user (Michael Irungu Muriithi / Phantomojo) is building GhostWire, a secure mesh communication system with AI-powered routing. The current routing model has **2.69% Top-1 accuracy** and needs to reach **15-70%+** for a competition submission (GCD4F).

**CRITICAL:** The user demands QUALITY FIRST - no shortcuts, no fallbacks, no stubs. Debug until fixed. Report honestly what works vs what's broken.

---

## 📁 PROJECT STRUCTURE

```
/home/ph/GhostWire-secure-mesh-communication/ghostwire/
├── src/
│   ├── core/
│   │   ├── mod.rs              # Core struct - has dtn_store field
│   │   └── ai/                 # AI module (ORPHANED - not wired in!)
│   │       ├── mod.rs          # References anomaly_detector, trust_predictor (MISSING FILES)
│   │       ├── onnx_inference.rs  # OnnxInferenceEngine using ort crate
│   │       └── routing_classifier.rs # RoutingClassifier, RoutingFeatures (7 fields)
│   └── ...
├── models/
│   ├── hrm_ghostwire_fixed.onnx    # Wrong input shape [batch, 64] token IDs
│   ├── hrm_ghostwire_hybrid_101k.pt # 7.9MB, 2.69% accuracy
│   └── hrm_lora_adapter.pt        # 101KB LoRA adapter
├── scripts/
│   ├── gnn_routing.py          # MeshGNN with GAT layers (PyG not installed)
│   └── export_hrm_to_onnx.py   # Exports old token-based model
└── Cargo.toml
```

---

## 🚨 CRITICAL BLOCKERS (Fix in Order)

### 1. AI Module is Orphaned
- `src/core/ai/` exists but `pub mod ai;` is **NOT** in `src/core/mod.rs`
- Rust compiler doesn't see the AI module at all
- **Fix:** Add `pub mod ai;` to `src/core/mod.rs`

### 2. Missing Rust Files
- `src/core/ai/mod.rs` references:
  - `anomaly_detector.rs` - **DOES NOT EXIST**
  - `trust_predictor.rs` - **DOES NOT EXIST**
- **Fix:** Create stub files or remove references

### 3. Model Interface Mismatch
- **Current model expects:** token IDs `[batch, 64]` → 200 classes
- **Rust expects:** 7 features → probability score
- **7 features:** RSSI, latency, packet_loss, trust_score, hop_count, battery_level, time_since_update
- **Fix:** Retrain model with correct input/output shape

### 4. PyTorch Geometric Not Installed
- `scripts/gnn_routing.py` exits immediately with `HAS_PYG = False`
- **Fix:** Install PyTorch Geometric and dependencies

### 5. No Real Telemetry Data
- Training data is synthetic/simulated
- **Fix:** Deploy test nodes for real telemetry collection

---

## 🏗️ ARCHITECTURE OVERVIEW

### 3-Layer AI Routing System

```
Layer 1: PRoPHET/DTN (Store-and-Forward)
  - Lindgren et al. 2003 protocol
  - Delivery predictability with encounter, decay, transitivity
  - Spray-and-Wait implementation
  - Located in: src/core/dtn/epidemic.rs

Layer 2: ML Routing Classifier (Real-time Next-Hop)
  - 7 features → probability score
  - Located in: src/core/ai/routing_classifier.rs
  - Uses ONNX runtime (ort crate)

Layer 3: GNN + DRL (Optimal Routing)
  - Graph Attention Networks (GAT)
  - Deep Reinforcement Learning (PPO)
  - Located in: scripts/gnn_routing.py
```

### Key Files

| File | Purpose | Status |
|------|---------|--------|
| `src/core/dtn/epidemic.rs` | PRoPHET + Spray-and-Wait | ✅ Working |
| `src/core/dtn/bundle.rs` | DtnBundle with CID, expiry, copy limits | ✅ Working |
| `src/core/ai/onnx_inference.rs` | OnnxInferenceEngine | ⚠️ Exists but not wired |
| `src/core/ai/routing_classifier.rs` | RoutingClassifier, RoutingFeatures | ⚠️ Exists but not wired |
| `src/core/ai/mod.rs` | Module definition | ❌ References missing files |
| `src/core/mod.rs` | Core module | ❌ Missing `pub mod ai;` |
| `models/hrm_ghostwire_fixed.onnx` | ONNX model | ❌ Wrong input shape |
| `scripts/gnn_routing.py` | GNN training | ❌ PyG not installed |

---

## 🔧 NEXT STEPS (Priority Order)

### Phase 1: Wire in AI Module (1-2 hours)
1. Add `pub mod ai;` to `src/core/mod.rs`
2. Create `src/core/ai/anomaly_detector.rs` (stub)
3. Create `src/core/ai/trust_predictor.rs` (stub)
4. Verify compilation with `cargo check`

### Phase 2: Retrain Model (4-6 hours)
1. Create proper training dataset with 7 features
2. Train model: `[batch, 7]` → delivery probability score
3. Export to ONNX with correct input shape `[1, 7]`
4. Save to `models/routing_classifier_v2.onnx`

### Phase 3: Test Rust ↔ ONNX Integration (2-3 hours)
1. Load new ONNX model in Rust
2. Run inference with test data
3. Verify output matches expected format
4. Integrate with routing classifier

### Phase 4: GNN + DRL (8-12 hours)
1. Install PyTorch Geometric
2. Generate mesh network dataset
3. Train MeshGNN with GAT layers
4. Implement PPO agent with reward function
5. Export GNN model to ONNX

### Phase 5: Real Telemetry (Ongoing)
1. Deploy test nodes on Raspberry Pi/Android
2. Collect real RSSI, latency, packet loss data
3. Retrain models with real data
4. Validate accuracy improvements

---

## 📝 KEY TECHNICAL DETAILS

### PRoPHET Formula (from epidemic.rs)
```rust
// Delivery predictability update on encounter
P_encounter = P_old + (1 - P_old) * P_init

// Aging/decay over time
P_aged = P_old * 2^(-K * time_since_encounter)

// Transitivity (friend of friend)
P_transitive = P_old + (1 - P_old) * P_encounter * P_other * beta
```

### Routing Features (7 fields)
```rust
struct RoutingFeatures {
    rssi: f32,           // Signal strength (-100 to 0 dBm)
    latency: f32,        // Round-trip time (ms)
    packet_loss: f32,    // Loss rate (0.0 to 1.0)
    trust_score: f32,    // Node trust (0.0 to 1.0)
    hop_count: f32,      // Hops to destination
    battery_level: f32,  // Battery (0.0 to 1.0)
    time_since_update: f32, // Seconds since last update
}
```

### Current Model Interface (WRONG)
- Input: `[batch, 64]` token IDs
- Output: 200 classes (softmax)
- This is a **text classification model**, not a routing model!

### Required Model Interface (CORRECT)
- Input: `[batch, 7]` float features
- Output: `[batch, 1]` delivery probability (sigmoid)

---

## 🛠️ TOOLS & DEPENDENCIES

### Rust (Cargo)
- `ort` crate for ONNX runtime
- `ndarray` for tensor operations
- `serde` for serialization

### Python
- PyTorch (installed)
- PyTorch Geometric (**NOT installed**)
- ONNX export tools
- scikit-learn for metrics

### Key Paths
- Project: `/home/ph/GhostWire-secure-mesh-communication/ghostwire/`
- Models: `/home/ph/GhostWire-secure-mesh-communication/ghostwire/models/`
- Scripts: `/home/ph/GhostWire-secure-mesh-communication/ghostwire/scripts/`

---

## 📊 CURRENT METRICS

| Metric | Value | Target |
|--------|-------|--------|
| Top-1 Accuracy | 2.69% | 15-70%+ |
| Model Size | 7.9MB | <10MB |
| Parameters | 537K | <1M |
| Inference Time | Unknown | <50ms |
| Training Data | Synthetic | Real + Synthetic |

---

## ⚠️ KNOWN ISSUES

1. **Model trained on wrong task:** 200-class classification instead of delivery probability scoring
2. **No real telemetry:** All training data is synthetic
3. **Missing Rust files:** anomaly_detector.rs, trust_predictor.rs
4. **AI module orphaned:** Not wired into core module
5. **PyG not installed:** GNN script can't run
6. **No Bluetooth transport:** DTN layer needs Bluetooth for real mesh

---

## 💡 NOTES FOR NEXT QWEN

- The user is technically sophisticated - don't oversimplify
- They have a 6+ hour hyperfocus work style
- They prefer direct, honest communication about what works vs what's broken
- They're working on a competition deadline (GCD4F)
- The PRoPHET implementation in `epidemic.rs` is solid - don't break it
- The ONNX inference engine in `onnx_inference.rs` is well-structured but needs the right model
- Focus on getting the model interface right first, then worry about accuracy improvements

---

## 📞 USER CONTEXT (from QWEN.md)

- Name: Michael Irungu Muriithi (Phantomojo)
- Location: Nairobi, Kenya
- GitHub: Phantomojo
- Email: [removed for public release]
- Work style: 6+ hour hyperfocus blocks, uses AI agents extensively
- Demands: Quality first, no shortcuts, debug until fixed

---

*End of handoff document. Pick up from Phase 1: Wire in AI Module.*
