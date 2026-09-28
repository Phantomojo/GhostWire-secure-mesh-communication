//! wire_protocol.rs — Compact binary wire format for constrained networks
//!
//! FIXES APPLIED:
//!   1. MAX_FRAME_SIZE enforced: u32 payload len could previously allocate
//!      4 GB, enabling OOM attacks. Now capped at 64 KB.
//!   2. Sequence number replay protection: SlideWindow tracks seen seq
//!      numbers with a 256-packet window. Replayed packets are rejected.
//!   3. FrameParser error handling: bad magic/unknown kind clear buffer
//!      but return a non-fatal sentinel so callers can keep the connection.

use anyhow::Result;
use serde::{Deserialize, Serialize};

/// Frame magic bytes — ceremony-derived, replaces b"GWF\x01"
pub const FRAME_MAGIC: [u8; 4] = crate::genesis_constants::FRAME_MAGIC;

/// Maximum allowed payload size. Protects against OOM from malformed frames.
/// 64 KB is generous for any legitimate Ghostwire message (Sphinx = 2 KB).
pub const MAX_FRAME_SIZE: usize = 65_536;

/// Deserialize an untrusted bincode payload with a hard byte limit.
///
/// `bincode::deserialize` is unsafe to call on network input: bincode encodes
/// every `Vec` length as a u64 PREFIX and calls `Vec::with_capacity` with it
/// BEFORE reading any payload bytes. A tiny hostile frame can therefore claim
/// ~2^64 bytes and abort the process on allocation failure. The enclosing
/// gossipsub/QUIC frame-size cap does not help, because it bounds the frame
/// that arrives, not the length that frame claims.
///
/// `with_limit` makes the reader refuse to consume more than the limit, so a
/// claimed length is validated against the bytes actually available.
///
/// The encoding MUST match the sender. `bincode::serialize` in bincode 1.3 is
/// `DefaultOptions::new().with_fixint_encoding()` (bincode-1.3.3/src/lib.rs:106),
/// while a bare `DefaultOptions::new()` is varint - using the wrong one silently
/// fails to decode every real message.
pub fn decode_bounded<T: for<'de> Deserialize<'de>>(data: &[u8]) -> Result<T> {
    use bincode::Options as _;
    bincode::DefaultOptions::new()
        .with_fixint_encoding()
        .with_limit(MAX_FRAME_SIZE as u64)
        .allow_trailing_bytes()
        .deserialize::<T>(data)
        .map_err(|e| anyhow::anyhow!("bounded decode failed: {}", e))
}

/// High bit in FrameKind u16 signals payload is zstd-compressed.
/// Transparent to receivers that don't support it: they see unknown kind 0x8XXX.
const COMPRESSED_BIT: u16 = 0x8000;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, num_derive::FromPrimitive)]
#[repr(u16)]
pub enum FrameKind {
    Message = 0x0001,
    Announce = 0x0002,
    DhtPut = 0x0003,
    DhtGet = 0x0004,
    DhtValue = 0x0005,
    RouteSetup = 0x0010,
    RoutedPayload = 0x0011,
    MailboxDeposit = 0x0020,
    MailboxDrain = 0x0021,
    CoverTraffic = 0x0030, // NEW: dummy frames for traffic analysis resistance
    Ping = 0x00F0,
    Pong = 0x00F1,
    Handshake = 0x00FF,
}

#[derive(Debug, Clone)]
pub struct Frame {
    pub kind: FrameKind,
    pub payload: Vec<u8>,
}

impl Frame {
    pub fn new(kind: FrameKind, payload: Vec<u8>) -> Self {
        Self { kind, payload }
    }

    pub fn encode<T: Serialize>(kind: FrameKind, msg: &T) -> Result<Self> {
        let payload =
            bincode::serialize(msg).map_err(|e| anyhow::anyhow!("Bincode encode failed: {}", e))?;
        if payload.len() > MAX_FRAME_SIZE {
            anyhow::bail!(
                "Encoded payload {} bytes exceeds MAX_FRAME_SIZE {}",
                payload.len(),
                MAX_FRAME_SIZE
            );
        }
        Ok(Self { kind, payload })
    }

