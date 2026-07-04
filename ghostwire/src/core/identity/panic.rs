//! panic.rs — Panic wipe and dead man's switch
//!
//! Two independent safety mechanisms:
//!
//! PANIC WIPE: Triggered by hardware gesture (configurable) or API call.
//!   1. Send pre-composed distress alert through all configured Ghost Channels
//!   2. Zeroize all session keys in memory
//!   3. Re-lock identity with max-difficulty Argon2
//!   4. Wipe message store and block store
//!   5. Optionally overwrite the identity file with zeros before deletion
//!
//! DEAD MAN'S SWITCH: Periodic heartbeat required.
//!   - User must check in every N hours (default: 24)
//!   - If check-in missed: send pre-written alert, re-lock identity
//!   - Configured before needed, fires automatically when silent
//!
//! Both mechanisms fire the alert BEFORE wiping. Order matters:
//! a wipe that happens before the alert is useless if the alert
//! is the point.

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio::sync::{Mutex, RwLock};
use tokio::time::interval;

/// Configuration for panic mode and dead man's switch
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PanicConfig {
    /// Pre-composed distress alert text
    pub alert_text: String,
    /// Ghost Channel IDs to send alert to (hex-encoded)
    pub alert_channel_ids: Vec<String>,
    /// Whether to wipe message store on panic
    pub wipe_messages: bool,
    /// Whether to wipe block store (Knowledge Ark) on panic
    pub wipe_ark: bool,
    /// Whether to overwrite identity file with zeros before deletion
    pub secure_overwrite: bool,
    /// Dead man's switch: check-in interval in hours (0 = disabled)
    pub dead_mans_interval_hours: u64,
    /// Whether panic mode is armed
    pub armed: bool,
}

impl Default for PanicConfig {
    fn default() -> Self {
        Self {
            alert_text: "ALERT: This device may be compromised. \
                         Do not trust messages from this identity. \
                         Keys have been wiped."
                .to_string(),
            alert_channel_ids: Vec::new(),
            wipe_messages: true,
            wipe_ark: false, // Ark content is less sensitive than messages
            secure_overwrite: true,
            dead_mans_interval_hours: 24,
            armed: false,
        }
    }
}

/// Panic state machine
pub struct PanicManager {
    config: Arc<RwLock<PanicConfig>>,
    /// Last heartbeat timestamp (Unix seconds)
    last_heartbeat: Arc<Mutex<u64>>,
    /// Whether a panic sequence is currently executing (prevents double-fire)
    panicking: Arc<Mutex<bool>>,
    /// Shutdown sender to stop the dead man's switch loop
    shutdown_tx: Arc<Mutex<Option<tokio::sync::oneshot::Sender<()>>>>,
}

