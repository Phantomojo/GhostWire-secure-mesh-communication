pub mod commands;

/// Read the auth token the daemon writes to `<GHOSTWIRE_HOME>/token`.
///
/// This is the same file and header the TUI already uses, so a CLI invocation
/// authenticates exactly like the terminal UI does.
fn read_daemon_token() -> Option<String> {
    // Must match the daemon's own resolution exactly (see the token reader in
    // web.rs): GHOSTWIRE_HOME when set, otherwise config_dir()/ghostwire.
    // Getting this wrong reads a STALE token from the default location while
    // the daemon is using a different one, which surfaces as a confusing 401
    // rather than as "wrong token file".
    let config_dir = match std::env::var("GHOSTWIRE_HOME") {
        Ok(home) => std::path::PathBuf::from(home),
        Err(_) => dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("ghostwire"),
    };
    let path = config_dir.join("token");
    std::fs::read_to_string(path)
        .ok()
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
}

/// A CLI command that talks to the daemon failed. `Unreachable` means nothing
/// is listening (so the "start the daemon" hint is useful); `Rejected` means
/// the daemon answered with an error and the hint would be misleading.
#[derive(Debug)]
pub enum DaemonError {
    Unreachable(String),
    Rejected(String),
}

impl std::error::Error for DaemonError {}

impl std::fmt::Display for DaemonError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DaemonError::Unreachable(m) | DaemonError::Rejected(m) => write!(f, "{}", m),
        }
    }
}

/// Pull the human-readable message out of an ApiV2 error envelope, falling
/// back to the raw body when it is not the expected shape. Dumping the whole
/// envelope at a user is unreadable.
fn extract_error_message(body: &str) -> String {
    serde_json::from_str::<serde_json::Value>(body)
        .ok()
        .and_then(|v| {
            v.get("error")
                .and_then(|e| e.get("message"))
                .and_then(|m| m.as_str())
                .map(|s| s.to_string())
        })
        .unwrap_or_else(|| body.trim().to_string())
}

/// Base URL for talking to a running daemon over its HTTP API.
///
/// `--api-host` is a real, accepted flag, but the URL builder below used to
/// hardcode `127.0.0.1` regardless of it. That made the flag silently
/// ineffective: a daemon bound to a Tailscale address (the normal setup for a
/// two-node mesh test, where the daemon must be reachable off-box) answered
/// requests on its LAN/overlay address while the CLI dialled loopback and
/// reported "Is the daemon running? Try: ghostwire start" — a message that
/// points at the wrong problem entirely, because the daemon was running and
/// healthy.
///
/// Honour the host the caller actually passed. `GHOSTWIRE_API_HOST` lets
/// scripts and the TUI set it without repeating the flag.
fn daemon_base_url(port: u16) -> String {
    let host = std::env::var("GHOSTWIRE_API_HOST")
        .ok()
        .filter(|h| !h.trim().is_empty())
        .or_else(|| {
            let args: Vec<String> = std::env::args().collect();
            let mut it = args.iter();
            while let Some(a) = it.next() {
                if a == "--api-host" {
                    return it.next().cloned();
                }
                if let Some(v) = a.strip_prefix("--api-host=") {
                    return Some(v.to_string());
                }
            }
            None
        })
        .unwrap_or_else(|| "127.0.0.1".to_string());

    // An IPv6 literal must be bracketed in a URL authority.
    if host.contains(':') && !host.starts_with('[') {
        format!("http://[{}]:{}", host, port)
    } else {
        format!("http://{}:{}", host, port)
    }
}

/// List peers by querying the RUNNING daemon over its HTTP API.
///
/// Previously `ghostwire peers` built a brand-new in-process `Core` — its own
/// identity, its own mesh, its own encryption state — purely to print a peer
/// list, while `src/cli/` contained no HTTP calls at all. With a daemon
/// already running, the CLI's freshly-built node knew nothing about the
/// daemon's peers, so the command reported a misleading empty list and paid
/// full startup cost every time.
///
/// `status` and `tui` already took the correct approach by returning before
/// the `Core` is constructed; this does the same.
pub async fn list_peers_from_daemon(port: u16) -> Result<()> {
    let url = format!("{}/api/v2/peers", daemon_base_url(port));
    let client = reqwest::Client::new();
    let mut req = client.get(&url);
    if let Some(token) = read_daemon_token() {
        req = req.header("X-Ghost-Token", token);
    }

    let resp = req.send().await.map_err(|e| {
        DaemonError::Unreachable(format!("Could not reach the daemon at {}: {}", url, e))
    })?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(DaemonError::Rejected(format!(
            "Daemon returned {} for /api/v2/peers: {}",
            status,
            extract_error_message(&body)
        ))
        .into());
    }

    let value: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("Malformed peer list from daemon: {}", e))?;

    let peers = value
        .get("data")
        .and_then(|d| d.get("peers"))
        .or_else(|| value.get("peers"))
        .and_then(|p| p.as_array())
        .cloned()
        .unwrap_or_default();

    if peers.is_empty() {
        println!("📡 No peers known to the daemon yet.");
        return Ok(());
    }

    println!("📡 Mesh Network Peers ({}):", peers.len());
    for peer in &peers {
        let id = peer
            .get("peer_id")
            .and_then(|v| v.as_str())
            .or_else(|| peer.get("id").and_then(|v| v.as_str()))
            .unwrap_or("<unknown>");
        let name = peer
            .get("name")
            .or_else(|| peer.get("nickname"))
            .and_then(|v| v.as_str())
            .unwrap_or("");
        let status = peer
            .get("status")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        if name.is_empty() {
            println!("  • {} [{}]", id, status);
        } else {
            println!("  • {} ({}) [{}]", name, id, status);
        }
    }
    Ok(())
}

