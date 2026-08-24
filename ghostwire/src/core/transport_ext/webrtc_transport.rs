//! webrtc_transport.rs — Tier 2 DPI-resistant transport
//!
//! Makes Ghostwire traffic look like legitimate WebRTC peer-to-peer
//! connections (the same protocol used by Google Meet, Discord, Jitsi).
//!
//! Why this is powerful: blocking WebRTC means blocking all video calling.
//! No government blocks video calling without enormous collateral damage.
//! Even Iran and China have not fully blocked WebRTC.
//!
//! Architecture:
//!   - libp2p-webrtc is already in Cargo.toml (alpha dep)
//!   - Ghostwire frames are carried inside WebRTC data channels
//!   - ICE negotiation uses standard STUN/TURN servers
//!   - From a DPI sensor: looks identical to a Discord call
//!
//! Auto-escalation: this tier activates automatically when stealth TCP
//! is blocked (detected by the AutoTransportSwitcher in transport_manager.rs)

use ring::hmac;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// WebRTC transport configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WebRtcTransportConfig {
    /// STUN servers for ICE negotiation (use standard public servers)
    pub stun_servers: Vec<String>,
    /// TURN relay servers (optional, for symmetric NAT traversal)
    pub turn_servers: Vec<TurnConfig>,
    /// Whether to use the steganography sub-layer (Tier 3)
    pub enable_stego_layer: bool,
    /// Data channel label (appears in WebRTC inspection as a normal channel)
    pub data_channel_label: String,
    /// Maximum message size for fragmentation
    pub max_chunk_bytes: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TurnConfig {
    pub url: String,
    pub username: String,
    pub credential: String,
}

impl Default for WebRtcTransportConfig {
    fn default() -> Self {
        Self {
            stun_servers: vec![
                "stun:stun.l.google.com:19302".into(),
                "stun:stun1.l.google.com:19302".into(),
            ],
            turn_servers: vec![],
            enable_stego_layer: false,
            data_channel_label: "data".into(), // Generic label — doesn't say "ghostwire"
            max_chunk_bytes: 16384,
        }
    }
}

/// Transport mode tier — determined by auto-detection
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportTier {
    /// Tier 1: stealth TCP (traffic looks like TLS but not any known protocol)
    StealthTcp,
    /// Tier 2: WebRTC data channel (traffic looks like video calls)
    WebRtcDataChannel,
    /// Tier 3: WebRTC video steganography (traffic IS a video call, data hidden in VP8)
    WebRtcStego,
    /// Tier 4: HTTPS domain fronting (traffic looks like HTTPS to a CDN)
    HttpsFronting,
    /// Fallback: Bluetooth / LoRa (no internet required)
    OfflineOnly,
}

impl TransportTier {
    /// Human-readable description for status bar display
    pub fn display_name(&self) -> &'static str {
        match self {
            Self::StealthTcp => "Stealth",
            Self::WebRtcDataChannel => "WebRTC",
            Self::WebRtcStego => "Covert",
            Self::HttpsFronting => "Fronted",
            Self::OfflineOnly => "Offline",
        }
    }

    /// Color for the UI status indicator
    pub fn status_color(&self) -> &'static str {
        match self {
            Self::StealthTcp => "green",
            Self::WebRtcDataChannel => "teal",
            Self::WebRtcStego => "purple",
            Self::HttpsFronting => "amber",
            Self::OfflineOnly => "gray",
        }
    }
}

