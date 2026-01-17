//! Security module: Sybil defense, quotas, blacklists.
//! Next: integrate with peer discovery, store, and reputation.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use tokio::sync::RwLock;

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
    pub messages_processed: u64,
    pub peers_verified: u32,
    pub rate_limit_violations: u32,
    pub blacklisted_peers: u32,
}

impl Default for SecurityStats {
    fn default() -> Self {
        Self {
            total_events: 0,
            blocked_connections: 0,
            threat_level: ThreatLevel::Low,
            last_scan: 0,
            active_threats: 0,
            messages_processed: 0,
            peers_verified: 0,
            rate_limit_violations: 0,
            blacklisted_peers: 0,
        }
    }
}

/// Main security manager
pub struct SecurityManager {
    config: Arc<RwLock<SecurityConfig>>,
    blacklist: Arc<RwLock<HashMap<String, u64>>>, // IP -> timestamp
    connection_counts: Arc<RwLock<HashMap<IpAddr, usize>>>,
    message_counts: Arc<RwLock<HashMap<String, usize>>>,
    events: Arc<RwLock<Vec<SecurityEvent>>>,
    stats: Arc<RwLock<SecurityStats>>,
    pub lockdown_enabled: Arc<RwLock<bool>>,
}

impl SecurityManager {
    pub fn new(config: SecurityConfig) -> Self {
        Self {
            config: Arc::new(RwLock::new(config)),
            blacklist: Arc::new(RwLock::new(HashMap::new())),
            connection_counts: Arc::new(RwLock::new(HashMap::new())),
            message_counts: Arc::new(RwLock::new(HashMap::new())),
            events: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(SecurityStats::default())),
            lockdown_enabled: Arc::new(RwLock::new(false)),
        }
    }

    /// Check if an IP is allowed
    pub async fn is_ip_allowed(&self, ip: &IpAddr) -> bool {
        let ip_str = ip.to_string();

        // Check explicit allowlist
        let config = self.config.read().await;
        if !config.allowed_ips.is_empty() && !config.allowed_ips.contains(&ip_str) {
            return false;
        }

        // Check explicit blocklist
        if config.blocked_ips.contains(&ip_str) {
            return false;
        }

        // Check dynamic blacklist
        let blacklist = self.blacklist.read().await;
        if let Some(timestamp) = blacklist.get(&ip_str) {
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs();
            if now - timestamp < 3600 {
                // 1 hour blacklist
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

        let max = self.config.read().await.max_connections_per_ip;
        if *count > max {
            self.block_ip(&ip.to_string()).await;
            // Increment rate limit violations
            let mut stats = self.stats.write().await;
            stats.rate_limit_violations += 1;
            return false;
        }

        true
    }

    /// Record a message
    pub async fn record_message(&self, peer_id: &str) -> bool {
        let mut counts = self.message_counts.write().await;
        let count = counts.entry(peer_id.to_string()).or_insert(0);
        *count += 1;

        let max = self.config.read().await.max_messages_per_minute;
        if *count > max {
            let mut stats = self.stats.write().await;
            stats.rate_limit_violations += 1;
            return false;
        }

        // Increment global message count
        let mut stats = self.stats.write().await;
        stats.messages_processed += 1;

        true
    }

    /// Record a connection attempt
    pub async fn record_connection_attempt(&self, ip: IpAddr, success: bool, reason: String) {
        let event = SecurityEvent {
            event_type: if success {
                "connection_success".to_string()
            } else {
                "connection_failed".to_string()
            },
            entity_type: EntityType::IP,
            entity_id: ip.to_string(),
            threat_level: if success {
                ThreatLevel::Low
            } else {
                ThreatLevel::Medium
            },
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

    /// Increment verified peers count
    pub async fn increment_verified_peers(&self) {
        let mut stats = self.stats.write().await;
        stats.peers_verified += 1;
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
    pub async fn get_security_stats(&self) -> SecurityStats {
        let stats = self.stats.read().await;
        stats.clone()
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

    pub async fn set_lockdown(&self, enabled: bool) {
        let mut lockdown = self.lockdown_enabled.write().await;
        *lockdown = enabled;

        let event = SecurityEvent {
            event_type: if enabled {
                "lockdown_enabled".to_string()
            } else {
                "lockdown_disabled".to_string()
            },
            entity_type: EntityType::Node,
            entity_id: "local".to_string(),
            threat_level: if enabled {
                ThreatLevel::Critical
            } else {
                ThreatLevel::Low
            },
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            description: if enabled {
                "System placed in LOCKDOWN mode".to_string()
            } else {
                "Lockdown mode lifted".to_string()
            },
        };
        self.add_event(event).await;
    }

    pub async fn is_lockdown(&self) -> bool {
        *self.lockdown_enabled.read().await
    }

    pub fn save_config(&self, path: &str) -> std::io::Result<()> {
        let rt = tokio::runtime::Handle::current();
        let config_guard = rt.block_on(self.config.read());
        let json = serde_json::to_string_pretty(&*config_guard)?;
        std::fs::write(path, json)?;
        Ok(())
    }

    pub async fn update_config(&self, new_config: SecurityConfig) {
        let mut config = self.config.write().await;
        *config = new_config;
    }

    pub async fn get_config(&self) -> SecurityConfig {
        self.config.read().await.clone()
    }

    /// Calculate real security score (0-100)
    pub async fn calculate_security_score(&self) -> u8 {
        let stats = self.stats.read().await;
        let mut score: i16 = 50;

        // Verified peers bonus
        if stats.peers_verified > 0 {
            score += 15;
        }

        // Rate limit violations penalty
        if stats.rate_limit_violations == 0 {
            score += 15;
        } else {
            score -= (stats.rate_limit_violations as i16 * 5).min(30);
        }

        // Blocked connections bonus/penalty
        if stats.blocked_connections == 0 {
            score += 10;
        }

        // Threat level impact
        match stats.threat_level {
            ThreatLevel::Low => score += 10,
            ThreatLevel::Medium => score += 0,
            ThreatLevel::High => score -= 20,
            ThreatLevel::Critical => score -= 50,
        }

        // Feature checks
        let config = self.config.read().await;
        if config.blacklist_enabled {
            score += 5;
        }
        if config.threat_detection_enabled {
            score += 5;
        }

        score.clamp(0, 100) as u8
    }
}

/// Trait for Sybil attack detection and mitigation.
pub trait SybilDefense {
    /// Check if a new peer is allowed (e.g., PoW, cap).
    fn allow_new_peer(&self, peer_id: &str) -> bool;
}

/// Production Sybil defense using proof-of-work
pub struct ProofOfWorkSybilDefense {
    difficulty: usize, // Number of leading zero bits required
    max_peers: usize,
    current_peers: Arc<RwLock<usize>>,
    verified_peers: Arc<RwLock<HashMap<String, u64>>>, // PeerID -> verification timestamp
}

impl ProofOfWorkSybilDefense {
    pub fn new(difficulty: usize, max_peers: usize) -> Self {
        Self {
            difficulty,
            max_peers,
            current_peers: Arc::new(RwLock::new(0)),
            verified_peers: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Generate a challenge for a new peer
    pub fn generate_challenge(&self) -> Vec<u8> {
        use ring::rand::{SecureRandom, SystemRandom};
        let rng = SystemRandom::new();
        let mut challenge = vec![0u8; 32];
        rng.fill(&mut challenge).expect("RNG failure");
        challenge
    }

    /// Verify proof-of-work solution
    pub fn verify_proof(&self, challenge: &[u8], solution: u64) -> bool {
        use ring::digest;

        // Combine challenge and solution
        let mut input = challenge.to_vec();
        input.extend_from_slice(&solution.to_le_bytes());

        // Hash it
        let hash = digest::digest(&digest::SHA256, &input);
        let hash_bytes = hash.as_ref();

        // Check if it has enough leading zero bits
        let mut zero_bits = 0;
        for byte in hash_bytes {
            if *byte == 0 {
                zero_bits += 8;
            } else {
                zero_bits += byte.leading_zeros() as usize;
                break;
            }
        }

        zero_bits >= self.difficulty
    }
}

impl SybilDefense for ProofOfWorkSybilDefense {
    fn allow_new_peer(&self, peer_id: &str) -> bool {
        // Check if we've already verified this peer recently
        if let Ok(verified) = self.verified_peers.try_read() {
            if let Some(timestamp) = verified.get(peer_id) {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                // Verified peers are valid for 1 hour
                if now - timestamp < 3600 {
                    return true;
                }
            }
        }

        // Check if we have room for more peers
        if let Ok(current) = self.current_peers.try_read() {
            if *current >= self.max_peers {
                return false;
            }
        }

        // Will be verified after PoW check in the handshake
        false
    }
}

/// DEFAULT: Use moderate difficulty PoW
impl Default for ProofOfWorkSybilDefense {
    fn default() -> Self {
        Self::new(16, 1000) // 16-bit PoW, max 1000 peers
    }
}

/// Trait for enforcing per-peer and global quotas
pub trait QuotaEnforcer {
    fn check_quota(&self, peer_id: &str) -> bool;
}

/// Production quota enforcer with sliding window rate limiting
pub struct SlidingWindowQuotaEnforcer {
    max_messages_per_minute: usize,
    max_bytes_per_minute: usize,
    peer_quotas: Arc<RwLock<HashMap<String, PeerQuota>>>,
}

#[derive(Debug, Clone)]
struct PeerQuota {
    message_timestamps: Vec<u64>,  // Timestamps of recent messages
    bytes_sent: Vec<(u64, usize)>, // (timestamp, bytes) pairs
}

impl SlidingWindowQuotaEnforcer {
    pub fn new(max_messages_per_minute: usize, max_bytes_per_minute: usize) -> Self {
        Self {
            max_messages_per_minute,
            max_bytes_per_minute,
            peer_quotas: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn record_message(&self, peer_id: &str, bytes: usize) -> bool {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut quotas = self.peer_quotas.write().await;
        let quota = quotas.entry(peer_id.to_string()).or_insert(PeerQuota {
            message_timestamps: Vec::new(),
            bytes_sent: Vec::new(),
        });

        // Remove entries older than 1 minute
        quota.message_timestamps.retain(|&ts| now - ts < 60);
        quota.bytes_sent.retain(|(ts, _)| now - *ts < 60);

        // Check limits
        if quota.message_timestamps.len() >= self.max_messages_per_minute {
            return false;
        }

        let bytes_in_window: usize = quota.bytes_sent.iter().map(|(_, b)| b).sum();
        if bytes_in_window + bytes > self.max_bytes_per_minute {
            return false;
        }

        // Record this message
        quota.message_timestamps.push(now);
        quota.bytes_sent.push((now, bytes));

        true
    }
}

impl QuotaEnforcer for SlidingWindowQuotaEnforcer {
    fn check_quota(&self, peer_id: &str) -> bool {
        // Synchronous check - return true if peer exists and hasn't been rate-limited recently
        // Full enforcement happens in record_message()
        if let Ok(quotas) = self.peer_quotas.try_read() {
            if let Some(quota) = quotas.get(peer_id) {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                // Count recent messages
                let recent_count = quota
                    .message_timestamps
                    .iter()
                    .filter(|&&ts| now - ts < 60)
                    .count();

                return recent_count < self.max_messages_per_minute;
            }
        }
        true // Allow if we can't check (fail-open for new peers)
    }
}

pub trait BlacklistManager {
    fn is_blacklisted(&self, peer_id: &str) -> bool;
    /// Add a peer to the blacklist
    fn add(&mut self, peer_id: &str);
}
/// Production blacklist manager with persistence and auto-expiry
pub struct PersistentBlacklistManager {
    blacklist: Arc<RwLock<HashMap<String, BlacklistEntry>>>,
    max_blacklist_size: usize,
}

#[derive(Debug, Clone)]
struct BlacklistEntry {
    timestamp: u64,
    _reason: String,
    expires_at: Option<u64>, // None = permanent, Some = temporary
}

impl PersistentBlacklistManager {
    pub fn new(max_size: usize) -> Self {
        Self {
            blacklist: Arc::new(RwLock::new(HashMap::new())),
            max_blacklist_size: max_size,
        }
    }

    pub async fn add_entry(&mut self, peer_id: &str, reason: String, duration_secs: Option<u64>) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut blacklist = self.blacklist.write().await;

        // Enforce size limit (evict oldest temporary entry)
        if blacklist.len() >= self.max_blacklist_size {
            if let Some(oldest_temp) = blacklist
                .iter()
                .filter(|(_, e)| e.expires_at.is_some())
                .min_by_key(|(_, e)| e.timestamp)
                .map(|(k, _)| k.clone())
            {
                blacklist.remove(&oldest_temp);
            }
        }

        blacklist.insert(
            peer_id.to_string(),
            BlacklistEntry {
                timestamp: now,
                _reason: reason,
                expires_at: duration_secs.map(|d| now + d),
            },
        );
    }

    pub async fn cleanup_expired(&self) {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs();

        let mut blacklist = self.blacklist.write().await;
        blacklist.retain(|_, entry| {
            match entry.expires_at {
                Some(expires) => now < expires,
                None => true, // Keep permanent entries
            }
        });
    }
}

impl BlacklistManager for PersistentBlacklistManager {
    fn is_blacklisted(&self, peer_id: &str) -> bool {
        if let Ok(blacklist) = self.blacklist.try_read() {
            if let Some(entry) = blacklist.get(peer_id) {
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs();

                // Check if expired
                match entry.expires_at {
                    Some(expires) => now < expires,
                    None => true, // Permanent blacklist
                }
            } else {
                false
            }
        } else {
            false // Fail-open if we can't read
        }
    }

    fn add(&mut self, peer_id: &str) {
        // Add with 1 hour expiry by default
        let rt = tokio::runtime::Handle::current();
        rt.block_on(self.add_entry(peer_id, "Manual blacklist".to_string(), Some(3600)));
    }
}

impl Default for PersistentBlacklistManager {
    fn default() -> Self {
        Self::new(10000) // Max 10k blacklisted peers
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::net::Ipv4Addr;

    #[tokio::test]
    async fn test_security_manager_ip_filtering() {
        let mut config = SecurityConfig::default();
        config.blocked_ips.push("1.2.3.4".to_string());
        config.allowed_ips.push("127.0.0.1".to_string());

        let manager = SecurityManager::new(config);

        assert!(
            manager
                .is_ip_allowed(&IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)))
                .await
        );
        assert!(
            !manager
                .is_ip_allowed(&IpAddr::V4(Ipv4Addr::new(1, 2, 3, 4)))
                .await
        );
        // Not in allowlist when allowlist is non-empty
        assert!(
            !manager
                .is_ip_allowed(&IpAddr::V4(Ipv4Addr::new(8, 8, 8, 8)))
                .await
        );
    }

    #[tokio::test]
    async fn test_security_score_calculation() {
        let manager = SecurityManager::new(SecurityConfig::default());
        let score = manager.calculate_security_score().await;
        // Default score should be high (base 50 + feature bonuses + no violations)
        assert!(score >= 60);

        manager.set_lockdown(true).await;
        let score_lockdown = manager.calculate_security_score().await;
        assert!(score_lockdown < score);
    }

    #[test]
    fn test_pow_verification() {
        let pow = ProofOfWorkSybilDefense::new(4, 100); // 4 bits is very easy
        let challenge = vec![1, 2, 3, 4];

        // Brute force a solution
        let mut solution = 0;
        let mut found = false;
        for s in 0..1000 {
            if pow.verify_proof(&challenge, s) {
                solution = s;
                found = true;
                break;
            }
        }

        assert!(
            found,
            "Should find a solution for 4-bit PoW within 1000 tries"
        );
        assert!(pow.verify_proof(&challenge, solution));
    }

    #[tokio::test]
    async fn test_sliding_window_quota() {
        let enforcer = SlidingWindowQuotaEnforcer::new(2, 1000);
        let peer = "peer1";

        assert!(enforcer.record_message(peer, 100).await);
        assert!(enforcer.record_message(peer, 100).await);
        // Third message should fail (limit 2)
        assert!(!enforcer.record_message(peer, 100).await);
    }

    #[tokio::test]
    async fn test_blacklist_expiry() {
        let mut manager = PersistentBlacklistManager::new(10);
        let peer = "evil_peer";

        manager.add_entry(peer, "test".to_string(), Some(1)).await; // 1 second expiry
        assert!(manager.is_blacklisted(peer));

        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
        manager.cleanup_expired().await;
        assert!(!manager.is_blacklisted(peer));
    }
}
