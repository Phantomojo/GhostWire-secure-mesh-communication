use anyhow::Result;
use rand::Rng;

pub struct Encryption {
    secret_key: Vec<u8>,
}

impl Encryption {
    pub fn new() -> Result<Self> {
        let mut secret_key = vec![0u8; 32];
        let mut rng = rand::thread_rng();
        for byte in &mut secret_key {
            *byte = rng.gen();
        }
        
        Ok(Self {
            secret_key,
        })
    }

    pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        // Simple XOR encryption for demo purposes
        let mut ciphertext = Vec::new();
        for (i, &byte) in plaintext.iter().enumerate() {
            let key_byte = self.secret_key[i % self.secret_key.len()];
            ciphertext.push(byte ^ key_byte);
        }
        ciphertext
    }

    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        // XOR decryption is the same as encryption
        Ok(self.encrypt(ciphertext))
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

    pub fn public_key(&self) -> Vec<u8> {
        // Return a simple public key for demo
        self.secret_key.clone()
    }
} 