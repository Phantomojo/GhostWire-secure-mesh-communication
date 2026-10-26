//! Platform medium-adapter contracts.
//!
//! Medium adapters own link/platform concerns. They never define GhostWire trust,
//! decrypt application messages, or bypass the normal authenticated route layer.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::process::Command;
use tokio::process::Command as TokioCommand;
use tracing::{debug, info, warn};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub enum MediumId {
    ManagedWifi,
    WifiDirect,
    WifiAware,
    Reticulum,
    Bluetooth,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdapterState {
    Compiled,
    Unsupported,
    Disabled,
    Configured,
    Starting,
    LinkUp,
    Listening,
    Discovering,
    Handshaking,
    AuthenticatedReady,
    Degraded,
    Draining,
    Stopped,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RouteState {
    Discovered,
    Dialing,
    Handshaking,
    AuthenticatedReady,
    Degraded,
    Draining,
    Closed,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct MediumCapabilities {
    pub implemented: bool,
    pub platform: String,
    pub requires_os_service: bool,
    pub supports_discovery: bool,
    pub supports_ip_route: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct AdapterSnapshot {
    pub medium: MediumId,
    pub state: AdapterState,
    pub capabilities: MediumCapabilities,
    pub last_error: Option<String>,
    pub routes: BTreeMap<String, RouteSnapshot>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RouteSnapshot {
    pub route_id: String,
    pub peer_id: Option<String>,
    pub address: Option<String>,
    pub state: RouteState,
    pub authenticated: bool,
    pub last_rtt_ms: Option<u64>,
    pub bytes_sent: u64,
    pub bytes_received: u64,
}

#[async_trait]
pub trait MediumAdapter: Send + Sync {
    fn medium(&self) -> MediumId;
    fn capabilities(&self) -> MediumCapabilities;
    async fn start(&self) -> Result<AdapterSnapshot, String>;
    async fn stop(&self) -> Result<(), String>;
    async fn snapshot(&self) -> AdapterSnapshot;
}

// ---------------------------------------------------------------------------
// Managed WiFi (standard 802.11 infrastructure mode via NetworkManager/nmcli)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct ManagedWifiAdapter;

impl ManagedWifiAdapter {
    fn has_nmcli() -> bool {
        Command::new("nmcli").arg("--version").output().is_ok()
    }
}

#[async_trait]
impl MediumAdapter for ManagedWifiAdapter {
    fn medium(&self) -> MediumId {
        MediumId::ManagedWifi
    }
    fn capabilities(&self) -> MediumCapabilities {
        MediumCapabilities {
            implemented: true,
            platform: "linux-nmcli".to_string(),
            requires_os_service: true,
            supports_discovery: true,
            supports_ip_route: true,
        }
    }
    async fn start(&self) -> Result<AdapterSnapshot, String> {
        if !Self::has_nmcli() {
            return Ok(AdapterSnapshot {
                medium: MediumId::ManagedWifi,
                state: AdapterState::Unsupported,
                capabilities: self.capabilities(),
                last_error: Some("nmcli not found — install NetworkManager".into()),
                routes: BTreeMap::new(),
            });
        }
        // Check WiFi radio state
        let output = TokioCommand::new("nmcli")
            .args(["radio", "wifi"])
            .output()
            .await
            .map_err(|e| format!("nmcli radio check failed: {}", e))?;
        let radio_on = String::from_utf8_lossy(&output.stdout).trim() == "enabled";

        if !radio_on {
            let _ = TokioCommand::new("nmcli")
                .args(["radio", "wifi", "on"])
                .output()
                .await;
        }

        info!("ManagedWiFi adapter started (radio was {})", if radio_on { "on" } else { "off → enabled" });

        Ok(AdapterSnapshot {
            medium: MediumId::ManagedWifi,
            state: AdapterState::Configured,
            capabilities: self.capabilities(),
            last_error: None,
            routes: BTreeMap::new(),
        })
    }
    async fn stop(&self) -> Result<(), String> {
        debug!("ManagedWiFi adapter stopped");
        Ok(())
    }
    async fn snapshot(&self) -> AdapterSnapshot {
        let state = if Self::has_nmcli() {
            AdapterState::Configured
        } else {
            AdapterState::Unsupported
        };
        AdapterSnapshot {
            medium: MediumId::ManagedWifi,
            state,
            capabilities: self.capabilities(),
            last_error: None,
            routes: BTreeMap::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// WiFi Direct (P2P via wpa_supplicant)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct WifiDirectAdapter;

impl WifiDirectAdapter {
    fn has_wpa_cli() -> bool {
        Command::new("wpa_cli").arg("ping").output().map(|o| o.status.success()).unwrap_or(false)
    }
}

#[async_trait]
impl MediumAdapter for WifiDirectAdapter {
    fn medium(&self) -> MediumId {
        MediumId::WifiDirect
    }
    fn capabilities(&self) -> MediumCapabilities {
        MediumCapabilities {
            implemented: true,
            platform: "linux-wpa-p2p".to_string(),
            requires_os_service: true,
            supports_discovery: true,
            supports_ip_route: true,
        }
    }
    async fn start(&self) -> Result<AdapterSnapshot, String> {
        if !Self::has_wpa_cli() {
            return Ok(AdapterSnapshot {
                medium: MediumId::WifiDirect,
                state: AdapterState::Unsupported,
                capabilities: self.capabilities(),
                last_error: Some("wpa_cli not found — install wpa_supplicant".into()),
                routes: BTreeMap::new(),
            });
        }

        let output = TokioCommand::new("wpa_cli")
            .args(["p2p_find"])
            .output()
            .await
            .map_err(|e| format!("wpa_cli p2p_find failed: {}", e))?;

        if output.status.success() {
            info!("WiFi Direct P2P discovery started");
            Ok(AdapterSnapshot {
                medium: MediumId::WifiDirect,
                state: AdapterState::Discovering,
                capabilities: self.capabilities(),
                last_error: None,
                routes: BTreeMap::new(),
            })
        } else {
            let stderr = String::from_utf8_lossy(&output.stderr);
            warn!("WiFi Direct P2P find failed: {}", stderr);
            Ok(AdapterSnapshot {
                medium: MediumId::WifiDirect,
                state: AdapterState::Degraded,
                capabilities: self.capabilities(),
                last_error: Some(format!("p2p_find failed: {}", stderr)),
                routes: BTreeMap::new(),
            })
        }
    }
    async fn stop(&self) -> Result<(), String> {
        let _ = TokioCommand::new("wpa_cli")
            .args(["p2p_stop_find"])
            .output()
            .await;
        debug!("WiFi Direct adapter stopped");
        Ok(())
    }
    async fn snapshot(&self) -> AdapterSnapshot {
        let state = if Self::has_wpa_cli() {
            AdapterState::Configured
        } else {
            AdapterState::Unsupported
        };
        AdapterSnapshot {
            medium: MediumId::WifiDirect,
            state,
            capabilities: self.capabilities(),
            last_error: None,
            routes: BTreeMap::new(),
        }
    }
}

// ---------------------------------------------------------------------------
// WiFi Aware (Android NAN — not available on Linux desktop)
// ---------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub struct WifiAwareAdapter;

#[async_trait]
impl MediumAdapter for WifiAwareAdapter {
    fn medium(&self) -> MediumId {
        MediumId::WifiAware
    }
    fn capabilities(&self) -> MediumCapabilities {
        MediumCapabilities {
            implemented: false,
            platform: "android-nan".to_string(),
            requires_os_service: true,
            supports_discovery: true,
            supports_ip_route: true,
        }
    }
    async fn start(&self) -> Result<AdapterSnapshot, String> {
        Ok(AdapterSnapshot {
            medium: MediumId::WifiAware,
            state: AdapterState::Unsupported,
            capabilities: self.capabilities(),
            last_error: Some("WiFi Aware (NAN) requires Android platform".into()),
            routes: BTreeMap::new(),
        })
    }
    async fn stop(&self) -> Result<(), String> {
        Ok(())
    }
    async fn snapshot(&self) -> AdapterSnapshot {
        AdapterSnapshot {
            medium: MediumId::WifiAware,
            state: AdapterState::Unsupported,
            capabilities: self.capabilities(),
            last_error: Some("WiFi Aware (NAN) requires Android platform".into()),
            routes: BTreeMap::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn managed_wifi_reports_implemented() {
        let adapter = ManagedWifiAdapter;
        let caps = adapter.capabilities();
        assert!(caps.implemented);
        assert_eq!(caps.platform, "linux-nmcli");
    }

    #[tokio::test]
    async fn wifi_direct_reports_implemented() {
        let adapter = WifiDirectAdapter;
        let caps = adapter.capabilities();
        assert!(caps.implemented);
        assert_eq!(caps.platform, "linux-wpa-p2p");
    }

    #[tokio::test]
    async fn wifi_aware_is_android_only() {
        let adapter = WifiAwareAdapter;
        let snapshot = adapter.snapshot().await;
        assert_eq!(snapshot.capabilities.platform, "android-nan");
        assert_eq!(snapshot.state, AdapterState::Unsupported);
        assert!(!snapshot.capabilities.implemented);
    }

    #[tokio::test]
    async fn managed_wifi_adapter_starts() {
        let adapter = ManagedWifiAdapter;
        let snapshot = adapter.start().await.unwrap();
        assert!(snapshot.capabilities.implemented);
    }
}