/// Probes connectivity to determine which tier to use.
/// Called by AutoTransportSwitcher every 10 minutes and on connection failure.
pub async fn probe_and_select_tier(target_peers: &[String]) -> TransportTier {
    // Probe tier 1: stealth TCP
    if probe_stealth_tcp(target_peers).await {
        tracing::debug!("Transport probe: Tier 1 (stealth TCP) available");
        return TransportTier::StealthTcp;
    }
    tracing::info!("Transport probe: Tier 1 blocked, escalating to Tier 2 (WebRTC)");

    // Probe tier 2: WebRTC
    if probe_webrtc_data_channel().await {
        tracing::info!("Transport probe: Tier 2 (WebRTC) available");
        return TransportTier::WebRtcDataChannel;
    }
    tracing::warn!("Transport probe: Tier 2 blocked, escalating to Tier 4 (HTTPS fronting)");

    // Probe tier 4: HTTPS fronting
    if probe_https_fronting().await {
        tracing::warn!(
            "Transport probe: Tier 4 (HTTPS fronting) active — deep censorship detected"
        );
        return TransportTier::HttpsFronting;
    }

    // Last resort: offline only
    tracing::error!("Transport probe: All internet tiers blocked — offline mode");
    TransportTier::OfflineOnly
}

async fn probe_stealth_tcp(peers: &[String]) -> bool {
    // Attempt a stealth TCP connection to any known peer
    // In production: try connecting to bootstrap nodes via stealth TCP
    // For now: check if we can open a TCP connection at all
    for peer in peers.iter().take(3) {
        let addr = format!("{}:8080", peer);
        if tokio::time::timeout(
            Duration::from_secs(5),
            tokio::net::TcpStream::connect(&addr),
        )
        .await
        .is_ok()
        {
            return true;
        }
    }
    // Default to true if no peers to probe — assume stealth TCP works
    peers.is_empty()
}

async fn probe_webrtc_data_channel() -> bool {
    // Attempt a STUN binding request to Google's STUN server
    // If STUN works, WebRTC will work
    use tokio::net::UdpSocket;
    match UdpSocket::bind("0.0.0.0:0").await {
        Ok(sock) => {
            // STUN binding request (RFC 5389)
            let stun_request = build_stun_binding_request();
            let target = "74.125.250.129:19302"; // stun.l.google.com
            if sock.send_to(&stun_request, target).await.is_ok() {
                let mut buf = [0u8; 512];
                match tokio::time::timeout(Duration::from_secs(5), sock.recv_from(&mut buf)).await {
                    Ok(Ok((n, _))) if n >= 20 => {
                        // Check STUN magic cookie (0x2112A442) at bytes 4-8
                        n >= 8 && buf[4..8] == [0x21, 0x12, 0xA4, 0x42]
                    }
                    _ => false,
                }
            } else {
                false
            }
        }
        Err(_) => false,
    }
}

async fn probe_https_fronting() -> bool {
    // Try an HTTPS request that looks like a CDN call
    // In production: hit a known CDN endpoint that fronts a Ghostwire relay
    // For now: check basic HTTPS connectivity
    match tokio::time::timeout(
        Duration::from_secs(10),
        reqwest::get("https://www.cloudflare.com/cdn-cgi/trace"),
    )
    .await
    {
        Ok(Ok(resp)) => resp.status().is_success(),
        _ => false,
    }
}

fn build_stun_binding_request() -> Vec<u8> {
    // Minimal STUN Binding Request (RFC 5389)
    let mut req = vec![
        0x00, 0x01, // Message Type: Binding Request
        0x00, 0x00, // Message Length: 0 (no attributes)
        0x21, 0x12, 0xA4, 0x42, // Magic Cookie
    ];
    // Transaction ID: 12 random bytes
    req.extend_from_slice(&[
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C,
    ]);
    req
}

/// Frame fragmenter for WebRTC data channels.
///
/// Every fragment is authenticated with a session-specific HMAC-SHA256 key.
/// The MAC covers the stream, sequence, logical length, whole-message digest,
/// and fragment payload. This prevents fragment substitution and reordering
/// from being accepted as a valid logical message. The outer session still
/// needs its normal AEAD; this MAC is not a replacement for E2EE.
pub struct WebRtcFragmenter {
    max_chunk: usize,
    auth_key: Option<[u8; 32]>,
}

