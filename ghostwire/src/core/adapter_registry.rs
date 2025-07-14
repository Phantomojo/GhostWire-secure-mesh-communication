use async_trait::async_trait;

/// Trait for protocol adapters (e.g., Meshtastic, Briar, Reticulum).
#[async_trait]
pub trait ProtocolAdapter: Send + Sync {
    /// Adapter name (e.g., "meshtastic").
    fn name(&self) -> &'static str;
    /// Send a message via the protocol.
    async fn send_message(&self, message: &Message) -> anyhow::Result<()>;
    /// Receive a message from the protocol.
    async fn receive_message(&self) -> anyhow::Result<Option<Message>>;
}

/// Dummy adapter for testing.
pub struct DummyAdapter;

#[async_trait]
impl ProtocolAdapter for DummyAdapter {
    fn name(&self) -> &'static str { "dummy" }
    async fn send_message(&self, _message: &Message) -> anyhow::Result<()> { Ok(()) }
    async fn receive_message(&self) -> anyhow::Result<Option<Message>> { Ok(None) }
}

/// Registry for protocol adapters.
pub struct AdapterRegistry {
    pub adapters: Vec<Box<dyn ProtocolAdapter>>,
}

impl AdapterRegistry {
    pub fn new() -> Self {
        let mut adapters: Vec<Box<dyn ProtocolAdapter>> = Vec::new();
        adapters.push(Box::new(DummyAdapter));
        // TODO: Register real adapters in next phase
        Self { adapters }
    }
    pub fn get(&self, name: &str) -> Option<&Box<dyn ProtocolAdapter>> {
        self.adapters.iter().find(|a| a.name() == name)
    }
}

/// Placeholder type for demo
pub struct Message; 