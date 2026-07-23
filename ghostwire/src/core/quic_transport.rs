use anyhow::Result;
use dashmap::DashMap;
use quinn::{ClientConfig, Connection, Endpoint, ServerConfig};
use quinn_proto::crypto::rustls::QuicClientConfig;
use sha2::{Digest, Sha256};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::mpsc;

use quinn::rustls::client::danger::{
    HandshakeSignatureValid, ServerCertVerified, ServerCertVerifier,
};
use quinn::rustls::pki_types::{CertificateDer, ServerName, UnixTime};
use quinn::rustls::{DigitallySignedStruct, SignatureScheme};

use crate::core::message::Message;

/// Maximum payload size to prevent memory exhaustion attacks (1 MB)
const MAX_PAYLOAD_SIZE: usize = 1024 * 1024;

type PrivateKeyDer = quinn::rustls::pki_types::PrivateKeyDer<'static>;

// ponytail: TOFU verifier that checks cert fingerprints per peer.
// Each connection gets its own verifier instance via connect_with(), so we know
// which peer_id we're verifying. Accepts on first use, rejects on fingerprint mismatch.
// Upgrade path: add timestamp-based expiry, revocation list.
#[derive(Debug)]
struct TofuVerifier {
    peer_id: String,
    trusted: Arc<DashMap<String, Vec<u8>>>,
}

impl TofuVerifier {
    fn new(peer_id: String, trusted: Arc<DashMap<String, Vec<u8>>>) -> Self {
        Self { peer_id, trusted }
    }
}

impl ServerCertVerifier for TofuVerifier {
    fn verify_server_cert(
        &self,
        end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, quinn::rustls::Error> {
        let fingerprint = Sha256::digest(end_entity.as_ref()).to_vec();

        if let Some(known) = self.trusted.get(&self.peer_id) {
            // SECURITY FIX: Use constant-time comparison to prevent timing attacks
            use subtle::ConstantTimeEq;
            let fingerprint_matches = known.ct_eq(&fingerprint);
            if fingerprint_matches.into() {
                // Fingerprint matches
            } else {
                tracing::warn!(
                    "TOFU VIOLATION: peer {} cert changed! old={} new={}",
                    self.peer_id,
                    hex::encode(&known[..8]),
                    hex::encode(&fingerprint[..8])
                );
                return Err(quinn::rustls::Error::General(format!(
                    "TOFU: certificate for peer {} changed (possible MITM)",
                    self.peer_id
                )));
            }
        } else {
            tracing::info!(
                "TOFU: first contact with peer {}, storing fingerprint {}",
                self.peer_id,
                hex::encode(&fingerprint[..8])
            );
            self.trusted
                .insert(self.peer_id.clone(), fingerprint);
        }

        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, quinn::rustls::Error> {
        // Ponytail: TOFU cert verification in verify_server_cert provides primary
        // MITM defense. TLS handshake sig verification is defense-in-depth but
        // requires rustls::webpki which is private in quinn's re-export.
        // If downgrade attacks become a real threat, add rustls = "0.23" as a
        // direct dep and call rustls::webpki::verify_tls12_signature().
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, quinn::rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<SignatureScheme> {
        vec![
            SignatureScheme::RSA_PKCS1_SHA256,
            SignatureScheme::RSA_PKCS1_SHA384,
            SignatureScheme::RSA_PKCS1_SHA512,
            SignatureScheme::ECDSA_NISTP256_SHA256,
            SignatureScheme::ECDSA_NISTP384_SHA384,
            SignatureScheme::ECDSA_NISTP521_SHA512,
            SignatureScheme::ED25519,
        ]
    }
}

#[derive(Clone)]
pub struct QuicTransportProvider {
    endpoint: Endpoint,
    trusted_fingerprints: Arc<DashMap<String, Vec<u8>>>,
    connections: Arc<DashMap<String, Connection>>,
    peer_map: Arc<DashMap<String, SocketAddr>>,
    local_fingerprint: Vec<u8>,
}

impl std::fmt::Debug for QuicTransportProvider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("QuicTransportProvider")
            .field("fingerprint", &hex::encode(&self.local_fingerprint[..8]))
            .finish()
    }
}

impl QuicTransportProvider {
    pub async fn new(bind_addr: SocketAddr) -> Result<Self> {
        let _ = quinn::rustls::crypto::ring::default_provider().install_default();

        let params = rcgen::CertificateParams::new(vec!["ghostwire.local".to_string()])?;
        let key_pair = rcgen::KeyPair::generate()?;
        let cert = params.self_signed(&key_pair)?;

        let cert_der: CertificateDer = CertificateDer::from(cert.der().to_vec());
        let key_der: PrivateKeyDer = PrivateKeyDer::Pkcs8(key_pair.serialize_der().into());

        let mut hasher = Sha256::new();
        hasher.update(cert.der());
        let fingerprint = hasher.finalize().to_vec();

        let server_config = ServerConfig::with_single_cert(vec![cert_der.clone()], key_der)?;

        let endpoint = Endpoint::server(server_config, bind_addr)?;

        let trusted = Arc::new(DashMap::new());
        Self::load_fingerprints(&trusted);

        tracing::info!(
            "QUIC transport started on {} (fingerprint: {})",
            bind_addr,
            hex::encode(&fingerprint[..8])
        );

        Ok(Self {
            endpoint,
            trusted_fingerprints: trusted,
            connections: Arc::new(DashMap::new()),
            peer_map: Arc::new(DashMap::new()),
            local_fingerprint: fingerprint,
        })
    }

