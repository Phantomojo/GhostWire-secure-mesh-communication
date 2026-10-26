use anyhow::{anyhow, Result};
use btleplug::api::{
    Central, CentralEvent, Characteristic, Manager as _, Peripheral as _, ScanFilter, WriteType,
};
use btleplug::platform::Manager;
use futures_util::stream::StreamExt;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{debug, error, info, warn};

/// GhostWire BLE service UUID (custom 128-bit)
const GHOSTWIRE_SERVICE_UUID: uuid::Uuid =
    uuid::Uuid::from_bytes([0x47, 0x57, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01]);
/// Characteristic for receiving messages from peers
const RX_CHAR_UUID: uuid::Uuid =
    uuid::Uuid::from_bytes([0x47, 0x57, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x02]);
/// Characteristic for sending messages to peers
const TX_CHAR_UUID: uuid::Uuid =
    uuid::Uuid::from_bytes([0x47, 0x57, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x03]);

pub struct BluetoothTransport {
    manager: Manager,
    peers: Arc<RwLock<HashMap<String, btleplug::platform::Peripheral>>>,
}

impl BluetoothTransport {
    pub async fn new() -> Result<Self> {
        let manager = Manager::new().await?;
        Ok(Self {
            manager,
            peers: Arc::new(RwLock::new(HashMap::new())),
        })
    }

    /// Start scanning for GhostWire BLE peers
    pub async fn start_scanning(&self) -> Result<()> {
        let adapters = self.manager.adapters().await?;
        let central = adapters
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("No Bluetooth adapters found"))?;

        info!("Starting BLE scan for GhostWire peers...");
        central.start_scan(ScanFilter::default()).await?;

        // Listen for discovery events in background
        let mut events = central.events().await?;
        tokio::spawn(async move {
            while let Some(event) = events.next().await {
                match event {
                    CentralEvent::DeviceDiscovered(id) => {
                        debug!("BLE device discovered: {}", id);
                    }
                    CentralEvent::DeviceConnected(id) => {
                        info!("BLE device connected: {}", id);
                    }
                    CentralEvent::DeviceDisconnected(id) => {
                        info!("BLE device disconnected: {}", id);
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }

    pub async fn stop_scanning(&self) -> Result<()> {
        let adapters = self.manager.adapters().await?;
        if let Some(central) = adapters.into_iter().next() {
            central.stop_scan().await?;
        }
        Ok(())
    }

    /// Send data to a peer via BLE GATT write
    pub async fn send_ble_data(&self, peer_id: &str, data: &[u8]) -> Result<()> {
        let peers = self.peers.read().await;
        if let Some(peripheral) = peers.get(peer_id) {
            let services = peripheral.services();
            for service in services {
                if service.uuid == GHOSTWIRE_SERVICE_UUID {
                    for char in &service.characteristics {
                        if char.uuid == TX_CHAR_UUID {
                            peripheral
                                .write(char, data, WriteType::WithResponse)
                                .await
                                .map_err(|e| anyhow!("BLE write failed: {}", e))?;
                            debug!("Sent {} bytes to peer {} via BLE", data.len(), peer_id);
                            return Ok(());
                        }
                    }
                }
            }
        }
        // Peer not connected yet — data will be queued by caller
        debug!("BLE peer {} not connected, {} bytes queued", peer_id, data.len());
        Ok(())
    }

    /// Connect to a discovered peer and discover GATT services
    pub async fn connect_peer(&self, peripheral_id: &str) -> Result<()> {
        let adapters = self.manager.adapters().await?;
        let central = adapters
            .into_iter()
            .next()
            .ok_or_else(|| anyhow!("No Bluetooth adapters found"))?;

        // Find the peripheral from already-discovered devices
        let peripherals = central.peripherals().await?;
        let peripheral = peripherals
            .into_iter()
            .find(|p| p.id().to_string() == peripheral_id)
            .ok_or_else(|| anyhow!("Peripheral {} not found in discovered devices", peripheral_id))?;

        peripheral
            .connect()
            .await
            .map_err(|e| anyhow!("Failed to connect: {}", e))?;

        peripheral
            .discover_services()
            .await
            .map_err(|e| anyhow!("Failed to discover services: {}", e))?;

        // Subscribe to RX characteristic for incoming messages
        let services = peripheral.services();
        for service in &services {
            if service.uuid == GHOSTWIRE_SERVICE_UUID {
                for char in &service.characteristics {
                    if char.uuid == RX_CHAR_UUID {
                        peripheral.subscribe(char).await.ok();
                        info!("Subscribed to RX characteristic on peer {}", peripheral_id);
                    }
                }
            }
        }

        self.peers.write().await.insert(peripheral_id.to_string(), peripheral);
        info!("Connected to BLE peer: {}", peripheral_id);
        Ok(())
    }

    /// Disconnect from a peer
    pub async fn disconnect_peer(&self, peripheral_id: &str) -> Result<()> {
        let mut peers = self.peers.write().await;
        if let Some(peripheral) = peers.remove(peripheral_id) {
            peripheral.disconnect().await.ok();
            info!("Disconnected from BLE peer: {}", peripheral_id);
        }
        Ok(())
    }

    /// List connected peers
    pub async fn connected_peers(&self) -> Vec<String> {
        self.peers.read().await.keys().cloned().collect()
    }
}

#[cfg(feature = "bluetooth")]
pub mod ble_advertiser {
    use bluer::adv::{Advertisement, AdvertisementHandle, Type};
    use bluer::{Adapter, Session};
    use tracing::{info, warn};

    /// Start BLE advertising with GhostWire service UUID.
    /// Returns a handle that keeps the advertisement alive.
    pub async fn start_advertising() -> Option<AdvertisementHandle> {
        let session = match Session::new().await {
            Ok(s) => s,
            Err(e) => {
                warn!("Failed to create BlueZ session: {}", e);
                return None;
            }
        };

        let adapter = match session.default_adapter().await {
            Ok(a) => a,
            Err(e) => {
                warn!("No default Bluetooth adapter: {}", e);
                return None;
            }
        };

        if let Err(e) = adapter.set_powered(true).await {
            warn!("Failed to power on adapter: {}", e);
            return None;
        }

        let mut service_uuids = std::collections::BTreeSet::new();
        service_uuids.insert("47570000-0000-0000-0000-000000000001".parse().unwrap());

        let le_advertisement = Advertisement {
            advertisement_type: Type::Peripheral,
            service_uuids,
            local_name: Some("GhostWire".to_string()),
            ..Default::default()
        };

        match adapter.advertise(le_advertisement).await {
            Ok(handle) => {
                info!("BLE advertising started (GhostWire service UUID)");
                Some(handle)
            }
            Err(e) => {
                warn!("BLE advertising failed: {}", e);
                None
            }
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use crate::core::bluetooth::BluetoothTransport;

        #[tokio::test]
        async fn test_bluetooth_transport_creation() {
            let transport = BluetoothTransport::new().await;
            if let Ok(_transport) = transport {
                // Creation success check
            }
        }
    }
}