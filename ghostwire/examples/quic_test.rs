use ghostwire::core::message::Message;
use ghostwire::core::quic_transport::QuicTransportProvider;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_env_filter("ghostwire=info")
        .init();

    let args: Vec<String> = std::env::args().collect();

    if args.iter().any(|a| a == "--server") {
        let port: u16 = args
            .windows(2)
            .find(|w| w[0] == "--port")
            .and_then(|w| w[1].parse().ok())
            .unwrap_or(5001);

        let addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;
        let transport = QuicTransportProvider::new(addr).await?;
        let mut rx = transport.start_listener();

        println!("[SERVER] QUIC listening on {}", addr);
        println!("[SERVER] Waiting for messages...\n");

        while let Some(msg) = rx.recv().await {
            println!("[RECV] from {}: {}", msg.sender, msg.content);
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
            .unwrap_or_else(|| "Hello from GhostWire QUIC!".to_string());

        let recipient = args
            .windows(2)
            .find(|w| w[0] == "--recipient")
            .map(|w| w[1].clone())
            .unwrap_or_else(|| "remote".to_string());

        let port: u16 = args
            .windows(2)
            .find(|w| w[0] == "--port")
            .and_then(|w| w[1].parse().ok())
            .unwrap_or(5002);

        let bind_addr: SocketAddr = format!("0.0.0.0:{}", port).parse()?;
        let transport = QuicTransportProvider::new(bind_addr).await?;

        transport.connect(&recipient, addr).await?;
        let msg = Message::new("local-node".to_string(), recipient.clone(), content.clone());
        transport.send_message(&msg).await?;

        println!("[CLIENT] Sent to {} via QUIC: {}", addr, content);

        // Keep endpoint alive so QUIC can deliver the message before CONNECTION_CLOSE
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    Ok(())
}
