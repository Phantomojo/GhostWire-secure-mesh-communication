//! TUI Daemon Client — HTTP client that talks to the GhostWire daemon
//!
//! Instead of holding Arc<Core> directly, the TUI connects to the daemon
//! via HTTP API. This means:
//! - No database lock conflicts (only the daemon owns the sled DB)
//! - Multiple clients can connect simultaneously
//! - The daemon stays alive when the TUI closes

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::time::Duration;

/// Base URL for the daemon API
fn api_base(port: u16) -> String {
    format!("http://127.0.0.1:{}", port)
}

/// Where the daemon writes the auth token
fn token_path() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join("ghostwire")
        .join("token")
}

/// Read the auth token from disk (same file the daemon writes)
fn read_auth_token() -> Option<String> {
    let path = token_path();
    match std::fs::read_to_string(&path) {
        Ok(content) => {
            let trimmed = content.trim().to_string();
            if trimmed.is_empty() {
                tracing::debug!(target: "tui", "Token file empty: {}", path.display());
                None
            } else {
                tracing::debug!(target: "tui", "Loaded auth token from {}", path.display());
                Some(trimmed)
            }
        }
        Err(e) => {
            tracing::debug!(target: "tui", "No token file: {}", e);
            None
        }
    }
}

/// Contact entry from the daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonContact {
    pub display_name: String,
    pub peer_id: String,
    pub online: bool,
    pub trust_score: f64,
    pub unread_count: usize,
    pub last_seen_secs: u64,
}

/// Message from the daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonMessage {
    pub sender: String,
    pub content: String,
    pub timestamp: u64,
    pub is_mine: bool,
}

/// Identity info from the daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonIdentity {
    pub id: String,
    pub nickname: String,
    pub did: String,
    pub is_locked: bool,
}

/// Peer info from the daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
#[allow(dead_code)] // ponytail: daemon peer type, not yet consumed by TUI
pub struct DaemonPeer {
    pub peer_id: String,
    pub display_name: String,
    pub online: bool,
    pub last_seen_secs: u64,
}

/// Network status from the daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonNetworkStatus {
    pub peer_count: usize,
    pub mode: String,
    pub encryption: String,
    pub bootstrap_state: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct DaemonDiscoveryHealth {
    pub enabled: bool,
    pub network_id: String,
    pub state: String,
    pub published_total: u64,
    pub accepted_total: u64,
    pub rejected_total: u64,
    pub candidate_count: usize,
    pub provider_active: bool,
}

/// The daemon HTTP client
pub struct DaemonClient {
    http: Client,
    base_url: String,
    port: u16,
    /// Auth token for X-Ghost-Token header
    auth_token: Option<String>,
}

impl DaemonClient {
    /// Create a new client connecting to the daemon on the given port
    pub fn new(port: u16) -> Self {
        Self {
            http: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("failed to build HTTP client"),
            base_url: api_base(port),
            port,
            auth_token: read_auth_token(),
        }
    }

    /// Get the daemon's port
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Clone for sending in a spawned task (reqwest::Client is cheap to clone)
    pub fn clone_for_send(&self) -> Self {
        Self {
            http: self.http.clone(),
            base_url: self.base_url.clone(),
            port: self.port,
            auth_token: self.auth_token.clone(),
        }
    }

