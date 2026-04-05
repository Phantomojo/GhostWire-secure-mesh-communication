use anyhow::Result;
use clap::Parser;
use ghostwire::Config;
use ghostwire::Core;
use ghostwire::Message;
use libp2p::Multiaddr;
use std::sync::Arc;
use tokio::time::{self, Duration};
use tracing::{info, warn};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(long, default_value = "0.0.0.0")]
    host: String,

    #[arg(long, default_value_t = 0)]
    port: u16,

    #[arg(long)]
    bootstrap: Vec<String>,

    #[arg(long)]
    mode: String, // "ping" or "pong"
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    info!("🚀 Starting Mesh Isolate Test (Mode: {})", args.mode);

    // 1. Create a minimal Config
    let mut config = Config::default();
    config.server.host = args.host.clone();
    config.server.port = 9000; // API port (unused here)
    config.network.mesh_enabled = true;

    // Convert bootstrap strings to Multiaddr
    let bootstrap_nodes: Vec<Multiaddr> = args
        .bootstrap
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();

    // 2. Initialize Core (using correct constructor signature)
    let core = Core::new(
        config,
        bootstrap_nodes,
        None, // sim_addr
        None, // profile (None = "default")
    )
    .await?;
    let core_arc = Arc::new(core);

    // 3. Start the Nervous System (this starts libp2p)
    core_arc
        .clone()
        .start_nervous_system(args.host, args.port)
        .await;

    // Wait for mesh to stabilize
    time::sleep(Duration::from_secs(5)).await;

    // 4. Main Loop
    let mut interval = time::interval(Duration::from_secs(5));

    loop {
        tokio::select! {
            _ = interval.tick() => {
                if args.mode == "ping" {
                    let ping = Message::new(
                        "system".to_string(), // sender
                        "all".to_string(),    // recipient (broadcast)
                        "PING".to_string(),   // content
                    );
                    info!("📤 Sending PING...");
                    if let Err(e) = core_arc.send_message(&ping).await {
                        warn!("❌ Failed to send PING: {}", e);
                    }
                }
            }
        }

        // Check for received messages
        if let Some(stats) = core_arc.get_mesh_stats().await {
            if stats.online_nodes > 0 {
                info!("📊 Peer count: {}", stats.online_nodes);
            }
        }
    }
}
