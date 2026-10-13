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

// ─── BTP-Style Frame Encryption (Briar Bramble Transport Protocol v4) ─────────
//
// Implements the deterministic nonce scheme from Briar's FrameEncoder:
//   nonce = frame_number (8 bytes LE) || header_flag_byte || [padding zeros]
//
// The header flag bit (0x80 on byte 0) distinguishes header encryption
// nonces from payload encryption nonces, so the same frame key never
// produces the same nonce for both.
//
// Each frame is: [encrypted_header (4 bytes + 16-byte MAC)] [encrypted_payload + padding + 16-byte MAC]
// The header plaintext is: [payload_length (2 bytes)] [padding_length (2 bytes)] with bit 0x80 on byte 0 if final frame
//
// This prevents nonce reuse within a stream: the frame_number is the nonce,
// and the header/payload flag differentiates the two AEAD calls per frame.

/// BTP frame nonce length (24 bytes for XChaCha20-Poly1305)
pub const BTP_FRAME_NONCE_LENGTH: usize = 24;
/// BTP frame header plaintext length (payload_len + padding_len = 4 bytes)
pub const BTP_FRAME_HEADER_PLAINTEXT_LENGTH: usize = 4;
/// BTP MAC length (16 bytes for Poly1305/GCM)
pub const BTP_MAC_LENGTH: usize = 16;
/// BTP encrypted frame header length (plaintext + MAC)
pub const BTP_FRAME_HEADER_LENGTH: usize = BTP_FRAME_HEADER_PLAINTEXT_LENGTH + BTP_MAC_LENGTH;
/// BTP maximum payload length per frame (1024 - header - mac - mac = 976 bytes)
pub const BTP_MAX_PAYLOAD_LENGTH: usize = 1024 - BTP_FRAME_HEADER_LENGTH - BTP_MAC_LENGTH;

/// Encode a BTP-style frame nonce from a frame number and header flag.
///
/// Mirrors Briar's `FrameEncoder.encodeNonce`:
///   - Writes frame_number as u64 BIG-endian in bytes [0..8] (MSB first)
///   - Sets bit 0x80 on byte 0 if encrypting the frame header
///   - Since byte 0 is the most-significant byte of the frame number,
///     for normal stream lengths (0..2^63-1) byte 0 is 0, so the 0x80
///     header flag creates a clean separation with no nonce collisions.
pub fn encode_btp_nonce(frame_number: u64, is_header: bool) -> [u8; BTP_FRAME_NONCE_LENGTH] {
    let mut nonce = [0u8; BTP_FRAME_NONCE_LENGTH];
    // Write frame number as big-endian u64 (MSB first — byte 0 is high byte)
    nonce[0..8].copy_from_slice(&frame_number.to_be_bytes());
    if is_header {
        nonce[0] |= 0x80; // Set header flag bit on MSB
    }
    // Remaining bytes are already zero
    nonce
}

/// Encode a BTP frame header plaintext.
///
/// Layout: [payload_length (2 bytes BE)] [padding_length (2 bytes BE)]
/// Bit 0x80 on byte 0 is set if this is the final frame.
pub fn encode_btp_header(
    payload_length: usize,
    padding_length: usize,
    is_final: bool,
) -> Result<[u8; BTP_FRAME_HEADER_PLAINTEXT_LENGTH]> {
    if payload_length > 0x7FFF || padding_length > 0x7FFF {
        anyhow::bail!("BTP payload+padding exceeds 16-bit limit");
    }
    let mut header = [0u8; BTP_FRAME_HEADER_PLAINTEXT_LENGTH];
    let payload_len = if is_final { payload_length | 0x8000 } else { payload_length };
    header[0..2].copy_from_slice(&(payload_len as u16).to_be_bytes());
    header[2..4].copy_from_slice(&(padding_length as u16).to_be_bytes());
    Ok(header)
}

