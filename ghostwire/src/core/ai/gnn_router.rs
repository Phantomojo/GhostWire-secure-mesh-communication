// GNN Router — Layer 3: Two-phase Graph Attention Network for Next-Hop Routing
//
// Architecture: 4-layer GAT encoder (offline, Python) + scoring MLP head (ONNX, Rust).
// Phase 1: Pre-computed node embeddings loaded from JSON (refreshed every 5 min).
// Phase 2: Scoring MLP (ONNX) runs locally per routing decision.
//
// Trained on Guifi.net mesh data (63 nodes, 7,931 temporal snapshots).
// Two-phase avoids exporting GATConv (which can't export to ONNX).

use anyhow::{Context, Result};
use blake2::Digest;
use libp2p::PeerId;
use ort::{session::Session, value::Tensor};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::RwLock;

use crate::core::ai::fallback;
use crate::core::ai::routing_classifier::RoutingFeatures;

// ── Constants matching the trained GNN model ─────────────────────────────────

pub const GNN_NODE_FEATURES: usize = 9;

pub const GNN_EDGE_FEATURES: usize = 2;

pub const MAX_CANDIDATES: usize = 5;

pub const GNN_CONFIDENCE_THRESHOLD: f32 = 0.5;

/// GNN was trained on 63-node Guifi.net mesh — guard against mismatch
pub const TRAINED_NODE_COUNT: usize = 63;

/// Embedding staleness threshold — after this, fall back to PRoPHET
pub const EMBEDDING_STALENESS_SECS: u64 = 1800;

/// How often to refresh embeddings by re-running the Python embed script
pub const EMBEDDING_REFRESH_SECS: u64 = 300;

/// Expected BLAKE2b-256 hash of the scoring model ONNX file.
/// Set this after model export: `sha256sum models/gnn_routing/gnn_routing.onnx`
/// or: `python3 -c "import hashlib; print(hashlib.new('blake2b', open('models/gnn_routing/gnn_routing.onnx','rb').read(), digest_size=32).hexdigest())"`
pub const EXPECTED_SCORING_HASH: &str = "4508c5f5bd0f5ffaaab769d8405373e2ca5d160187dbc4a500b8e9b2d28dfc20";

// ── Embedding freshness ──────────────────────────────────────────────────────

#[derive(Clone, Debug)]
pub enum EmbeddingState {
    Fresh,
    Stale { age_secs: u64 },
    Missing,
}

// ── Graph data structures ────────────────────────────────────────────────────

#[derive(Clone, Debug, Default)]
pub struct GnnNodeFeatures {
    pub out_degree_norm: f32,
    pub in_degree_norm: f32,
    pub avg_link_quality: f32,
    pub avg_hop_norm: f32,
    pub transit_load: f32,
    pub link_diversity: f32,
    pub node_degree_norm: f32,
    pub avg_edge_strength: f32,
    pub trust_score: f32,
}

impl GnnNodeFeatures {
    pub fn to_vector(&self) -> [f32; GNN_NODE_FEATURES] {
        [
            self.out_degree_norm,
            self.in_degree_norm,
            self.avg_link_quality,
            self.avg_hop_norm,
            self.transit_load,
            self.link_diversity,
            self.node_degree_norm,
            self.avg_edge_strength,
            self.trust_score,
        ]
    }
}

#[derive(Clone, Debug, Default)]
pub struct GnnEdgeFeatures {
    pub link_metric: f32,
    pub is_direct: f32,
}

impl GnnEdgeFeatures {
    pub fn to_vector(&self) -> [f32; GNN_EDGE_FEATURES] {
        [self.link_metric, self.is_direct]
    }
}

#[derive(Clone, Debug)]
pub struct MeshTopology {
    pub num_nodes: usize,
    pub node_features: Vec<GnnNodeFeatures>,
    pub adjacency: HashMap<usize, Vec<usize>>,
    pub edge_features: HashMap<(usize, usize), GnnEdgeFeatures>,
    pub routing: HashMap<usize, HashMap<usize, usize>>,
}

