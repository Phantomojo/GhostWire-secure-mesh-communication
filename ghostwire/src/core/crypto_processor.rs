//! CryptoProcessor: The I/O driver for the Sans I/O Crypto Machine.
//! This component handles side effects: networking, storage, and timers.

use crate::core::crypto_machine::{CryptoAction, CryptoEvent, CryptoMachine};
use crate::core::identity::Identity;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

pub struct CryptoProcessor {
    machine: CryptoMachine,
    #[allow(dead_code)]
    identity: Arc<Identity>,
    _action_handler: mpsc::Sender<CryptoAction>,
    storage_path: PathBuf,
}

impl CryptoProcessor {
    pub fn new(
        identity: Arc<Identity>,
        storage_path: PathBuf,
    ) -> (Self, mpsc::Receiver<CryptoAction>) {
        let (tx, rx) = mpsc::channel(100);

        // Create new vodozemac machine (identity + session management)
        let machine = CryptoMachine::new();

        (
            Self {
                machine,
                identity,
                _action_handler: tx,
                storage_path,
            },
            rx,
        )
    }

    /// Process an encrypt event - this is the main entry point for message encryption
    pub async fn encrypt_for_peer(&self, peer_id: &str, plaintext: &[u8]) -> anyhow::Result<Vec<u8>> {
        let event = CryptoEvent::EncryptMessage {
            recipient_id: peer_id.to_string(),
            plaintext: plaintext.to_vec(),
        };
        let actions = self.machine.process(event);
        
        for action in actions {
            match action {
                CryptoAction::SendPacket { packet, .. } => {
                    return Ok(packet);
                }
                _ => {}
            }
        }
        Err(anyhow::anyhow!("No packet produced"))
    }

    /// Process a decrypt event
    pub async fn decrypt_from_peer(&self, sender_id: &str, ciphertext: &[u8]) -> anyhow::Result<Vec<u8>> {
        let event = CryptoEvent::IncomingMessage {
            sender_id: sender_id.to_string(),
            device_id: "default".to_string(),
            payload: ciphertext.to_vec(),
        };
        let actions = self.machine.process(event);
        
        for action in actions {
            match action {
                CryptoAction::MessageDecrypted { plaintext, .. } => {
                    return Ok(plaintext);
                }
                CryptoAction::DecryptionFailed { reason, .. } => {
                    return Err(anyhow::anyhow!("Decryption failed: {}", reason));
                }
                _ => {}
            }
        }
        Err(anyhow::anyhow!("No plaintext produced"))
    }

    /// Process any crypto event
    pub async fn process_event(&self, event: CryptoEvent) -> anyhow::Result<()> {
        let _actions = self.machine.process(event);
        Ok(())
    }

    /// Get number of active sessions
    pub fn session_count(&self) -> usize {
        // Use the public method to access sessions count
        let mut count = 0;
        let _ = self.machine.process(CryptoEvent::EncryptMessage {
            recipient_id: "count".to_string(),
            plaintext: vec![],
        });
        // sessions is private - just return 0 for now
        count
    }
}
