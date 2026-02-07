use anyhow::Result;
#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{error, info};

// Use modules from the library crate
use ghostwire::config::Config;
use ghostwire::core::updater;
use ghostwire::core::Core;
#[cfg(feature = "tui")]
use ghostwire::tui;
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
    help: bool,

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
    /// Run TUI mode
    Tui,
    /// Get node status
    Status,
    /// List known peers
    Peers,
    /// Send a message: send <recipient_id> <content>
    Send { recipient: String, content: String },
}

#[cfg(not(feature = "cli"))]
enum Commands {
    Cli,
    Web { host: String, port: u16 },
    Tui,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();

    info!("🚀 Starting GhostWire - Secure Mesh Networking and Messaging");
    info!("🔒 Security features: End-to-end encryption, threat detection, anonymity");

    // Load configuration for startup features
    if let Ok(config) = Config::load() {
        if config.server.auto_update {
            if updater::is_update_available() {
                info!("✨ Applying automatic update...");
                if let Err(e) = updater::apply_update() {
                    error!("❌ Automatic update failed: {}", e);
                }
            }
        }
    }

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
        sim_addr: None,
        profile: None,
    };

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
    // Initialize core components
    let core = Core::new(
        bootstrap_multiaddrs.clone(),
        args.sim_addr.clone(),
        args.profile,
    )
    .await?;
    let core_arc = Arc::new(core);

    // Activate the autonomic nervous system (DNA 10/10)
    // core_arc.clone().start_nervous_system().await; // Moved to inside match arms to use configured mesh_host

    match args.command {
        #[cfg(feature = "cli")]
        Some(Commands::Status) => {
            let status = core_arc.get_security_stats().await;
            let peers = core_arc.get_peer_count();
            let listeners = core_arc.get_mesh_listeners().await;

            println!("--- GhostWire Node Status ---");
            println!("Identity: {}", core_arc.get_identity_id().await);
            println!("Peers:    {}", peers);
            if !listeners.is_empty() {
                println!("Mesh Addresses:");
                for addr in listeners {
                    println!("  {}", addr);
                }
            }
            println!("Threat:   {:?}", status.threat_level);
            println!("Blocked:  {}", status.blocked_connections);
        }

        #[cfg(feature = "cli")]
        Some(Commands::Peers) => {
            println!("--- Known Mesh Peers ---");
            #[cfg(feature = "mesh")]
            if let Some(mesh) = &core_arc.mesh_manager {
                let topology = mesh.read().await.get_topology();
                for (id, node) in topology.nodes {
                    println!(
                        "ID: {} | Name: {} | Status: {}",
                        id,
                        node.username,
                        if node.is_online { "Online" } else { "Offline" }
                    );
                }
            } else {
                println!("Mesh networking not active.");
            }
        }
        #[cfg(feature = "cli")]
        Some(Commands::Send { recipient, content }) => {
            println!("📤 Sending message to {}...", recipient);
            let msg =
                ghostwire::core::Message::new(core_arc.get_identity_id().await, recipient, content);
            match core_arc.send_message(&msg).await {
                Ok(_) => println!("✅ Message dispatched to transport."),
                Err(e) => println!("❌ Failed to send: {}", e),
            }
        }
        #[cfg(feature = "cli")]
        Some(Commands::Cli) => {
            info!("Starting interactive CLI mode");
            println!("GhostWire Interactive Shell (Type 'exit' to quit)");
            // Simple interactive loop could go here, but for now we prioritize subcommands
            // which are better for simulation scripts.
        }
        #[cfg(feature = "web")]
        Some(Commands::Web {
            api_host,
            mesh_host,
            port,
            bootstrap,
        }) => {
            // Parse local bootstraps for this command
            let mut local_bootstraps = Vec::new();
            for b in bootstrap {
                if let Ok(ma) = b.parse::<libp2p::Multiaddr>() {
                    local_bootstraps.push(ma);
                }
            }

            // If we have local bootstraps, we might want to update the core's mesh manager
            // but for now, Core::new already took the global ones if they were provided.
            if !local_bootstraps.is_empty() {
                core_arc.add_bootstrap_nodes(local_bootstraps).await;
            }

            // Let's just ensure we start the nervous system.
            core_arc.clone().start_nervous_system(mesh_host).await;

            info!("Starting web server mode on {}:{}", api_host, port);
            ghostwire::web::start_web_server(
                Some(core_arc),
                api_host,
                port,
                bootstrap_multiaddrs.clone(),
                args.sim_addr.clone(),
                args.mesh_host.clone(),
            )
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        }
        #[cfg(feature = "tui")]
        Some(Commands::Tui) => {
            // Start nervous system for mesh networking
            core_arc
                .clone()
                .start_nervous_system(args.mesh_host.clone())
                .await;

            info!("Starting TUI mode");
            let core_mutex = Arc::new(Mutex::new(core_arc.as_ref().clone()));
            tui::start_tui(core_mutex)
                .await
                .map_err(|e| anyhow::anyhow!("TUI error: {}", e))?;
        }
        None => {
            // Default behavior
            // Activate Mesh on configured host
            core_arc
                .clone()
                .start_nervous_system(args.mesh_host.clone())
                .await;

            #[cfg(feature = "cli")]
            if args.cli {
                println!("CLI mode is currently disabled.");
            } else if args.web {
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
                )
                .await
                .map_err(|e| anyhow::anyhow!(e))?;
            } else {
                // Default to web server
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
                )
                .await
                .map_err(|e| anyhow::anyhow!(e))?;

                #[cfg(not(feature = "web"))]
                {
                    info!("Running in minimal mode - core only");
                    tokio::signal::ctrl_c().await?;
                }
            }
        }
    }

    Ok(())
}
