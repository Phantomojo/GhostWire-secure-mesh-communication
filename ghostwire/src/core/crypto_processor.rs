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
        let machine = Self::load_state(&storage_path).unwrap_or_else(|_| {
            GhostCryptoMachine::new(
                identity.id.clone(),
                identity.keypair().public().encode_protobuf(),
            )
        });

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

        for action in actions {
            match action {
                CryptoAction::StoreSecret { .. } => {
                    // Trigger an immediate save when secrets are updated
                    let _ = self.save_state().await;
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
                            let _ = self
                                .action_handler
                                .send(CryptoAction::MessageDecrypted {
                                    sender_id,
                                    plaintext,
                                })
                                .await;
                        } else {
                            let _ = self
                                .action_handler
                                .send(CryptoAction::Log {
                                    level: "WARN".into(),
                                    message: "Failed to decrypt sealed packet".into(),
                                })
                                .await;
                        }
                    }
                }
                _ => {
                    self.action_handler.send(action).await?;
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
                .unwrap()
                .as_secs();

            let actions = {
                let mut machine = self.machine.write().await;
                machine.tick(now)
            };

            for action in actions {
                let _ = self.action_handler.send(action).await;
            }

            // Periodically save state even if no secrets changed
            let _ = self.save_state().await;
        }
    }
}
