#!/usr/bin/env python3
"""
GhostWire Mesh Network Dataset Generator
Generates synthetic mesh network topologies and optimal routing decisions
For training HRM (Hierarchical Reasoning Model)
"""

import numpy as np
import json
from pathlib import Path
from tqdm import tqdm

# Configuration
OUTPUT_DIR = Path(__file__).parent.parent / "data" / "ghostwire-mesh-1k"
OUTPUT_DIR.mkdir(parents=True, exist_ok=True)

print("=" * 70)
print("GHOSTWIRE MESH NETWORK DATASET GENERATOR")
print("=" * 70)

def generate_mesh_topology(num_nodes=100, connectivity=0.15):
    """
    Generate random mesh network topology with guaranteed connectivity
    """
    while True:
        # Generate random adjacency matrix
        adj = np.random.random((num_nodes, num_nodes)) < connectivity
        adj = np.triu(adj, 1)
        adj = adj + adj.T
        
        # Ensure connected: create a spanning path first
        for i in range(num_nodes - 1):
            adj[i, i+1] = 1
            adj[i+1, i] = 1
        
        # Check if graph is connected using BFS
        visited = set()
        queue = [0]
        while queue:
            node = queue.pop(0)
            if node in visited:
                continue
            visited.add(node)
            for neighbor in range(num_nodes):
                if adj[node, neighbor] and neighbor not in visited:
                    queue.append(neighbor)
        
        if len(visited) == num_nodes:
            return adj  # Graph is connected

def dijkstra_shortest_path(adj, src, dst):
    """
    Find shortest path using Dijkstra's algorithm
    
    Args:
        adj: Adjacency matrix
        src: Source node index
        dst: Destination node index
    
    Returns:
        List of node indices in path
    """
    n = adj.shape[0]
    dist = np.full(n, np.inf)
    dist[src] = 0
    prev = np.full(n, -1, dtype=int)
    visited = np.zeros(n, dtype=bool)
    
    for _ in range(n):
        # Find unvisited node with minimum distance
        min_dist = np.inf
        u = -1
        for i in range(n):
            if not visited[i] and dist[i] < min_dist:
                min_dist = dist[i]
                u = i
        
        if u == -1:
            break
        
        visited[u] = True
        
        if u == dst:
            break
        
        # Update distances to neighbors
        for v in range(n):
            if adj[u, v] and not visited[v]:
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
    
    # Return path if valid
    if len(path) > 0 and path[-1] == src:
        return path[::-1]
    return []

def encode_topology_for_hrm(adj, src, dst, max_edges=50):
    """
    Encode mesh topology as token sequence for HRM model
    
    Args:
        adj: Adjacency matrix
        src: Source node
        dst: Destination node
        max_edges: Maximum number of edges to encode
    
    Returns:
        Token sequence (list of int)
    """
    n = adj.shape[0]
    
    # Extract edges
    edges = []
    for i in range(n):
        for j in range(i+1, n):
            if adj[i, j]:
                edges.append((i, j))
    
    # Create token sequence
    # Format: [num_nodes, num_edges, src, dst, edge1_node1, edge1_node2, ...]
    tokens = [n, len(edges), src, dst]
    
    for (i, j) in edges[:max_edges]:
        tokens.extend([i, j])
    
    # Pad to fixed length (64 tokens)
    while len(tokens) < 64:
        tokens.append(-1)  # Padding token
    
    return tokens[:64]

def generate_training_sample():
    """
    Generate single training sample for HRM
    
    Returns:
        Dictionary with inputs, labels, and metadata
    """
    # Generate topology
    adj = generate_mesh_topology(num_nodes=100, connectivity=0.09)
    
    # Random source and destination
    src = np.random.randint(0, 100)
    dst = np.random.randint(0, 100)
    while dst == src:
        dst = np.random.randint(0, 100)
    
    # Find optimal path (label)
    path = dijkstra_shortest_path(adj, src, dst)
    
    if len(path) < 2:
        return None  # No path found
    
    next_hop = path[1]  # First hop after source
    
    # Encode for HRM
    input_tokens = encode_topology_for_hrm(adj, src, dst)
    
    return {
        "inputs": [int(x) for x in input_tokens],
        "labels": [int(next_hop)],
        "puzzle_identifiers": [0],  # Routing task ID
        "metadata": {
            "num_nodes": 100,
            "num_edges": int(adj.sum() / 2),
            "path_length": len(path),
            "src": int(src),
            "dst": int(dst),
        }
    }

def main():
    """Generate complete dataset"""
    
    print("\nGenerating 1,000 synthetic mesh routing scenarios...")
    print("This may take 2-3 minutes...\n")
    
    examples = []
    failed = 0
    
    for _ in tqdm(range(1000)):
        sample = generate_training_sample()
        if sample is not None:
            examples.append(sample)
        else:
            failed += 1
    
    # Save dataset
    output_file = OUTPUT_DIR / "ghostwire_mesh_1k.json"
    with open(output_file, 'w') as f:
        json.dump(examples, f, indent=2)
    
    # Statistics
    path_lengths = [e['metadata']['path_length'] for e in examples]
    num_edges = [e['metadata']['num_edges'] for e in examples]
    
    print("\n" + "=" * 70)
    print("DATASET GENERATION COMPLETE")
    print("=" * 70)
    print(f"\nGenerated: {len(examples):,} samples")
    print(f"Failed: {failed}")
    print(f"\nSaved to: {output_file}")
    print(f"File size: {output_file.stat().st_size / 1024 / 1024:.2f} MB")
    
    print(f"\nPath Length Statistics:")
    print(f"  Min: {min(path_lengths)}")
    print(f"  Max: {max(path_lengths)}")
    print(f"  Avg: {np.mean(path_lengths):.2f}")
    print(f"  Std: {np.std(path_lengths):.2f}")
    
    print(f"\nNetwork Statistics:")
    print(f"  Avg edges: {np.mean(num_edges):.1f}")
    print(f"  Min edges: {min(num_edges)}")
    print(f"  Max edges: {max(num_edges)}")
    
    # Create HRM-compatible format
    print("\n" + "=" * 70)
    print("CONVERTING TO HRM FORMAT")
    print("=" * 70)
    
    hrm_dir = OUTPUT_DIR.parent / "hrm-mesh-1k"
    hrm_dir.mkdir(parents=True, exist_ok=True)
    
    # Convert to HRM training format
    hrm_examples = []
    for ex in examples:
        hrm_examples.append({
            "inputs": ex["inputs"],
            "labels": ex["labels"],
            "puzzle_identifiers": ex["puzzle_identifiers"],
        })
    
    hrm_file = hrm_dir / "train.json"
    with open(hrm_file, 'w') as f:
        json.dump(hrm_examples, f)
    
    print(f"\nHRM format saved to: {hrm_file}")
    print(f"File size: {hrm_file.stat().st_size / 1024 / 1024:.2f} MB")
    
    print("\n✅ Dataset ready for HRM training!")
    print("\nNext step: Run HRM training with:")
    print(f"  cd /path/to/HRM")
    print(f"  python pretrain.py data_path={hrm_dir.absolute()}")

if __name__ == "__main__":
    main()
