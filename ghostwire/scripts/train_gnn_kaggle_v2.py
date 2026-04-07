# ======================================================================
# GHOSTWIRE L3 GNN TRAINING - Real Guifi.net Mesh Data
# Graph Attention Network for next-hop routing prediction
# Dataset: 63 nodes, 7931 temporal snapshots, real routing tables
# ======================================================================

import os
import sys
import json
import time
import bz2
import pickle
import zipfile
import subprocess
import numpy as np
import torch
import torch.nn as nn
import torch.nn.functional as F
from torch.optim.lr_scheduler import CosineAnnealingLR
from pathlib import Path
from tqdm import tqdm

print("=" * 70)
print("GHOSTWIRE L3 GNN - Real Guifi.net Mesh Training")
print("=" * 70)

# Check GPU
print(f"\nCUDA Available: {torch.cuda.is_available()}")
if torch.cuda.is_available():
    gpu_name = torch.cuda.get_device_name(0)
    gpu_memory = torch.cuda.get_device_properties(0).total_memory / 1024**3
    print(f"GPU: {gpu_name}")
    print(f"Memory: {gpu_memory:.1f} GB")
    device = torch.device("cuda")
else:
    print("Using CPU")
    device = torch.device("cpu")

# Install torch-geometric (don't touch torch - Kaggle has it pre-installed)
print("\nInstalling torch-geometric...")
subprocess.run([sys.executable, "-m", "pip", "install", "-q", "torch-geometric", "--no-deps"], check=True)
from torch_geometric.nn import GATConv
print("Dependencies installed")

# ======================================================================
# Load REAL Guifi.net dataset
# ======================================================================
print("\nLoading Guifi.net mesh dataset...")

possible_paths = [
    Path("/kaggle/input/guifi-net-guifisants-mesh-topology"),
    Path("/kaggle/input/guifi_net_guifisants_mesh_topology"),
]

data_file = None
for p in possible_paths:
    if p.exists():
        # Check for direct pickle file
        rt_file = p / "dataset_meshmon_rt.pkl.bz2"
        if rt_file.exists():
            data_file = rt_file
            print(f"  Found: {rt_file}")
            break
        # Check for zip
        zf = p / "dataverse_files.zip"
        if zf.exists():
            print(f"  Extracting: {zf}")
            with zipfile.ZipFile(zf, 'r') as z:
                z.extractall("/tmp/guifi_data")
            data_file = Path("/tmp/guifi_data/dataset_meshmon_rt.pkl.bz2")
            break

if data_file is None or not data_file.exists():
    print("  Dataset not found! Listing /kaggle/input/:")
    input_dir = Path("/kaggle/input")
    if input_dir.exists():
        for item in input_dir.iterdir():
            print(f"    {item}")
    print("ERROR: Cannot proceed without dataset.")
    sys.exit(1)

print(f"  Loading from: {data_file}")
with bz2.open(data_file, 'rb') as f:
    rt_data = pickle.load(f)

adj_list = rt_data['adj']
routing_tables = rt_data['rt']
metrics = rt_data['metric']
num_nodes = adj_list[0].shape[0]

print(f"  Nodes: {num_nodes}")
print(f"  Snapshots: {len(adj_list)}")
print(f"  Edges (first): {np.count_nonzero(adj_list[0])}")
print(f"  Routing shape: {routing_tables[0].shape}")

# ======================================================================
# Build graph samples
# ======================================================================
print("\nBuilding graph samples...")