const FRAGMENT_MAGIC: &[u8; 4] = b"GWF1";
const FRAGMENT_VERSION: u8 = 1;
const FRAGMENT_HEADER_LEN: usize = 4 + 1 + 4 + 2 + 2 + 1 + 4 + 32;
const FRAGMENT_TAG_LEN: usize = 32;
const MAX_FRAGMENT_COUNT: usize = 1024;
const MAX_REASSEMBLY_BYTES: usize = 16 * 1024 * 1024;
const REASSEMBLY_TIMEOUT: Duration = Duration::from_secs(30);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FragmentError {
    InvalidConfiguration(&'static str),
    AuthenticationRequired,
    Malformed(&'static str),
    BoundsExceeded(&'static str),
    AuthenticationFailed,
    ConflictingDuplicate,
}

impl std::fmt::Display for FragmentError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidConfiguration(message)
            | Self::Malformed(message)
            | Self::BoundsExceeded(message) => f.write_str(message),
            Self::AuthenticationRequired => f.write_str("fragment authentication key required"),
            Self::AuthenticationFailed => f.write_str("fragment authentication failed"),
            Self::ConflictingDuplicate => f.write_str("conflicting duplicate fragment"),
        }
    }
}

impl std::error::Error for FragmentError {}

#[derive(Debug)]
pub struct ReassemblyState {
    total: usize,
    logical_len: usize,
    digest: [u8; 32],
    fragments: Vec<Option<Vec<u8>>>,
    created_at: Instant,
}

impl WebRtcFragmenter {
    pub fn new(max_chunk: usize) -> Self {
        Self {
            max_chunk,
            auth_key: None,
        }
    }

    pub fn new_authenticated(max_chunk: usize, auth_key: [u8; 32]) -> Result<Self, FragmentError> {
        if max_chunk <= FRAGMENT_HEADER_LEN + FRAGMENT_TAG_LEN {
            return Err(FragmentError::InvalidConfiguration(
                "fragment chunk size leaves no payload capacity",
            ));
        }
        Ok(Self {
            max_chunk,
            auth_key: Some(auth_key),
        })
    }

    pub fn fragment(&self, frame: &[u8], stream_id: u32) -> Result<Vec<Vec<u8>>, FragmentError> {
        let auth_key = self
            .auth_key
            .as_ref()
            .ok_or(FragmentError::AuthenticationRequired)?;
        let data_per_chunk = self
            .max_chunk
            .checked_sub(FRAGMENT_HEADER_LEN + FRAGMENT_TAG_LEN)
            .ok_or(FragmentError::InvalidConfiguration("invalid fragment size"))?;
        if frame.is_empty() {
            return Err(FragmentError::Malformed("empty logical frame"));
        }
        if frame.len() > MAX_REASSEMBLY_BYTES {
            return Err(FragmentError::BoundsExceeded("logical frame exceeds limit"));
        }
        let total_chunks = frame.len().div_ceil(data_per_chunk);
        if total_chunks == 0
            || total_chunks > MAX_FRAGMENT_COUNT
            || total_chunks > u16::MAX as usize
        {
            return Err(FragmentError::BoundsExceeded(
                "fragment count exceeds limit",
            ));
        }

        let digest: [u8; 32] = Sha256::digest(frame).into();
        let mut chunks = Vec::with_capacity(total_chunks);
        for (idx, chunk_data) in frame.chunks(data_per_chunk).enumerate() {
            let final_flag = u8::from(idx + 1 == total_chunks);
            let mut chunk = Vec::with_capacity(self.max_chunk);
            chunk.extend_from_slice(FRAGMENT_MAGIC);
            chunk.push(FRAGMENT_VERSION);
            chunk.extend_from_slice(&stream_id.to_be_bytes());
            chunk.extend_from_slice(&(total_chunks as u16).to_be_bytes());
            chunk.extend_from_slice(&(idx as u16).to_be_bytes());
            chunk.push(final_flag);
            chunk.extend_from_slice(&(frame.len() as u32).to_be_bytes());
            chunk.extend_from_slice(&digest);
            chunk.extend_from_slice(chunk_data);
            let tag = self.mac(auth_key, &chunk);
            chunk.extend_from_slice(&tag);
            chunks.push(chunk);
        }
        Ok(chunks)
    }

