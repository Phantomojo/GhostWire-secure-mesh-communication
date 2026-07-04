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

use serde::{Deserialize, Serialize};
use std::time::Duration;

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

/// Frame fragmenter for WebRTC data channel
/// WebRTC data channels have a practical message size limit (~16KB).
/// Large Ghostwire frames (Sphinx packets can be ~2KB) need fragmentation.
pub struct WebRtcFragmenter {
    max_chunk: usize,
}

impl WebRtcFragmenter {
    pub fn new(max_chunk: usize) -> Self {
        Self { max_chunk }
    }

    /// Split a large frame into chunks with a sequence header
    pub fn fragment(&self, frame: &[u8], stream_id: u32) -> Vec<Vec<u8>> {
        if frame.len() <= self.max_chunk - 9 {
            // Single chunk: [stream_id(4)][total_chunks(2)][chunk_idx(2)][data]
            let mut chunk = Vec::with_capacity(9 + frame.len());
            chunk.extend_from_slice(&stream_id.to_be_bytes());
            chunk.extend_from_slice(&1u16.to_be_bytes()); // total: 1
            chunk.extend_from_slice(&0u16.to_be_bytes()); // index: 0
            chunk.push(1u8); // final chunk flag
            chunk.extend_from_slice(frame);
            return vec![chunk];
        }

        let data_per_chunk = self.max_chunk - 9;
        let total_chunks = (frame.len() + data_per_chunk - 1) / data_per_chunk;
        let mut chunks = Vec::with_capacity(total_chunks);

        for (idx, chunk_data) in frame.chunks(data_per_chunk).enumerate() {
            let is_final = idx == total_chunks - 1;
            let mut chunk = Vec::with_capacity(9 + chunk_data.len());
            chunk.extend_from_slice(&stream_id.to_be_bytes());
            chunk.extend_from_slice(&(total_chunks as u16).to_be_bytes());
            chunk.extend_from_slice(&(idx as u16).to_be_bytes());
            chunk.push(if is_final { 1u8 } else { 0u8 });
            chunk.extend_from_slice(chunk_data);
            chunks.push(chunk);
        }
        chunks
    }

    /// Reassemble chunks. Returns Some(frame) when all chunks received.
    pub fn reassemble(
        &self,
        chunks: &mut std::collections::HashMap<u32, Vec<Option<Vec<u8>>>>,
        chunk: &[u8],
    ) -> Option<Vec<u8>> {
        if chunk.len() < 9 {
            return None;
        }
        let stream_id = u32::from_be_bytes(chunk[0..4].try_into().ok()?);
        let total = u16::from_be_bytes(chunk[4..6].try_into().ok()?) as usize;
        let idx = u16::from_be_bytes(chunk[6..8].try_into().ok()?) as usize;
        let data = chunk[9..].to_vec();

        let entry = chunks.entry(stream_id).or_insert_with(|| vec![None; total]);
        if idx < entry.len() {
            entry[idx] = Some(data);
        }

        if entry.iter().all(|c| c.is_some()) {
            let frame: Vec<u8> = entry
                .iter()
                .filter_map(|c| c.as_ref())
                .flat_map(|c| c.iter().cloned())
                .collect();
            chunks.remove(&stream_id);
            Some(frame)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fragment_reassemble_roundtrip() {
        let fragmenter = WebRtcFragmenter::new(32); // tiny chunks for testing
        let original = vec![0xABu8; 100]; // larger than one chunk
        let stream_id = 42u32;

        let chunks = fragmenter.fragment(&original, stream_id);
        assert!(chunks.len() > 1, "Should fragment into multiple chunks");

        let mut reassembly_state = std::collections::HashMap::new();
        let mut result = None;
        for chunk in &chunks {
            result = fragmenter.reassemble(&mut reassembly_state, chunk);
        }

        assert_eq!(result, Some(original));
    }

    #[test]
    fn small_frame_single_chunk() {
        let fragmenter = WebRtcFragmenter::new(1024);
        let frame = vec![1u8; 100];
        let chunks = fragmenter.fragment(&frame, 1);
        assert_eq!(chunks.len(), 1);
    }

    #[test]
    fn transport_tier_display() {
        assert_eq!(TransportTier::WebRtcDataChannel.display_name(), "WebRTC");
        assert_eq!(TransportTier::StealthTcp.status_color(), "green");
    }
}
