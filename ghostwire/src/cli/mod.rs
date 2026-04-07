pub mod commands;

use crate::core::Core;
use anyhow::Result;
use clap::{Parser, Subcommand};
use std::ffi::OsString;
use std::sync::Arc;
use tracing::{error, info};

#[derive(Parser)]
#[command(name = "ghostwire")]
#[command(about = "GhostWire - Secure Mesh Networking and Messaging")]
#[command(version = "0.1.0")]
pub struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new GhostWire identity
    Init {
        #[arg(short = 'u', long)]
        username: Option<String>,
    },
    /// Start the GhostWire node
    Start {
        #[arg(long, default_value = "127.0.0.1")]
        host: String,
        #[arg(short = 'P', long, default_value_t = 8080)]
        port: u16,
        #[arg(long, default_value_t = 0)]
        mesh_port: u16,
        #[arg(long)]
        web: bool,
    },
    /// Send a message to a peer
    Send {
        #[arg(short = 'r', long)]
        recipient: String,
        #[arg(short = 'm', long)]
        message: String,
    },
    /// List connected peers
    Peers,
    /// Show network statistics
    Stats,
    /// Show security information
    Security,
    /// Generate a new key pair
    GenKey,
    /// Encrypt a message
    Encrypt {
        #[arg(short = 'e', long)]
        message: String,
    },
    /// Decrypt a message
    Decrypt {
        #[arg(short = 'd', long)]
        encrypted: String,
    },
    /// Connect to a peer by username
    Connect {
        #[arg(short = 'u', long)]
        username: String,
    },
    /// Manage GhostWire (start/stop/restart/status/logs/backup/restore)
    Manage {
        #[command(subcommand)]
        subcommand: ManageCommand,
    },
    /// Deploy GhostWire payloads
    Deploy {
        #[arg(short = 't', long)]
        target: String,
        #[arg(short = 'p', long)]
        platform: String,
    },
    /// Monitor network and security
    Monitor {
        #[arg(short = 'f', long)]
        follow: bool,
    },
    /// Interactive terminal interface
    Tui,
    /// Setup dependencies and environment
    Setup,
}

#[derive(Subcommand)]
pub enum ManageCommand {
    Start,
    Stop,
    Restart,
    Status,
    Logs {
        #[arg(short = 'f', long)]
        follow: bool,
        #[arg(short = 'n', long, default_value_t = 50)]
        lines: usize,
    },
    Backup,
    Restore {
        #[arg(short = 'i', long)]
        input: String,
    },
}

/// Run the CLI application (legacy — parses from env)
pub async fn run_cli() -> Result<()> {
    run_cli_with_args(std::env::args_os().skip(1).collect()).await
}

/// Run the CLI with explicit arguments (for integration with main.rs)
pub async fn run_cli_with_args(args: Vec<OsString>) -> Result<()> {
    let cli = Cli::try_parse_from(args).map_err(|e| anyhow::anyhow!("{}", e))?;

    let config = crate::config::Config::load().unwrap_or_else(|_| crate::config::Config::default());

    // Initialize core components with no bootstraps for CLI mode
    let core = Core::new(config.clone(), Vec::new(), None, None).await?;
    let core_arc = Arc::new(core);

    dispatch_cli_command(cli.command, &core_arc, &config).await
}

/// Run CLI with a pre-existing Core (avoids double-open of sled DB)
pub async fn run_cli_with_args_and_core(args: Vec<OsString>, core_arc: Arc<Core>) -> Result<()> {
    let cli = Cli::try_parse_from(args).map_err(|e| anyhow::anyhow!("{}", e))?;
    let config = crate::config::Config::load().unwrap_or_else(|_| crate::config::Config::default());
    dispatch_cli_command(cli.command, &core_arc, &config).await
}

