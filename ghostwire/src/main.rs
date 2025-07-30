#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};
use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;
use tracing::{info, error};
use base64::Engine;
use std::env;
#[cfg(feature = "web")]
use reqwest;
#[cfg(feature = "web")]
use web::get_local_ip;
use hostname;
use crate::core::Core;
use crate::web::start_web_server as imported_start_web_server;
use anyhow::Result;

// #[cfg(feature = "cli")]
// mod cli;
mod config;
mod core;
#[cfg(feature = "tui")]
mod tui;
#[cfg(feature = "web")]
mod web;
// mod ioc;
// mod messaging;
// mod trust;
// mod tor_integration;

#[derive(Parser)]
#[cfg(feature = "cli")]
#[command(name = "ghostwire")]
#[command(about = "GhostWire - Secure Mesh Networking and Messaging")]
#[command(version = "0.1.0")]
#[command(disable_help_flag = true)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
    
    #[arg(long, default_value = "127.0.0.1")]
    host: String,
    
    #[arg(long, default_value_t = 8080)]
    port: u16,
    
    #[arg(long)]
    web: bool,
    
    #[arg(long)]
    cli: bool,
    
    #[arg(long)]
    help: bool,
}

#[cfg(not(feature = "cli"))]
struct Args {
    command: Option<Commands>,
    host: String,
    port: u16,
    web: bool,
    cli: bool,
}

#[cfg(feature = "cli")]
#[derive(Subcommand)]
enum Commands {
    /// Run CLI mode
    Cli,
    /// Run web server mode
    Web {
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        #[arg(long, default_value_t = 8080)]
        port: u16,
    },
    /// Run TUI mode
    Tui,
}

#[cfg(not(feature = "cli"))]
enum Commands {
    Cli,
    Web { host: String, port: u16 },
    Tui,
}

#[cfg(feature = "web")]
async fn start_web_server(core: Arc<Core>, host: String, port: u16) -> Result<()> {
    web::start_web_server(core, host, port).await
}

#[cfg(feature = "web")]
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
    
    #[cfg(feature = "cli")]
    let args = Args::parse();
    #[cfg(not(feature = "cli"))]
    let args = Args { command: None, host: "127.0.0.1".to_string(), port: 8080, web: false, cli: false };
    
    // Initialize core components
    let core = Core::new().await?;
    let core_arc = Arc::new(core);
    
    match args.command {
        #[cfg(feature = "cli")]
        Some(Commands::Cli) => {
            info!("Starting CLI mode");
            // return cli::run_cli().await;
            println!("CLI mode is currently disabled.");
        }
        #[cfg(feature = "web")]
        Some(Commands::Web { host, port }) => {
            info!("Starting web server mode on {}:{}", host, port);
            start_web_server(core_arc, host, port).await
                .map_err(|e| anyhow::anyhow!(e))?;
        }
        #[cfg(feature = "tui")]
        Some(Commands::Tui) => {
            info!("Starting TUI mode");
            let core_for_tui = Core::new().await?;
            let core_mutex = Arc::new(Mutex::new(core_for_tui));
            tui::start_tui(core_mutex).await
                .map_err(|e| anyhow::anyhow!("TUI error: {}", e))?;
        }
        None => {
            // Default behavior based on flags
            #[cfg(feature = "cli")]
            if args.cli {
                info!("Starting CLI mode");
                // return cli::run_cli().await;
                println!("CLI mode is currently disabled.");
            } else 
            if args.web {
                info!("Starting web server mode on {}:{}", args.host, args.port);
                start_web_server(core_arc, args.host, args.port).await
                    .map_err(|e| anyhow::anyhow!(e))?;
            } else {
                // Default to web server if available, otherwise just run core
                {
                    info!("Starting web server mode on {}:{}", args.host, args.port);
                    start_web_server(core_arc, args.host, args.port).await
                        .map_err(|e| anyhow::anyhow!(e))?;
                }
                #[cfg(not(feature = "web"))]
                {
                    info!("Running in minimal mode - core only");
                    // Keep the core running
                    tokio::signal::ctrl_c().await?;
                }
            }
        }
    }
    
    Ok(())
}