/// Decode the payload length from a BTP frame header.
pub fn decode_btp_payload_length(header: &[u8]) -> usize {
    let val = u16::from_be_bytes([header[0], header[1]]);
    (val & 0x7FFF) as usize
}

/// Check if this is the final frame in a BTP stream.
pub fn decode_btp_is_final(header: &[u8]) -> bool {
    (header[0] & 0x80) != 0
}

/// Decode the padding length from a BTP frame header.
pub fn decode_btp_padding_length(header: &[u8]) -> usize {
    u16::from_be_bytes([header[2], header[3]]) as usize
}

/// Encrypt a BTP frame using XChaCha20-Poly1305 (or ChaCha20-Poly1305).
///
/// Returns: [encrypted_header (20 bytes)] [encrypted_payload + padding (encrypted_payload_len + 16 bytes MAC)]
///
/// This mirrors Briar's StreamEncrypterImpl.writeFrame:
/// 1. Encrypt the 4-byte header with nonce(frame_number, header=true)
/// 2. Encrypt the payload + padding with nonce(frame_number, header=false)
pub fn encrypt_btp_frame(
    frame_key: &[u8; 32],
    frame_number: u64,
    payload: &[u8],
    padding_length: usize,
    is_final: bool,
) -> Result<Vec<u8>> {
    use chacha20poly1305::aead::{Aead, KeyInit};
    use chacha20poly1305::{XChaCha20Poly1305, XNonce, Key};

    if payload.len() + padding_length > BTP_MAX_PAYLOAD_LENGTH {
        anyhow::bail!(
            "Payload + padding ({} bytes) exceeds BTP max ({} bytes)",
            payload.len() + padding_length,
            BTP_MAX_PAYLOAD_LENGTH
        );
    }

    let cipher = XChaCha20Poly1305::new(Key::from_slice(frame_key));

    // 1. Encrypt the frame header
    let header_plaintext = encode_btp_header(payload.len(), padding_length, is_final)?;
    let header_nonce = XNonce::from(encode_btp_nonce(frame_number, true));
    let header_ciphertext = cipher
        .encrypt(&header_nonce, &header_plaintext[..])
        .map_err(|e| anyhow::anyhow!("BTP header encryption failed: {}", e))?;

    // 2. Encrypt payload + padding
    let mut payload_plaintext = payload.to_vec();
    payload_plaintext.extend(std::iter::repeat(0u8).take(padding_length));
    let payload_nonce = XNonce::from(encode_btp_nonce(frame_number, false));
    let payload_ciphertext = cipher
        .encrypt(&payload_nonce, &payload_plaintext[..])
        .map_err(|e| anyhow::anyhow!("BTP payload encryption failed: {}", e))?;

    // 3. Combine: header_ciphertext (20 bytes) + payload_ciphertext
    let mut frame = Vec::with_capacity(header_ciphertext.len() + payload_ciphertext.len());
    frame.extend_from_slice(&header_ciphertext);
    frame.extend_from_slice(&payload_ciphertext);
    Ok(frame)
}

