use anyhow::Result;
use clap::Parser;
use ghostwire::Config;
use ghostwire::Core;
use ghostwire::Message;
use libp2p::Multiaddr;
use std::sync::Arc;
use tokio::sync::Mutex;
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
    mode: String,

    #[arg(long)]
    dm: bool,

    #[arg(long, default_value_t = 10)]
    stats_interval: u64,

    #[arg(long)]
    profile: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    info!("Starting Mesh Isolate Test (mode: {})", args.mode);

    let mut config = Config::default();
    config.server.host = args.host.clone();
    config.server.port = 9000;
    config.network.mesh_enabled = true;

    let bootstrap_nodes: Vec<Multiaddr> = args
        .bootstrap
        .iter()
        .filter_map(|s| s.parse().ok())
        .collect();

    let core = Core::new(config, bootstrap_nodes, None, args.profile).await?;
    let core_arc = Arc::new(core);

    core_arc
        .clone()
        .start_nervous_system(args.host, args.port)
        .await;

    let mut events = core_arc.event_bus.subscribe();

    let my_id = core_arc.get_identity_id().await;
    info!("My identity: {}", my_id);

    let known_peers: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));

    time::sleep(Duration::from_secs(3)).await;

    let peers_for_events = known_peers.clone();
    let event_handle = tokio::spawn(async move {
        while let Ok(event) = events.recv().await {
            match event {
                ghostwire::core::CoreEvent::NewMessage(msg) => {
                    let enc = if msg.message_type.starts_with("encrypted:v4:") {
                        "ratchet"
                    } else if msg.message_type.starts_with("encrypted:") {
                        "static"
                    } else {
                        "plain"
                    };
                    info!(
                        "  RECV [{}] from {}: {} (enc={}, hops={}, ttl={})",
                        &msg.id.to_string()[..8],
                        &msg.sender[..msg.sender.len().min(12)],
                        &msg.content[..msg.content.len().min(60)],
                        enc,
                        msg.hops,
                        msg.ttl,
                    );
                }
                ghostwire::core::CoreEvent::PeerConnected(peer) => {
                    info!("  PEER+ {}", &peer[..peer.len().min(20)]);
                    let mut peers = peers_for_events.lock().await;
                    if !peers.contains(&peer) {
                        peers.push(peer);
                    }
                }
                ghostwire::core::CoreEvent::PeerDisconnected(peer) => {
                    info!("  PEER- {}", &peer[..peer.len().min(20)]);
                    let mut peers = peers_for_events.lock().await;
                    peers.retain(|p| p != &peer);
                }
                ghostwire::core::CoreEvent::SecurityLockdown(active) => {
                    warn!("  LOCKDOWN: {}", active);
                }
                _ => {}
            }
        }
    });

    let mut ping_interval = time::interval(Duration::from_secs(5));
    let mut stats_interval = time::interval(Duration::from_secs(args.stats_interval));
    let mut tick_count: u64 = 0;

    loop {
        tokio::select! {
            _ = ping_interval.tick() => {
                tick_count += 1;

                if args.mode == "ping" {
                    let ping = Message::new(
                        "system".to_string(),
                        "all".to_string(),
                        format!("PING #{}", tick_count),
                    );
                    if let Err(e) = core_arc.send_message(&ping).await {
                        warn!("Failed to send broadcast PING: {}", e);
                    } else {
                        info!("TX broadcast PING #{}", tick_count);
                    }

                    if args.dm {
                        let peers = known_peers.lock().await;
                        if let Some(target) = peers.first() {
                            let dm = Message::new(
                                my_id.clone(),
                                target.clone(),
                                format!("DM-PING #{}", tick_count),
                            );
                            if let Err(e) = core_arc.send_direct_message(target.clone(), &dm).await {
                                warn!("Failed to send DM: {}", e);
                            } else {
                                info!("TX DM PING #{} to {}", tick_count, &target[..target.len().min(12)]);
                            }
                        } else {
                            info!("DM waiting for peer connection...");
                        }
                    }
                }

                if args.mode == "pong" && tick_count > 1 {
                    let msgs = core_arc.get_messages("all", 1).await;
                    if let Some(last) = msgs.last() {
                        if last.message_type != "ack" {
                            let reply = Message::new(
                                my_id.clone(),
                                last.sender.clone(),
                                format!("PONG: {}", last.content),
                            );
                            if args.dm {
                                if let Err(e) = core_arc.send_direct_message(last.sender.clone(), &reply).await {
                                    warn!("Failed to send DM PONG: {}", e);
                                } else {
                                    info!("TX DM PONG to {}", &last.sender[..last.sender.len().min(12)]);
                                }
                            } else {
                                if let Err(e) = core_arc.send_message(&reply).await {
                                    warn!("Failed to send broadcast PONG: {}", e);
                                } else {
                                    info!("TX broadcast PONG");
                                }
                            }
                        }
                    }
                }
            }

            _ = stats_interval.tick() => {
                if let Some(stats) = core_arc.get_mesh_stats().await {
                    info!(
                        "STATS peers={} online={} local={}",
                        stats.total_nodes,
                        stats.online_nodes,
                        &stats.local_node_id[..stats.local_node_id.len().min(12)],
                    );
                }

                let rs = core_arc.reliability.stats();
                info!(
                    "  RELIABILITY ack_pending={} ack_rcvd={} ack_timeout={} retry_queued={} retry_done={} retry_drop={}",
                    rs.ack.pending,
                    rs.ack.acks_received,
                    rs.ack.timeouts,
                    rs.retry.queued,
                    rs.retry.retried,
                    rs.retry.dropped,
                );

                let msgs = core_arc.get_messages("all", 1000).await;
                info!("  MESSAGES total={}", msgs.len());
            }

            _ = tokio::signal::ctrl_c() => {
                info!("Shutting down...");
                event_handle.abort();
                break;
            }
        }
    }

    Ok(())
}
