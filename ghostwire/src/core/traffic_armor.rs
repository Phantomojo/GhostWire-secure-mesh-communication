//! traffic_armor.rs — Traffic analysis resistance
//!
//! Implements two complementary defenses borrowed from academic mixnet research:
//!
//!   1. **Message padding** — all messages padded to fixed 512-byte blocks
//!      before encryption. Eliminates message length as a side channel.
//!
//!   2. **Cover traffic** — when a session is idle, periodically send
//!      indistinguishable dummy packets. An adversary cannot tell whether
//!      silence means no messages or just no *real* messages.
//!
//! Veilid lesson: their research paper (IEEE CSR 2024) flagged that Veilid
//! still leaks timing and size information. We implement what they haven't yet.

use crate::core::armor::DeviceTier;
use anyhow::Result;
use rand::Rng;
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio::sync::RwLock;
use zeroize::Zeroize;

pub const PAD_BLOCK_SIZE: usize = 512;
pub const MAX_PADDED_SIZE: usize = 65_536;

/// A padded, typed message frame. The `is_cover` flag is encrypted — the
/// receiver discards cover frames silently; observers see identical frames.
#[derive(Debug, Clone, Serialize, Deserialize, Zeroize)]
pub struct PaddedFrame {
    /// Always `PAD_BLOCK_SIZE` aligned. Includes trailing random padding.
    pub data: Vec<u8>,
    /// Encrypted flag: if true, receiver silently discards after decryption.
    pub is_cover: bool,
    /// Monotonic counter (per-session, wraps) for replay detection.
    pub sequence: u32,
}

/// Pad `plaintext` to the next multiple of `PAD_BLOCK_SIZE`.
///
/// Padding format (PKCS7-inspired but fixed-block):
///   [plaintext][0x01 marker][random bytes][2-byte LE length of plaintext]
///
/// The 2-byte length at the end lets the receiver unpad exactly.
pub fn pad_message(plaintext: &[u8], rng: &SystemRandom) -> Result<Vec<u8>> {
    let content_len = plaintext.len();
    if content_len + 3 > MAX_PADDED_SIZE {
        anyhow::bail!("Message too large to pad: {} bytes", content_len);
    }

    // Next block boundary
    let raw_size = content_len + 3; // +3 for marker + 2-byte length
    let padded_size = ((raw_size + PAD_BLOCK_SIZE - 1) / PAD_BLOCK_SIZE) * PAD_BLOCK_SIZE;
    let pad_len = padded_size - raw_size;

    let mut buf = Vec::with_capacity(padded_size);
    buf.extend_from_slice(plaintext);
    buf.push(0x01); // marker byte

    // Random padding
    let mut padding = vec![0u8; pad_len];
    rng.fill(&mut padding)
        .map_err(|_| anyhow::anyhow!("RNG failure in pad_message"))?;
    buf.extend_from_slice(&padding);

    // 2-byte LE length of original plaintext
    buf.push((content_len & 0xFF) as u8);
    buf.push(((content_len >> 8) & 0xFF) as u8);

    debug_assert_eq!(buf.len() % PAD_BLOCK_SIZE, 0);
    Ok(buf)
}

/// Recover the original plaintext from a padded buffer.
pub fn unpad_message(padded: &[u8]) -> Result<Vec<u8>> {
    if padded.len() < 3 || padded.len() % PAD_BLOCK_SIZE != 0 {
        anyhow::bail!("Invalid padded message length: {}", padded.len());
    }

    let lo = padded[padded.len() - 2] as usize;
    let hi = padded[padded.len() - 1] as usize;
    let content_len = lo | (hi << 8);

    if content_len + 3 > padded.len() {
        anyhow::bail!(
            "Corrupt padded message: claimed length {} > buffer {}",
            content_len,
            padded.len()
        );
    }

    // Verify marker byte
    if padded[content_len] != 0x01 {
        anyhow::bail!("Corrupt padded message: missing marker byte");
    }

    Ok(padded[..content_len].to_vec())
}

/// Generate a random-looking cover frame of the same block-aligned size.
pub fn generate_cover_frame(rng: &SystemRandom, sequence: u32) -> Result<PaddedFrame> {
    let mut data = vec![0u8; PAD_BLOCK_SIZE];
    rng.fill(&mut data)
        .map_err(|_| anyhow::anyhow!("RNG failure in cover frame"))?;

    Ok(PaddedFrame {
        data,
        is_cover: true,
        sequence,
    })
}

// ── Cover traffic scheduler ──────────────────────────────────────────────────

/// Manages the cover traffic loop for a single session.
///
/// Usage:
/// ```rust
/// let (cover_tx, mut cover_rx) = mpsc::channel(32);
/// let scheduler = CoverTrafficScheduler::new(cover_tx);
/// scheduler.start().await;
///
/// // In your send loop:
/// tokio::select! {
///     Some(frame) = cover_rx.recv() => { send_frame(frame).await; }
///     Some(real_msg) = msg_rx.recv() => { send_real(real_msg).await; }
/// }
/// ```
pub struct CoverTrafficScheduler {
    tx: mpsc::Sender<PaddedFrame>,
    enabled: Arc<RwLock<bool>>,
    rng: SystemRandom,
    tier: DeviceTier,
}

impl CoverTrafficScheduler {
    pub fn new(tx: mpsc::Sender<PaddedFrame>, tier: DeviceTier) -> Self {
        Self {
            tx,
            enabled: Arc::new(RwLock::new(true)),
            rng: SystemRandom::new(),
            tier,
        }
    }

