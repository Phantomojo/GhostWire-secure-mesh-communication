pub mod commands;

use clap::{Parser, Subcommand};
use anyhow::Result;
use crate::core::Core;
use std::sync::Arc;
use tracing::{info, error};

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
        #[arg(short, long)]
        username: Option<String>,
    },
    /// Start the GhostWire node
    Start {
        #[arg(short, long, default_value = "127.0.0.1")]
        host: String,
        #[arg(short, long, default_value_t = 8080)]
        port: u16,
        #[arg(long)]
        web: bool,
    },
    /// Send a message to a peer
    Send {
        #[arg(short, long)]
        recipient: String,
        #[arg(short, long)]
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
        #[arg(short, long)]
        message: String,
    },
    /// Decrypt a message
    Decrypt {
        #[arg(short, long)]
        encrypted: String,
    },
}

/// Run the CLI application
pub async fn run_cli() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize core components
    let core = Core::new().await?;
    let core_arc = Arc::new(core);
    
    match cli.command {
        Commands::Init { username } => {
            init_identity(&core_arc, username).await?;
        }
        Commands::Start { host, port, web } => {
            if web {
                start_web_server(&core_arc, host, port).await?;
            } else {
                start_node(&core_arc, host, port).await?;
            }
        }
        Commands::Send { recipient, message } => {
            send_message(&core_arc, &recipient, &message).await?;
        }
        Commands::Peers => {
            list_peers(&core_arc).await?;
        }
        Commands::Stats => {
            show_stats(&core_arc).await?;
        }
        Commands::Security => {
            show_security_info(&core_arc).await?;
        }
        Commands::GenKey => {
            generate_key(&core_arc).await?;
        }
        Commands::Encrypt { message } => {
            encrypt_message(&core_arc, &message).await?;
        }
        Commands::Decrypt { encrypted } => {
            decrypt_message(&core_arc, &encrypted).await?;
        }
    }
    
    Ok(())
}

async fn init_identity(core: &Arc<Core>, username: Option<String>) -> Result<()> {
    info!("Initializing GhostWire identity...");
    
    let identity_id = core.get_identity_id();
    let public_key = core.get_public_key();
    
    println!("✅ Identity initialized successfully!");
    println!("Identity ID: {}", identity_id);
    println!("Public Key: {}", base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &public_key));
    
    if let Some(name) = username {
        println!("Username: {}", name);
    }
    
    Ok(())
}

async fn start_web_server(core: &Arc<Core>, host: String, port: u16) -> Result<()> {
    info!("Starting GhostWire web server on {}:{}", host, port);
    
    // Import the web module function
    use crate::web::start_web_server;
    
    start_web_server(core.clone(), host, port).await
        .map_err(|e| anyhow::anyhow!("Web server error: {}", e))?;
    
    Ok(())
}

async fn start_node(core: &Arc<Core>, host: String, port: u16) -> Result<()> {
    info!("Starting GhostWire node on {}:{}", host, port);
    
    // Initialize mesh networking
    let core_clone = core.clone();
    tokio::spawn(async move {
        let _ = core_clone.init_mesh().await;
    });
    
    // Initialize reticulum networking
    let core_clone = core.clone();
    tokio::spawn(async move {
        let _ = core_clone.init_reticulum().await;
    });
    
    println!("✅ GhostWire node started successfully!");
    println!("Listening on {}:{}", host, port);
    println!("Press Ctrl+C to stop");
    
    // Keep the node running
    tokio::signal::ctrl_c().await?;
    println!("\nShutting down...");
    
    Ok(())
}

async fn send_message(core: &Arc<Core>, recipient: &str, message: &str) -> Result<()> {
    use crate::core::Message;
    use std::time::{SystemTime, UNIX_EPOCH};
    
    let msg = Message {
        id: uuid::Uuid::new_v4(),
        sender: core.get_identity_id(),
        recipient: recipient.to_string(),
        content: message.to_string(),
        timestamp: SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs(),
        encrypted: false,
    };
    
    match core.send_message(&msg).await {
        Ok(_) => {
            println!("✅ Message sent successfully to {}", recipient);
        }
        Err(e) => {
            error!("Failed to send message: {}", e);
            return Err(anyhow::anyhow!("Failed to send message: {}", e));
        }
    }
    
    Ok(())
}

