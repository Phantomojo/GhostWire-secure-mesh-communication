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
// `disable_help_flag` was set here, which made `ghostwire --help` fail with
// "unexpected argument '--help' found" — the single most conventional flag in
// any CLI. No custom help field replaced it (the `help` subcommand is
// clap's own), so it was simply a leftover. `-h` is free: the only short flag
// already claimed is `-v` for --verbose.
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

    /// Peer multiaddrs to bootstrap the mesh from (repeatable)
    #[arg(long)]
    bootstrap: Vec<String>,

    /// Deprecated: use the `web` subcommand instead
    #[arg(long, hide = true)]
    web: bool,

    /// Deprecated: this flag was never read; use the `cli` subcommand instead
    #[arg(long, hide = true)]
    cli: bool,

    /// Run in mesh node mode
    #[arg(long)]
    mesh: bool,

    /// Path to a configuration file
    #[arg(long)]
    config: Option<String>,

    /// Simulation listen address (used by the test harness)
    #[arg(long)]
    sim_addr: Option<String>,

    /// Identity profile to operate as
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
        /// Mesh (libp2p) listen port. 0 = ephemeral, which binds a random port
        /// and leaves you with nothing stable to firewall or advertise.
        #[arg(long, default_value_t = 4001)]
        mesh_port: u16,
        /// Dev/test node: machine-keyed persistence with no passphrase, so the
        /// node can actually store messages without anyone unlocking it.
        /// For throwaway and CI nodes only -- NOT for real identities.
        #[arg(long)]
        dev_profile: bool,
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

