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
            TransportKind::StealthTCP(_) => {
                "Stealth TCP — NOT SECURE. Message content is encoded as bare bincode \
                 with no AEAD and no MAC; there is no traffic morphing in this \
                 implementation. Do not use for confidential messages."
            }
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
            TransportKind::StealthTCP(provider) => provider.send_message(message).await,
            TransportKind::Quic(provider) => provider.send_message(message).await,
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
            TransportKind::StealthTCP(provider) => match provider.poll_receive().await {
                Some((_addr, msg)) => Ok(Some(msg)),
                None => Ok(None),
            },
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
#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct TransportDescriptor {
    pub name: String,
    pub description: String,
    pub registered: bool,
    /// Compile/runtime capability label; registration alone is not readiness.
    pub implementation: String,
    /// Authenticated actions currently supported by the backend for this transport.
    pub controls: Vec<String>,
}

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

    /// List all registered transports in deterministic order.
    pub fn list(&self) -> Vec<String> {
        let mut names: Vec<String> = self.transports.keys().cloned().collect();
        names.sort();
        names
    }

    pub fn is_registered(&self, name: &str) -> bool {
        self.transports.contains_key(name)
    }

    /// Remove a transport after its lifecycle has been stopped.
    pub fn unregister(&mut self, name: &str) -> Option<TransportKind> {
        self.transports.remove(name)
    }

    /// Return only registered transports understood by the selector.
    /// Media labels and placeholders are deliberately excluded.
    pub fn eligible_selector_transports(&self) -> Vec<crate::core::disaster::TransportType> {
        self.list()
            .into_iter()
            .filter_map(|name| match name.as_str() {
                "mesh" => Some(crate::core::disaster::TransportType::Mesh),
                "quic" => Some(crate::core::disaster::TransportType::Quic),
                "stealth_tcp" => Some(crate::core::disaster::TransportType::StealthTcp),
                "bluetooth" => Some(crate::core::disaster::TransportType::Bluetooth),
                "reticulum" => Some(crate::core::disaster::TransportType::Reticulum),
                _ => None,
            })
            .collect()
    }

    /// Return capability metadata without exposing transport internals.
    pub fn descriptors(&self) -> Vec<TransportDescriptor> {
        let mut descriptors: Vec<_> = self
            .transports
            .values()
            .map(|transport| {
                let name = transport.name().to_string();
                let (implementation, controls) = match name.as_str() {
                    "mesh" | "quic" => (
                        "compiled_and_registered".to_string(),
                        vec!["observe".to_string()],
                    ),
                    "reticulum" => (
                        "compiled_and_registered".to_string(),
                        vec![
                            "observe".to_string(),
                            "start".to_string(),
                            "stop".to_string(),
                        ],
                    ),
                    "bluetooth" => (
                        "compiled_and_registered_platform_gated".to_string(),
                        vec!["observe".to_string()],
                    ),
                    _ => (
                        "registered_placeholder".to_string(),
                        vec!["observe".to_string()],
                    ),
                };
                TransportDescriptor {
                    name,
                    description: transport.description().to_string(),
                    registered: true,
                    implementation,
                    controls,
                }
            })
            .collect();
        descriptors.sort_by(|a, b| a.name.cmp(&b.name));
        descriptors
    }
}

// Feature flag scaffolding for modular transports
// Example usage (in main or core):
// #[cfg(feature = "mesh-transport")]
// registry.register(MeshTransport::new(...));
// #[cfg(feature = "stealth-tcp-transport")]
// registry.register(StealthTCPProvider::new(...));

// Future: refactor transport implementations to use this registry and trait interface.

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn registry_lists_names_deterministically_and_reports_registration() {
        let mut registry = TransportRegistry::new();
        registry.register(TransportKind::Dummy);
        assert_eq!(registry.list(), vec!["dummy".to_string()]);
        assert!(registry.is_registered("dummy"));
        assert!(!registry.is_registered("quic"));
    }

    #[test]
    fn registry_descriptor_is_explicitly_registered() {
        let mut registry = TransportRegistry::new();
        registry.register(TransportKind::Dummy);
        let descriptors = registry.descriptors();
        assert_eq!(descriptors.len(), 1);
        assert_eq!(descriptors[0].name, "dummy");
        assert!(descriptors[0].registered);
        assert!(!descriptors[0].description.is_empty());
        assert_eq!(descriptors[0].implementation, "registered_placeholder");
        assert_eq!(descriptors[0].controls, vec!["observe".to_string()]);
    }
}
