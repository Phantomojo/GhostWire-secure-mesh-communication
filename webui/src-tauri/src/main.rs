use clap::{Parser, Subcommand};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, error};
use base64::Engine;
use std::env;
use reqwest;
use web::get_local_ip;
use hostname;
use tauri::Manager;

mod cli;
mod core;
mod web;
// mod ioc;
// mod messaging;
// mod trust;
// mod tor_integration;

use core::Core;
use web::app;

#[allow(dead_code)]
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

#[allow(dead_code)]
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

#[allow(dead_code)]
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
    
    // Start backend server in background
    let core_clone = core.clone();
    tokio::spawn(async move {
        let host = "0.0.0.0".to_string();
        let port = 3000u16;
        if let Err(e) = start_web_server(core_clone, host, port).await {
            tracing::error!("Backend server failed: {}", e);
        }
    });

    // Launch Tauri window
    tauri::Builder::default()
        .setup(|_app| {
            // You can add custom setup here if needed
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

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