    /// Check if the daemon is reachable
    pub async fn health_check(&self) -> bool {
        self.http
            .get(format!("{}/api/v2/ready", self.base_url))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    fn authenticated_get(&self, path: &str) -> reqwest::RequestBuilder {
        let req = self.http.get(format!("{}{}", self.base_url, path));
        if let Some(ref token) = self.auth_token {
            req.header("X-Ghost-Token", token)
        } else {
            req
        }
    }

    fn authenticated_post(&self, path: &str) -> reqwest::RequestBuilder {
        let req = self.http.post(format!("{}{}", self.base_url, path));
        if let Some(ref token) = self.auth_token {
            req.header("X-Ghost-Token", token)
        } else {
            req
        }
    }

    /// Get my identity info
    pub async fn get_identity(&self) -> Option<DaemonIdentity> {
        match self.authenticated_get("/api/v2/identity").send().await {
            Ok(resp) if resp.status().is_success() => {
                match resp.json::<serde_json::Value>().await {
                    Ok(val) => {
                        let data = val.get("data").unwrap_or(&val);
                        Some(DaemonIdentity {
                            id: data
                                .get("id")
                                .and_then(|v| v.as_str())
                                .unwrap_or("unknown")
                                .to_string(),
                            nickname: data
                                .get("nickname")
                                .and_then(|v| v.as_str())
                                .unwrap_or("unnamed")
                                .to_string(),
                            did: data
                                .get("did")
                                .and_then(|v| v.as_str())
                                .unwrap_or("")
                                .to_string(),
                            is_locked: data
                                .get("is_locked")
                                .and_then(|v| v.as_bool())
                                .unwrap_or(false),
                        })
                    }
                    Err(e) => {
                        tracing::warn!(target: "tui", "Failed to parse identity response: {}", e);
                        None
                    }
                }
            }
            Ok(resp) => {
                tracing::warn!(target: "tui", "Identity endpoint returned {}", resp.status());
                None
            }
            Err(e) => {
                tracing::warn!(target: "tui", "Failed to reach identity endpoint: {}", e);
                None
            }
        }
    }

    /// Get the list of contacts/peers
    pub async fn get_contacts(&self) -> Vec<DaemonContact> {
        match self.authenticated_get("/api/v2/peers").send().await {
            Ok(resp) if resp.status().is_success() => {
                match resp.json::<serde_json::Value>().await {
                    Ok(val) => {
                        let data = val.get("data").unwrap_or(&val);
                        if let Some(peers) = data.get("peers").and_then(|v| v.as_array()) {
                            peers
                                .iter()
                                .map(|p| DaemonContact {
                                    display_name: p
                                        .get("display_name")
                                        .or_else(|| p.get("nickname"))
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("Unknown")
                                        .to_string(),
                                    peer_id: p
                                        .get("peer_id")
                                        .or_else(|| p.get("id"))
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("")
                                        .to_string(),
                                    online: p
                                        .get("online")
                                        .or_else(|| p.get("connected"))
                                        .and_then(|v| v.as_bool())
                                        .unwrap_or(false),
                                    trust_score: p
                                        .get("trust_score")
                                        .and_then(|v| v.as_f64())
                                        .unwrap_or(0.5),
                                    unread_count: 0,
                                    last_seen_secs: p
                                        .get("last_seen_secs")
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(0),
                                })
                                .collect()
                        } else {
                            Vec::new()
                        }
                    }
                    Err(e) => {
                        tracing::warn!(target: "tui", "Failed to parse contacts response: {}", e);
                        Vec::new()
                    }
                }
            }
            Ok(resp) => {
                tracing::warn!(target: "tui", "Contacts endpoint returned {}", resp.status());
                Vec::new()
            }
            Err(e) => {
                tracing::warn!(target: "tui", "Failed to reach contacts endpoint: {}", e);
                Vec::new()
            }
        }
    }

    /// Get network status
    pub async fn get_network_status(&self) -> DaemonNetworkStatus {
        match self
            .authenticated_get("/api/v2/network/status")
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                match resp.json::<serde_json::Value>().await {
                    Ok(val) => {
                        let data = val.get("data").unwrap_or(&val);
                        DaemonNetworkStatus {
                            peer_count: data
                                .get("peer_count")
                                .or_else(|| data.get("active_peers"))
                                .and_then(|v| v.as_u64())
                                .unwrap_or(0) as usize,
                            mode: data
                                .get("transport_mode")
                                .or_else(|| data.get("mode"))
                                .and_then(|v| v.as_str())
                                .unwrap_or("Mesh P2P")
                                .to_string(),
                            encryption: data
                                .get("encryption")
                                .and_then(|v| v.as_str())
                                .unwrap_or("XChaCha20-Poly1305")
                                .to_string(),
                            bootstrap_state: data
                                .get("bootstrap_state")
                                .and_then(|v| v.as_str())
                                .unwrap_or("disabled")
                                .to_string(),
                        }
                    }
                    Err(e) => {
                        tracing::warn!(target: "tui", "Failed to parse network status: {}", e);
                        DaemonNetworkStatus {
                            peer_count: 0,
                            mode: "Mesh P2P".to_string(),
                            encryption: "XChaCha20-Poly1305".to_string(),
                            bootstrap_state: "unavailable".to_string(),
                        }
                    }
                }
            }
            Ok(resp) => {
                tracing::warn!(target: "tui", "Network status endpoint returned {}", resp.status());
                DaemonNetworkStatus {
                    peer_count: 0,
                    mode: "Mesh P2P".to_string(),
                    encryption: "XChaCha20-Poly1305".to_string(),
                    bootstrap_state: "unavailable".to_string(),
                }
            }
            Err(e) => {
                tracing::warn!(target: "tui", "Failed to reach network status endpoint: {}", e);
                DaemonNetworkStatus {
                    peer_count: 0,
                    mode: "Mesh P2P".to_string(),
                    encryption: "XChaCha20-Poly1305".to_string(),
                    bootstrap_state: "unavailable".to_string(),
                }
            }
        }
    }

    pub async fn get_discovery_health(&self) -> DaemonDiscoveryHealth {
        match self
            .authenticated_get("/api/v2/system/discovery")
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                let value = resp.json::<serde_json::Value>().await.unwrap_or_default();
                let data = value.get("data").unwrap_or(&value);
                serde_json::from_value(data.clone()).unwrap_or_default()
            }
            Ok(resp) => {
                tracing::debug!(target: "tui", "Discovery endpoint returned {}", resp.status());
                DaemonDiscoveryHealth::default()
            }
            Err(error) => {
                tracing::debug!(target: "tui", "Discovery endpoint unavailable: {}", error);
                DaemonDiscoveryHealth::default()
            }
        }
    }

    pub async fn publish_discovery(&self) -> Result<(), String> {
        match self
            .authenticated_post("/api/v2/system/discovery/publish")
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => Ok(()),
            Ok(resp) => Err(format!("Discovery publish rejected: {}", resp.status())),
            Err(error) => Err(format!("Discovery publish failed: {}", error)),
        }
    }

    pub async fn refresh_discovery(&self) -> Result<(), String> {
        match self
            .authenticated_post("/api/v2/system/discovery/refresh")
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => Ok(()),
            Ok(resp) => Err(format!("Discovery refresh rejected: {}", resp.status())),
            Err(error) => Err(format!("Discovery refresh failed: {}", error)),
        }
    }

    /// Send a message to a peer
    pub async fn send_message(&self, recipient: &str, content: &str) -> Result<(), String> {
        let body = serde_json::json!({
            "recipient": recipient,
            "content": content,
        });

        match self
            .authenticated_post("/api/v2/messages/send")
            .json(&body)
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => Ok(()),
            Ok(resp) => {
                let status = resp.status();
                let body_text = resp.text().await.unwrap_or_default();
                tracing::error!(target: "tui", "Send failed: {} — {}", status, body_text);
                Err(format!("Server error: {}", status))
            }
            Err(e) => {
                tracing::error!(target: "tui", "Send connection failed: {}", e);
                Err(format!("Connection failed: {}", e))
            }
        }
    }

    pub async fn get_messages(&self, peer_id: &str) -> Vec<DaemonMessage> {
        if peer_id.is_empty() {
            return Vec::new();
        }

        let url = format!("/api/v2/messages?peer_id={}&limit=50", peer_id,);

        match self.authenticated_get(&url).send().await {
            Ok(resp) if resp.status().is_success() => {
                match resp.json::<serde_json::Value>().await {
                    Ok(val) => {
                        let data = val.get("data").unwrap_or(&val);
                        let msgs = data.as_array().cloned().unwrap_or_else(|| {
                            data.get("messages")
                                .and_then(|v| v.as_array())
                                .cloned()
                                .unwrap_or_default()
                        });

                        let result: Vec<DaemonMessage> = msgs
                            .iter()
                            .filter_map(|m| {
                                let sender = m
                                    .get("sender")
                                    .or_else(|| m.get("sender_id"))
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("unknown")
                                    .to_string();

                                let content = m
                                    .get("content")
                                    .or_else(|| m.get("message"))
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string();

                                let timestamp = m
                                    .get("timestamp")
                                    .or_else(|| m.get("created_at"))
                                    .and_then(|v| v.as_u64())
                                    .unwrap_or(0);

                                Some(DaemonMessage {
                                    sender,
                                    content,
                                    timestamp,
                                    is_mine: false,
                                })
                            })
                            .collect();

                        tracing::debug!(
                            target: "tui",
                            "Loaded {} messages for peer {}",
                            result.len(),
                            peer_id
                        );
                        result
                    }
                    Err(e) => {
                        tracing::warn!(target: "tui", "Failed to parse messages: {}", e);
                        Vec::new()
                    }
                }
            }
            Ok(resp) => {
                tracing::warn!(
                    target: "tui",
                    "Messages endpoint returned {} for peer {}",
                    resp.status(),
                    peer_id
                );
                Vec::new()
            }
            Err(e) => {
                tracing::warn!(target: "tui", "Failed to reach messages endpoint: {}", e);
                Vec::new()
            }
        }
    }

    pub async fn get_ai_status_summary(&self) -> String {
        match self
            .http
            .get(format!("{}/api/v2/ai/status", self.base_url))
            .timeout(Duration::from_secs(2))
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                match resp.json::<serde_json::Value>().await {
                    Ok(val) => {
                        let data = val.get("data").unwrap_or(&val);
                        let active = data
                            .get("active")
                            .and_then(|v| v.as_bool())
                            .unwrap_or(false);
                        let layer = data
                            .get("active_layer")
                            .and_then(|v| v.as_str())
                            .unwrap_or("none");
                        let latency = data.get("latency_us").and_then(|v| v.as_u64());
                        if active {
                            match latency {
                                Some(us) => format!("{} Active ({}μs)", layer, us),
                                None => format!("{} Active", layer),
                            }
                        } else {
                            "Inactive".to_string()
                        }
                    }
                    Err(_) => "parse error".to_string(),
                }
            }
            _ => "not available".to_string(),
        }
    }
}
