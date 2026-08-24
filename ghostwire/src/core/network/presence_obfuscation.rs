//! presence_obfuscation.rs — Intersection attack mitigation
//!
//! Intersection attacks work by correlating *who was online* with *when
//! messages were delivered*. Over time, even encrypted traffic leaks a
//! social graph: "Alice always receives messages minutes after Bob connects."
//!
//! Defenses implemented here:
//!
//!   1. **Presence jitter** — randomise how quickly we announce connectivity.
//!      Rather than broadcasting "I'm online" the moment we connect, wait a
//!      random delay. Adversaries can't correlate connection events to messages.
//!
//!   2. **Offline message pre-fetching** — fetch messages during random windows,
//!      not just at login. Makes "user fetched messages → user is reading them"
//!      correlation unreliable.
//!
//!   3. **Dummy activity** — periodically perform fake DHT lookups and send
//!      cover HAVE announcements for blocks we don't actually hold.
//!      Adversaries can't distinguish real activity from noise.
//!
//!   4. **Stable pseudo-presence** — instead of going truly offline, maintain
//!      a minimal keepalive that doesn't leak session timing.

use rand::Rng;
use serde::{Deserialize, Serialize};
use std::time::Duration;
use tokio::time::sleep;

/// Configuration for presence obfuscation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PresenceConfig {
    /// Whether to delay presence announcement on connect
    pub jitter_on_connect: bool,
    /// Max jitter delay on connect (seconds)
    pub connect_jitter_max_secs: u64,
    /// Whether to fetch messages on a randomised schedule (not just at login)
    pub randomised_fetch: bool,
    /// Interval range for randomised message fetch (seconds)
    pub fetch_interval_min_secs: u64,
    pub fetch_interval_max_secs: u64,
    /// Whether to send dummy DHT activity
    pub dummy_activity: bool,
    /// Mean interval between dummy activity events (seconds)
    pub dummy_mean_secs: u64,
}

impl Default for PresenceConfig {
    fn default() -> Self {
        Self {
            jitter_on_connect: true,
            connect_jitter_max_secs: 30,
            randomised_fetch: true,
            fetch_interval_min_secs: 60,
            fetch_interval_max_secs: 600,
            dummy_activity: true,
            dummy_mean_secs: 120,
        }
    }
}

impl PresenceConfig {
    pub fn high_anonymity() -> Self {
        Self {
            connect_jitter_max_secs: 120,
            fetch_interval_min_secs: 300,
            fetch_interval_max_secs: 3600,
            dummy_mean_secs: 60,
            ..Default::default()
        }
    }

    pub fn low_latency() -> Self {
        Self {
            jitter_on_connect: false,
            connect_jitter_max_secs: 0,
            randomised_fetch: false,
            fetch_interval_min_secs: 30,
            fetch_interval_max_secs: 60,
            dummy_activity: false,
            dummy_mean_secs: 600,
        }
    }
}

// ── Connect jitter ────────────────────────────────────────────────────────────

/// Sleep for a random jitter period before announcing presence.
/// Call this when the node first connects to the mesh.
///
/// Usage in mesh.rs connect flow:
/// ```ignore
/// presence_obfuscation::apply_connect_jitter(&config).await;
/// // Now publish username to DHT
/// core.publish_username().await?;
/// ```
pub async fn apply_connect_jitter(config: &PresenceConfig) {
    if !config.jitter_on_connect || config.connect_jitter_max_secs == 0 {
        return;
    }

    let jitter_ms = rand::rng().random_range(0..=config.connect_jitter_max_secs * 1000);

    if jitter_ms > 0 {
        tracing::debug!("Presence jitter: waiting {}ms before announcing", jitter_ms);
        sleep(Duration::from_millis(jitter_ms)).await;
    }
}

// ── Randomised message fetch scheduler ────────────────────────────────────────

/// Spawn a background task that periodically fetches mailbox messages
/// on a randomised schedule, regardless of user activity.
///
/// This means: even if a user is not actively using the app, their node
/// fetches messages occasionally — breaking the "active session = messages"
/// correlation.
pub fn start_randomised_fetch_loop(
    config: PresenceConfig,
    fetch_tx: tokio::sync::mpsc::Sender<()>,
) {
    tokio::spawn(async move {
        loop {
            let interval_ms = rand::rng().random_range(
                config.fetch_interval_min_secs * 1000..=config.fetch_interval_max_secs * 1000,
            );
            sleep(Duration::from_millis(interval_ms)).await;

            if config.randomised_fetch {
                let _ = fetch_tx.try_send(());
            }
        }
    });
}

