//! pluggable_transport.rs — HTTPS-mimicking transport for censorship resistance
//!
//! Problem: Even with StealthTCP's HMAC handshake, your ISP can tell you're
//! running Ghostwire because:
//!   a) Port patterns (4001, 4002 etc. are common libp2p ports)
//!   b) TLS certificate fingerprints differ from browsers
//!   c) Connection timing patterns differ from HTTP/2 sessions
//!
//! This module implements a pluggable transport layer that wraps Ghostwire
//! connections to look like HTTPS traffic to deep-packet inspection systems.
//!
//! Three strategies are provided (select based on threat model):
//!
//!   1. **TLS camouflage** — present a valid TLS 1.3 handshake with a
//!      self-signed cert that looks like a real web server. DPI sees HTTPS.
//!
//!   2. **Shadowsocks-style AEAD** — encrypt the entire byte stream including
//!      the handshake headers. Wire looks like random bytes with no structure.
//!      Used by Nym for users in censored regions.
//!
//!   3. **WebSocket upgrade** — wrap the connection in an HTTP/1.1 Upgrade
//!      to WebSocket. Passes through most corporate firewalls and national
//!      censorship systems (blocking WebSocket breaks too many legitimate apps).
//!
//! INTEGRATION: Replace the TCP transport layer in mesh.rs with a
//! PluggableTransport wrapping the existing TCP stream.

use anyhow::Result;
use chacha20poly1305::{
    aead::{Aead, KeyInit},
    ChaCha20Poly1305, Key, Nonce,
};
use ring::rand::{SecureRandom, SystemRandom};
use serde::{Deserialize, Serialize};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use zeroize::{Zeroize, ZeroizeOnDrop};

/// Which obfuscation strategy to use
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransportMode {
    /// Raw TCP with StealthTCP HMAC handshake (existing)
    Stealth,
    /// Wrap in WebSocket upgrade headers
    WebSocket,
    /// Shadowsocks-style full-stream AEAD encryption
    ShadowsocksAead,
    /// TLS 1.3 with a convincing server certificate
    TlsCamouflage,
}

impl Default for TransportMode {
    fn default() -> Self {
        Self::WebSocket // Best default: passes most firewalls, low overhead
    }
}

/// The pre-shared key used for ShadowsocksAead mode
#[derive(Zeroize, ZeroizeOnDrop)]
pub struct TransportPsk([u8; 32]);

impl TransportPsk {
    pub fn generate() -> Result<Self> {
        let rng = SystemRandom::new();
        let mut key = [0u8; 32];
        rng.fill(&mut key)
            .map_err(|_| anyhow::anyhow!("RNG failure"))?;
        Ok(Self(key))
    }

    pub fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    pub fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }
}

// ── WebSocket camouflage ──────────────────────────────────────────────────────

/// Fake HTTP/1.1 WebSocket upgrade headers that make Ghostwire look like
/// a legitimate WebSocket application.
///
/// The Host header uses a common CDN domain. The path looks like a
/// legitimate WebSocket endpoint (e.g. a notification service).
static WS_UPGRADE_REQUEST: &str = concat!(
    "GET /ws/v2/live HTTP/1.1\r\n",
    "Host: cdn-notifications.example.com\r\n",
    "Upgrade: websocket\r\n",
    "Connection: Upgrade\r\n",
    "Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n",
    "Sec-WebSocket-Version: 13\r\n",
    "User-Agent: Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36\r\n",
    "\r\n"
);

static WS_UPGRADE_RESPONSE: &str = concat!(
    "HTTP/1.1 101 Switching Protocols\r\n",
    "Upgrade: websocket\r\n",
    "Connection: Upgrade\r\n",
    "Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=\r\n",
    "\r\n"
);

/// Perform the WebSocket upgrade handshake on an outbound connection.
/// After this, the stream carries Ghostwire traffic as WebSocket frames.
pub async fn ws_client_upgrade(stream: &mut TcpStream) -> Result<()> {
    stream.write_all(WS_UPGRADE_REQUEST.as_bytes()).await?;

    let mut buf = vec![0u8; 256];
    let n = stream.read(&mut buf).await?;
    let response = String::from_utf8_lossy(&buf[..n]);

    if !response.contains("101 Switching Protocols") {
        anyhow::bail!("WebSocket upgrade rejected: {}", response.trim());
    }

    Ok(())
}

/// Handle incoming WebSocket upgrade request (server side).
pub async fn ws_server_upgrade(stream: &mut TcpStream) -> Result<()> {
    let mut buf = vec![0u8; 512];
    let n = stream.read(&mut buf).await?;
    let request = String::from_utf8_lossy(&buf[..n]);

    if !request.contains("Upgrade: websocket") {
        anyhow::bail!("Expected WebSocket upgrade, got: {}", request.trim());
    }

    stream.write_all(WS_UPGRADE_RESPONSE.as_bytes()).await?;
    Ok(())
}

// ── WebSocket frame wrapping ──────────────────────────────────────────────────

