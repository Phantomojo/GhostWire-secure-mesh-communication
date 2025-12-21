use async_trait::async_trait;
use crate::core::message::Message;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Transport Enum for async dispatch
#[derive(Clone)]
pub enum TransportKind {
    #[cfg(feature = "mesh")]
    Mesh(Arc<tokio::sync::RwLock<crate::core::MeshManager>>),
    Dummy, // Placeholder for now
}

impl TransportKind {
    pub fn name(&self) -> &'static str {
        match self {
            #[cfg(feature = "mesh")]
            TransportKind::Mesh(_) => "mesh",
            TransportKind::Dummy => "dummy",
        }
    }
    pub fn description(&self) -> &'static str {
        match self {
            #[cfg(feature = "mesh")]
            TransportKind::Mesh(_) => "Libp2p-based Mesh Network",
            TransportKind::Dummy => "Dummy transport (placeholder)",
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
        }
    }
    pub async fn receive_message(&self) -> Result<Option<Message>> {
        match self {
            #[cfg(feature = "mesh")]
            TransportKind::Mesh(_) => Ok(None), // Gossipsub handles receive via events
            TransportKind::Dummy => Ok(None),
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
        self.transports.insert(transport.name().to_string(), transport);
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

// TODO: Refactor all transport implementations to use this registry and trait interface. 