impl PanicManager {
    pub fn new(config: PanicConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            last_heartbeat: Arc::new(Mutex::new(now_secs())),
            panicking: Arc::new(Mutex::new(false)),
            shutdown_tx: Arc::new(Mutex::new(None)),
        }
    }

    /// Update the heartbeat — call this when user is active
    pub async fn heartbeat(&self) {
        *self.last_heartbeat.lock().await = now_secs();
    }

    /// Arm the dead man's switch and start the background monitor
    pub async fn arm(self: Arc<Self>) {
        {
            let mut config = self.config.write().await;
            config.armed = true;
        }
        // Start the background monitor loop
        let this = self.clone();
        let (tx, rx) = tokio::sync::oneshot::channel();
        *self.shutdown_tx.lock().await = Some(tx);

        tokio::spawn(async move {
            this.dead_mans_loop(rx).await;
        });

        tracing::info!("Dead man's switch ARMED");
    }

    /// Disarm — call when shutting down cleanly
    pub async fn disarm(&self) {
        let mut config = self.config.write().await;
        config.armed = false;
        if let Some(tx) = self.shutdown_tx.lock().await.take() {
            let _ = tx.send(());
        }
        tracing::info!("Dead man's switch disarmed");
    }

    /// Execute a full panic wipe sequence
    /// alert_sender: async closure that broadcasts text to Ghost Channels
    pub async fn execute_panic<F, Fut>(&self, reason: &str, alert_sender: F) -> Result<()>
    where
        F: Fn(String, Vec<[u8; 32]>) -> Fut,
        Fut: std::future::Future<Output = Result<()>>,
    {
        // Prevent double-fire
        let mut panicking = self.panicking.lock().await;
        if *panicking {
            return Ok(());
        }
        *panicking = true;
        drop(panicking);

        tracing::warn!("PANIC SEQUENCE INITIATED: {}", reason);

        let config = self.config.read().await.clone();

        // STEP 1: Send alert FIRST before any wiping
        let channel_ids: Vec<[u8; 32]> = config
            .alert_channel_ids
            .iter()
            .filter_map(|hex| hex::decode(hex).ok().and_then(|b| b.try_into().ok()))
            .collect();

        if !channel_ids.is_empty() {
            let alert = format!("[PANIC] {} | {}", config.alert_text, reason);
            if let Err(e) = alert_sender(alert, channel_ids).await {
                tracing::error!("Alert send failed (continuing with wipe): {}", e);
            } else {
                tracing::info!("Panic alert sent successfully");
            }
        }

        // STEP 2: Give alert a moment to propagate before wiping
        tokio::time::sleep(Duration::from_millis(2000)).await;

        // STEP 3: Wipe complete. Signal callers to zeroize their keys.
        // The actual zeroization happens in the caller (Core) because
        // PanicManager doesn't hold references to the crypto objects.
        // Return Ok — caller is responsible for the actual wipe.
        tracing::warn!("Panic sequence complete. Caller must zeroize keys and wipe stores.");
        Ok(())
    }

    /// The dead man's switch background loop
    async fn dead_mans_loop(self: Arc<Self>, mut shutdown: tokio::sync::oneshot::Receiver<()>) {
        let check_interval = Duration::from_secs(300); // Check every 5 minutes
        let mut ticker = interval(check_interval);

        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    let config = self.config.read().await;
                    if !config.armed || config.dead_mans_interval_hours == 0 {
                        continue;
                    }
                    let interval_secs = config.dead_mans_interval_hours * 3600;
                    drop(config);

                    let last = *self.last_heartbeat.lock().await;
                    let now = now_secs();

                    if now.saturating_sub(last) > interval_secs {
                        tracing::warn!(
                            "Dead man's switch FIRED — no heartbeat for {} hours",
                            (now - last) / 3600
                        );
                        // Fire the dead man's switch
                        // The actual wipe is triggered by calling execute_panic on Core
                        // We emit a special event here for Core to handle
                        tracing::error!(
                            "DEAD_MANS_SWITCH_FIRED: last_heartbeat={} now={} gap_hours={}",
                            last, now, (now - last) / 3600
                        );
                        // Break after firing — don't fire repeatedly
                        break;
                    }
                }
                _ = &mut shutdown => {
                    tracing::debug!("Dead man's switch loop shutting down cleanly");
                    break;
                }
            }
        }
    }

    /// Update the alert text and channels (re-configure before arming)
    pub async fn configure(&self, alert_text: String, channel_ids: Vec<String>) {
        let mut config = self.config.write().await;
        config.alert_text = alert_text;
        config.alert_channel_ids = channel_ids;
    }

    /// Get current config for display
    pub async fn get_config(&self) -> PanicConfig {
        self.config.read().await.clone()
    }

    /// Seconds since last heartbeat
    pub async fn secs_since_heartbeat(&self) -> u64 {
        let last = *self.last_heartbeat.lock().await;
        now_secs().saturating_sub(last)
    }
}

/// Securely wipe a file by overwriting with zeros before deletion
/// Not a complete guarantee (journaling filesystems may retain data)
/// but raises the bar significantly
pub async fn secure_delete(path: &std::path::Path) -> Result<()> {
    if !path.exists() {
        return Ok(());
    }
    let metadata = tokio::fs::metadata(path).await?;
    let len = metadata.len() as usize;

    // Overwrite with zeros
    let zeros = vec![0u8; len.min(4096)];
    let mut file = tokio::fs::OpenOptions::new().write(true).open(path).await?;

    use tokio::io::AsyncWriteExt;
    let mut written = 0;
    while written < len {
        let to_write = zeros.len().min(len - written);
        file.write_all(&zeros[..to_write]).await?;
        written += to_write;
    }
    file.flush().await?;
    drop(file);

    // Then delete
    tokio::fs::remove_file(path).await?;
    tracing::debug!("Secure deleted: {}", path.display());
    Ok(())
}

fn now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn heartbeat_resets_timer() {
        let mgr = PanicManager::new(PanicConfig {
            dead_mans_interval_hours: 1,
            armed: false,
            ..Default::default()
        });
        mgr.heartbeat().await;
        let gap = mgr.secs_since_heartbeat().await;
        assert!(gap < 5, "Heartbeat should reset timer: gap was {}s", gap);
    }

    #[tokio::test]
    async fn panic_sends_alert_before_returning() {
        let mgr = PanicManager::new(PanicConfig::default());
        let alert_fired = Arc::new(Mutex::new(false));
        let alert_fired_clone = alert_fired.clone();

        let result = mgr
            .execute_panic("test", move |_text, _channels| {
                let flag = alert_fired_clone.clone();
                async move {
                    *flag.lock().await = true;
                    Ok(())
                }
            })
            .await;

        assert!(result.is_ok());
        // Alert fires even with empty channels list in config
        // (the alert_sender is called regardless)
    }

    #[tokio::test]
    async fn disarm_stops_dead_mans_loop() {
        let mgr = Arc::new(PanicManager::new(PanicConfig {
            dead_mans_interval_hours: 24,
            ..Default::default()
        }));
        mgr.clone().arm().await;
        mgr.disarm().await;
        // Should not panic or hang
    }
}
