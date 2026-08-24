//! Platform medium-adapter contracts.
//!
//! Medium adapters own link/platform concerns. They never define GhostWire trust,
//! decrypt application messages, or bypass the normal authenticated route layer.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, Ord, PartialOrd)]
pub enum MediumId {
    ManagedWifi,
    WifiDirect,
    WifiAware,
    Quic,
    StealthTcp,
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

#[derive(Debug, Clone)]
pub struct WifiDirectAdapter;

#[derive(Debug, Clone)]
pub struct WifiAwareAdapter;

fn unsupported_snapshot(medium: MediumId, name: &str) -> AdapterSnapshot {
    AdapterSnapshot {
        medium,
        state: AdapterState::Unsupported,
        capabilities: MediumCapabilities {
            implemented: false,
            platform: name.to_string(),
            requires_os_service: true,
            supports_discovery: true,
            supports_ip_route: true,
        },
        last_error: Some("Platform adapter boundary exists; OS provider is not installed".into()),
        routes: BTreeMap::new(),
    }
}

#[async_trait]
impl MediumAdapter for WifiDirectAdapter {
    fn medium(&self) -> MediumId {
        MediumId::WifiDirect
    }
    fn capabilities(&self) -> MediumCapabilities {
        unsupported_snapshot(MediumId::WifiDirect, "android-or-linux-p2p").capabilities
    }
    async fn start(&self) -> Result<AdapterSnapshot, String> {
        Ok(unsupported_snapshot(
            MediumId::WifiDirect,
            "android-or-linux-p2p",
        ))
    }
    async fn stop(&self) -> Result<(), String> {
        Ok(())
    }
    async fn snapshot(&self) -> AdapterSnapshot {
        unsupported_snapshot(MediumId::WifiDirect, "android-or-linux-p2p")
    }
}

#[async_trait]
impl MediumAdapter for WifiAwareAdapter {
    fn medium(&self) -> MediumId {
        MediumId::WifiAware
    }
    fn capabilities(&self) -> MediumCapabilities {
        unsupported_snapshot(MediumId::WifiAware, "android-nan").capabilities
    }
    async fn start(&self) -> Result<AdapterSnapshot, String> {
        Ok(unsupported_snapshot(MediumId::WifiAware, "android-nan"))
    }
    async fn stop(&self) -> Result<(), String> {
        Ok(())
    }
    async fn snapshot(&self) -> AdapterSnapshot {
        unsupported_snapshot(MediumId::WifiAware, "android-nan")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn wifi_direct_is_not_reported_ready_without_platform_provider() {
        let snapshot = WifiDirectAdapter.snapshot().await;
        assert_eq!(snapshot.state, AdapterState::Unsupported);
        assert!(!snapshot.capabilities.implemented);
    }

    #[tokio::test]
    async fn wifi_aware_is_explicitly_android_scoped() {
        let snapshot = WifiAwareAdapter.snapshot().await;
        assert_eq!(snapshot.capabilities.platform, "android-nan");
        assert_eq!(snapshot.state, AdapterState::Unsupported);
    }
}