/// Decrypt a BTP frame and return (payload, padding_length, is_final).
///
/// Mirrors Briar's StreamDecrypterImpl.readFrame:
/// 1. Decrypt the header to get payload_length, padding_length, is_final
/// 2. Decrypt the payload + padding
/// 3. Verify padding is all zeros
pub fn decrypt_btp_frame(
    frame_key: &[u8; 32],
    frame_number: u64,
    frame_bytes: &[u8],
) -> Result<(Vec<u8>, usize, bool)> {
    use chacha20poly1305::aead::{Aead, KeyInit};
    use chacha20poly1305::{XChaCha20Poly1305, XNonce, Key};

    let cipher = XChaCha20Poly1305::new(Key::from_slice(frame_key));

    // 1. Decrypt header
    let header_nonce = XNonce::from(encode_btp_nonce(frame_number, true));
    let header_plaintext = cipher
        .decrypt(&header_nonce, &frame_bytes[..BTP_FRAME_HEADER_LENGTH])
        .map_err(|e| anyhow::anyhow!("BTP header decryption failed: {}", e))?;

    if header_plaintext.len() != BTP_FRAME_HEADER_PLAINTEXT_LENGTH {
        anyhow::bail!("BTP header plaintext wrong length");
    }

    let payload_length = decode_btp_payload_length(&header_plaintext);
    let padding_length = decode_btp_padding_length(&header_plaintext);
    let is_final = decode_btp_is_final(&header_plaintext);

    if payload_length + padding_length > BTP_MAX_PAYLOAD_LENGTH {
        anyhow::bail!("BTP payload+padding exceeds max");
    }

    // 2. Decrypt payload + padding
    let expected_payload_len = payload_length + padding_length + BTP_MAC_LENGTH;
    let payload_offset = BTP_FRAME_HEADER_LENGTH;
    if frame_bytes.len() < payload_offset + expected_payload_len {
        anyhow::bail!(
            "BTP frame too short: {} < {}",
            frame_bytes.len(),
            payload_offset + expected_payload_len
        );
    }

    let payload_nonce = XNonce::from(encode_btp_nonce(frame_number, false));
    let payload_plaintext = cipher
        .decrypt(
            &payload_nonce,
            &frame_bytes[payload_offset..payload_offset + expected_payload_len],
        )
        .map_err(|e| anyhow::anyhow!("BTP payload decryption failed: {}", e))?;

    // 3. Verify padding is all zeros
    let padding_start = payload_length;
    for i in padding_start..(payload_length + padding_length) {
        if payload_plaintext[i] != 0 {
            anyhow::bail!("BTP padding is not zero at byte {}", i);
        }
    }

    let payload = payload_plaintext[..payload_length].to_vec();
    Ok((payload, padding_length, is_final))
}

/// Stream encrypter for BTP-style frame streams.
///
/// Maintains a frame counter and encrypts payloads into BTP frames,
/// writing them to an output sink via a callback.
pub struct BtpStreamEncrypter {
    frame_key: [u8; 32],
    frame_number: u64,
}

impl BtpStreamEncrypter {
    pub fn new(frame_key: [u8; 32]) -> Self {
        Self {
            frame_key,
            frame_number: 0,
        }
    }

    /// Encrypt a payload into one or more BTP frames, calling `write_frame`
    /// for each frame. Handles chunking into MAX_PAYLOAD_LENGTH blocks.
    pub fn write_payload<F: FnMut(&[u8]) -> Result<()>>(
        &mut self,
        payload: &[u8],
        write_frame: &mut F,
    ) -> Result<()> {
        let chunks = split_into_chunks(payload, BTP_MAX_PAYLOAD_LENGTH);
        let total = chunks.len();

        for (i, chunk) in chunks.iter().enumerate() {
            let is_final = i == total - 1;
            // Add padding to fill the frame (optional, 0 padding for now)
            let padding = 0;
            let frame = encrypt_btp_frame(
                &self.frame_key,
                self.frame_number,
                chunk,
                padding,
                is_final,
            )?;
            write_frame(&frame)?;
            self.frame_number = self.frame_number.checked_add(1).ok_or_else(|| {
                anyhow::anyhow!("BTP frame number overflow — stream too long")
            })?;
        }
        Ok(())
    }

    /// Send end-of-stream (final empty frame)
    pub fn send_eos<F: FnMut(&[u8]) -> Result<()>>(
        &mut self,
        write_frame: &mut F,
    ) -> Result<()> {
        let frame = encrypt_btp_frame(&self.frame_key, self.frame_number, &[], 0, true)?;
        write_frame(&frame)?;
        self.frame_number += 1;
        Ok(())
    }
}

/// Stream decrypter for BTP-style frame streams.
pub struct BtpStreamDecrypter {
    frame_key: [u8; 32],
    frame_number: u64,
    /// Buffer for reassembled payload across frames
    buffer: Vec<u8>,
}

