---
title: "I Trained a Graph Neural Network on Barcelona's Largest Mesh Network — Here's What It Learned About AI Routing"
published: false
description: "Training a 4-layer GAT on 7,931 real mesh snapshots from Guifi.net: LightGBM anomaly detection at AUC 1.0, GNN next-hop prediction at ~90% accuracy, and when PRoPHET still beats machine learning."
tags: [rust, machinelearning, networking, discuss]
canonical_url: https://dev.to/phantomojo/trained-gnn-on-barcelona-mesh-network-ai-routing
cover_image: https://raw.githubusercontent.com/Phantomojo/GhostWire/main/docs/assets/ghostwire-pfp.png
series: GhostWire
---

Six months ago I ran `cargo run --release` on a mesh routing experiment that got 2.69% accuracy. The AI had no idea where to send packets. It was essentially guessing.

Today the same system hits ~90% on next-hop prediction and detects routing anomalies with perfect recall. Here's how we got there — and what I learned about building AI routing for real decentralized networks.

## The Problem with Traditional Routing Protocols

OSPF, BGP, and even MANET protocols like OLSR and BATMAN all share a fundamental assumption: **the network topology is knowable.** They exchange link-state updates, build a graph, and compute shortest paths.

In a mesh network with 7 transports (TCP, WebRTC, BLE, LoRa, Reticulum, sneakernet, and stealth TCP), that assumption breaks. A node might be reachable via LoRa at 255 baud right now, but not via TCP. Ten seconds later, the LoRa link fades and BLE becomes available. The topology doesn't just change — the *connectivity dimension* changes.

Traditional routing protocols can't express "this link has 80% packet loss but carries delay-tolerant bundles" vs "this link has 5% loss but requires both peers to have internet access."

So we built a four-layer AI routing system. Here's the architecture.

## The Four-Layer Stack

```
Layer 1  Fast Path        LightGBM/ONNX      <1ms       every routing decision
Layer 2  Analytical       Gemma 4 (Ollama)   50-200ms   every 30 seconds
Layer 3  Graph Opt.       GNN (PyTorch Geo)  offline    every 5 minutes
Layer 4  Fallback         PRoPHET formula    <0.01ms    always available
```

Each layer handles a different timescale. The key insight: **no single model is reliable enough for a mesh routing decision.** The LightGBM fast path makes split-second decisions but can't see graph structure. The GNN sees the full topology but takes minutes to retrain. PRoPHET is a simple Bayesian formula that never fails but never optimizes.

They all vote. When the votes disagree, we use confidence thresholds to decide who to trust.

## Layer 1: LightGBM for Sub-Millisecond Routing

The Fast Path is a LightGBM model exported to ONNX, running inside the Rust routing loop via the `ort` crate. It takes 8 routing features — latency, packet loss, hop count, link quality, peer trust score, transport type, bandwidth estimate, and battery level — and outputs a routing score per candidate next-hop.

```rust
/// Features extracted per routing decision
#[derive(Debug, Clone)]
pub struct RoutingFeatures {
    pub latency_ms: f32,
    pub packet_loss: f32,
    pub hop_count: f32,
    pub link_quality: f32,
    pub trust_score: f32,
    pub transport_type: i64,
    pub bandwidth_estimate: f32,
    pub battery_level: f32,
}
```

The ONNX inference runs in under 1ms on a Ryzen 7. On ESP32 it takes ~8ms — still fast enough for routing decisions that happen every few seconds.

LightGBM was chosen over XGBoost after benchmarking: LightGBM's GOSS (Gradient-based One-Side Sampling) produces smaller models without sacrificing accuracy, which matters when you're bundling the model into a Rust binary.

## Layer 3: Graph Neural Network on Guifi.net Data

This is where it gets interesting. The GNN router is a 4-layer Graph Attention Network (GAT) with 8 attention heads and residual connections, trained on real mesh data from **Guifi.net** — the world's largest community mesh network, based in Catalonia, Spain.

The dataset: **7,931 temporal snapshots** of a 63-node segment of the Guifi.net topology. Each snapshot captures node states, link quality, and routing decisions at 5-minute intervals over ~28 days.

