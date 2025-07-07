use clap::{Parser, Subcommand};
use std::sync::Arc;
use tokio::net::TcpListener;
use tracing::{info, error};
use base64::Engine;
use std::env;
use reqwest;
use web::get_local_ip;
use hostname;
use crate::core::Core;
use crate::web::start_web_server;
use anyhow::Result;

mod cli;
mod core;
mod web;
// mod ioc;
// mod messaging;
// mod trust;
// mod tor_integration;

#[derive(Parser)]
#[command(name = "ghostwire")]
#[command(about = "GhostWire - Secure Mesh Networking and Messaging")]
#[command(version = "0.1.0")]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
    
    #[arg(short, long, default_value = "127.0.0.1")]
    host: String,
    
    #[arg(short, long, default_value_t = 8080)]
    port: u16,
    
    #[arg(long)]
    web: bool,
    
    #[arg(long)]
    cli: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Run CLI mode
    Cli,
    /// Run web server mode
    Web {
        #[arg(short, long, default_value = "127.0.0.1")]
        host: String,
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
    },
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
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    info!("🚀 Starting GhostWire - Secure Mesh Networking and Messaging");
    info!("🔒 Security features: End-to-end encryption, threat detection, anonymity");
    
    let args = Args::parse();
    
    // Initialize core components
    let core = Core::new().await?;
    let core_arc = Arc::new(core);
    
    match args.command {
        Some(Commands::Cli) => {
            info!("Starting CLI mode");
            return cli::run_cli().await;
        }
        Some(Commands::Web { host, port }) => {
            info!("Starting web server mode on {}:{}", host, port);
            start_web_server(core_arc, host, port).await
                .map_err(|e| anyhow::anyhow!("Web server error: {}", e))?;
        }
        None => {
            // Default behavior based on flags
            if args.cli {
                info!("Starting CLI mode");
                return cli::run_cli().await;
            } else if args.web {
                info!("Starting web server mode on {}:{}", args.host, args.port);
                start_web_server(core_arc, args.host, args.port).await
                    .map_err(|e| anyhow::anyhow!("Web server error: {}", e))?;
            } else {
                // Default to web server
                info!("Starting web server mode on {}:{}", args.host, args.port);
                start_web_server(core_arc, args.host, args.port).await
                    .map_err(|e| anyhow::anyhow!("Web server error: {}", e))?;
            }
        }
    }
    
    Ok(())
}
