use crate::core::message::Message;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;

// Transport Enum for async dispatch
#[derive(Clone)]
pub enum TransportKind {
    #[cfg(feature = "mesh")]
    Mesh(Arc<tokio::sync::RwLock<crate::core::MeshManager>>),
    Dummy, // No-op transport for tests/default
    StealthTCP(Arc<crate::core::stealth_tcp::StealthTCPProvider>),
    Bluetooth(Arc<tokio::sync::RwLock<crate::core::bluetooth::BluetoothTransport>>),
    Simulated(Arc<tokio::sync::RwLock<crate::core::sim_transport::SimTransport>>),
}

impl TransportKind {
    pub fn name(&self) -> &'static str {
        match self {
            #[cfg(feature = "mesh")]
            TransportKind::Mesh(_) => "mesh",
            TransportKind::Dummy => "dummy",
            TransportKind::StealthTCP(_) => "stealth_tcp",
            TransportKind::Bluetooth(_) => "bluetooth",
            TransportKind::Simulated(_) => "simulated",
        }
    }
    pub fn description(&self) -> &'static str {
        match self {
            #[cfg(feature = "mesh")]
            TransportKind::Mesh(_) => "Libp2p-based Mesh Network",
            TransportKind::Dummy => "Dummy transport (no-op)",
            TransportKind::StealthTCP(_) => "Stealth TCP with Traffic Morphing",
            TransportKind::Bluetooth(_) => "Bluetooth Low Energy Mesh",
            TransportKind::Simulated(_) => "Simulated Hardware (BT/WiFi)",
        }
    }
    pub async fn send_message(&self, message: &Message) -> Result<()> {
        match self {
            #[cfg(feature = "mesh")]
            TransportKind::Mesh(manager) => {
                let mut manager = manager.write().await;
                manager.send_message(message).await
            }
            TransportKind::Dummy => {
                tracing::info!("Dummy transport sending message: {}", message.content);
                Ok(())
            }
            TransportKind::StealthTCP(_provider) => {
                tracing::info!(
                    "StealthTCP transport sending message (Mock IP Resolution): {}",
                    message.content
                );
                // In future: Resolve ID -> IP and provider.connect(ip).await
                Ok(())
            }
            TransportKind::Bluetooth(transport) => {
                let transport = transport.read().await;
                transport
                    .send_ble_data(&message.recipient, message.content.as_bytes())
                    .await
            }
            TransportKind::Simulated(transport) => {
                let transport = transport.read().await;
                // Default to Bluetooth mock for теперь
                transport
                    .send_simulated(
                        &message.recipient,
                        crate::core::sim_transport::SimLinkType::Bluetooth,
                        message.content.as_bytes(),
                    )
                    .await
            }
        }
    }
    pub async fn receive_message(&self) -> Result<Option<Message>> {
        match self {
            #[cfg(feature = "mesh")]
            TransportKind::Mesh(_) => Ok(None), // Gossipsub handles receive via events
            TransportKind::Dummy => Ok(None),
            TransportKind::StealthTCP(_) => Ok(None),
            TransportKind::Bluetooth(_) => Ok(None),
            TransportKind::Simulated(transport) => {
                let transport = transport.read().await;
                let packet = transport.receive_next().await?;
                Ok(Some(Message::new(
                    packet.sender_id,
                    "me".to_string(), // In reality would be our ID
                    String::from_utf8_lossy(&packet.payload).to_string(),
                )))
            }
        }
    }
}

// Registry for all available transports (built-in and plugins)
pub struct TransportRegistry {
    transports: HashMap<String, TransportKind>,
}

impl TransportRegistry {
    pub fn new() -> Self {
        Self {
            transports: HashMap::new(),
        }
    }

    /// Register a transport (built-in or plugin)
    pub fn register(&mut self, transport: TransportKind) {
        self.transports
            .insert(transport.name().to_string(), transport);
    }

    /// Get a transport by name
    pub fn get(&self, name: &str) -> Option<&TransportKind> {
        self.transports.get(name)
    }

    /// List all registered transports
    pub fn list(&self) -> Vec<String> {
        self.transports.keys().cloned().collect()
    }
}

// Feature flag scaffolding for modular transports
// Example usage (in main or core):
// #[cfg(feature = "mesh-transport")]
// registry.register(MeshTransport::new(...));
// #[cfg(feature = "stealth-tcp-transport")]
// registry.register(StealthTCPProvider::new(...));

// Future: refactor transport implementations to use this registry and trait interface.