#[derive(Clone, Debug)]
pub struct GnnRoutingDecision {
    pub selected_peer: PeerId,
    pub confidence: f32,
    pub from_gnn: bool,
    pub scored_candidates: Vec<(PeerId, f32)>,
}

// ── Node ID mapping ──────────────────────────────────────────────────────────

#[derive(Clone, Debug, Default)]
pub struct NodeIdMapper {
    pub peer_to_index: HashMap<PeerId, usize>,
    pub index_to_peer: HashMap<usize, PeerId>,
    pub next_index: usize,
}

impl NodeIdMapper {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn register(&mut self, peer: PeerId) -> usize {
        if let Some(&idx) = self.peer_to_index.get(&peer) {
            return idx;
        }
        let idx = self.next_index;
        self.next_index += 1;
        self.peer_to_index.insert(peer.clone(), idx);
        self.index_to_peer.insert(idx, peer);
        idx
    }

    pub fn get_index(&self, peer: &PeerId) -> Option<usize> {
        self.peer_to_index.get(peer).copied()
    }

    pub fn get_peer(&self, index: usize) -> Option<PeerId> {
        self.index_to_peer.get(&index).cloned()
    }

    pub fn peers(&self) -> Vec<PeerId> {
        self.index_to_peer.values().cloned().collect()
    }

    pub fn len(&self) -> usize {
        self.peer_to_index.len()
    }

    pub fn clear(&mut self) {
        self.peer_to_index.clear();
        self.index_to_peer.clear();
        self.next_index = 0;
    }
}

// ── Two-phase GNN Inference Engine ───────────────────────────────────────────

/// Pre-computed node embeddings from the GAT encoder (Phase 1 — Python).
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NodeEmbeddingData {
    pub num_nodes: usize,
    pub embedding_dim: usize,
    pub timestamp: String,
    pub compute_time_ms: f64,
    pub embeddings: Vec<Vec<f32>>,
}

pub struct GnnRouter {
    session: Arc<std::sync::Mutex<Session>>,
    pub mapper: Arc<RwLock<NodeIdMapper>>,
    model_info: GnnModelInfo,
    node_embeddings: Arc<RwLock<Option<NodeEmbeddingData>>>,
    embedding_loaded_at: Arc<RwLock<Option<Instant>>>,
    embedding_path: std::path::PathBuf,
    embed_script: std::path::PathBuf,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GnnModelInfo {
    pub node_features: usize,
    pub edge_features: usize,
    pub num_layers: usize,
    pub hidden_dim: usize,
    pub training_accuracy: Option<f32>,
    pub scoring_input_dim: usize,
    pub max_candidates: usize,
    pub onnx_opset: Option<u32>,
}

impl Default for GnnModelInfo {
    fn default() -> Self {
        Self {
            node_features: GNN_NODE_FEATURES,
            edge_features: GNN_EDGE_FEATURES,
            num_layers: 4,
            hidden_dim: 128,
            training_accuracy: None,
            scoring_input_dim: 128 * 3,
            max_candidates: MAX_CANDIDATES,
            onnx_opset: Some(17),
        }
    }
}

fn verify_blake2b(model_path: &str, expected_hash: &str) -> Result<()> {
    if expected_hash.is_empty() {
        tracing::warn!(
            "No BLAKE2b hash configured for {}, skipping verification",
            model_path
        );
        return Ok(());
    }
    let data = std::fs::read(model_path)
        .with_context(|| format!("Failed to read model for hash verification: {}", model_path))?;
    let mut hasher = blake2::Blake2b::<blake2::digest::consts::U32>::new();
    hasher.update(&data);
    let hash = hex::encode(hasher.finalize());
    if hash != expected_hash {
        anyhow::bail!(
            "BLAKE2b hash mismatch for {}: expected {}, got {}",
            model_path,
            expected_hash,
            hash
        );
    }
    tracing::info!("BLAKE2b verification passed for {}", model_path);
    Ok(())
}

fn compute_blake2b_file(model_path: &str) -> Result<String> {
    let data = std::fs::read(model_path)?;
    let mut hasher = blake2::Blake2b::<blake2::digest::consts::U32>::new();
    hasher.update(&data);
    Ok(hex::encode(hasher.finalize()))
}

impl GnnRouter {
    pub fn new(model_path: &str, embedding_path: &str) -> Result<Self> {
        verify_blake2b(model_path, EXPECTED_SCORING_HASH)?;

        let mut builder =
            Session::builder().context("Failed to create ONNX session builder for scoring head")?;
        builder = builder
            .with_optimization_level(ort::session::builder::GraphOptimizationLevel::Level3)
            .unwrap_or_else(|e| e.recover());

        let session = builder
            .commit_from_file(model_path)
            .with_context(|| format!("Failed to load scoring head from {}", model_path))?;

        let model_info = GnnModelInfo::default();

        let onnx_hash = compute_blake2b_file(model_path).unwrap_or_default();
        tracing::info!(
            "GNN scoring head loaded: {} params, BLAKE2b={}",
            session.inputs().len(),
            &onnx_hash[..16]
        );

        let scripts_dir = std::path::PathBuf::from(model_path)
            .parent()
            .unwrap_or(std::path::Path::new("."))
            .join("..")
            .join("scripts");

        Ok(Self {
            session: Arc::new(std::sync::Mutex::new(session)),
            mapper: Arc::new(RwLock::new(NodeIdMapper::new())),
            model_info,
            node_embeddings: Arc::new(RwLock::new(None)),
            embedding_loaded_at: Arc::new(RwLock::new(None)),
            embedding_path: std::path::PathBuf::from(embedding_path),
            embed_script: scripts_dir.join("embed_topology.py"),
        })
    }