/// Dispatch a CLI command to the appropriate handler
async fn dispatch_cli_command(command: Commands, core_arc: &Arc<Core>, _config: &crate::config::Config) -> Result<()> {
    match command {
        Commands::Init { username } => {
            init_identity(core_arc, username).await?;
        }
        Commands::Start { host, port, mesh_port, web } => {
            if web {
                start_web_server(core_arc, host, port, mesh_port).await?;
            } else {
                start_node(core_arc, host, port, mesh_port).await?;
            }
        }
        Commands::Send { recipient, message } => {
            send_message(core_arc, &recipient, &message).await?;
        }
        Commands::Peers => {
            list_peers(core_arc).await?;
        }
        Commands::Stats => {
            show_stats(core_arc).await?;
        }
        Commands::Security => {
            show_security_info(core_arc).await?;
        }
        Commands::GenKey => {
            generate_key(core_arc).await?;
        }
        Commands::Encrypt { message } => {
            encrypt_message(core_arc, &message).await?;
        }
        Commands::Decrypt { encrypted } => {
            decrypt_message(core_arc, &encrypted).await?;
        }
        Commands::Connect { username } => {
            connect_by_username(core_arc, &username).await?;
        }
        Commands::Manage { subcommand } => {
            handle_manage(core_arc, subcommand).await?;
        }
        Commands::Deploy { target, platform } => {
            handle_deploy(core_arc, target, platform).await?;
        }
        Commands::Monitor { follow } => {
            handle_monitor(core_arc, follow).await?;
        }
        Commands::Tui => {
            run_tui_mode(core_arc).await?;
        }
        Commands::Setup => {
            handle_setup().await?;
        }
    }

    Ok(())
}

async fn init_identity(core: &Arc<Core>, username: Option<String>) -> Result<()> {
    info!("Initializing GhostWire identity...");

    let identity_id = core.get_identity_id().await;
    let public_key = core.get_public_key();

    println!("✅ Identity initialized successfully!");
    println!("Identity ID: {}", identity_id);
    println!(
        "Public Key: {}",
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &public_key)
    );

    if let Some(name) = username {
        println!("Username: {}", name);
    }

    Ok(())
}

async fn start_web_server(core: &Arc<Core>, host: String, port: u16, mesh_port: u16) -> Result<()> {
    info!("Starting GhostWire web server on {}:{}", host, port);

    // Import the web module function
    use crate::web::start_web_server;

    start_web_server(
        Some(core.clone()),
        host,
        port,
        Vec::new(),
        None,
        String::new(),
        mesh_port,
        core.config.clone(),
    )
    .await
    .map_err(|e| anyhow::anyhow!("Web server error: {}", e))?;

    Ok(())
}

async fn start_node(core: &Arc<Core>, host: String, port: u16, mesh_port: u16) -> Result<()> {
    info!("Starting GhostWire node on {}:{}", host, port);

    // Initialize mesh networking

    let core_clone = core.clone();
    let host_clone = host.clone();
    tokio::spawn(async move {
        core_clone.start_nervous_system(host_clone, mesh_port).await;
    });

    // Initialize reticulum networking

    tokio::spawn(async move {
        // TODO: Implement proper reticulum initialization
        println!("[CLI] Reticulum initialization requested");
    });

    println!("✅ GhostWire node started successfully!");
    println!("Listening on {}:{}", host, port);
    println!("Press Ctrl+C to stop");

    // Keep the node running
    tokio::signal::ctrl_c().await?;
    println!("\nShutting down...");

    Ok(())
}

pub async fn send_message(core: &Arc<Core>, recipient: &str, message: &str) -> Result<()> {
    use crate::core::Message;
    use std::time::{SystemTime, UNIX_EPOCH};

    let mut final_recipient = recipient.to_string();

    // Resolve @username if provided
    if recipient.starts_with('@') {
        let username = &recipient[1..];
        println!("🔍 Discovering peer for username: {}...", username);

        #[cfg(feature = "mesh")]
        if let Some(mesh_manager) = &core.mesh_manager {
            let mut mgr = mesh_manager.write().await;
            if let Ok(Some(record)) = mgr.lookup_username(username).await {
                final_recipient = record.peer_id.to_string();
                println!("✅ Resolved {} to {}", recipient, final_recipient);
            } else {
                println!(
                    "⚠️ Could not resolve username {}. Attempting broadcast/manual discovery.",
                    recipient
                );
            }
        }
    }

    // Use Message::new() which sets proper defaults including sequence_num and requires_ack
    let msg = Message::new(
        core.get_identity_id().await,
        final_recipient.clone(),
        message.to_string(),
    );

    // Use send_direct_message for proper routing (mailbox fallback, peer health)
    match core.send_direct_message(final_recipient.clone(), &msg).await {
        Ok(_) => {
            println!("✅ Message sent successfully to {} (id={})", recipient, msg.id);
        }
        Err(e) => {
            error!("Failed to send message: {}", e);
            return Err(anyhow::anyhow!("Failed to send message: {}", e));
        }
    }

    Ok(())
}

