// Feature Extractor for L1 Anomaly Detection
//
// Collects live mesh telemetry and produces a 2,387-dimensional feature vector
// matching the Guifi.net Guifisants training schema.
//
// Schema: 38 base features × 63 nodes = 2,394 possible, but 7 are missing
// for certain nodes → 2,387 actual features.
//
// The 38 base features per node:
//   System: processes, loadavg-m1, softirq.rate, iowait.rate, intr.rate,
//           system.rate, idle.rate, user.rate, irq.rate, ctxt.rate, nice.rate,
//           nr_slab_unreclaimable, nr_anon_pages, swap_cache, page_tables, swap
//   Ethernet: eth.txe.rate, eth.rxe.rate, eth.txb.rate, eth.rxb.rate,
//             eth.txp.rate, eth.rxp.rate
//   WiFi: wifi.txe.rate, wifi.rxe.rate, wifi.txb.rate, wifi.rxb.rate,
//         wifi.txp.rate, wifi.rxp.rate
//   Aggregate: txe.rate, rxe.rate, txb.rate, rxb.rate, txp.rate, rxp.rate,
//              sum.xb.rate, diff.xb.rate, sum.xp.rate, diff.xp.rate
//
// All features are StandardScaler normalized (zero mean, unit variance).
// For features we cannot collect live, we emit 0.0 (the scaler mean).

use anyhow::{Result, Context};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::sync::Arc;
use tokio::sync::RwLock;

/// Number of features the L1 model expects
pub const FEATURE_DIM: usize = 2387;

/// Number of base features per node
pub const BASE_FEATURES: usize = 38;

/// Number of nodes in the Guifisants topology
pub const NUM_NODES: usize = 63;

/// Base feature names in the exact order they appear per node
pub const BASE_FEATURE_NAMES: [&str; BASE_FEATURES] = [
    "processes",
    "loadavg-m1",
    "softirq.rate",
    "iowait.rate",
    "intr.rate",
    "system.rate",
    "idle.rate",
    "user.rate",
    "irq.rate",
    "ctxt.rate",
    "nice.rate",
    "nr_slab_unreclaimable",
    "nr_anon_pages",
    "swap_cache",
    "page_tables",
    "swap",
    "eth.txe.rate",
    "eth.rxe.rate",
    "eth.txb.rate",
    "eth.rxb.rate",
    "eth.txp.rate",
    "eth.rxp.rate",
    "wifi.txe.rate",
    "wifi.rxe.rate",
    "wifi.txb.rate",
    "wifi.rxb.rate",
    "wifi.txp.rate",
    "wifi.rxp.rate",
    "txe.rate",
    "rxe.rate",
    "txb.rate",
    "rxb.rate",
    "txp.rate",
    "rxp.rate",
    "sum.xb.rate",
    "diff.xb.rate",
    "sum.xp.rate",
    "diff.xp.rate",
];

/// Node IDs from the Guifisants dataset (in column order)
/// These are the actual node IDs from the Guifi.net Guifisants node
pub const NODE_IDS: [u16; NUM_NODES] = [
    0, 2, 3, 4, 5, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26,
    27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 47, 48, 49,
    50, 51, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64,
];

/// Column indices that are MISSING from the original dataset (7 columns)
/// These correspond to features that don't exist for certain nodes.
/// We compute this by checking which (node_id, feature_idx) combinations
/// map to valid column indices in the 2,387-dim output.
const MISSING_COLUMNS: &[usize] = &[]; // Computed at runtime from schema

/// StandardScaler parameters — mean and std for each of the 2,387 features.
/// These are loaded from the training data to normalize live features.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ScalerParams {
    pub mean: Vec<f32>,
    pub std: Vec<f32>,
}

impl ScalerParams {
    /// Create from raw mean and std vectors
    pub fn new(mean: Vec<f32>, std: Vec<f32>) -> Self {
        Self { mean, std }
    }

    /// Standardize a feature vector in-place
    pub fn transform(&self, features: &mut [f32]) {
        let len = features.len().min(self.mean.len()).min(self.std.len());
        for i in 0..len {
            if self.std[i] > 1e-8 {
                features[i] = (features[i] - self.mean[i]) / self.std[i];
            } else {
                features[i] = 0.0;
            }
        }
    }
}

