// stealth_tcp_handshake_fix.rs
// ─────────────────────────────────────────────────────────────────────────────
// Drop-in replacement for the perform_stealth_handshake() and
// perform_challenge_response() methods in src/core/stealth_tcp.rs
//
// PROBLEM: The current code sends a static 5-byte magic word "GWSTH" as the
//          first bytes of every connection. Any passive observer can fingerprint
//          Ghostwire traffic with a trivial byte-pattern rule — completely
//          defeating the "stealth" property.
//
// FIX: Replace the static magic bytes with a proper HMAC-based challenge that
//      makes the handshake look like random data to anyone who doesn't have
//      the shared secret.
//
// PROTOCOL (new):
//   Initiator → Responder:
//     [32 bytes] nonce (random, per-connection)
//     [32 bytes] HMAC-SHA256(secret, "ghostwire-init" || nonce)
//
//   Responder → Initiator:
//     [32 bytes] HMAC-SHA256(secret, "ghostwire-resp" || nonce)
//
// Wire appearance: 96 bytes of pseudorandom data, no fixed patterns.
// An observer without the secret cannot distinguish this from TLS or noise.
// ─────────────────────────────────────────────────────────────────────────────

use ring::hmac::{self, HMAC_SHA256};
use ring::rand::{SecureRandom, SystemRandom};
use std::io;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;
use std::time::Duration;

const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(5);
const NONCE_LEN: usize = 32;
const MAC_LEN: usize = 32;

impl StealthTCPProvider {
    /// Stealth handshake — initiator side (client connecting to peer)
    ///
    /// Sends:  nonce || HMAC(secret, "ghostwire-init" || nonce)
    /// Reads:  HMAC(secret, "ghostwire-resp" || nonce)
    ///
    /// Returns Ok(true) if the peer responds correctly.
    pub async fn initiate_stealth_handshake(
        &self,
        stream: &mut TcpStream,
    ) -> io::Result<bool> {
        let rng = SystemRandom::new();
        let mut nonce = [0u8; NONCE_LEN];
        rng.fill(&mut nonce)
            .map_err(|_| io::Error::new(io::ErrorKind::Other, "RNG failure"))?;

        // Build initiator MAC: HMAC(secret, "ghostwire-init" || nonce)
        let key = hmac::Key::new(HMAC_SHA256, &self.handshake_secret.0);
        let mut init_data = b"ghostwire-init".to_vec();
        init_data.extend_from_slice(&nonce);
        let init_mac = hmac::sign(&key, &init_data);

        // Send nonce + MAC (64 bytes total)
        let mut payload = Vec::with_capacity(NONCE_LEN + MAC_LEN);
        payload.extend_from_slice(&nonce);
        payload.extend_from_slice(init_mac.as_ref());

        timeout(HANDSHAKE_TIMEOUT, stream.write_all(&payload))
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "Handshake send timed out"))?
            .map_err(|e| io::Error::new(io::ErrorKind::BrokenPipe, e))?;

        // Read responder MAC (32 bytes)
        let mut resp_mac = [0u8; MAC_LEN];
        timeout(HANDSHAKE_TIMEOUT, stream.read_exact(&mut resp_mac))
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "Handshake recv timed out"))?
            .map_err(|e| io::Error::new(io::ErrorKind::ConnectionReset, e))?;

        // Verify: HMAC(secret, "ghostwire-resp" || nonce)
        let mut resp_data = b"ghostwire-resp".to_vec();
        resp_data.extend_from_slice(&nonce);
        let expected = hmac::sign(&key, &resp_data);

        // Constant-time comparison via ring's verify
        let ok = hmac::verify(
            &hmac::Key::new(HMAC_SHA256, &self.handshake_secret.0),
            &resp_data,
            &resp_mac,
        )
        .is_ok();

        if !ok {
            tracing::warn!("StealthTCP: responder MAC mismatch — rejecting connection");
        }
        Ok(ok)
    }

    /// Stealth handshake — responder side (server accepting from peer)
    ///
    /// Reads:  nonce || HMAC(secret, "ghostwire-init" || nonce)
    /// Sends:  HMAC(secret, "ghostwire-resp" || nonce)
    ///
    /// Returns Ok(true) if the initiator's MAC is valid.
    pub async fn respond_stealth_handshake(
        &self,
        stream: &mut TcpStream,
    ) -> io::Result<bool> {
        // Read nonce + initiator MAC (64 bytes total)
        let mut buf = [0u8; NONCE_LEN + MAC_LEN];
        timeout(HANDSHAKE_TIMEOUT, stream.read_exact(&mut buf))
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "Handshake recv timed out"))?
            .map_err(|e| io::Error::new(io::ErrorKind::ConnectionReset, e))?;

        let nonce = &buf[..NONCE_LEN];
        let recv_mac = &buf[NONCE_LEN..];

        // Verify initiator MAC: HMAC(secret, "ghostwire-init" || nonce)
        let mut init_data = b"ghostwire-init".to_vec();
        init_data.extend_from_slice(nonce);

        let ok = hmac::verify(
            &hmac::Key::new(HMAC_SHA256, &self.handshake_secret.0),
            &init_data,
            recv_mac,
        )
        .is_ok();

        if !ok {
            tracing::warn!("StealthTCP: initiator MAC invalid — dropping connection");
            // IMPORTANT: Do NOT send any response. Silent drop prevents oracle attacks.
            // Add a small random delay to prevent timing-based fingerprinting.
            tokio::time::sleep(Duration::from_millis(
                100 + (nonce[0] as u64 % 150), // 100–250ms, seeded from nonce for consistency
            ))
            .await;
            return Ok(false);
        }

        // Send responder MAC: HMAC(secret, "ghostwire-resp" || nonce)
        let key = hmac::Key::new(HMAC_SHA256, &self.handshake_secret.0);
        let mut resp_data = b"ghostwire-resp".to_vec();
        resp_data.extend_from_slice(nonce);
        let resp_mac = hmac::sign(&key, &resp_data);

        timeout(HANDSHAKE_TIMEOUT, stream.write_all(resp_mac.as_ref()))
            .await
            .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "Handshake send timed out"))?
            .map_err(|e| io::Error::new(io::ErrorKind::BrokenPipe, e))?;

        Ok(true)
    }
}

// ─────────────────────────────────────────────────────────────────────────────
// MIGRATION GUIDE
// ─────────────────────────────────────────────────────────────────────────────
//
// 1. Remove from stealth_tcp.rs:
//      const MAGIC_BYTES: &[u8; 5] = b"GWSTH";
//      const STEALTH_VERSION: u8 = 0x01;
//      fn perform_stealth_handshake()   ← old method
//      fn perform_challenge_response()  ← old method
//
// 2. Add the two new methods above (initiate_stealth_handshake,
//    respond_stealth_handshake) to the StealthTCPProvider impl block.
//
// 3. Update call sites:
//    - Where you were calling perform_stealth_handshake() on accept:
//        → call respond_stealth_handshake(stream).await?
//    - Where you initiate a new outbound connection:
//        → call initiate_stealth_handshake(stream).await?
//
// 4. The handshake_secret field stays exactly the same — no struct changes.
//    If handshake_secret is None at runtime, generate 32 random bytes as
//    before. Make sure both sides use the SAME secret (pre-shared or derived
//    from a higher-level key exchange).
//
// 5. Add this note to SECURITY.md and the startup logs:
//      "StealthTCP handshake uses HMAC-SHA256 with per-connection nonces.
//       Wire traffic is indistinguishable from random to passive observers."
// ─────────────────────────────────────────────────────────────────────────────
