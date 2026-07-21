//! CryptoProcessor: The I/O driver for the Sans I/O Crypto Machine.
//! This component handles side effects: networking, storage, and timers.

use crate::core::crypto_machine::{CryptoAction, CryptoEvent, CryptoMachine, GhostCryptoMachine};
use crate::core::identity::Identity;
use aes_gcm::aead::Aead;
use std::fs;
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};

pub struct CryptoProcessor {
    machine: Arc<RwLock<GhostCryptoMachine>>,
    #[allow(dead_code)]
    identity: Arc<Identity>,
    action_handler: mpsc::Sender<CryptoAction>,
    storage_path: PathBuf,
}

impl CryptoProcessor {
    pub fn new(
        identity: Arc<Identity>,
        storage_path: PathBuf,
    ) -> (Self, mpsc::Receiver<CryptoAction>) {
        let (tx, rx) = mpsc::channel(100);

        // Attempt to load existing machine state
        // FIX #13: Log error when load fails instead of silent fallback
        let machine = match Self::load_state(&storage_path) {
            Ok(m) => m,
            Err(e) => {
                tracing::warn!(
                    error = %e,
                    path = %storage_path.display(),
                    "Failed to load crypto state from disk, creating new machine"
                );
                GhostCryptoMachine::new(
                    identity.id.clone(),
                    identity.public_key_bytes(),
                )
            }
        };

        (
            Self {
                machine: Arc::new(RwLock::new(machine)),
                identity,
                action_handler: tx,
                storage_path,
            },
            rx,
        )
    }

    fn load_state(path: &PathBuf) -> anyhow::Result<GhostCryptoMachine> {
        let data = fs::read(path)?;
        let machine = bincode::deserialize(&data)?;
        Ok(machine)
    }

    pub async fn save_state(&self) -> anyhow::Result<()> {
        let machine = self.machine.read().await;
        let data = bincode::serialize(&*machine)?;
        fs::write(&self.storage_path, data)?;
        Ok(())
    }

    pub async fn process_event(&self, event: CryptoEvent) -> anyhow::Result<()> {
        let actions = {
            let mut machine = self.machine.write().await;
            machine.handle_event(event)
        };

        // FIX #14: Properly handle channel send errors instead of silent drops
        for action in actions {
            match action {
                CryptoAction::StoreSecret { .. } => {
                    // Trigger an immediate save when secrets are updated
                    if let Err(e) = self.save_state().await {
                        tracing::error!(error = %e, "Failed to save crypto state after secret update");
                    }
                }
                CryptoAction::DecryptSealedPacket {
                    sender_id,
                    ephemeral_pub,
                    ciphertext,
                } => {
                    use crypto_box::{ChaChaBox, PublicKey, SecretKey};

                    let my_sk = SecretKey::from(self.identity.x25519_secret);
                    let ephem_pk = PublicKey::from(ephemeral_pub);
                    let b = ChaChaBox::new(&ephem_pk, &my_sk);

                    if ciphertext.len() >= 24 {
                        let (nonce_bytes, real_ciphertext) = ciphertext.split_at(24);
                        let nonce = crypto_box::Nonce::from_slice(nonce_bytes);

                        if let Ok(plaintext) = b.decrypt(nonce, real_ciphertext) {
                            if let Err(e) = self
                                .action_handler
                                .send(CryptoAction::MessageDecrypted {
                                    sender_id,
                                    plaintext,
                                })
                                .await
                            {
                                tracing::error!(
                                    error = %e,
                                    "Failed to send MessageDecrypted action"
                                );
                            }
                        } else {
                            let msg = "Failed to decrypt sealed packet".to_string();
                            tracing::warn!(sender_id = %sender_id, "{}", msg);
                            if let Err(e) = self
                                .action_handler
                                .send(CryptoAction::Log {
                                    level: "WARN".into(),
                                    message: msg,
                                })
                                .await
                            {
                                tracing::error!(
                                    error = %e,
                                    "Failed to queue decryption failure log"
                                );
                            }
                        }
                    }
                }
                CryptoAction::Log { ref level, ref message } => {
                    // Log action is informational, use warn level for errors
                    if level == "WARN" || level == "ERROR" {
                        tracing::warn!(level = %level, message = %message, "Crypto log action");
                    } else {
                        tracing::info!(level = %level, message = %message, "Crypto log action");
                    }
                    // Don't send Log actions to handler (would cause infinite loop)
                }
                CryptoAction::DepositMailbox { .. } | CryptoAction::PublishPrekeys { .. } => {
                    // These are informational actions, log if send fails
                    if let Err(e) = self.action_handler.send(action).await {
                        tracing::warn!(error = %e, "Failed to queue crypto action");
                    }
                }
                _ => {
                    // Critical actions must be sent
                    self.action_handler.send(action).await.map_err(|e| {
                        anyhow::anyhow!("Crypto action channel closed: {}", e)
                    })?;
                }
            }
        }

        Ok(())
    }

    pub async fn run_heartbeat(&self) {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(60));
        loop {
            interval.tick().await;
            let now = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .expect("Time went backwards")
                .as_secs();

            let actions = {
                let mut machine = self.machine.write().await;
                machine.tick(now)
            };

            for action in actions {
                if let Err(e) = self.action_handler.send(action).await {
                    tracing::warn!(error = %e, "Failed to send heartbeat action");
                }
            }

            // Periodically save state even if no secrets changed
            if let Err(e) = self.save_state().await {
                tracing::error!(error = %e, "Failed to save state during heartbeat");
            }
        }
    }
}
