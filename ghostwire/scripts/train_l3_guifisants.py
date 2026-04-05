#!/usr/bin/env python3
"""
Train L3 GNN on REAL Guifi.net mesh data (Guifisants node).

Dataset:
  - 63 nodes (mesh routers)
  - 80 temporal adjacency snapshots (time-series topology)
  - 7,931 routing table samples (63×63 next-hop matrices)
  - 7,931 metric samples (63×63 link quality matrices)

Task: Next-hop routing prediction
  For each (src, dst) pair, predict the optimal next-hop node.

Architecture: Graph Attention Network (GAT) with real topology
"""

import numpy as np
import torch
import torch.nn as nn
import torch.nn.functional as F
from torch.optim.lr_scheduler import CosineAnnealingLR
from torch_geometric.nn import GATConv
from torch_geometric.data import Data
from torch_geometric.loader import DataLoader
from scipy import sparse
import glob
import pickle
import time
import json
from pathlib import Path

SCRIPT_DIR = Path(__file__).parent
PROJECT_DIR = SCRIPT_DIR.parent
DATA_DIR = PROJECT_DIR.parent / "datasets" / "processed"
MODEL_DIR = PROJECT_DIR / "models" / "gnn_routing"
MODEL_DIR.mkdir(parents=True, exist_ok=True)

print("=" * 70)
print("GHOSTWIRE L3 — GNN on REAL Guifi.net Mesh Topology")
print("=" * 70)

# ============================================================
# Load real mesh data
# ============================================================
print("\nLoading real Guifi.net mesh data...")

# Adjacency matrices (80 temporal snapshots)
adj_files = sorted(glob.glob(str(DATA_DIR / "l3_gnn" / "guifisants_adj_*.npz")))
print(f"  Adjacency snapshots: {len(adj_files)}")

# Load first to get dimensions
adj0_data = np.load(adj_files[0])
adj0 = sparse.csr_matrix(
    (adj0_data['data'], adj0_data['indices'], adj0_data['indptr']),
    shape=tuple(adj0_data['shape'])
)
num_nodes = adj0.shape[0]
print(f"  Nodes: {num_nodes}")
print(f"  First snapshot edges: {adj0.nnz}")

# Routing tables (7,931 samples of 63×63 next-hop matrices)
with open(DATA_DIR / "l3_gnn" / "guifisants_routing.pkl", 'rb') as f:
    routing_tables = pickle.load(f)
print(f"  Routing samples: {len(routing_tables)}")
print(f"  Routing shape: {routing_tables[0].shape}")

# Metrics (7,931 samples of 63×63 link quality)
with open(DATA_DIR / "l3_gnn" / "guifisants_metrics.pkl", 'rb') as f:
    metrics = pickle.load(f)
print(f"  Metric samples: {len(metrics)}")
print(f"  Metric shape: {metrics[0].shape}")

# ============================================================
# Build graph dataset from real routing data
# ============================================================
print(f"\n{'=' * 70}")
print("BUILDING GRAPH DATASET")
print(f"{'=' * 70}")

