use std::collections::{HashMap, HashSet};
use std::net::{IpAddr, Ipv4Addr, Ipv6Addr};
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use serde::{Deserialize, Serialize};
use tokio::time::interval;
use tracing::{info, warn, error, debug};

/// Security threat levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThreatLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// Types of security events
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum SecurityEvent {
    ConnectionAttempt {
        ip: IpAddr,
        timestamp: u64,
        success: bool,
        reason: String,
    },
    RateLimitExceeded {
        ip: IpAddr,
        timestamp: u64,
        attempts: usize,
    },
    InvalidHandshake {
        ip: IpAddr,
        timestamp: u64,
        bytes_received: Vec<u8>,
    },
    SuspiciousActivity {
        ip: IpAddr,
        timestamp: u64,
        activity_type: String,
        details: String,
    },
    KeyCompromise {
        key_id: String,
        timestamp: u64,
        reason: String,
    },
    PeerBlacklisted {
        peer_id: String,
        timestamp: u64,
        reason: String,
    },
}

/// Security configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    // Network security
    pub max_connections_per_ip: usize,
    pub connection_timeout: Duration,
    pub handshake_timeout: Duration,
    pub max_failed_attempts: usize,
    pub blacklist_duration: Duration,
    
    // Rate limiting
    pub rate_limit_window: Duration,
    pub max_requests_per_window: usize,
    
    // IP filtering
    pub allowed_networks: Vec<NetworkRange>,
    pub blocked_networks: Vec<NetworkRange>,
    pub allow_private_networks: bool,
    pub allow_loopback: bool,
    
    // Key management
    pub key_rotation_interval: Duration,
    pub max_key_age: Duration,
    pub require_key_verification: bool,
    
    // Threat detection
    pub enable_threat_detection: bool,
    pub threat_threshold: usize,
    pub auto_blacklist: bool,
    
    // Logging and monitoring
    pub log_security_events: bool,
    pub log_level: String,
    pub audit_trail_retention: Duration,
}

impl Default for SecurityConfig {
    fn default() -> Self {
        Self {
            max_connections_per_ip: 10,
            connection_timeout: Duration::from_secs(30),
            handshake_timeout: Duration::from_secs(5),
            max_failed_attempts: 5,
            blacklist_duration: Duration::from_secs(3600), // 1 hour
            
            rate_limit_window: Duration::from_secs(60),
            max_requests_per_window: 100,
            
            allowed_networks: vec![
                NetworkRange::new(IpAddr::V4(Ipv4Addr::new(10, 0, 0, 0)), 8),
                NetworkRange::new(IpAddr::V4(Ipv4Addr::new(172, 16, 0, 0)), 12),
                NetworkRange::new(IpAddr::V4(Ipv4Addr::new(192, 168, 0, 0)), 16),
            ],
            blocked_networks: vec![],
            allow_private_networks: true,
            allow_loopback: false,
            
            key_rotation_interval: Duration::from_secs(86400 * 7), // 7 days
            max_key_age: Duration::from_secs(86400 * 30), // 30 days
            require_key_verification: true,
            
            enable_threat_detection: true,
            threat_threshold: 10,
            auto_blacklist: true,
            
            log_security_events: true,
            log_level: "info".to_string(),
            audit_trail_retention: Duration::from_secs(86400 * 90), // 90 days
        }
    }
}

/// Network range for IP filtering
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkRange {
    pub network: IpAddr,
    pub prefix_length: u8,
}

impl NetworkRange {
    pub fn new(network: IpAddr, prefix_length: u8) -> Self {
        Self {
            network,
            prefix_length,
        }
    }
    
    pub fn contains(&self, ip: &IpAddr) -> bool {
        match (self.network, ip) {
            (IpAddr::V4(net), IpAddr::V4(addr)) => {
                let net_u32 = u32::from(net);
                let addr_u32 = u32::from(*addr);
                let mask = if self.prefix_length == 0 {
                    0
                } else {
                    u32::MAX << (32 - self.prefix_length)
                };
                (net_u32 & mask) == (addr_u32 & mask)
            }
            (IpAddr::V6(net), IpAddr::V6(addr)) => {
                // Simplified IPv6 check - in production, use proper IPv6 subnet logic
                net == *addr
            }
            _ => false,
        }
    }
}

/// Blacklisted entity
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlacklistedEntity {
    pub identifier: String, // IP or peer ID
    pub entity_type: EntityType,
    pub reason: String,
    pub timestamp: u64,
    pub expires_at: Option<u64>,
    pub threat_level: ThreatLevel,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
    IP,
    Peer,
    Key,
}

