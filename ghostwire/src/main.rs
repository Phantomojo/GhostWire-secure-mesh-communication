// Copyright (C) 2025-2026 Michael Irungu
// SPDX-License-Identifier: AGPL-3.0-or-later

use anyhow::Result;
#[cfg(feature = "cli")]
use clap::{Parser, Subcommand};
use std::ffi::OsString;
use std::sync::Arc;
use std::time::Duration;

use tracing::{error, info};
use tracing_subscriber::prelude::*;

use opentelemetry::trace::TracerProvider;

// Use modules from the library crate
use ghostwire::config::Config;
use ghostwire::core::updater;
use ghostwire::core::Core;
use ghostwire::daemon;

/// Holds the guards that must stay in scope for the app lifetime
struct ObservabilityGuards {
    sentry: Option<sentry::ClientInitGuard>,
    tracer_provider: Option<opentelemetry_sdk::trace::SdkTracerProvider>,
}

/// Initialize logging, Sentry error tracking, and Datadog APM tracing
fn init_logging() -> ObservabilityGuards {
    let sentry_dsn = std::env::var("GHOSTWIRE_SENTRY_DSN").ok();
    let sentry_enabled = sentry_dsn.is_some();
    let dd_api_key = std::env::var("DD_API_KEY").ok();
    let dd_enabled = dd_api_key.is_some();

    // ── Sentry ──────────────────────────────────────────────
    let sentry_guard = if let Some(ref dsn) = sentry_dsn {
        let guard = sentry::init((
            dsn.as_str(),
            sentry::ClientOptions {
                release: sentry::release_name!(),
                traces_sample_rate: 1.0,
                send_default_pii: false,
                ..Default::default()
            },
        ));
        Some(guard)
    } else {
        None
    };

    // ── Datadog APM ─────────────────────────────────────────
    let tracer_provider = if dd_enabled {
        let provider = datadog_opentelemetry::tracing().init();
        Some(provider)
    } else {
        None
    };

    // ── Tracing subscriber pipeline ─────────────────────────
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(true)
        .with_thread_ids(true)
        .with_thread_names(true);

    #[cfg(not(debug_assertions))]
    let fmt_layer = fmt_layer.json();

    let env_filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("ghostwire=info,libp2p=warn"));

    // Build the subscriber pipeline
    // Sentry layer: only sends when DSN is configured (controlled by sentry::init above)
    // Datadog layer: only sends when DD_API_KEY is set (controlled by datadog_opentelemetry::init above)
    if let Some(ref provider) = tracer_provider {
        let tracer = provider.tracer("ghostwire");
        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .with(sentry::integrations::tracing::layer())
            .with(tracing_opentelemetry::layer().with_tracer(tracer))
            .init();
    } else {
        tracing_subscriber::registry()
            .with(env_filter)
            .with(fmt_layer)
            .with(sentry::integrations::tracing::layer())
            .init();
    }

    // Log initialization (now that the subscriber is active)
    if sentry_enabled {
        info!(target: "ghostwire", release = ?sentry::release_name!(), "Sentry error tracking initialized");
    } else {
        info!(target: "ghostwire", "Sentry DSN not configured — error tracking disabled");
    }
    if dd_enabled {
        info!(target: "ghostwire", "Datadog APM tracing initialized (service={})",
            std::env::var("DD_SERVICE").unwrap_or_else(|_| "ghostwire".into()));
    } else {
        info!(target: "ghostwire", "Datadog API key not configured — APM tracing disabled");
    }

    ObservabilityGuards {
        sentry: sentry_guard,
        tracer_provider,
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
    daemon: bool,
    tailscale_bootstrap: Vec<String>,
}

#[cfg(feature = "cli")]
#[derive(Subcommand)]
enum Commands {
    /// Start the GhostWire daemon in the background
    Start,
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
        #[arg(long)]
        bootstrap: Vec<String>,
    },
    /// List known peers
    Peers,
    /// Send a message: send <recipient_id> <content>
    Send {
        recipient: String,
        content: String,
    },
    /// Interactive terminal user interface (connects to daemon)
    Tui,
}

#[cfg(not(feature = "cli"))]
enum Commands {
    Cli,
    Web { host: String, port: u16 },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize observability (guards must be kept in scope)
    let _guards = init_logging();

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
        port: daemon::DEFAULT_DAEMON_PORT,
        bootstrap: Vec::new(),
        web: false,
        cli: false,
        mesh: false,
        profile: None,
        daemon: false,
        tailscale_bootstrap: Vec::new(),
    };

    // Handle daemon management commands (don't need Core — they talk to the daemon)
    #[cfg(feature = "cli")]
    {
        match &args.command {
            Some(Commands::Start) => {
                match daemon::start_daemon(Some(args.port)) {
                    Ok(pid) => {
                        println!("✅ GhostWire daemon started (PID {})", pid);
                        println!("   Port: {}", args.port);
                        println!("   API:  http://127.0.0.1:{}", args.port);
                        println!("   Logs: {}", daemon::daemon_log_path().display());
                        println!();
                        println!("Run 'ghostwire tui' to open the terminal interface.");
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
                // TUI connects to the daemon — ensure it's running
                let port = args.port;
                if !daemon::is_daemon_running() {
                    eprintln!("⚫ Daemon not running — starting it automatically...");
                    match daemon::start_daemon(Some(port)) {
                        Ok(pid) => {
                            eprintln!("✅ Daemon started (PID {})", pid);
                            // Give it a moment to initialize
                            std::thread::sleep(std::time::Duration::from_secs(2));
                        }
                        Err(e) => {
                            eprintln!("❌ Failed to auto-start daemon: {}", e);
                            eprintln!("   Try running 'ghostwire start' manually.");
                            return Err(e);
                        }
                    }
                }

                eprintln!("🔌 Connecting to daemon on port {}...", port);
                ghostwire::tui::run_tui(port).await
                    .map_err(|e| anyhow::anyhow!("TUI error: {}", e))?;
                return Ok(());
            }
            _ => {}
        }
    }

    // Load configuration (only needed for Core-based commands)
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

    // Handle daemon management commands (don't need Core)
    #[cfg(feature = "cli")]
    {
        match &args.command {
            Some(Commands::Start) => {
                match daemon::start_daemon(Some(args.port)) {
                    Ok(pid) => {
                        println!("✅ GhostWire daemon started (PID {})", pid);
                        println!("   Port: {}", args.port);
                        println!("   API:  http://127.0.0.1:{}", args.port);
                        println!("   Logs: {}", daemon::daemon_log_path().display());
                        println!();
                        println!("Run 'ghostwire tui' to open the terminal interface.");
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
            _ => {}
        }
    }

    // Auto-update check (skip for daemon mode — it runs forever)
    if !args.daemon && config.server.auto_update
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

    // Parse Tailscale bootstrap nodes
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

    // If TAILSCALE=1, scan for GhostWire nodes on Tailscale network
    if std::env::var("TAILSCALE").unwrap_or_default() == "1" {
        tracing::info!("TAILSCALE=1 detected — scanning 100.64.0.0/10 for GhostWire peers");
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
    core_arc.link_ai_components();

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
        #[cfg(feature = "cli")]
        Some(Commands::Start) | Some(Commands::Stop) | Some(Commands::Status) | Some(Commands::Tui) => {
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

                // Start the web API server (this is how clients talk to the daemon)
                info!("🌐 Daemon API listening on 127.0.0.1:{}", args.port);
                ghostwire::web::start_web_server(
                    Some(core_arc),
                    "127.0.0.1".to_string(),
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
