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
use std::sync::Arc;
use tokio::net::TcpStream;
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, MaybeTlsStream, WebSocketStream};

use tracing::{debug, error, info, trace, warn};

use super::theme;

/// Imported for fingerprint computation
use base64::Engine;
use sha2::Digest;

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

/// Pending connection request from another peer
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerRequest {
    /// Unique request ID
    pub request_id: String,
    /// Sender's node ID
    pub from_node_id: String,
    /// Sender's nickname
    pub from_nickname: String,
    /// Optional personal message
    pub message: Option<String>,
    /// Unix timestamp
    pub timestamp: u64,
}

/// Message from the daemon
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DaemonMessage {
    pub sender: String,
    pub content: String,
    pub timestamp: u64,
    pub is_mine: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshAnalysis {
    pub optimal_relays: Vec<MeshRoutingDecision>,
    pub network_health: f32,
    pub anomalies: Vec<String>,
    pub recommendations: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshRoutingDecision {
    pub best_relay: String,
    pub confidence: f32,
    pub reasoning: String,
    pub alternative_relays: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAnalysis {
    pub threat_level: String,
    pub confidence: f32,
    pub threats: Vec<ThreatDetail>,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetail {
    pub threat_type: String,
    pub severity: f32,
    pub description: String,
    pub affected_nodes: Vec<String>,
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

/// Response from the peer ping API
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerPingResponse {
    pub node_id: String,
    pub latency_ms: u32,
    pub status: String,
}

/// Trust entry from the daemon's trust store.
/// Deserializes from the PeerRecord JSON shape returned by GET /api/v2/trust.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrustEntry {
    pub peer_id: String,
    /// Serialized TrustLevel: "Unknown", "FirstContact", "Trusted", "Vouched",
    /// "Verified", "KeyChanged", "Blocked"
    pub trust_level: String,
    #[serde(default)]
    pub interaction_count: u32,
    #[serde(default)]
    pub display_name: Option<String>,
    #[serde(default)]
    pub first_seen: u64,
    #[serde(default)]
    pub last_seen: u64,
}

/// Audit log entry from the daemon's security audit ring buffer.
/// Mirrors the backend's `AuditEntry` in `src/web.rs`.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub id: u64,
    /// RFC 3339 timestamp string
    pub timestamp: String,
    /// Event type string: "PeerConnected", "PeerDisconnected", "PeerVerified",
    /// "PeerBlocked", "AuthFailure", "SecurityLockdown", "SystemStartup"
    #[serde(default)]
    pub event_type: String,
    pub peer_id: Option<String>,
    pub detail: String,
}

impl TrustEntry {
    /// Compute a trust score (0.0–1.0) compatible with the backend's
    /// `PeerRecord::computed_trust_score()`. Uses trust_level as base and
    /// interaction count as a small bonus.
    pub fn trust_score(&self) -> f32 {
        let base = match self.trust_level.to_lowercase().as_str() {
            "unknown" => 0.0,
            "firstcontact" => 0.3,
            "trusted" => 0.6,
            "vouched" => 0.7,
            "verified" => 1.0,
            _ => 0.0,
        };
        let interaction_bonus = (self.interaction_count as f32 / 100.0).min(0.1);
        (base + interaction_bonus).min(1.0)
    }
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
    token: String,
    /// Connection health status tracking
    health: Arc<Mutex<HealthStatus>>,
}

/// WebSocket stream type for the daemon's /ws event bus
pub type WsStream = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Connection health status and retry tracking
#[derive(Debug, Clone)]
struct HealthStatus {
    last_attempt: std::time::Instant,
    consecutive_failures: usize,
    retry_delay_ms: u64,
    connected: bool,
    verified: bool,
}

impl HealthStatus {
    /// Record a failed health check and compute exponential backoff
    fn health_check_failed(&mut self) {
        self.consecutive_failures += 1;
        self.connected = false;

        // Exponential backoff: 2^n * base_delay, capped at 30 seconds
        let exponent = self.consecutive_failures.min(10); // Cap at ~10 retries
        self.retry_delay_ms = (2u64.pow(exponent as u32) * 250).min(30_000);

        warn!(
            "Health check failed {} consecutive times. Retry in {}ms. Daemon may not be running.",
            self.consecutive_failures,
            self.retry_delay_ms
        );
    }
}
/// Enhanced health check result with context
#[derive(Debug, Clone)]
pub enum HealthResult {
    /// Successfully connected to daemon
    Connected,
    /// Still waiting for retry cooldown
    WaitingRetry,
    /// Authentication failure (invalid token/bad credentials)
    AuthFailure(reqwest::StatusCode),
    /// Network failure (daemon not reachable)
    NetworkFailure(String),
}

impl DaemonClient {
    /// Create a new client connecting to the daemon on the given port
    pub fn new(port: u16) -> Self {
        let token = Self::load_token();

        // Initialize health tracking with exponential backoff starting at 250ms
        let health = Arc::new(Mutex::new(HealthStatus {
            last_attempt: std::time::Instant::now(),
            consecutive_failures: 0,
            retry_delay_ms: 250, // Start at 250ms
            connected: false,
            verified: false,
        }));

        Self {
            http: Client::builder()
                .timeout(Duration::from_secs(10))
                .build()
                .expect("failed to build HTTP client"),
            base_url: api_base(port),
            port,
            token,
            health,
        }
    }

    fn load_token() -> String {
        let config_dir = match dirs::config_dir() {
            Some(dir) => dir.join("ghostwire"),
            None => {
                warn!("Could not find config directory, using current directory");
                std::path::PathBuf::from(".")
            }
        };

        let token_file = config_dir.join("token");
        match std::fs::read_to_string(&token_file) {
            Ok(content) => {
                let trimmed = content.trim().to_string();
                if !trimmed.is_empty() {
                    debug!("Loaded token from: {:?}", token_file);
                    trimmed
                } else {
                    trace!("Token file exists but is empty at: {:?}", token_file);
                    String::new()
                }
            }
            Err(err) => {
                trace!("Could not read token file at {:?}: {}", token_file, err);
                String::new()
            }
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
            token: self.token.clone(),
            health: self.health.clone(),
        }
    }

    /// Connect to the daemon's WebSocket event bus at /ws?token=<AUTH_TOKEN>
    ///
    /// Returns a stream that yields CoreEvent JSON messages from the daemon.
    /// The daemon broadcasts events (NewMessage, PeerConnected, etc.) to all
    /// connected WebSocket clients in real time.
    pub async fn connect_websocket(&self) -> Result<WsStream, String> {
        let url = format!("ws://127.0.0.1:{}/ws?token={}", self.port, self.token);
        info!("Connecting WebSocket to daemon event bus on port {}", self.port);
        match connect_async(&url).await {
            Ok((ws_stream, _)) => {
                info!("WebSocket connected to daemon event bus on port {}", self.port);
                Ok(ws_stream)
            }
            Err(e) => {
                warn!("WebSocket connection failed on port {}: {}", self.port, e);
                Err(format!("WebSocket connection to daemon failed: {}", e))
            }
        }
    }

    /// Ping a specific peer to measure network latency.
    ///
    /// POST `/api/v2/peers/ping` with `{"node_id": "..."}`.
    /// Returns `Some(latency_ms)` on success, `None` on failure.
    pub async fn ping_peer(&self, node_id: &str) -> Option<u32> {
        let body = serde_json::json!({
            "node_id": node_id,
        });

        match self
            .http
            .post(format!("{}/api/v2/peers/ping", self.base_url))
            .header("X-Ghost-Token", &self.token)
            .json(&body)
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                match resp.json::<serde_json::Value>().await {
                    Ok(val) => {
                        let data = val.get("data").unwrap_or(&val);
                        data.get("latency_ms")
                            .and_then(|v| v.as_u64())
                            .map(|v| v as u32)
                    }
                    Err(_) => None,
                }
            }
            _ => None,
        }
    }

    /// Check if the daemon is reachable with enhanced error handling
    /// Distinguishes between "daemon not running" vs "network failure"
    pub async fn health_check(&self) -> HealthResult {
        let mut health = self.health.lock().await;

        let now = std::time::Instant::now();
        let elapsed = now.duration_since(health.last_attempt);

        // Only retry after delay period, unless it's the very first attempt
        if !health.connected && health.consecutive_failures > 0 && elapsed.as_millis() < health.retry_delay_ms as u128 {
            return HealthResult::WaitingRetry;
        }

        health.last_attempt = now;
        trace!("Health check: Attempt {} (retry_delay: {}ms)",
            health.consecutive_failures + 1,
            health.retry_delay_ms);

        match self.http
            .get(format!("{}/health", self.base_url))
            .header("X-Ghost-Token", &self.token)
            .send()
            .await
        {
            Ok(response) => {
                if response.status().is_success() {
                    health.connected = true;
                    health.consecutive_failures = 0;
                    health.retry_delay_ms = 250; // Reset to minimum delay
                    health.verified = true;
                    HealthResult::Connected
                } else {
                    health.health_check_failed();
                    HealthResult::AuthFailure(response.status())
                }
            },
            Err(err) => {
                trace!("Health check network error: {}", err);
                health.health_check_failed();
                HealthResult::NetworkFailure(err.to_string())
            }
        }
    }

    /// Get the current retry delay in milliseconds for UI feedback
    pub async fn retry_delay_ms(&self) -> u64 {
        self.health.lock().await.retry_delay_ms
    }

    /// Forget previous failures and reset connection state
    pub async fn reset_health_status(&self) {
        let mut health = self.health.lock().await;
        health.consecutive_failures = 0;
        health.retry_delay_ms = 250;
        health.connected = false;
        health.verified = false;
    }

    /// Shared helper for API v2 GET requests
    async fn get_api_v2<T: for<'de> Deserialize<'de>>(&self, url: &str) -> Option<T> {
        match self.http
            .get(url)
            .header("X-Ghost-Token", &self.token)
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                match resp.json::<serde_json::Value>().await {
                    Ok(val) => {
                        // GhostWire API V2 wraps everything in a "data" field
                        if let Some(data) = val.get("data") {
                            serde_json::from_value(data.clone()).ok()
                        } else {
                            serde_json::from_value(val).ok()
                        }
                    }
                    Err(_) => None,
                }
            }
            _ => None,
        }
    }

    /// Get my identity info
    pub async fn get_identity(&self) -> Option<DaemonIdentity> {
        // Try the API v2 identity endpoint
        match self
            .http
            .get(format!("{}/api/v2/identity", self.base_url))
            .header("X-Ghost-Token", &self.token)
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
            .header("X-Ghost-Token", &self.token)
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
                                        .or_else(|| p.get("node_id"))
                                        .or_else(|| p.get("id"))
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("")
                                        .to_string(),
                                    online: p
                                        .get("online")
                                        .or_else(|| p.get("is_online"))
                                        .or_else(|| p.get("connected"))
                                        .and_then(|v| v.as_bool())
                                        .unwrap_or(false),
                                    trust_score: p
                                        .get("trust_score")
                                        .and_then(|v| v.as_f64())
                                        .unwrap_or(0.5),
                                    unread_count: p
                                        .get("unread_count")
                                        .or_else(|| p.get("unread"))
                                        .or_else(|| p.get("messages_unread"))
                                        .and_then(|v| v.as_u64())
                                        .unwrap_or(0)
                                        as usize,
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
            .header("X-Ghost-Token", &self.token)
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
            .header("X-Ghost-Token", &self.token)
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
    pub async fn get_messages(&self, peer_id: &str) -> Vec<DaemonMessage> {
        let url = format!(
            "{}/api/v2/messages?peer_id={}&limit=100",
            self.base_url, peer_id
        );

        match self
            .http
            .get(&url)
            .header("X-Ghost-Token", &self.token)
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                match resp.json::<serde_json::Value>().await {
                    Ok(val) => {
                        let data = val.get("data").unwrap_or(&val);
                        if let Some(messages) = data.as_array() {
                            messages
                                .iter()
                                .map(|m| DaemonMessage {
                                    sender: m
                                        .get("sender")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("")
                                        .to_string(),
                                    content: m
                                        .get("content")
                                        .and_then(|v| v.as_str())
                                        .unwrap_or("")
                                        .to_string(),
                                    timestamp: m.get("timestamp").and_then(|v| v.as_u64()).unwrap_or(0),
                                    is_mine: m.get("is_mine").and_then(|v| v.as_bool()).unwrap_or(false),
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

    pub async fn get_ai_status(&self) -> String {
        let url = format!("{}/api/v2/ai/status", self.base_url);
        match self.get_api_v2::<serde_json::Value>(&url).await {
            Some(res) => {
                let healthy = res.get("gemma_client")
                    .and_then(|g| g.get("healthy"))
                    .and_then(|h| h.as_bool())
                    .unwrap_or(false);
                if healthy {
                    "ONLINE (Gemma 4)".to_string()
                } else {
                    "OFFLINE (Check Ollama)".to_string()
                }
            }
            None => "UNREACHABLE".to_string(),
        }
    }

    pub async fn get_mesh_analysis(&self) -> Option<MeshAnalysis> {
        let url = format!("{}/api/v2/ai/mesh-analysis", self.base_url);
        self.get_api_v2::<MeshAnalysis>(&url).await
    }

    pub async fn get_threat_analysis(&self) -> Option<ThreatAnalysis> {
        let url = format!("{}/api/v2/ai/threat-analysis", self.base_url);
        self.get_api_v2::<ThreatAnalysis>(&url).await
    }

    /// Send a connection request to another peer.
    /// Returns true on success, false on failure.
    pub async fn connect_peer(&self, node_id: &str) -> bool {
        let url = format!("{}/api/v2/peers/connect", self.base_url);
        let body = serde_json::json!({
            "node_id": node_id,
        });
        match self
            .http
            .post(&url)
            .header("X-Ghost-Token", &self.token)
            .json(&body)
            .send()
            .await
        {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    /// Disconnect from a connected peer by node_id.
    /// DELETE /api/v2/peers/:node_id
    /// Returns true on success, false on failure.
    pub async fn disconnect_peer(&self, node_id: &str) -> bool {
        let url = format!("{}/api/v2/peers/{}", self.base_url, node_id);
        match self
            .http
            .delete(&url)
            .header("X-Ghost-Token", &self.token)
            .send()
            .await
        {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    /// Fetch pending inbound connection requests.
    /// Maps the API's `ConnectRequest` type into our local `PeerRequest`.
    pub async fn get_peer_requests(&self) -> Vec<PeerRequest> {
        let url = format!("{}/api/v2/peers/requests", self.base_url);
        match self.get_api_v2::<serde_json::Value>(&url).await {
            Some(val) => {
                if let Some(requests) = val.get("requests").and_then(|r| r.as_array()) {
                    requests
                        .iter()
                        .filter_map(|r| {
                            Some(PeerRequest {
                                request_id: r.get("request_id")?.as_str()?.to_string(),
                                from_node_id: r.get("from_node_id")?.as_str()?.to_string(),
                                from_nickname: r
                                    .get("from_nickname")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("")
                                    .to_string(),
                                message: r
                                    .get("message")
                                    .and_then(|v| v.as_str())
                                    .map(|s| s.to_string()),
                                timestamp: r
                                    .get("timestamp")
                                    .and_then(|v| v.as_u64())
                                    .unwrap_or(0),
                            })
                        })
                        .collect()
                } else {
                    Vec::new()
                }
            }
            None => Vec::new(),
        }
    }

    /// Accept or deny a pending connection request.
    /// `request_id` is the ID of the request to respond to.
    /// `accept` true = accept, false = deny/decline.
    /// Returns true on success.
    pub async fn respond_to_request(&self, request_id: &str, accept: bool) -> bool {
        let url = format!("{}/api/v2/peers/requests/respond", self.base_url);
        let body = serde_json::json!({
            "request_id": request_id,
            "accept": accept,
        });
        match self
            .http
            .post(&url)
            .header("X-Ghost-Token", &self.token)
            .json(&body)
            .send()
            .await
        {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    /// Fetch the trust list for all known peers.
    /// GET /api/v2/trust
    /// Returns a Vec of TrustEntry with real trust_level values from the backend.
    pub async fn get_trust_list(&self) -> Vec<TrustEntry> {
        let url = format!("{}/api/v2/trust", self.base_url);
        self.get_api_v2::<Vec<TrustEntry>>(&url).await.unwrap_or_default()
    }

    /// Verify a peer's public key.
    /// POST /api/v2/trust/:peer_id/verify
    /// `public_key_hex` must be a hex-encoded Ed25519 public key (32 bytes = 64 hex chars).
    pub async fn verify_peer(&self, peer_id: &str, public_key_hex: &str) -> bool {
        let url = format!("{}/api/v2/trust/{}/verify", self.base_url, peer_id);
        let body = serde_json::json!({ "public_key": public_key_hex });
        match self
            .http
            .post(&url)
            .header("X-Ghost-Token", &self.token)
            .json(&body)
            .send()
            .await
        {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    /// Block a peer via the trust system (different from disconnect — this
    /// prevents routing through / to this peer).
    /// POST /api/v2/trust/:peer_id/block
    pub async fn block_peer_trust(&self, peer_id: &str) -> bool {
        let url = format!("{}/api/v2/trust/{}/block", self.base_url, peer_id);
        match self
            .http
            .post(&url)
            .header("X-Ghost-Token", &self.token)
            .send()
            .await
        {
            Ok(resp) => resp.status().is_success(),
            Err(_) => false,
        }
    }

    /// Fetch the audit log from the daemon's security audit ring buffer.
    /// GET /api/v2/system/audit
    /// Returns Vec of AuditEntry, newest first.
    pub async fn get_audit_log(&self) -> Vec<AuditEntry> {
        let url = format!("{}/api/v2/system/audit", self.base_url);
        match self.http
            .get(&url)
            .header("X-Ghost-Token", &self.token)
            .send()
            .await
        {
            Ok(resp) if resp.status().is_success() => {
                match resp.json::<serde_json::Value>().await {
                    Ok(val) => {
                        let data = val.get("data").unwrap_or(&val);
                        if let Some(entries) = data.get("entries").and_then(|e| e.as_array()) {
                            entries
                                .iter()
                                .filter_map(|e| {
                                    Some(AuditEntry {
                                        id: e.get("id")?.as_u64()?,
                                        timestamp: e.get("timestamp")?.as_str()?.to_string(),
                                        event_type: e
                                            .get("event_type")
                                            .and_then(|v| v.as_str())
                                            .unwrap_or("Unknown")
                                            .to_string(),
                                        peer_id: e.get("peer_id").and_then(|v| v.as_str()).map(|s| s.to_string()),
                                        detail: e.get("detail")?.as_str()?.to_string(),
                                    })
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

    /// Fetch the public key and key_id from the daemon.
    /// Returns `(key_id, public_key_b64)` on success.
    pub async fn get_public_key(&self) -> Option<(String, String)> {
        let url = format!("{}/api/v2/profile/public-key", self.base_url);
        match self.get_api_v2::<serde_json::Value>(&url).await {
            Some(val) => {
                let key_id = val
                    .get("key_id")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                let public_key = val
                    .get("public_key")
                    .and_then(|v| v.as_str())
                    .unwrap_or("")
                    .to_string();
                if !key_id.is_empty() && !public_key.is_empty() {
                    Some((key_id, public_key))
                } else {
                    None
                }
            }
            None => None,
        }
    }
}

/// Compute a human-readable fingerprint from a base64-encoded public key.
///
/// Decodes the base64 value, SHA-256 hashes the raw bytes, takes the first 8
/// bytes, and formats them as colon-separated uppercase hex pairs.
///
/// Example: `AA:BB:CC:DD:EE:FF:00:11`
pub fn fingerprint_from_b64(b64: &str) -> Option<String> {
    let raw = base64::engine::general_purpose::STANDARD.decode(b64).ok()?;
    let hash = sha2::Sha256::digest(&raw);
    let pairs: Vec<String> = hash[..8].iter().map(|b| format!("{:02X}", b)).collect();
    Some(pairs.join(":"))
}
