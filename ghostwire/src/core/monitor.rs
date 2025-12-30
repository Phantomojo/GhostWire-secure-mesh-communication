use crate::core::disaster::DisasterMetrics;
#[cfg(feature = "mesh")]
use crate::core::mesh::MeshManager;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::RwLock;
use tracing::{debug, error, info};

/// Periodically monitors network health.
pub struct NetworkMonitor {
    #[cfg(feature = "mesh")]
    mesh_manager: Option<Arc<RwLock<MeshManager>>>,
    current_metrics: Arc<RwLock<DisasterMetrics>>,
    monitor_interval: Duration,
    canary_target: String, // e.g., "8.8.8.8" or a known bootstrap node
}

impl NetworkMonitor {
    pub fn new(
        #[cfg(feature = "mesh")] mesh_manager: Option<Arc<RwLock<MeshManager>>>,
        interval: Duration,
    ) -> Self {
        Self {
            #[cfg(feature = "mesh")]
            mesh_manager,
            current_metrics: Arc::new(RwLock::new(DisasterMetrics::default())),
            monitor_interval: interval,
            canary_target: "8.8.8.8:53".to_string(), // DNS for reachability check
        }
    }

    pub async fn get_metrics(&self) -> DisasterMetrics {
        self.current_metrics.read().await.clone()
    }

    /// Starts the background monitoring loop.
    pub async fn start(self: Arc<Self>) {
        info!("Starting Network Monitor heartbeat...");
        let monitor = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(monitor.monitor_interval);
            loop {
                interval.tick().await;
                if let Err(e) = monitor.update_metrics().await {
                    error!("Fatal error updating network metrics: {}", e);
                }
            }
        });
    }

    async fn update_metrics(&self) -> anyhow::Result<()> {
        let mut metrics = DisasterMetrics::default();

        // 1. Check Mesh Peer Count
        #[cfg(feature = "mesh")]
        if let Some(mesh_mgr) = &self.mesh_manager {
            let mgr = mesh_mgr.read().await;
            let stats = mgr.get_stats().await;
            metrics.active_peers = stats.online_nodes as u32;
        }

        // 2. Check Internet Reachability (Simple TCP connect to DNS)
        let start = SystemTime::now();
        match tokio::net::TcpStream::connect(&self.canary_target).await {
            Ok(_) => {
                metrics.is_internet_reachable = true;
                metrics.latency_ms = start.elapsed()?.as_millis() as u32;
            }
            Err(_) => {
                metrics.is_internet_reachable = false;
                metrics.latency_ms = 9999; // Peak latency for unreachable
            }
        }

        // 3. Packet Loss Estimation (In a real system, we'd track retransmissions in noise/yamux)
        // For now, we'll derive it from consecutive failed canary checks if we want to be sophisticated
        // but for this v1 upgrade, we'll keep it to basic success/fail.

        metrics.last_update = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let mut current = self.current_metrics.write().await;
        *current = metrics;

        debug!(
            "Network Metrics Updated: Peers={}, Internet={}, Latency={}ms",
            current.active_peers, current.is_internet_reachable, current.latency_ms
        );

        Ok(())
    }
}
