<!--
  Copyright 2025-2026 Michael Irungu and GhostWire Contributors
  SPDX-License-Identifier: Apache-2.0

  Licensed under the Apache License, Version 2.0 (the "License");
  you may not use this file except in compliance with the License.
  You may obtain a copy of the License at

      http://www.apache.org/licenses/LICENSE-2.0

  Unless required by applicable law or agreed to in writing, software
  distributed under the License is distributed on an "AS IS" BASIS,
  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
  See the License for the specific language governing permissions and
  limitations under the License.

  AI Training Restriction: This specification may not be used for training,
  fine-tuning, or otherwise developing machine learning models, large language
  models, or artificial intelligence systems without prior written permission
  from the copyright holder.
-->

# AI Routing Specification

**GIP:** Pending
**Version:** 0.2.0-draft
**Date:** April 12, 2026

---

## Overview

This specification defines the four-layer AI-powered routing system used by GhostWire to select optimal paths through the mesh network. The routing system uses machine learning models trained on real mesh telemetry data to predict link quality, latency, and delivery probability. The four layers form a cascading fallback: if a higher layer is unavailable or fails, the next layer takes over.

```
L1: LightGBM (2387-dim features, <1ms, ONNX)
  ↓ fallback
L2: Gemma 4 (270M params, 32K context, Ollama/LiteRT)
  ↓ fallback
L3: GNN (4-layer GAT, 8 heads, ONNX, 91μs)
  ↓ fallback
L4: PRoPHET (epidemic routing, delivery predictability)
```

---

## Layer 1: LightGBM

### Model Specification

| Property | Value |
|----------|-------|
| **Algorithm** | LightGBM (Gradient Boosting Decision Trees) |
| **Format** | ONNX (Open Neural Network Exchange) |
| **Input dimensions** | 2387 features |
| **Output** | Delivery probability score (0.0 to 1.0) |
| **Inference time** | <1ms on CPU |
| **Model size** | ~2-5 MB (depending on tree depth) |

### Training Data

| Property | Value |
|----------|-------|
| **Source** | GuifiSants mesh network |
| **Samples** | 7,931 |
| **Nodes** | 63 |
| **Duration** | 31 days |
| **Features** | Mesh telemetry (link quality, latency, packet loss, node uptime, geographic distance, time of day) |

### Feature Extraction

Features are extracted from mesh telemetry and organized into categories:

| Category | Feature Count | Examples |
|----------|--------------|----------|
| **Link quality** | 256 | SNR, RSSI, packet loss rate, retransmission count |
| **Latency** | 128 | RTT, jitter, one-way delay, queue depth |
| **Node health** | 64 | Uptime, CPU usage, memory usage, buffer occupancy |
| **Topology** | 512 | Degree, betweenness centrality, clustering coefficient, path length |
| **Temporal** | 256 | Hour of day, day of week, season, recent activity patterns |
| **Geographic** | 128 | Distance, elevation, line-of-sight probability, urban/rural |
| **Historical** | 512 | Rolling averages (1h, 6h, 24h, 7d) of all above metrics |
| **Derived** | 531 | Interaction terms, polynomial features, lag features |

**Feature vector construction:**
```
feature_vector = concatenate([
    link_quality_features,      // 256 dims
    latency_features,           // 128 dims
    node_health_features,       // 64 dims
    topology_features,          // 512 dims
    temporal_features,          // 256 dims
    geographic_features,        // 128 dims
    historical_features,        // 512 dims
    derived_features,           // 531 dims
])  // Total: 2387 dims
```

**Missing feature handling:** Missing features are filled with the median value from the training distribution. If more than 30% of features are missing, L1 is considered unavailable and fallback to L2 is triggered.

### Inference

```
score = LightGBM.predict(feature_vector)
score = clamp(score, 0.0, 1.0)
```

The score represents the predicted probability that a message sent via this route will be successfully delivered within the TTL window.

### Fallback Conditions

L1 falls back to L2 when:
- ONNX runtime is not available.
- Feature extraction fails (insufficient telemetry data).
- Model file is missing or corrupted.
- Inference takes longer than 100ms (timeout).

---

## Layer 2: Gemma 4

### Model Specification

| Property | Value |
|----------|-------|
| **Model** | Gemma 4 (270M parameter variant) |
| **Context window** | 32,768 tokens |
| **Runtime** | Ollama or LiteRT |
| **Quantization** | Q4_K_M (4-bit quantization, ~150 MB) |
| **Inference time** | ~30 seconds (depends on hardware) |

### Prompt Engineering

Gemma 4 is used as a reasoning model to analyze routing decisions when L1 is unavailable. The prompt includes:

