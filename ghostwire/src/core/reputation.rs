//! Reputation system: local scoring, signed scores, gaming resistance.
//! Next: integrate with peer management and federation.

/// Trait for guarding against reputation system gaming.
pub trait ReputationGuard {
    /// Update a peer's score, enforcing caps and signatures.
    fn update_score(&mut self, peer_id: &str, delta: f32, signature: &[u8]);
    /// Decay/rotate scores.
    fn decay(&mut self);
}

/// Default stub: no-op.
pub struct NoOpReputationGuard;
impl ReputationGuard for NoOpReputationGuard {
    fn update_score(&mut self, _peer_id: &str, _delta: f32, _signature: &[u8]) {}
    fn decay(&mut self) {}
}

/// Struct for signed reputation scores.
#[derive(Default, Clone)]
pub struct SignedScore {
    pub peer_id: String,
    pub score: f32,
    pub signature: Vec<u8>,
} 