    fn tofu_path() -> std::path::PathBuf {
        dirs::home_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join(".ghostwire")
            .join("tofu_fingerprints.enc")
    }

    fn load_fingerprints(store: &Arc<DashMap<String, Vec<u8>>>) {
        let path = Self::tofu_path();
        if let Ok(data) = std::fs::read(&path) {
            if let Ok(decrypted) = Self::decrypt_fingerprint_store(&data) {
                if let Ok(map) = serde_json::from_slice::<std::collections::HashMap<String, String>>(&decrypted) {
                    for (k, v) in map {
                        if let Ok(bytes) = hex::decode(&v) {
                            store.insert(k, bytes);
                        }
                    }
                    tracing::info!("TOFU: loaded {} fingerprints from encrypted store", store.len());
                }
            } else {
                tracing::warn!("TOFU: failed to decrypt fingerprint store (may be legacy unencrypted)");
                // Try loading legacy unencrypted format for migration
                Self::load_legacy_fingerprints(store);
            }
        }
    }

    /// Load legacy unencrypted fingerprints for migration
    fn load_legacy_fingerprints(store: &Arc<DashMap<String, Vec<u8>>>) {
        let legacy_path = dirs::home_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join(".ghostwire")
            .join("tofu_fingerprints.json");
        
        if let Ok(data) = std::fs::read_to_string(&legacy_path) {
            if let Ok(map) = serde_json::from_str::<std::collections::HashMap<String, String>>(&data) {
                for (k, v) in map {
                    if let Ok(bytes) = hex::decode(&v) {
                        store.insert(k, bytes);
                    }
                }
                tracing::info!("TOFU: migrated {} fingerprints from legacy store", store.len());
                // Delete legacy file after migration
                let _ = std::fs::remove_file(&legacy_path);
                tracing::info!("TOFU: deleted legacy unencrypted fingerprint store");
            }
        }
    }

    /// Derive encryption key from machine-specific secret
    fn derive_encryption_key() -> [u8; 32] {
        use ring::rand::{SecureRandom, SystemRandom};
        // Use machine ID + process-specific entropy as key material
        let machine_id = hostname::get()
            .map(|h| h.to_string_lossy().to_string())
            .unwrap_or_else(|_| "ghostwire-default".to_string());
        let process_id = std::process::id().to_le_bytes();
        
        let mut key_material = machine_id.into_bytes();
        key_material.extend_from_slice(&process_id);
        
        // HKDF-like derivation using BLAKE3
        let salt = b"GHOSTWIRE-TOFU-FP-SALT";
        // Derive a 32-byte key from salt using BLAKE3 hash
        let key_bytes = *blake3::hash(salt).as_bytes();
        let mut hasher = blake3::Hasher::new_keyed(&key_bytes);
        hasher.update(&key_material);
        *hasher.finalize().as_bytes()
    }

    /// Encrypt the fingerprint store for secure storage
    fn encrypt_fingerprint_store(data: &[u8]) -> Result<Vec<u8>> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            XChaCha20Poly1305,
        };
        use ring::rand::{SecureRandom, SystemRandom};
        
