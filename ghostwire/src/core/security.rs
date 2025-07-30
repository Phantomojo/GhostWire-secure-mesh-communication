//! Security module: Sybil defense, quotas, blacklists.
//! Next: integrate with peer discovery, store, and reputation.

use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

/// Threat level enumeration
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Entity type for security events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    Peer,
    IP,
    Node,
    Unknown,
}

/// Security event structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityEvent {
    pub event_type: String,
    pub entity_type: EntityType,
    pub entity_id: String,
    pub threat_level: ThreatLevel,
    pub timestamp: u64,
    pub description: String,
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub max_connections_per_ip: usize,
    pub max_messages_per_minute: usize,
    pub blacklist_enabled: bool,
    pub threat_detection_enabled: bool,
    pub allowed_ips: Vec<String>,
    pub blocked_ips: Vec<String>,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            max_connections_per_ip: 10,
            max_messages_per_minute: 100,
            blacklist_enabled: true,
            threat_detection_enabled: true,
            allowed_ips: vec![],
            blocked_ips: vec![],
        }
    }
}

/// Security statistics
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityStats {
    pub total_events: u64,
    pub blocked_connections: u64,
    pub threat_level: ThreatLevel,
    pub last_scan: u64,
    pub active_threats: usize,
}

impl Default for SecurityStats {
    fn default() -> Self {
        Self {
            total_events: 0,
            blocked_connections: 0,
            threat_level: ThreatLevel::Low,
            last_scan: 0,
            active_threats: 0,
        }
    }
}

/// Main security manager
pub struct SecurityManager {
    config: SecurityConfig,
    blacklist: Arc<RwLock<HashMap<String, u64>>>, // IP -> timestamp
    connection_counts: Arc<RwLock<HashMap<IpAddr, usize>>>,
    message_counts: Arc<RwLock<HashMap<String, usize>>>,
    events: Arc<RwLock<Vec<SecurityEvent>>>,
    stats: Arc<RwLock<SecurityStats>>,
}

impl SecurityManager {
    pub fn new(config: SecurityConfig) -> Self {
        Self {
            config,
            blacklist: Arc::new(RwLock::new(HashMap::new())),
            connection_counts: Arc::new(RwLock::new(HashMap::new())),
            message_counts: Arc::new(RwLock::new(HashMap::new())),
            events: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(SecurityStats::default())),
        }
    }

    /// Check if an IP is allowed
    pub async fn is_ip_allowed(&self, ip: &IpAddr) -> bool {
        let ip_str = ip.to_string();
        
        // Check explicit allowlist
        if !self.config.allowed_ips.is_empty() && !self.config.allowed_ips.contains(&ip_str) {
            return false;
        }
        
        // Check explicit blocklist
        if self.config.blocked_ips.contains(&ip_str) {
            return false;
        }
        
        // Check dynamic blacklist
        let blacklist = self.blacklist.read().await;
        if let Some(timestamp) = blacklist.get(&ip_str) {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            if now - timestamp < 3600 { // 1 hour blacklist
                return false;
            }
        }
        
        true
    }

    /// Record a connection attempt
    pub async fn record_connection(&self, ip: IpAddr) -> bool {
        if !self.is_ip_allowed(&ip).await {
            return false;
        }
        
        let mut counts = self.connection_counts.write().await;
        let count = counts.entry(ip).or_insert(0);
        *count += 1;
        
        if *count > self.config.max_connections_per_ip {
            self.block_ip(&ip.to_string()).await;
            return false;
        }
        
        true
    }

    /// Record a message
    pub async fn record_message(&self, peer_id: &str) -> bool {
        let mut counts = self.message_counts.write().await;
        let count = counts.entry(peer_id.to_string()).or_insert(0);
        *count += 1;
        
        if *count > self.config.max_messages_per_minute {
            return false;
        }
        
        true
    }

    /// Record a connection attempt
    pub async fn record_connection_attempt(&self, ip: IpAddr, success: bool, reason: String) {
        let event = SecurityEvent {
            event_type: if success { "connection_success".to_string() } else { "connection_failed".to_string() },
            entity_type: EntityType::IP,
            entity_id: ip.to_string(),
            threat_level: if success { ThreatLevel::Low } else { ThreatLevel::Medium },
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            description: reason,
        };
        
        self.add_event(event).await;
        
        if !success {
            // Increment failed connection count
            let mut stats = self.stats.write().await;
            stats.blocked_connections += 1;
        }
    }

    /// Block an IP address
    pub async fn block_ip(&self, ip: &str) {
        let mut blacklist = self.blacklist.write().await;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        blacklist.insert(ip.to_string(), now);
        
        // Update stats
        let mut stats = self.stats.write().await;
        stats.blocked_connections += 1;
    }

    /// Add a security event
    pub async fn add_event(&self, event: SecurityEvent) {
        let mut events = self.events.write().await;
        events.push(event.clone());
        
        // Update stats
        let mut stats = self.stats.write().await;
        stats.total_events += 1;
        if event.threat_level > stats.threat_level {
            stats.threat_level = event.threat_level;
        }
    }

    /// Get security statistics
    pub fn get_security_stats(&self) -> SecurityStats {
        // In a real implementation, this would be async and read from the Arc<RwLock>
        SecurityStats::default()
    }

    /// Clean up old entries
    pub async fn cleanup(&self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Clean blacklist
        let mut blacklist = self.blacklist.write().await;
        blacklist.retain(|_, timestamp| now - *timestamp < 3600);
        
        // Clean connection counts
        let mut connection_counts = self.connection_counts.write().await;
        connection_counts.clear();
        
        // Clean message counts
        let mut message_counts = self.message_counts.write().await;
        message_counts.clear();
        
        // Update last scan
        let mut stats = self.stats.write().await;
        stats.last_scan = now;
    }
}

/// Trait for Sybil attack detection and mitigation.
pub trait SybilDefense {
    /// Check if a new peer is allowed (e.g., PoW, cap).
    fn allow_new_peer(&self, peer_id: &str) -> bool;
}

/// Default stub: always allow.
pub struct AllowAllSybilDefense;
impl SybilDefense for AllowAllSybilDefense {
    fn allow_new_peer(&self, _peer_id: &str) -> bool { true }
    }
    
/// Trait for enforcing per-peer and global quotas.
pub trait QuotaEnforcer {
    /// Check if a peer is within quota.
    fn check_quota(&self, peer_id: &str) -> bool;
}

/// Default stub: always allow.
pub struct AllowAllQuotaEnforcer;
impl QuotaEnforcer for AllowAllQuotaEnforcer {
    fn check_quota(&self, _peer_id: &str) -> bool { true }
    }
    
/// Trait for managing local blacklists.
pub trait BlacklistManager {
    /// Check if a peer is blacklisted.
    fn is_blacklisted(&self, peer_id: &str) -> bool;
    /// Add a peer to the blacklist.
    fn add(&mut self, peer_id: &str);
    }
    
/// Default stub: never blacklists.
pub struct NoBlacklistManager;
impl BlacklistManager for NoBlacklistManager {
    fn is_blacklisted(&self, _peer_id: &str) -> bool { false }
    fn add(&mut self, _peer_id: &str) {}
} 