def build_graph_samples(routing_tables, metrics, adj_list, max_samples=7931):
    num_nodes = routing_tables[0].shape[0]
    graphs = []
    rng = np.random.RandomState(42)
    indices = rng.choice(len(routing_tables), min(max_samples, len(routing_tables)), replace=False)
    samples_per_graph = 10

    for idx in tqdm(indices, desc="Building graphs"):
        snap_idx = idx % len(adj_list)
        routing = routing_tables[idx]
        metric = metrics[idx]
        adj = adj_list[snap_idx]

        # Node features (8 dims)
        node_features = np.zeros((num_nodes, 8), dtype=np.float32)
        for i in range(num_nodes):
            out_deg = (routing[i] >= 0).sum()
            in_deg = (routing[:, i] >= 0).sum()
            valid = metric[i][metric[i] != -1]
            avg_q = valid.mean() if len(valid) > 0 else 0
            valid_hops = routing[i][routing[i] >= 0]
            avg_h = valid_hops.mean() / num_nodes if len(valid_hops) > 0 else 0
            transit = (routing[:, :] == i).sum() / num_nodes
            link_div = len(valid) / num_nodes if len(valid) > 0 else 0
            adj_deg = np.count_nonzero(adj[i]) / num_nodes
            adj_str = adj[i].sum() / max(np.count_nonzero(adj[i]), 1)
            node_features[i] = [out_deg/num_nodes, in_deg/num_nodes, avg_q/1e8, avg_h, transit, link_div, adj_deg, adj_str]

        # Build edges
        edge_list = []
        edge_attrs = []
        for i in range(num_nodes):
            for j in range(num_nodes):
                if routing[i, j] >= 0:
                    edge_list.append([i, j])
                    lm = metric[i, j] if metric[i, j] != -1 else 0
                    edge_attrs.append([lm / 1e8, 1.0 if routing[i, j] == j else 0.0])

        if not edge_list:
            continue

        edge_index = np.array(edge_list, dtype=np.int64).T
        edge_attr = np.array(edge_attrs, dtype=np.float32)

        # Extract training samples
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

    print(f"  Generated {len(graphs):,} samples")
    return graphs

graphs = build_graph_samples(routing_tables, metrics, adj_list)

# Train/val/test split
np.random.seed(42)
indices = np.random.permutation(len(graphs))
n = len(graphs)
train_end = int(0.8 * n)
val_end = int(0.9 * n)
train_data = [graphs[i] for i in indices[:train_end]]
val_data = [graphs[i] for i in indices[train_end:val_end]]
test_data = [graphs[i] for i in indices[val_end:]]
print(f"  Train: {len(train_data):,} | Val: {len(val_data):,} | Test: {len(test_data):,}")

# ======================================================================
# Model: GAT with residual connections
# ======================================================================
class MeshGAT(nn.Module):
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
        for conv, norm in zip(self.convs, self.norms):
            h_new = conv(h, edge_index, edge_attr=edge_attr)
            h_new = norm(h_new)
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

# ======================================================================
# Training
# ======================================================================
CONFIG = {
    "node_features": 8, "edge_features": 2, "hidden_dim": 128,
    "num_layers": 4, "num_heads": 8, "dropout": 0.15,
    "batch_size": 64, "epochs": 150, "lr": 5e-4,
    "weight_decay": 1e-4, "patience": 20, "seed": 42,
}

OUTPUT_DIR = Path("/kaggle/working/models/gnn_routing")
OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
np.random.seed(CONFIG["seed"])
torch.manual_seed(CONFIG["seed"])

model = MeshGAT(
    node_features=CONFIG["node_features"], edge_features=CONFIG["edge_features"],
    hidden_dim=CONFIG["hidden_dim"], num_layers=CONFIG["num_layers"],
    num_heads=CONFIG["num_heads"], dropout=CONFIG["dropout"],
).to(device)

num_params = sum(p.numel() for p in model.parameters())
print(f"\nModel: {num_params:,} parameters on {device}")

optimizer = torch.optim.AdamW(model.parameters(), lr=CONFIG["lr"], weight_decay=CONFIG["weight_decay"])
scheduler = CosineAnnealingLR(optimizer, T_max=CONFIG["epochs"])