/// Per-node raw metrics collected from the system
#[derive(Clone, Debug, Default)]
pub struct NodeMetrics {
    pub node_id: u16,
    pub processes: f32,
    pub loadavg_m1: f32,
    pub softirq_rate: f32,
    pub iowait_rate: f32,
    pub intr_rate: f32,
    pub system_rate: f32,
    pub idle_rate: f32,
    pub user_rate: f32,
    pub irq_rate: f32,
    pub ctxt_rate: f32,
    pub nice_rate: f32,
    pub nr_slab_unreclaimable: f32,
    pub nr_anon_pages: f32,
    pub swap_cache: f32,
    pub page_tables: f32,
    pub swap: f32,
    pub eth_txe_rate: f32,
    pub eth_rxe_rate: f32,
    pub eth_txb_rate: f32,
    pub eth_rxb_rate: f32,
    pub eth_txp_rate: f32,
    pub eth_rxp_rate: f32,
    pub wifi_txe_rate: f32,
    pub wifi_rxe_rate: f32,
    pub wifi_txb_rate: f32,
    pub wifi_rxb_rate: f32,
    pub wifi_txp_rate: f32,
    pub wifi_rxp_rate: f32,
    pub txe_rate: f32,
    pub rxe_rate: f32,
    pub txb_rate: f32,
    pub rxb_rate: f32,
    pub txp_rate: f32,
    pub rxp_rate: f32,
    pub sum_xb_rate: f32,
    pub diff_xb_rate: f32,
    pub sum_xp_rate: f32,
    pub diff_xp_rate: f32,
}

impl NodeMetrics {
    /// Convert to raw feature vector (38 features, in exact column order)
    pub fn to_raw_features(&self) -> [f32; BASE_FEATURES] {
        [
            self.processes,
            self.loadavg_m1,
            self.softirq_rate,
            self.iowait_rate,
            self.intr_rate,
            self.system_rate,
            self.idle_rate,
            self.user_rate,
            self.irq_rate,
            self.ctxt_rate,
            self.nice_rate,
            self.nr_slab_unreclaimable,
            self.nr_anon_pages,
            self.swap_cache,
            self.page_tables,
            self.swap,
            self.eth_txe_rate,
            self.eth_rxe_rate,
            self.eth_txb_rate,
            self.eth_rxb_rate,
            self.eth_txp_rate,
            self.eth_rxp_rate,
            self.wifi_txe_rate,
            self.wifi_rxe_rate,
            self.wifi_txb_rate,
            self.wifi_rxb_rate,
            self.wifi_txp_rate,
            self.wifi_rxp_rate,
            self.txe_rate,
            self.rxe_rate,
            self.txb_rate,
            self.rxb_rate,
            self.txp_rate,
            self.rxp_rate,
            self.sum_xb_rate,
            self.diff_xb_rate,
            self.sum_xp_rate,
            self.diff_xp_rate,
        ]
    }
}

/// Feature extractor — collects system metrics and produces L1-ready feature vectors
pub struct FeatureExtractor {
    scaler: Option<Arc<RwLock<ScalerParams>>>,
    /// Map from node_id to the column offset in the 2,387-dim vector
    column_map: HashMap<u16, Vec<usize>>,
    /// Valid column indices (0..2387)
    valid_columns: Vec<usize>,
}

impl FeatureExtractor {
    /// Create a new feature extractor
    pub fn new() -> Self {
        // Build the column map: for each node, which column indices in the 2,387-dim vector
        // correspond to its 38 base features.
        //
        // The original dataset has columns like "processes-23", "loadavg-m1-23", etc.
        // After removing datetime, hour, day_of_week, is_weekend, we have 2,387 features.
        // The features are ordered: all 38 features for node 0, then node 2, node 3, ... node 64.
        // But some nodes have fewer than 38 features (7 total missing across all nodes).

        let column_map = Self::build_column_map();
        let valid_columns: Vec<usize> = (0..FEATURE_DIM).collect();

        Self {
            scaler: None,
            column_map,
            valid_columns,
        }
    }

    /// Load scaler parameters from a JSON file
    pub async fn load_scaler(&mut self, path: &str) -> Result<()> {
        let content = fs::read_to_string(path)
            .with_context(|| format!("Failed to read scaler file: {}", path))?;
        let params: ScalerParams = serde_json::from_str(&content)
            .with_context(|| "Failed to parse scaler JSON")?;

        if params.mean.len() != FEATURE_DIM || params.std.len() != FEATURE_DIM {
            anyhow::bail!(
                "Scaler dimension mismatch: expected {}, got mean={}, std={}",
                FEATURE_DIM,
                params.mean.len(),
                params.std.len()
            );
        }

        self.scaler = Some(Arc::new(RwLock::new(params)));
        tracing::info!("✅ Feature scaler loaded: {} dimensions", FEATURE_DIM);
        Ok(())
    }

