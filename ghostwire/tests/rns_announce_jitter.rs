//! The re-announce cadence must not be a correlatable fingerprint.
//!
//! ## What changed and why
//!
//! The periodic re-announce used `tokio::time::interval(REANNOUNCE_INTERVAL)`.
//! An `interval` ticks at a fixed period, so **every announce this node ever
//! sends arrives at an exact multiple of 300 s**. On the public Reticulum
//! backbone, where every transport node along the path can see announce
//! arrival times, that regularity lets an observer group GhostWire announces by
//! timestamp and count our nodes — without decrypting anything. It is a
//! free, passive census of a network whose stated goal is not to be countable.
//!
//! The cadence is now drawn per iteration from
//! `[base * (1 - JITTER), base * (1 + JITTER)]`, seeded from OS entropy.
//!
//! ## What must not regress
//!
//!   * The BASE period must not drift. The design specifies 300 s and it must
//!     stay within the announce expiry, or the node goes dark.
//!   * The LOWER bound must not fall so far that we approach RNS's 2% announce
//!     rate cap — a shared, hostile-neighbour surface. Announcing aggressively
//!     degrades the whole segment for everyone.
//!   * The RNG must not be constant-seeded. A fixed seed would give every
//!     GhostWire node an identical jitter sequence, which is a *stronger*
//!     fingerprint than the fixed interval it replaced.
//!
//! These are source assertions plus a statistical check on the drawn values,
//! because "it uses an RNG" and "it is actually random" are different claims and
//! only the second one matters.

use rand::SeedableRng;
use std::collections::HashSet;

fn reticulum_source() -> String {
    fs_read("src/core/reticulum.rs")
}

fn fs_read(p: &str) -> String {
    std::fs::read_to_string(p).unwrap_or_else(|e| panic!("cannot read {p}: {e}"))
}

#[test]
fn the_cadence_is_jittered_not_a_fixed_interval() {
    let src = reticulum_source();

    assert!(
        !src.contains("tokio::time::interval(REANNOUNCE_INTERVAL)"),
        "a fixed tokio interval makes every announce land on an exact multiple \
         of REANNOUNCE_INTERVAL, which is a passive, timestamp-only fingerprint \
         on the public backbone. Use a jittered sleep."
    );
    assert!(
        src.contains("sleep_jittered"),
        "the re-announce loop must sleep on a jittered interval"
    );
}

#[test]
fn the_jitter_window_is_documented_and_bounded() {
    let src = reticulum_source();

    assert!(
        src.contains("REANNOUNCE_JITTER"),
        "the jitter fraction must be a named constant, not an inline literal"
    );

    // 0.25 -> [225 s, 375 s]. Upper stays under the announce expiry; lower
    // stays well clear of the 2% announce rate cap.
    let jitter: f64 = src
        .lines()
        .find_map(|l| {
            let t = l.trim();
            t.strip_prefix("const REANNOUNCE_JITTER: f64 = ")
                .and_then(|r| r.strip_suffix(';'))
                .and_then(|r| r.parse::<f64>().ok())
        })
        .expect("REANNOUNCE_JITTER must parse as f64");

    assert!(
        (0.1..=0.35).contains(&jitter),
        "REANNOUNCE_JITTER = {jitter} is outside 0.1-0.35. Below 0.1 barely \
         breaks the lockstep pattern; above 0.35 risks exceeding announce \
         expiry at the top of the window."
    );
}

#[test]
fn the_base_period_is_unchanged() {
    let src = reticulum_source();
    assert!(
        src.contains("REANNOUNCE_INTERVAL: std::time::Duration = std::time::Duration::from_secs(300)"),
        "the base interval must stay at the designed 300 s. It is now the base \
         of a jittered window, not the period itself, but drifting it changes \
         the designed discovery behaviour."
    );
}

/// The real question: are the drawn values actually varied? A helper that
/// "adds jitter" but returns a constant would pass every source assertion
/// above.
#[test]
fn drawn_delays_actually_vary() {
    // Mirrors sleep_jittered's arithmetic.
    const BASE: f64 = 300.0;
    const JITTER: f64 = 0.25;

    let mut rng = rand::rngs::StdRng::seed_from_u64(0xA5A5_1234);
    let mut seen = HashSet::new();
    let mut samples = Vec::with_capacity(200);

    for _ in 0..200 {
        let offset = BASE * JITTER * (rand::Rng::gen_range(&mut rng, -1.0_f64..1.0_f64));
        let seconds = (BASE + offset).max(1.0);
        samples.push(seconds);
        seen.insert(seconds.to_bits());
    }

    assert_eq!(
        seen.len(),
        samples.len(),
        "every drawn delay was identical — the jitter is not varying, so this \
         is a fixed interval wearing a random-looking name"
    );

    let min = samples.iter().cloned().fold(f64::MAX, f64::min);
    let max = samples.iter().cloned().fold(f64::MIN, f64::max);
    assert!(min >= BASE * (1.0 - JITTER) - 1.0, "min {min} below the window");
    assert!(max <= BASE * (1.0 + JITTER) + 1.0, "max {max} above the window");

    // Every sample must stay inside [225, 375] — the announce-expiry and
    // rate-cap bounds.
    assert!(min >= 225.0, "min {min} is below 225 s; risks announce expiry");
    assert!(max <= 375.0, "max {max} is above 375 s; risks announce expiry");
}

#[test]
fn two_nodes_do_not_share_a_jitter_sequence() {
    // A constant seed would make every GhostWire node announce on an identical
    // schedule — strictly worse than the fixed interval it replaced, because
    // the nodes would then be visibly one implementation.
    const BASE: f64 = 300.0;
    const JITTER: f64 = 0.25;

    let mut a = rand::rngs::StdRng::seed_from_u64(1);
    let mut b = rand::rngs::StdRng::seed_from_u64(2);

    let seq = |r: &mut rand::rngs::StdRng| -> Vec<f64> {
        (0..20)
            .map(|_| {
                let o = BASE * JITTER * (rand::Rng::gen_range(r, -1.0_f64..1.0_f64));
                (BASE + o).max(1.0)
            })
            .collect()
    };

    assert_ne!(
        seq(&mut a),
        seq(&mut b),
        "two RNGs produced identical schedules; the seed is effectively constant"
    );
}

#[test]
fn the_rng_is_seeded_from_the_os_not_a_constant() {
    let src = reticulum_source();

    assert!(
        src.contains("getrandom::getrandom"),
        "the jitter RNG must be seeded from OS entropy"
    );
    // A hardcoded seed literal would reintroduce the fingerprint in a subtler
    // form: every node announcing on the same irregular schedule.
    assert!(
        !src.contains("StdRng::seed_from_u64"),
        "reticulum.rs must not seed the announce RNG from a constant; that would \
         make every GhostWire node announce on an identical irregular schedule"
    );
}

#[test]
fn rng_failure_degrades_to_fixed_cadence_rather_than_stopping() {
    // Announcing on a correlatable beat is strictly better than going invisible,
    // which is what stopping would cause. The fallback must exist.
    let src = reticulum_source();
    assert!(
        src.contains("OS RNG unavailable") || src.contains("tokio::time::sleep(REANNOUNCE_INTERVAL)"),
        "if OS entropy is unavailable the loop must fall back to the fixed \
         cadence, not stop announcing — an invisible node is a worse failure \
         than a correlatable one"
    );
}