use ring::rand::{SecureRandom, SystemRandom};
use ring::signature::Ed25519KeyPair;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedMessage {
    pub ciphertext: Vec<u8>,
    pub nonce: Vec<u8>,
    pub signature: Vec<u8>,
    pub sender_id: String,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostKeyPair {
    pub public_key: Vec<u8>,
    pub private_key: Vec<u8>,
    pub key_id: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PeerKey {
    pub public_key: Vec<u8>,
    pub key_id: String,
    pub last_updated: u64,
}

pub struct EncryptionManager {
    key_pair: GhostKeyPair,
    peer_keys: Arc<Mutex<HashMap<String, PeerKey>>>,
    rng: SystemRandom,
}

impl EncryptionManager {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let rng = SystemRandom::new();
        
        // Generate Ed25519 key pair for signing
        let key_pair_bytes = Ed25519KeyPair::generate_pkcs8(&rng)
            .map_err(|e| format!("Failed to generate Ed25519 key pair: {:?}", e))?;
        let _key_pair_ed25519 = Ed25519KeyPair::from_pkcs8(key_pair_bytes.as_ref())
            .map_err(|e| format!("Failed to create Ed25519 key pair: {:?}", e))?;
        
        // Generate X25519 key pair for encryption
        let x25519_private_key = ring::agreement::EphemeralPrivateKey::generate(
            &ring::agreement::X25519,
            &rng,
        ).map_err(|e| format!("Failed to generate X25519 private key: {:?}", e))?;
        let x25519_public_key = x25519_private_key.compute_public_key()
            .map_err(|e| format!("Failed to compute X25519 public key: {:?}", e))?;
        // TODO: Cannot extract private key bytes from EphemeralPrivateKey in ring. Use empty Vec as placeholder.
        let key_pair = GhostKeyPair {
            public_key: x25519_public_key.as_ref().to_vec(),
            private_key: Vec::new(),
            key_id: Self::generate_key_id(),
        };
        
        Ok(EncryptionManager {
            key_pair,
            peer_keys: Arc::new(Mutex::new(HashMap::new())),
            rng,
        })
    }
    
    fn generate_key_id() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        format!("key_{:x}", timestamp)
    }
    
    pub fn get_public_key(&self) -> &[u8] {
        &self.key_pair.public_key
    }
    
    pub fn get_key_id(&self) -> &str {
        &self.key_pair.key_id
    }
    
    pub fn add_peer_key(&self, peer_id: String, public_key: Vec<u8>) {
        let mut peer_keys = self.peer_keys.lock().unwrap();
        peer_keys.insert(peer_id, PeerKey {
            public_key,
            key_id: Self::generate_key_id(),
            last_updated: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        });
    }
    
    pub fn encrypt_message(&self, message: &str, recipient_id: &str) -> Result<EncryptedMessage, Box<dyn std::error::Error>> {
        let peer_keys = self.peer_keys.lock().unwrap();
        let _peer_key = peer_keys.get(recipient_id)
            .ok_or("Recipient not found")?;
        
        // For now, use a simple encryption approach
        let key = self.derive_symmetric_key(recipient_id)?;
        let nonce_bytes = self.generate_nonce();
        
        // Simple XOR encryption for demo (replace with proper encryption)
        let mut ciphertext = message.as_bytes().to_vec();
        for (i, byte) in ciphertext.iter_mut().enumerate() {
            *byte ^= key[i % key.len()];
        }
        
        // Sign the encrypted message
        let signature = self.sign_message(&ciphertext)?;
        
        Ok(EncryptedMessage {
            ciphertext,
            nonce: nonce_bytes.to_vec(),
            signature,
            sender_id: self.key_pair.key_id.clone(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
        })
    }
    
    pub fn decrypt_message(&self, encrypted_msg: &EncryptedMessage) -> Result<String, Box<dyn std::error::Error>> {
        // Verify signature first
        self.verify_signature(&encrypted_msg.ciphertext, &encrypted_msg.signature)?;
        
        // For now, use symmetric key approach
        let key = self.derive_symmetric_key(&encrypted_msg.sender_id)?;
        
        // Simple XOR decryption for demo
        let mut plaintext = encrypted_msg.ciphertext.clone();
        for (i, byte) in plaintext.iter_mut().enumerate() {
            *byte ^= key[i % key.len()];
        }
        
        String::from_utf8(plaintext).map_err(|e| e.into())
    }
    
    // Simple encryption method for CLI compatibility
    pub fn encrypt(&self, data: &[u8]) -> Vec<u8> {
        // Simple XOR encryption for demo
        let key = b"ghostwire_demo_key_32_bytes_long!";
        let mut encrypted = data.to_vec();
        for (i, byte) in encrypted.iter_mut().enumerate() {
            *byte ^= key[i % key.len()];
        }
        encrypted
    }
    
    // Simple decryption method for CLI compatibility
    pub fn decrypt(&self, data: &[u8]) -> Vec<u8> {
        // Simple XOR decryption for demo
        let key = b"ghostwire_demo_key_32_bytes_long!";
        let mut decrypted = data.to_vec();
        for (i, byte) in decrypted.iter_mut().enumerate() {
            *byte ^= key[i % key.len()];
        }
        decrypted
    }
    
    // Public signature methods for core module
    pub fn sign_message(&self, message: &[u8]) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        // For now, use a simple HMAC-based signature
        let key = b"ghostwire_signature_key";
        let signature = ring::hmac::sign(&ring::hmac::Key::new(ring::hmac::HMAC_SHA256, key), message);
        Ok(signature.as_ref().to_vec())
    }
    
    pub fn verify_signature(&self, message: &[u8], signature: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
        let key = b"ghostwire_signature_key";
        let expected_signature = ring::hmac::sign(&ring::hmac::Key::new(ring::hmac::HMAC_SHA256, key), message);
        
        if signature == expected_signature.as_ref() {
            Ok(())
        } else {
            Err("Invalid signature".into())
        }
    }
    
    fn generate_nonce(&self) -> [u8; 12] {
        let mut nonce = [0u8; 12];
        self.rng.fill(&mut nonce).unwrap();
        nonce
    }
    
    fn derive_symmetric_key(&self, peer_id: &str) -> Result<[u8; 32], Box<dyn std::error::Error>> {
        // In a real implementation, this would use the peer's public key
        // For now, use a deterministic key derivation
        let mut key = [0u8; 32];
        let mut hasher = ring::digest::Context::new(&ring::digest::SHA256);
        hasher.update(peer_id.as_bytes());
        hasher.update(&self.key_pair.private_key);
        key.copy_from_slice(hasher.finish().as_ref());
        Ok(key)
    }
    
    pub fn export_public_key(&self) -> Vec<u8> {
        self.key_pair.public_key.clone()
    }
    
    pub fn get_peer_count(&self) -> usize {
        self.peer_keys.lock().unwrap().len()
    }
}

impl Default for EncryptionManager {
    fn default() -> Self {
        Self::new().expect("Failed to create encryption manager")
    }
}

// Alias for CLI compatibility
pub type Encryption = EncryptionManager;

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_encryption_roundtrip() {
        let manager = EncryptionManager::new().unwrap();
        let message = "Hello, GhostWire!";
        let recipient = "test_peer";
        
        // Add a dummy peer key
        manager.add_peer_key(recipient.to_string(), vec![1, 2, 3, 4]);
        
        let encrypted = manager.encrypt_message(message, recipient).unwrap();
        let decrypted = manager.decrypt_message(&encrypted).unwrap();
        
        assert_eq!(message, decrypted);
    }
    
    #[test]
    fn test_key_generation() {
        let manager = EncryptionManager::new().unwrap();
        assert!(!manager.get_public_key().is_empty());
        assert!(!manager.get_key_id().is_empty());
    }
} 