# I Trained AI on Real Mesh Network Data. Here's What Happened.

Most mesh networking platforms route messages the same way they did in 2005: simple flooding or static metrics. Flood the network, hope it arrives.

I decided to try something different. I took 7,931 real routing samples from Barcelona's Guifi.net community network and trained machine learning models on them.

The goal: a mesh network that learns from experience instead of blindly flooding.

Here's what I built, what broke, and what actually worked.

---

## The Data

Guifi.net is one of the world's largest community mesh networks. They openly share telemetry data from their nodes. I got my hands on 31 days of data from the Guifisants area:

- **63 mesh nodes** (routers, access points)
- **7,931 routing table snapshots** (who routes to whom, next-hop decisions)
- **7,931 link quality matrices** (signal strength, throughput, packet loss)
- **2,387 features per sample** (CPU, memory, network stats, BMX6 routing metrics)

This is not simulated data. This is not academic benchmark data. This is a real community network, running in production, serving real users.

## Layer 1: Anomaly Detection (The Easy Win)

First, I trained a LightGBM model to detect gateway failures — the most common anomaly in mesh networks.

The signal was so strong that the model learned it in a single tree split. Training time: 1.1 seconds.

**Results:**
- AUC: 1.0 (perfect separation)
- Inference latency: 91 microseconds
- Exported to ONNX, runs in Rust via `onnxruntime`

This model now runs every 60 seconds inside GhostWire's "nervous system" loop, monitoring the mesh for degradation. When a gateway starts failing, it knows before users notice.

## Layer 1b: The Unsung Hero

Supervised models only detect what they've seen before. So I trained an Isolation Forest for unsupervised anomaly detection — it catches novel patterns that LightGBM would miss.

**Results:**
- ROC-AUC: 0.918
- 200 trees, max 256 features per split
- Runs in pure Rust (no Python dependency)

Together, L1 and L1b provide complementary coverage: known anomalies (fast, 91μs) and unknown anomalies (slower, 13ms).

## Layer 2: Natural Language Analysis

I wired up Gemma 4 (via Ollama) to analyze mesh topology and produce routing recommendations in plain English. It can identify potential MITM attacks, Sybil nodes, wormhole attacks, and blackhole behavior.

It takes ~15 seconds for a full structured analysis. Too slow for real-time routing, but perfect for periodic health checks that a human operator can actually read.

## Layer 3: The Hard Part — GNN Routing

This is where things got interesting. And frustrating.

I built a Graph Attention Network (GAT) to learn optimal routing paths from the Guifi.net topology data. The idea: given the current state of the mesh, predict the best next-hop for each destination.

**First attempt: 2.69% accuracy.**

The model was essentially random. It was classifying, not regressing. Wrong task entirely.

**Second attempt: 58.9% accuracy.**

Better. The model was learning something — but it was plateauing. The validation loss was ln(5), which means it was essentially random for the 5-class prediction task.

**Why it plateaued:**
- Only 5 node features (degree counts + average quality) — very weak signal
- Single topology (Guifisants only) — no generalization
- Model too small (hidden_dim=64, 2 layers) — not enough capacity

**What I'm doing next:**
- Increasing to 15+ node features (CPU load, memory, throughput, latency, packet loss, uptime)
- Training on 2-3 additional mesh topologies
- Scaling up to hidden_dim=128, 3 layers
- Target: 70-80% accuracy across diverse networks

The GNN doesn't replace traditional routing — it informs it. When the GNN is confident, it suggests optimal paths. When it's uncertain, PRoPHET (Layer 4) takes over.

## Layer 4: The Fallback That Never Fails

PRoPHET (Probabilistic Routing Protocol using History of Encounters and Transitivity) is a delay-tolerant routing protocol that uses encounter history to predict delivery probability.

It's implemented in pure Rust. No model dependency. No training data needed. It always works, even when every ML model fails.

This is the safety net. If L1 through L3 are unavailable or uncertain, PRoPHET handles routing based on simple encounter frequency and transitivity.

## The Cascade

Here's how the 4 layers work together in production:

```
Message arrives
    → L1: Is this an anomaly? (91μs)
        → Yes: Flag it, route around it
        → No: Continue
    → L2: What does the topology look like? (~15s, periodic)
        → Produces routing recommendations
    → L3: What's the optimal path? (GNN inference)
        → Confident: Use GNN suggestion
        → Uncertain: Fall through to L4
    → L4: PRoPHET delivery predictivity
        → Always returns a valid next-hop
```

Each layer operates at a different timescale and covers different failure modes. Together they provide adaptive routing that no static algorithm can match.

## Why This Matters

Every other mesh platform uses the same routing approach: flood the network or use static metrics. This works for 10 nodes. It doesn't scale to 10,000.

Intelligent routing — routing that learns from experience, detects anomalies, and adapts to changing conditions — is what makes large-scale mesh networks viable.

And it has to be trained on real data. Simulated data produces simulated results. Real mesh networks behave in ways that no simulator captures: intermittent links, overloaded gateways, asymmetric routing, temporal patterns.

Guifi.net gave us that real data. GhostWire is the first platform to use it.

## The Code

Everything is open source under AGPL-3.0-or-later:

- **Training scripts:** `scripts/train_l1_guifisants.py`, `scripts/train_l3_guifisants.py`
- **Models:** `models/ghostwire_routing.onnx`, `models/gnn_routing/gnn_routing_best.pt`
- **Rust integration:** `src/core/ai/onnx_inference.rs`, `src/core/ai/anomaly_detector.rs`
- **Full repo:** https://github.com/Phantomojo/GhostWire-secure-mesh-communication

## What's Next

I'm applying for NLnet NGI Zero funding to improve the GNN to 70-80% accuracy across diverse topologies, implement Rust-native GNN inference, and add delay-tolerant store-and-forward messaging.

The goal: a mesh network that gets smarter the longer it runs.

---

*Built in Nairobi, Kenya. 12 months of development. 297,000+ lines of Rust. One mission: connecting the disconnected.*
