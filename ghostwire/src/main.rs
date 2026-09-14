// Copyright (C) 2025-2026 Michael Irungu
// SPDX-License-Identifier: AGPL-3.0-or-later

use anyhow::Result;
#[cfg(feature = "cli")]
use clap::{Parser, Subcommand, ValueEnum};
use std::ffi::OsString;
use std::process::Command;
use std::sync::Arc;
use std::time::Duration;

use tracing::{error, info};

/// Panic hook: log the panic with backtrace before aborting.
/// In release builds, panic=abort means no unwinding — but this hook
/// ensures the panic message and backtrace are captured in the log.
fn install_panic_hook() {
    let default_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |panic_info| {
        let payload = panic_info.payload();
        let msg = if let Some(s) = payload.downcast_ref::<&str>() {
            s.to_string()
        } else if let Some(s) = payload.downcast_ref::<String>() {
            s.clone()
        } else {
            "Box<dyn Any>".to_string()
        };
        let location = panic_info
            .location()
            .map(|l| format!("{}:{}:{}", l.file(), l.line(), l.column()))
            .unwrap_or_else(|| "unknown".to_string());
        error!("PANIC at {}: {}", location, msg);
        eprintln!("GhostWire PANIC at {}: {}", location, msg);
        default_hook(panic_info);
    }));
}

// Use modules from the library crate
use ghostwire::config::Config;
use ghostwire::controllers::ProfilesController;
use ghostwire::core::updater_fixed;
use ghostwire::core::Core;
use ghostwire::daemon;

/// Fetch seed-hint peers from the Cloudflare bootstrap worker.
///
/// Contract (matches `cloudflare/bootstrap-worker`):
/// `GET {base}/api/peers` -> `{peers: [{peer_id, multiaddrs[]}]}`.
/// Records are UNTRUSTED discovery hints: every entry is validated here
/// (peer ID parses, addr parses, /p2p/ suffix enforced) and anything the
/// worker returns still passes the GhostWire handshake + local admission
/// before application use. Fail-open: any error yields an empty list and
/// the node continues in standalone/flag-configured mode.
async fn fetch_worker_bootstrap_peers(base: &str) -> anyhow::Result<Vec<libp2p::Multiaddr>> {
    #[derive(serde::Deserialize)]
    struct PeerEntry {
        peer_id: String,
        multiaddrs: Vec<String>,
    }
    #[derive(serde::Deserialize)]
    struct PeersResponse {
        peers: Vec<PeerEntry>,
    }

    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(10))
        .build()?;
    let res = client
        .get(format!("{}/api/peers", base))
        .send()
        .await?
        .error_for_status()?;
    let body: PeersResponse = res.json().await?;

    let mut out = Vec::new();
    for entry in body.peers {
        let peer_id: libp2p::PeerId = match entry.peer_id.parse() {
            Ok(id) => id,
            Err(_) => {
                tracing::warn!("worker seed with bad peer_id ignored");
                continue;
            }
        };
        for addr in entry.multiaddrs {
            match addr.parse::<libp2p::Multiaddr>() {
                Ok(ma) => {
                    let has_p2p = ma
                        .iter()
                        .any(|p| matches!(p, libp2p::multiaddr::Protocol::P2p(_)));
                    if has_p2p {
                        out.push(ma);
                    } else {
                        out.push(ma.with(libp2p::multiaddr::Protocol::P2p(peer_id)));
                    }
                }
                Err(_) => tracing::warn!("worker seed with bad multiaddr ignored"),
            }
        }
    }
    Ok(out)
}

/// Initialize logging with structured JSON output for production
fn init_logging(filter: &str) {
    #[cfg(not(debug_assertions))]
    {
        tracing_subscriber::fmt()
            .json()
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_env_filter(filter)
            .init();
    }

    #[cfg(debug_assertions)]
    {
        tracing_subscriber::fmt()
            .pretty()
            .with_target(true)
            .with_thread_ids(true)
            .with_thread_names(true)
            .with_env_filter(filter)
            .init();
    }
}

#[cfg(feature = "cli")]
#[derive(Parser)]
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

    #[arg(long, default_value_t = daemon::DEFAULT_DAEMON_PORT)]
    port: u16,
    /// QUIC transport port (0 = ephemeral; default 8081)
    #[arg(long, default_value_t = 8081)]
    quic_port: u16,

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

    /// Run as a background daemon (used internally by 'ghostwire start')
    #[arg(long)]
    daemon: bool,

    /// Tailscale bootstrap node (format: <tailscale-ip>/<peer-id>)
    #[arg(long)]
    tailscale_bootstrap: Vec<String>,

    /// Mesh network size: small (2-5 peers), medium (6-20), large (20+)
    #[arg(long, default_value = "small")]
    mesh_size: String,

    /// Verbose output (debug level logging)
    #[arg(long, short, global = true)]
    verbose: bool,

    /// Quiet output (warn level logging only)
    #[arg(long, short, global = true)]
    quiet: bool,
}