async fn connect_by_username(core: &Arc<Core>, username: &str) -> Result<()> {
    let name = if username.starts_with('@') {
        &username[1..]
    } else {
        username
    };
    println!("🔍 Searching for peer: {}...", name);

    #[cfg(feature = "mesh")]
    if let Some(mesh_manager) = &core.mesh_manager {
        let mut mgr = mesh_manager.write().await;
        match mgr.lookup_username(name).await {
            Ok(Some(record)) => {
                println!("✅ Found peer {}", record.peer_id);
                // Dial the peer
                if let Ok(peer_id) = record.peer_id.parse::<libp2p::PeerId>() {
                    if let Err(e) = mgr.connect_peer(peer_id).await {
                        println!("❌ Failed to dial peer: {}", e);
                    } else {
                        println!("🚀 Dialing initiated via DHT discovery.");
                    }
                } else {
                    println!("❌ Invalid PeerId: {}", record.peer_id);
                }
            }
            Ok(None) => {
                println!("⌛ Discovery started. Please wait a moment and try again if connection doesn't happen automatically.");
            }
            Err(e) => {
                println!("❌ Error during discovery: {}", e);
            }
        }
    } else {
        println!("❌ Mesh network not enabled.");
    }

    Ok(())
}

pub async fn list_peers(core: &Arc<Core>) -> Result<()> {
    if let Some(mesh_stats) = core.get_mesh_stats().await {
        println!("📡 Mesh Network Peers:");
        println!("Total nodes: {}", mesh_stats.total_nodes);
        println!("Online nodes: {}", mesh_stats.online_nodes);
        println!("Routes count: {}", mesh_stats.routes_count);
    } else {
        println!("❌ Mesh network not initialized");
    }

    if let Some(reticulum_stats) = core.get_reticulum_stats().await {
        println!("\n🕸️  Reticulum Network:");
        println!("Total nodes: {}", reticulum_stats.total_nodes);
        println!("Online nodes: {}", reticulum_stats.online_nodes);
        println!("Routes count: {}", reticulum_stats.routes_count);
    } else {
        println!("❌ Reticulum network not initialized");
    }

    Ok(())
}

pub async fn show_stats(core: &Arc<Core>) -> Result<()> {
    println!("📊 GhostWire Network Statistics");
    println!("================================");

    if let Some(mesh_stats) = core.get_mesh_stats().await {
        println!("Mesh Network:");
        println!("  Total nodes: {}", mesh_stats.total_nodes);
        println!("  Online nodes: {}", mesh_stats.online_nodes);
        println!("  Routes count: {}", mesh_stats.routes_count);
    }

    if let Some(reticulum_stats) = core.get_reticulum_stats().await {
        println!("\nReticulum Network:");
        println!("  Total nodes: {}", reticulum_stats.total_nodes);
        println!("  Online nodes: {}", reticulum_stats.online_nodes);
        println!("  Routes count: {}", reticulum_stats.routes_count);
    }

    // Display security statistics
    let security_stats = core.get_security_stats().await;
    println!("\n🔒 Security Statistics:");
    println!("  Threat level: {:?}", security_stats.threat_level);
    println!("  Total events: {}", security_stats.total_events);
    println!("  Active threats: {}", security_stats.active_threats);
    println!(
        "  Blocked connections: {}",
        security_stats.blocked_connections
    );
    println!("  Last scan: {}", security_stats.last_scan);

    Ok(())
}