/// Security manager for the mesh network
pub struct SecurityManager {
    config: SecurityConfig,
    blacklist: Arc<RwLock<HashMap<String, BlacklistedEntity>>>,
    connection_attempts: Arc<Mutex<HashMap<IpAddr, Vec<Instant>>>>,
    security_events: Arc<RwLock<Vec<SecurityEvent>>>,
    threat_scores: Arc<Mutex<HashMap<IpAddr, usize>>>,
    key_registry: Arc<RwLock<HashMap<String, KeyInfo>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeyInfo {
    pub key_id: String,
    pub public_key: Vec<u8>,
    pub created_at: u64,
    pub last_used: u64,
    pub usage_count: usize,
    pub is_compromised: bool,
}

impl SecurityManager {
    pub fn new(config: SecurityConfig) -> Self {
        Self {
            config,
            blacklist: Arc::new(RwLock::new(HashMap::new())),
            connection_attempts: Arc::new(Mutex::new(HashMap::new())),
            security_events: Arc::new(RwLock::new(Vec::new())),
            threat_scores: Arc::new(Mutex::new(HashMap::new())),
            key_registry: Arc::new(RwLock::new(HashMap::new())),
        }
    }
    
    /// Check if an IP address is allowed to connect
    pub fn is_ip_allowed(&self, ip: &IpAddr) -> bool {
        // Check blacklist first
        if self.is_blacklisted(&ip.to_string(), EntityType::IP) {
            return false;
        }
        
        // Check network ranges
        if !self.config.allowed_networks.is_empty() {
            let allowed = self.config.allowed_networks.iter()
                .any(|range| range.contains(ip));
            if !allowed {
                return false;
            }
        }
        
        // Check blocked networks
        if self.config.blocked_networks.iter()
            .any(|range| range.contains(ip)) {
            return false;
        }
        
        // Check private network settings
        match ip {
            IpAddr::V4(addr) => {
                if addr.is_loopback() && !self.config.allow_loopback {
                    return false;
                }
                if addr.is_private() && !self.config.allow_private_networks {
                    return false;
                }
            }
            IpAddr::V6(addr) => {
                if addr.is_loopback() && !self.config.allow_loopback {
                    return false;
                }
                // Add IPv6 private network checks as needed
            }
        }
        
        true
    }
    
    /// Record a connection attempt
    pub fn record_connection_attempt(&self, ip: IpAddr, success: bool, reason: String) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Record the attempt
        {
            let mut attempts = self.connection_attempts.lock().unwrap();
            let entry = attempts.entry(ip).or_insert_with(Vec::new);
            entry.push(Instant::now());
            
            // Clean old attempts
            let cutoff = Instant::now() - self.config.rate_limit_window;
            entry.retain(|&time| time > cutoff);
        }
        
        // Log security event
        if self.config.log_security_events {
            let event = SecurityEvent::ConnectionAttempt {
                ip,
                timestamp,
                success,
                reason,
            };
            self.log_security_event(event);
        }
        
        // Check for rate limiting
        if !success {
            self.check_rate_limit(ip);
        }
    }
    
    /// Check if an entity is blacklisted
    pub fn is_blacklisted(&self, identifier: &str, entity_type: EntityType) -> bool {
        let blacklist = self.blacklist.read().unwrap();
        if let Some(entity) = blacklist.get(identifier) {
            // Check if blacklist entry has expired
            if let Some(expires_at) = entity.expires_at {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                if now > expires_at {
                    return false; // Entry has expired
                }
            }
            true
        } else {
            false
        }
    }
    
    /// Check rate limiting for an IP
    fn check_rate_limit(&self, ip: IpAddr) {
        let attempts = {
            let connection_attempts = self.connection_attempts.lock().unwrap();
            connection_attempts.get(&ip)
                .map(|v| v.len())
                .unwrap_or(0)
        };
        
        if attempts > self.config.max_requests_per_window {
            let timestamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_secs();
            
            if self.config.log_security_events {
                let event = SecurityEvent::RateLimitExceeded {
                    ip,
                    timestamp,
                    attempts,
                };
                self.log_security_event(event);
            }
            
            if self.config.auto_blacklist {
                self.blacklist_entity(
                    ip.to_string(),
                    EntityType::IP,
                    format!("Rate limit exceeded: {} attempts", attempts),
                    ThreatLevel::Medium,
                );
            }
        }
    }
    
    /// Log a security event
    fn log_security_event(&self, event: SecurityEvent) {
        let mut events = self.security_events.write().unwrap();
        events.push(event);
        
        // Clean old events
        let retention_secs = self.config.audit_trail_retention.as_secs();
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let cutoff = now.saturating_sub(retention_secs);
        
        events.retain(|event| {
            let event_timestamp = match event {
                SecurityEvent::ConnectionAttempt { timestamp, .. } => *timestamp,
                SecurityEvent::RateLimitExceeded { timestamp, .. } => *timestamp,
                SecurityEvent::InvalidHandshake { timestamp, .. } => *timestamp,
                SecurityEvent::SuspiciousActivity { timestamp, .. } => *timestamp,
                SecurityEvent::KeyCompromise { timestamp, .. } => *timestamp,
                SecurityEvent::PeerBlacklisted { timestamp, .. } => *timestamp,
            };
            event_timestamp > cutoff
        });
    }
    