```
You are a mesh network routing assistant. Given the following network state,
recommend the optimal route from {source} to {destination}.

Network state:
- Available peers: {peer_list}
- Link qualities: {link_metrics}
- Recent delivery rates: {historical_rates}
- Current time: {timestamp}
- Message priority: {priority}
- TTL: {ttl_seconds}

Recommend a route as a list of peer IDs, with confidence score (0-1).
```

### Output Parsing

The model's response is parsed to extract:
- **Route:** Ordered list of peer IDs.
- **Confidence:** Numeric score (0.0 to 1.0).
- **Reasoning:** Text explanation (logged for debugging).

If parsing fails, fallback to L3 is triggered.

### Hardware Requirements

| Requirement | Minimum | Recommended |
|-------------|---------|-------------|
| **RAM** | 4 GB | 8 GB |
| **GPU** | None (CPU inference) | CUDA-capable GPU (optional acceleration) |
| **Storage** | 200 MB (model + runtime) | 500 MB |

**Mobile/embedded:** On devices with <4 GB RAM, L2 is skipped and fallback to L3 is immediate.

### Fallback Conditions

L2 falls back to L3 when:
- Ollama/LiteRT is not installed or not running.
- Model file is missing or fails to load.
- Inference exceeds 60-second timeout.
- Response parsing fails.
- Device has insufficient RAM (<4 GB).

---

## Layer 3: Graph Neural Network (GNN)

### Model Specification

| Property | Value |
|----------|-------|
| **Architecture** | Graph Attention Network (GAT) |
| **Layers** | 4 GAT layers |
| **Attention heads** | 8 heads per layer |
| **Format** | ONNX |
| **Inference time** | 91μs (ONNX Runtime, CPU) |
| **Input** | Graph structure + node features |

### Graph Construction

The mesh network is represented as a graph G = (V, E) where:
- **V (nodes):** Active peers in the routing table.
- **E (edges):** Active communication links between peers.

**Node features:**
```
node_features[v] = concatenate([
    uptime_hours,           // 1 dim
    buffer_usage,           // 1 dim
    avg_latency_ms,         // 1 dim
    packet_loss_rate,       // 1 dim
    trust_score,            // 1 dim
    geographic_lat,         // 1 dim
    geographic_lon,         // 1 dim
    transport_capacity,     // N dims (one per transport type)
])  // Total: ~10-20 dims per node
```

**Edge features:**
```
edge_features[u, v] = concatenate([
    link_quality,           // 1 dim
    rtt_ms,                 // 1 dim
    bandwidth_mbps,         // 1 dim
    last_seen_seconds,      // 1 dim
])  // Total: 4 dims per edge
```

### Attention Mechanism

Each GAT layer computes attention-weighted messages:

```
α_uv = softmax_v(LeakyReLU(a^T [Wh_u || Wh_v]))
h'_v = σ(Σ_{u∈N(v)} α_uv * Wh_u)
```

Where:
- `W` is a learnable weight matrix.
- `a` is the attention vector.
- `h_u`, `h_v` are node feature vectors.
- `N(v)` is the set of neighbors of node v.
- `||` denotes concatenation.
- `σ` is the activation function (ELU).

**Multi-head attention:** 8 independent attention mechanisms are computed and concatenated (or averaged for the final layer).

### Output

The GNN outputs a delivery probability score for each candidate route:

```
route_score = GNN(graph, source, destination)
route_score = sigmoid(route_score)  // Normalize to [0, 1]
```

### Fallback Conditions

L3 falls back to L4 when:
- ONNX runtime is not available.
- Graph construction fails (no peers in routing table).
- Model file is missing or corrupted.
- Inference produces NaN or Inf values.

---

## Layer 4: PRoPHET (Probabilistic Routing Protocol using History of Encounters and Transitivity)

### Protocol Specification

PRoPHET is a delay-tolerant routing protocol that uses delivery predictability based on encounter history.

| Property | Value |
|----------|-------|
| **Protocol** | PRoPHET v2 (epidemic routing) |
| **Delivery predictability update** | P(a,b) = P_old + (1 - P_old) × P_init |
| **Time decay** | γ = 0.98 per hour |
| **Transitivity weight** | β = 0.25 |
| **Initial predictability** | P_init = 0.75 |

### Delivery Predictability

**Encounter update:** When node A encounters node B:
```
P(a,b) = P(a,b)_old + (1 - P(a,b)_old) × P_init
```

**Time decay:** For each hour since the last encounter:
```
P(a,b) = P(a,b)_old × γ^(hours_elapsed)
```

**Transitivity:** If A frequently encounters B, and B frequently encounters C, then A's predictability to C is boosted:
```
P(a,c)_new = P(a,c)_old + (1 - P(a,c)_old) × P(a,b) × P(b,c) × β
```

### Routing Decision

When forwarding a message, the node selects the next hop with the highest delivery predictability to the destination:

```
next_hop = argmax_{h ∈ neighbors} P(h, destination)
```