    pub fn reassemble(
        &self,
        chunks: &mut HashMap<u32, ReassemblyState>,
        chunk: &[u8],
    ) -> Result<Option<Vec<u8>>, FragmentError> {
        let auth_key = self
            .auth_key
            .as_ref()
            .ok_or(FragmentError::AuthenticationRequired)?;
        if chunk.len() > self.max_chunk || chunk.len() < FRAGMENT_HEADER_LEN + FRAGMENT_TAG_LEN {
            return Err(FragmentError::Malformed("fragment length is invalid"));
        }
        if &chunk[..4] != FRAGMENT_MAGIC || chunk[4] != FRAGMENT_VERSION {
            return Err(FragmentError::Malformed(
                "fragment magic or version is invalid",
            ));
        }
        let stream_id = u32::from_be_bytes(chunk[5..9].try_into().unwrap());
        let total = u16::from_be_bytes(chunk[9..11].try_into().unwrap()) as usize;
        let idx = u16::from_be_bytes(chunk[11..13].try_into().unwrap()) as usize;
        let final_flag = chunk[13];
        let logical_len = u32::from_be_bytes(chunk[14..18].try_into().unwrap()) as usize;
        let digest: [u8; 32] = chunk[18..50].try_into().unwrap();
        let payload_end = chunk.len() - FRAGMENT_TAG_LEN;
        let payload = &chunk[50..payload_end];
        let tag = &chunk[payload_end..];

        if total == 0 || total > MAX_FRAGMENT_COUNT || idx >= total {
            return Err(FragmentError::BoundsExceeded(
                "fragment sequence is invalid",
            ));
        }
        if logical_len == 0 || logical_len > MAX_REASSEMBLY_BYTES {
            return Err(FragmentError::BoundsExceeded(
                "logical frame length is invalid",
            ));
        }
        let data_per_chunk = self.max_chunk - FRAGMENT_HEADER_LEN - FRAGMENT_TAG_LEN;
        let expected_total = logical_len.div_ceil(data_per_chunk);
        if total != expected_total {
            return Err(FragmentError::Malformed(
                "fragment count does not match logical length",
            ));
        }
        if final_flag != u8::from(idx + 1 == total) {
            return Err(FragmentError::Malformed("invalid final-fragment flag"));
        }
        if idx + 1 < total && payload.len() != data_per_chunk {
            return Err(FragmentError::Malformed(
                "non-final fragment has invalid length",
            ));
        }
        if idx + 1 == total && (payload.is_empty() || payload.len() > data_per_chunk) {
            return Err(FragmentError::Malformed(
                "final fragment has invalid length",
            ));
        }
        if !hmac::verify(
            &hmac::Key::new(hmac::HMAC_SHA256, auth_key),
            &chunk[..payload_end],
            tag,
        )
        .is_ok()
        {
            return Err(FragmentError::AuthenticationFailed);
        }

        chunks.retain(|_, state| state.created_at.elapsed() <= REASSEMBLY_TIMEOUT);
        let state = chunks.entry(stream_id).or_insert_with(|| ReassemblyState {
            total,
            logical_len,
            digest,
            fragments: vec![None; total],
            created_at: Instant::now(),
        });
        if state.total != total || state.logical_len != logical_len || state.digest != digest {
            chunks.remove(&stream_id);
            return Err(FragmentError::ConflictingDuplicate);
        }
        if let Some(existing) = &state.fragments[idx] {
            if existing.as_slice() != payload {
                chunks.remove(&stream_id);
                return Err(FragmentError::ConflictingDuplicate);
            }
            return Ok(None);
        }
        state.fragments[idx] = Some(payload.to_vec());

        if state.fragments.iter().all(Option::is_some) {
            let frame: Vec<u8> = state
                .fragments
                .iter()
                .flat_map(|fragment| fragment.as_deref().unwrap_or_default())
                .copied()
                .collect();
            if frame.len() != state.logical_len
                || Sha256::digest(&frame).as_slice() != state.digest.as_slice()
            {
                chunks.remove(&stream_id);
                return Err(FragmentError::Malformed("logical frame digest mismatch"));
            }
            chunks.remove(&stream_id);
            Ok(Some(frame))
        } else {
            Ok(None)
        }
    }