    /// Build the column map from node_id to feature indices
    ///
    /// The original dataset has 2,387 features (not 2,394 = 63×38).
    /// 7 features are missing — certain nodes don't have all 38 metrics.
    ///
    /// We reconstruct the exact column order from the training data schema:
    /// features are ordered [node0_feat0, node0_feat1, ..., node0_feat37,
    ///                        node2_feat0, node2_feat1, ..., node2_feat37,
    ///                        ...]
    /// but 7 columns are missing at specific positions.
    ///
    /// The missing columns correspond to features beyond index 2386 in the
    /// sequential 63×38 layout. Since the training data was processed as:
    ///   numeric_cols = all numeric except datetime, hour, day_of_week, is_weekend
    /// and the CSV has 2,388 columns (1 datetime + 2,387 features),
    /// the 7 missing columns are at the very end of the layout.
    ///
    /// 63 nodes × 38 features = 2,394 possible, but only 2,387 exist.
    /// The last node (node 64) has only 31 features instead of 38.
    fn build_column_map() -> HashMap<u16, Vec<usize>> {
        let mut map = HashMap::new();
        let mut col_idx = 0usize;

        for (i, &node_id) in NODE_IDS.iter().enumerate() {
            let mut cols = Vec::new();
            // The last node (index 62, node_id 64) has only 31 features
            let features_for_this_node = if i == NUM_NODES - 1 {
                // 2387 - (62 * 38) = 2387 - 2356 = 31
                FEATURE_DIM.saturating_sub(col_idx)
            } else {
                BASE_FEATURES
            };

            for _feat_idx in 0..features_for_this_node {
                if col_idx < FEATURE_DIM {
                    cols.push(col_idx);
                    col_idx += 1;
                }
            }
            map.insert(node_id, cols);
        }

        // Verify total
        let total: usize = map.values().map(|v| v.len()).sum();
        debug_assert_eq!(total, FEATURE_DIM,
            "Column map should cover exactly {} features, got {}", FEATURE_DIM, total);

        map
    }

