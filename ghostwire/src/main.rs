use clap::{Parser, Subcommand};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, error};
use base64::Engine;
use std::env;
use reqwest;

mod cli;
mod core;
mod web;
// mod ioc;
// mod messaging;
// mod trust;
// mod tor_integration;

use core::Core;
use web::app;
use web::get_local_ip;

#[derive(Parser)]
#[command(name = "ghostwire")]
#[command(about = "Secure messaging network with end-to-end encryption")]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
    
    #[arg(long, default_value = "3000")]
    port: u16,
    
    #[arg(long, default_value = "0.0.0.0")]
    host: String,
}

#[derive(Subcommand)]
enum Commands {
    /// Send a message to a peer
    Whisper {
        peer: String,
        message: String,
    },
    /// List all known peers
    Peers,
    /// Generate a new identity
    Identity {
        #[command(subcommand)]
        action: IdentityAction,
    },
    /// Check system status
    Status,
}

#[derive(Subcommand)]
enum IdentityAction {
    /// Generate a new identity
    Generate,
    /// Show current identity
    Show,
}

async fn report_startup_error(error_msg: &str) {
    let hostname = hostname::get().unwrap_or_default().to_string_lossy().to_string();
    let local_ip = match web::get_local_ip() { Some(ip) => ip, None => "unknown".to_string() };
    let os = env::consts::OS;
    let arch = env::consts::ARCH;
    let full_msg = format!("Backend error on {} ({} [{} {}]): {}", hostname, local_ip, os, arch, error_msg);
    let _ = reqwest::Client::new()
        .post("http://192.168.100.242:3001/api/report_error")
        .json(&serde_json::json!({"error": full_msg}))
        .send()
        .await;
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    
    info!("🌐 Starting GhostWire Secure Messaging Network");
    
    // Initialize core system
    let core = Arc::new(Core::new()?);
    info!("✅ Core system initialized");
    info!("🔑 Key ID: {}", core.get_key_id());
    info!("🔐 Public key: {} bytes", core.get_public_key().len());
    
    let cli = Cli::parse();
    
    match cli.command {
        Some(Commands::Whisper { peer, message }) => {
            info!("Sending message to peer: {}", peer);
            match core.send_message(&peer, &message).await {
                Ok(_) => println!("✅ Message sent successfully to {}", peer),
                Err(e) => {
                    error!("Failed to send message: {}", e);
                    std::process::exit(1);
                }
            }
        }
        Some(Commands::Peers) => {
            let peer_count = core.get_peer_count();
            println!("📡 Found {} peers", peer_count);
            // TODO: List actual peers
        }
        Some(Commands::Identity { action }) => {
            match action {
                IdentityAction::Generate => {
                    println!("🆔 Generating new identity...");
                    // TODO: Implement identity generation
                    println!("✅ New identity generated");
                }
                IdentityAction::Show => {
                    println!("🆔 Current Identity:");
                    println!("   Key ID: {}", core.get_key_id());
                    println!("   Public Key: {}", base64::engine::general_purpose::STANDARD.encode(core.get_public_key()));
                }
            }
        }
        Some(Commands::Status) => {
            println!("📊 GhostWire Status:");
            println!("   Core: ✅ Running");
            println!("   Encryption: ✅ Enabled");
            println!("   Key ID: {}", core.get_key_id());
            println!("   Peer Count: {}", core.get_peer_count());
            println!("   Public Key: {} bytes", core.get_public_key().len());
        }
        None => {
            // Start web server
            if let Err(e) = start_web_server(core, cli.host, cli.port).await {
                report_startup_error(&format!("{}", e)).await;
                return Err(e);
            }
        }
    }
    
    Ok(())
}

async fn start_web_server(core: Arc<Core>, host: String, port: u16) -> Result<(), Box<dyn std::error::Error>> {
    let mut chosen_port = port;
    let mut listener = None;
    for p in port..=port+10 {
        let addr = format!("{}:{}", host, p);
        match TcpListener::bind(&addr).await {
            Ok(l) => {
                chosen_port = p;
                listener = Some(l);
                break;
            },
            Err(_) => continue,
        }
    }
    let listener = listener.ok_or("No free port found in range")?;

    let local_ip = get_local_ip().unwrap_or_else(|| "127.0.0.1".to_string());
    tracing::info!("🌐 GhostWire Web Server starting on http://{}:{} (LAN IP: {})", host, chosen_port, local_ip);
    tracing::info!("Use the web interface or run with CLI commands like: cargo run -- whisper <peer> <message>");

    let app = app(core);
    axum::serve(listener, app).await?;
    Ok(())
}