    pub fn with_model_info(mut self, info_path: &str) -> Result<Self> {
        if let Ok(content) = std::fs::read_to_string(info_path) {
            if let Ok(info) = serde_json::from_str::<GnnModelInfo>(&content) {
                self.model_info = info;
                tracing::info!("GNN model info loaded from {}", info_path);
            }
        }
        Ok(self)
    }

    /// Load pre-computed embeddings from the JSON file written by embed_topology.py
    pub async fn load_embeddings(&self) -> Result<()> {
        let path = &self.embedding_path;
        if !path.exists() {
            tracing::debug!("Embedding file not found: {}", path.display());
            return Ok(());
        }
        let content = tokio::fs::read_to_string(path)
            .await
            .with_context(|| format!("Failed to read embeddings from {}", path.display()))?;
        let data: NodeEmbeddingData =
            serde_json::from_str(&content).with_context(|| "Failed to parse embedding JSON")?;

        if data.num_nodes > TRAINED_NODE_COUNT {
            tracing::warn!(
                "Embedding has {} nodes but GNN trained on {} — truncating",
                data.num_nodes,
                TRAINED_NODE_COUNT
            );
        }

        *self.node_embeddings.write().await = Some(data);
        *self.embedding_loaded_at.write().await = Some(Instant::now());
        tracing::info!("Loaded embeddings from {}", path.display());
        Ok(())
    }

    pub async fn embedding_state(&self) -> EmbeddingState {
        let loaded_at = *self.embedding_loaded_at.read().await;
        match loaded_at {
            None => EmbeddingState::Missing,
            Some(ts) => {
                let age = ts.elapsed().as_secs();
                if age < EMBEDDING_STALENESS_SECS {
                    EmbeddingState::Fresh
                } else {
                    EmbeddingState::Stale { age_secs: age }
                }
            }
        }
    }

    /// Trigger embedding refresh by running the Python embed script
    pub async fn refresh_embeddings(&self) -> Result<()> {
        let output = tokio::process::Command::new("python3")
            .arg(&self.embed_script)
            .arg("--output")
            .arg(&self.embedding_path)
            .output()
            .await
            .context("Failed to spawn embed_topology.py")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("embed_topology.py failed: {}", stderr);
        }