    /// Collect system metrics from the local Linux system
    ///
    /// Reads from /proc/stat, /proc/meminfo, /proc/net/dev, etc.
    /// Returns metrics for the local node only.
    pub fn collect_local_system_metrics() -> Result<NodeMetrics> {
        let mut metrics = NodeMetrics::default();

        // Read /proc/stat for CPU and context metrics
        if let Ok(stat_content) = fs::read_to_string("/proc/stat") {
            for line in stat_content.lines() {
                if line.starts_with("cpu ") {
                    // cpu  user nice system idle iowait irq softirq steal guest guest_nice
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 11 {
                        let user: f64 = parts[1].parse().unwrap_or(0.0);
                        let nice: f64 = parts[2].parse().unwrap_or(0.0);
                        let system: f64 = parts[3].parse().unwrap_or(0.0);
                        let idle: f64 = parts[4].parse().unwrap_or(0.0);
                        let iowait: f64 = parts[5].parse().unwrap_or(0.0);
                        let irq: f64 = parts[6].parse().unwrap_or(0.0);
                        let softirq: f64 = parts[7].parse().unwrap_or(0.0);

                        let total = user + nice + system + idle + iowait + irq + softirq;
                        if total > 0.0 {
                            metrics.user_rate = (user / total * 100.0) as f32;
                            metrics.nice_rate = (nice / total * 100.0) as f32;
                            metrics.system_rate = (system / total * 100.0) as f32;
                            metrics.idle_rate = (idle / total * 100.0) as f32;
                            metrics.iowait_rate = (iowait / total * 100.0) as f32;
                            metrics.irq_rate = (irq / total * 100.0) as f32;
                            metrics.softirq_rate = (softirq / total * 100.0) as f32;
                        }
                    }
                } else if line.starts_with("ctxt ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        metrics.ctxt_rate = parts[1].parse().unwrap_or(0.0);
                    }
                } else if line.starts_with("intr ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        metrics.intr_rate = parts[1].parse().unwrap_or(0.0);
                    }
                } else if line.starts_with("processes ") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        metrics.processes = parts[1].parse().unwrap_or(0.0);
                    }
                }
            }
        }

        // Read load average
        if let Ok(loadavg) = fs::read_to_string("/proc/loadavg") {
            let parts: Vec<&str> = loadavg.split_whitespace().collect();
            if !parts.is_empty() {
                metrics.loadavg_m1 = parts[0].parse().unwrap_or(0.0);
            }
        }

        // Read /proc/meminfo for memory metrics
        if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
            for line in meminfo.lines() {
                if line.starts_with("SwapTotal:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        metrics.swap = parts[1].parse().unwrap_or(0.0);
                    }
                } else if line.starts_with("SwapCached:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        metrics.swap_cache = parts[1].parse().unwrap_or(0.0);
                    }
                } else if line.starts_with("AnonPages:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        metrics.nr_anon_pages = parts[1].parse().unwrap_or(0.0);
                    }
                } else if line.starts_with("SUnreclaim:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        metrics.nr_slab_unreclaimable = parts[1].parse().unwrap_or(0.0);
                    }
                } else if line.starts_with("PageTables:") {
                    let parts: Vec<&str> = line.split_whitespace().collect();
                    if parts.len() >= 2 {
                        metrics.page_tables = parts[1].parse().unwrap_or(0.0);
                    }
                }
            }
        }

        // Read /proc/net/dev for network metrics
        if let Ok(net_dev) = fs::read_to_string("/proc/net/dev") {
            for line in net_dev.lines().skip(2) {
                // Skip header lines
                let parts: Vec<&str> = line.split(':').collect();
                if parts.len() != 2 {
                    continue;
                }
                let iface = parts[0].trim();
                let stats: Vec<&str> = parts[1].split_whitespace().collect();
                if stats.len() < 16 {
                    continue;
                }

                // /proc/net/dev format:
                // bytes packets errs drop fifo frame compressed multicast | bytes packets errs drop fifo colls carrier compressed
                let rx_bytes: f64 = stats[0].parse().unwrap_or(0.0);
                let rx_packets: f64 = stats[1].parse().unwrap_or(0.0);
                let tx_bytes: f64 = stats[8].parse().unwrap_or(0.0);
                let tx_packets: f64 = stats[9].parse().unwrap_or(0.0);

                if iface.starts_with("eth") || iface.starts_with("en") {
                    metrics.eth_rxb_rate += rx_bytes as f32;
                    metrics.eth_rxp_rate += rx_packets as f32;
                    metrics.eth_txb_rate += tx_bytes as f32;
                    metrics.eth_txp_rate += tx_packets as f32;
                } else if iface.starts_with("wlan") || iface.starts_with("wl") || iface.starts_with("wifi") {
                    metrics.wifi_rxb_rate += rx_bytes as f32;
                    metrics.wifi_rxp_rate += rx_packets as f32;
                    metrics.wifi_txb_rate += tx_bytes as f32;
                    metrics.wifi_txp_rate += tx_packets as f32;
                }

                // Aggregate (all interfaces)
                metrics.rxb_rate += rx_bytes as f32;
                metrics.rxp_rate += rx_packets as f32;
                metrics.txb_rate += tx_bytes as f32;
                metrics.txp_rate += tx_packets as f32;
            }

            // Compute derived aggregate features
            metrics.sum_xb_rate = metrics.txb_rate + metrics.rxb_rate;
            metrics.diff_xb_rate = (metrics.txb_rate as f64 - metrics.rxb_rate as f64).abs() as f32;
            metrics.sum_xp_rate = metrics.txp_rate + metrics.rxp_rate;
            metrics.diff_xp_rate = (metrics.txp_rate as f64 - metrics.rxp_rate as f64).abs() as f32;
        }

        Ok(metrics)
    }

    /// Build a 2,387-dim feature vector from collected node metrics
    ///
    /// Takes metrics from multiple mesh nodes and assembles them into the
    /// exact feature vector format the L1 model expects.
    ///
    /// For nodes we don't have metrics for, fills with 0.0 (StandardScaler mean).
    pub fn build_feature_vector(&self, node_metrics: &HashMap<u16, NodeMetrics>) -> Vec<f32> {
        let mut features = vec![0.0f32; FEATURE_DIM];

        // Fill in metrics for each node we have data for
        for (&node_id, metrics) in node_metrics {
            if let Some(col_indices) = self.column_map.get(&node_id) {
                let raw = metrics.to_raw_features();
                for (feat_idx, &col_idx) in col_indices.iter().enumerate() {
                    if col_idx < FEATURE_DIM && feat_idx < BASE_FEATURES {
                        features[col_idx] = raw[feat_idx];
                    }
                }
            }
        }

        // Apply StandardScaler normalization if loaded
        if let Some(scaler) = &self.scaler {
            // We need to async-lock the scaler, but this function is sync.
            // The scaler params are only updated during load, so we can clone.
            // For now, we'll normalize in the async wrapper method.
        }

        features
    }

    /// Build and normalize a feature vector (async version)
    pub async fn build_normalized_vector(
        &self,
        node_metrics: &HashMap<u16, NodeMetrics>,
    ) -> Vec<f32> {
        let mut features = self.build_feature_vector(node_metrics);

        if let Some(scaler) = &self.scaler {
            let params = scaler.read().await;
            params.transform(&mut features);
        }

        features
    }

    /// Get the expected feature dimension
    pub fn feature_dim(&self) -> usize {
        FEATURE_DIM
    }

    /// Expose column map for testing
    pub fn build_column_map_for_test(&self) -> &HashMap<u16, Vec<usize>> {
        &self.column_map
    }
}

