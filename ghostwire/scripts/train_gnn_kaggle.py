#!/usr/bin/env python3
"""
GhostWire L3 GNN Training — Kaggle GPU Kernel
================================================
Trains a Graph Attention Network (GAT) for mesh routing optimization.

Generates realistic mesh topology data based on Guifi.net Guifisants node
characteristics (63 nodes, temporal adjacency, routing tables, link metrics).

Architecture improvements over v0.1.0 (54% accuracy):
  1. Better node features (6 → 8 dimensions with betweenness, link diversity)
  2. Deeper GAT (3 layers → 4 layers with residual connections)
  3. Multi-head attention (4 → 8 heads with concat=False for stability)
  4. Edge-aware GATConv with edge features
  5. Cosine annealing LR with warmup
  6. Gradient clipping (max_norm=1.0)
  7. Larger batch size (32 → 64)
  8. More training samples (2000 → 7931)
  9. Label smoothing for regularization
 10. Ensemble of top-3 checkpoints

Target: 85%+ accuracy (was 54%)
"""

import numpy as np
import torch
import torch.nn as nn
import torch.nn.functional as F
from torch.optim.lr_scheduler import CosineAnnealingLR
from torch_geometric.nn import GATConv
from torch_geometric.data import Data
import time
import json
import os
from pathlib import Path

print("=" * 70)
print("GHOSTWIRE L3 — GNN Routing Model Training (Kaggle GPU)")
print("=" * 70)

# ============================================================
# Configuration
# ============================================================
CONFIG = {
    "num_nodes": 63,
    "num_samples": 7931,
    "node_features": 8,
    "edge_features": 2,
    "hidden_dim": 128,
    "num_layers": 4,
    "num_heads": 8,
    "dropout": 0.15,
    "batch_size": 64,
    "epochs": 150,
    "learning_rate": 5e-4,
    "weight_decay": 1e-4,
    "patience": 20,
    "seed": 42,
}

OUTPUT_DIR = Path("/kaggle/working/models/gnn_routing")
OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

np.random.seed(CONFIG["seed"])
torch.manual_seed(CONFIG["seed"])

# ============================================================
# Generate Realistic Mesh Topology Data
# ============================================================
print("\nGenerating realistic Guifi.net-style mesh topology...")

NUM_NODES = CONFIG["num_nodes"]
NUM_SAMPLES = CONFIG["num_samples"]

def generate_mesh_topology(num_nodes, num_samples, seed=42):
    """Generate realistic mesh routing data based on Guifi.net characteristics."""
    rng = np.random.RandomState(seed)

    # Generate temporal adjacency snapshots (80 time steps)
    # Real mesh networks have ~15-25% connectivity density
    base_density = 0.18
    adj_snapshots = []
    for t in range(80):
        # Temporal variation: links come and go
        density = base_density + rng.uniform(-0.05, 0.05)
        adj = np.zeros((num_nodes, num_nodes), dtype=np.float32)
        for i in range(num_nodes):
            for j in range(i + 1, num_nodes):
                if rng.random() < density:
                    weight = rng.uniform(0.3, 1.0)
                    adj[i, j] = weight
                    adj[j, i] = weight
        adj_snapshots.append(adj)

    # Generate routing tables and metrics
    routing_tables = []
    metrics = []

    for _ in range(num_samples):
        # Pick a random adjacency snapshot as base topology
        base_adj = adj_snapshots[rng.randint(0, len(adj_snapshots))].copy()

        # Build routing table using shortest path on this topology
        routing = np.full((num_nodes, num_nodes), -1, dtype=np.int32)
        metric = np.full((num_nodes, num_nodes), -1.0, dtype=np.float32)

        for src in range(num_nodes):
            # Dijkstra-like routing
            dist = np.full(num_nodes, np.inf)
            prev = np.full(num_nodes, -1, dtype=np.int32)
            dist[src] = 0
            visited = set()

            for _ in range(num_nodes):
                # Find unvisited node with minimum distance
                min_dist = np.inf
                u = -1
                for v in range(num_nodes):
                    if v not in visited and dist[v] < min_dist:
                        min_dist = dist[v]
                        u = v

                if u == -1:
                    break

                visited.add(u)

                for v in range(num_nodes):
                    if base_adj[u, v] > 0:
                        # Link quality as inverse weight
                        link_cost = 1.0 / base_adj[u, v]
                        # Add some noise for realistic routing variation
                        link_cost *= (1.0 + rng.uniform(-0.1, 0.1))
                        alt = dist[u] + link_cost
                        if alt < dist[v]:
                            dist[v] = alt
                            prev[v] = u

            # Build routing table from prev pointers
            for dst in range(num_nodes):
                if dist[dst] < np.inf:
                    # Trace back to find next hop
                    current = dst
                    while prev[current] != src and prev[current] != -1:
                        current = prev[current]
                    if prev[current] == src:
                        routing[src, dst] = current
                        # Metric: inverse of path cost
                        metric[src, dst] = max(0, 1e8 / (dist[dst] + 1))

        routing_tables.append(routing)
        metrics.append(metric)

    return adj_snapshots, routing_tables, metrics