// ── Dummy activity ────────────────────────────────────────────────────────────

/// A dummy activity event — one of several types
#[derive(Debug, Clone)]
pub enum DummyActivity {
    /// Fake DHT lookup for a random key
    DhtLookup { key_hash: [u8; 32] },
    /// Fake HAVE announcement for a block we don't hold
    FakeHave { cid_hex: String },
    /// Fake wantlist broadcast (empty or random)
    FakeWant { cid_hex: String },
}

/// Generate a random dummy activity event
pub fn generate_dummy_activity() -> DummyActivity {
    let mut rng = rand::rng();
    let activity_type: u8 = rng.random_range(0..3);

    match activity_type {
        0 => {
            let mut key = [0u8; 32];
            rng.fill(&mut key);
            DummyActivity::DhtLookup { key_hash: key }
        }
        1 => {
            let mut cid = [0u8; 32];
            rng.fill(&mut cid);
            DummyActivity::FakeHave {
                cid_hex: hex::encode(cid),
            }
        }
        _ => {
            let mut cid = [0u8; 32];
            rng.fill(&mut cid);
            DummyActivity::FakeWant {
                cid_hex: hex::encode(cid),
            }
        }
    }
}

/// Spawn a background task that emits dummy activity events at Poisson-distributed
/// intervals (exponentially distributed inter-event times).
///
/// Usage:
/// ```ignore
/// let (dummy_tx, mut dummy_rx) = tokio::sync::mpsc::channel(32);
/// presence_obfuscation::start_dummy_activity_loop(config.clone(), dummy_tx);
///
/// // In your mesh event loop:
/// tokio::select! {
///     Some(activity) = dummy_rx.recv() => {
///         handle_dummy_activity(activity).await;
///     }
/// }
/// ```
pub fn start_dummy_activity_loop(
    config: PresenceConfig,
    tx: tokio::sync::mpsc::Sender<DummyActivity>,
) {
    if !config.dummy_activity {
        return;
    }

    tokio::spawn(async move {
        use rand_distr::{Distribution, Exp};
        let lambda = 1.0 / (config.dummy_mean_secs as f64 * 1000.0);
        let exp = Exp::new(lambda).expect("Valid lambda");

        loop {
            let delay_ms = exp.sample(&mut rand::rng()) as u64;
            let delay_ms = delay_ms.clamp(1_000, config.dummy_mean_secs * 10_000);
            sleep(Duration::from_millis(delay_ms)).await;

            let activity = generate_dummy_activity();
            let _ = tx.try_send(activity);
        }
    });
}

// ── Stable pseudo-presence ────────────────────────────────────────────────────

/// Keepalive config: send periodic keepalives so connection timing is stable.
///
/// Without this, a user who is "idle" has a detectable silence pattern.
/// With this, keepalive frequency is constant regardless of user activity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeepaliveConfig {
    /// Interval between keepalive pings (seconds)
    pub interval_secs: u64,
    /// Jitter range to add to each interval (seconds)
    pub jitter_secs: u64,
}

impl Default for KeepaliveConfig {
    fn default() -> Self {
        Self {
            interval_secs: 30,
            jitter_secs: 10,
        }
    }
}

/// Spawn a keepalive loop.
pub fn start_keepalive_loop(config: KeepaliveConfig, tx: tokio::sync::mpsc::Sender<()>) {
    tokio::spawn(async move {
        loop {
            let jitter_ms = rand::rng().random_range(0..=config.jitter_secs * 1000);
            let interval_ms = config.interval_secs * 1000 + jitter_ms;
            sleep(Duration::from_millis(interval_ms)).await;
            let _ = tx.try_send(());
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dummy_activity_generates_all_types() {
        let mut has_dht = false;
        let mut has_have = false;
        let mut has_want = false;

        for _ in 0..300 {
            match generate_dummy_activity() {
                DummyActivity::DhtLookup { .. } => has_dht = true,
                DummyActivity::FakeHave { .. } => has_have = true,
                DummyActivity::FakeWant { .. } => has_want = true,
            }
        }
        assert!(has_dht, "Should generate DHT lookups");
        assert!(has_have, "Should generate fake HAVEs");
        assert!(has_want, "Should generate fake WANTs");
    }

    #[tokio::test]
    async fn connect_jitter_disabled_is_instant() {
        let config = PresenceConfig {
            jitter_on_connect: false,
            ..Default::default()
        };
        let start = std::time::Instant::now();
        apply_connect_jitter(&config).await;
        assert!(start.elapsed() < Duration::from_millis(50));
    }
}
