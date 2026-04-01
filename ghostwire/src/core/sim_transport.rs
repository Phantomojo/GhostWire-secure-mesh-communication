use anyhow::Result;

use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tracing::{debug, info};

#[derive(Debug, Serialize, Deserialize)]
pub enum SimLinkType {
    Bluetooth,
    WiFiDirect,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SimPacket {
    pub link_type: SimLinkType,
    pub sender_id: String,
    pub payload: Vec<u8>,
}

#[derive(Debug)]
pub struct SimTransport {
    socket: UdpSocket,
    node_id: String,
    // Map of PeerID -> IP for the simulation environment
    peer_map: std::collections::HashMap<String, SocketAddr>,
}

impl SimTransport {
    pub async fn new(node_id: String, listen_addr: &str) -> Result<Self> {
        let socket = UdpSocket::bind(listen_addr).await?;
        info!("🚀 SimTransport started for {} on {}", node_id, listen_addr);

        Ok(Self {
            socket,
            node_id,
            peer_map: std::collections::HashMap::new(),
        })
    }

    pub fn register_peer(&mut self, peer_id: String, addr: SocketAddr) {
        self.peer_map.insert(peer_id, addr);
    }

    pub async fn send_simulated(
        &self,
        recipient_id: &str,
        link_type: SimLinkType,
        payload: &[u8],
    ) -> Result<()> {
        if let Some(addr) = self.peer_map.get(recipient_id) {
            let packet = SimPacket {
                link_type,
                sender_id: self.node_id.clone(),
                payload: payload.to_vec(),
            };
            let encoded = serde_json::to_vec(&packet)?;
            self.socket.send_to(&encoded, addr).await?;
            debug!(
                "Sent simulated {:?} packet to {}",
                packet.link_type, recipient_id
            );
            Ok(())
        } else {
            Err(anyhow::anyhow!(
                "Peer {} not registered in SimTransport",
                recipient_id
            ))
        }
    }

    pub async fn receive_next(&self) -> Result<SimPacket> {
        let mut buf = vec![0u8; 65535];
        let (len, _addr) = self.socket.recv_from(&mut buf).await?;
        let packet: SimPacket = serde_json::from_slice(&buf[..len])?;
        debug!(
            "Received simulated {:?} packet from {}",
            packet.link_type, packet.sender_id
        );
        Ok(packet)
    }
}