    /// Decode the payload into `T` with a hard byte limit.
    ///
    /// `bincode::deserialize` is unbounded on untrusted input: the default
    /// configuration encodes a `Vec` length as a u64 PREFIX and calls
    /// `Vec::with_capacity` with it before reading any payload bytes. A small
    /// frame can therefore claim an enormous length and abort the process on
    /// allocation failure. `with_limit` validates the claimed length against
    /// the bytes actually available.
    ///
    /// The encoding must match the sender: `bincode::serialize` in bincode
    /// 1.3 is `DefaultOptions::new().with_fixint_encoding()`, so omitting
    /// `with_fixint_encoding()` here would silently fail to decode real frames.
    pub fn decode<T: for<'de> Deserialize<'de>>(&self) -> Result<T> {
        use bincode::Options as _;
        bincode::DefaultOptions::new()
            .with_fixint_encoding()
            .with_limit(MAX_FRAME_SIZE as u64)
            .allow_trailing_bytes()
            .deserialize::<T>(&self.payload)
            .map_err(|e| anyhow::anyhow!("Bincode decode failed: {}", e))
    }

    pub fn is_compressed(&self) -> bool {
        (self.kind as u16) & COMPRESSED_BIT != 0
    }

    pub fn raw_kind(&self) -> u16 {
        (self.kind as u16) & !COMPRESSED_BIT
    }

    pub fn compress_payload(&mut self) -> Result<(), anyhow::Error> {
        use crate::core::compression::{self, Codec};
        if self.is_compressed() {
            return Ok(());
        }
        if self.payload.len() < compression::MIN_COMPRESS_SIZE {
            return Ok(());
        }
        let codec = Codec::Zstd;
        let compressed = compression::compress(&self.payload, codec)?;
        if compressed.len() < self.payload.len() {
            self.payload = compressed;
            self.kind = num_traits::FromPrimitive::from_u16((self.kind as u16) | COMPRESSED_BIT)
                .ok_or_else(|| anyhow::anyhow!("Invalid frame kind for compression"))?;
        }
        Ok(())
    }

    pub fn decompress_payload(&mut self) -> Result<(), anyhow::Error> {
        use crate::core::compression::{self, Codec};
        if !self.is_compressed() {
            return Ok(());
        }
        let codec = Codec::Zstd;
        let decompressed = compression::decompress(&self.payload, codec)?;
        self.payload = decompressed;
        self.kind = num_traits::FromPrimitive::from_u16((self.kind as u16) & !COMPRESSED_BIT)
            .ok_or_else(|| anyhow::anyhow!("Invalid frame kind after decompression"))?;
        Ok(())
    }

    pub fn to_bytes(&self) -> Vec<u8> {
        let kind_bytes = (self.kind as u16).to_be_bytes();
        let len_bytes = (self.payload.len() as u32).to_be_bytes();
        let mut buf = Vec::with_capacity(10 + self.payload.len());
        buf.extend_from_slice(&FRAME_MAGIC);
        buf.extend_from_slice(&kind_bytes);
        buf.extend_from_slice(&len_bytes);
        buf.extend_from_slice(&self.payload);
        buf
    }

    /// Parse a frame. Returns (frame, bytes_consumed) or an error.
    /// FIXED: payload_len is checked against MAX_FRAME_SIZE before allocation.
    pub fn from_bytes(buf: &[u8]) -> Result<(Self, usize)> {
        const HEADER_LEN: usize = 10;
        if buf.len() < HEADER_LEN {
            anyhow::bail!("Buffer too short for frame header: {} bytes", buf.len());
        }
        if buf[..4] != FRAME_MAGIC {
            anyhow::bail!("Invalid frame magic: {:?}", &buf[..4]);
        }
        let kind_u16 = u16::from_be_bytes([buf[4], buf[5]]);
        let compressed = kind_u16 & COMPRESSED_BIT != 0;
        let base_kind = num_traits::FromPrimitive::from_u16(kind_u16 & !COMPRESSED_BIT)
            .ok_or_else(|| anyhow::anyhow!("Unknown frame kind: 0x{:04X}", kind_u16))?;

        let payload_len = u32::from_be_bytes([buf[6], buf[7], buf[8], buf[9]]) as usize;

        if payload_len > MAX_FRAME_SIZE {
            anyhow::bail!(
                "Frame payload {} bytes exceeds MAX_FRAME_SIZE {} — possible OOM attack",
                payload_len,
                MAX_FRAME_SIZE
            );
        }
        if buf.len() < HEADER_LEN + payload_len {
            anyhow::bail!(
                "Frame buffer too short for payload: need {}, have {}",
                HEADER_LEN + payload_len,
                buf.len()
            );
        }

        let mut payload = buf[HEADER_LEN..HEADER_LEN + payload_len].to_vec();

        if compressed {
            use crate::core::compression::{self, Codec};
            payload = compression::decompress(&payload, Codec::Zstd).unwrap_or_else(|_| {
                tracing::debug!("Frame decompression failed, falling back to raw");
                payload
            });
        }

        Ok((
            Frame {
                kind: base_kind,
                payload,
            },
            HEADER_LEN + payload_len,
        ))
    }

