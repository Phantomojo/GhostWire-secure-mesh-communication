#!/usr/bin/env python3
"""
GhostWire GNN Training Script — Graph Attention Network for Mesh Routing
Layer 3: Routing table optimization (every 5 minutes, offline)

Architecture:
  Graph Attention Network (GAT) with 3 layers, 4 heads
  Input: Node features (5) + Edge features (2) → Next-hop prediction
  Output: Probability distribution over candidate next-hops

Training:
  - Dataset: 10K synthetic mesh topologies
  - Loss: Cross-entropy over next-hop classes
  - Optimizer: AdamW with cosine annealing
  - Early stopping on validation loss

Export:
  - PyTorch checkpoint (.pt)
  - ONNX model for Rust inference (via ort crate)

Run anywhere:
  Local CPU:    python3 scripts/train_gnn.py
  Kaggle T4:    Paste into Kaggle notebook (set OUTPUT_DIR=/kaggle/working/models/gnn_routing)
  Lightning AI: python3 scripts/train_gnn.py --device cuda
"""

# ============================================================
# Dependency installation (Kaggle environment needs pip install)
# ============================================================
import subprocess
import sys
from pathlib import Path

if Path("/kaggle/working").exists():
    print("🔵 Kaggle environment detected — installing dependencies...")
    # Install torch-geometric only (pyg-lib/torch-scatter are optional accelerators)
    # Kaggle has PyTorch 2.10.0 which may not have prebuilt PyG wheels
    subprocess.check_call([sys.executable, "-m", "pip", "install", "-q",
                          "torch-geometric", "scipy"])
    print("✅ Dependencies installed.")

import os
import torch
import torch.nn as nn
import torch.nn.functional as F
from torch.optim.lr_scheduler import CosineAnnealingLR
import numpy as np
from pathlib import Path
import json
import time
import argparse

# PyTorch Geometric
from torch_geometric.nn import GATConv
from torch_geometric.data import Data
from torch_geometric.loader import DataLoader
from scipy.sparse import csgraph

# ============================================================
# Configuration — auto-detects environment
# ============================================================
SCRIPT_DIR = Path(__file__).parent
PROJECT_DIR = SCRIPT_DIR.parent

# Auto-detect output directory
if Path("/kaggle/working").exists():
    OUTPUT_DIR = Path("/kaggle/working/models/gnn_routing")
    print("🔵 Kaggle environment detected — output to /kaggle/working/")
else:
    OUTPUT_DIR = PROJECT_DIR / "models" / "gnn_routing"

OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