def build_graph_samples(routing_tables, metrics, adj_files, max_samples=2000):
    """Convert real mesh data into PyG graph samples for GNN training."""
    num_nodes = routing_tables[0].shape[0]
    graphs = []
    failed = 0

    # Load all adjacency snapshots
    adj_snapshots = []
    for f in adj_files:
        d = np.load(f)
        adj = sparse.csr_matrix(
            (d['data'], d['indices'], d['indptr']),
            shape=tuple(d['shape'])
        )
        adj_snapshots.append(adj)

    n_snapshots = len(adj_snapshots)
    n_routing = len(routing_tables)

    # Sample indices
    rng = np.random.RandomState(42)
    indices = rng.choice(n_routing, min(max_samples, n_routing), replace=False)

    samples_per_graph = 5  # Extract 5 (src,dst) pairs per routing snapshot
    target_samples = max_samples

    for idx in indices:
        if len(graphs) >= target_samples:
            break

        snap_idx = idx % n_snapshots
        routing = routing_tables[idx]
        metric = metrics[idx]

        # Precompute node features
        node_features = np.zeros((num_nodes, 5), dtype=np.float32)
        for i in range(num_nodes):
            out_degree = (routing[i] >= 0).sum()
            in_degree = (routing[:, i] >= 0).sum()
            valid_links = metric[i][metric[i] != -1]
            avg_quality = valid_links.mean() if len(valid_links) > 0 else 0
            valid_hops = routing[i][routing[i] >= 0]
            avg_hops = valid_hops.mean() / num_nodes if len(valid_hops) > 0 else 0
            node_features[i] = [
                out_degree / num_nodes,
                in_degree / num_nodes,
                avg_quality / 1e8,
                avg_hops,
                out_degree / num_nodes,
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

        edge_index = torch.tensor(edge_list, dtype=torch.long).t().contiguous()
        edge_attr = torch.tensor(edge_attrs, dtype=torch.float)
        x = torch.tensor(node_features, dtype=torch.float)

        # Extract (src, dst, next_hop) training samples
        src_list, dst_list, cand_list, label_list = [], [], [], []

        for src in range(num_nodes):
            neighbors = np.where(routing[src] >= 0)[0]
            if len(neighbors) < 2:
                continue

            # Sample a few destinations per source
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

                src_list.append(src)
                dst_list.append(dst)
                cand_list.append(candidates.copy())
                label_list.append(label_idx)

        if not src_list:
            failed += 1
            continue

        for i in range(len(src_list)):
            graphs.append({
                'x': x.clone(),
                'edge_index': edge_index.clone(),
                'edge_attr': edge_attr.clone(),
                'src': src_list[i],
                'dst': dst_list[i],
                'candidates': torch.tensor(cand_list[i], dtype=torch.long),
                'label': label_list[i],
            })

    print(f"  Generated {len(graphs):,} routing samples ({failed} failed)")
    return graphs


# Build dataset
start = time.time()
np.random.seed(42)
graphs = build_graph_samples(routing_tables, metrics, adj_files, max_samples=3000)
build_time = time.time() - start
print(f"  Build time: {build_time:.1f}s")

if not graphs:
    print("ERROR: No valid graphs generated. Check data format.")
    exit(1)

# ============================================================
# Train/val/test split
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
# Model: GAT for routing
# ============================================================
class MeshGAT(nn.Module):
    """Graph Attention Network for real mesh routing."""

    def __init__(self, node_features=5, edge_features=2, hidden_dim=64,
                 num_layers=2, num_heads=4, dropout=0.1):
        super().__init__()
        self.node_encoder = nn.Linear(node_features, hidden_dim)
        self.convs = nn.ModuleList()
        for i in range(num_layers):
            self.convs.append(
                GATConv(hidden_dim, hidden_dim, heads=num_heads,
                        concat=False, dropout=dropout, edge_dim=edge_features)
            )
        self.router = nn.Sequential(
            nn.Linear(hidden_dim * 3, hidden_dim),
            nn.ReLU(),
            nn.Dropout(dropout),
            nn.Linear(hidden_dim, hidden_dim // 2),
            nn.ReLU(),
            nn.Dropout(dropout),
            nn.Linear(hidden_dim // 2, 1),
        )

    def forward_single(self, x, edge_index, edge_attr, src, dst, candidates):
        h = F.relu(self.node_encoder(x))
        for conv in self.convs:
            h = F.relu(conv(h, edge_index, edge_attr=edge_attr))
            h = F.dropout(h, p=0.1, training=self.training)

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
# Training functions
# ============================================================
def train_epoch(model, data_list, optimizer, device, batch_size=64):
    model.train()
    total_loss = 0
    correct = 0
    total = 0

    np.random.shuffle(data_list)
    for i in range(0, len(data_list), batch_size):
        batch = data_list[i:i+batch_size]
        optimizer.zero_grad()

        losses = []
        for sample in batch:
            x = sample['x'].to(device)
            edge_index = sample['edge_index'].to(device)
            edge_attr = sample['edge_attr'].to(device)
            src = sample['src']
            dst = sample['dst']
            candidates = sample['candidates'].to(device)
            label = torch.tensor(sample['label'], dtype=torch.long, device=device)

            scores = model.forward_single(x, edge_index, edge_attr, src, dst, candidates)
            loss = F.cross_entropy(scores.unsqueeze(0), label.unsqueeze(0))
            losses.append(loss)

            pred = scores.argmax().item()
            correct += (1 if pred == sample['label'] else 0)
            total += 1

        total_loss = sum(l.item() for l in losses)
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
        x = sample['x'].to(device)
        edge_index = sample['edge_index'].to(device)
        edge_attr = sample['edge_attr'].to(device)
        src = sample['src']
        dst = sample['dst']
        candidates = sample['candidates'].to(device)
        label = torch.tensor(sample['label'], dtype=torch.long, device=device)

        scores = model.forward_single(x, edge_index, edge_attr, src, dst, candidates)
        loss = F.cross_entropy(scores.unsqueeze(0), label.unsqueeze(0))
        total_loss += loss.item()

        pred = scores.argmax().item()
        correct += (1 if pred == sample['label'] else 0)
        total += 1

    return total_loss / max(total, 1), correct / max(total, 1)


# ============================================================
# Main training loop
# ============================================================
print(f"\n{'=' * 70}")
print("TRAINING")
print(f"{'=' * 70}")

device = torch.device("cuda" if torch.cuda.is_available() else "cpu")
print(f"  Device: {device}")
if torch.cuda.is_available():
    print(f"  GPU: {torch.cuda.get_device_name(0)}")

model = MeshGAT(
    node_features=5,
    edge_features=2,
    hidden_dim=64,
    num_layers=2,
    num_heads=4,
    dropout=0.1,
).to(device)

num_params = sum(p.numel() for p in model.parameters())
print(f"  Parameters: {num_params:,}")

optimizer = torch.optim.AdamW(model.parameters(), lr=1e-3, weight_decay=1e-4)
scheduler = CosineAnnealingLR(optimizer, T_max=50)

best_val_acc = 0
patience_counter = 0
training_history = []
epochs = 50

for epoch in range(1, epochs + 1):
    start = time.time()
    train_loss, train_acc = train_epoch(model, train_data, optimizer, device)
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

    print(f"  Epoch {epoch:3d}/{epochs} | "
          f"Train: {train_loss:.4f} ({train_acc:.1%}) | "
          f"Val: {val_loss:.4f} ({val_acc:.1%}) | "
          f"Time: {epoch_time:.1f}s")

    if val_acc > best_val_acc:
        best_val_acc = val_acc
        patience_counter = 0
        torch.save({
            "epoch": epoch,
            "model_state_dict": model.state_dict(),
            "optimizer_state_dict": optimizer.state_dict(),
            "val_acc": val_acc,
            "val_loss": val_loss,
            "config": {
                "node_features": 5,
                "edge_features": 2,
                "hidden_dim": 64,
                "num_layers": 2,
                "num_heads": 4,
                "num_nodes": num_nodes,
            },
        }, MODEL_DIR / "gnn_routing_best.pt")
    else:
        patience_counter += 1
        if patience_counter >= 10:
            print(f"  Early stopping at epoch {epoch}")
            break

# Test evaluation
print(f"\n{'=' * 70}")
print("TEST EVALUATION")
print(f"{'=' * 70}")
checkpoint = torch.load(MODEL_DIR / "gnn_routing_best.pt", weights_only=True)
model.load_state_dict(checkpoint["model_state_dict"])
test_loss, test_acc = evaluate(model, test_data, device)
print(f"  Test Loss: {test_loss:.4f}")
print(f"  Test Accuracy: {test_acc:.1%}")
print(f"  Best Val Accuracy: {best_val_acc:.1%}")

# Save training history
with open(MODEL_DIR / "training_history.json", 'w') as f:
    json.dump(training_history, f, indent=2)

# Summary
print(f"\n{'=' * 70}")
print("TRAINING COMPLETE")
print(f"{'=' * 70}")
print(f"  Dataset: Guifi.net Guifisants (real mesh: {num_nodes} nodes)")
print(f"  Test Accuracy: {test_acc:.1%}")
print(f"  Best Val Accuracy: {best_val_acc:.1%}")
print(f"  Model: {MODEL_DIR / 'gnn_routing_best.pt'}")
print(f"\nNext: Wire into Rust for periodic routing table optimization")
