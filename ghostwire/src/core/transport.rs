use crate::core::message::Message;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;

#[derive(Clone)]
pub enum TransportKind {
    #[cfg(feature = "mesh")]
    Mesh(Arc<tokio::sync::RwLock<crate::core::MeshManager>>),
    Dummy,
    Quic(Arc<crate::core::quic_transport::QuicTransportProvider>),
    StealthTCP(Arc<crate::core::stealth_tcp::StealthTCPProvider>),
    #[cfg(feature = "bluetooth")]
    Bluetooth(Arc<tokio::sync::RwLock<crate::core::bluetooth::BluetoothTransport>>),
    Simulated(Arc<tokio::sync::RwLock<crate::core::sim_transport::SimTransport>>),
    #[cfg(feature = "reticulum")]
    Reticulum(Arc<tokio::sync::RwLock<crate::core::reticulum::ReticulumTransport>>),
}

impl TransportKind {
    pub fn name(&self) -> &'static str {
        match self {
            #[cfg(feature = "mesh")]
            TransportKind::Mesh(_) => "mesh",
            TransportKind::Dummy => "dummy",
            TransportKind::Quic(_) => "quic",
            TransportKind::StealthTCP(_) => "stealth_tcp",
            #[cfg(feature = "bluetooth")]
            TransportKind::Bluetooth(_) => "bluetooth",
            TransportKind::Simulated(_) => "simulated",
            #[cfg(feature = "reticulum")]
            TransportKind::Reticulum(_) => "reticulum",
        }
    }
    pub fn description(&self) -> &'static str {
        match self {
            #[cfg(feature = "mesh")]
            TransportKind::Mesh(_) => "Libp2p-based Mesh Network",
            TransportKind::Dummy => "Dummy transport (no-op)",
            TransportKind::Quic(_) => "QUIC Transport (Direct Internet)",
            TransportKind::StealthTCP(_) => "Stealth TCP with Traffic Morphing",
            #[cfg(feature = "bluetooth")]
            TransportKind::Bluetooth(_) => "Bluetooth Low Energy Mesh",
            TransportKind::Simulated(_) => "Simulated Hardware (BT/WiFi)",
            #[cfg(feature = "reticulum")]
            TransportKind::Reticulum(_) => "Reticulum RNS Network",
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
                tracing::trace!("Dummy transport sending message to {}", message.recipient);
                Ok(())
            }
            TransportKind::StealthTCP(provider) => {
                provider.send_message(message).await
            }
            TransportKind::Quic(provider) => {
                provider.send_message(message).await
            }
            #[cfg(feature = "bluetooth")]
            TransportKind::Bluetooth(transport) => {
                let transport = transport.read().await;
                transport
                    .send_ble_data(&message.recipient, message.content.as_bytes())
                    .await
            }
            TransportKind::Simulated(transport) => {
                let transport = transport.read().await;
                transport
                    .send_simulated(
                        &message.recipient,
                        crate::core::sim_transport::SimLinkType::Bluetooth,
                        message.content.as_bytes(),
                    )
                    .await
            }
            #[cfg(feature = "reticulum")]
            TransportKind::Reticulum(transport) => {
                let mut transport = transport.write().await;
                transport.send_message(message).await
            }
        }
    }
    pub async fn receive_message(&self) -> Result<Option<Message>> {
        match self {
            #[cfg(feature = "mesh")]
            TransportKind::Mesh(_) => Ok(None), // Gossipsub handles receive via events
            TransportKind::Dummy => Ok(None),
            TransportKind::StealthTCP(_) => Ok(None),
            TransportKind::Quic(_) => Ok(None),
            #[cfg(feature = "bluetooth")]
            TransportKind::Bluetooth(_) => Ok(None),
            TransportKind::Simulated(transport) => {
                let transport = transport.read().await;
                let packet = transport.receive_next().await?;
                Ok(Some(Message::new(
                    packet.sender_id,
                    "me".to_string(),
                    String::from_utf8_lossy(&packet.payload).to_string(),
                )))
            }
            #[cfg(feature = "reticulum")]
            TransportKind::Reticulum(transport) => {
                let mut transport = transport.write().await;
                transport.receive_message().await
            }
        }
    }
}

// Registry for all available transports (built-in and plugins)
pub struct TransportRegistry {
    transports: HashMap<String, TransportKind>,
}

impl Default for TransportRegistry {
    fn default() -> Self {
        Self::new()
    }
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