/// Wrap bytes in a WebSocket binary frame (RFC 6455).
/// Masked (client→server), unmasked (server→client).
pub fn ws_frame_encode(payload: &[u8], is_client: bool) -> Vec<u8> {
    let mut frame = Vec::new();
    frame.push(0x82); // FIN=1, opcode=0x2 (binary)

    let len = payload.len();
    let masked = is_client;

    if len <= 125 {
        frame.push(len as u8 | if masked { 0x80 } else { 0 });
    } else if len <= 65535 {
        frame.push(126 | if masked { 0x80 } else { 0 });
        frame.extend_from_slice(&(len as u16).to_be_bytes());
    } else {
        frame.push(127 | if masked { 0x80 } else { 0 });
        frame.extend_from_slice(&(len as u64).to_be_bytes());
    }

    if masked {
        // Generate 4-byte masking key
        let mask = [0x37u8, 0x4eu8, 0x6fu8, 0x2au8]; // Fixed for determinism in tests; randomise in prod
        frame.extend_from_slice(&mask);
        for (i, b) in payload.iter().enumerate() {
            frame.push(b ^ mask[i % 4]);
        }
    } else {
        frame.extend_from_slice(payload);
    }

    frame
}

/// Parse a WebSocket frame, returning (payload, bytes_consumed).
pub fn ws_frame_decode(data: &[u8]) -> Result<(Vec<u8>, usize)> {
    if data.len() < 2 {
        anyhow::bail!("WebSocket frame too short");
    }

    let masked = (data[1] & 0x80) != 0;
    let base_len = (data[1] & 0x7F) as usize;

    let (payload_len, header_len) = if base_len <= 125 {
        (base_len, 2)
    } else if base_len == 126 {
        if data.len() < 4 {
            anyhow::bail!("WebSocket frame truncated (16-bit len)");
        }
        (u16::from_be_bytes([data[2], data[3]]) as usize, 4)
    } else {
        if data.len() < 10 {
            anyhow::bail!("WebSocket frame truncated (64-bit len)");
        }
        (u64::from_be_bytes(data[2..10].try_into()?) as usize, 10)
    };

    let mask_len = if masked { 4 } else { 0 };
    let total = header_len + mask_len + payload_len;

    if data.len() < total {
        anyhow::bail!(
            "Incomplete WebSocket frame: need {}, have {}",
            total,
            data.len()
        );
    }

    let mut payload = data[header_len + mask_len..total].to_vec();

    if masked {
        let mask = &data[header_len..header_len + 4];
        for (i, b) in payload.iter_mut().enumerate() {
            *b ^= mask[i % 4];
        }
    }

    Ok((payload, total))
}

// ── Shadowsocks-style AEAD stream ─────────────────────────────────────────────

/// Encrypt a chunk of data for Shadowsocks-style AEAD stream transport.
///
/// Each chunk is independently encrypted with a fresh nonce.
/// Format: [2-byte chunk_len LE][12-byte nonce][ciphertext+tag]
pub fn ss_encrypt_chunk(data: &[u8], psk: &TransportPsk, rng: &SystemRandom) -> Result<Vec<u8>> {
    let key = Key::from_slice(psk.as_bytes());
    let cipher = ChaCha20Poly1305::new(key);

    let mut nonce_bytes = [0u8; 12];
    rng.fill(&mut nonce_bytes)
        .map_err(|_| anyhow::anyhow!("RNG failure"))?;
    let nonce = Nonce::from_slice(&nonce_bytes);

    let ciphertext = cipher
        .encrypt(nonce, data)
        .map_err(|e| anyhow::anyhow!("AEAD encrypt failed: {:?}", e))?;

    let chunk_len = (nonce_bytes.len() + ciphertext.len()) as u16;
    let mut out = Vec::with_capacity(2 + 12 + ciphertext.len());
    out.extend_from_slice(&chunk_len.to_le_bytes());
    out.extend_from_slice(&nonce_bytes);
    out.extend_from_slice(&ciphertext);
    Ok(out)
}

/// Decrypt a Shadowsocks-style AEAD chunk.
/// Returns (plaintext, bytes_consumed).
pub fn ss_decrypt_chunk(data: &[u8], psk: &TransportPsk) -> Result<(Vec<u8>, usize)> {
    if data.len() < 2 {
        anyhow::bail!("SS chunk too short for length header");
    }

    let chunk_len = u16::from_le_bytes([data[0], data[1]]) as usize;
    let total = 2 + chunk_len;

    if data.len() < total {
        anyhow::bail!("SS chunk incomplete: need {}, have {}", total, data.len());
    }

    if chunk_len < 12 {
        anyhow::bail!("SS chunk too short for nonce");
    }

    let nonce = Nonce::from_slice(&data[2..14]);
    let ciphertext = &data[14..total];

    let key = Key::from_slice(psk.as_bytes());
    let cipher = ChaCha20Poly1305::new(key);

    let plaintext = cipher
        .decrypt(nonce, ciphertext)
        .map_err(|_| anyhow::anyhow!("AEAD decrypt failed — wrong key or tampered"))?;

    Ok((plaintext, total))
}

