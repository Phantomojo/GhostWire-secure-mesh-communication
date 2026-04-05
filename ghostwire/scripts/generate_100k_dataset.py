#!/usr/bin/env python3
"""
GhostWire Mesh Dataset Generator - 100K Samples
Generates synthetic mesh network topologies with routing labels
"""

import json
import random
import numpy as np
from pathlib import Path
from tqdm import tqdm
import sys
sys.setrecursionlimit(10000)

# Configuration
OUTPUT_DIR = Path("/home/ph/GhostWire-secure-mesh-communication/ghostwire/data")
OUTPUT_FILE = OUTPUT_DIR / "ghostwire_mesh_100k.json"
NUM_SAMPLES = 100000
NUM_TOPOLOGIES = 10000
HARD_EXAMPLES_RATIO = 0.1  # 10% hard examples

def generate_connected_graph(num_nodes, connectivity):
    """Generate a connected random graph"""
    # Start with spanning tree (ensures connectivity)
    adj = np.zeros((num_nodes, num_nodes), dtype=np.float32)
    
    # Create spanning tree
    nodes = list(range(num_nodes))
    random.shuffle(nodes)
    for i in range(1, num_nodes):
        parent = random.randint(0, i-1)
        adj[nodes[i], nodes[parent]] = 1.0
        adj[nodes[parent], nodes[i]] = 1.0
    
    # Add random edges based on connectivity
    for i in range(num_nodes):
        for j in range(i+1, num_nodes):
            if adj[i, j] == 0 and random.random() < connectivity:
                adj[i, j] = 1.0
                adj[j, i] = 1.0
    
    return adj

def ensure_connected(adj):
    """Ensure graph is connected using Union-Find"""
    num_nodes = adj.shape[0]
    parent = list(range(num_nodes))
    
    def find(x):
        if parent[x] != x:
            parent[x] = find(parent[x])
        return parent[x]
    
    def union(x, y):
        px, py = find(x), find(y)
        if px != py:
            parent[px] = py
    
    # Union all connected nodes
    for i in range(num_nodes):
        for j in range(num_nodes):
            if adj[i, j] > 0:
                union(i, j)
    
    # Connect disconnected components
    components = {}
    for i in range(num_nodes):
        root = find(i)
        if root not in components:
            components[root] = []
        components[root].append(i)
    
    if len(components) > 1:
        # Connect components
        component_roots = list(components.keys())
        for i in range(1, len(component_roots)):
            # Connect component i to component 0
            node1 = random.choice(components[component_roots[0]])
            node2 = random.choice(components[component_roots[i]])
            adj[node1, node2] = 1.0
            adj[node2, node1] = 1.0
    
    return adj

def dijkstra(adj, src, dst):
    """Find shortest path using Dijkstra's algorithm"""
    num_nodes = adj.shape[0]
    dist = np.full(num_nodes, np.inf)
    dist[src] = 0
    prev = np.full(num_nodes, -1, dtype=int)
    visited = np.zeros(num_nodes, dtype=bool)
    
    for _ in range(num_nodes):
        # Find unvisited node with minimum distance
        min_dist = np.inf
        u = -1
        for i in range(num_nodes):
            if not visited[i] and dist[i] < min_dist:
                min_dist = dist[i]
                u = i
        
        if u == -1 or u == dst:
            break
        
        visited[u] = True
        
        # Update distances to neighbors
        for v in range(num_nodes):
            if adj[u, v] > 0 and not visited[v]:
                alt = dist[u] + 1
                if alt < dist[v]:
                    dist[v] = alt
                    prev[v] = u
    
    # Reconstruct path
    path = []
    current = dst
    while current != -1:
        path.append(current)
        current = prev[current]
    
    if len(path) > 0 and path[-1] == src:
        return path[::-1]
    return []

def encode_topology(adj, src, dst, max_seq_len=64):
    """Encode topology as token sequence for transformer"""
    num_nodes = adj.shape[0]
    
    # Extract edges
    edges = []
    for i in range(num_nodes):
        for j in range(i+1, num_nodes):
            if adj[i, j] > 0:
                edges.append((i, j))
    
    # Create token sequence
    # Format: [num_nodes, num_edges, src, dst, edge1_node1, edge1_node2, ...]
    tokens = [num_nodes, len(edges), src, dst]
    
    for (i, j) in edges[:30]:  # Limit edges to fit sequence
        tokens.extend([i, j])
    
    # Pad to fixed length
    while len(tokens) < max_seq_len:
        tokens.append(-1)  # Padding token
    
    return tokens[:max_seq_len]