    /// Get security statistics
    pub fn get_security_stats(&self) -> SecurityStats {
        let blacklist = self.blacklist.read().unwrap();
        let events = self.security_events.read().unwrap();
        let threat_scores = self.threat_scores.read().unwrap();
        
        let high_threat_events = events.values()
            .filter(|event| event.threat_level == ThreatLevel::High)
            .count() as u64;
        
        let blocked_connections = events.values()
            .filter(|event| event.event_type == "connection_blocked")
            .count() as u64;
        
        let encryption_errors = events.values()
            .filter(|event| event.event_type == "encryption_error")
            .count() as u64;
        
        SecurityStats {
            total_events: events.len() as u64,
            high_threat_events,
            blocked_connections,
            encryption_errors,
            last_scan_time: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            threat_level: self.calculate_current_threat_level(),
        }
    }
    
    /// Calculate current threat level based on recent events
    fn calculate_current_threat_level(&self) -> ThreatLevel {
        let events = self.security_events.read().unwrap();
        let recent_events = events.iter()
            .filter(|event| {
                let event_timestamp = match event {
                    SecurityEvent::ConnectionAttempt { timestamp, .. } => *timestamp,
                    SecurityEvent::RateLimitExceeded { timestamp, .. } => *timestamp,
                    SecurityEvent::InvalidHandshake { timestamp, .. } => *timestamp,
                    SecurityEvent::SuspiciousActivity { timestamp, .. } => *timestamp,
                    SecurityEvent::KeyCompromise { timestamp, .. } => *timestamp,
                    SecurityEvent::PeerBlacklisted { timestamp, .. } => *timestamp,
                };
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_secs();
                now - event_timestamp < 3600 // Last hour
            })
            .count();
        
        match recent_events {
            0..=5 => ThreatLevel::Low,
            6..=20 => ThreatLevel::Medium,
            21..=50 => ThreatLevel::High,
            _ => ThreatLevel::Critical,
        }
    }
    
    /// Start security monitoring
    pub async fn start_monitoring(&self) {
        let mut interval = interval(Duration::from_secs(300)); // Every 5 minutes
        
        loop {
            interval.tick().await;
            self.cleanup_expired_entries();
            self.analyze_threat_patterns();
        }
    }
    
    /// Clean up expired blacklist entries
    fn cleanup_expired_entries(&self) {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let mut blacklist = self.blacklist.write().unwrap();
        blacklist.retain(|_, entity| {
            if let Some(expires_at) = entity.expires_at {
                now <= expires_at
            } else {
                true // Permanent entries
            }
        });
    }
    
    /// Analyze threat patterns and update threat scores
    fn analyze_threat_patterns(&self) {
        let events = self.security_events.read().unwrap();
        let mut threat_scores = self.threat_scores.lock().unwrap();
        
        // Reset scores
        threat_scores.clear();
        
        // Analyze recent events for patterns
        for event in events.iter() {
            let ip = match event {
                SecurityEvent::ConnectionAttempt { ip, .. } => *ip,
                SecurityEvent::RateLimitExceeded { ip, .. } => *ip,
                SecurityEvent::InvalidHandshake { ip, .. } => *ip,
                SecurityEvent::SuspiciousActivity { ip, .. } => *ip,
                _ => continue,
            };
            
            let score = threat_scores.entry(ip).or_insert(0);
            *score += 1;
        }
    }
    
    /// Add an entity to the blacklist
    pub fn blacklist_entity(&self, identifier: String, entity_type: EntityType, reason: String, threat_level: ThreatLevel) {
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        let expires_at = if threat_level == ThreatLevel::Critical {
            None // Permanent blacklist
        } else {
            Some(timestamp + self.config.blacklist_duration.as_secs())
        };
        
        let entity = BlacklistedEntity {
            identifier: identifier.clone(),
            entity_type,
            reason: reason.clone(),
            timestamp,
            expires_at,
            threat_level,
        };
        
        {
            let mut blacklist = self.blacklist.write().unwrap();
            blacklist.insert(identifier.clone(), entity.clone());
        }
        
        if self.config.log_security_events {
            let event = SecurityEvent::PeerBlacklisted {
                peer_id: identifier,
                timestamp,
                reason,
            };
            self.log_security_event(event);
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityStats {
    pub total_events: u64,
    pub high_threat_events: u64,
    pub blocked_connections: u64,
    pub encryption_errors: u64,
    pub last_scan_time: u64,
    pub threat_level: ThreatLevel,
}

impl Default for SecurityStats {
    fn default() -> Self {
        Self {
            total_events: 0,
            high_threat_events: 0,
            blocked_connections: 0,
            encryption_errors: 0,
            last_scan_time: 0,
            threat_level: ThreatLevel::Low,
        }
    }
}

impl Default for SecurityManager {
    fn default() -> Self {
        Self::new(SecurityConfig::default())
    }
} 