use ghostwire::core::message::Message;
use ghostwire::core::security::{SecurityConfig, SecurityManager};
use ghostwire::core::stealth_tcp::StealthTCPProvider;
use std::net::SocketAddr;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("ghostwire=info")
        .init();

    let args: Vec<String> = std::env::args().collect();

    let security_manager = Arc::new(SecurityManager::new(SecurityConfig::default()));
    let provider = StealthTCPProvider::new(None, security_manager, None, false);

    if args.iter().any(|a| a == "--server") {
        let port: u16 = args
            .windows(2)
            .find(|w| w[0] == "--port")
            .and_then(|w| w[1].parse().ok())
            .unwrap_or(9001);

        let addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;
        let mut rx = provider.start_listener(addr).await?;
        println!("[SERVER] Listening on {}", addr);
        println!("[SERVER] Waiting for messages...\n");

        while let Some((peer, msg)) = rx.recv().await {
            println!("[RECV] from {} via {}: {}", msg.sender, peer, msg.content);
        }
    } else {
        let addr: SocketAddr = args
            .windows(2)
            .find(|w| w[0] == "--addr")
            .and_then(|w| w[1].parse().ok())
            .expect("Usage: --addr <ip:port>");

        let content = args
            .windows(2)
            .find(|w| w[0] == "--message")
            .map(|w| w[1].clone())
            .unwrap_or_else(|| "Hello from GhostWire Stealth TCP!".to_string());

        let recipient = args
            .windows(2)
            .find(|w| w[0] == "--recipient")
            .map(|w| w[1].clone())
            .unwrap_or_else(|| "remote".to_string());

        let msg = Message::new("local-node".to_string(), recipient.clone(), content.clone());
        provider.register_peer(&recipient, addr);

        println!("[CLIENT] Connecting to {}...", addr);
        provider.send_message(&msg).await?;
        println!("[CLIENT] Sent to {}: {}", recipient, content);
    }

    Ok(())
}