/// Print a daemon-client failure. The "is it running?" hint is only shown when
/// nothing was listening — after a 500 from a live daemon it is noise, and it
/// previously appeared on every error.
fn report_daemon_error(err: anyhow::Error) {
    use ghostwire::cli::DaemonError;
    eprintln!("\n❌ {}", err);
    let unreachable = err
        .downcast_ref::<DaemonError>()
        .map(|d| matches!(d, DaemonError::Unreachable(_)))
        .unwrap_or(false);
    if unreachable {
        eprintln!("   Is the daemon running? Try: ghostwire start");
    }
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

    // `status`, `peers` and `send` are short-lived client commands that talk to
    // an existing daemon. The startup banner is noise for them: it appeared
    // above the actual output and made a simple `ghostwire status` look like it
    // was booting a node. Keep it for the long-running modes.
    let is_daemon_client = matches!(
        args.command,
        Some(Commands::Status) | Some(Commands::Peers) | Some(Commands::Send { .. })
    );
    if !is_daemon_client {
        info!("🚀 Starting GhostWire - Secure Mesh Networking and Messaging");
    }

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
                    // Forwarded so the child honours them. Previously dropped:
                    // the child re-executes this binary and rebuilt its own
                    // argv, so a custom config file was silently ignored and
                    // `--mesh-size large` printed "large" while gossipsub ran
                    // with mesh_n=2.
                    config: args.config.clone(),
                    profile: args.profile.clone(),
                    mesh_size: Some(args.mesh_size.clone()),
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
                        daemon::wait_for_ready(
                            args.port,
                            &effective_api_host,
                            Duration::from_secs(20),
                        )?;
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
            // `peers` and `send` are daemon CLIENTS. They previously fell
            // through to the code below that constructs a full in-process
            // `Core` (own identity, own mesh, own crypto state) just to print
            // a list or send a message, so with a daemon already running they
            // reported a state the daemon knew nothing about. Route them to the
            // running daemon's HTTP API, like `status` and `tui` already do.
            // NOTE: deliberately no `is_daemon_running()` gate here. That
            // helper only checks a PID file written by `ghostwire start`, so a
            // daemon started any other way — `ghostwire web` in the
            // foreground, a service unit, a manually launched process — is
            // reported as "not running" even while it is serving HTTP. The
            // request itself is the real liveness probe, and its connection
            // error produces a better message than a stale PID file.
            Some(Commands::Peers) => {
                let port = args.port;
                if let Err(e) = ghostwire::cli::list_peers_from_daemon(port).await {
                    report_daemon_error(e);
                    std::process::exit(1);
                }
                return Ok(());
            }
            Some(Commands::Send { recipient, content }) => {
                let port = args.port;
                if let Err(e) =
                    ghostwire::cli::send_message_via_daemon(port, &recipient, &content).await
                {
                    report_daemon_error(e);
                    std::process::exit(1);
                }
                return Ok(());
            }
            Some(Commands::Tui) => {
                let port = args.port;
                if !daemon::is_daemon_running() {
                    let extra = daemon::load_saved_args().unwrap_or_else(|| daemon::DaemonArgs {
                        api_host: "127.0.0.1".to_string(),
                        config: args.config.clone(),
                        profile: args.profile.clone(),
                        mesh_size: Some(args.mesh_size.clone()),
                        tailscale_bootstrap: Vec::new(),
                        bootstrap: Vec::new(),
                        mesh_port: 4001,
                        quic_port: 8081,
                        ..Default::default()
                    });
                    let tui_api_host = extra.api_host.clone();
                    match daemon::start_daemon(Some(port), Some(extra)) {
                        Ok(pid) => {
                            eprintln!("✅ Daemon started (PID {})", pid);
                            daemon::wait_for_ready(port, &tui_api_host, Duration::from_secs(20))?;
                        }
                        Err(e) => {
                            eprintln!("❌ Failed to auto-start daemon: {}", e);
                            return Err(e);
                        }
                    }
                }
                // The daemon may have been started earlier with a non-loopback
                // api-host; probe that address, not a hardcoded 127.0.0.1.
                let tui_api_host = daemon::load_saved_args()
                    .map(|a| a.api_host)
                    .filter(|h| !h.is_empty())
                    .unwrap_or_else(|| "127.0.0.1".to_string());
                daemon::wait_for_ready(port, &tui_api_host, Duration::from_secs(20))?;
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
                    // First run: no identity exists yet.
                    //
                    // This branch used to print "No identity found. Run
                    // 'ghostwire start' to create one." and exit — which made
                    // the interactive setup unreachable in the only situation
                    // its help text ("Interactive first-run setup") describes,
                    // while it worked only when an identity already existed
                    // and setup was unnecessary.
                    //
                    // Create it here instead, using the same prompts. Defaults
                    // are accepted by pressing Enter, so the common case is
                    // three Enters. `GHOSTWIRE_PASSWORD` is honoured for
                    // scripted/unattended first runs; it is deliberately NOT a
                    // flag, because a password on the command line lands in
                    // shell history and is visible in `ps`.
                    use std::io::Write;
                    let ghost_home = std::env::var("GHOSTWIRE_HOME")
                        .map(std::path::PathBuf::from)
                        .unwrap_or_else(|_| {
                            dirs::home_dir().unwrap_or_default().join(".ghostwire")
                        });
                    let _ = std::fs::create_dir_all(&ghost_home);

                    println!("Welcome to GhostWire — first-run setup.");
                    println!("Press Enter to accept the default in each prompt.\n");

                    print!("Nickname [ghostwire]: ");
                    std::io::stdout().flush()?;
                    let mut nickname = String::new();
                    std::io::stdin().read_line(&mut nickname)?;
                    let nickname = nickname.trim();
                    let nickname = if nickname.is_empty() {
                        "ghostwire".to_string()
                    } else {
                        nickname.to_string()
                    };

                    print!("Bio [anonymous mesh node]: ");
                    std::io::stdout().flush()?;
                    let mut bio = String::new();
                    std::io::stdin().read_line(&mut bio)?;
                    let bio = bio.trim().to_string();

                    let password = match std::env::var("GHOSTWIRE_PASSWORD") {
                        Ok(p) if !p.is_empty() => {
                            println!("(password taken from GHOSTWIRE_PASSWORD)");
                            p
                        }
                        _ => {
                            print!("Password: ");
                            std::io::stdout().flush()?;
                            let mut p = String::new();
                            std::io::stdin().read_line(&mut p)?;
                            p.trim().to_string()
                        }
                    };

                    // Match the API's own rule (controllers/identity.rs) so the
                    // CLI cannot create an identity the HTTP API would reject.
                    if password.as_bytes().len() < 12 {
                        eprintln!("❌ Password must be at least 12 bytes. Nothing was created.");
                        return Ok(());
                    }

                    let identity = Identity::load_or_create(None)
                        .map_err(|e| anyhow::anyhow!("Failed to create identity: {}", e))?;
                    identity.update_metadata(
                        Some(nickname.clone()),
                        if bio.is_empty() { None } else { Some(bio) },
                        None,
                        Some(true),
                    );
                    identity
                        .save_to_disk(&password)
                        .map_err(|e| anyhow::anyhow!("Failed to save identity: {}", e))?;

                    let _ = std::fs::remove_file(storage_path.with_file_name(".initial-password"));

                    println!("\n✅ Identity created");
                    println!("   Nickname: {}", nickname);
                    println!("   DID: {}", identity.did);
                    println!("   Stored in: {}", ghost_home.display());
                    println!("\nRun 'ghostwire start' to begin.");
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
            Err(e) => {
                tracing::warn!(worker = %base, "Bootstrap worker unreachable, continuing: {}", e)
            }
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

    // Dev-profile mode must be known BEFORE Core is constructed: Core reads
    // this to decide between a locked passphrase profile and machine-keyed
    // storage. Setting it inside the command match was too late -- core_arc was
    // already built by then, so `--dev-profile` silently did nothing and the
    // node came up locked anyway.
    if matches!(
        &args.command,
        Some(Commands::Web {
            dev_profile: true,
            ..
        })
    ) {
        std::env::set_var("GHOSTWIRE_DEV_PROFILE", "1");
        println!(
            "\n  ⚠️  DEV PROFILE MODE — machine-keyed storage, NO passphrase.\n     \
             Throwaway/testing node. Do not put real secrets on it.\n"
        );
    }

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
            mesh_port,
            dev_profile,
            port,
            quic_port,
            bootstrap,
        }) => {
            info!("Starting web server via CLI on {}:{}", api_host, port);
            let _ = dev_profile; // handled before Core::new
            let mut bs = bootstrap_multiaddrs.clone();
            for b in &bootstrap {
                if let Ok(ma) = b.parse::<libp2p::Multiaddr>() {
                    bs.push(ma);
                }
            }
            core_arc
                .clone()
                .start_nervous_system(mesh_host.clone(), mesh_port)
                .await;
            // Apply the web subcommand's own --quic-port. The earlier setup
            // assigned from the global args, which shadowed this flag, so
            // `ghostwire web --quic-port N` was accepted and then ignored.
            let mut web_config = config.clone();
            web_config.server.quic_port = quic_port;

            ghostwire::web::start_web_server(
                Some(core_arc),
                api_host,
                port,
                bs,
                None,
                mesh_host,
                mesh_port,
                web_config,
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
