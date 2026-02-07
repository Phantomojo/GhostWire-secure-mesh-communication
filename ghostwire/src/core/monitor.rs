use crate::core::disaster::DisasterMetrics;
#[cfg(feature = "mesh")]
use crate::core::mesh::MeshManager;
use std::sync::atomic::{AtomicBool, Ordering};
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
    running: Arc<AtomicBool>,
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
            running: Arc::new(AtomicBool::new(false)),
        }
    }

    pub async fn get_metrics(&self) -> DisasterMetrics {
        self.current_metrics.read().await.clone()
    }

    /// Starts the background monitoring loop.
    pub async fn start(self: Arc<Self>) {
        info!("Starting Network Monitor heartbeat...");
        self.running.store(true, Ordering::SeqCst);
        let monitor = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(monitor.monitor_interval);
            while monitor.running.load(Ordering::SeqCst) {
                interval.tick().await;
                if !monitor.running.load(Ordering::SeqCst) {
                    break;
                }
                if let Err(e) = monitor.update_metrics().await {
                    error!("Fatal error updating network metrics: {}", e);
                }
            }
            info!("Network Monitor heartbeat stopped.");
        });
    }

    pub fn stop(&self) {
        self.running.store(false, Ordering::SeqCst);
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

        // 3. Packet Loss Estimation (Derive from latency variability and reachability)
        if metrics.is_internet_reachable {
            // Simulate 1% - 5% baseline loss if latency is high, otherwise 0%
            if metrics.latency_ms > 200 {
                metrics.packet_loss_ratio = 0.05;
            } else if metrics.latency_ms > 100 {
                metrics.packet_loss_ratio = 0.02;
            } else {
                metrics.packet_loss_ratio = 0.0;
            }
        } else {
            metrics.packet_loss_ratio = 1.0; // Total loss
        }

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
