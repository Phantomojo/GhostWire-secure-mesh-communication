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

use crate::core::crypto_profile::{
    negotiate, CryptoCapabilities, CryptoProfile, CryptoProfileId, NegotiatedProfile,
};
use crate::core::message::Message;

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

const CAPABILITY_MAGIC: &[u8; 4] = b"GWCH";
const DATA_MAGIC: &[u8; 4] = b"GWDT";
const WIRE_VERSION: u8 = 1;
const MAX_CAPABILITY_BYTES: usize = 4096;
const PROFILE_TRANSCRIPT_DOMAIN: &[u8] = b"GhostWire-QUIC-profile-transcript-v1";

#[derive(Clone)]
pub struct QuicTransportProvider {
    endpoint: Endpoint,
    trusted_fingerprints: Arc<DashMap<String, Vec<u8>>>,
    connections: Arc<DashMap<String, Connection>>,
    negotiated_profiles: Arc<DashMap<String, NegotiatedProfile>>,
    peer_map: Arc<DashMap<String, SocketAddr>>,
    local_capabilities: CryptoCapabilities,
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
        let local_capabilities = CryptoCapabilities::new(
            vec![CryptoProfileId::Gw0Session],
            CryptoProfile::GW0_SESSION.max_plaintext_fragment,
            false,
        )
        .map_err(|e| anyhow::anyhow!("invalid local QUIC crypto capabilities: {}", e))?;

        tracing::info!(
            "QUIC transport started on {} (fingerprint: {})",
            bind_addr,
            hex::encode(&fingerprint[..8])
        );

