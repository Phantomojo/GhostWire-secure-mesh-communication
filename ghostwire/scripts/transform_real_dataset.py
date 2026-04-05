#!/usr/bin/env python3
"""
Transform Real WSN Dataset to GhostWire Format
Converts Besra WSN dataset to our topology + routing format
"""

import pandas as pd
import numpy as np
import json
from pathlib import Path
from tqdm import tqdm

# Paths
INPUT_PATH = Path("/home/ph/GhostWire-secure-mesh-communication/ghostwire/data/real/besra_wsn_dataset.csv")
OUTPUT_PATH = Path("/home/ph/GhostWire-secure-mesh-communication/ghostwire/data/ghostwire_mesh_real_wsn.json")

print("=" * 70)
print("TRANSFORMING REAL WSN DATASET TO GHOSTWIRE FORMAT")
print("=" * 70)

# Load dataset
print(f"\nLoading WSN dataset from: {INPUT_PATH}")
df = pd.read_csv(INPUT_PATH)
print(f"  Loaded {len(df):,} records")
print(f"  Columns: {len(df.columns)}")
print(f"  Attack types: {df['Attack_Type'].nunique()}")

# Analyze data
print(f"\nDataset Statistics:")
print(f"  Nodes: {df['Node_ID'].nunique()}")
print(f"  Avg neighbors: {df['Neighbor_Node_Count'].mean():.1f}")
print(f"  Avg link stability: {df['Link_Stability_Index'].mean():.3f}")
print(f"  Avg routing path stability: {df['Routing_Path_Stability'].mean():.3f}")

def encode_topology(adj, link_qualities, src, dst, num_nodes, max_seq_len=128):
    """Encode topology to token sequence"""
    # Extract edges with qualities
    edges = []
    for i in range(num_nodes):
        for j in range(i+1, num_nodes):
            if adj[i, j] > 0:
                quality_int = int(link_qualities[i, j] * 99)
                edges.append((i, j, quality_int))
    
    # Create token sequence
    tokens = [num_nodes, len(edges), src, dst]
    
    for (i, j, quality) in edges[:60]:
        tokens.extend([i, j, quality])
    
    # Pad to fixed length
    while len(tokens) < max_seq_len:
        tokens.append(-1)
    
    return tokens[:max_seq_len]

# Create network topologies from WSN data
print(f"\nCreating network topologies...")

# Group nodes by attack type to create different network scenarios
attack_types = df['Attack_Type'].unique()
print(f"  Attack types: {list(attack_types)}")

examples = []

for attack_type in tqdm(attack_types):
    # Get nodes for this attack scenario
    attack_df = df[df['Attack_Type'] == attack_type]
    
    # Create adjacency matrix based on neighbor counts and link stability
    nodes = attack_df['Node_ID'].unique()
    num_nodes = len(nodes)
    
    if num_nodes < 5:
        continue
    
    # Create node index mapping
    node_to_idx = {node: i for i, node in enumerate(nodes)}
    
    # Build adjacency matrix
    adj = np.zeros((num_nodes, num_nodes), dtype=np.float32)
    link_qualities = np.zeros((num_nodes, num_nodes), dtype=np.float32)
    
    for _, row in attack_df.iterrows():
        src_idx = node_to_idx[row['Node_ID']]
        
        # Create edges based on neighbor count and link stability
        # Higher link stability = stronger connection
        num_neighbors = min(int(row['Neighbor_Node_Count']), num_nodes - 1)
        
        # Connect to nearest neighbors (simulated by index proximity)
        for offset in range(1, num_neighbors + 1):
            for direction in [-1, 1]:
                neighbor_idx = (src_idx + offset * direction) % num_nodes
                if neighbor_idx != src_idx:
                    # Link quality based on stability index
                    quality = row['Link_Stability_Index'] * row['Routing_Path_Stability']
                    adj[src_idx, neighbor_idx] = 1.0
                    adj[neighbor_idx, src_idx] = 1.0
                    link_qualities[src_idx, neighbor_idx] = quality
                    link_qualities[neighbor_idx, src_idx] = quality
    
    # Ensure connectivity
    # Simple approach: connect to at least one neighbor
    for i in range(num_nodes):
        if adj[i].sum() == 0:
            # Connect to nearest node
            neighbor = (i + 1) % num_nodes
            adj[i, neighbor] = 1.0
            adj[neighbor, i] = 1.0
            link_qualities[i, neighbor] = 0.5
            link_qualities[neighbor, i] = 0.5
    
    # Generate routing decisions
    # Use link quality + distance to determine optimal next hop
    for src_node in nodes:
        src_idx = node_to_idx[src_node]
        
        # Find neighbors
        neighbors = np.where(adj[src_idx] > 0)[0]
        if len(neighbors) == 0:
            continue
        
        # For each possible destination
        for dst_node in nodes:
            if dst_node == src_node:
                continue
            
            dst_idx = node_to_idx[dst_node]
            
            # Find best next hop based on link quality and distance
            best_next_hop = None
            best_score = -1
            
            for neighbor_idx in neighbors:
                # Score based on link quality and progress toward destination
                quality = link_qualities[src_idx, neighbor_idx]
                
                # Distance to destination (Euclidean in index space)
                dist_to_dst = abs(neighbor_idx - dst_idx)
                max_dist = num_nodes / 2
                progress = 1.0 - (dist_to_dst / max_dist)
                
                # Combined score
                score = quality * 0.6 + progress * 0.4
                
                if score > best_score:
                    best_score = score
                    best_next_hop = neighbor_idx
            
            if best_next_hop is not None:
                # Encode topology
                tokens = encode_topology(adj, link_qualities, src_idx, dst_idx, num_nodes)
                
                examples.append({
                    "inputs": [int(x) for x in tokens],
                    "labels": [int(best_next_hop)],
                    "puzzle_identifiers": [0],
                    "metadata": {
                        "num_nodes": int(num_nodes),
                        "num_edges": int(adj.sum() / 2),
                        "path_length": 2,  # Simplified
                        "environment": "real_wsn",
                        "is_real": True,
                        "attack_type": attack_type,
                        "source": "besra_wsn_dataset",
                    }
                })

# Shuffle and save
print(f"\nShuffling {len(examples):,} examples...")
import random
random.shuffle(examples)

print(f"\nSaving to {OUTPUT_PATH}...")
dataset = {
    "examples": examples,
    "metadata": {
        "num_samples": len(examples),
        "source": "besra_wsn_dataset",
        "is_real": True,
        "attack_types": list(attack_types),
        "generated_at": "2026-04-03",
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
print(f"Attack types: {len(attack_types)}")
print(f"\nNext step: Combine with synthetic data")