async fn show_security_info(core: &Arc<Core>) -> Result<()> {
    println!("🔒 GhostWire Security Information");
    println!("==================================");

    let identity_id = core.get_identity_id().await;
    let public_key = core.get_public_key();
    let key_id = core.get_key_id();

    println!("Identity ID: {}", identity_id);
    println!("Key ID: {}", key_id);
    println!(
        "Public Key: {}",
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &public_key)
    );

    // Display security statistics
    let security_stats = core.get_security_stats().await;
    println!("\n🔒 Security Statistics:");
    println!("  Threat Level: {:?}", security_stats.threat_level);
    println!("  Total Events: {}", security_stats.total_events);
    println!("  Active Threats: {}", security_stats.active_threats);
    println!(
        "  Blocked Connections: {}",
        security_stats.blocked_connections
    );
    println!("  Last Scan: {}", security_stats.last_scan);

    Ok(())
}

async fn generate_key(core: &Arc<Core>) -> Result<()> {
    info!("Generating new encryption key pair...");

    // The core already has encryption initialized
    let public_key = core.get_public_key();
    let key_id = core.get_key_id();

    println!("✅ New key pair generated!");
    println!("Key ID: {}", key_id);
    println!(
        "Public Key: {}",
        base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &public_key)
    );

    Ok(())
}

async fn encrypt_message(core: &Arc<Core>, message: &str) -> Result<()> {
    let encrypted = core.encryption.encrypt(message.as_bytes());
    let encoded = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &encrypted);

    println!("🔐 Encrypted message:");
    println!("{}", encoded);

    Ok(())
}