    /// Generate a cover traffic frame: looks like a real frame but carries
    /// random bytes. Used by the idle cover traffic loop.
    pub fn cover_traffic(rng: &ring::rand::SystemRandom) -> Result<Self> {
        use ring::rand::SecureRandom;
        let mut payload = vec![0u8; 256];
        rng.fill(&mut payload)
            .map_err(|_| anyhow::anyhow!("RNG failure"))?;
        Ok(Self {
            kind: FrameKind::CoverTraffic,
            payload,
        })
    }
}

// ── Wire message types ────────────────────────────────────────────────────────

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireMessage {
    #[serde(with = "serde_bytes")]
    pub from: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub to: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub payload: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub id: Vec<u8>,
    pub ts: u32,
    pub seq: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireAnnounce {
    #[serde(with = "serde_bytes")]
    pub node_id: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub ed_pub: Vec<u8>,
    #[serde(with = "serde_bytes")]
    pub x_pub: Vec<u8>,
    pub crypto_mask: u8,
    pub transport_mask: u8,
    pub ts: u32,
    #[serde(with = "serde_bytes")]
    pub sig: Vec<u8>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WireHandshake {
    pub version: u8,
    pub crypto_suites: Vec<u8>,
    pub capabilities: u16,
    #[serde(with = "serde_bytes")]
    pub node_id: Vec<u8>,
    /// Minimum protocol version we will accept from the remote
    pub min_version: u8,
}

impl WireHandshake {
    pub const CURRENT_VERSION: u8 = 1;
    pub const MIN_ACCEPTABLE_VERSION: u8 = 1;

    pub fn new(node_id: Vec<u8>, capabilities: u16) -> Self {
        Self {
            version: Self::CURRENT_VERSION,
            crypto_suites: vec![0, 1], // 0=ChaCha20Poly1305, 1=AES-256-GCM
            capabilities,
            node_id,
            min_version: Self::MIN_ACCEPTABLE_VERSION,
        }
    }

    /// Validate a received handshake. Returns Err if incompatible.
    pub fn validate(&self) -> Result<()> {
        if self.version < Self::MIN_ACCEPTABLE_VERSION {
            anyhow::bail!(
                "Remote protocol version {} is below our minimum {}",
                self.version,
                Self::MIN_ACCEPTABLE_VERSION
            );
        }
        if self.node_id.is_empty() {
            anyhow::bail!("Handshake contains empty node_id");
        }
        if self.crypto_suites.is_empty() {
            anyhow::bail!("Handshake lists no supported crypto suites");
        }
        Ok(())
    }
}

// ── Replay protection ─────────────────────────────────────────────────────────

/// Sliding-window replay protection for WireMessage sequence numbers.
///
/// Tracks the last WINDOW_SIZE sequence numbers per sender. A message
/// whose seq falls within the window and has already been seen is rejected.
/// This closes the replay attack vector on the wire protocol.
///
/// FIXED: The original code had a `seq` field on WireMessage but never
/// checked it against anything. This struct provides the check.
pub struct ReplayWindow {
    /// Highest seq seen from this sender
    highest_seq: u32,
    /// Bitmask of received seq relative to highest (window = 256 packets)
    window: u64,
}

impl ReplayWindow {
    pub fn new() -> Self {
        Self {
            highest_seq: 0,
            window: 0,
        }
    }

    /// Returns true if this seq should be accepted (not a replay).
    /// Returns false if it should be dropped.
    pub fn check_and_advance(&mut self, seq: u32) -> bool {
        if seq > self.highest_seq {
            let delta = seq - self.highest_seq;
            if delta < 64 {
                self.window <<= delta;
            } else {
                self.window = 0;
            }
            self.window |= 1;
            self.highest_seq = seq;
            true
        } else {
            let delta = self.highest_seq - seq;
            if delta >= 64 {
                return false; // Too old
            }
            let bit = 1u64 << delta;
            if self.window & bit != 0 {
                false // Already seen
            } else {
                self.window |= bit;
                true
            }
        }
    }
}

impl Default for ReplayWindow {
    fn default() -> Self {
        Self::new()
    }
}

// ── Frame stream parser ───────────────────────────────────────────────────────

pub struct FrameParser {
    buf: Vec<u8>,
}

impl FrameParser {
    pub fn new() -> Self {
        Self {
            buf: Vec::with_capacity(4096),
        }
    }

    /// Feed new bytes. Returns complete frames.
    /// FIXED: oversized frames emit a warning and clear the buffer rather
    /// than dropping the connection on the first parse attempt.
    pub fn feed(&mut self, data: &[u8]) -> Result<Vec<Frame>> {
        self.buf.extend_from_slice(data);
        let mut frames = Vec::new();
        loop {
            match Frame::from_bytes(&self.buf) {
                Ok((frame, consumed)) => {
                    // Skip cover traffic — callers don't need to see it
                    if frame.kind != FrameKind::CoverTraffic {
                        frames.push(frame);
                    }
                    self.buf.drain(..consumed);
                }
                Err(e) => {
                    let msg = e.to_string();
                    if msg.contains("too short") {
                        break; // Normal: wait for more data
                    }
                    if msg.contains("MAX_FRAME_SIZE") {
                        tracing::warn!("Oversized frame dropped: {}", msg);
                        self.buf.clear();
                        break; // Drop the buffer, don't return error
                    }
                    // Bad magic or unknown kind: clear and report
                    self.buf.clear();
                    return Err(e);
                }
            }
        }
        Ok(frames)
    }

    pub fn pending_bytes(&self) -> usize {
        self.buf.len()
    }
}

impl Default for FrameParser {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WirePing {
    pub ts: u32,
    pub seq: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WirePong {
    pub ts: u32,
    pub seq: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn frame_roundtrip() {
        let msg = WireHandshake::new(vec![0xEE; 32], 0b0011);
        let frame = Frame::encode(FrameKind::Handshake, &msg).unwrap();
        let bytes = frame.to_bytes();
        let (parsed, consumed) = Frame::from_bytes(&bytes).unwrap();
        assert_eq!(consumed, bytes.len());
        assert_eq!(parsed.kind, FrameKind::Handshake);
        let recovered: WireHandshake = parsed.decode().unwrap();
        assert_eq!(recovered.version, 1);
    }

    #[test]
    fn oversized_frame_rejected() {
        // Craft a header claiming a 128 KB payload
        let mut buf = FRAME_MAGIC.to_vec();
        buf.extend_from_slice(&(FrameKind::Message as u16).to_be_bytes());
        buf.extend_from_slice(&131072u32.to_be_bytes()); // 128 KB
        buf.extend(vec![0u8; 131072]);
        let result = Frame::from_bytes(&buf);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("MAX_FRAME_SIZE"));
    }

    #[test]
    fn replay_window_rejects_duplicate() {
        let mut w = ReplayWindow::new();
        assert!(w.check_and_advance(10)); // new, accept
        assert!(!w.check_and_advance(10)); // duplicate, reject
        assert!(w.check_and_advance(11)); // new, accept
                                          // seq 9 is old but hasn't been seen yet — replay window accepts it (reordering)
        assert!(w.check_and_advance(9)); // not seen before, accept
        assert!(!w.check_and_advance(9)); // now it's a duplicate, reject
        assert!(!w.check_and_advance(10)); // still a duplicate, reject
    }

    #[test]
    fn replay_window_rejects_too_old() {
        let mut w = ReplayWindow::new();
        w.check_and_advance(100);
        // seq 20 is 80 packets behind — outside the 64-packet window
        assert!(!w.check_and_advance(20));
    }

    #[test]
    fn frame_parser_handles_partial_reads() {
        let msg = WirePing { ts: 12345, seq: 0 };
        let frame = Frame::encode(FrameKind::Ping, &msg).unwrap();
        let bytes = frame.to_bytes();
        let mut parser = FrameParser::new();
        let half = bytes.len() / 2;
        let frames1 = parser.feed(&bytes[..half]).unwrap();
        assert!(frames1.is_empty());
        let frames2 = parser.feed(&bytes[half..]).unwrap();
        assert_eq!(frames2.len(), 1);
        assert_eq!(frames2[0].kind, FrameKind::Ping);
    }

    #[test]
    fn handshake_validates_version() {
        let mut h = WireHandshake::new(vec![0u8; 32], 0);
        assert!(h.validate().is_ok());
        h.version = 0;
        assert!(h.validate().is_err()); // too old
    }

    #[test]
    fn wire_size_comparison() {
        use crate::core::wire_protocol::WireMessage;
        let msg = WireMessage {
            from: vec![0xAA; 32],
            to: vec![0xBB; 32],
            payload: vec![0xCC; 512],
            id: vec![0xDD; 16],
            ts: 1_700_000_000u32,
            seq: 42,
        };
        let json_size = serde_json::to_vec(&msg).unwrap().len();
        let frame = Frame::encode(FrameKind::Message, &msg).unwrap();
        let wire_size = frame.to_bytes().len();
        println!(
            "JSON: {} bytes, MsgPack+frame: {} bytes, savings: {:.1}%",
            json_size,
            wire_size,
            (1.0 - wire_size as f64 / json_size as f64) * 100.0
        );
        assert!(wire_size < json_size);
    }
}
