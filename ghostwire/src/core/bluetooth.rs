use anyhow::{anyhow, Result};
use btleplug::api::{Central, Manager as _, ScanFilter};
use btleplug::platform::Manager;
use tracing::{debug, info};

#[derive(Debug)]
pub struct BluetoothTransport {
    manager: Manager,
}

impl BluetoothTransport {
    pub async fn new() -> Result<Self> {
        let manager = Manager::new().await?;
        Ok(Self { manager })
    }

    pub async fn start_scanning(&self) -> Result<()> {
        let adapters = self.manager.adapters().await?;
        let central = adapters
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("No Bluetooth adapters found"))?;

        info!("Starting Bluetooth scan...");
        central.start_scan(ScanFilter::default()).await?;

        Ok(())
    }

    pub async fn stop_scanning(&self) -> Result<()> {
        // Implementation for stopping scan
        Ok(())
    }

    pub async fn send_ble_data(&self, _peer_id: &str, data: &[u8]) -> Result<()> {
        debug!("Sending {} bytes via Bluetooth (Skeleton)", data.len());
        // In real implementation, find peripheral by peer_id and write to characteristic
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_bluetooth_transport_creation() {
        let transport = BluetoothTransport::new().await;
        // This might fail if no bluetooth stack is available in test environment
        // but it shouldn't panic
        if let Ok(_transport) = transport {
            // Creation success check
        }
    }

    #[tokio::test]
    async fn test_send_ble_data_mock() {
        let transport = BluetoothTransport {
            manager: Manager::new().await.unwrap(),
        };
        assert!(transport.send_ble_data("peer1", b"hello").await.is_ok());
    }
}
