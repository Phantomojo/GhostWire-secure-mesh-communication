// Copyright (C) 2025-2026 Michael Irungu
// SPDX-License-Identifier: AGPL-3.0-or-later

use anyhow::Result;
#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};
use std::ffi::OsString;
use std::sync::Arc;

use tracing::{error, info};

// Use modules from the library crate
use ghostwire::config::Config;
use ghostwire::core::updater;
use ghostwire::core::Core;

/// Initialize logging with structured JSON output for production
fn init_logging() {
    // Production: JSON format for log aggregation
    #[cfg(not(debug_assertions))]
    {
        tracing_subscriber::fmt()
            .json()
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .init();
        info!(target: "ghostwire", "Structured JSON logging initialized");
    }

    // Development: Pretty format for readability
    #[cfg(debug_assertions)]
    {
        tracing_subscriber::fmt()
            .pretty()
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .init();
        info!(target: "ghostwire", "Pretty logging initialized");
    }
}
#[cfg(any(feature = "web", feature = "cli"))]
#[derive(Parser)]
#[cfg(feature = "cli")]
#[command(name = "ghostwire")]
#[command(about = "GhostWire - Secure Mesh Networking and Messaging")]
#[command(version = "0.1.0")]
#[command(disable_help_flag = true)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,

    #[arg(long, alias = "host", default_value = "127.0.0.1")]
    api_host: String,

    #[arg(long, default_value = "0.0.0.0")]
    mesh_host: String,

    #[arg(long, default_value_t = 0)]
    mesh_port: u16,

    #[arg(long, default_value_t = 8080)]
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

    /// Tailscale bootstrap node (format: <tailscale-ip>/<peer-id>)
    /// Can be specified multiple times. Also scans 100.64.0.0/10 if TAILSCALE=1.
    #[arg(long)]
    tailscale_bootstrap: Vec<String>,
}

#[cfg(not(feature = "cli"))]
struct Args {
    command: Option<Commands>,
    api_host: String,
    mesh_host: String,
    mesh_port: u16,
    port: u16,
    bootstrap: Vec<String>,
    web: bool,
    cli: bool,
    mesh: bool,
    profile: Option<String>,
    tailscale_bootstrap: Vec<String>,
}

#[cfg(feature = "cli")]
#[derive(Subcommand)]
enum Commands {
    /// Run CLI mode
    Cli {
        #[arg(num_args = 0..)]
        rest: Vec<String>,
    },
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
    /// Interactive terminal user interface
    Tui,
}

#[cfg(not(feature = "cli"))]
enum Commands {
    Cli,
    Web { host: String, port: u16 },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize structured logging
    init_logging();

    info!("🚀 Starting GhostWire - Secure Mesh Networking and Messaging");
    info!("🔒 Security features: End-to-end encryption, threat detection, anonymity");

    #[cfg(feature = "cli")]
    let args = Args::parse();
    #[cfg(not(feature = "cli"))]
    let args = Args {
        command: None,
        api_host: "127.0.0.1".to_string(),
        mesh_host: "0.0.0.0".to_string(),
        mesh_port: 0,
        port: 8080,
        bootstrap: Vec::new(),
        web: false,
        cli: false,
        mesh: false,
        profile: None,
        tailscale_bootstrap: Vec::new(),
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
    if config.server.auto_update
        && updater::is_update_available() {
            info!("✨ Applying automatic update...");
            if let Err(e) = updater::apply_update() {
                error!("❌ Automatic update failed: {}", e);
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

    // Feature 3: Parse Tailscale bootstrap nodes
    // Format: <tailscale-ip>/<peer-id> e.g. 100.64.1.5/12D3KooW...
    for ts in &args.tailscale_bootstrap {
        if let Some((ip, peer_id)) = ts.split_once('/') {
            if let Ok(peer_id) = peer_id.parse::<libp2p::PeerId>() {
                let addr_str = format!("/ip4/{}/tcp/4001", ip);
                if let Ok(ma) = addr_str.parse::<libp2p::Multiaddr>() {
                    let full_addr = ma.with(libp2p::multiaddr::Protocol::P2p(peer_id));
                    tracing::info!("Adding Tailscale bootstrap node: {} ({})", peer_id, full_addr);
                    bootstrap_multiaddrs.push(full_addr);
                }
            } else {
                tracing::warn!("Failed to parse Tailscale bootstrap PeerId: {}", peer_id);
            }
        } else {
            tracing::warn!(
                "Invalid Tailscale bootstrap format: {} (expected <ip>/<peer-id>)",
                ts
            );
        }
    }

    // Feature 3: If TAILSCALE=1, scan for GhostWire nodes on Tailscale network (100.64.0.0/10)
    if std::env::var("TAILSCALE").unwrap_or_default() == "1" {
        tracing::info!("TAILSCALE=1 detected — scanning 100.64.0.0/10 for GhostWire peers");
        // Note: Active scanning requires knowing peer IPs. In practice, Tailscale's
        // built-in DNS (tailnet names) or the Tailscale API would provide peer lists.
        // For now, we log this and rely on --tailscale-bootstrap for explicit peers.
        // Future: integrate with `tailscale status --json` to discover peers.
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
        Some(Commands::Cli { rest }) => {
            // Pass the already-created Core to CLI (avoid double-open of sled DB)
            let mut cli_args = vec![OsString::from("ghostwire")];
            cli_args.extend(rest.into_iter().map(OsString::from));
            ghostwire::cli::run_cli_with_args_and_core(cli_args, core_arc).await?;
        }
        #[cfg(feature = "cli")]
        Some(Commands::Status) => {
            ghostwire::cli::show_stats(&core_arc).await?;
        }
        #[cfg(feature = "cli")]
        Some(Commands::Peers) => {
            ghostwire::cli::list_peers(&core_arc).await?;
        }
        #[cfg(feature = "cli")]
        Some(Commands::Send { recipient, content }) => {
            ghostwire::cli::send_message(&core_arc, &recipient, &content).await?;
        }
        #[cfg(feature = "cli")]
        Some(Commands::Tui) => {
            info!("Starting TUI mode");
            ghostwire::tui::run_tui(core_arc).await
                .map_err(|e| anyhow::anyhow!("TUI error: {}", e))?;
        }
        #[cfg(feature = "cli")]
        Some(Commands::Web { api_host, mesh_host, port, bootstrap }) => {
            info!("Starting web server via CLI on {}:{}", api_host, port);
            let mut bs = bootstrap_multiaddrs.clone();
            for b in &bootstrap {
                if let Ok(ma) = b.parse::<libp2p::Multiaddr>() { bs.push(ma); }
            }
            core_arc.clone().start_nervous_system(mesh_host.clone(), 0).await;
            ghostwire::web::start_web_server(
                Some(core_arc), api_host, port, bs, None, mesh_host, 0, config.clone(),
            ).await.map_err(|e| anyhow::anyhow!(e))?;
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
                .start_nervous_system(args.mesh_host.clone(), args.mesh_port)
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
                    args.mesh_port,
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
