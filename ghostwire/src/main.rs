mod core;
mod cli;
mod web;
// mod ioc;
// mod messaging;
// mod trust;
// mod tor_integration;

use clap::Parser;
use cli::GhostWireCli;
use core::identity::EphemeralIdentity;
use core::store::MessageCache;
use core::transport::MockTransport;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    env_logger::init();

    // Initialize ephemeral identity
    let identity = EphemeralIdentity::new()?;
    // Initialize message cache
    let cache = MessageCache::new();
    // Initialize transport (mock for now)
    let transport = MockTransport::new().await?;
    // Initialize encryption
    let encryption = core::encryption::Encryption::new()?;

    // Check if CLI arguments were provided
    let args: Vec<String> = std::env::args().collect();
    
    if args.len() > 1 {
        // Run CLI command and exit
        let cli = GhostWireCli::parse();
        cli.execute(&identity, &cache, &transport, &encryption).await
    } else {
        // Start web server
        let web_state = Arc::new(web::AppState::default());
        let app = web::app(web_state);
        
        println!("GhostWire Web Server starting on http://127.0.0.1:3000");
        println!("Use the web interface or run with CLI commands like: cargo run -- whisper <peer> <message>");
        
        let listener = tokio::net::TcpListener::bind("127.0.0.1:3000").await?;
        axum::serve(listener, app).await?;
        
        Ok(())
    }
}