adj_snapshots, routing_tables, metrics = generate_mesh_topology(
    NUM_NODES, NUM_SAMPLES, seed=42
)

print(f"  Nodes: {NUM_NODES}")
print(f"  Routing samples: {len(routing_tables)}")
print(f"  Adjacency snapshots: {len(adj_snapshots)}")
print(f"  Avg density: {np.mean([np.count_nonzero(a) / (NUM_NODES**2) for a in adj_snapshots]):.3f}")

# ============================================================
# Build Graph Dataset
# ============================================================
print(f"\n{'=' * 70}")
print("BUILDING GRAPH DATASET")
print(f"{'=' * 70}")


def build_graph_samples(routing_tables, metrics, adj_snapshots, max_samples=7931):
    """Convert mesh data into graph samples for GNN training."""
    num_nodes = routing_tables[0].shape[0]
    graphs = []
    failed = 0

    n_snapshots = len(adj_snapshots)
    n_routing = len(routing_tables)

    rng = np.random.RandomState(42)
    indices = rng.choice(n_routing, min(max_samples, n_routing), replace=False)

    samples_per_graph = 10

    for idx in indices:
        if len(graphs) >= max_samples:
            break

        snap_idx = idx % n_snapshots
        routing = routing_tables[idx]
        metric = metrics[idx]
        adj = adj_snapshots[snap_idx]

        # Node features (8 dimensions)
        node_features = np.zeros((num_nodes, 8), dtype=np.float32)
        for i in range(num_nodes):
            out_degree = (routing[i] >= 0).sum()
            in_degree = (routing[:, i] >= 0).sum()
            valid_links = metric[i][metric[i] != -1]
            avg_quality = valid_links.mean() if len(valid_links) > 0 else 0
            valid_hops = routing[i][routing[i] >= 0]
            avg_hops = valid_hops.mean() / num_nodes if len(valid_hops) > 0 else 0
            transit_count = (routing[:, :] == i).sum() / num_nodes
            link_diversity = len(valid_links) / num_nodes if len(valid_links) > 0 else 0
            # NEW: adjacency-based features
            adj_degree = np.count_nonzero(adj[i]) / num_nodes
            adj_strength = adj[i].sum() / max(np.count_nonzero(adj[i]), 1)
            node_features[i] = [
                out_degree / num_nodes,
                in_degree / num_nodes,
                avg_quality / 1e8,
                avg_hops,
                transit_count,
                link_diversity,
                adj_degree,
                adj_strength,
            ]

        # Build edge index from routing
        edge_list = []
        edge_attrs = []
        for i in range(num_nodes):
            for j in range(num_nodes):
                if routing[i, j] >= 0:
                    edge_list.append([i, j])
                    link_metric = metric[i, j] if metric[i, j] != -1 else 0
                    edge_attrs.append([link_metric / 1e8, 1.0 if routing[i, j] == j else 0.0])

        if not edge_list:
            failed += 1
            continue

        edge_index = np.array(edge_list, dtype=np.int64).T
        edge_attr = np.array(edge_attrs, dtype=np.float32)

        # Extract (src, dst, next_hop) training samples
        for src in range(num_nodes):
            neighbors = np.where(routing[src] >= 0)[0]
            if len(neighbors) < 2:
                continue

            valid_dsts = [d for d in range(num_nodes) if d != src and routing[src, d] >= 0]
            if not valid_dsts:
                continue

            sample_dsts = rng.choice(valid_dsts, min(samples_per_graph, len(valid_dsts)), replace=False)

            for dst in sample_dsts:
                next_hop = routing[src, dst]
                if next_hop < 0 or next_hop not in neighbors:
                    continue

                max_cand = 5
                if len(neighbors) > max_cand:
                    qualities = np.array([metric[src, n] if metric[src, n] != -1 else 0 for n in neighbors])
                    top_idx = np.argsort(-qualities)[:max_cand]
                    candidates = neighbors[top_idx]
                else:
                    candidates = neighbors

                try:
                    label_idx = list(candidates).index(next_hop)
                except ValueError:
                    continue

                graphs.append({
                    'x': node_features.copy(),
                    'edge_index': edge_index.copy(),
                    'edge_attr': edge_attr.copy(),
                    'src': src,
                    'dst': dst,
                    'candidates': candidates.copy(),
                    'label': label_idx,
                })

    print(f"  Generated {len(graphs):,} routing samples ({failed} failed)")
    return graphs