async fn decrypt_message(core: &Arc<Core>, encrypted: &str) -> Result<()> {
    let decoded = base64::Engine::decode(&base64::engine::general_purpose::STANDARD, encrypted)
        .map_err(|e| anyhow::anyhow!("Invalid base64: {}", e))?;

    let decrypted = core.encryption.decrypt(&decoded)
        .map_err(|e| anyhow::anyhow!("Decryption failed: {:?}", e))?;
    let message =
        String::from_utf8(decrypted).map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))?;

    println!("🔓 Decrypted message:");
    println!("{}", message);

    Ok(())
}
async fn handle_manage(core: &Arc<Core>, subcommand: ManageCommand) -> Result<()> {
    match subcommand {
        ManageCommand::Start => {
            info!("🚀 Starting GhostWire node...");
            start_node(core, "0.0.0.0".to_string(), 8080, 0).await?;
        }
        ManageCommand::Stop => {
            info!("🛑 Stopping GhostWire processes...");
            #[cfg(unix)]
            {
                let output = std::process::Command::new("pkill")
                    .arg("-f")
                    .arg("ghostwire")
                    .output()?;
                if output.status.success() {
                    println!("✅ GhostWire processes terminated.");
                } else {
                    println!("⚠️ No running GhostWire processes found.");
                }
            }
            #[cfg(not(unix))]
            println!("Stop command not yet implemented for this platform.");
        }
        ManageCommand::Restart => {
            info!("🔄 Restarting GhostWire node...");
            // Simple stop then start
            let _ = std::process::Command::new("pkill")
                .arg("-f")
                .arg("ghostwire")
                .output();
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            start_node(core, "0.0.0.0".to_string(), 8080, 0).await?;
        }
        ManageCommand::Status => {
            show_stats(core).await?;
        }
        ManageCommand::Logs { follow, lines } => {
            info!("📋 Viewing logs (last {} lines)...", lines);
            let log_file = "/tmp/ghostwire.log"; // Default for now
            if std::path::Path::new(log_file).exists() {
                let tail_cmd = if follow { "tail -f" } else { "tail -n" };
                let mut child = std::process::Command::new("sh")
                    .arg("-c")
                    .arg(format!("{} {} {}", tail_cmd, lines, log_file))
                    .spawn()?;
                child.wait()?;
            } else {
                println!("⚠️ Log file not found at {}", log_file);
            }
        }
        ManageCommand::Backup => {
            info!("💾 Creating backup of GhostWire data...");
            let home = dirs::home_dir().unwrap();
            let backup_path = home.join(format!(
                "ghostwire_backup_{}.tar.gz",
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)?
                    .as_secs()
            ));
            let output = std::process::Command::new("tar")
                .arg("-czf")
                .arg(&backup_path)
                .arg("-C")
                .arg(home.join(".ghostwire"))
                .arg(".")
                .output()?;
            if output.status.success() {
                println!("✅ Backup created at: {:?}", backup_path);
            } else {
                println!(
                    "❌ Backup failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
        ManageCommand::Restore { input } => {
            info!("📂 Restoring from backup: {}...", input);
            let home = dirs::home_dir().unwrap();
            let output = std::process::Command::new("tar")
                .arg("-xzf")
                .arg(&input)
                .arg("-C")
                .arg(home.join(".ghostwire"))
                .output()?;
            if output.status.success() {
                println!("✅ Restore complete.");
            } else {
                println!(
                    "❌ Restore failed: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
            }
        }
    }
    Ok(())
}

async fn handle_deploy(_core: &Arc<Core>, target: String, platform: String) -> Result<()> {
    info!(
        "🚀 Deploying GhostWire payload to {} ({})",
        target, platform
    );

    if platform.to_lowercase() == "android" {
        println!("📲 Pushing payload to Android device via ADB...");
        let binaries = [
            "ghostwire-aarch64-linux-android",
            "ghostwire-armv7-linux-androideabi",
        ];
        let mut found = false;
        for bin in &binaries {
            let path = format!("src-tauri/binaries/{}", bin);
            if std::path::Path::new(&path).exists() {
                std::process::Command::new("adb")
                    .arg("push")
                    .arg(&path)
                    .arg("/data/local/tmp/ghostwire")
                    .status()?;
                std::process::Command::new("adb")
                    .arg("shell")
                    .arg("chmod +x /data/local/tmp/ghostwire")
                    .status()?;
                println!(
                    "✅ Successfully pushed {} to /data/local/tmp/ghostwire",
                    bin
                );
                found = true;
                break;
            }
        }
        if !found {
            println!("❌ No Android sidecar binaries found in src-tauri/binaries/");
        }
    } else {
        println!(
            "⚠️ Deployment to platform '{}' not yet implemented in CLI.",
            platform
        );
    }

    Ok(())
}

async fn handle_monitor(core: &Arc<Core>, follow: bool) -> Result<()> {
    info!("🛡️ Monitoring GhostWire security events...");

    // Attempt to access security stats first
    let stats = core.get_security_stats().await;
    println!("Current Security Status:");
    println!("  Threat Level: {:?}", stats.threat_level);
    println!("  Active Threats: {}", stats.active_threats);
    println!("  Total Security Events: {}", stats.total_events);

    if follow {
        println!("\n📡 Streaming real-time events (Press Ctrl+C to stop)...");
        let mut rx = core.event_bus.subscribe();
        while let Ok(event) = rx.recv().await {
            match event {
                crate::core::CoreEvent::SystemAlert(msg) => {
                    println!("🚨 [ALERT] {}", msg);
                }
                crate::core::CoreEvent::SecurityLockdown(active) => {
                    println!(
                        "🔒 [SECURITY] Lockdown mode: {}",
                        if active { "ENABLED" } else { "DISABLED" }
                    );
                }
                _ => {} // Ignore other events for monitor mode
            }
        }
    }

    Ok(())
}

async fn handle_setup() -> Result<()> {
    info!("🛠️ Setting up GhostWire environment...");

    let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?;
    let gw_dir = home.join(".ghostwire");
    let vault_dir = gw_dir.join("vault");
    let logs_dir = gw_dir.join("logs");

    for dir in &[&gw_dir, &vault_dir, &logs_dir] {
        if !dir.exists() {
            println!("📁 Creating directory: {:?}", dir);
            std::fs::create_dir_all(dir)?;
        }
    }

    println!("✅ GhostWire directories initialized.");

    // Check for common dependencies
    println!("🔍 Checking dependencies...");
    let deps = ["git", "curl", "npm"];
    for dep in &deps {
        match std::process::Command::new(dep).arg("--version").output() {
            Ok(_) => println!("  ✅ {} is installed", dep),
            Err(_) => println!("  ⚠️  {} is missing", dep),
        }
    }

    println!("\n✨ Setup complete. Run 'ghostwire manage start' to begin.");
    Ok(())
}

/// Run the interactive TUI mode — starts node + TUI in one command
async fn run_tui_mode(core: &Arc<Core>) -> Result<()> {
    info!("🖥️  Starting GhostWire TUI (node running in background)...");
    crate::tui::run_tui(core.clone())
        .await
        .map_err(|e| anyhow::anyhow!("TUI error: {}", e))
}
