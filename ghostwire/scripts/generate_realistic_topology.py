#!/usr/bin/env python3
"""
GhostWire Realistic Topology Generator
Based on EvoTopo algorithm (Umlauft et al., 2026)
Generates realistic wireless mesh network topologies
"""

import numpy as np
import random
from pathlib import Path
import json
from tqdm import tqdm

# Configuration
OUTPUT_DIR = Path(__file__).resolve().parent.parent / "data"
OUTPUT_FILE = OUTPUT_DIR / "ghostwire_mesh_realistic_100k.json"
NUM_SAMPLES = 10000  # Reduced for faster generation

print("=" * 70)
print("GHOSTWIRE REALISTIC TOPOLOGY GENERATOR")
print("Based on EvoTopo Algorithm (Umlauft et al., 2026)")
print("=" * 70)

def generate_realistic_topology(num_nodes, environment='urban'):
    """
    Generate realistic wireless mesh topology using EvoTopo-inspired algorithm
    
    Parameters:
    - num_nodes: Number of nodes in the network
    - environment: 'urban', 'suburban', 'rural', or 'indoor'
    
    Returns:
    - adjacency matrix with realistic connectivity
    - node positions (x, y)
    - link qualities
    """
    
    # Environment-specific parameters
    env_params = {
        'urban': {
            'area_size': 1000,  # 1km x 1km
            'tx_range': 150,    # 150m transmission range
            'path_loss': 3.5,   # Higher path loss in urban
            'obstacle_density': 0.3,
            'node_distribution': 'clustered'  # Nodes cluster around buildings
        },
        'suburban': {
            'area_size': 2000,
            'tx_range': 250,
            'path_loss': 3.0,
            'obstacle_density': 0.15,
            'node_distribution': 'semi_clustered'
        },
        'rural': {
            'area_size': 5000,
            'tx_range': 500,
            'path_loss': 2.5,
            'obstacle_density': 0.05,
            'node_distribution': 'random'
        },
        'indoor': {
            'area_size': 100,
            'tx_range': 30,
            'path_loss': 4.0,
            'obstacle_density': 0.5,
            'node_distribution': 'grid_with_noise'
        }
    }
    
    params = env_params[environment]
    
    # Generate node positions
    if params['node_distribution'] == 'clustered':
        # Urban: nodes cluster around "buildings"
        num_clusters = max(3, num_nodes // 20)
        cluster_centers = np.random.uniform(0, params['area_size'], (num_clusters, 2))
        
        positions = []
        nodes_per_cluster = num_nodes // num_clusters
        for i in range(num_clusters):
            cluster_nodes = np.random.normal(
                cluster_centers[i], 
                params['area_size'] / (num_clusters * 2),
                (nodes_per_cluster, 2)
            )
            positions.append(cluster_nodes)
        
        # Add remaining nodes
        remaining = num_nodes - len(positions) * nodes_per_cluster
        if remaining > 0:
            extra = np.random.uniform(0, params['area_size'], (remaining, 2))
            positions.append(extra)
        
        positions = np.vstack(positions)
        
    elif params['node_distribution'] == 'grid_with_noise':
        # Indoor: grid layout with noise
        grid_size = int(np.ceil(np.sqrt(num_nodes)))
        x = np.linspace(0, params['area_size'], grid_size)
        y = np.linspace(0, params['area_size'], grid_size)
        xx, yy = np.meshgrid(x, y)
        positions = np.column_stack([xx.ravel(), yy.ravel()])[:num_nodes]
        # Add noise
        positions += np.random.normal(0, params['area_size'] * 0.05, positions.shape)
        
    else:
        # Random or semi-clustered
        positions = np.random.uniform(0, params['area_size'], (num_nodes, 2))
    
    # Clip to area
    positions = np.clip(positions, 0, params['area_size'])
    
    # Calculate distances
    dist_matrix = np.zeros((num_nodes, num_nodes))
    for i in range(num_nodes):
        for j in range(i+1, num_nodes):
            dist = np.sqrt(np.sum((positions[i] - positions[j])**2))
            dist_matrix[i, j] = dist
            dist_matrix[j, i] = dist
    
    # Calculate link qualities based on distance and path loss
    adj_matrix = np.zeros((num_nodes, num_nodes), dtype=np.float32)
    link_qualities = np.zeros((num_nodes, num_nodes), dtype=np.float32)
    
    for i in range(num_nodes):
        for j in range(i+1, num_nodes):
            dist = dist_matrix[i, j]
            
            # Check if within transmission range
            if dist <= params['tx_range']:
                # Calculate link quality using log-distance path loss model
                # PL(d) = PL(d0) + 10*n*log10(d/d0)
                d0 = 1.0  # Reference distance
                PL_d0 = 40.0  # Path loss at reference distance (dB)
                
                path_loss = PL_d0 + 10 * params['path_loss'] * np.log10(max(dist, d0) / d0)
                
                # Add shadowing (log-normal)
                shadowing = np.random.normal(0, 8)  # 8 dB standard deviation
                
                # Add obstacle attenuation
                if random.random() < params['obstacle_density']:
                    shadowing += np.random.uniform(5, 20)  # 5-20 dB additional loss
                
                received_power = 0 - path_loss + shadowing  # Assuming 0 dBm transmit power
                
                # Convert to quality (0-1)
                # Typical receiver sensitivity: -90 dBm
                sensitivity = -90
                if received_power > sensitivity:
                    quality = min(1.0, (received_power - sensitivity) / 30.0)  # Normalize to 0-1
                else:
                    quality = 0.0
                
                # Only connect if quality is reasonable
                if quality > 0.1:
                    adj_matrix[i, j] = 1.0
                    adj_matrix[j, i] = 1.0
                    link_qualities[i, j] = quality
                    link_qualities[j, i] = quality
    
    # Ensure connectivity (add edges if disconnected)
    # Use BFS to find connected components
    visited = set()
    components = []
    
    for start_node in range(num_nodes):
        if start_node in visited:
            continue
        
        # BFS
        component = []
        queue = [start_node]
        while queue:
            node = queue.pop(0)
            if node in visited:
                continue
            visited.add(node)
            component.append(node)
            
            for neighbor in range(num_nodes):
                if adj_matrix[node, neighbor] > 0 and neighbor not in visited:
                    queue.append(neighbor)
        
        components.append(component)
    
    # Connect components
    if len(components) > 1:
        for i in range(1, len(components)):
            # Find closest nodes between components
            min_dist = float('inf')
            best_pair = None
            
            for node1 in components[0]:
                for node2 in components[i]:
                    dist = dist_matrix[node1, node2]
                    if dist < min_dist:
                        min_dist = dist
                        best_pair = (node1, node2)
            
            if best_pair:
                # Add edge between components
                adj_matrix[best_pair[0], best_pair[1]] = 1.0
                adj_matrix[best_pair[1], best_pair[0]] = 1.0
                
                # Calculate quality for this edge
                quality = max(0.1, 1.0 - (min_dist / params['tx_range']))
                link_qualities[best_pair[0], best_pair[1]] = quality
                link_qualities[best_pair[1], best_pair[0]] = quality
    
    return adj_matrix, positions, link_qualities

def encode_topology(adj_matrix, positions, link_qualities, src, dst, max_seq_len=128):
    """
    Encode topology with realistic features
    
    Includes:
    - Node count
    - Edge count
    - Source/destination
    - Edge list with link qualities
    - Node positions (normalized)
    """
    num_nodes = adj_matrix.shape[0]
    
    # Extract edges with qualities
    edges = []
    for i in range(num_nodes):
        for j in range(i+1, num_nodes):
            if adj_matrix[i, j] > 0:
                edges.append((i, j, link_qualities[i, j]))
    
    # Create token sequence
    # Format: [num_nodes, num_edges, src, dst, edge1_n1, edge1_n2, edge1_quality, ...]
    tokens = [num_nodes, len(edges), src, dst]
    
    for (i, j, quality) in edges[:60]:  # Limit edges to fit sequence
        # Quantize quality to 0-99 range
        quality_int = int(quality * 99)
        tokens.extend([i, j, quality_int])
    
    # Pad to fixed length
    while len(tokens) < max_seq_len:
        tokens.append(-1)
    
    return tokens[:max_seq_len]

def dijkstra(adj, src, dst):
    """Find shortest path using Dijkstra's algorithm"""
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
    
    if len(path) > 0 and path[-1] == src:
        return path[::-1]
    return []

def main():
    print(f"\nGenerating {NUM_SAMPLES:,} realistic topology samples...")
    print(f"  Environments: urban (40%), suburban (30%), rural (20%), indoor (10%)")
    print(f"  Node counts: 20-200 nodes")
    print(f"  Link qualities: Realistic path loss + shadowing")
    
    environments = ['urban'] * 40 + ['suburban'] * 30 + ['rural'] * 20 + ['indoor'] * 10
    examples = []
    
    for i in tqdm(range(NUM_SAMPLES)):
        # Random environment
        env = random.choice(environments)
        
        # Random node count (realistic range)
        num_nodes = random.randint(20, 200)
        
        # Generate topology
        adj_matrix, positions, link_qualities = generate_realistic_topology(num_nodes, env)
        
        # Random source-destination
        src = random.randint(0, num_nodes - 1)
        dst = random.randint(0, num_nodes - 1)
        while dst == src:
            dst = random.randint(0, num_nodes - 1)
        
        # Compute optimal path
        path = dijkstra(adj_matrix, src, dst)
        if len(path) < 2:
            continue
        
        next_hop = path[1]
        
        # Encode topology
        tokens = encode_topology(adj_matrix, positions, link_qualities, src, dst)
        
        examples.append({
            "inputs": [int(x) for x in tokens],
            "labels": [int(next_hop)],
            "puzzle_identifiers": [0],
            "metadata": {
                "num_nodes": int(num_nodes),
                "num_edges": int(adj_matrix.sum() / 2),
                "path_length": int(len(path)),
                "environment": env,
                "is_realistic": True,
                "avg_link_quality": float(link_qualities[link_qualities > 0].mean()) if (link_qualities > 0).any() else 0.0,
            }
        })
    
    # Shuffle
    print("\nShuffling dataset...")
    random.shuffle(examples)
    
    # Save
    print(f"\nSaving to {OUTPUT_FILE}...")
    dataset = {
        "examples": examples,
        "metadata": {
            "num_samples": len(examples),
            "generated_at": "2026-04-03",
            "algorithm": "EvoTopo-inspired",
            "environments": {
                "urban": sum(1 for e in examples if e['metadata']['environment'] == 'urban'),
                "suburban": sum(1 for e in examples if e['metadata']['environment'] == 'suburban'),
                "rural": sum(1 for e in examples if e['metadata']['environment'] == 'rural'),
                "indoor": sum(1 for e in examples if e['metadata']['environment'] == 'indoor'),
            },
            "features": [
                "Realistic node positioning",
                "Log-distance path loss model",
                "Log-normal shadowing",
                "Obstacle attenuation",
                "Connectivity guarantee",
                "Link quality metrics"
            ]
        }
    }
    
    with open(OUTPUT_FILE, 'w') as f:
        json.dump(dataset, f)
    
    file_size_mb = OUTPUT_FILE.stat().st_size / 1024 / 1024
    print(f"\n{'='*70}")
    print("REALISTIC DATASET GENERATION COMPLETE")
    print(f"{'='*70}")
    print(f"\nOutput: {OUTPUT_FILE}")
    print(f"Size: {file_size_mb:.1f} MB")
    print(f"Samples: {len(examples):,}")
    print(f"\nEnvironment distribution:")
    for env, count in dataset['metadata']['environments'].items():
        print(f"  {env}: {count:,} ({100*count/len(examples):.1f}%)")

if __name__ == "__main__":
    main()