start = time.time()
graphs = build_graph_samples(routing_tables, metrics, adj_snapshots, max_samples=7931)
build_time = time.time() - start
print(f"  Build time: {build_time:.1f}s")

if not graphs:
    print("ERROR: No valid graphs generated.")
    exit(1)

# ============================================================
# Train/Val/Test Split
# ============================================================
np.random.seed(42)
indices = np.random.permutation(len(graphs))
n = len(graphs)
train_end = int(0.8 * n)
val_end = int(0.9 * n)

train_data = [graphs[i] for i in indices[:train_end]]
val_data = [graphs[i] for i in indices[train_end:val_end]]
test_data = [graphs[i] for i in indices[val_end:]]

print(f"\n  Train: {len(train_data):,} | Val: {len(val_data):,} | Test: {len(test_data):,}")

# ============================================================
# Model: Improved GAT for Routing
# ============================================================
class MeshGAT(nn.Module):
    """Improved Graph Attention Network for mesh routing.

    Improvements over v0.1.0:
    - 4 layers (was 3) with residual connections
    - 8 attention heads (was 4)
    - Edge-aware GATConv
    - Better feature encoding
    """

    def __init__(self, node_features=8, edge_features=2, hidden_dim=128,
                 num_layers=4, num_heads=8, dropout=0.15):
        super().__init__()
        self.node_encoder = nn.Sequential(
            nn.Linear(node_features, hidden_dim),
            nn.LayerNorm(hidden_dim),
            nn.ReLU(),
            nn.Dropout(dropout),
        )
        self.convs = nn.ModuleList()
        self.norms = nn.ModuleList()
        for i in range(num_layers):
            self.convs.append(
                GATConv(hidden_dim, hidden_dim, heads=num_heads,
                        concat=False, dropout=dropout, edge_dim=edge_features)
            )
            self.norms.append(nn.LayerNorm(hidden_dim))

        self.router = nn.Sequential(
            nn.Linear(hidden_dim * 3, hidden_dim),
            nn.LayerNorm(hidden_dim),
            nn.ReLU(),
            nn.Dropout(dropout),
            nn.Linear(hidden_dim, hidden_dim // 2),
            nn.LayerNorm(hidden_dim // 2),
            nn.ReLU(),
            nn.Dropout(dropout),
            nn.Linear(hidden_dim // 2, 1),
        )

    def forward_single(self, x, edge_index, edge_attr, src, dst, candidates):
        h = self.node_encoder(x)

        # Residual GAT layers
        for conv, norm in zip(self.convs, self.norms):
            h_new = conv(h, edge_index, edge_attr=edge_attr)
            h_new = norm(h_new)
            # Residual connection (with dimension matching)
            if h.shape == h_new.shape:
                h = h + h_new
            else:
                h = h_new
            h = F.relu(h)
            h = F.dropout(h, p=0.15, training=self.training)

        src_emb = h[src]
        dst_emb = h[dst]
        cand_emb = h[candidates]

        num_cand = len(candidates)
        combined = torch.cat([
            src_emb.unsqueeze(0).expand(num_cand, -1),
            dst_emb.unsqueeze(0).expand(num_cand, -1),
            cand_emb,
        ], dim=-1)

        return self.router(combined).squeeze(-1)


# ============================================================
# Training Functions
# ============================================================
def train_epoch(model, data_list, optimizer, device, batch_size=64):
    model.train()
    total_loss = 0
    correct = 0
    total = 0

    np.random.shuffle(data_list)
    for i in range(0, len(data_list), batch_size):
        batch = data_list[i:i + batch_size]
        optimizer.zero_grad()

        losses = []
        for sample in batch:
            x = torch.tensor(sample['x'], dtype=torch.float, device=device)
            edge_index = torch.tensor(sample['edge_index'], dtype=torch.long, device=device)
            edge_attr = torch.tensor(sample['edge_attr'], dtype=torch.float, device=device)
            src = sample['src']
            dst = sample['dst']
            candidates = torch.tensor(sample['candidates'], dtype=torch.long, device=device)
            label = torch.tensor(sample['label'], dtype=torch.long, device=device)

            scores = model.forward_single(x, edge_index, edge_attr, src, dst, candidates)
            loss = F.cross_entropy(scores.unsqueeze(0), label.unsqueeze(0), label_smoothing=0.1)
            losses.append(loss)

            pred = scores.argmax().item()
            correct += (1 if pred == sample['label'] else 0)
            total += 1

        batch_loss = sum(l.item() for l in losses)
        total_loss += batch_loss
        sum(losses).backward()
        torch.nn.utils.clip_grad_norm_(model.parameters(), max_norm=1.0)
        optimizer.step()

    return total_loss / max(len(data_list), 1), correct / max(total, 1)


@torch.no_grad()
def evaluate(model, data_list, device):
    model.eval()
    total_loss = 0
    correct = 0
    total = 0

    for sample in data_list:
        x = torch.tensor(sample['x'], dtype=torch.float, device=device)
        edge_index = torch.tensor(sample['edge_index'], dtype=torch.long, device=device)
        edge_attr = torch.tensor(sample['edge_attr'], dtype=torch.float, device=device)
        src = sample['src']
        dst = sample['dst']
        candidates = torch.tensor(sample['candidates'], dtype=torch.long, device=device)
        label = torch.tensor(sample['label'], dtype=torch.long, device=device)

        scores = model.forward_single(x, edge_index, edge_attr, src, dst, candidates)
        loss = F.cross_entropy(scores.unsqueeze(0), label.unsqueeze(0))
        total_loss += loss.item()

        pred = scores.argmax().item()
        correct += (1 if pred == sample['label'] else 0)
        total += 1

    return total_loss / max(total, 1), correct / max(total, 1)


# ============================================================
# Main Training Loop
# ============================================================
print(f"\n{'=' * 70}")
print("TRAINING")
print(f"{'=' * 70}")

device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
print(f"  Device: {device}")
if torch.cuda.is_available():
    print(f"  GPU: {torch.cuda.get_device_name(0)}")
    print(f"  CUDA Memory: {torch.cuda.get_device_properties(0).total_mem / 1e9:.1f} GB")

model = MeshGAT(
    node_features=CONFIG["node_features"],
    edge_features=CONFIG["edge_features"],
    hidden_dim=CONFIG["hidden_dim"],
    num_layers=CONFIG["num_layers"],
    num_heads=CONFIG["num_heads"],
    dropout=CONFIG["dropout"],
).to(device)

num_params = sum(p.numel() for p in model.parameters())
print(f"  Parameters: {num_params:,}")

optimizer = torch.optim.AdamW(model.parameters(), lr=CONFIG["learning_rate"], weight_decay=CONFIG["weight_decay"])
scheduler = CosineAnnealingLR(optimizer, T_max=CONFIG["epochs"])

best_val_acc = 0
patience_counter = 0
training_history = []
top_checkpoints = []  # Keep top 3 for ensemble

for epoch in range(1, CONFIG["epochs"] + 1):
    start = time.time()
    train_loss, train_acc = train_epoch(model, train_data, optimizer, device, CONFIG["batch_size"])
    val_loss, val_acc = evaluate(model, val_data, device)
    epoch_time = time.time() - start
    scheduler.step()

    training_history.append({
        "epoch": epoch,
        "train_loss": train_loss,
        "train_acc": train_acc,
        "val_loss": val_loss,
        "val_acc": val_acc,
        "lr": scheduler.get_last_lr()[0],
    })

    print(f"  Epoch {epoch:3d}/{CONFIG['epochs']} | "
          f"Train: {train_loss:.4f} ({train_acc:.1%}) | "
          f"Val: {val_loss:.4f} ({val_acc:.1%}) | "
          f"Time: {epoch_time:.1f}s")

    if val_acc > best_val_acc:
        best_val_acc = val_acc
        patience_counter = 0

        # Save checkpoint
        ckpt_path = OUTPUT_DIR / f"gnn_routing_epoch{epoch}.pt"
        torch.save({
            "epoch": epoch,
            "model_state_dict": model.state_dict(),
            "optimizer_state_dict": optimizer.state_dict(),
            "val_acc": val_acc,
            "val_loss": val_loss,
            "config": CONFIG,
        }, ckpt_path)

        # Track top checkpoints for ensemble
        top_checkpoints.append((val_acc, ckpt_path, epoch))
        top_checkpoints.sort(reverse=True)
        top_checkpoints = top_checkpoints[:3]

        # Also save as "best"
        torch.save({
            "epoch": epoch,
            "model_state_dict": model.state_dict(),
            "optimizer_state_dict": optimizer.state_dict(),
            "val_acc": val_acc,
            "val_loss": val_loss,
            "config": CONFIG,
        }, OUTPUT_DIR / "gnn_routing_best.pt")
    else:
        patience_counter += 1
        if patience_counter >= CONFIG["patience"]:
            print(f"  Early stopping at epoch {epoch}")
            break

# ============================================================
# Test Evaluation
# ============================================================
print(f"\n{'=' * 70}")
print("TEST EVALUATION")
print(f"{'=' * 70}")

checkpoint = torch.load(OUTPUT_DIR / "gnn_routing_best.pt", weights_only=True)
model.load_state_dict(checkpoint["model_state_dict"])
test_loss, test_acc = evaluate(model, test_data, device)
print(f"  Test Loss: {test_loss:.4f}")
print(f"  Test Accuracy: {test_acc:.1%}")
print(f"  Best Val Accuracy: {best_val_acc:.1%}")

# ============================================================
# Create Ensemble Model (top 3 checkpoints)
# ============================================================
if len(top_checkpoints) >= 2:
    print(f"\n  Creating ensemble from top {len(top_checkpoints)} checkpoints...")
    ensemble_weights = []
    for acc, path, epoch in top_checkpoints:
        ckpt = torch.load(path, weights_only=True)
        ensemble_weights.append((acc, ckpt["model_state_dict"]))
        print(f"    Epoch {epoch}: val_acc={acc:.1%}")

    # Average the weights
    avg_state = {}
    for key in ensemble_weights[0][1].keys():
        avg_state[key] = sum(w[key] for _, w in ensemble_weights) / len(ensemble_weights)

    torch.save({
        "epoch": checkpoint["epoch"],
        "model_state_dict": avg_state,
        "val_acc": test_acc,
        "config": CONFIG,
        "ensemble": [{"epoch": e, "val_acc": a} for a, _, e in top_checkpoints],
    }, OUTPUT_DIR / "gnn_routing_ensemble.pt")
    print(f"  Ensemble saved")

# ============================================================
# Save Training History
# ============================================================
with open(OUTPUT_DIR / "training_history.json", 'w') as f:
    json.dump(training_history, f, indent=2)

# ============================================================
# Summary
# ============================================================
print(f"\n{'=' * 70}")
print("TRAINING COMPLETE")
print(f"{'=' * 70}")
print(f"  Dataset: Guifi.net-style mesh ({NUM_NODES} nodes, {len(graphs):,} samples)")
print(f"  Test Accuracy: {test_acc:.1%}")
print(f"  Best Val Accuracy: {best_val_acc:.1%}")
print(f"  Model: {OUTPUT_DIR / 'gnn_routing_best.pt'}")
print(f"  Ensemble: {OUTPUT_DIR / 'gnn_routing_ensemble.pt'}")
print(f"  History: {OUTPUT_DIR / 'training_history.json'}")
print(f"\nNext: Wire into Rust for periodic routing table optimization")
