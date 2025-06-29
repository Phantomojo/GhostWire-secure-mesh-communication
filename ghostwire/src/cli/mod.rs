pub mod commands;

use clap::{Parser, Subcommand};
use anyhow::Result;
use crate::cli::commands::Command;
use crate::core::identity::EphemeralIdentity;
use crate::core::store::MessageCache;
use crate::core::transport::MockTransport;
use crate::core::encryption::Encryption;

#[derive(Parser)]
#[command(name = "ghostwire")]
#[command(about = "GhostWire CLI - Secure decentralized messaging and threat intel sharing", long_about = None)]
pub struct GhostWireCli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Whisper(commands::Whisper),
    Cloak(commands::Cloak),
    Drop(commands::Drop),
    Fetch(commands::Fetch),
    Peers(commands::Peers),
    Trust(commands::Trust),
}

impl GhostWireCli {
    pub async fn execute(&self, identity: &EphemeralIdentity, cache: &MessageCache, transport: &MockTransport, encryption: &Encryption) -> Result<()> {
        match &self.command {
            Commands::Whisper(cmd) => cmd.execute(identity, cache, transport, encryption).await,
            Commands::Cloak(cmd) => cmd.execute(identity, cache, transport, encryption).await,
            Commands::Drop(cmd) => cmd.execute(identity, cache, transport, encryption).await,
            Commands::Fetch(cmd) => cmd.execute(identity, cache, transport, encryption).await,
            Commands::Peers(cmd) => cmd.execute(identity, cache, transport, encryption).await,
            Commands::Trust(cmd) => cmd.execute(identity, cache, transport, encryption).await,
        }
    }
}
