//! TUI Daemon Client — HTTP client that talks to the GhostWire daemon
//!
//! Instead of holding Arc<Core> directly, the TUI connects to the daemon
//! via HTTP API. This means:
//! - No database lock conflicts (only the daemon owns the sled DB)
//! - Multiple clients can connect simultaneously
//! - The daemon stays alive when the TUI closes

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::theme;

/// Base URL for the daemon API
fn api_base(port: u16) -> String {
    format!("http://127.0.0.1:{}", port)
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
}

/// The daemon HTTP client
pub struct DaemonClient {
    http: Client,
    base_url: String,
    port: u16,
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
        }
    }

    /// Check if the daemon is reachable
    pub async fn health_check(&self) -> bool {
        self.http
            .get(format!("{}/health", self.base_url))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    /// Get my identity info
    pub async fn get_identity(&self) -> Option<DaemonIdentity> {
        // Try the API v2 identity endpoint
        match self
            .http
            .get(format!("{}/api/v2/identity", self.base_url))
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                // Parse the response — adapt to actual API format
                match resp.json::<serde_json::Value>().await {
                    Ok(val) => {
                        // Extract from API response wrapper
                        let data = val.get("data").unwrap_or(&val);
                        Some(DaemonIdentity {
                            id: data.get("id").and_then(|v| v.as_str()).unwrap_or("unknown").to_string(),
                            nickname: data
                                .get("nickname")
                                .and_then(|v| v.as_str())
                                .unwrap_or("unnamed")
                                .to_string(),
                            did: data.get("did").and_then(|v| v.as_str()).unwrap_or("").to_string(),
                            is_locked: data.get("is_locked").and_then(|v| v.as_bool()).unwrap_or(false),
                        })
                    }
                    Err(_) => None,
                }
            }
            _ => None,
        }
    }

    /// Get the list of contacts/peers
    pub async fn get_contacts(&self) -> Vec<DaemonContact> {
        // Try to get peers from the mesh
        match self
            .http
            .get(format!("{}/api/v2/peers", self.base_url))
            .send()
            .await
        {
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
                                    unread_count: 0, // TODO: wire to mailbox
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
                    Err(_) => Vec::new(),
                }
            }
            _ => Vec::new(),
        }
    }

    /// Get network status
    pub async fn get_network_status(&self) -> DaemonNetworkStatus {
        match self
            .http
            .get(format!("{}/api/v2/network/status", self.base_url))
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
                                .and_then(|v| v.as_u64())
                                .unwrap_or(0) as usize,
                            mode: data
                                .get("mode")
                                .and_then(|v| v.as_str())
                                .unwrap_or("Mesh P2P")
                                .to_string(),
                            encryption: data
                                .get("encryption")
                                .and_then(|v| v.as_str())
                                .unwrap_or("XChaCha20-Poly1305")
                                .to_string(),
                        }
                    }
                    Err(_) => DaemonNetworkStatus {
                        peer_count: 0,
                        mode: "Mesh P2P".to_string(),
                        encryption: "XChaCha20-Poly1305".to_string(),
                    },
                }
            }
            _ => DaemonNetworkStatus {
                peer_count: 0,
                mode: "Mesh P2P".to_string(),
                encryption: "XChaCha20-Poly1305".to_string(),
            },
        }
    }

    /// Send a message to a peer
    pub async fn send_message(&self, recipient: &str, content: &str) -> Result<(), String> {
        let body = serde_json::json!({
            "recipient": recipient,
            "content": content,
        });

        match self
            .http
            .post(format!("{}/api/v2/messages/send", self.base_url))
            .json(&body)
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => Ok(()),
            Ok(resp) => Err(format!("Server error: {}", resp.status())),
            Err(e) => Err(format!("Connection failed: {}", e)),
        }
    }

    /// Get message history with a peer
    pub async fn get_messages(&self, _peer_id: &str) -> Vec<DaemonMessage> {
        // TODO: wire to actual message history endpoint
        // For now, return empty — messages are shown optimistically in the TUI
        Vec::new()
    }
}