    fn mac(&self, auth_key: &[u8; 32], authenticated_header_and_payload: &[u8]) -> [u8; 32] {
        let tag = hmac::sign(
            &hmac::Key::new(hmac::HMAC_SHA256, auth_key),
            authenticated_header_and_payload,
        );
        tag.as_ref()
            .try_into()
            .expect("HMAC-SHA256 tag is 32 bytes")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fragment_reassemble_roundtrip() {
        let fragmenter = WebRtcFragmenter::new_authenticated(128, [7u8; 32]).unwrap();
        let original = vec![0xABu8; 100];
        let stream_id = 42u32;

        let chunks = fragmenter.fragment(&original, stream_id).unwrap();
        assert!(chunks.len() > 1, "Should fragment into multiple chunks");

        let mut reassembly_state = HashMap::new();
        let mut result = None;
        for chunk in &chunks {
            result = fragmenter.reassemble(&mut reassembly_state, chunk).unwrap();
        }

        assert_eq!(result, Some(original));
    }

    #[test]
    fn small_frame_single_chunk() {
        let fragmenter = WebRtcFragmenter::new_authenticated(128, [7u8; 32]).unwrap();
        let frame = vec![1u8; 10];
        let chunks = fragmenter.fragment(&frame, 1).unwrap();
        assert_eq!(chunks.len(), 1);
    }

    #[test]
    fn tampering_and_wrong_key_are_rejected() {
        let fragmenter = WebRtcFragmenter::new_authenticated(128, [7u8; 32]).unwrap();
        let wrong_key = WebRtcFragmenter::new_authenticated(128, [8u8; 32]).unwrap();
        let original = vec![3u8; 20];
        let mut chunk = fragmenter.fragment(&original, 9).unwrap().remove(0);
        let last = chunk.len() - 1;
        chunk[last] ^= 1;

        let mut state = HashMap::new();
        assert_eq!(
            fragmenter.reassemble(&mut state, &chunk),
            Err(FragmentError::AuthenticationFailed)
        );
        let valid = fragmenter.fragment(&original, 9).unwrap().remove(0);
        assert_eq!(
            wrong_key.reassemble(&mut state, &valid),
            Err(FragmentError::AuthenticationFailed)
        );
    }

    #[test]
    fn conflicting_duplicate_is_rejected() {
        let fragmenter = WebRtcFragmenter::new_authenticated(128, [7u8; 32]).unwrap();
        let original = vec![4u8; 100];
        let chunks = fragmenter.fragment(&original, 10).unwrap();
        let mut state = HashMap::new();
        assert_eq!(fragmenter.reassemble(&mut state, &chunks[0]).unwrap(), None);
        let mut conflicting = chunks[0].clone();
        let payload_index = FRAGMENT_HEADER_LEN;
        conflicting[payload_index] ^= 1;
        let tag = fragmenter.mac(
            &[7u8; 32],
            &conflicting[..conflicting.len() - FRAGMENT_TAG_LEN],
        );
        let tag_start = conflicting.len() - FRAGMENT_TAG_LEN;
        conflicting[tag_start..].copy_from_slice(&tag);
        assert_eq!(
            fragmenter.reassemble(&mut state, &conflicting),
            Err(FragmentError::ConflictingDuplicate)
        );
    }

    #[test]
    fn transport_tier_display() {
        assert_eq!(TransportTier::WebRtcDataChannel.display_name(), "WebRTC");
        assert_eq!(TransportTier::StealthTcp.status_color(), "green");
    }
}
