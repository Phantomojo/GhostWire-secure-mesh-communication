//! DHT-based Username Discovery System
//! Allows nodes to publish and lookup username → PeerID mappings across the internet

use anyhow::{anyhow, Result};
use libp2p::kad::{Event as KademliaEvent, GetRecordOk, QueryResult, RecordKey};
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, info, warn};

/// Username record stored in DHT
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsernameRecord {
    pub username: String,
    pub peer_id: String,
    pub public_key: Vec<u8>,
    pub timestamp: u64,
    pub signature: Vec<u8>, // Signature of (username || peer_id || timestamp)
}

impl UsernameRecord {
    /// Create and sign a new username record
    pub fn new(
        username: String,
        peer_id: String,
        public_key: Vec<u8>,
        private_key: &[u8],
    ) -> Result<Self> {
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        // Create signature data
        let mut sig_data = Vec::new();
        sig_data.extend_from_slice(username.as_bytes());
        sig_data.extend_from_slice(peer_id.as_bytes());
        sig_data.extend_from_slice(&timestamp.to_le_bytes());

        // Sign with Ed25519
        use ring::signature::Ed25519KeyPair;
        let key_pair = Ed25519KeyPair::from_pkcs8(private_key)
            .map_err(|e| anyhow!("Failed to load key pair: {:?}", e))?;
        let signature = key_pair.sign(&sig_data);

        Ok(Self {
            username,
            peer_id,
            public_key,
            timestamp,
            signature: signature.as_ref().to_vec(),
        })
    }

    /// Create a record with a pre-computed signature (e.g. from libp2p Keypair).
    /// Use this when the signer is not PKCS8 (e.g. libp2p protobuf keypair).
    pub fn new_signed(
        username: String,
        peer_id: String,
        public_key: Vec<u8>,
        timestamp: u64,
        signature: Vec<u8>,
    ) -> Self {
        Self {
            username,
            peer_id,
            public_key,
            timestamp,
            signature,
        }
    }

    /// Verify the signature on this record
    pub fn verify(&self) -> Result<()> {
        // Reconstruct signature data
        let mut sig_data = Vec::new();
        sig_data.extend_from_slice(self.username.as_bytes());
        sig_data.extend_from_slice(self.peer_id.as_bytes());
        sig_data.extend_from_slice(&self.timestamp.to_le_bytes());

        // Verify signature
        let public_key =
            ring::signature::UnparsedPublicKey::new(&ring::signature::ED25519, &self.public_key);

        public_key
            .verify(&sig_data, &self.signature)
            .map_err(|e| anyhow!("Signature verification failed: {:?}", e))?;

        Ok(())
    }

    /// Serialize to bytes for DHT storage
    pub fn to_bytes(&self) -> Result<Vec<u8>> {
        bincode::serialize(self).map_err(|e| anyhow!("Failed to serialize username record: {}", e))
    }

    /// Deserialize from bytes
    pub fn from_bytes(data: &[u8]) -> Result<Self> {
        bincode::deserialize(data)
            .map_err(|e| anyhow!("Failed to deserialize username record: {}", e))
    }
}

/// DHT-based username discovery manager
pub struct UsernameDiscovery {
    local_peer_id: PeerId,
    local_username: String,
    // kad: Arc<RwLock<Kademlia<MemoryStore>>>, // Removed: Kademlia is owned by Swarm
    username_cache: Arc<RwLock<HashMap<String, UsernameRecord>>>,
    peer_cache: Arc<RwLock<HashMap<String, String>>>, // PeerID -> Username
}

