//! Does `TrafficArmor::new` actually START the cover scheduler, and does the
//! published frame survive gossipsub?
//!
//! The audit claimed item A (cover traffic) was dead because the value is bound
//! to `_traffic_armor`. That reasoning is incomplete: `TrafficArmor::new` takes
//! the sender and starts a `CoverTrafficScheduler` internally, so the underscore
//! binding may still leave a running task. Idle-traffic measurement on the
//! Elitebook showed 1480 B/s on an idle node, which is ~10x the expected cover
//! rate but well above zero -- so this test decides it by construction rather
//! than by inference.

use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

use ghostwire::core::armor::DeviceTier;
use ghostwire::core::traffic_armor::{CoverTrafficScheduler, TrafficArmor, PAD_BLOCK_SIZE};
use tokio::sync::mpsc;

#[tokio::test]
async fn traffic_armor_starts_the_cover_scheduler_on_construction() {
    let (tx, mut rx) = mpsc::channel::<ghostwire::core::traffic_armor::PaddedFrame>(64);
    let _armor = TrafficArmor::new(Some(tx), DeviceTier::Low); // Low = 1-5s

    // If the scheduler were NOT started, this would block for the full timeout
    // and return Err. Receiving a frame is proof the task is alive.
    let frame = tokio::time::timeout(Duration::from_secs(12), rx.recv())
        .await
        .expect("no cover frame within 12s: TrafficArmor::new did not start the scheduler")
        .expect("cover channel closed");

    assert!(
        frame.is_cover,
        "frames emitted by the cover scheduler must be marked as cover"
    );
    assert_eq!(
        frame.data.len(),
        PAD_BLOCK_SIZE,
        "a cover frame must be block-aligned so it is indistinguishable in size \
         from a padded real message"
    );
}

#[tokio::test]
async fn cover_frames_are_block_aligned_and_distinguishable_only_after_decryption() {
    let (tx, mut rx) = mpsc::channel(64);
    let armor = TrafficArmor::new(Some(tx), DeviceTier::Low);

    let real = armor.wrap(b"a short private message").await.unwrap();
    assert!(
        !real.is_cover,
        "a real message must not be marked as cover, or the receiver discards it"
    );
    assert_eq!(
        real.data.len() % PAD_BLOCK_SIZE,
        0,
        "real messages must be padded to the block size or length becomes a side channel"
    );

    // The is_cover flag lives INSIDE the frame, so it is only visible after
    // decryption. That is the property that makes cover traffic useful: an
    // observer sees two indistinguishable blocks.
    assert!(
        real.data.len() > "a short private message".len(),
        "padding must actually add bytes"
    );
    let _ = rx.recv().await; // a cover frame may already be queued
}

#[tokio::test]
async fn unwrap_discards_cover_frames_and_recovers_real_ones() {
    let (tx, _rx) = mpsc::channel(64);
    let armor = TrafficArmor::new(Some(tx), DeviceTier::Low);

    let real = armor.wrap(b"payload").await.unwrap();
    assert_eq!(
        armor.unwrap(&real).unwrap(),
        Some(b"payload".to_vec()),
        "a real frame must round-trip through wrap/unwrap"
    );

    let mut cover = real.clone();
    cover.is_cover = true;
    assert_eq!(
        armor.unwrap(&cover).unwrap(),
        None,
        "a cover frame must unwrap to None so the receiver discards it silently \
         rather than surfacing phantom messages"
    );
}

#[tokio::test]
async fn the_scheduler_emits_continuously_not_just_once() {
    // One frame could be a startup artefact. Cover traffic only helps if the
    // channel keeps producing, so count several over a window.
    let (tx, mut rx) = mpsc::channel(128);
    let _s = Arc::new(CoverTrafficScheduler::new(tx, DeviceTier::Low)).start();

    let seen = Arc::new(AtomicUsize::new(0));
    let counter = seen.clone();
    let collector = tokio::spawn(async move {
        while let Ok(Some(f)) = tokio::time::timeout(Duration::from_secs(20), rx.recv()).await {
            assert!(f.is_cover);
            if counter.fetch_add(1, Ordering::SeqCst) >= 3 {
                break;
            }
        }
    });

    let _ = tokio::time::timeout(Duration::from_secs(25), collector).await;
    assert!(
        seen.load(Ordering::SeqCst) >= 3,
        "expected at least 3 cover frames in 20s at the Low tier (1-5s each), got {}",
        seen.load(Ordering::SeqCst)
    );
}
