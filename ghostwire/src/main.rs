use anyhow::Result;
#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};
use std::sync::Arc;

use tracing::{error, info};

// Use modules from the library crate
use ghostwire::config::Config;
use ghostwire::core::updater;
use ghostwire::core::Core;
#[cfg(feature = "web")]
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
    api_host: String,

    #[arg(long, default_value = "0.0.0.0")]
    mesh_host: String,

    #[arg(long, default_value_t = 5050)]
    port: u16,

    #[arg(long)]
    bootstrap: Vec<String>,

    #[arg(long)]
    web: bool,

    #[arg(long)]
    cli: bool,

    #[arg(long)]
    mesh: bool,

    #[arg(long)]
    config: Option<String>,

    #[arg(long)]
    sim_addr: Option<String>,

    #[arg(long)]
    profile: Option<String>,
}

#[cfg(not(feature = "cli"))]
struct Args {
    command: Option<Commands>,
    api_host: String,
    mesh_host: String,
    port: u16,
    bootstrap: Vec<String>,
    web: bool,
    cli: bool,
    profile: Option<String>,
}

#[cfg(feature = "cli")]
#[derive(Subcommand)]
enum Commands {
    /// Run CLI mode
    Cli,
    /// Run web server mode
    Web {
        #[arg(long, default_value = "127.0.0.1")]
        api_host: String,
        #[arg(long, default_value = "0.0.0.0")]
        mesh_host: String,
        #[arg(long, default_value_t = 8080)]
        port: u16,
        #[arg(long)]
        bootstrap: Vec<String>,
    },
    Status,
    /// List known peers
    Peers,
    /// Send a message: send <recipient_id> <content>
    Send {
        recipient: String,
        content: String,
    },
}

#[cfg(not(feature = "cli"))]
enum Commands {
    Cli,
    Web { host: String, port: u16 },
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
    let args = Args {
        command: None,
        api_host: "127.0.0.1".to_string(),
        mesh_host: "0.0.0.0".to_string(),
        port: 8080,
        bootstrap: Vec::new(),
        web: false,
        cli: false,
        mesh: false,
        config: None,
        sim_addr: None,
        profile: None,
    };

    // Load configuration
    let config = if let Some(config_path) = &args.config {
        match Config::load_from_path(config_path) {
            Ok(c) => c,
            Err(e) => {
                error!("❌ Failed to load config from {}: {}", config_path, e);
                return Err(anyhow::anyhow!("Configuration load failed"));
            }
        }
    } else {
        Config::load().unwrap_or_else(|_| Config::default())
    };

    // Auto-update check
    if config.server.auto_update {
        if updater::is_update_available() {
            info!("✨ Applying automatic update...");
            if let Err(e) = updater::apply_update() {
                error!("❌ Automatic update failed: {}", e);
            }
        }
    }

    // Parse bootstrap nodes
    let mut bootstrap_multiaddrs = Vec::new();
    for b in &args.bootstrap {
        if let Ok(ma) = b.parse::<libp2p::Multiaddr>() {
            bootstrap_multiaddrs.push(ma);
        } else {
            tracing::warn!("Failed to parse bootstrap address: {}", b);
        }
    }

    // Initialize core components
    let core = Core::new(
        config.clone(),
        bootstrap_multiaddrs.clone(),
        args.sim_addr.clone(),
        args.profile,
    )
    .await?;
    let core_arc = Arc::new(core);

    match args.command {
        #[cfg(feature = "cli")]
        Some(_) => {
            // Let the unified CLI module handle the command
            ghostwire::cli::run_cli().await?;
        }
        #[cfg(not(feature = "cli"))]
        Some(_) => {
            println!("CLI feature not enabled.");
        }
        None => {
            // Default behavior: start the services
            // Activate the autonomic nervous system
            core_arc
                .clone()
                .start_nervous_system(args.mesh_host.clone())
                .await;

            if args.web || config.server.web_enabled {
                info!(
                    "Starting web server mode on {}:{}",
                    args.api_host, args.port
                );
                ghostwire::web::start_web_server(
                    Some(core_arc),
                    args.api_host,
                    args.port,
                    bootstrap_multiaddrs.clone(),
                    args.sim_addr.clone(),
                    args.mesh_host.clone(),
                    config.clone(),
                )
                .await
                .map_err(|e| anyhow::anyhow!(e))?;
            } else {
                info!("Running in minimal mode - core only");
                tokio::signal::ctrl_c().await?;
            }
        }
    }

    Ok(())
}