**Tie-breaking:** If multiple neighbors have equal predictability (within 0.01), prefer:
1. The neighbor with the lowest current buffer usage.
2. The neighbor with the most recent encounter with the destination.
3. Random selection (deterministic via BLAKE3 hash of bundle ID).

### PRoPHET State

Each node maintains a delivery predictability table:

```rust
pub struct ProphetState {
    pub predictabilities: HashMap<PeerId, f64>,  // P(self, peer)
    pub last_encounter: HashMap<PeerId, u64>,     // Unix timestamp
    pub decay_rate: f64,                          // γ (default: 0.98)
    pub transitivity_weight: f64,                 // β (default: 0.25)
    pub initial_predictability: f64,              // P_init (default: 0.75)
}
```

**State persistence:** The predictability table is persisted to local storage and survives restarts. Entries older than 30 days are pruned.

---

## Fallback Logic

The four-layer fallback operates as a cascade:

```
fn select_route(source, destination, candidates) -> Option<Route> {
    // Try L1: LightGBM
    if let Some(score) = lightgbm_predict(source, destination, candidates) {
        return Some(Route::from_scores(score));
    }

    // Try L2: Gemma 4
    if let Some(response) = gemma4_recommend_route(source, destination, candidates) {
        if let Some(score) = parse_gemma_response(response) {
            return Some(Route::from_scores(score));
        }
    }

    // Try L3: GNN
    if let Some(score) = gnn_predict(source, destination, candidates) {
        return Some(Route::from_scores(score));
    }

    // Fallback to L4: PRoPHET
    Some(Route::from_prophet(source, destination, candidates))
}
```

**Anti-thrashing:** To prevent rapid oscillation between layers, a hysteresis mechanism is used:
- Once a layer is selected, it remains active for a minimum of 60 seconds.
- A layer is only replaced if the current layer fails 3 consecutive times.
- After fallback, the system attempts to re-enable higher layers every 5 minutes.

---

## Model Updates

### Update Mechanism

Models are updated via:
1. **Manual download:** Users download updated model files from trusted sources.
2. **DHT distribution:** Model hashes are published to the DHT; peers verify against known-good hashes.
3. **In-app update:** The client checks for model updates periodically (default: weekly).

### Model Verification

Before loading a new model:
1. Compute BLAKE3 hash of the model file.
2. Compare against the expected hash (published by the GhostWire maintainers).
3. If the hash does not match, reject the model and log a warning.

### Training Pipeline

Models are retrained periodically using fresh mesh telemetry data:

```
1. Collect telemetry from participating nodes (opt-in)
2. Preprocess: normalize, handle missing values, feature engineering
3. Train LightGBM on labeled data (delivery success/failure)
4. Export to ONNX format
5. Train GNN on graph-structured data
6. Export to ONNX format
7. Publish model hashes to DHT
8. Distribute via peer-to-peer sync
```

**Privacy:** Telemetry data is anonymized. No personally identifiable information is included in training data. Node IDs are hashed before export.

---

## Security Properties

| Property | Mechanism |
|----------|-----------|
| **Model integrity** | BLAKE3 hash verification before loading |
| **Adversarial input resistance** | Feature clipping to training distribution bounds |
| **Fallback safety** | PRoPHET (L4) is deterministic and requires no external model |
| **Resource exhaustion** | Timeout on each layer; RAM limits for L2 |
| **Privacy** | Telemetry anonymized; no PII in training data |

### Threat Model

| Adversary | Capability | Protection |
|-----------|-----------|------------|
| **Model poisoning** | Replaces model file with malicious version | Hash verification rejects tampered models |
| **Feature manipulation** | Injects fake telemetry data | Feature clipping; outlier detection |
| **Resource exhaustion** | Forces expensive L2 inference repeatedly | Timeout + RAM limits; hysteresis anti-thrashing |
| **Routing manipulation** | Biases routing toward compromised nodes | Web of Trust scores factor into routing; multi-path routing available |

---

## Reference Implementation

See the following modules in the GhostWire reference implementation:

| Module | Path |
|--------|------|
| L1 LightGBM | `src/core/ai_routing/lightgbm.rs` |
| L2 Gemma 4 | `src/core/ai_routing/gemma4.rs` |
| L3 GNN | `src/core/ai_routing/gnn.rs` |
| L4 PRoPHET | `src/core/ai_routing/prophet.rs` |
| Feature extraction | `src/core/ai_routing/features.rs` |
| Fallback logic | `src/core/ai_routing/fallback.rs` |
| Model management | `src/core/ai_routing/model_manager.rs` |
---

> **Lawful Use Only:** GhostWire is designed for lawful communication in connectivity-constrained environments. Users are responsible for complying with all applicable laws and regulations in their jurisdiction. The developers assume no liability for misuse of this software.

---
