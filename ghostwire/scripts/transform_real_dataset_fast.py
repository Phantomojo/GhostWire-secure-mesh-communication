#!/usr/bin/env python3
"""
Transform Real WSN Dataset to GhostWire Format (Fast Version)
Creates network topologies from WSN node features
"""

import pandas as pd
import numpy as np
import json
from pathlib import Path
from tqdm import tqdm
import random

# Paths
SCRIPT_DIR = Path(__file__).resolve().parent
REPO_DIR = SCRIPT_DIR.parent  # ghostwire/
INPUT_PATH = REPO_DIR / "data" / "real" / "besra_wsn_dataset.csv"
OUTPUT_PATH = REPO_DIR / "data" / "ghostwire_mesh_real_wsn.json"

print("=" * 70)
print("TRANSFORMING REAL WSN DATASET (FAST)")
print("=" * 70)

# Load dataset
print(f"\nLoading WSN dataset...")
df = pd.read_csv(INPUT_PATH)
print(f"  Loaded {len(df):,} nodes")

# Create network scenarios from node features
print(f"\nCreating network scenarios...")

examples = []
num_scenarios = 100  # Create 100 network scenarios

for scenario_idx in tqdm(range(num_scenarios)):
    # Sample nodes for this scenario
    scenario_size = random.randint(20, 80)
    nodes_df = df.sample(n=min(scenario_size, len(df)), random_state=scenario_idx)
    
    num_nodes = len(nodes_df)
    node_ids = nodes_df['Node_ID'].values
    
    # Create adjacency based on link stability and neighbor count
    adj = np.zeros((num_nodes, num_nodes), dtype=np.float32)
    link_qualities = np.zeros((num_nodes, num_nodes), dtype=np.float32)
    
    for i in range(num_nodes):
        row = nodes_df.iloc[i]
        num_neighbors = min(int(row['Neighbor_Node_Count']), num_nodes - 1)
        link_stability = row['Link_Stability_Index']
        
        # Connect to random neighbors
        possible_neighbors = list(range(num_nodes))
        possible_neighbors.remove(i)
        neighbors = random.sample(possible_neighbors, min(num_neighbors, len(possible_neighbors)))
        
        for neighbor in neighbors:
            # Quality based on link stability and routing path stability
            quality = link_stability * row['Routing_Path_Stability']
            adj[i, neighbor] = 1.0
            adj[neighbor, i] = 1.0
            link_qualities[i, neighbor] = quality
            link_qualities[neighbor, i] = quality
    
    # Ensure connectivity
    for i in range(num_nodes):
        if adj[i].sum() == 0:
            neighbor = (i + 1) % num_nodes
            adj[i, neighbor] = 1.0
            adj[neighbor, i] = 1.0
            link_qualities[i, neighbor] = 0.5
            link_qualities[neighbor, i] = 0.5
    
    # Generate routing decisions
    for _ in range(10):  # 10 routing decisions per scenario
        src = random.randint(0, num_nodes - 1)
        dst = random.randint(0, num_nodes - 1)
        while dst == src:
            dst = random.randint(0, num_nodes - 1)
        
        # Find best next hop
        neighbors = np.where(adj[src] > 0)[0]
        if len(neighbors) == 0:
            continue
        
        # Score neighbors by link quality + progress toward destination
        best_next_hop = None
        best_score = -1
        
        for neighbor in neighbors:
            quality = link_qualities[src, neighbor]
            progress = 1.0 - (abs(neighbor - dst) / num_nodes)
            score = quality * 0.6 + progress * 0.4
            
            if score > best_score:
                best_score = score
                best_next_hop = neighbor
        
        if best_next_hop is not None:
            # Encode topology
            edges = []
            for i in range(num_nodes):
                for j in range(i+1, num_nodes):
                    if adj[i, j] > 0:
                        quality_int = int(link_qualities[i, j] * 99)
                        edges.append((i, j, quality_int))
            
            tokens = [num_nodes, len(edges), src, dst]
            for (i, j, quality) in edges[:60]:
                tokens.extend([i, j, quality])
            
            while len(tokens) < 128:
                tokens.append(-1)
            
            examples.append({
                "inputs": [int(x) for x in tokens[:128]],
                "labels": [int(best_next_hop)],
                "puzzle_identifiers": [0],
                "metadata": {
                    "num_nodes": int(num_nodes),
                    "num_edges": int(adj.sum() / 2),
                    "path_length": 2,
                    "environment": "real_wsn",
                    "is_real": True,
                    "source": "besra_wsn_dataset",
                    "scenario": scenario_idx,
                }
            })

# Shuffle and save
print(f"\nShuffling {len(examples):,} examples...")
random.shuffle(examples)

print(f"\nSaving to {OUTPUT_PATH}...")
dataset = {
    "examples": examples,
    "metadata": {
        "num_samples": len(examples),
        "source": "besra_wsn_dataset",
        "is_real": True,
        "generated_at": "2026-04-03",
        "num_scenarios": num_scenarios,
    }
}

with open(OUTPUT_PATH, 'w') as f:
    json.dump(dataset, f)

file_size_mb = OUTPUT_PATH.stat().st_size / 1024 / 1024
print(f"\n{'='*70}")
print("TRANSFORMATION COMPLETE")
print(f"{'='*70}")
print(f"\nOutput: {OUTPUT_PATH}")
print(f"Size: {file_size_mb:.1f} MB")
print(f"Samples: {len(examples):,}")
print(f"Scenarios: {num_scenarios}")
