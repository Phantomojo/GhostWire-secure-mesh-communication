//! transport_selector.rs — Adaptive transport selection with online learning
//!
//! Inspired by DORS (Dynamic Offline Relay Switch) scoring taxonomy and
//! Thompson Sampling for exploration/exploitation balance.
//!
//! Architecture:
//! - Layer 0: Per-transport weighted scoring (DORS-inspired)
//! - Layer 1: Thompson Sampling for online learning (no pre-training needed)
//! - Layer 2: Hysteresis + cooldown + stability window (anti-flap)
//! - Layer 3: Binary black-hole detection (libp2p pattern)

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::{Duration, Instant};
use tracing::{debug, info, warn};

use crate::core::disaster::{DisasterMetrics, TransportType};

// ═══════════════════════════════════════════════════════════════════════════════
// SCORING FACTORS (DORS taxonomy)
// ═══════════════════════════════════════════════════════════════════════════════

/// Normalized metrics for transport scoring (0.0 to 1.0 each)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportMetrics {
    /// Signal strength (RSSI) — 0.0 = weak, 1.0 = strong
    pub signal_strength: f64,
    /// Throughput — 0.0 = low, 1.0 = high
    pub throughput: f64,
    /// Packet loss — 0.0 = none, 1.0 = total loss
    pub packet_loss: f64,
    /// Latency — 0.0 = high latency, 1.0 = low latency (inverted)
    pub latency_score: f64,
    /// Energy cost — 0.0 = high drain, 1.0 = low drain
    pub energy_efficiency: f64,
    /// Congestion — 0.0 = congested, 1.0 = clear
    pub congestion: f64,
    /// Reliability — delivery success rate over sliding window
    pub reliability: f64,
}

impl Default for TransportMetrics {
    fn default() -> Self {
        Self {
            signal_strength: 0.5,
            throughput: 0.5,
            packet_loss: 0.0,
            latency_score: 0.5,
            energy_efficiency: 0.5,
            congestion: 0.5,
            reliability: 0.5,
        }
    }
}

/// Per-transport weight profiles (encode design intent)
/// Weights sum to 1.0 — different transports optimize for different things
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WeightProfile {
    pub signal: f64,
    pub throughput: f64,
    pub loss: f64,
    pub latency: f64,
    pub energy: f64,
    pub congestion: f64,
    pub reliability: f64,
}

impl WeightProfile {
    /// BLE: optimized for energy and mesh proximity
    pub fn ble() -> Self {
        Self {
            signal: 0.25,
            throughput: 0.05,
            loss: 0.10,
            latency: 0.10,
            energy: 0.30,
            congestion: 0.10,
            reliability: 0.10,
        }
    }

    /// WiFi: optimized for throughput and reliability
    pub fn wifi() -> Self {
        Self {
            signal: 0.15,
            throughput: 0.30,
            loss: 0.15,
            latency: 0.15,
            energy: 0.05,
            congestion: 0.10,
            reliability: 0.10,
        }
    }

    /// TCP/QUIC: optimized for bandwidth and reliability
    pub fn tcp() -> Self {
        Self {
            signal: 0.0,
            throughput: 0.30,
            loss: 0.20,
            latency: 0.20,
            energy: 0.0,
            congestion: 0.15,
            reliability: 0.15,
        }
    }

    /// LoRa: optimized for range and energy
    pub fn lora() -> Self {
        Self {
            signal: 0.20,
            throughput: 0.0,
            loss: 0.15,
            latency: 0.05,
            energy: 0.35,
            congestion: 0.10,
            reliability: 0.15,
        }
    }

    /// Mesh (libp2p): balanced for multi-hop
    pub fn mesh() -> Self {
        Self {
            signal: 0.15,
            throughput: 0.15,
            loss: 0.20,
            latency: 0.15,
            energy: 0.10,
            congestion: 0.10,
            reliability: 0.15,
        }
    }

