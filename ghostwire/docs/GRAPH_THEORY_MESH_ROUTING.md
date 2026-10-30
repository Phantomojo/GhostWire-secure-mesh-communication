# Graph Theory for GhostWire Mesh Routing

## Purpose

Apply graph-theoretic metrics to GhostWire's mesh routing decisions.
Betweenness centrality, PageRank, and clustering coefficients inform
which peers are critical relays and where to add redundancy.

## Key Metrics

### Betweenness Centrality
Measures how often a node lies on shortest paths between other nodes.
High betweenness = critical relay. If this node goes down, network
fragments.

```
C_B(v) = Σ_{s≠v≠t} σ_st(v) / σ_st
```
where σ_st = number of shortest paths from s to t, σ_st(v) = number
through v.

### PageRank
Importance of a node based on who links to it. In GhostWire, "links"
are active connections. High PageRank = well-connected peer.

```
PR(v) = (1-d)/N + d * Σ_{u→v} PR(u)/L(u)
```

### Clustering Coefficient
Measures how interconnected a node's neighbors are. High clustering =
resilient local neighborhood.

```
C(v) = 2 * E_v / (k_v * (k_v - 1))
```
where E_v = edges between neighbors, k_v = degree.

### Eccentricity & Diameter
- Eccentricity: max distance from v to any other node
- Diameter: max eccentricity (worst-case latency)
- Radius: min eccentricity (best-case coverage)

## Application to GhostWire

### 1. Critical Relay Detection
Nodes with high betweenness centrality are critical. GhostWire should:
- Monitor these nodes' health more frequently
- Add redundant paths around them
- Alert operator if they go offline

### 2. Peer Selection for Sphinx
Sphinx routing should avoid high-betweenness nodes (they're surveillance
targets). Prefer low-betweenness paths for anonymity.

### 3. Network Health Dashboard
Display graph metrics in the System panel:
- Network diameter (worst-case hops)
- Average clustering (resilience indicator)
- Top-3 critical relays (by betweenness)

### 4. Adaptive Transport Selection
Transport selector should consider graph position:
- High-betweenness nodes: prefer reliable transports (libp2p)
- Low-betweenness nodes: can use opportunistic transports (RNS, BLE)

## Implementation Plan

1. Add graph metrics to MeshHealth (betweenness, PageRank, clustering)
2. Compute on mesh topology changes (incremental update)
3. Display in System panel
4. Feed into transport selector weights
5. Use for Sphinx path selection (avoid high-betweenness nodes)

## References

- Freeman 1977: "A Set of Measures of Centrality Based on Betweenness"
- Page et al. 1999: "The PageRank Citation Ranking"
- Watts & Strogatz 1998: "Collective Dynamics of Small-World Networks"
- NetworkX: graph algorithms library (Python reference)
EOF
echo "created"