impl UsernameDiscovery {
    pub fn new(local_peer_id: PeerId, local_username: String) -> Self {
        Self {
            local_peer_id,
            local_username,
            username_cache: Arc::new(RwLock::new(HashMap::new())),
            peer_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Prepare a username record for publication
    /// Returns (Key, Value) to be put into DHT by MeshTransport
    pub fn prepare_username_publication(
        &self,
        public_key: Vec<u8>,
        private_key: &[u8],
    ) -> Result<(RecordKey, Vec<u8>)> {
        info!(
            "Preparing username '{}' for DHT publication",
            self.local_username
        );

        // Create signed record
        let record = UsernameRecord::new(
            self.local_username.clone(),
            self.local_peer_id.to_string(),
            public_key,
            private_key,
        )?;

        // Derive DHT key from username (hash for privacy)
        let username_key = self.derive_username_key(&self.local_username);

        // Serialize record
        let record_bytes = record.to_bytes()?;

        Ok((RecordKey::new(&username_key), record_bytes))
    }

    /// Prepare a username record using an external signer (e.g. libp2p Keypair::sign).
    /// Use this when you have a keypair that is not in PKCS8 format.
    pub fn prepare_username_publication_with_signer<F>(
        &self,
        public_key: Vec<u8>,
        sign: F,
    ) -> Result<(RecordKey, Vec<u8>)>
    where
        F: FnOnce(&[u8]) -> Result<Vec<u8>>,
    {
        info!(
            "Preparing username '{}' for DHT publication (with signer)",
            self.local_username
        );

        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();

        let mut sig_data = Vec::new();
        sig_data.extend_from_slice(self.local_username.as_bytes());
        sig_data.extend_from_slice(self.local_peer_id.to_string().as_bytes());
        sig_data.extend_from_slice(&timestamp.to_le_bytes());

        let signature = sign(&sig_data)?;

        let record = UsernameRecord::new_signed(
            self.local_username.clone(),
            self.local_peer_id.to_string(),
            public_key,
            timestamp,
            signature,
        );

        let username_key = self.derive_username_key(&self.local_username);
        let record_bytes = record.to_bytes()?;

        Ok((RecordKey::new(&username_key), record_bytes))
    }

    /// Lookup a peer by username
    /// Returns Cached(Record) or DhtQuery(Key)
    pub async fn find_peer_by_username(&self, username: &str) -> Result<UsernameLookupResult> {
        // Check cache first
        {
            let cache = self.username_cache.read().await;
            if let Some(record) = cache.get(username) {
                // Check if record is still fresh (< 1 hour old)
                let now = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs();

                if now - record.timestamp < 3600 {
                    debug!("Found username '{}' in cache", username);
                    return Ok(UsernameLookupResult::Cached(record.clone()));
                }
            }
        }

        info!("Looking up username '{}' in DHT", username);

        // Derive DHT key
        let username_key = self.derive_username_key(username);

        Ok(UsernameLookupResult::DhtQuery(RecordKey::new(
            &username_key,
        )))
    }

    /// Handle DHT events (call this from swarm event loop)
    pub async fn handle_kad_event(&self, event: &KademliaEvent) -> Result<()> {
        if let KademliaEvent::OutboundQueryProgressed { result, .. } = event {
            match result {
                QueryResult::GetRecord(Ok(GetRecordOk::FoundRecord(record))) => {
                    // Got a username record from DHT
                    match UsernameRecord::from_bytes(&record.record.value) {
                        Ok(username_record) => {
                            // Verify signature
                            if let Err(e) = username_record.verify() {
                                warn!("Invalid signature on username record: {}", e);
                                return Ok(());
                            }

                            info!(
                                "Found username '{}' -> PeerID {}",
                                username_record.username, username_record.peer_id
                            );

                            // Cache it
                            let mut cache = self.username_cache.write().await;
                            cache.insert(
                                username_record.username.clone(),
                                username_record.clone(),
                            );

                            let mut peer_cache = self.peer_cache.write().await;
                            peer_cache.insert(
                                username_record.peer_id.clone(),
                                username_record.username.clone(),
                            );
                        }
                        Err(e) => {
                            warn!("Failed to parse username record: {}", e);
                        }
                    }
                }
                QueryResult::PutRecord(Ok(_)) => {
                    info!("Username record published to DHT");
                }
                QueryResult::GetRecord(Err(e)) => {
                    debug!("DHT get record failed: {:?}", e);
                }
                _ => {}
            }
        }

        Ok(())
    }

    /// Derive a consistent DHT key from username
    /// Uses SHA-256 hash for privacy (username not directly visible in DHT)
    fn derive_username_key(&self, username: &str) -> Vec<u8> {
        use ring::digest;

        // Hash username for privacy
        let hash = digest::digest(&digest::SHA256, username.as_bytes());
        hash.as_ref().to_vec()
    }

    /// Get username for a peer (from cache)
    pub async fn get_username_for_peer(&self, peer_id: &str) -> Option<String> {
        let cache = self.peer_cache.read().await;
        cache.get(peer_id).cloned()
    }
}

pub enum UsernameLookupResult {
    Cached(UsernameRecord),
    DhtQuery(RecordKey),
}

#[cfg(test)]
mod tests {
    use super::*;
    use ring::signature::KeyPair;

    #[test]
    fn test_username_record_creation() {
        use ring::rand::SystemRandom;
        use ring::signature::Ed25519KeyPair;

        let rng = SystemRandom::new();
        let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let keypair = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).unwrap();
        let public_key = keypair.public_key().as_ref().to_vec();

        let record = UsernameRecord::new(
            "alice".to_string(),
            "12D3KooWTest".to_string(),
            public_key,
            pkcs8.as_ref(),
        )
        .unwrap();

        assert_eq!(record.username, "alice");
        assert!(record.verify().is_ok());
    }

    #[test]
    fn test_username_record_serialization() {
        use ring::rand::SystemRandom;
        use ring::signature::Ed25519KeyPair;

        let rng = SystemRandom::new();
        let pkcs8 = Ed25519KeyPair::generate_pkcs8(&rng).unwrap();
        let keypair = Ed25519KeyPair::from_pkcs8(pkcs8.as_ref()).unwrap();
        let public_key = keypair.public_key().as_ref().to_vec();

        let record = UsernameRecord::new(
            "bob".to_string(),
            "12D3KooWTest2".to_string(),
            public_key,
            pkcs8.as_ref(),
        )
        .unwrap();

        let bytes = record.to_bytes().unwrap();
        let decoded = UsernameRecord::from_bytes(&bytes).unwrap();

        assert_eq!(record.username, decoded.username);
        assert_eq!(record.peer_id, decoded.peer_id);
        assert!(decoded.verify().is_ok());
    }
}