# ============================================================
# Model Definition
# ============================================================
class MeshGAT(nn.Module):
    """Graph Attention Network for mesh routing decisions."""

    def __init__(self, node_features=5, edge_features=2, hidden_dim=128,
                 num_layers=3, num_heads=4, dropout=0.1):
        super().__init__()

        self.node_encoder = nn.Linear(node_features, hidden_dim)

        self.convs = nn.ModuleList()
        for i in range(num_layers):
            in_dim = hidden_dim
            self.convs.append(
                GATConv(in_dim, hidden_dim, heads=num_heads,
                        concat=False, dropout=dropout, edge_dim=edge_features)
            )

        self.edge_encoder = nn.Linear(edge_features, hidden_dim)

        self.router = nn.Sequential(
            nn.Linear(hidden_dim * 3, hidden_dim),  # src + dst + cand = 3*hidden
            nn.ReLU(),
            nn.Dropout(dropout),
            nn.Linear(hidden_dim, hidden_dim // 2),
            nn.ReLU(),
            nn.Dropout(dropout),
            nn.Linear(hidden_dim // 2, 1),  # Score per candidate
        )

    def forward(self, x, edge_index, edge_attr, src_nodes, dst_nodes, candidates,
                batch_vec=None, num_candidates=None):
        """
        PyG native batching: graphs are concatenated, not padded.

        Args:
            x: Node features [total_nodes, node_features]
            edge_index: Edge connectivity [2, total_edges]
            edge_attr: Edge features [total_edges, edge_features]
            src_nodes: Source node indices per graph [batch_size]
            dst_nodes: Destination node indices per graph [batch_size]
            candidates: Candidate next-hop indices [batch_size, max_candidates]
            batch_vec: Graph assignment for each node [total_nodes]
            num_candidates: Number of valid candidates per graph [batch_size]

        Returns:
            scores: Routing scores per candidate [batch_size, max_candidates]
        """
        h = F.relu(self.node_encoder(x))

        for conv in self.convs:
            h = F.relu(conv(h, edge_index, edge_attr=edge_attr))
            h = F.dropout(h, p=0.1, training=self.training)

        # Pool node embeddings per graph using mean pooling
        if batch_vec is None:
            batch_vec = torch.zeros(x.size(0), dtype=torch.long, device=x.device)

        # Mean pool to get graph-level embeddings
        batch_size = num_candidates.size(0) if num_candidates is not None else src_nodes.size(0)
        graph_emb = torch.zeros(batch_size, h.size(1), device=h.device)
        graph_emb.scatter_add_(0, batch_vec.unsqueeze(1).expand(-1, h.size(1)), h)
        counts = torch.bincount(batch_vec, minlength=batch_size).unsqueeze(1).float()
        graph_emb = graph_emb / counts.clamp(min=1)

        # Gather source/destination/candidate embeddings from node embeddings
        src_emb = h[src_nodes]  # [batch, hidden]
        dst_emb = h[dst_nodes]  # [batch, hidden]

        max_candidates = candidates.size(1)
        # candidates shape: [batch, max_candidates] — these are node indices
        batch_idx = torch.arange(batch_size, device=h.device).unsqueeze(1).expand(-1, max_candidates)
        cand_emb = h[batch_idx, candidates]  # [batch, max_candidates, hidden]

        combined = torch.cat([
            src_emb.unsqueeze(1).expand(-1, max_candidates, -1),
            dst_emb.unsqueeze(1).expand(-1, max_candidates, -1),
            cand_emb,
        ], dim=-1)  # [batch, max_candidates, hidden*3]

        scores = self.router(combined).squeeze(-1)  # [batch, max_candidates]
        return scores


# ============================================================
# Dataset Generation
# ============================================================
def generate_mesh_topology(num_nodes, tx_range=200.0):
    """Generate a random mesh topology with distance-based connectivity."""
    positions = np.random.uniform(0, 1000, (num_nodes, 2))

    adj = np.zeros((num_nodes, num_nodes))
    link_qualities = np.zeros((num_nodes, num_nodes))

    for i in range(num_nodes):
        for j in range(i + 1, num_nodes):
            dist = np.sqrt(np.sum((positions[i] - positions[j]) ** 2))
            if dist < tx_range:
                adj[i, j] = 1
                adj[j, i] = 1
                link_qualities[i, j] = 1.0 - (dist / tx_range)
                link_qualities[j, i] = 1.0 - (dist / tx_range)

    # Ensure connectivity: add edges to connect components
    n_components, labels = csgraph.connected_components(adj)
    if n_components > 1:
        for comp in range(1, n_components):
            comp_nodes = np.where(labels == comp)[0]
            other_nodes = np.where(labels != comp)[0]
            min_dist = np.inf
            best_i, best_j = -1, -1
            for ci in comp_nodes:
                for oj in other_nodes:
                    d = np.sqrt(np.sum((positions[ci] - positions[oj]) ** 2))
                    if d < min_dist:
                        min_dist = d
                        best_i, best_j = ci, oj
            if best_i >= 0:
                adj[best_i, best_j] = 1
                adj[best_j, best_i] = 1
                link_qualities[best_i, best_j] = max(0.3, 1.0 - min_dist / tx_range)
                link_qualities[best_j, best_i] = link_qualities[best_i, best_j]

    return adj, link_qualities, positions


def dijkstra(adj, src, dst):
    """Find shortest path using Dijkstra's algorithm."""
    num_nodes = adj.shape[0]
    dist = np.full(num_nodes, np.inf)
    dist[src] = 0
    prev = np.full(num_nodes, -1, dtype=int)
    visited = np.zeros(num_nodes, dtype=bool)

    for _ in range(num_nodes):
        min_dist = np.inf
        u = -1
        for i in range(num_nodes):
            if not visited[i] and dist[i] < min_dist:
                min_dist = dist[i]
                u = i
        if u == -1 or u == dst:
            break
        visited[u] = True
        for v in range(num_nodes):
            if adj[u, v] > 0 and not visited[v]:
                alt = dist[u] + 1
                if alt < dist[v]:
                    dist[v] = alt
                    prev[v] = u

    path = []
    current = dst
    while current != -1:
        path.append(current)
        current = prev[current]
    if path and path[-1] == src:
        return path[::-1]
    return []


def topology_to_graph(adj, link_qualities, positions, src, dst, max_candidates=5):
    """Convert network topology to PyG-compatible graph data."""
    num_nodes = adj.shape[0]

    # Node features
    degrees = adj.sum(axis=1)
    avg_neighbor_quality = np.zeros(num_nodes)
    for i in range(num_nodes):
        neighbors = np.where(adj[i] > 0)[0]
        if len(neighbors) > 0:
            avg_neighbor_quality[i] = link_qualities[i, neighbors].mean()

    dist_to_dst = np.sqrt(np.sum((positions - positions[dst]) ** 2, axis=1))
    max_dist = dist_to_dst.max()
    if max_dist > 0:
        dist_to_dst = dist_to_dst / max_dist

    node_features = np.column_stack([
        degrees / max(num_nodes, 1),
        avg_neighbor_quality,
        dist_to_dst,
        (np.arange(num_nodes) == src).astype(float),
        (np.arange(num_nodes) == dst).astype(float),
    ])

    # Edge index and attributes
    edge_list = []
    edge_attrs = []
    for i in range(num_nodes):
        for j in range(num_nodes):
            if adj[i, j] > 0:
                edge_list.append([i, j])
                dist = np.sqrt(np.sum((positions[i] - positions[j]) ** 2))
                edge_attrs.append([link_qualities[i, j], dist / 1000.0])

    if not edge_list:
        return None

    edge_index = torch.tensor(edge_list, dtype=torch.long).t().contiguous()
    edge_attr = torch.tensor(edge_attrs, dtype=torch.float)
    x = torch.tensor(node_features, dtype=torch.float)

    # Find candidates (neighbors of source)
    neighbors = np.where(adj[src] > 0)[0]
    if len(neighbors) == 0:
        return None

    # Select up to max_candidates
    if len(neighbors) > max_candidates:
        top_idx = np.argsort(-link_qualities[src, neighbors])[:max_candidates]
        candidates = neighbors[top_idx]
    else:
        candidates = neighbors

    # Find optimal next-hop
    path = dijkstra(adj, src, dst)
    if len(path) < 2:
        return None

    optimal_next_hop = path[1]

    # Create label: index within candidates
    try:
        label_idx = list(candidates).index(optimal_next_hop)
    except ValueError:
        best_cand = max(candidates, key=lambda c: link_qualities[src, c])
        label_idx = list(candidates).index(best_cand)

    data = Data(
        x=x,
        edge_index=edge_index,
        edge_attr=edge_attr,
        src=torch.tensor(src, dtype=torch.long),
        dst=torch.tensor(dst, dtype=torch.long),
        candidates=torch.tensor(candidates, dtype=torch.long),
        label=torch.tensor(label_idx, dtype=torch.long),
        num_candidates=torch.tensor(len(candidates), dtype=torch.long),
    )
    return data


def generate_dataset(num_samples=10000):
    """Generate GNN training dataset."""
    print(f"Generating {num_samples:,} GNN samples...")
    graphs = []
    failed = 0

    for _ in range(num_samples):
        num_nodes = np.random.randint(20, 80)
        adj, link_qualities, positions = generate_mesh_topology(num_nodes)

        src = np.random.randint(0, num_nodes)
        dst = np.random.randint(0, num_nodes)
        while dst == src:
            dst = np.random.randint(0, num_nodes)

        data = topology_to_graph(adj, link_qualities, positions, src, dst)
        if data is not None:
            graphs.append(data)
        else:
            failed += 1

    print(f"  Generated {len(graphs):,} valid samples ({failed} failed)")
    return graphs


# ============================================================
# Collation and Training
# ============================================================
def collate_fn(batch):
    """Custom collate that returns a dict to avoid PyG Batch issues."""
    # Return list of individual Data objects — we'll process them one by one
    return batch  # Just return the list of Data objects


def train_epoch(model, loader, optimizer, device):
    """Single training epoch — batch_size=1, one graph at a time."""
    model.train()
    total_loss = 0
    correct = 0
    total = 0

    for data in loader:
        x = data.x.to(device)
        edge_index = data.edge_index.to(device)
        edge_attr = data.edge_attr.to(device)
        # PyG batch_size=1 wraps scalars as [1] tensors — squeeze them
        src_node = data.src.squeeze().to(device)
        dst_node = data.dst.squeeze().to(device)
        candidates = data.candidates.to(device)  # [num_candidates] or [1, num_candidates]
        if candidates.dim() == 1:
            candidates = candidates.unsqueeze(0)
        label = data.label.squeeze().to(device)

        optimizer.zero_grad()

        h = F.relu(model.node_encoder(x))
        for conv in model.convs:
            h = F.relu(conv(h, edge_index, edge_attr=edge_attr))
            h = F.dropout(h, p=0.1, training=model.training)

        src_emb = h[src_node]  # [hidden]
        dst_emb = h[dst_node]  # [hidden]
        cand_emb = h[candidates.squeeze(0)]  # [num_candidates, hidden]

        num_cand = candidates.size(1)
        combined = torch.cat([
            src_emb.unsqueeze(0).expand(num_cand, -1),
            dst_emb.unsqueeze(0).expand(num_cand, -1),
            cand_emb,
        ], dim=-1)

        scores = model.router(combined).squeeze(-1).unsqueeze(0)
        loss = F.cross_entropy(scores, label.unsqueeze(0))
        loss.backward()
        torch.nn.utils.clip_grad_norm_(model.parameters(), max_norm=1.0)
        optimizer.step()

        total_loss += loss.item()
        pred = scores.argmax(dim=1)
        correct += (pred == label).sum().item()
        total += 1

    return total_loss / max(total, 1), correct / max(total, 1)


@torch.no_grad()
def evaluate(model, loader, device):
    """Evaluate model — batch_size=1, one graph at a time."""
    model.eval()
    total_loss = 0
    correct = 0
    total = 0

    for data in loader:
        x = data.x.to(device)
        edge_index = data.edge_index.to(device)
        edge_attr = data.edge_attr.to(device)
        src_node = data.src.squeeze().to(device)
        dst_node = data.dst.squeeze().to(device)
        candidates = data.candidates.to(device)
        if candidates.dim() == 1:
            candidates = candidates.unsqueeze(0)
        label = data.label.squeeze().to(device)

        h = F.relu(model.node_encoder(x))
        for conv in model.convs:
            h = F.relu(conv(h, edge_index, edge_attr=edge_attr))

        src_emb = h[src_node]
        dst_emb = h[dst_node]
        cand_emb = h[candidates.squeeze(0)]

        num_cand = candidates.size(1)
        combined = torch.cat([
            src_emb.unsqueeze(0).expand(num_cand, -1),
            dst_emb.unsqueeze(0).expand(num_cand, -1),
            cand_emb,
        ], dim=-1)

        scores = model.router(combined).squeeze(-1).unsqueeze(0)
        loss = F.cross_entropy(scores, label.unsqueeze(0))

        total_loss += loss.item()
        pred = scores.argmax(dim=1)
        correct += (pred == label).sum().item()
        total += 1

    return total_loss / max(total, 1), correct / max(total, 1)


# ============================================================
# ONNX Export
# ============================================================
def export_to_onnx(model, device, output_path):
    """Export DNN to ONNX format."""
    model.eval()
    model.to("cpu")

    # Create dummy input
    batch_size = 1
    max_nodes = 50
    max_edges = 200
    max_candidates = 5
    node_features = 5
    edge_features = 2

    x = torch.randn(batch_size, max_nodes, node_features)
    edge_index = torch.randint(0, max_nodes, (batch_size, 2, max_edges), dtype=torch.long)
    edge_attr = torch.randn(batch_size, max_edges, edge_features)
    src_nodes = torch.randint(0, max_nodes, (batch_size,), dtype=torch.long)
    dst_nodes = torch.randint(0, max_nodes, (batch_size,), dtype=torch.long)
    candidates = torch.randint(0, max_nodes, (batch_size, max_candidates), dtype=torch.long)

    input_names = ["x", "edge_index", "edge_attr", "src_nodes", "dst_nodes", "candidates"]
    dynamic_axes = {
        "x": {0: "batch", 1: "num_nodes"},
        "edge_index": {0: "batch", 2: "num_edges"},
        "edge_attr": {0: "batch", 1: "num_edges"},
        "src_nodes": {0: "batch"},
        "dst_nodes": {0: "batch"},
        "candidates": {0: "batch", 1: "num_candidates"},
        "scores": {0: "batch", 1: "num_candidates"},
    }

    try:
        torch.onnx.export(
            model,
            (x, edge_index, edge_attr, src_nodes, dst_nodes, candidates),
            output_path,
            input_names=input_names,
            output_names=["scores"],
            dynamic_axes=dynamic_axes,
            opset_version=15,
        )
        size_kb = Path(output_path).stat().st_size / 1024
        print(f"  ✅ ONNX export: {output_path} ({size_kb:.1f} KB)")
        return True
    except Exception as e:
        print(f"  ⚠️  ONNX export failed: {e}")
        return False


# ============================================================
# Main Training Loop
# ============================================================
def main():
    parser = argparse.ArgumentParser(description="Train GhostWire GNN routing model")
    parser.add_argument("--epochs", type=int, default=30)  # Reduced for CPU
    parser.add_argument("--batch-size", type=int, default=32)
    parser.add_argument("--hidden-dim", type=int, default=128)
    parser.add_argument("--num-heads", type=int, default=4)
    parser.add_argument("--num-layers", type=int, default=3)
    parser.add_argument("--lr", type=float, default=1e-3)
    parser.add_argument("--num-samples", type=int, default=2000)  # Reduced for CPU training
    parser.add_argument("--patience", type=int, default=15)
    parser.add_argument("--device", type=str, default=None)
    args = parser.parse_args()

    print("=" * 70)
    print("GHOSTWIRE GNN TRAINING — Graph Attention Network")
    print("=" * 70)

    # Device — force CPU on Kaggle P100 (CUDA 6.0 incompatible with PyTorch 2.10)
    if Path("/kaggle/working").exists():
        # Kaggle free tier gives P100 (sm_60) which PyTorch 2.10 doesn't support
        device = torch.device("cpu")
        print(f"\n⚠️  Kaggle P100 detected — forcing CPU (P100 sm_60 incompatible with PyTorch 2.10)")
    elif args.device:
        device = torch.device(args.device)
    elif torch.cuda.is_available():
        capability = torch.cuda.get_device_capability(0)
        if capability[0] < 7:
            device = torch.device("cpu")
            print(f"\n⚠️  GPU capability {capability[0]}.{capability[1]} too old — forcing CPU")
        else:
            device = torch.device("cuda")
    else:
        device = torch.device("cpu")
    print(f"Device: {device}")
    if torch.cuda.is_available() and device.type == "cuda":
        print(f"  GPU: {torch.cuda.get_device_name(0)}")
        print(f"  VRAM: {torch.cuda.get_device_properties(0).total_memory / 1e9:.1f} GB")

    # Generate dataset
    print(f"\n{'=' * 70}")
    print("GENERATING DATASET")
    print(f"{'=' * 70}")
    start = time.time()
    graphs = generate_dataset(args.num_samples)
    gen_time = time.time() - start
    print(f"  Generation time: {gen_time:.1f}s")

    # Split: 80/10/10
    np.random.seed(42)
    indices = np.random.permutation(len(graphs))
    n = len(graphs)
    train_end = int(0.8 * n)
    val_end = int(0.9 * n)

    train_graphs = [graphs[i] for i in indices[:train_end]]
    val_graphs = [graphs[i] for i in indices[train_end:val_end]]
    test_graphs = [graphs[i] for i in indices[val_end:]]

    print(f"  Train: {len(train_graphs):,} | Val: {len(val_graphs):,} | Test: {len(test_graphs):,}")

    # Data loaders — batch_size=1 to avoid PyG batching issues with custom attributes
    train_loader = DataLoader(train_graphs, batch_size=1, shuffle=True)
    val_loader = DataLoader(val_graphs, batch_size=1, shuffle=False)
    test_loader = DataLoader(test_graphs, batch_size=1, shuffle=False)

    # Model
    print(f"\n{'=' * 70}")
    print("MODEL ARCHITECTURE")
    print(f"{'=' * 70}")
    model = MeshGAT(
        node_features=5,
        edge_features=2,
        hidden_dim=args.hidden_dim,
        num_layers=args.num_layers,
        num_heads=args.num_heads,
        dropout=0.1,
    )
    model = model.to(device)
    num_params = sum(p.numel() for p in model.parameters())
    print(f"  Parameters: {num_params:,}")
    print(f"  Architecture: GAT ({args.num_layers} layers, {args.num_heads} heads)")
    print(f"  Hidden dim: {args.hidden_dim}")

    # Training
    print(f"\n{'=' * 70}")
    print("TRAINING")
    print(f"{'=' * 70}")
    optimizer = torch.optim.AdamW(model.parameters(), lr=args.lr, weight_decay=1e-4)
    scheduler = CosineAnnealingLR(optimizer, T_max=args.epochs)

    best_val_acc = 0
    best_val_loss = float('inf')
    patience_counter = 0
    training_history = []

    for epoch in range(1, args.epochs + 1):
        start = time.time()
        train_loss, train_acc = train_epoch(model, train_loader, optimizer, device)
        val_loss, val_acc = evaluate(model, val_loader, device)
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

        print(f"  Epoch {epoch:3d}/{args.epochs} | "
              f"Train: {train_loss:.4f} ({train_acc:.1%}) | "
              f"Val: {val_loss:.4f} ({val_acc:.1%}) | "
              f"LR: {scheduler.get_last_lr()[0]:.6f} | "
              f"Time: {epoch_time:.1f}s")
        sys.stdout.flush()  # Ensure output is visible on Kaggle

        # Early stopping
        if val_acc > best_val_acc:
            best_val_acc = val_acc
            best_val_loss = val_loss
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
                    "hidden_dim": args.hidden_dim,
                    "num_layers": args.num_layers,
                    "num_heads": args.num_heads,
                },
            }, OUTPUT_DIR / "gnn_routing_best.pt")
        else:
            patience_counter += 1
            if patience_counter >= args.patience:
                print(f"\n  ⏹️  Early stopping at epoch {epoch} (patience={args.patience})")
                break

    # Test evaluation
    print(f"\n{'=' * 70}")
    print("TEST EVALUATION")
    print(f"{'=' * 70}")
    # Load best model
    checkpoint = torch.load(OUTPUT_DIR / "gnn_routing_best.pt", weights_only=True)
    model.load_state_dict(checkpoint["model_state_dict"])
    test_loss, test_acc = evaluate(model, test_loader, device)
    print(f"  Test Loss: {test_loss:.4f}")
    print(f"  Test Accuracy: {test_acc:.1%}")
    print(f"  Best Val Accuracy: {best_val_acc:.1%}")

    # Export to ONNX
    print(f"\n{'=' * 70}")
    print("ONNX EXPORT")
    print(f"{'=' * 70}")
    export_to_onnx(model, device, OUTPUT_DIR / "gnn_routing.onnx")

    # Save training history
    history_path = OUTPUT_DIR / "training_history.json"
    with open(history_path, 'w') as f:
        json.dump(training_history, f, indent=2)
    print(f"  Training history: {history_path}")

    # Summary
    print(f"\n{'=' * 70}")
    print("TRAINING COMPLETE")
    print(f"{'=' * 70}")
    print(f"  Test Accuracy: {test_acc:.1%}")
    print(f"  Best Val Accuracy: {best_val_acc:.1%}")
    print(f"  Model: {OUTPUT_DIR / 'gnn_routing_best.pt'}")
    print(f"  ONNX:  {OUTPUT_DIR / 'gnn_routing.onnx'}")
    print(f"\nNext: Wire ONNX model into Rust for periodic routing table optimization")


if __name__ == "__main__":
    main()
