mod core;
mod cli;
// mod ioc;
// mod messaging;
// mod trust;
// mod tor_integration;

use clap::Parser;
use cli::GhostWireCli;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    let cli = GhostWireCli::parse();
    cli.execute().await
}
