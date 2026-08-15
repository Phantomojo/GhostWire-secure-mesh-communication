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

use crate::core::constants::MAX_MESSAGE_SIZE;
use crate::core::message::Message;

/// QUIC framing includes serialized metadata in addition to message content.
/// The ceiling remains finite to reject peer-controlled allocation requests.
const MAX_QUIC_FRAME_SIZE: usize = MAX_MESSAGE_SIZE * 2;

fn checked_frame_len(length: u32) -> Result<usize> {
    let length = length as usize;
    if length > MAX_QUIC_FRAME_SIZE {
        anyhow::bail!(
            "QUIC frame exceeds maximum size: {} bytes (limit {})",
            length,
            MAX_QUIC_FRAME_SIZE
        );
    }
    Ok(length)
}

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
            if *known != fingerprint {
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
            self.trusted.insert(self.peer_id.clone(), fingerprint);
        }

        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, quinn::rustls::Error> {
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
            .join("tofu_fingerprints.json")
    }

    fn load_fingerprints(store: &DashMap<String, Vec<u8>>) {
        let path = Self::tofu_path();
        if let Ok(data) = std::fs::read_to_string(&path) {
            if let Ok(map) =
                serde_json::from_str::<std::collections::HashMap<String, String>>(&data)
            {
                for (k, v) in map {
                    if let Ok(bytes) = hex::decode(&v) {
                        store.insert(k, bytes);
                    }
                }
                tracing::info!(
                    "TOFU: loaded {} fingerprints from {}",
                    store.len(),
                    path.display()
                );
            }
        }
    }

    fn save_fingerprints(&self) {
        let path = Self::tofu_path();
        let map: std::collections::HashMap<String, String> = self
            .trusted_fingerprints
            .iter()
            .map(|entry| (entry.key().clone(), hex::encode(entry.value())))
            .collect();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        if let Ok(json) = serde_json::to_string_pretty(&map) {
            let _ = std::fs::write(&path, json);
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
        if payload.len() > MAX_QUIC_FRAME_SIZE {
            anyhow::bail!(
                "serialized message exceeds QUIC frame limit: {} bytes (limit {})",
                payload.len(),
                MAX_QUIC_FRAME_SIZE
            );
        }

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
                                            match checked_frame_len(u32::from_be_bytes(len_buf)) {
                                                Ok(length) => length,
                                                Err(error) => {
                                                    tracing::warn!(
                                                        remote = %remote,
                                                        error = %error,
                                                        "Rejected oversized QUIC frame"
                                                    );
                                                    break;
                                                }
                                            };

                                        let mut payload = vec![0u8; payload_len];
                                        if recv.read_exact(&mut payload).await.is_err() {
                                            break;
                                        }

                                        if let Ok(msg) = bincode::deserialize::<Message>(&payload) {
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
    fn frame_length_limit_rejects_oversized_input() {
        assert_eq!(
            checked_frame_len(MAX_QUIC_FRAME_SIZE as u32).unwrap(),
            MAX_QUIC_FRAME_SIZE
        );
        assert!(checked_frame_len((MAX_QUIC_FRAME_SIZE + 1) as u32).is_err());
    }

    #[test]
    fn debug_format() {
        let _ = std::format!("{:?}", "quic_transport debug test");
    }
}