    pub async fn enable(&self) {
        *self.enabled.write().await = true;
    }

    pub async fn disable(&self) {
        *self.enabled.write().await = false;
    }

    /// Spawn the cover traffic loop. Call once per session.
    pub fn start(self: Arc<Self>) {
        let (_, _, _, cover_min_ms, cover_max_ms) = DeviceTier::params(self.tier);
        tokio::spawn(async move {
            let mut sequence: u32 = 0;
            loop {
                let delay_ms = rand::rng().random_range(cover_min_ms..=cover_max_ms);
                tokio::time::sleep(Duration::from_millis(delay_ms)).await;

                if !*self.enabled.read().await {
                    continue;
                }

                match generate_cover_frame(&self.rng, sequence) {
                    Ok(frame) => {
                        sequence = sequence.wrapping_add(1);
                        // Non-blocking send — if channel is full, drop the cover frame
                        // (real messages take priority)
                        let _ = self.tx.try_send(frame);
                    }
                    Err(e) => {
                        tracing::warn!("Cover frame generation failed: {}", e);
                    }
                }
            }
        });
    }
}

// ── TrafficArmor — integrated interface ─────────────────────────────────────

/// High-level interface: wraps a message with padding and tracks cover traffic.
///
/// This replaces the stub `TrafficArmor` in `src/core/armor.rs`.
pub struct TrafficArmor {
    rng: SystemRandom,
    sequence: Arc<RwLock<u32>>,
    cover_scheduler: Option<Arc<CoverTrafficScheduler>>,
}

impl TrafficArmor {
    pub fn new(cover_tx: Option<mpsc::Sender<PaddedFrame>>, tier: DeviceTier) -> Self {
        let cover_scheduler = cover_tx.map(|tx| {
            let s = Arc::new(CoverTrafficScheduler::new(tx, tier));
            s.clone().start();
            s
        });

        Self {
            rng: SystemRandom::new(),
            sequence: Arc::new(RwLock::new(0)),
            cover_scheduler,
        }
    }

    /// Wrap plaintext into a padded frame ready for encryption.
    pub async fn wrap(&self, plaintext: &[u8]) -> Result<PaddedFrame> {
        let padded = pad_message(plaintext, &self.rng)?;
        let mut seq = self.sequence.write().await;
        let frame = PaddedFrame {
            data: padded,
            is_cover: false,
            sequence: *seq,
        };
        *seq = seq.wrapping_add(1);
        Ok(frame)
    }

    /// Unwrap a received padded frame. Returns None for cover frames.
    pub fn unwrap(&self, frame: &PaddedFrame) -> Result<Option<Vec<u8>>> {
        if frame.is_cover {
            return Ok(None); // Silently discard
        }
        let plaintext = unpad_message(&frame.data)?;
        Ok(Some(plaintext))
    }

    /// Temporarily suppress cover traffic (e.g. during high-volume send bursts)
    pub async fn suppress_cover(&self) {
        if let Some(s) = &self.cover_scheduler {
            s.disable().await;
        }
    }

    pub async fn resume_cover(&self) {
        if let Some(s) = &self.cover_scheduler {
            s.enable().await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pad_unpad_roundtrip() {
        let rng = SystemRandom::new();
        let case3 = vec![0xAB; 511];
        let case4 = vec![0xCD; 512];
        let case5 = vec![0xEF; 513];
        let case6 = vec![0x00; 1024];
        let cases: Vec<&[u8]> = vec![
            b"",
            b"hello",
            b"exactly 512 bytes of padding test input right here, let us see how this works out",
            &case3,
            &case4,
            &case5,
            &case6,
        ];

        for original in cases {
            let padded = pad_message(original, &rng).unwrap();
            assert_eq!(padded.len() % PAD_BLOCK_SIZE, 0, "Not block-aligned");
            let recovered = unpad_message(&padded).unwrap();
            assert_eq!(
                recovered,
                original,
                "Roundtrip failed for len={}",
                original.len()
            );
        }
    }

    #[test]
    fn cover_frame_is_block_aligned() {
        let rng = SystemRandom::new();
        let frame = generate_cover_frame(&rng, 0).unwrap();
        assert_eq!(frame.data.len() % PAD_BLOCK_SIZE, 0);
        assert!(frame.is_cover);
    }

    #[test]
    fn device_tier_params_ordering() {
        let (h_min, h_max, _, h_cover_min, h_cover_max) = DeviceTier::High.params();
        let (m_min, m_max, _, m_cover_min, m_cover_max) = DeviceTier::Medium.params();
        let (l_min, l_max, _, l_cover_min, l_cover_max) = DeviceTier::Low.params();
        assert!(h_min > m_min && m_min > l_min);
        assert!(h_cover_min > m_cover_min && m_cover_min > l_cover_min);
        assert!(h_cover_max > m_cover_max && m_cover_max > l_cover_max);
    }

    #[tokio::test]
    async fn traffic_armor_roundtrip() {
        let armor = TrafficArmor::new(None, DeviceTier::Medium);
        let msg = b"Ubuntu - I am because we are";
        let frame = armor.wrap(msg).await.unwrap();
        let recovered = armor.unwrap(&frame).unwrap().unwrap();
        assert_eq!(recovered, msg);
    }

    #[tokio::test]
    async fn cover_frame_returns_none() {
        let armor = TrafficArmor::new(None, DeviceTier::Medium);
        let rng = SystemRandom::new();
        let cover = generate_cover_frame(&rng, 0).unwrap();
        let result = armor.unwrap(&cover).unwrap();
        assert!(result.is_none());
    }
}
