use anyhow::Result;
use base64::{engine::general_purpose, Engine as _};
use qrcode_generator::QrCodeEcc;
use tracing::info;

pub struct QrManager;

impl QrManager {
    /// Generate a QR code for an identity (PeerID + Public Key)
    pub fn generate_identity_qr(peer_id: &str, public_key: &[u8]) -> Result<Vec<u8>> {
        let pub_key_b64 = general_purpose::STANDARD.encode(public_key);
        let data = format!("ghostwire://{}?pk={}", peer_id, pub_key_b64);

        info!("Generating QR code for identity: {}", peer_id);

        let png_bytes = qrcode_generator::to_png_to_vec(data, QrCodeEcc::Low, 256)
            .map_err(|e| anyhow::anyhow!("QR generation failed: {}", e))?;

        Ok(png_bytes)
    }

    /// Verify a scanned identity string
    pub fn verify_scanned_identity(scanned_data: &str) -> Option<(String, Vec<u8>)> {
        if !scanned_data.starts_with("ghostwire://") {
            return None;
        }

        let parts: Vec<&str> = scanned_data
            .trim_start_matches("ghostwire://")
            .split("?pk=")
            .collect();
        if parts.len() != 2 {
            return None;
        }

        let peer_id = parts[0].to_string();
        let public_key = general_purpose::STANDARD.decode(parts[1]).ok()?;

        Some((peer_id, public_key))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_qr_identity_roundtrip() {
        let peer_id = "12D3KooWTest";
        let public_key = vec![1, 2, 3, 4, 5];

        // Generate QR bytes
        let qr_bytes = QrManager::generate_identity_qr(peer_id, &public_key).unwrap();
        assert!(!qr_bytes.is_empty());

        // In a real test we'd decode the QR image, but we can test the verification logic separately
        let encoded = format!(
            "ghostwire://{}?pk={}",
            peer_id,
            general_purpose::STANDARD.encode(&public_key)
        );
        let (verified_id, verified_pk) = QrManager::verify_scanned_identity(&encoded).unwrap();

        assert_eq!(verified_id, peer_id);
        assert_eq!(verified_pk, public_key);
    }

    #[test]
    fn test_verify_invalid_identity() {
        assert!(QrManager::verify_scanned_identity("invalid").is_none());
        assert!(QrManager::verify_scanned_identity("ghostwire://only_peer_id").is_none());
    }
}
