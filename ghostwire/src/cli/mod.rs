pub mod commands;

use clap::{Parser, Subcommand};
use anyhow::Result;
use crate::cli::commands::Command;

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
    pub async fn execute(&self) -> Result<()> {
        match &self.command {
            Commands::Whisper(cmd) => cmd.execute().await,
            Commands::Cloak(cmd) => cmd.execute().await,
            Commands::Drop(cmd) => cmd.execute().await,
            Commands::Fetch(cmd) => cmd.execute().await,
            Commands::Peers(cmd) => cmd.execute().await,
            Commands::Trust(cmd) => cmd.execute().await,
        }
    }
}
