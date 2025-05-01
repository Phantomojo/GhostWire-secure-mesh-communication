use anyhow::Result;
use crate::core::encryption::Encryption;
use crate::core::networking::{Networking};
use tokio::sync::mpsc;
use libp2p::swarm::SwarmEvent;
use log::info;
use std::sync::Arc;
use tokio::sync::Mutex;
use clap::Args;

#[derive(Args)]
pub struct Whisper {
    #[arg(help = "Recipient peer ID")]
    pub recipient: String,
    #[arg(help = "Message to send")]
    pub message: String,
}

#[derive(Args)]
pub struct Cloak {
    #[arg(long, help = "Enable or disable cloak")]
    pub enable: bool,
}

#[derive(Args)]
pub struct Drop {
    #[arg(help = "IOC file to share")]
    pub ioc_file: String,
}

#[derive(Args)]
pub struct Fetch {
    #[arg(long, help = "Category filter (IP/hash/domain)")]
    pub category: Option<String>,
    #[arg(long, help = "Reputation filter (high/low)")]
    pub reputation: Option<String>,
}

#[derive(Args)]
pub struct Peers {
    #[arg(long, help = "List peers")]
    pub list: bool,
    #[arg(long, help = "Show status")]
    pub status: bool,
}

#[derive(Args)]
pub struct Trust {
    #[arg(long, help = "Peer ID to view or update")]
    pub peer_id: Option<String>,
    #[arg(long, help = "Score to update")]
    pub score: Option<i32>,
}

#[async_trait::async_trait]
pub trait Command {
    async fn execute(&self) -> Result<()>;
}

#[async_trait::async_trait]
impl Command for Whisper {
    async fn execute(&self) -> Result<()> {
        // Initialize encryption engine
        let encryption = Encryption::new()?;

        // Setup channel for networking events
        let (event_sender, mut event_receiver) = mpsc::unbounded_channel();

        // Initialize networking
        let networking = Arc::new(Mutex::new(Networking::new(event_sender).await?));

        // Start networking in background task
        let networking_clone = networking.clone();
        tokio::spawn(async move {
            let mut net = networking_clone.lock().await;
            if let Err(e) = net.start().await {
                eprintln!("Networking error: {:?}", e);
            }
        });

        // Send encrypted message to recipient peer
        let peer_id = self.recipient.parse()?;
        {
            let mut net = networking.lock().await;
            let encrypted_message = encryption.encrypt(self.message.as_bytes());
            let request_id = net.send_message(&peer_id, encrypted_message).await?;
            info!("Sent message with request id: {:?}", request_id);
        }

        // Process networking events and handle incoming messages
        while let Some(event) = event_receiver.recv().await {
            info!("Network event: {:?}", event);
            // Handle incoming messages and decrypt them
            if let libp2p::swarm::SwarmEvent::Behaviour(libp2p::request_response::RequestResponseEvent::Message { peer, message }) = &event {
                match message {
                    libp2p::request_response::RequestResponseMessage::Request { request, channel, .. } => {
                        info!("Received request from {}: {:?}", peer, request);
                        // Decrypt the incoming message
                        match encryption.decrypt(&request) {
                            Ok(plaintext) => {
                                let msg_str = String::from_utf8_lossy(&plaintext);
                                info!("Decrypted message from {}: {}", peer, msg_str);
                                // Use public method to send response instead of accessing private field
                                let mut net = networking.lock().await;
                                if let Err(e) = net.send_response(channel.clone(), request.clone()).await {
                                    eprintln!("Failed to send response: {:?}", e);
                                }
                            }
                            Err(e) => {
                                eprintln!("Failed to decrypt message from {}: {:?}", peer, e);
                            }
                        }
                    }
                    libp2p::request_response::RequestResponseMessage::Response { response, .. } => {
                        info!("Received response from {}: {:?}", peer, response);
                    }
                }
            }
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl Command for Cloak {
    async fn execute(&self) -> Result<()> {
        println!("Cloak command placeholder");
        Ok(())
    }
}

#[async_trait::async_trait]
impl Command for Drop {
    async fn execute(&self) -> Result<()> {
        println!("Drop command placeholder");
        Ok(())
    }
}

#[async_trait::async_trait]
impl Command for Fetch {
    async fn execute(&self) -> Result<()> {
        println!("Fetch command placeholder");
        Ok(())
    }
}

#[async_trait::async_trait]
impl Command for Peers {
    async fn execute(&self) -> Result<()> {
        println!("Peers command placeholder");
        Ok(())
    }
}

#[async_trait::async_trait]
impl Command for Trust {
    async fn execute(&self) -> Result<()> {
        println!("Trust command placeholder");
        Ok(())
    }
}