        self.load_embeddings().await
    }

    pub async fn register_peer(&self, peer: PeerId) -> usize {
        let mut mapper = self.mapper.write().await;
        mapper.register(peer)
    }

    pub async fn select_next_hop(
        &self,
        topology: &MeshTopology,
        src: usize,
        dst: usize,
        candidates: &[usize],
        peer_features: &HashMap<usize, RoutingFeatures>,
    ) -> Result<GnnRoutingDecision> {
        if candidates.is_empty() {
            anyhow::bail!("No candidates for GNN routing");
        }

        if topology.num_nodes > TRAINED_NODE_COUNT {
            tracing::warn!(
                "Node count {} exceeds trained count {} — GNN predictions may be unreliable",
                topology.num_nodes,
                TRAINED_NODE_COUNT
            );
        }

        match self.embedding_state().await {
            EmbeddingState::Fresh => {}
            EmbeddingState::Stale { age_secs } => {
                tracing::debug!("Embeddings stale ({}s old), attempting refresh", age_secs);
                if let Err(e) = self.refresh_embeddings().await {
                    tracing::warn!("Embedding refresh failed: {}", e);
                }
            }
            EmbeddingState::Missing => {
                tracing::debug!("No embeddings loaded, attempting initial load");
                if let Err(e) = self.load_embeddings().await {
                    tracing::warn!("Initial embedding load failed: {}", e);
                }
            }
        }

        match self
            .run_scoring_inference(topology, src, dst, candidates)
            .await
        {
            Ok((scores, candidate_peers)) => {
                let mut scored: Vec<(PeerId, f32)> = candidate_peers
                    .iter()
                    .zip(scores.iter())
                    .map(|(peer, &score)| (peer.clone(), score))
                    .collect();

                scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

                let best_confidence = scored.first().map(|(_, s)| *s).unwrap_or(0.0);

                if best_confidence >= GNN_CONFIDENCE_THRESHOLD {
                    let (selected_peer, confidence) = scored.first().cloned().unwrap();
                    return Ok(GnnRoutingDecision {
                        selected_peer,
                        confidence,
                        from_gnn: true,
                        scored_candidates: scored,
                    });
                }

                tracing::debug!(
                    "GNN confidence {:.3} below threshold {:.3}, falling back to PRoPHET",
                    best_confidence,
                    GNN_CONFIDENCE_THRESHOLD
                );
            }
            Err(e) => {
                tracing::debug!("GNN inference failed: {}, falling back to PRoPHET", e);
            }
        }

        let mut candidates_with_features = Vec::new();
        for &cand_idx in candidates {
            let features =
                peer_features
                    .get(&cand_idx)
                    .cloned()
                    .unwrap_or_else(|| RoutingFeatures {
                        rssi: -70,
                        latency_ms: 100,
                        packet_loss: 0.1,
                        trust_score: 0.5,
                        hop_count: 1,
                        battery_level: 1.0,
                        time_of_day: 12,
                    });
            let mapper = self.mapper.read().await;
            if let Some(peer) = mapper.get_peer(cand_idx) {
                candidates_with_features.push((peer, features));
            }
        }

        let mut scored = Vec::new();
        for (peer, features) in &candidates_with_features {
            let score = fallback::calculate_delivery_predictivity(
                features.rssi,
                features.latency_ms,
                features.packet_loss,
                features.trust_score,
            ) as f32;
            scored.push((peer.clone(), score));
        }

        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let (selected_peer, confidence) = scored.first().cloned().unwrap_or_else(|| {
            (
                candidates_with_features
                    .first()
                    .map(|(p, _)| p.clone())
                    .unwrap_or_else(PeerId::random),
                0.0,
            )
        });

        Ok(GnnRoutingDecision {
            selected_peer,
            confidence,
            from_gnn: false,
            scored_candidates: scored,
        })
    }