#[cfg(not(feature = "cli"))]
struct Args {
    command: Option<Commands>,
    api_host: String,
    mesh_host: String,
    mesh_port: u16,
    port: u16,
    quic_port: u16,
    bootstrap: Vec<String>,
    web: bool,
    cli: bool,
    mesh: bool,
    profile: Option<String>,
    daemon: bool,
    tailscale_bootstrap: Vec<String>,
    mesh_size: String,
}

#[cfg(feature = "cli")]
#[derive(Clone, Copy, Debug, Eq, PartialEq, ValueEnum)]
enum Interface {
    Web,
    Tui,
    None,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the GhostWire daemon in the background
    Start {
        /// Interface to launch after the daemon is ready.
        #[arg(long, value_enum, default_value_t = Interface::Web)]
        interface: Interface,
        /// Host for the local HTTP API and embedded web UI.
        #[arg(long, default_value = "127.0.0.1")]
        api_host: String,
        /// Mesh network port (0 = random)
        #[arg(long, default_value_t = 0)]
        mesh_port: u16,
        /// QUIC transport port (0 = ephemeral)
        #[arg(long, default_value_t = 8081)]
        quic_port: u16,
        /// Tailscale bootstrap node (format: <tailscale-ip>/<peer-id>)
        #[arg(long)]
        tailscale_bootstrap: Vec<String>,
        /// Bootstrap multiaddr
        #[arg(long)]
        bootstrap: Vec<String>,
        /// Mesh network size: small (2-5 peers), medium (6-20), large (20+)
        #[arg(long, default_value = "small")]
        mesh_size: String,
    },
    /// Stop the running GhostWire daemon
    Stop,
    /// Check if the GhostWire daemon is running
    Status,
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
        /// QUIC transport port (0 = ephemeral)
        #[arg(long, default_value_t = 8081)]
        quic_port: u16,
        #[arg(long)]
        bootstrap: Vec<String>,
    },
    /// List known peers
    Peers,
    /// Send a message: send <recipient_id> <content>
    Send { recipient: String, content: String },
    /// Interactive terminal user interface (connects to daemon)
    Tui,
    /// Interactive first-run setup (nickname, password, bio)
    Setup,
}

#[cfg(not(feature = "cli"))]
enum Commands {
    Cli,
    Web { host: String, port: u16 },
}