def train_epoch(model, data_list, optimizer, device, batch_size=64):
    model.train()
    total_loss = 0
    correct = 0
    total = 0
    np.random.shuffle(data_list)
    for i in tqdm(range(0, len(data_list), batch_size), desc="Training", leave=False):
        batch = data_list[i:i+batch_size]
        optimizer.zero_grad()
        losses = []
        for sample in batch:
            x = torch.tensor(sample['x'], dtype=torch.float, device=device)
            edge_index = torch.tensor(sample['edge_index'], dtype=torch.long, device=device)
            edge_attr = torch.tensor(sample['edge_attr'], dtype=torch.float, device=device)
            src, dst = sample['src'], sample['dst']
            candidates = torch.tensor(sample['candidates'], dtype=torch.long, device=device)
            label = torch.tensor(sample['label'], dtype=torch.long, device=device)
            scores = model.forward_single(x, edge_index, edge_attr, src, dst, candidates)
            loss = F.cross_entropy(scores.unsqueeze(0), label.unsqueeze(0), label_smoothing=0.1)
            losses.append(loss)
            pred = scores.argmax().item()
            correct += (1 if pred == sample['label'] else 0)
            total += 1
        total_loss += sum(l.item() for l in losses)
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
        src, dst = sample['src'], sample['dst']
        candidates = torch.tensor(sample['candidates'], dtype=torch.long, device=device)
        label = torch.tensor(sample['label'], dtype=torch.long, device=device)
        scores = model.forward_single(x, edge_index, edge_attr, src, dst, candidates)
        loss = F.cross_entropy(scores.unsqueeze(0), label.unsqueeze(0))
        total_loss += loss.item()
        pred = scores.argmax().item()
        correct += (1 if pred == sample['label'] else 0)
        total += 1
    return total_loss / max(total, 1), correct / max(total, 1)

# Main loop
best_val_acc = 0
patience_counter = 0
training_history = []
top_checkpoints = []

print(f"\n{'=' * 70}")
print("TRAINING")
print(f"{'=' * 70}")

for epoch in range(1, CONFIG["epochs"] + 1):
    start = time.time()
    train_loss, train_acc = train_epoch(model, train_data, optimizer, device, CONFIG["batch_size"])
    val_loss, val_acc = evaluate(model, val_data, device)
    epoch_time = time.time() - start
    scheduler.step()

    training_history.append({
        "epoch": epoch, "train_loss": train_loss, "train_acc": train_acc,
        "val_loss": val_loss, "val_acc": val_acc, "lr": scheduler.get_last_lr()[0],
    })

    print(f"  Epoch {epoch:3d}/{CONFIG['epochs']} | "
          f"Train: {train_loss:.4f} ({train_acc:.1%}) | "
          f"Val: {val_loss:.4f} ({val_acc:.1%}) | "
          f"Time: {epoch_time:.1f}s")

    if val_acc > best_val_acc:
        best_val_acc = val_acc
        patience_counter = 0
        torch.save({
            "epoch": epoch, "model_state_dict": model.state_dict(),
            "optimizer_state_dict": optimizer.state_dict(),
            "val_acc": val_acc, "val_loss": val_loss, "config": CONFIG,
        }, OUTPUT_DIR / "gnn_routing_best.pt")
        top_checkpoints.append((val_acc, epoch))
        top_checkpoints.sort(reverse=True)
        top_checkpoints = top_checkpoints[:3]
    else:
        patience_counter += 1
        if patience_counter >= CONFIG["patience"]:
            print(f"  Early stopping at epoch {epoch}")
            break

# Test evaluation
print(f"\n{'=' * 70}")
print("TEST EVALUATION")
print(f"{'=' * 70}")
checkpoint = torch.load(OUTPUT_DIR / "gnn_routing_best.pt", weights_only=True)
model.load_state_dict(checkpoint["model_state_dict"])
test_loss, test_acc = evaluate(model, test_data, device)
print(f"  Test Loss: {test_loss:.4f}")
print(f"  Test Accuracy: {test_acc:.1%}")
print(f"  Best Val Accuracy: {best_val_acc:.1%}")

# Save history
with open(OUTPUT_DIR / "training_history.json", 'w') as f:
    json.dump(training_history, f, indent=2)

print(f"\n{'=' * 70}")
print("TRAINING COMPLETE")
print(f"{'=' * 70}")
print(f"  Dataset: Guifi.net Guifisants ({num_nodes} nodes, {len(graphs):,} samples)")
print(f"  Test Accuracy: {test_acc:.1%}")
print(f"  Best Val Accuracy: {best_val_acc:.1%}")
print(f"  Model: {OUTPUT_DIR / 'gnn_routing_best.pt'}")
print(f"  History: {OUTPUT_DIR / 'training_history.json'}")