def generate_hard_example():
    """Generate a challenging routing scenario"""
    # Larger network
    num_nodes = random.randint(100, 200)
    connectivity = random.uniform(0.05, 0.10)
    
    adj = generate_connected_graph(num_nodes, connectivity)
    adj = ensure_connected(adj)
    
    # Add node failures (5-15% offline)
    num_failed = int(num_nodes * random.uniform(0.05, 0.15))
    failed_nodes = random.sample(range(num_nodes), num_failed)
    for node in failed_nodes:
        adj[node, :] = 0
        adj[:, node] = 0
    
    # Ensure still connected after failures
    adj = ensure_connected(adj)
    
    # Random source-destination
    available_nodes = [i for i in range(num_nodes) if adj[i].sum() > 0]
    if len(available_nodes) < 2:
        return None
    
    src = random.choice(available_nodes)
    dst = random.choice(available_nodes)
    while dst == src:
        dst = random.choice(available_nodes)
    
    # Compute optimal path
    path = dijkstra(adj, src, dst)
    if len(path) < 2:
        return None
    
    next_hop = path[1]
    
    # Encode
    tokens = encode_topology(adj, src, dst)
    
    return {
        "inputs": [int(x) for x in tokens],
        "labels": [int(next_hop)],
        "puzzle_identifiers": [0],
        "metadata": {
            "num_nodes": int(num_nodes),
            "num_edges": int(adj.sum() / 2),
            "path_length": int(len(path)),
            "failed_nodes": int(num_failed),
            "is_hard": True,
        }
    }

def generate_normal_example():
    """Generate a standard routing scenario"""
    # Smaller network
    num_nodes = random.randint(50, 100)
    connectivity = random.uniform(0.08, 0.15)
    
    adj = generate_connected_graph(num_nodes, connectivity)
    adj = ensure_connected(adj)
    
    # Random source-destination pairs (10 per topology)
    examples = []
    for _ in range(10):
        src = random.randint(0, num_nodes-1)
        dst = random.randint(0, num_nodes-1)
        while dst == src:
            dst = random.randint(0, num_nodes-1)
        
        # Compute optimal path
        path = dijkstra(adj, src, dst)
        if len(path) < 2:
            continue
        
        next_hop = path[1]
        
        # Encode
        tokens = encode_topology(adj, src, dst)
        
        examples.append({
            "inputs": [int(x) for x in tokens],
            "labels": [int(next_hop)],
            "puzzle_identifiers": [0],
            "metadata": {
                "num_nodes": int(num_nodes),
                "num_edges": int(adj.sum() / 2),
                "path_length": int(len(path)),
                "is_hard": False,
            }
        })
    
    return examples

def main():
    print("=" * 70)
    print("GHOSTWIRE MESH DATASET GENERATOR - 100K SAMPLES")
    print("=" * 70)
    
    OUTPUT_DIR.mkdir(parents=True, exist_ok=True)
    
    examples = []
    num_hard = int(NUM_SAMPLES * HARD_EXAMPLES_RATIO)
    num_normal = NUM_SAMPLES - num_hard
    
    print(f"\nGenerating {NUM_SAMPLES:,} samples...")
    print(f"  - Normal examples: {num_normal:,}")
    print(f"  - Hard examples: {num_hard:,}")
    print(f"  - Topologies: ~{NUM_TOPOLOGIES:,}")
    
    # Generate normal examples
    print("\n[1/2] Generating normal examples...")
    topologies_generated = 0
    while len(examples) < num_normal:
        topologies_generated += 1
        
        new_examples = generate_normal_example()
        examples.extend(new_examples)
        
        if topologies_generated % 1000 == 0:
            print(f"  Generated {topologies_generated:,} topologies, {len(examples):,} samples")
    
    examples = examples[:num_normal]
    print(f"  ✓ Generated {len(examples):,} normal examples from {topologies_generated:,} topologies")
    
    # Generate hard examples
    print("\n[2/2] Generating hard examples...")
    hard_examples = []
    attempts = 0
    while len(hard_examples) < num_hard and attempts < num_hard * 3:
        attempts += 1
        example = generate_hard_example()
        if example is not None:
            hard_examples.append(example)
        
        if attempts % 1000 == 0:
            print(f"  Attempt {attempts:,}, collected {len(hard_examples):,} hard examples")
    
    examples.extend(hard_examples)
    print(f"  ✓ Generated {len(hard_examples):,} hard examples")
    
    # Shuffle
    print("\nShuffling dataset...")
    random.shuffle(examples)
    
    # Save
    print(f"\nSaving to {OUTPUT_FILE}...")
    dataset = {
        "examples": examples,
        "metadata": {
            "num_samples": len(examples),
            "num_normal": num_normal,
            "num_hard": len(hard_examples),
            "num_topologies": topologies_generated + attempts,
            "generated_at": "2026-04-03",
            "seq_len": 64,
            "vocab_size": 1000,
        }
    }
    
    with open(OUTPUT_FILE, 'w') as f:
        json.dump(dataset, f)
    
    file_size_mb = OUTPUT_FILE.stat().st_size / 1024 / 1024
    print(f"\n{'='*70}")
    print("DATASET GENERATION COMPLETE")
    print(f"{'='*70}")
    print(f"\nOutput: {OUTPUT_FILE}")
    print(f"Size: {file_size_mb:.1f} MB")
    print(f"Samples: {len(examples):,}")
    print(f"Hard examples: {len(hard_examples):,} ({100*len(hard_examples)/len(examples):.1f}%)")
    print(f"\nNext step: Train model on augmented dataset")

if __name__ == "__main__":
    main()