        Ok(Self {
            endpoint,
            trusted_fingerprints: trusted,
            connections: Arc::new(DashMap::new()),
            negotiated_profiles: Arc::new(DashMap::new()),
            peer_map: Arc::new(DashMap::new()),
            local_capabilities,
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

        let negotiated = self.client_profile_handshake(&conn).await?;
        self.connections.insert(peer_id.to_string(), conn);
        self.negotiated_profiles
            .insert(peer_id.to_string(), negotiated);
        self.peer_map.insert(peer_id.to_string(), addr);
        self.save_fingerprints();

        tracing::info!(
            "QUIC connected to {} at {} with profile {:?}",
            peer_id,
            addr,
            negotiated.profile.id
        );
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

        let negotiated = self
            .negotiated_profiles
            .get(peer_id)
            .map(|entry| *entry.value())
            .ok_or_else(|| anyhow::anyhow!("QUIC profile not negotiated for {}", peer_id))?;
        let conn = self
            .connections
            .get(peer_id)
            .ok_or_else(|| anyhow::anyhow!("Not connected to {}", peer_id))?;

        let (mut send, _recv) = conn.open_bi().await?;
        let frame = Self::encode_message_frame(message, negotiated)?;

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

        let local_capabilities = self.local_capabilities.clone();
        tokio::spawn(async move {
            while let Some(incoming) = endpoint.accept().await {
                let tx = tx.clone();

                match incoming.await {
                    Ok(conn) => {
                        let remote = conn.remote_address();
                        tracing::info!("QUIC incoming connection from {}", remote);

                        let connection_capabilities = local_capabilities.clone();
                        tokio::spawn(async move {
                            let negotiated = match conn.accept_bi().await {
                                Ok((mut send, mut recv)) => {
                                    match Self::server_profile_handshake(
                                        &mut send,
                                        &mut recv,
                                        &connection_capabilities,
                                    )
                                    .await
                                    {
                                        Ok(profile) => profile,
                                        Err(error) => {
                                            tracing::warn!(
                                                remote = %remote,
                                                error = %error,
                                                "QUIC crypto-profile negotiation rejected"
                                            );
                                            conn.close(0u32.into(), b"crypto profile rejected");
                                            return;
                                        }
                                    }
                                }
                                Err(_) => return,
                            };

                            loop {
                                match conn.accept_bi().await {
                                    Ok((_send, mut recv)) => {
                                        match Self::decode_message_frame(&mut recv, negotiated)
                                            .await
                                        {
                                            Ok(msg) => {
                                                let _ = tx.send(msg).await;
                                            }
                                            Err(error) => {
                                                tracing::warn!(
                                                    remote = %remote,
                                                    error = %error,
                                                    "QUIC message frame rejected"
                                                );
                                            }
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

    fn local_profile_binding(
        client_capabilities: &[u8],
        server_capabilities: &[u8],
        negotiated: NegotiatedProfile,
    ) -> Vec<u8> {
        let mut hasher = Sha256::new();
        hasher.update(PROFILE_TRANSCRIPT_DOMAIN);
        hasher.update((client_capabilities.len() as u32).to_be_bytes());
        hasher.update(client_capabilities);
        hasher.update((server_capabilities.len() as u32).to_be_bytes());
        hasher.update(server_capabilities);
        hasher.update(negotiated.profile.id.wire_id().to_be_bytes());
        hasher.update(negotiated.max_frame_size.to_be_bytes());
        hasher.update([u8::from(negotiated.fragmentation_required)]);
        hasher.finalize().to_vec()
    }

    async fn client_profile_handshake(&self, conn: &Connection) -> Result<NegotiatedProfile> {
        let (mut send, mut recv) = conn.open_bi().await?;
        let local_bytes = self.local_capabilities.canonical_bytes();
        if local_bytes.len() > MAX_CAPABILITY_BYTES {
            return Err(anyhow::anyhow!("local QUIC capabilities exceed limit"));
        }
        send.write_all(CAPABILITY_MAGIC).await?;
        send.write_all(&[WIRE_VERSION]).await?;
        send.write_all(&(local_bytes.len() as u32).to_be_bytes())
            .await?;
        send.write_all(&local_bytes).await?;
        send.finish()?;

        let mut magic = [0u8; 4];
        recv.read_exact(&mut magic).await?;
        if &magic != b"GWPR" {
            return Err(anyhow::anyhow!("invalid QUIC profile response magic"));
        }
        let mut version = [0u8; 1];
        recv.read_exact(&mut version).await?;
        if version[0] != WIRE_VERSION {
            return Err(anyhow::anyhow!("unsupported QUIC profile response version"));
        }
        let server_bytes = Self::read_bounded_bytes(&mut recv, MAX_CAPABILITY_BYTES).await?;
        let server_capabilities = CryptoCapabilities::from_canonical_bytes(&server_bytes)
            .map_err(|e| anyhow::anyhow!("invalid server capabilities: {}", e))?;
        let negotiated = negotiate(&self.local_capabilities, &server_capabilities, 1)
            .map_err(|e| anyhow::anyhow!("QUIC profile negotiation failed: {}", e))?;
        Self::reject_unimplemented_fragmentation(negotiated)?;

        let selected_id = Self::read_u16(&mut recv).await?;
        let selected_frame_size = Self::read_u32(&mut recv).await?;
        let mut fragmentation = [0u8; 1];
        recv.read_exact(&mut fragmentation).await?;
        let mut binding = vec![0u8; 32];
        recv.read_exact(&mut binding).await?;

        if selected_id != negotiated.profile.id.wire_id()
            || selected_frame_size != negotiated.max_frame_size
            || fragmentation[0] != u8::from(negotiated.fragmentation_required)
            || binding != Self::local_profile_binding(&local_bytes, &server_bytes, negotiated)
        {
            return Err(anyhow::anyhow!("QUIC profile transcript binding mismatch"));
        }
        Ok(negotiated)
    }

    async fn server_profile_handshake(
        send: &mut quinn::SendStream,
        recv: &mut quinn::RecvStream,
        local_capabilities: &CryptoCapabilities,
    ) -> Result<NegotiatedProfile> {
        let mut magic = [0u8; 4];
        recv.read_exact(&mut magic).await?;
        if &magic != CAPABILITY_MAGIC {
            return Err(anyhow::anyhow!("invalid QUIC capability magic"));
        }
        let mut version = [0u8; 1];
        recv.read_exact(&mut version).await?;
        if version[0] != WIRE_VERSION {
            return Err(anyhow::anyhow!("unsupported QUIC capability version"));
        }
        let client_bytes = Self::read_bounded_bytes(recv, MAX_CAPABILITY_BYTES).await?;
        let client_capabilities = CryptoCapabilities::from_canonical_bytes(&client_bytes)
            .map_err(|e| anyhow::anyhow!("invalid client capabilities: {}", e))?;
        let negotiated = negotiate(&client_capabilities, local_capabilities, 1)
            .map_err(|e| anyhow::anyhow!("QUIC profile negotiation failed: {}", e))?;
        Self::reject_unimplemented_fragmentation(negotiated)?;
        let server_bytes = local_capabilities.canonical_bytes();

        send.write_all(b"GWPR").await?;
        send.write_all(&[WIRE_VERSION]).await?;
        send.write_all(&(server_bytes.len() as u32).to_be_bytes())
            .await?;
        send.write_all(&server_bytes).await?;
        send.write_all(&negotiated.profile.id.wire_id().to_be_bytes())
            .await?;
        send.write_all(&negotiated.max_frame_size.to_be_bytes())
            .await?;
        send.write_all(&[u8::from(negotiated.fragmentation_required)])
            .await?;
        send.write_all(&Self::local_profile_binding(
            &client_bytes,
            &server_bytes,
            negotiated,
        ))
        .await?;
        send.finish()?;
        Ok(negotiated)
    }

    async fn read_bounded_bytes(recv: &mut quinn::RecvStream, max: usize) -> Result<Vec<u8>> {
        let length = Self::read_u32(recv).await? as usize;
        if length == 0 || length > max {
            return Err(anyhow::anyhow!("QUIC capability length out of bounds"));
        }
        let mut bytes = vec![0u8; length];
        recv.read_exact(&mut bytes).await?;
        Ok(bytes)
    }

    async fn read_u16(recv: &mut quinn::RecvStream) -> Result<u16> {
        let mut bytes = [0u8; 2];
        recv.read_exact(&mut bytes).await?;
        Ok(u16::from_be_bytes(bytes))
    }

    async fn read_u32(recv: &mut quinn::RecvStream) -> Result<u32> {
        let mut bytes = [0u8; 4];
        recv.read_exact(&mut bytes).await?;
        Ok(u32::from_be_bytes(bytes))
    }

    fn reject_unimplemented_fragmentation(negotiated: NegotiatedProfile) -> Result<()> {
        if negotiated.fragmentation_required {
            return Err(anyhow::anyhow!(
                "QUIC profile requires fragmentation, but QUIC fragmentation is not integrated"
            ));
        }
        Ok(())
    }

    fn encode_message_frame(message: &Message, negotiated: NegotiatedProfile) -> Result<Vec<u8>> {
        let payload = bincode::serialize(message)?;
        if payload.len() > negotiated.max_frame_size as usize {
            return Err(anyhow::anyhow!(
                "QUIC message exceeds negotiated frame size"
            ));
        }
        let mut frame = Vec::with_capacity(4 + 1 + 2 + 4 + payload.len());
        frame.extend_from_slice(DATA_MAGIC);
        frame.push(WIRE_VERSION);
        frame.extend_from_slice(&negotiated.profile.id.wire_id().to_be_bytes());
        frame.extend_from_slice(&(payload.len() as u32).to_be_bytes());
        frame.extend_from_slice(&payload);
        Ok(frame)
    }

    async fn decode_message_frame(
        recv: &mut quinn::RecvStream,
        negotiated: NegotiatedProfile,
    ) -> Result<Message> {
        let mut magic = [0u8; 4];
        recv.read_exact(&mut magic).await?;
        if &magic != DATA_MAGIC {
            return Err(anyhow::anyhow!("invalid QUIC data frame magic"));
        }
        let mut version = [0u8; 1];
        recv.read_exact(&mut version).await?;
        if version[0] != WIRE_VERSION {
            return Err(anyhow::anyhow!("unsupported QUIC data frame version"));
        }
        let profile_id = Self::read_u16(recv).await?;
        if profile_id != negotiated.profile.id.wire_id() {
            return Err(anyhow::anyhow!("QUIC data frame profile mismatch"));
        }
        let payload_len = Self::read_u32(recv).await? as usize;
        if payload_len > negotiated.max_frame_size as usize {
            return Err(anyhow::anyhow!(
                "QUIC payload exceeds negotiated frame size"
            ));
        }
        let mut payload = vec![0u8; payload_len];
        recv.read_exact(&mut payload).await?;
        Ok(bincode::deserialize::<Message>(&payload)?)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        negotiate, CryptoCapabilities, CryptoProfile, CryptoProfileId, QuicTransportProvider,
    };

    #[test]
    fn debug_format() {
        let _ = std::format!("{:?}", "quic_transport debug test");
    }

    #[test]
    fn profile_binding_changes_when_capabilities_change() {
        let local = CryptoCapabilities::new(
            vec![CryptoProfileId::Gw0Session],
            CryptoProfile::GW0_SESSION.max_plaintext_fragment,
            false,
        )
        .unwrap();
        let remote = local.clone();
        let negotiated = negotiate(&local, &remote, 1).unwrap();
        let first = QuicTransportProvider::local_profile_binding(
            &local.canonical_bytes(),
            &remote.canonical_bytes(),
            negotiated,
        );

        let changed_remote = CryptoCapabilities::new(
            vec![CryptoProfileId::Gw0Session],
            CryptoProfile::GW0_SESSION.max_plaintext_fragment + 1,
            false,
        )
        .unwrap();
        let changed_negotiated = negotiate(&local, &changed_remote, 1).unwrap();
        let second = QuicTransportProvider::local_profile_binding(
            &local.canonical_bytes(),
            &changed_remote.canonical_bytes(),
            changed_negotiated,
        );
        assert_ne!(first, second);
    }
}