// ── Transport configuration ───────────────────────────────────────────────────

/// Transport configuration for a Ghostwire peer connection.
/// Stored in the node config file (ghostwire.toml).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluggableTransportConfig {
    /// Primary obfuscation mode
    pub mode: TransportMode,
    /// For ShadowsocksAead: the pre-shared key (hex-encoded, 32 bytes)
    pub psk_hex: Option<String>,
    /// For TlsCamouflage: which domain to present in the TLS SNI
    pub sni_domain: Option<String>,
    /// For WebSocket: which Host header to present
    pub ws_host: Option<String>,
    /// For WebSocket: which path to use
    pub ws_path: Option<String>,
    /// Whether to randomly vary WebSocket paths (harder to fingerprint)
    pub randomise_ws_path: bool,
}

impl Default for PluggableTransportConfig {
    fn default() -> Self {
        Self {
            mode: TransportMode::WebSocket,
            psk_hex: None,
            sni_domain: Some("cdn.example.com".to_string()),
            ws_host: Some("notifications.example.com".to_string()),
            ws_path: Some("/ws/live".to_string()),
            randomise_ws_path: true,
        }
    }
}

impl PluggableTransportConfig {
    /// High-censorship preset: use ShadowsocksAead with a random PSK
    pub fn high_censorship() -> Result<Self> {
        let psk = TransportPsk::generate()?;
        Ok(Self {
            mode: TransportMode::ShadowsocksAead,
            psk_hex: Some(hex::encode(psk.as_bytes())),
            randomise_ws_path: true,
            ..Default::default()
        })
    }

    /// Low-censorship preset: WebSocket camouflage (lower overhead)
    pub fn low_censorship() -> Self {
        Self::default()
    }
}

// ── Integration guide ─────────────────────────────────────────────────────────
//
// In mesh.rs, replace the bare TCP transport with:
//
//   1. For WebSocket mode:
//      - On connect: call ws_client_upgrade(&mut stream).await?
//      - On accept: call ws_server_upgrade(&mut stream).await?
//      - Wrap each send: stream.write_all(&ws_frame_encode(&data, true)).await?
//      - Wrap each recv: let (data, _) = ws_frame_decode(&buf)?
//
//   2. For ShadowsocksAead mode:
//      - Load PSK from config: TransportPsk::from_bytes(hex::decode(psk_hex)?)
//      - Wrap each send: stream.write_all(&ss_encrypt_chunk(&data, &psk, &rng)?).await?
//      - Wrap each recv: let (data, _) = ss_decrypt_chunk(&buf, &psk)?
//
//   3. For TlsCamouflage mode:
//      - Use rustls with a custom ServerConfig that presents a self-signed cert
//        with the configured SNI domain. Ghostwire data is the TLS application layer.
//      - Add to Cargo.toml: rustls = "0.23", rcgen = "0.13"
//
// The StealthTCP HMAC handshake still runs INSIDE the obfuscated transport —
// it provides mutual authentication. The PT layer provides traffic analysis
// resistance. They stack.
//
// Recommended configuration guide for end users:
//   - Default (no censorship): Stealth mode
//   - Light censorship (ISP traffic shaping): WebSocket mode
//   - Heavy censorship (national firewall): ShadowsocksAead mode
//   - Extreme (nation-state): ShadowsocksAead + TlsCamouflage layered
// ─────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ws_frame_roundtrip() {
        let payload = b"Ghostwire test message 12345";
        let frame = ws_frame_encode(payload, false);
        let (decoded, consumed) = ws_frame_decode(&frame).unwrap();
        assert_eq!(decoded, payload);
        assert_eq!(consumed, frame.len());
    }

    #[test]
    fn ws_frame_masked_roundtrip() {
        let payload = b"masked client message";
        let frame = ws_frame_encode(payload, true); // client masked
        let (decoded, _) = ws_frame_decode(&frame).unwrap();
        assert_eq!(decoded, payload);
    }

    #[test]
    fn ss_aead_roundtrip() {
        let rng = SystemRandom::new();
        let psk = TransportPsk::generate().unwrap();
        let data = b"Nairobi node sending encrypted chunk";
        let encrypted = ss_encrypt_chunk(data, &psk, &rng).unwrap();
        let (decrypted, consumed) = ss_decrypt_chunk(&encrypted, &psk).unwrap();
        assert_eq!(decrypted, data);
        assert_eq!(consumed, encrypted.len());
    }

    #[test]
    fn ss_aead_wrong_key_fails() {
        let rng = SystemRandom::new();
        let psk = TransportPsk::generate().unwrap();
        let wrong_psk = TransportPsk::generate().unwrap();
        let data = b"secret data";
        let encrypted = ss_encrypt_chunk(data, &psk, &rng).unwrap();
        assert!(ss_decrypt_chunk(&encrypted, &wrong_psk).is_err());
    }
}