#[tokio::main]
async fn main() -> Result<()> {
    #[cfg(feature = "cli")]
    let args = Args::parse();
    #[cfg(not(feature = "cli"))]
    let args = Args {
        command: None,
        api_host: "127.0.0.1".to_string(),
        mesh_host: "0.0.0.0".to_string(),
        mesh_port: 0,
        port: daemon::DEFAULT_DAEMON_PORT,
        quic_port: 8081,
        bootstrap: Vec::new(),
        web: false,
        cli: false,
        mesh: false,
        profile: None,
        daemon: false,
        tailscale_bootstrap: Vec::new(),
        mesh_size: "small".to_string(),
        verbose: false,
        quiet: false,
    };

    let log_filter = if let Ok(rust_log) = std::env::var("RUST_LOG") {
        rust_log
    } else if args.quiet {
        "ghostwire=warn".to_string()
    } else if args.verbose {
        "ghostwire=debug,libp2p_gossipsub=debug".to_string()
    } else {
        "ghostwire=info".to_string()
    };
    init_logging(&log_filter);
    install_panic_hook();

    // Anti-debug: abort if being traced (must run before any key material loads)
    ghostwire::core::binary_hardening::detect_debugger();

    info!("🚀 Starting GhostWire - Secure Mesh Networking and Messaging");

    // Handle daemon management commands (don't need Core — they talk to the daemon)
    #[cfg(feature = "cli")]
    {
        match &args.command {
            Some(Commands::Start {
                interface,
                api_host,
                mesh_port,
                quic_port,
                tailscale_bootstrap,
                bootstrap,
                mesh_size,
            }) => {
                // `ghostwire --quic-port X start` and
                // `ghostwire start --quic-port X` must converge. The
                // subcommand value wins when explicitly non-default; otherwise
                // inherit the global option parsed before the subcommand.
                let effective_api_host = if api_host == "127.0.0.1" && args.api_host != "127.0.0.1"
                {
                    args.api_host.clone()
                } else {
                    api_host.clone()
                };
                let effective_mesh_port = if *mesh_port == 0 {
                    args.mesh_port
                } else {
                    *mesh_port
                };
                let effective_quic_port = if *quic_port == 8081 {
                    args.quic_port
                } else {
                    *quic_port
                };
                let extra = daemon::DaemonArgs {
                    api_host: effective_api_host.clone(),
                    tailscale_bootstrap: tailscale_bootstrap.clone(),
                    bootstrap: bootstrap.clone(),
                    mesh_port: effective_mesh_port,
                    quic_port: effective_quic_port,
                };
                match daemon::start_daemon(Some(args.port), Some(extra)) {
                    Ok(pid) => {
                        println!("✅ GhostWire daemon started (PID {})", pid);
                        println!("   Port: {}", args.port);
                        println!("   API:  http://127.0.0.1:{}", args.port);
                        println!("   Logs: {}", daemon::daemon_log_path().display());
                        if !tailscale_bootstrap.is_empty() {
                            println!("   Tailscale bootstrap: {}", tailscale_bootstrap.join(", "));
                        }
                        if effective_mesh_port > 0 {
                            println!("   Mesh port: {}", effective_mesh_port);
                        }
                        println!("   QUIC port: {}", effective_quic_port);
                        if mesh_size != "small" {
                            println!("   Mesh size: {}", mesh_size);
                        }
                        println!();
                        daemon::wait_for_ready(args.port, Duration::from_secs(20))?;
                        println!(
                            "✅ GhostWire is ready on http://{}:{}",
                            effective_api_host, args.port
                        );
                        match interface {
                            Interface::Web => {
                                let browser_host = if api_host == "0.0.0.0" {
                                    "127.0.0.1"
                                } else {
                                    api_host
                                };
                                let url = format!("http://{}:{}", browser_host, args.port);
                                println!("🌐 Web UI: {}", url);
                                if std::env::var_os("DISPLAY").is_some()
                                    || std::env::var_os("WAYLAND_DISPLAY").is_some()
                                {
                                    let _ = Command::new("xdg-open").arg(&url).spawn();
                                }
                            }
                            Interface::Tui => {
                                ghostwire::tui::run_tui(args.port)
                                    .await
                                    .map_err(|e| anyhow::anyhow!("TUI error: {}", e))?;
                            }
                            Interface::None => {}
                        }
                    }
                    Err(e) => {
                        eprintln!("❌ {}", e);
                        return Err(e);
                    }
                }
                return Ok(());
            }
            Some(Commands::Stop) => {
                match daemon::stop_daemon() {
                    Ok(()) => {
                        println!("✅ GhostWire daemon stopped");
                    }
                    Err(e) => {
                        eprintln!("❌ {}", e);
                        return Err(e);
                    }
                }
                return Ok(());
            }
            Some(Commands::Status) => {
                println!("{}", daemon::daemon_status());
                return Ok(());
            }
            Some(Commands::Tui) => {
                let port = args.port;
                if !daemon::is_daemon_running() {
                    let extra = daemon::load_saved_args().unwrap_or_else(|| daemon::DaemonArgs {
                        api_host: "127.0.0.1".to_string(),
                        mesh_port: 4001,
                        quic_port: 8081,
                        ..Default::default()
                    });
                    match daemon::start_daemon(Some(port), Some(extra)) {
                        Ok(pid) => {
                            eprintln!("✅ Daemon started (PID {})", pid);
                            daemon::wait_for_ready(port, Duration::from_secs(20))?;
                        }
                        Err(e) => {
                            eprintln!("❌ Failed to auto-start daemon: {}", e);
                            return Err(e);
                        }
                    }
                }
                daemon::wait_for_ready(port, Duration::from_secs(20))?;
                eprintln!("🔌 Connecting to daemon on port {}...", port);
                ghostwire::tui::run_tui(port)
                    .await
                    .map_err(|e| anyhow::anyhow!("TUI error: {}", e))?;
                return Ok(());
            }
            Some(Commands::Setup) => {
                use ghostwire::core::identity::core_identity::Identity;
                let storage_path = std::env::var("GHOSTWIRE_HOME")
                    .map(std::path::PathBuf::from)
                    .unwrap_or_else(|_| dirs::home_dir().unwrap_or_default().join(".ghostwire"))
                    .join("identity.json");

                if storage_path.exists() {
                    let identity = Identity::load_or_create(None)
                        .map_err(|e| anyhow::anyhow!("Failed to load identity: {}", e))?;
                    let meta = identity.get_metadata();
                    println!("Identity already exists ({})", &identity.id[..8]);
                    if identity.is_locked() {
                        println!(
                            "It is currently locked. Unlock via POST /api/v2/profile/unlock first."
                        );
                        return Ok(());
                    }
                    println!("It is currently unlocked.\n");

                    print!("Nickname [{}]: ", meta.nickname);
                    use std::io::Write;
                    std::io::stdout().flush()?;
                    let mut nickname = String::new();
                    std::io::stdin().read_line(&mut nickname)?;
                    let nickname = nickname.trim();
                    let nickname = if nickname.is_empty() {
                        meta.nickname.clone()
                    } else {
                        nickname.to_string()
                    };

                    print!("Bio [{}]: ", meta.bio);
                    std::io::stdout().flush()?;
                    let mut bio = String::new();
                    std::io::stdin().read_line(&mut bio)?;
                    let bio = bio.trim();
                    let bio = if bio.is_empty() {
                        meta.bio.clone()
                    } else {
                        bio.to_string()
                    };

                    identity.update_metadata(Some(nickname.clone()), Some(bio), None, Some(true));

                    print!("Password to save: ");
                    std::io::stdout().flush()?;
                    let mut password = String::new();
                    std::io::stdin().read_line(&mut password)?;
                    let password = password.trim().to_string();

                    if password.is_empty() {
                        eprintln!("❌ Password required to save identity");
                        return Ok(());
                    }

                    identity
                        .save_to_disk(&password)
                        .map_err(|e| anyhow::anyhow!("Failed to save: {}", e))?;

                    let _ = std::fs::remove_file(storage_path.with_file_name(".initial-password"));

                    println!("\n✅ Identity updated");
                    println!("   Nickname: {}", nickname);
                    println!("   DID: {}", identity.did);
                    println!("   API: http://127.0.0.1:{}", args.port);
                    println!("\nRun 'ghostwire start' to begin.");
                } else {
                    println!("No identity found. Run 'ghostwire start' to create one.");
                }
                return Ok(());
            }
            _ => {}
        }
    }

    // Load configuration (only needed for Core-based commands)
    let mut config = if let Some(config_path) = &args.config {
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

    // Auto-update check (skip for daemon mode — it runs forever).
    // Only the signed-manifest updater is allowed to inspect or install updates.
    if !args.daemon && config.server.auto_update {
        if let Some(manifest) = updater_fixed::check_for_update().await {
            info!(
                "✨ Applying verified GhostWire update {}...",
                manifest.version
            );
            if let Err(e) = updater_fixed::apply_update(&manifest).await {
                error!("❌ Verified automatic update failed: {}", e);
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

    // Parse Tailscale bootstrap nodes
    for ts in &args.tailscale_bootstrap {
        if let Some((ip, peer_id)) = ts.split_once('/') {
            if let Ok(peer_id) = peer_id.parse::<libp2p::PeerId>() {
                let mesh_port = if args.mesh_port > 0 {
                    args.mesh_port
                } else {
                    ghostwire::core::constants::network::DEFAULT_MESH_PORT
                };
                let addr_str = format!("/ip4/{}/tcp/{}", ip, mesh_port);
                if let Ok(ma) = addr_str.parse::<libp2p::Multiaddr>() {
                    let full_addr = ma.with(libp2p::multiaddr::Protocol::P2p(peer_id));
                    tracing::info!(
                        "Adding Tailscale bootstrap node: {} ({})",
                        peer_id,
                        full_addr
                    );
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

    // If TAILSCALE=1, scan for GhostWire nodes on Tailscale network
    if std::env::var("TAILSCALE").unwrap_or_default() == "1" {
        tracing::info!("TAILSCALE=1 detected — scanning 100.64.0.0/10 for GhostWire peers");
    }

    // Cloudflare bootstrap workers (opt-in seed hints, never authority).
    // GHOSTWIRE_BOOTSTRAP_WORKER_URLS: comma-separated independent rendezvous
    // (different operators — same-domain entries add availability only, not
    // privacy or eclipse resistance). Unset = standalone mode. Records from
    // every source are untrusted hints: validated here, gated by the
    // GhostWire handshake + local admission before any application use.
    // Singular GHOSTWIRE_BOOTSTRAP_WORKER_URL still honored as a one-entry list.
    let worker_urls: Vec<String> = std::env::var("GHOSTWIRE_BOOTSTRAP_WORKER_URLS")
        .map(|v| {
            v.split(',')
                .map(str::trim)
                .filter(|s| !s.is_empty())
                .map(|s| s.trim_end_matches('/').to_string())
                .collect()
        })
        .unwrap_or_default();
    let worker_urls: Vec<String> = if worker_urls.is_empty() {
        std::env::var("GHOSTWIRE_BOOTSTRAP_WORKER_URL")
            .map(|v| vec![v.trim_end_matches('/').to_string()])
            .unwrap_or_default()
    } else {
        worker_urls
    };
    for base in &worker_urls {
        match fetch_worker_bootstrap_peers(base).await {
            Ok(addrs) => {
                tracing::info!(worker = %base, seeds = addrs.len(), "Bootstrap worker supplied seed hints");
                bootstrap_multiaddrs.extend(addrs);
            }
            Err(e) => tracing::warn!(worker = %base, "Bootstrap worker unreachable, continuing: {}", e),
        }
    }

    // Set mesh size for gossipsub config (read by hardened_gossipsub_config)
    std::env::set_var("GHOSTWIRE_MESH_SIZE", &args.mesh_size);

    // Apply the CLI transport override without changing the persisted identity.
    config.server.quic_port = args.quic_port;

    // Restore the last selected profile unless the CLI explicitly overrides it.
    let selected_profile = args
        .profile
        .clone()
        .or_else(ProfilesController::load_active_profile);

    // Initialize core components
    let core = Core::new(
        config.clone(),
        bootstrap_multiaddrs.clone(),
        args.sim_addr.clone(),
        selected_profile,
    )
    .await?;
    let core_arc = Arc::new(core);

    match args.command {
        #[cfg(feature = "cli")]
        Some(Commands::Cli { rest }) => {
            let mut cli_args = vec![OsString::from("ghostwire")];
            cli_args.extend(rest.into_iter().map(OsString::from));
            ghostwire::cli::run_cli_with_args_and_core(cli_args, core_arc).await?;
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
        Some(Commands::Web {
            api_host,
            mesh_host,
            port,
            quic_port,
            bootstrap,
        }) => {
            info!("Starting web server via CLI on {}:{}", api_host, port);
            let mut bs = bootstrap_multiaddrs.clone();
            for b in &bootstrap {
                if let Ok(ma) = b.parse::<libp2p::Multiaddr>() {
                    bs.push(ma);
                }
            }
            core_arc
                .clone()
                .start_nervous_system(mesh_host.clone(), 0)
                .await;
            ghostwire::web::start_web_server(
                Some(core_arc),
                api_host,
                port,
                bs,
                None,
                mesh_host,
                0,
                config.clone(),
            )
            .await
            .map_err(|e| anyhow::anyhow!(e))?;
        }
        #[cfg(feature = "cli")]
        Some(Commands::Start { .. })
        | Some(Commands::Stop)
        | Some(Commands::Status)
        | Some(Commands::Tui)
        | Some(Commands::Setup) => {
            // Already handled above (before Core creation)
            unreachable!()
        }
        #[cfg(not(feature = "cli"))]
        Some(_) => {
            println!("CLI feature not enabled.");
        }
        None => {
            if args.daemon {
                // ═══════════════════════════════════════════════
                // DAEMON MODE — runs in background, owns the DB
                // ═══════════════════════════════════════════════
                info!("🔧 Running as daemon on port {}", args.port);

                // Start the nervous system (mesh networking, AI, etc.)
                core_arc
                    .clone()
                    .start_nervous_system(args.mesh_host.clone(), args.mesh_port)
                    .await;

                // Start the web API server (this is how clients talk to the daemon).
                // Honor the operator-selected bind; the CLI default remains loopback.
                let daemon_api_host = args.api_host.clone();
                info!(
                    "🌐 Daemon API listening on {}:{}",
                    daemon_api_host, args.port
                );
                ghostwire::web::start_web_server(
                    Some(core_arc),
                    daemon_api_host,
                    args.port,
                    bootstrap_multiaddrs.clone(),
                    None,
                    args.mesh_host.clone(),
                    args.mesh_port,
                    config.clone(),
                )
                .await
                .map_err(|e| anyhow::anyhow!(e))?;
            } else {
                // ═══════════════════════════════════════════════
                // INTERACTIVE MODE — direct Core access (legacy)
                // ═══════════════════════════════════════════════
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
    }

    Ok(())
}
