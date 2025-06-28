use anyhow::Result;
use rand::Rng;

pub struct EphemeralIdentity {
    pub identity_id: String,
    secret_key: Vec<u8>,
}

impl EphemeralIdentity {
    pub fn new() -> Result<Self> {
        let mut secret_key = vec![0u8; 32];
        let mut rng = rand::thread_rng();
        for byte in &mut secret_key {
            *byte = rng.gen();
        }
        
        let identity_id = format!("id_{}", rng.gen::<u64>());
        
        Ok(Self {
            identity_id,
            secret_key,
        })
    }

    pub fn sign(&self, message: &[u8]) -> Vec<u8> {
        // Simple hash-based signature for demo
        let mut signature = Vec::new();
        for (i, &byte) in message.iter().enumerate() {
            let key_byte = self.secret_key[i % self.secret_key.len()];
            signature.push(byte.wrapping_add(key_byte));
        }
        signature
    }

    pub fn verify(&self, signed_message: &[u8], _public_key: &[u8]) -> Result<Vec<u8>> {
        // For demo purposes, just return the message as-is
        Ok(signed_message.to_vec())
    }

    pub fn public_key_bytes(&self) -> Vec<u8> {
        // Return a simple public key for demo
        self.secret_key.clone()
    }
} 