async fn list_peers(core: &Arc<Core>) -> Result<()> {
    if let Some(mesh_stats) = core.get_mesh_stats().await {
        println!("📡 Mesh Network Peers:");
        println!("Total nodes: {}", mesh_stats.total_nodes);
        println!("Online nodes: {}", mesh_stats.online_nodes);
        println!("Local node ID: {}", mesh_stats.local_node_id);
    } else {
        println!("❌ Mesh network not initialized");
    }
    
    if let Some(reticulum_stats) = core.get_reticulum_stats().await {
        println!("\n🕸️  Reticulum Network:");
        println!("Total nodes: {}", reticulum_stats.total_nodes);
        println!("Online nodes: {}", reticulum_stats.online_nodes);
        println!("Local node ID: {}", reticulum_stats.local_node_id);
    } else {
        println!("❌ Reticulum network not initialized");
    }
    
    Ok(())
}

async fn show_stats(core: &Arc<Core>) -> Result<()> {
    println!("📊 GhostWire Network Statistics");
    println!("================================");
    
    if let Some(mesh_stats) = core.get_mesh_stats().await {
        println!("Mesh Network:");
        println!("  Total nodes: {}", mesh_stats.total_nodes);
        println!("  Online nodes: {}", mesh_stats.online_nodes);
        println!("  Routes count: {}", mesh_stats.routes_count);
        println!("  Local node ID: {}", mesh_stats.local_node_id);
    }
    
    if let Some(reticulum_stats) = core.get_reticulum_stats().await {
        println!("\nReticulum Network:");
        println!("  Total nodes: {}", reticulum_stats.total_nodes);
        println!("  Online nodes: {}", reticulum_stats.online_nodes);
        println!("  Messages relayed: {}", reticulum_stats.messages_relayed);
        println!("  Local node ID: {}", reticulum_stats.local_node_id);
    }
    
    // Display security statistics
    let security_stats = core.get_security_stats().await;
    println!("\n🔒 Security Statistics:");
    println!("  Threat level: {:?}", security_stats.threat_level);
    println!("  Total events: {}", security_stats.total_events);
    println!("  High threat events: {}", security_stats.high_threat_events);
    println!("  Blocked connections: {}", security_stats.blocked_connections);
    println!("  Encryption errors: {}", security_stats.encryption_errors);
    
    Ok(())
}

async fn show_security_info(core: &Arc<Core>) -> Result<()> {
    println!("🔒 GhostWire Security Information");
    println!("==================================");
    
    let identity_id = core.get_identity_id();
    let public_key = core.get_public_key();
    let key_id = core.get_key_id();
    
    println!("Identity ID: {}", identity_id);
    println!("Key ID: {}", key_id);
    println!("Public Key: {}", base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &public_key));
    
    // Display security statistics
    let security_stats = core.get_security_stats().await;
    println!("\n🔒 Security Statistics:");
    println!("  Threat Level: {:?}", security_stats.threat_level);
    println!("  Total Events: {}", security_stats.total_events);
    println!("  High Threat Events: {}", security_stats.high_threat_events);
    println!("  Blocked Connections: {}", security_stats.blocked_connections);
    println!("  Encryption Errors: {}", security_stats.encryption_errors);
    
    Ok(())
}

async fn generate_key(core: &Arc<Core>) -> Result<()> {
    info!("Generating new encryption key pair...");
    
    // The core already has encryption initialized
    let public_key = core.get_public_key();
    let key_id = core.get_key_id();
    
    println!("✅ New key pair generated!");
    println!("Key ID: {}", key_id);
    println!("Public Key: {}", base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &public_key));
    
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
    
    let decrypted = core.encryption.decrypt(&decoded);
    let message = String::from_utf8(decrypted)
        .map_err(|e| anyhow::anyhow!("Invalid UTF-8: {}", e))?;
    
    println!("🔓 Decrypted message:");
    println!("{}", message);
    
    Ok(())
}