    /// Phase 2 inference: scoring MLP (ONNX) on pre-computed embeddings.
    ///
    /// Constructs [src_emb || dst_emb || cand_emb] vectors from the loaded
    /// embedding JSON and feeds them to the scoring head ONNX model.
    async fn run_scoring_inference(
        &self,
        _topology: &MeshTopology,
        src: usize,
        dst: usize,
        candidates: &[usize],
    ) -> Result<(Vec<f32>, Vec<PeerId>)> {
        let embeddings_guard = self.node_embeddings.read().await;
        let emb_data = embeddings_guard.as_ref().context("No embeddings loaded")?;

        let n_emb = emb_data.num_nodes.min(TRAINED_NODE_COUNT);
        if src >= n_emb || dst >= n_emb {
            anyhow::bail!(
                "src={} or dst={} out of embedding range (0..{})",
                src,
                dst,
                n_emb
            );
        }

        let mapper = self.mapper.read().await;
        let valid_candidates: Vec<(usize, PeerId)> = candidates
            .iter()
            .filter_map(|&idx| {
                if idx < n_emb {
                    mapper.get_peer(idx).map(|p| (idx, p))
                } else {
                    None
                }
            })
            .collect();

        if valid_candidates.is_empty() {
            anyhow::bail!("No valid candidates within embedding range");
        }

        let hidden_dim = self.model_info.hidden_dim;
        let src_emb = &emb_data.embeddings[src];
        let dst_emb = &emb_data.embeddings[dst];

        let mut input_data: Vec<f32> = Vec::with_capacity(valid_candidates.len() * hidden_dim * 3);
        for &(cand_idx, _) in &valid_candidates {
            let cand_emb = &emb_data.embeddings[cand_idx];
            input_data.extend_from_slice(src_emb);
            input_data.extend_from_slice(dst_emb);
            input_data.extend_from_slice(cand_emb);
        }

        let num_cand = valid_candidates.len() as i64;
        let input_dim = (hidden_dim * 3) as i64;
        let input_shape = vec![num_cand, input_dim];
        let tensor = Tensor::from_array((input_shape, input_data))
            .context("Failed to create scoring input tensor")?;

        let mut session = self
            .session
            .lock()
            .map_err(|e| anyhow::anyhow!("Scoring session mutex poisoned: {}", e))?;

        let input_name = session
            .inputs()
            .first()
            .context("Scoring model has no inputs")?
            .name()
            .to_string();
        let output_name = session
            .outputs()
            .first()
            .context("Scoring model has no outputs")?
            .name()
            .to_string();

        let outputs = session
            .run(ort::inputs![input_name => tensor])
            .context("Scoring inference failed")?;

        let output_value = outputs
            .get(output_name.as_str())
            .context("Scoring output not found")?;

        let (_shape, output_data) = output_value
            .try_extract_tensor::<f32>()
            .context("Failed to extract scoring output tensor")?;

        let scores: Vec<f32> = output_data.to_vec();
        let candidate_peers: Vec<PeerId> = valid_candidates.into_iter().map(|(_, p)| p).collect();

        Ok((scores, candidate_peers))
    }

    pub async fn node_count(&self) -> usize {
        self.mapper.read().await.len()
    }
}

#[allow(dead_code)] // used in tests
fn apply_softmax(scores: &[f32]) -> Vec<f32> {
    if scores.is_empty() {
        return vec![];
    }
    let max_score = scores.iter().fold(f32::NEG_INFINITY, |a, &b| a.max(b));
    let exps: Vec<f32> = scores.iter().map(|&s| (s - max_score).exp()).collect();
    let sum: f32 = exps.iter().sum();
    if sum < 1e-8 {
        let uniform = 1.0 / scores.len() as f32;
        return vec![uniform; scores.len()];
    }
    exps.iter().map(|&e| e / sum).collect()
}