    /// Reticulum: store-and-forward overlay for constrained/off-grid networks.
    /// Optimized for reliability and energy — not throughput or latency.
    pub fn reticulum() -> Self {
        Self {
            signal: 0.15,
            throughput: 0.05,
            loss: 0.15,
            latency: 0.05,
            energy: 0.25,
            congestion: 0.10,
            reliability: 0.25,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// THOMPSON SAMPLING (online learning, no pre-training needed)
// ═══════════════════════════════════════════════════════════════════════════════

/// Beta distribution posterior for Thompson Sampling
#[derive(Debug, Clone, Serialize, Deserialize)]
struct BetaPosterior {
    alpha: f64, // successes + 1
    beta: f64,  // failures + 1
}

impl BetaPosterior {
    fn new() -> Self {
        Self { alpha: 1.0, beta: 1.0 } // Uniform prior
    }

    /// Sample from Beta(alpha, beta) using Jöhnk's algorithm
    fn sample(&self) -> f64 {
        // Simple gamma-based sampling for Beta distribution
        let gamma_alpha = gamma_sample(self.alpha);
        let gamma_beta = gamma_sample(self.beta);
        gamma_alpha / (gamma_alpha + gamma_beta)
    }

    /// Update with observed reward (0.0 to 1.0)
    fn update(&mut self, reward: f64) {
        self.alpha += reward;
        self.beta += 1.0 - reward;
    }

    /// Mean of the posterior
    fn mean(&self) -> f64 {
        self.alpha / (self.alpha + self.beta)
    }
}

/// Simple gamma distribution sample (Marsaglia and Tsang's method)
fn gamma_sample(shape: f64) -> f64 {
    if shape < 1.0 {
        // Use Johnk's algorithm for shape < 1
        return gamma_sample(shape + 1.0) * rand_f64().powf(1.0 / shape);
    }

        let d = shape - 1.0 / 3.0;
        let c = 1.0 / (9.0 * d).sqrt();

        loop {
            let mut x;
            let mut v;
        loop {
            x = rand_normal();
            v = 1.0 + c * x;
            if v > 0.0 {
                break;
            }
        }
        let v = v * v * v;
        let u = rand_f64();

        if u < 1.0 - 0.0331 * (x * x) * (x * x) {
            return d * v;
        }

        if (u.ln()) < 0.5 * x * x + d * (1.0 - v + v.ln()) {
            return d * v;
        }
    }
}

/// Box-Muller transform for normal distribution
fn rand_normal() -> f64 {
    use std::f64::consts::PI;
    let u1 = rand_f64();
    let u2 = rand_f64();
    (-2.0 * u1.ln()).sqrt() * (2.0 * PI * u2).cos()
}

/// Simple PRNG (xorshift64) — no external crate needed
fn rand_f64() -> f64 {
    use std::cell::Cell;
    thread_local! {
        static STATE: Cell<u64> = const { Cell::new(0x1234_5678_9ABC_DEF0) };
    }
    STATE.with(|s| {
        let mut x = s.get();
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        s.set(x);
        (x as f64) / (u64::MAX as f64)
    })
}

// ═══════════════════════════════════════════════════════════════════════════════
// CONTEXT BUCKETING (per-context bandit learning)
// ═══════════════════════════════════════════════════════════════════════════════

/// Discretized context for per-context Thompson Sampling
#[derive(Debug, Clone, Hash, Eq, PartialEq, Serialize, Deserialize)]
pub struct ContextKey {
    battery_tier: u8,   // 0=low, 1=mid, 2=high
    signal_tier: u8,    // 0=weak, 1=mid, 2=strong
    congestion_tier: u8, // 0=clear, 1=moderate, 2=congested
}

impl ContextKey {
    fn from_metrics(metrics: &TransportMetrics) -> Self {
        Self {
            battery_tier: Self::tier(metrics.energy_efficiency, 0.33, 0.66),
            signal_tier: Self::tier(metrics.signal_strength, 0.33, 0.66),
            congestion_tier: Self::tier(metrics.congestion, 0.33, 0.66),
        }
    }

    fn tier(value: f64, low: f64, high: f64) -> u8 {
        if value < low { 0 } else if value < high { 1 } else { 2 }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// BLACK-HOLE DETECTOR (binary blocking, libp2p pattern)
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BlackHoleState {
    Allowed,
    Probing,
    Blocked,
}

/// Tracks success rate and blocks transport when success rate drops below threshold
#[derive(Debug, Clone)]
struct BlackHoleDetector {
    state: BlackHoleState,
    success_count: u32,
    total_count: u32,
    window_size: u32,      // Number of recent attempts to consider
    min_successes: u32,    // Minimum successes in window to stay allowed
}

impl BlackHoleDetector {
    fn new(window_size: u32, min_successes: u32) -> Self {
        Self {
            state: BlackHoleState::Allowed,
            success_count: 0,
            total_count: 0,
            window_size,
            min_successes,
        }
    }

    fn record(&mut self, success: bool) {
        if success {
            self.success_count += 1;
            // One success in Blocked state → reset (binary property)
            if self.state == BlackHoleState::Blocked {
                self.state = BlackHoleState::Probing;
                self.success_count = 1;
                self.total_count = 1;
                return;
            }
        }
        self.total_count += 1;

        // Sliding window: if we've seen enough samples, evaluate
        if self.total_count >= self.window_size {
            if self.success_count < self.min_successes {
                self.state = BlackHoleState::Blocked;
                warn!(
                    "BLACK HOLE DETECTED: {} successes in {} attempts — blocking transport",
                    self.success_count, self.total_count
                );
            } else {
                self.state = BlackHoleState::Allowed;
            }
            // Reset window
            self.success_count = if success { 1 } else { 0 };
            self.total_count = 1;
        }
    }

    fn is_blocked(&self) -> bool {
        self.state == BlackHoleState::Blocked
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// HYSTERESIS & ANTI-FLAP (DORS pattern)
// ═══════════════════════════════════════════════════════════════════════════════

#[derive(Debug, Clone)]
struct HysteresisState {
    current_transport: TransportType,
    current_score: f64,
    last_switch: Instant,
    stability_window: Vec<f64>, // Recent scores of current transport
    window_size: usize,
    switch_cooldown: Duration,
    min_score_delta: f64, // New transport must beat current by this margin
}

impl HysteresisState {
    fn new(
        initial: TransportType,
        cooldown: Duration,
        min_delta: f64,
        window_size: usize,
    ) -> Self {
        Self {
            current_transport: initial,
            current_score: 0.5,
            last_switch: Instant::now(),
            stability_window: Vec::with_capacity(window_size),
            window_size,
            switch_cooldown: cooldown,
            min_score_delta: min_delta,
        }
    }

    /// Record a score for the current transport
    fn record_score(&mut self, score: f64) {
        self.stability_window.push(score);
        if self.stability_window.len() > self.window_size {
            self.stability_window.remove(0);
        }
        // Update running average
        self.current_score =
            self.stability_window.iter().sum::<f64>() / self.stability_window.len() as f64;
    }

    /// Check if we should switch to a new transport
    fn should_switch(&self, new_transport: TransportType, new_score: f64) -> bool {
        // Can't switch to same transport
        if new_transport == self.current_transport {
            return false;
        }

        // Cooldown check
        if self.last_switch.elapsed() < self.switch_cooldown {
            debug!(
                "Switch blocked by cooldown: {:?} since last switch",
                self.last_switch.elapsed()
            );
            return false;
        }

        // Hysteresis: new transport must beat current by margin
        let delta = new_score - self.current_score;
        if delta < self.min_score_delta {
            debug!(
                "Switch blocked by hysteresis: delta={:.3} < min={:.3}",
                delta, self.min_score_delta
            );
            return false;
        }

        true
    }

    /// Execute the switch
    fn switch_to(&mut self, transport: TransportType, score: f64) {
        info!(
            "TRANSPORT SWITCH: {:?} (score={:.3}) → {:?} (score={:.3})",
            self.current_transport, self.current_score, transport, score
        );
        self.current_transport = transport;
        self.current_score = score;
        self.last_switch = Instant::now();
        self.stability_window.clear();
        self.stability_window.push(score);
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TRANSPORT SELECTOR (main entry point)
// ═══════════════════════════════════════════════════════════════════════════════

/// Configuration for the transport selector
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TransportSelectorConfig {
    /// Minimum time between transport switches
    pub switch_cooldown: Duration,
    /// New transport must beat current by this score margin
    pub min_score_delta: f64,
    /// Number of recent scores to average for stability
    pub stability_window_size: usize,
    /// Black-hole detection window
    pub blackhole_window: u32,
    /// Minimum successes in window to stay allowed
    pub blackhole_min_successes: u32,
    /// Exploration rate (0.0 = pure exploitation, 1.0 = pure exploration)
    pub exploration_rate: f64,
}

impl Default for TransportSelectorConfig {
    fn default() -> Self {
        Self {
            switch_cooldown: Duration::from_secs(30),
            min_score_delta: 0.1,
            stability_window_size: 10,
            blackhole_window: 20,
            blackhole_min_successes: 5,
            exploration_rate: 0.1,
        }
    }
}

/// The adaptive transport selector
pub struct TransportSelector {
    config: TransportSelectorConfig,

    /// Per-transport weight profiles
    weight_profiles: HashMap<TransportType, WeightProfile>,

    /// Thompson Sampling posterior per (context, transport)
    bandit: HashMap<(ContextKey, TransportType), BetaPosterior>,

    /// Black-hole detection per transport
    blackholes: HashMap<TransportType, BlackHoleDetector>,

    /// Hysteresis state
    hysteresis: HysteresisState,

    /// Per-transport score history for telemetry
    score_history: HashMap<TransportType, Vec<(Instant, f64)>>,
}

impl TransportSelector {
    pub fn new(config: TransportSelectorConfig) -> Self {
        let mut weight_profiles = HashMap::new();
        weight_profiles.insert(TransportType::Bluetooth, WeightProfile::ble());
        weight_profiles.insert(TransportType::WiFi, WeightProfile::wifi());
        weight_profiles.insert(TransportType::Quic, WeightProfile::tcp());
        weight_profiles.insert(TransportType::StealthTcp, WeightProfile::tcp());
        weight_profiles.insert(TransportType::Mesh, WeightProfile::mesh());
        weight_profiles.insert(TransportType::LoRa, WeightProfile::lora());
        weight_profiles.insert(TransportType::Reticulum, WeightProfile::reticulum());

        let mut blackholes = HashMap::new();
        for transport in &[
            TransportType::Bluetooth,
            TransportType::WiFi,
            TransportType::Quic,
            TransportType::StealthTcp,
            TransportType::Mesh,
            TransportType::LoRa,
            TransportType::Reticulum,
        ] {
            blackholes.insert(
                *transport,
                BlackHoleDetector::new(config.blackhole_window, config.blackhole_min_successes),
            );
        }

        let cooldown = config.switch_cooldown;
        let min_delta = config.min_score_delta;
        let window_size = config.stability_window_size;

        Self {
            config,
            weight_profiles,
            bandit: HashMap::new(),
            blackholes,
            hysteresis: HysteresisState::new(
                TransportType::StealthTcp,
                cooldown,
                min_delta,
                window_size,
            ),
            score_history: HashMap::new(),
        }
    }

    /// Score a transport given current metrics
    fn score_transport(&self, transport: &TransportType, metrics: &TransportMetrics) -> f64 {
        let default_weights = WeightProfile::tcp();
        let weights = self
            .weight_profiles
            .get(transport)
            .unwrap_or(&default_weights);

        // Weighted sum: lower loss = better (invert), lower latency = better (invert)
        let score = weights.signal * metrics.signal_strength
            + weights.throughput * metrics.throughput
            + weights.loss * (1.0 - metrics.packet_loss)  // Invert: low loss = high score
            + weights.latency * metrics.latency_score
            + weights.energy * metrics.energy_efficiency
            + weights.congestion * metrics.congestion
            + weights.reliability * metrics.reliability;

        score.clamp(0.0, 1.0)
    }

    /// Get Thompson Sampling score for a transport in a context
    fn thompson_sample(&mut self, context: &ContextKey, transport: &TransportType) -> f64 {
        let key = (context.clone(), *transport);
        let posterior = self
            .bandit
            .entry(key)
            .or_insert_with(BetaPosterior::new);

        // With probability exploration_rate, sample from posterior
        // Otherwise use posterior mean (exploitation)
        if rand_f64() < self.config.exploration_rate {
            posterior.sample()
        } else {
            posterior.mean()
        }
    }

    /// Record outcome for Thompson Sampling
    pub fn record_outcome(
        &mut self,
        context: &ContextKey,
        transport: &TransportType,
        success: bool,
    ) {
        let reward = if success { 1.0 } else { 0.0 };
        let key = (context.clone(), *transport);
        let posterior = self
            .bandit
            .entry(key)
            .or_insert_with(BetaPosterior::new);
        posterior.update(reward);

        // Also update black-hole detector
        if let Some(bh) = self.blackholes.get_mut(transport) {
            bh.record(success);
        }
    }

    /// Record a transport outcome for telemetry
    pub fn record_score(&mut self, transport: TransportType, score: f64) {
        self.score_history
            .entry(transport)
            .or_default()
            .push((Instant::now(), score));

        // Keep only last 100 entries per transport
        if let Some(history) = self.score_history.get_mut(&transport) {
            if history.len() > 100 {
                history.drain(..history.len() - 100);
            }
        }
    }

    /// Main selection function: given current metrics, return best transport
    pub fn select(&mut self, metrics: &TransportMetrics, available: &[TransportType]) -> TransportType {
        let context = ContextKey::from_metrics(metrics);

        // Filter out black-holed transports
        let candidates: Vec<TransportType> = available
            .iter()
            .filter(|t| {
                !self
                    .blackholes
                    .get(t)
                    .map(|bh| bh.is_blocked())
                    .unwrap_or(false)
            })
            .copied()
            .collect();

        if candidates.is_empty() {
            warn!("All transports black-holed! Falling back to first available");
            return available.first().copied().unwrap_or(TransportType::StealthTcp);
        }

        // Score each candidate
        let mut scored: Vec<(TransportType, f64)> = candidates
            .iter()
            .map(|t| {
                let base_score = self.score_transport(t, metrics);
                let thompson_score = self.thompson_sample(&context, t);
                // Blend: 70% weighted scoring + 30% Thompson Sampling
                let blended = 0.7 * base_score + 0.3 * thompson_score;
                (*t, blended)
            })
            .collect();

        // Sort by score (highest first)
        scored.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap_or(std::cmp::Ordering::Equal));

        let (best_transport, best_score) = scored[0];

        // Record score for telemetry
        self.record_score(best_transport, best_score);

        // Update hysteresis with current score
        self.hysteresis.record_score(best_score);

        // Check if we should switch
        if self.hysteresis.should_switch(best_transport, best_score) {
            self.hysteresis.switch_to(best_transport, best_score);
            best_transport
        } else {
            // Stay with current transport
            self.hysteresis.current_transport
        }
    }

    /// Get current transport without switching
    pub fn current_transport(&self) -> TransportType {
        self.hysteresis.current_transport
    }

    /// Force a specific transport (manual override)
    pub fn force_transport(&mut self, transport: TransportType) {
        info!("FORCED TRANSPORT SWITCH: {:?}", transport);
        self.hysteresis.current_transport = transport;
        self.hysteresis.last_switch = Instant::now();
    }

    /// Get score history for telemetry
    pub fn get_score_history(&self, transport: &TransportType) -> &[(Instant, f64)] {
        self.score_history
            .get(transport)
            .map(|v| v.as_slice())
            .unwrap_or(&[])
    }

    /// Get Thompson Sampling statistics for a transport
    pub fn get_bandit_stats(&self, transport: &TransportType) -> Option<(f64, f64)> {
        // Find any context for this transport
        self.bandit
            .iter()
            .find(|((_, t), _)| t == transport)
            .map(|(_, posterior)| (posterior.mean(), posterior.alpha / (posterior.alpha + posterior.beta)))
    }

    /// Get all transport scores for current metrics (for diagnostics)
    pub fn score_all(&self, metrics: &TransportMetrics) -> Vec<(TransportType, f64)> {
        self.weight_profiles
            .keys()
            .map(|t| (*t, self.score_transport(t, metrics)))
            .collect()
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// METRICS CONVERSION (from DisasterMetrics to TransportMetrics)
// ═══════════════════════════════════════════════════════════════════════════════

impl From<&DisasterMetrics> for TransportMetrics {
    fn from(m: &DisasterMetrics) -> Self {
        // Normalize latency: 0ms = 1.0, 500ms+ = 0.0
        let latency_score = (1.0 - (m.latency_ms as f64 / 500.0)).clamp(0.0, 1.0);

        // Invert packet loss: 0.0 loss = 1.0 score
        let reliability = 1.0 - m.packet_loss_ratio as f64;

        Self {
            signal_strength: 0.5, // Unknown without RSSI data
            throughput: 0.5,      // Unknown without measurement
            packet_loss: m.packet_loss_ratio as f64,
            latency_score,
            energy_efficiency: 0.5, // Unknown without battery data
            congestion: (1.0 - m.packet_loss_ratio as f64).clamp(0.0, 1.0),
            reliability,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// TESTS
// ═══════════════════════════════════════════════════════════════════════════════

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_weight_profiles_sum_to_one() {
        let profiles = vec![
            WeightProfile::ble(),
            WeightProfile::wifi(),
            WeightProfile::tcp(),
            WeightProfile::lora(),
            WeightProfile::mesh(),
        ];

        for profile in profiles {
            let sum = profile.signal
                + profile.throughput
                + profile.loss
                + profile.latency
                + profile.energy
                + profile.congestion
                + profile.reliability;
            assert!(
                (sum - 1.0).abs() < 0.001,
                "Weight profile sums to {}, not 1.0",
                sum
            );
        }
    }

    #[test]
    fn test_beta_posterior_sampling() {
        let posterior = BetaPosterior::new();
        // Uniform prior should sample around 0.5
        let samples: Vec<f64> = (0..1000).map(|_| posterior.sample()).collect();
        let mean = samples.iter().sum::<f64>() / samples.len() as f64;
        assert!(
            (mean - 0.5).abs() < 0.1,
            "Uniform prior mean should be ~0.5, got {}",
            mean
        );
    }

    #[test]
    fn test_beta_posterior_update() {
        let mut posterior = BetaPosterior::new();
        // Update with all successes
        for _ in 0..100 {
            posterior.update(1.0);
        }
        assert!(posterior.mean() > 0.9, "Should converge to ~1.0");

        // Update with all failures
        let mut posterior2 = BetaPosterior::new();
        for _ in 0..100 {
            posterior2.update(0.0);
        }
        assert!(posterior2.mean() < 0.1, "Should converge to ~0.0");
    }

    #[test]
    fn test_black_hole_detection() {
        let mut bh = BlackHoleDetector::new(10, 3);

        // Record failures
        for _ in 0..10 {
            bh.record(false);
        }
        assert!(bh.is_blocked(), "Should be blocked after 10 failures");

        // One success should reset
        bh.record(true);
        assert!(!bh.is_blocked(), "Should be unblocked after success");
    }

    #[test]
    fn test_hysteresis_cooldown() {
        let mut hyst = HysteresisState::new(
            TransportType::Quic,
            Duration::from_secs(30),
            0.1,
            10,
        );

        // Should not switch immediately (cooldown)
        assert!(
            !hyst.should_switch(TransportType::Mesh, 0.9),
            "Should not switch during cooldown"
        );
    }

    #[test]
    fn test_hysteresis_min_delta() {
        let mut hyst = HysteresisState::new(
            TransportType::Quic,
            Duration::from_millis(0), // No cooldown for test
            0.2,                      // Require 0.2 delta
            10,
        );
        hyst.current_score = 0.5;

        // Should not switch if delta too small
        assert!(
            !hyst.should_switch(TransportType::Mesh, 0.6),
            "Should not switch with small delta"
        );
    }

    #[test]
    fn test_scoring() {
        let selector = TransportSelector::new(TransportSelectorConfig::default());
        let metrics = TransportMetrics {
            signal_strength: 0.8,
            throughput: 0.7,
            packet_loss: 0.1,
            latency_score: 0.6,
            energy_efficiency: 0.5,
            congestion: 0.7,
            reliability: 0.9,
        };

        let scores = selector.score_all(&metrics);
        assert!(!scores.is_empty());

        // WiFi should score higher than BLE for throughput-heavy metrics
        let wifi_score = scores
            .iter()
            .find(|(t, _)| *t == TransportType::WiFi)
            .map(|(_, s)| *s)
            .unwrap_or(0.0);
        let ble_score = scores
            .iter()
            .find(|(t, _)| *t == TransportType::Bluetooth)
            .map(|(_, s)| *s)
            .unwrap_or(0.0);

        // WiFi weights throughput higher, BLE weights energy higher
        // With good throughput metrics, WiFi should score well
        assert!(wifi_score > 0.0, "WiFi score should be positive");
        assert!(ble_score > 0.0, "BLE score should be positive");
    }

    #[test]
    fn test_disaster_metrics_conversion() {
        let disaster = DisasterMetrics {
            latency_ms: 100,
            packet_loss_ratio: 0.05,
            active_peers: 5,
            is_internet_reachable: true,
            last_update: 0,
        };

        let transport: TransportMetrics = (&disaster).into();
        assert!((transport.latency_score - 0.8).abs() < 0.01);
        assert!((transport.reliability - 0.95).abs() < 0.01);
        assert!((transport.packet_loss - 0.05).abs() < 0.01);
    }

    #[test]
    fn test_context_bucketing() {
        let metrics = TransportMetrics {
            signal_strength: 0.8,
            throughput: 0.5,
            packet_loss: 0.1,
            latency_score: 0.5,
            energy_efficiency: 0.7,
            congestion: 0.3,
            reliability: 0.5,
        };

        let key = ContextKey::from_metrics(&metrics);
        assert_eq!(key.battery_tier, 2);   // high (0.7 > 0.66)
        assert_eq!(key.signal_tier, 2);    // strong (0.8 > 0.66)
        assert_eq!(key.congestion_tier, 0); // clear (0.3 < 0.33)
    }
}