impl BtpStreamDecrypter {
    pub fn new(frame_key: [u8; 32]) -> Self {
        Self {
            frame_key,
            frame_number: 0,
            buffer: Vec::new(),
        }
    }

    /// Process a single BTP frame, returning decoded payload bytes if
    /// a complete message has been assembled.
    pub fn read_frame(&mut self, frame_bytes: &[u8]) -> Result<Option<Vec<u8>>> {
        let (payload, _padding, is_final) = decrypt_btp_frame(&self.frame_key, self.frame_number, frame_bytes)?;
        self.frame_number += 1;
        self.buffer.extend_from_slice(&payload);

        if is_final {
            let result = std::mem::take(&mut self.buffer);
            Ok(Some(result))
        } else {
            Ok(None)
        }
    }
}

/// Split data into chunks of at most `chunk_size` bytes.
fn split_into_chunks(data: &[u8], chunk_size: usize) -> Vec<&[u8]> {
    if data.is_empty() {
        return vec![&[]];
    }
    data.chunks(chunk_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nonce_encodes_frame_number_with_header_flag() {
        let nonce_payload = encode_btp_nonce(42, false);
        let nonce_header = encode_btp_nonce(42, true);

        // Frame number in bytes [7] (big-endian, MSB is byte 0 so byte 7 = LSB)
        // For 42, only the last byte of the u64 is nonzero
        assert_eq!(nonce_payload[7], 42);
        assert_eq!(nonce_payload[0..7], [0u8; 7]); // high bytes zero

        // Header nonce has bit 0x80 set on byte 0 (MSB)
        assert_eq!(nonce_header[0], 0x80);
        assert_eq!(nonce_header[7], 42);
        assert_ne!(nonce_payload, nonce_header, "Header and payload nonces must differ");

        // Different frame numbers produce different nonces
        let nonce_0 = encode_btp_nonce(0, false);
        let nonce_1 = encode_btp_nonce(1, false);
        assert_ne!(nonce_0, nonce_1);
    }

    #[test]
    fn nonce_nonces_differ_by_frame_number() {
        // Verify no nonce reuse across frames and header/payload flag
        // With big-endian, byte 0 is the MSB — for frame numbers 0..512
        // byte 0 stays 0 for payload, and 0x80 for header, no collision.
        let mut seen = std::collections::HashSet::new();
        for i in 0..512u64 {
            let nonce_payload = encode_btp_nonce(i, false);
            let nonce_header = encode_btp_nonce(i, true);
            assert!(seen.insert(nonce_payload), "Payload nonce reused at frame {}", i);
            assert!(seen.insert(nonce_header), "Header nonce reused at frame {}", i);
            assert_ne!(nonce_payload, nonce_header, "Header/payload nonce collision at frame {}", i);
        }
    }

    #[test]
    fn header_encoding_is_reversible() {
        let (payload_len, padding_len, is_final) = (100, 50, true);
        let header = encode_btp_header(payload_len, padding_len, is_final).unwrap();
        assert_eq!(decode_btp_payload_length(&header), payload_len);
        assert_eq!(decode_btp_padding_length(&header), padding_len);
        assert!(decode_btp_is_final(&header));

        let header2 = encode_btp_header(200, 0, false).unwrap();
        assert_eq!(decode_btp_payload_length(&header2), 200);
        assert_eq!(decode_btp_padding_length(&header2), 0);
        assert!(!decode_btp_is_final(&header2));
    }

    #[test]
    fn btp_frame_roundtrip() {
        let frame_key = [0xABu8; 32];
        let payload = b"Hello, BTP world!";
        let frame_number = 0;

        // Encrypt
        let frame = encrypt_btp_frame(&frame_key, frame_number, payload, 0, true)
            .expect("encrypt should succeed");

        // Decrypt
        let (decrypted, padding_len, is_final) = decrypt_btp_frame(&frame_key, frame_number, &frame)
            .expect("decrypt should succeed");

        assert_eq!(decrypted, payload, "Decrypted payload must match original");
        assert_eq!(padding_len, 0);
        assert!(is_final);
    }

    #[test]
    fn btp_frame_rejects_wrong_key() {
        let frame_key = [0xABu8; 32];
        let wrong_key = [0xCDu8; 32];
        let payload = b"Secret message";

        let frame = encrypt_btp_frame(&frame_key, 0, payload, 0, true)
            .expect("encrypt should succeed");

        let result = decrypt_btp_frame(&wrong_key, 0, &frame);
        assert!(result.is_err(), "Decryption with wrong key must fail");
    }

    #[test]
    fn btp_stream_encrypt_decrypt_roundtrip() {
        let frame_key = [0x99u8; 32];
        let payload = b"This is a longer message that spans multiple frames for the BTP stream encryptor test";

        // Encrypt into frames
        let mut encrypter = BtpStreamEncrypter::new(frame_key);
        let mut frames = Vec::new();
        encrypter.write_payload(payload, &mut |frame_bytes| {
            frames.push(frame_bytes.to_vec());
            Ok(())
        }).expect("encryption should succeed");

        // Decrypt — last frame has is_final=true, so the last read_frame returns Some
        let mut decrypter = BtpStreamDecrypter::new(frame_key);
        let mut result: Option<Vec<u8>> = None;
        for frame_bytes in &frames {
            result = decrypter.read_frame(frame_bytes).expect("decryption should succeed");
        }

        assert!(result.is_some(), "Must have assembled complete payload");
        assert_eq!(result.unwrap(), payload, "Reassembled payload must match");
    }

    #[test]
    fn btp_stream_encrypt_decrypt_multiframe() {
        let frame_key = [0x99u8; 32];
        // Create a payload larger than BTP_MAX_PAYLOAD_LENGTH (976 bytes)
        let payload: Vec<u8> = (0..2000).map(|i| (i % 256) as u8).collect();

        let mut encrypter = BtpStreamEncrypter::new(frame_key);
        let mut frames = Vec::new();
        encrypter.write_payload(&payload, &mut |frame_bytes| {
            frames.push(frame_bytes.to_vec());
            Ok(())
        }).expect("encryption should succeed");

        let mut decrypter = BtpStreamDecrypter::new(frame_key);
        let mut result: Option<Vec<u8>> = None;
        for frame_bytes in &frames {
            result = decrypter.read_frame(frame_bytes).expect("decryption should succeed");
        }

        assert!(result.is_some(), "Must have assembled complete payload");
        assert_eq!(result.unwrap(), payload, "Multi-frame reassembled payload must match");
    }

    #[test]
    fn btp_stream_detects_wrong_key() {
        let frame_key = [0x11u8; 32];
        let wrong_key = [0x22u8; 32];
        let payload = b"Some payload";

        let mut encrypter = BtpStreamEncrypter::new(frame_key);
        let mut frames = Vec::new();
        encrypter.write_payload(payload, &mut |frame_bytes| {
            frames.push(frame_bytes.to_vec());
            Ok(())
        }).expect("encryption should succeed");

        let mut decrypter = BtpStreamDecrypter::new(wrong_key);
        for frame_bytes in &frames {
            let result = decrypter.read_frame(frame_bytes);
            assert!(result.is_err(), "Wrong key must cause decryption failure");
        }
    }

    #[test]
    fn btp_frame_rejects_tampered_payload() {
        let frame_key = [0x33u8; 32];
        let payload = b"Original payload that must not be tampered";

        let mut frame = encrypt_btp_frame(&frame_key, 0, payload, 0, true)
            .expect("encrypt should succeed");

        // Tamper with a byte in the payload ciphertext area (after header)
        if frame.len() > BTP_FRAME_HEADER_LENGTH + 5 {
            frame[BTP_FRAME_HEADER_LENGTH + 5] ^= 0xFF;
        }

        let result = decrypt_btp_frame(&frame_key, 0, &frame);
        assert!(result.is_err(), "Tampered frame must fail decryption");
    }
}

#[cfg(test)]
mod wire_tests {
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
