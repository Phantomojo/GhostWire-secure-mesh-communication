// Added message sending and receiving support

use libp2p::{
    development_transport,
    identity,
    PeerId,
    swarm::{Swarm, SwarmEvent, NetworkBehaviourEventProcess},
    mdns::{Mdns, MdnsEvent},
    NetworkBehaviour,
    futures::StreamExt,
    request_response::{
        ProtocolName, RequestId, RequestResponse, RequestResponseCodec, RequestResponseConfig,
        RequestResponseEvent, RequestResponseMessage,
    },
};
use anyhow::Result;
use tokio::sync::mpsc;
use std::{error::Error, iter};
use log::{info, error};
use bytes::Bytes;
use async_trait::async_trait;
use std::io;

#[derive(Debug, Clone)]
struct GhostWireProtocol();

impl ProtocolName for GhostWireProtocol {
    fn protocol_name(&self) -> &[u8] {
        b"/ghostwire/1.0.0"
    }
}

#[derive(Clone)]
struct GhostWireCodec();

#[async_trait]
impl RequestResponseCodec for GhostWireCodec {
    type Protocol = GhostWireProtocol;
    type Request = Bytes;
    type Response = Bytes;

    async fn read_request<T>(&mut self, _: &GhostWireProtocol, io: &mut T) -> io::Result<Self::Request>
    where
        T: futures::AsyncRead + Unpin + Send,
    {
        let mut buf = Vec::new();
        futures::io::AsyncReadExt::read_to_end(io, &mut buf).await?;
        Ok(Bytes::from(buf))
    }

    async fn read_response<T>(&mut self, _: &GhostWireProtocol, io: &mut T) -> io::Result<Self::Response>
    where
        T: futures::AsyncRead + Unpin + Send,
    {
        let mut buf = Vec::new();
        futures::io::AsyncReadExt::read_to_end(io, &mut buf).await?;
        Ok(Bytes::from(buf))
    }

    async fn write_request<T>(&mut self, _: &GhostWireProtocol, io: &mut T, data: Self::Request) -> io::Result<()>
    where
        T: futures::AsyncWrite + Unpin + Send,
    {
        futures::io::AsyncWriteExt::write_all(io, &data).await?;
        futures::io::AsyncWriteExt::close(io).await?;
        Ok(())
    }

    async fn write_response<T>(&mut self, _: &GhostWireProtocol, io: &mut T, data: Self::Response) -> io::Result<()>
    where
        T: futures::AsyncWrite + Unpin + Send,
    {
        futures::io::AsyncWriteExt::write_all(io, &data).await?;
        futures::io::AsyncWriteExt::close(io).await?;
        Ok(())
    }
}

#[derive(NetworkBehaviour)]
#[behaviour(event_process = true)]
struct GhostWireBehaviour {
    mdns: Mdns,
    request_response: RequestResponse<GhostWireCodec>,
}

impl GhostWireBehaviour {
    async fn new() -> Result<Self> {
        let protocols = iter::once((GhostWireProtocol(), libp2p::request_response::ProtocolSupport::Full));
        let cfg = RequestResponseConfig::default();
        let request_response = RequestResponse::new(GhostWireCodec(), protocols, cfg);

        Ok(Self {
            mdns: Mdns::new(Default::default()).await?,
            request_response,
        })
    }
}

pub struct Networking {
    peer_id: PeerId,
    swarm: Swarm<GhostWireBehaviour>,
    event_sender: mpsc::UnboundedSender<SwarmEvent<(), ()>>,
}

impl Networking {
    pub async fn new(event_sender: mpsc::UnboundedSender<SwarmEvent<(), ()>>) -> Result<Self> {
        // Create a random key for ourselves.
        let local_key = identity::Keypair::generate_ed25519();
        let peer_id = PeerId::from(local_key.public());
        info!("Local peer id: {:?}", peer_id);

        // Set up an encrypted DNS-enabled TCP transport over the Mplex protocol.
        let transport = development_transport(local_key.clone()).await?;

        // Create a Swarm to manage peers and events.
        let behaviour = GhostWireBehaviour::new().await?;

        let mut swarm = Swarm::new(transport, behaviour, peer_id);

        // Listen on all interfaces and a random, OS-assigned port.
        swarm.listen_on("/ip4/0.0.0.0/tcp/0".parse()?)?;

        Ok(Self {
            peer_id,
            swarm,
            event_sender,
        })
    }

    pub async fn start(mut self) -> Result<()> {
        loop {
            match self.swarm.next().await {
                Some(event) => {
                    info!("Swarm event: {:?}", event);
                    // Convert event to expected type or handle error
                    let send_result = self.event_sender.send(event.map_err(|e| e.into()));
                    if let Err(e) = send_result {
                        error!("Failed to send swarm event: {:?}", e);
                    }
                }
                None => break,
            }
        }
        Ok(())
    }

    pub fn peer_id(&self) -> PeerId {
        self.peer_id
    }

    pub async fn send_message(&mut self, peer: &PeerId, data: Vec<u8>) -> Result<RequestId> {
        Ok(self.swarm.behaviour_mut().request_response.send_request(peer, data.into()))
    }

    pub async fn send_response(&mut self, channel: libp2p::request_response::ResponseChannel<Bytes>, response: Bytes) -> Result<()> {
        self.swarm.behaviour_mut().request_response.send_response(channel, response).map_err(|e| anyhow::anyhow!(e))?;
        Ok(())
    }
}

impl NetworkBehaviourEventProcess<MdnsEvent> for GhostWireBehaviour {
    fn inject_event(&mut self, event: MdnsEvent) {
        match event {
            MdnsEvent::Discovered(list) => {
                for (peer_id, multiaddr) in list {
                    info!("Discovered peer {} at {}", peer_id, multiaddr);
                    self.request_response.add_address(&peer_id, multiaddr);
                }
            }
            MdnsEvent::Expired(list) => {
                for (peer_id, multiaddr) in list {
                    info!("Expired peer {} at {}", peer_id, multiaddr);
                    self.request_response.remove_address(&peer_id, &multiaddr);
                }
            }
        }
    }
}

impl NetworkBehaviourEventProcess<RequestResponseEvent<Bytes, Bytes>> for GhostWireBehaviour {
    fn inject_event(&mut self, event: RequestResponseEvent<Bytes, Bytes>) {
        match event {
            RequestResponseEvent::Message { peer, message } => {
                match message {
                    RequestResponseMessage::Request { request, channel, .. } => {
                        info!("Received request from {}: {:?}", peer, request);
                        // Echo back the request as response for now
                        self.request_response.send_response(channel, request).unwrap();
                    }
                    RequestResponseMessage::Response { response, .. } => {
                        info!("Received response from {}: {:?}", peer, response);
                    }
                }
            }
            RequestResponseEvent::OutboundFailure { peer, error, request_id } => {
                error!("Outbound failure to {}: {:?} (request id: {:?})", peer, error, request_id);
            }
            RequestResponseEvent::InboundFailure { peer, error, request_id } => {
                error!("Inbound failure from {}: {:?} (request id: {:?})", peer, error, request_id);
            }
            RequestResponseEvent::ResponseSent { peer, request_id } => {
                info!("Response sent to {} (request id: {:?})", peer, request_id);
            }
        }
    }
}