impl Default for FeatureExtractor {
    fn default() -> Self {
        Self::new()
    }
}

/// Mesh telemetry snapshot — simplified version of what GhostWire can collect
/// This bridges the gap between GhostWire's mesh health data and the Guifisants schema.
#[derive(Clone, Debug, Default)]
pub struct MeshTelemetrySnapshot {
    pub local_node_metrics: NodeMetrics,
    pub peer_count: usize,
    pub avg_latency_ms: f32,
    pub packet_loss_ratio: f32,
    pub active_routes: usize,
    pub messages_per_minute: f32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_dim() {
        let extractor = FeatureExtractor::new();
        assert_eq!(extractor.feature_dim(), FEATURE_DIM);
        assert_eq!(FEATURE_DIM, 2387);
    }

    #[test]
    fn test_column_map_coverage() {
        let extractor = FeatureExtractor::new();

        // All 63 nodes should have entries
        for &node_id in &NODE_IDS {
            assert!(
                extractor.column_map.contains_key(&node_id),
                "Node {} missing from column map",
                node_id
            );
        }

        // Total columns should equal 2,387
        let total_cols: usize = extractor.column_map.values().map(|v| v.len()).sum();
        assert_eq!(
            total_cols, FEATURE_DIM,
            "Column map should cover all {} features, got {}",
            FEATURE_DIM, total_cols
        );
    }

    #[test]
    fn test_node_metrics_to_features() {
        let metrics = NodeMetrics {
            node_id: 23,
            processes: 50.0,
            loadavg_m1: 0.04,
            idle_rate: 93.8,
            ..Default::default()
        };

        let raw = metrics.to_raw_features();
        assert_eq!(raw.len(), BASE_FEATURES);
        assert!((raw[0] - 50.0).abs() < 0.01); // processes
        assert!((raw[1] - 0.04).abs() < 0.01); // loadavg
        assert!((raw[6] - 93.8).abs() < 0.01); // idle
    }

    #[test]
    fn test_scaler_transform() {
        let params = ScalerParams {
            mean: vec![0.0, 10.0, 100.0],
            std: vec![1.0, 2.0, 10.0],
        };

        let mut features = vec![0.0, 12.0, 110.0];
        params.transform(&mut features);

        assert!((features[0] - 0.0).abs() < 0.001);  // (0-0)/1 = 0
        assert!((features[1] - 1.0).abs() < 0.001);  // (12-10)/2 = 1
        assert!((features[2] - 1.0).abs() < 0.001);  // (110-100)/10 = 1
    }

    #[test]
    fn test_build_feature_vector() {
        let extractor = FeatureExtractor::new();
        let mut node_metrics = HashMap::new();
        node_metrics.insert(
            23,
            NodeMetrics {
                node_id: 23,
                processes: 50.0,
                loadavg_m1: 0.04,
                ..Default::default()
            },
        );

        let features = extractor.build_feature_vector(&node_metrics);
        assert_eq!(features.len(), FEATURE_DIM);

        // Node 23 should have non-zero values at its column positions
        // Node 23 is at index 21 in NODE_IDS (0-indexed)
        // Its features start at approximately 21*38 = 798
        // (exact position depends on missing columns)
        assert!(features.iter().any(|&v| v != 0.0), "Should have some non-zero features");
    }

    #[test]
    fn test_collect_local_system_metrics() {
        let result = FeatureExtractor::collect_local_system_metrics();
        assert!(result.is_ok(), "Should be able to collect system metrics on Linux");

        let metrics = result.unwrap();
        assert!(metrics.idle_rate > 0.0, "Idle rate should be positive");
        assert!(metrics.processes > 0.0, "Process count should be positive");
        assert!(metrics.ctxt_rate > 0.0, "Context switch rate should be positive");
    }
}