pub fn build_topology_from_mesh(
    peers: &HashMap<PeerId, RoutingFeatures>,
    routing_table: &HashMap<PeerId, HashMap<PeerId, PeerId>>,
    link_metrics: &HashMap<(PeerId, PeerId), f32>,
) -> MeshTopology {
    let num_nodes = peers.len();
    let peer_list: Vec<PeerId> = peers.keys().cloned().collect();
    let peer_to_idx: HashMap<PeerId, usize> = peer_list
        .iter()
        .enumerate()
        .map(|(i, p)| (p.clone(), i))
        .collect();

    let mut node_features = vec![GnnNodeFeatures::default(); num_nodes];
    let mut adjacency: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut edge_features: HashMap<(usize, usize), GnnEdgeFeatures> = HashMap::new();

    for (peer, features) in peers {
        let idx = peer_to_idx[peer];
        let out_degree = routing_table.get(peer).map(|rt| rt.len()).unwrap_or(0);
        let in_degree = routing_table
            .values()
            .filter(|rt| rt.values().any(|next_hop| next_hop == peer))
            .count();

        let num_nodes_f = num_nodes as f32;
        node_features[idx] = GnnNodeFeatures {
            out_degree_norm: out_degree as f32 / num_nodes_f,
            in_degree_norm: in_degree as f32 / num_nodes_f,
            avg_link_quality: (1.0 - features.packet_loss).clamp(0.0, 1.0),
            avg_hop_norm: features.hop_count as f32 / num_nodes_f,
            transit_load: 0.0,
            link_diversity: out_degree as f32 / num_nodes_f,
            node_degree_norm: (out_degree + in_degree) as f32 / (2.0 * num_nodes_f),
            avg_edge_strength: link_metrics
                .iter()
                .filter(|((src, _), _)| src == peer)
                .map(|(_, m)| *m)
                .sum::<f32>()
                / (out_degree.max(1) as f32),
            trust_score: features.trust_score,
        };

        if let Some(rt) = routing_table.get(peer) {
            let mut neighbors = Vec::new();
            for (dst, next_hop) in rt {
                if let Some(&nh_idx) = peer_to_idx.get(next_hop) {
                    if !neighbors.contains(&nh_idx) {
                        neighbors.push(nh_idx);
                    }
                    if let Some(&dst_idx) = peer_to_idx.get(dst) {
                        let metric = link_metrics
                            .get(&(*peer, *next_hop))
                            .copied()
                            .unwrap_or(0.5);
                        let is_direct = if next_hop == dst { 1.0 } else { 0.0 };
                        edge_features.insert(
                            (idx, dst_idx),
                            GnnEdgeFeatures {
                                link_metric: metric,
                                is_direct,
                            },
                        );
                    }
                }
            }
            adjacency.insert(idx, neighbors);
        }
    }

    let mut routing: HashMap<usize, HashMap<usize, usize>> = HashMap::new();
    for (src, rt) in routing_table {
        if let Some(&src_idx) = peer_to_idx.get(src) {
            let mut src_rt = HashMap::new();
            for (dst, next_hop) in rt {
                if let (Some(&dst_idx), Some(&nh_idx)) =
                    (peer_to_idx.get(dst), peer_to_idx.get(next_hop))
                {
                    src_rt.insert(dst_idx, nh_idx);
                }
            }
            routing.insert(src_idx, src_rt);
        }
    }

    MeshTopology {
        num_nodes,
        node_features,
        adjacency,
        edge_features,
        routing,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_apply_softmax() {
        let scores = vec![1.0, 2.0, 3.0];
        let probs = apply_softmax(&scores);
        assert_eq!(probs.len(), 3);
        let sum: f32 = probs.iter().sum();
        assert!(
            (sum - 1.0).abs() < 1e-5,
            "Softmax should sum to 1.0, got {}",
            sum
        );
        assert!(probs[2] > probs[1] && probs[1] > probs[0]);
    }

    #[test]
    fn test_apply_softmax_empty() {
        let probs = apply_softmax(&[]);
        assert!(probs.is_empty());
    }

    #[test]
    fn test_node_features_to_vector_9d() {
        let features = GnnNodeFeatures {
            out_degree_norm: 0.5,
            in_degree_norm: 0.3,
            avg_link_quality: 0.8,
            avg_hop_norm: 0.1,
            transit_load: 0.2,
            link_diversity: 0.4,
            node_degree_norm: 0.6,
            avg_edge_strength: 0.7,
            trust_score: 0.9,
        };
        let vec = features.to_vector();
        assert_eq!(vec.len(), GNN_NODE_FEATURES);
        assert!((vec[0] - 0.5).abs() < 1e-5);
        assert!((vec[8] - 0.9).abs() < 1e-5);
    }

    #[test]
    fn test_edge_features_to_vector() {
        let features = GnnEdgeFeatures {
            link_metric: 0.75,
            is_direct: 1.0,
        };
        let vec = features.to_vector();
        assert_eq!(vec.len(), GNN_EDGE_FEATURES);
        assert!((vec[0] - 0.75).abs() < 1e-5);
        assert!((vec[1] - 1.0).abs() < 1e-5);
    }

    #[test]
    fn test_node_id_mapper() {
        let mut mapper = NodeIdMapper::new();
        let peer1 = PeerId::random();
        let peer2 = PeerId::random();
        let idx1 = mapper.register(peer1.clone());
        let idx2 = mapper.register(peer2.clone());
        assert_ne!(idx1, idx2);
        assert_eq!(mapper.get_index(&peer1), Some(idx1));
        assert_eq!(mapper.get_peer(idx1), Some(peer1));
        assert_eq!(mapper.get_peer(idx2), Some(peer2));
        assert_eq!(mapper.len(), 2);
    }

    #[test]
    fn test_node_id_mapper_duplicate() {
        let mut mapper = NodeIdMapper::new();
        let peer = PeerId::random();
        let idx1 = mapper.register(peer.clone());
        let idx2 = mapper.register(peer.clone());
        assert_eq!(idx1, idx2);
        assert_eq!(mapper.len(), 1);
    }

    #[test]
    fn test_build_topology_from_mesh() {
        let peer1 = PeerId::random();
        let peer2 = PeerId::random();
        let peer3 = PeerId::random();

        let mut peers = HashMap::new();
        peers.insert(
            peer1.clone(),
            RoutingFeatures {
                rssi: -50,
                latency_ms: 20,
                packet_loss: 0.02,
                trust_score: 0.9,
                hop_count: 1,
                battery_level: 1.0,
                time_of_day: 12,
            },
        );
        peers.insert(
            peer2.clone(),
            RoutingFeatures {
                rssi: -70,
                latency_ms: 50,
                packet_loss: 0.05,
                trust_score: 0.7,
                hop_count: 2,
                battery_level: 0.8,
                time_of_day: 12,
            },
        );
        peers.insert(
            peer3.clone(),
            RoutingFeatures {
                rssi: -80,
                latency_ms: 100,
                packet_loss: 0.1,
                trust_score: 0.5,
                hop_count: 3,
                battery_level: 0.6,
                time_of_day: 12,
            },
        );

        let mut routing_table = HashMap::new();
        let mut rt1 = HashMap::new();
        rt1.insert(peer2.clone(), peer2.clone());
        rt1.insert(peer3.clone(), peer2.clone());
        routing_table.insert(peer1.clone(), rt1);

        let mut link_metrics = HashMap::new();
        link_metrics.insert((peer1.clone(), peer2.clone()), 0.8);
        link_metrics.insert((peer1.clone(), peer3.clone()), 0.5);

        let topology = build_topology_from_mesh(&peers, &routing_table, &link_metrics);

        assert_eq!(topology.num_nodes, 3);
        assert_eq!(topology.node_features.len(), 3);
        let peer1_idx = 0;
        assert!(topology.adjacency.contains_key(&peer1_idx));
    }
}
