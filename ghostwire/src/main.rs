use anyhow::Result;
#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::info;

// Use modules from the library crate
use ghostwire::core::Core;
#[cfg(feature = "tui")]
use ghostwire::tui;
#[cfg(feature = "web")]
use ghostwire::web;
// use ghostwire::config; // If needed

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

    #[arg(long, default_value_t = 8080)]
    port: u16,

    #[arg(long)]
    bootstrap: Vec<String>,

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
    api_host: String,
    mesh_host: String,
    port: u16,
    bootstrap: Vec<String>,
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
    let core = Core::new(bootstrap_multiaddrs).await?;
    let core_arc = Arc::new(core);

    // Activate the autonomic nervous system (DNA 10/10)
    // core_arc.clone().start_nervous_system().await; // Moved to inside match arms to use configured mesh_host

    match args.command {
        #[cfg(feature = "cli")]
        Some(Commands::Cli) => {
            info!("Starting CLI mode");
            println!("CLI mode is currently disabled.");
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
            start_web_server(core_arc, api_host, port)
                .await
                .map_err(|e| anyhow::anyhow!(e))?;
        }
        #[cfg(feature = "tui")]
        Some(Commands::Tui) => {
            // TUI defaults to 0.0.0.0 for mesh since it's local interactive
            core_arc
                .clone()
                .start_nervous_system("0.0.0.0".to_string())
                .await;

            info!("Starting TUI mode");
            // For now, TUI gets no bootstraps unless we pass 'em through
            let core_mutex = Arc::new(Mutex::new(Core::new(Vec::new()).await?));
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
                start_web_server(core_arc, args.api_host, args.port)
                    .await
                    .map_err(|e| anyhow::anyhow!(e))?;
            } else {
                // Default to web server
                info!(
                    "Starting web server mode on {}:{}",
                    args.api_host, args.port
                );
                start_web_server(core_arc, args.api_host, args.port)
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