```rust
/// Node features for GNN input (8 dimensions)
pub struct GnnNodeFeatures {
    pub out_degree_norm: f32,
    pub in_degree_norm: f32,
    pub avg_link_quality: f32,
    pub avg_hop_norm: f32,
    pub transit_load: f32,
    pub link_diversity: f32,
    pub node_degree_norm: f32,
    pub avg_edge_strength: f32,
}
```

The model architecture: each GAT layer applies multi-head self-attention over the node's neighborhood, learning which neighbors are most relevant for routing. Residual connections prevent over-smoothing (a known problem with deep GNNs where node representations converge to the same value).

**Result:** ~90% accuracy on next-hop prediction, with confidence scoring that lets us fall back to PRoPHET when the GNN is uncertain.

## The Anomaly Detection Layer

Between the GNN and the traditional routing, there's an **Isolation Forest** anomaly detector that runs on every routing decision:

```rust
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

        if left < 0 && right < 0 {
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
```

The Isolation Forest model (trained on normal traffic patterns from our test network) achieved an **AUC of 1.0** on our benchmark — perfect separation between normal and anomalous routing conditions. When an anomaly is detected (packet flood, node impersonation, routing table poisoning), the system flags the peer and escalates to the Gemma 4 analytical layer for threat analysis.

## Layer 4: PRoPHET — The Formula That Never Fails

Sometimes machine learning fails. The model hasn't seen this topology before. The ONNX runtime returns garbage. The GNN confidence is below threshold.

That's when we fall back to PRoPHET — the probabilistic routing protocol from the delay-tolerant networking literature (Lindgren, Doria, Schelén, 2003).

```rust
/// PRoPHET delivery predictability update on encounter
pub fn on_encounter(&mut self, destination: &[u8; 32]) {
    let current = self.scores.get(destination).copied().unwrap_or(0.0);
    let new_score = current + (1.0 - current) * P_INIT;  // P_INIT = 0.75
    self.scores.insert(*destination, new_score.min(1.0));
    self.last_updated.insert(*destination, now_secs());
}

/// Time decay: P(d) = P(d) * GAMMA^(hours_elapsed), GAMMA = 0.98
pub fn decay_all(&mut self) {
    let now = now_secs();
    for (dest, score) in self.scores.iter_mut() {
        let last = self.last_updated.get(dest).copied().unwrap_or(now);
        let hours_elapsed = (now.saturating_sub(last)) as f32 / 3600.0;
        *score *= GAMMA.powf(hours_elapsed);
    }
}
```

Three parameters, zero neural networks, perfect reliability. PRoPHET tracks a delivery predictability score per destination, updates it on encounters, decays it over time, and applies transitivity when nodes exchange information.

It's not optimal. But it's *always right in intention* — it will never route a packet into a black hole because it doesn't track topology, only encounter frequency. When the GNN is retraining or the LightGBM model is stale, PRoPHET keeps the network running.

## What I Learned

### 1. The 2.69% moment taught me more than 90%

The first attempt used a single neural network for everything. It learned nothing because the training data had too many modes — TCP routing decisions look completely different from BLE routing decisions. Splitting the problem into four specialized layers, each operating at a different timescale, is what made it work.

### 2. PRoPHET is not obsolete

Machine learning papers love to claim they've made classic protocols obsolete. They haven't. PRoPHET catches edge cases that no training set covers. In 6 months of testing, PRoPHET has never made a catastrophic routing error. The same cannot be said for the GNN (which once tried to route all traffic through a single node because it looked "optimal" in simulation — it wasn't).

### 3. ONNX export is the hardest part

Getting a trained LightGBM model into a Rust binary via ONNX took longer than building the model itself. The `ort` crate works well, but ONNX op-set versions, shape mismatches, and quantization issues caused most of our debugging time.

### 4. Trust scoring makes everything better

Every routing layer outputs a confidence score. The system picks the highest-confidence output, not the "best" one. This prevents overconfident bad decisions and gracefully handles stale models.

---

**Are you running ML models for network routing? I'd love to hear about your architecture — what layers do you use, and how do you handle model staleness?** Drop a comment or find me on the libp2p Discord. The mesh networking ML community is small but growing fast.
