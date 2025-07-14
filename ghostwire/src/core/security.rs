//! Security module: Sybil defense, quotas, blacklists.
//! Next: integrate with peer discovery, store, and reputation.

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