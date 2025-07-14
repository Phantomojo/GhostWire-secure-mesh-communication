//! Disaster mode: triggers, fallback order, metrics.
//! Next: integrate with transport manager and peer logic.

/// Trait for disaster mode triggers.
pub trait DisasterTrigger {
    /// Should disaster mode be triggered?
    fn should_trigger(&self, metrics: &DisasterMetrics) -> bool;
}

/// Default stub: never triggers.
pub struct NeverDisasterTrigger;
impl DisasterTrigger for NeverDisasterTrigger {
    fn should_trigger(&self, _metrics: &DisasterMetrics) -> bool { false }
}

/// Trait for disaster fallback policy.
pub trait DisasterPolicy {
    /// Get fallback transport order.
    fn fallback_order(&self) -> Vec<TransportType>;
}

/// Default stub: returns empty order.
pub struct EmptyDisasterPolicy;
impl DisasterPolicy for EmptyDisasterPolicy {
    fn fallback_order(&self) -> Vec<TransportType> { vec![] }
}

/// Placeholder types for demo
pub struct DisasterMetrics;
pub enum TransportType { LoRa, Bluetooth, WiFi, StealthTcp, Mesh } 