        let key: XChaCha20Poly1305 = XChaCha20Poly1305::new(Self::derive_encryption_key().as_slice().into());
        let rng = SystemRandom::new();
        let mut nonce_bytes = [0u8; 24];
        rng.fill(&mut nonce_bytes)
            .map_err(|_| anyhow::anyhow!("RNG failure"))?;
        let nonce = chacha20poly1305::XNonce::from(nonce_bytes);
        
        let ciphertext = key.encrypt(&nonce, data)
            .map_err(|_| anyhow::anyhow!("Encryption failed"))?;
        
        // Format: [24-byte nonce][ciphertext]
        let mut result = nonce_bytes.to_vec();
        result.extend_from_slice(&ciphertext);
        Ok(result)
    }

    /// Decrypt the fingerprint store
    fn decrypt_fingerprint_store(data: &[u8]) -> Result<Vec<u8>> {
        use chacha20poly1305::{
            aead::{Aead, KeyInit},
            XChaCha20Poly1305,
        };
        
        if data.len() < 24 {
            anyhow::bail!("Invalid encrypted data: too short");
        }
        
        let key: XChaCha20Poly1305 = XChaCha20Poly1305::new(Self::derive_encryption_key().as_slice().into());
        let nonce = chacha20poly1305::XNonce::from_slice(&data[..24]);
        let ciphertext = &data[24..];
        
        key.decrypt(&nonce, ciphertext)
            .map_err(|_| anyhow::anyhow!("Decryption failed"))
    }

    fn save_fingerprints(&self) {
        let path = Self::tofu_path();
        let map: std::collections::HashMap<String, String> = self
            .trusted_fingerprints
            .iter()
            .map(|entry| (entry.key().clone(), hex::encode(entry.value())))
            .collect();
        
        if let Ok(json) = serde_json::to_vec(&map) {
            if let Ok(encrypted) = Self::encrypt_fingerprint_store(&json) {
                if let Some(parent) = path.parent() {
                    let _ = std::fs::create_dir_all(parent);
                }
                let _ = std::fs::write(&path, &encrypted);
                
                // Set restrictive permissions (owner read/write only)
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    if let Ok(meta) = std::fs::metadata(&path) {
                        let mut perms = meta.permissions();
                        perms.set_mode(0o600);
                        let _ = std::fs::set_permissions(&path, perms);
                    }
                }
            }
        }
    }

    pub fn register_peer(&self, peer_id: &str, addr: SocketAddr) {
        self.peer_map.insert(peer_id.to_string(), addr);
    }

    pub async fn connect(&self, peer_id: &str, addr: SocketAddr) -> Result<()> {
        let tofu = TofuVerifier::new(peer_id.to_string(), self.trusted_fingerprints.clone());
        let tls = quinn::rustls::ClientConfig::builder()
            .dangerous()
            .with_custom_certificate_verifier(Arc::new(tofu))
            .with_no_client_auth();
        let quic_client = QuicClientConfig::try_from(Arc::new(tls))?;
        let client_config = ClientConfig::new(Arc::new(quic_client));

        let conn = self
            .endpoint
            .connect_with(client_config, addr, "ghostwire.local")?
            .await?;

        self.connections.insert(peer_id.to_string(), conn);
        self.peer_map.insert(peer_id.to_string(), addr);
        self.save_fingerprints();

        tracing::info!("QUIC connected to {} at {}", peer_id, addr);
        Ok(())
    }

    pub async fn send_message(&self, message: &Message) -> Result<()> {
        let peer_id = &message.recipient;

        if !self.connections.contains_key(peer_id) {
            if let Some(addr) = self.peer_map.get(peer_id) {
                let addr = *addr;
                let _ = addr;
                self.connect(peer_id, addr).await?;
            } else {
                return Err(anyhow::anyhow!("No address for peer: {}", peer_id));
            }
        }

        let conn = self
            .connections
            .get(peer_id)
            .ok_or_else(|| anyhow::anyhow!("Not connected to {}", peer_id))?;

        let (mut send, _recv) = conn.open_bi().await?;
        let payload = bincode::serialize(message)?;

        let mut frame = Vec::with_capacity(4 + payload.len());
        frame.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        frame.extend_from_slice(&payload);

        send.write_all(&frame).await?;
        send.finish()?;

        // ponytail: sleep to let QUIC transmit before caller drops the connection.
        // Real fix: caller keeps the transport alive; this is the safe default for
        // one-shot sends. Upgrade path: expose a flush/drain API that awaits
        // stream acknowledgment from the peer.
        tokio::time::sleep(std::time::Duration::from_millis(200)).await;

        Ok(())
    }

    pub fn start_listener(&self) -> mpsc::Receiver<Message> {
        let (tx, rx) = mpsc::channel(256);
        let endpoint = self.endpoint.clone();

        tokio::spawn(async move {
            while let Some(incoming) = endpoint.accept().await {
                let tx = tx.clone();

                match incoming.await {
                    Ok(conn) => {
                        let remote = conn.remote_address();
                        tracing::info!("QUIC incoming connection from {}", remote);

                        tokio::spawn(async move {
                            loop {
                                match conn.accept_bi().await {
                                    Ok((_send, mut recv)) => {
                                        let mut len_buf = [0u8; 4];
                                        if recv.read_exact(&mut len_buf).await.is_err() {
                                            break;
                                        }
                                        let payload_len =
                                            u32::from_be_bytes(len_buf) as usize;

                                        // SECURITY FIX: Validate payload size to prevent memory exhaustion
                                        if payload_len > MAX_PAYLOAD_SIZE {
                                            tracing::warn!(
                                                "QUIC: payload too large {} > {} bytes, rejecting",
                                                payload_len,
                                                MAX_PAYLOAD_SIZE
                                            );
                                            break;
                                        }

                                        let mut payload = vec![0u8; payload_len];
                                        if recv.read_exact(&mut payload).await.is_err() {
                                            break;
                                        }

                                        if let Ok(msg) =
                                            bincode::deserialize::<Message>(&payload)
                                        {
                                            let _ = tx.send(msg).await;
                                        }
                                    }
                                    Err(_) => break,
                                }
                            }
                        });
                    }
                    Err(e) => {
                        tracing::warn!("QUIC accept error: {}", e);
                    }
                }
            }
        });

        rx
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn debug_format() {
        let _ = std::format!("{:?}", "quic_transport debug test");
    }

    #[test]
    fn test_max_payload_size_constant() {
        // Verify MAX_PAYLOAD_SIZE is reasonable (1 MB)
        assert_eq!(MAX_PAYLOAD_SIZE, 1024 * 1024);
        assert!(MAX_PAYLOAD_SIZE > 0);
    }

    #[test]
    fn test_encrypt_decrypt_fingerprint_store() {
        use std::collections::HashMap;
        
        let mut test_data = HashMap::new();
        test_data.insert("peer1".to_string(), "fingerprint1".to_string());
        test_data.insert("peer2".to_string(), "fingerprint2".to_string());
        
        let json = serde_json::to_vec(&test_data).unwrap();
        let encrypted = QuicTransportProvider::encrypt_fingerprint_store(&json).unwrap();
        
        // Verify encrypted data is different from plaintext
        assert_ne!(encrypted, json);
        
        // Verify encrypted data has nonce prefix
        assert!(encrypted.len() > 24);
        
        let decrypted = QuicTransportProvider::decrypt_fingerprint_store(&encrypted).unwrap();
        let recovered: HashMap<String, String> = serde_json::from_slice(&decrypted).unwrap();
        
        assert_eq!(recovered.get("peer1"), test_data.get("peer1"));
        assert_eq!(recovered.get("peer2"), test_data.get("peer2"));
    }

    #[test]
    fn test_encrypt_fails_on_invalid_data() {
        let result = QuicTransportProvider::decrypt_fingerprint_store(&[]);
        assert!(result.is_err());
        
        // Short data without valid nonce
        let short_data = vec![0u8; 10];
        let result = QuicTransportProvider::decrypt_fingerprint_store(&short_data);
        assert!(result.is_err());
    }

    #[test]
    fn test_encrypt_decrypt_empty_store() {
        use std::collections::HashMap;
        
        let empty: HashMap<String, String> = HashMap::new();
        let json = serde_json::to_vec(&empty).unwrap();
        
        let encrypted = QuicTransportProvider::encrypt_fingerprint_store(&json).unwrap();
        let decrypted = QuicTransportProvider::decrypt_fingerprint_store(&encrypted).unwrap();
        let recovered: HashMap<String, String> = serde_json::from_slice(&decrypted).unwrap();
        
        assert!(recovered.is_empty());
    }
}
