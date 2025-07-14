use async_trait::async_trait;
use crate::core::message::Message;
use anyhow::Result;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[async_trait]
pub trait Transport: Send + Sync {
    /// Unique name/id for the transport (e.g., "mesh", "stealth_tcp", "briar")
    fn name(&self) -> &'static str;
    /// Human-readable description
    fn description(&self) -> &'static str { "" }
    /// Feature flag (for conditional compilation)
    fn feature_flag(&self) -> Option<&'static str> { None }
    /// Send a message
    async fn send_message(&mut self, message: &Message) -> Result<()>;
    /// Receive a message
    async fn receive_message(&self) -> Result<Option<Message>>;
}

/// Registry for all available transports (built-in and plugins)
pub struct TransportRegistry {
    transports: HashMap<String, Arc<tokio::sync::Mutex<dyn Transport>>>,
}

impl TransportRegistry {
    pub fn new() -> Self {
        Self {
            transports: HashMap::new(),
        }
    }

    /// Register a transport (built-in or plugin)
    pub fn register<T: Transport + 'static>(&mut self, transport: T) {
        self.transports.insert(transport.name().to_string(), Arc::new(tokio::sync::Mutex::new(transport)));
    }

    /// Get a transport by name
    pub fn get(&self, name: &str) -> Option<Arc<tokio::sync::Mutex<dyn Transport>>> {
        self.transports.get(name).cloned()
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