/// Send a message through the running daemon rather than a private node.
pub async fn send_message_via_daemon(port: u16, recipient: &str, content: &str) -> Result<()> {
    let url = format!("{}/api/v2/messages/send", daemon_base_url(port));
    let client = reqwest::Client::new();
    let mut req = client.post(&url).json(&serde_json::json!({
        "recipient": recipient,
        "content": content,
    }));
    if let Some(token) = read_daemon_token() {
        req = req.header("X-Ghost-Token", token);
    }

    let resp = req.send().await.map_err(|e| {
        DaemonError::Unreachable(format!("Could not reach the daemon at {}: {}", url, e))
    })?;

    if !resp.status().is_success() {
        let status = resp.status();
        let body = resp.text().await.unwrap_or_default();
        return Err(DaemonError::Rejected(format!(
            "Daemon returned {}: {}",
            status,
            extract_error_message(&body)
        ))
        .into());
    }

    // A 2xx is NOT sufficient proof of delivery here. The API signals failure
    // in the body while still returning HTTP 200:
    //   {"error":{"code":"INTERNAL_ERROR",
    //             "message":"Transmission failed: Invalid P2P Recipient ID: ..."}}
    // Checking only the status code reported a successful send to a peer that
    // does not exist. Surface the real outcome.
    let body: serde_json::Value = resp
        .json()
        .await
        .map_err(|e| anyhow::anyhow!("Malformed response from daemon: {}", e))?;
    if let Some(err) = body.get("error") {
        let message = err
            .get("message")
            .and_then(|m| m.as_str())
            .unwrap_or("unknown error");
        anyhow::bail!("{}", message);
    }
    if body.get("delivered").and_then(|d| d.as_bool()) == Some(false) {
        anyhow::bail!("Daemon accepted the request but did not deliver the message");
    }

    println!("✅ Message sent to {}", recipient);
    Ok(())
}

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
        #[arg(long)]
        start: bool,
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
        /// Transport mode: hybrid, libp2p-only, reticulum-only
        #[arg(long, default_value = "hybrid")]
        mode: String,
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
        /// Use anonymous introduction (SURB-based)
        #[arg(long)]
        anonymous: bool,
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
async fn dispatch_cli_command(
    command: Commands,
    core_arc: &Arc<Core>,
    _config: &crate::config::Config,
) -> Result<()> {
    match command {
        Commands::Init { username, start } => {
            init_identity(core_arc, username, start).await?;
        }
        Commands::Start {
            host,
            port,
            mesh_port,
            web,
            mode,
        } => {
            if !mode.is_empty() {
                info!("Transport mode: {}", mode);
            }
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
        Commands::Connect {
            username,
            anonymous,
        } => {
            if anonymous {
                connect_anonymous(core_arc, &username).await?;
            } else {
                connect_by_username(core_arc, &username).await?;
            }
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

async fn init_identity(core: &Arc<Core>, username: Option<String>, auto_start: bool) -> Result<()> {
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

    if auto_start {
        println!("\n🚀 Starting GhostWire node...");
        start_node(core, "127.0.0.1".to_string(), 8080, 0).await?;
    } else {
        println!("\n💡 Run `ghostwire start` to begin mesh networking.");
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

    let mut final_recipient = recipient.to_string();

    // Resolve @username if provided
    if let Some(username) = recipient.strip_prefix('@') {
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
    match core
        .send_direct_message(final_recipient.clone(), &msg)
        .await
    {
        Ok(_) => {
            println!(
                "✅ Message sent successfully to {} (id={})",
                recipient, msg.id
            );
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

async fn connect_anonymous(core: &Arc<Core>, username: &str) -> Result<()> {
    use crate::core::identity::anonymous_intro::generate_intro_invite;

    let name = if username.starts_with('@') {
        &username[1..]
    } else {
        username
    };

    println!("Generating anonymous introduction for {}...", name);

    // Collect mesh peers as Sphinx relay nodes (minimum SPHINX_MIN_HOPS = 2)
    let (route_keys, route_ids) = match core.get_mesh_topology().await {
        Some(topology) => {
            let peers: Vec<_> = topology
                .nodes
                .values()
                .filter(|n| n.is_online && n.id != topology.local_node_id)
                .take(3)
                .collect();
            if peers.len() < 2 {
                println!(
                    "Need at least 2 online peers as relay nodes, found {}.",
                    peers.len()
                );
                return Ok(());
            }
            let keys: Vec<[u8; 32]> = peers
                .iter()
                .filter_map(|p| {
                    if p.public_key.len() >= 32 {
                        p.public_key[..32].try_into().ok()
                    } else {
                        None
                    }
                })
                .collect();
            let ids: Vec<String> = peers.iter().map(|p| p.id.clone()).collect();
            (keys, ids)
        }
        None => {
            println!("Mesh network not initialized. Start the node first.");
            return Ok(());
        }
    };

    let (ed25519_pub, nickname) = {
        let id_lock = core.identity.read().await;
        let key_bytes = id_lock.get_public_key();
        let ed: [u8; 32] = key_bytes[..32]
            .try_into()
            .map_err(|_| anyhow::anyhow!("Ed25519 key not 32 bytes"))?;
        (ed, id_lock.get_metadata().nickname.clone())
    };

    let x25519_pub: [u8; 32] = core
        .encryption
        .x25519_public_key()
        .ok_or_else(|| anyhow::anyhow!("X25519 key not available"))?;

    match generate_intro_invite(
        &route_keys,
        &route_ids,
        ed25519_pub,
        x25519_pub,
        Some(nickname),
    ) {
        Ok(invite) => {
            let invite_str = invite
                .to_invite_string()
                .map_err(|e| anyhow::anyhow!("Invite encode: {}", e))?;
            println!("Anonymous introduction generated!");
            println!("Invite string: {}", invite_str);
            println!("Send this invite to {} through an anonymous channel.", name);
            println!("   They can reply without knowing your identity.");
        }
        Err(e) => {
            println!("Failed to generate introduction: {}", e);
        }
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

    let decrypted = core
        .encryption
        .decrypt(&decoded)
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

/// Prepare the GhostWire environment.
///
/// This used to be a second, entirely separate `setup` command that made three
/// directories, checked for `git`/`curl`/`npm`, printed
/// "Run 'ghostwire manage start' to begin." — and that suggested command does
/// not exist at the top level, so the instruction could not be followed. It
/// also hardcoded `~/.ghostwire` while the daemon, identity, vault, and
/// mailbox all honour `GHOSTWIRE_HOME`, so with that variable set it reported
/// success on directories no other component used.
///
/// `ghostwire setup` (main.rs) is the real identity setup and owns the
/// first-run flow. This is now only the directory preparation that command also
/// needs, and it points at the one true setup rather than pretending to be a
/// second path of its own.
async fn handle_setup() -> Result<()> {
    info!("🛠️ Preparing the GhostWire environment...");

    // Honour GHOSTWIRE_HOME, matching daemon.rs, core_identity.rs,
    // persistence_manager.rs and core/mod.rs (mailbox). Using ~/.ghostwire here
    // while everything else uses the env var means setup reports success on
    // directories the node never reads.
    let gw_dir = match std::env::var("GHOSTWIRE_HOME") {
        Ok(h) if !h.is_empty() => std::path::PathBuf::from(h),
        _ => dirs::home_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find home directory"))?
            .join(".ghostwire"),
    };

    let mut created = Vec::new();
    for dir in [gw_dir.clone(), gw_dir.join("vault"), gw_dir.join("logs")] {
        if !dir.exists() {
            std::fs::create_dir_all(&dir)?;
            created.push(dir.display().to_string());
        }
    }

    if created.is_empty() {
        println!("✅ Environment already prepared at {}", gw_dir.display());
    } else {
        println!(
            "✅ Created {} director{}(s):",
            created.len(),
            if created.len() == 1 { "y" } else { "ies" }
        );
        for d in &created {
            println!("   {}", d);
        }
        println!("   root: {}", gw_dir.display());
    }

    // Report what is actually needed. `adb` matters (it backs `deploy`);
    // git/curl/npm do not, and a successful spawn was previously reported as
    // "installed" without ever checking the exit status.
    println!("\nOptional tooling:");
    for (dep, why) in [("adb", "required by 'ghostwire deploy'")] {
        let present = std::process::Command::new(dep)
            .arg("version")
            .output()
            .map(|o| o.status.success())
            .unwrap_or(false);
        if present {
            println!("  ✅ {dep} — {why}");
        } else {
            println!("  ⚠️  {dep} not found — {why}");
        }
    }

    // Direct to the single real setup command instead of the nonexistent one.
    println!("\nNext: run 'ghostwire setup' to create your identity.");
    Ok(())
}

/// Run the interactive TUI mode — starts node + TUI in one command
async fn run_tui_mode(core: &Arc<Core>) -> Result<()> {
    info!("🖥️  Starting GhostWire TUI (node running in background)...");
    let port = core.config.server.port;
    crate::tui::run_tui(port)
        .await
        .map_err(|e| anyhow::anyhow!("TUI error: {}", e))
}
