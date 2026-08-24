use crate::core::calls::{CallId, CallSignal, MediaState, ICECandidate, ICECandidateGathered, CallState, CallEndReason};
use crate::core::crypto_machine::Machine;
use anyhow::{Result, anyhow};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{mpsc, RwLock};
use tracing::{debug, error, info, warn};
use webrtc::api::{MediaEngine, Registry};
use webrtc::peer_connection::peer_connection_state::RTCPeerConnectionState;
use webrtc::peer_connection::sdp::session_description::RTCSessionDescription;
use webrtc::peer_connection::{PeerConnection, PeerConnectionBuilder, PeerConnectionEventHandler, RTCConfigurationBuilder, RTCIceServer, register_default_interceptors};
use webrtc::peer_connection::ice_transport::ice_candidate::RTCIceCandidateInit;
use webrtc::media_stream::track_local_static_rtp::TrackLocalStaticRTP;
use webrtc::media_stream::track_local_static_sample::{TrackLocalStaticSample, SampleWriter};
use webrtc::media_stream::track_remote::{TrackRemote, TrackRemoteEvent};
use webrtc::rtp_transceiver::rtp_sender::RtpCodecKind;
use webrtc::rtp_transceiver::RTCRtpTransceiverInit;
use webrtc::rtp_transceiver::RTCRtpTransceiverDirection;
use webrtc::media_stream::Sample;

pub struct MediaSession {
    pub call_id: CallId,
    pub peer_connection: Arc<PeerConnection>,
    pub state: CallState,
    pub media_state: MediaState,
    pub local_video_track: Option<Arc<TrackLocalStaticSample>>,
    pub local_audio_track: Option<Arc<TrackLocalStaticSample>>,
}

pub struct MediaHandler {
    sessions: HashMap<CallId, MediaSession>,
    signal_tx: mpsc::Sender<CallSignal>,
    signal_rx: Arc<RwLock<mpsc::Receiver<CallSignal>>>,
    machine: Arc<Machine>,
}

#[derive(Debug, Clone)]
pub enum MediaError {
    PeerConnectionFailed(String),
    ICEFailed(String),
    SDPFailed(String),
    MediaTrackFailed(String),
    SessionNotFound(CallId),
}

impl std::fmt::Display for MediaError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MediaError::PeerConnectionFailed(msg) => write!(f, "PeerConnection failed: {}", msg),
            MediaError::ICEFailed(msg) => write!(f, "ICE failed: {}", msg),
            MediaError::SDPFailed(msg) => write!(f, "SDP failed: {}", msg),
            MediaError::MediaTrackFailed(msg) => write!(f, "Media track failed: {}", msg),
            MediaError::SessionNotFound(call_id) => write!(f, "Session not found for call {}", call_id),
        }
    }
}

impl std::error::Error for MediaError {}

struct MediaHandlerEventHandler {
    call_id: CallId,
    signal_tx: mpsc::Sender<CallSignal>,
    ice_candidates: Arc<RwLock<Vec<ICECandidate>>>,
}

#[async_trait::async_trait]
impl PeerConnectionEventHandler for MediaHandlerEventHandler {
    async fn on_peer_connection_state_change(&self, state: RTCPeerConnectionState) {
        debug!("PeerConnection state changed for call {}: {:?}", self.call_id, state);
        match state {
            RTCPeerConnectionState::Connected => {
                info!("WebRTC connection established for call {}", self.call_id);
            }
            RTCPeerConnectionState::Disconnected | RTCPeerConnectionState::Failed => {
                warn!("WebRTC connection lost for call {}", self.call_id);
            }
            _ => {}
        }
    }

    async fn on_ice_candidate(&self, candidate: Option<webrtc::peer_connection::ice_transport::ice_candidate::RTCIceCandidate>) {
        if let Some(ice_candidate) = candidate {
            let candidate_str = ice_candidate.to_string();
            let candidate_init = RTCIceCandidateInit {
                candidate: candidate_str.clone(),
                ..Default::default()
            };
            
            let signal = CallSignal::ICECandidate(ICECandidate {
                call_id: self.call_id,
                candidate: candidate_str,
            });
            
            if let Err(e) = self.signal_tx.send(signal).await {
                error!("Failed to send ICE candidate: {}", e);
            }
        }
    }
}

impl MediaHandler {
    pub fn new(machine: Arc<Machine>) -> Self {
        let (signal_tx, signal_rx) = mpsc::channel(100);
        
        MediaHandler {
            sessions: HashMap::new(),
            signal_tx,
            signal_rx: Arc::new(RwLock::new(signal_rx)),
            machine,
        }
    }

    pub async fn create_session(&mut self, call_id: CallId) -> Result<Arc<PeerConnection>, MediaError> {
        if self.sessions.contains_key(&call_id) {
            return Err(MediaError::SessionNotFound(call_id));
        }

        let mut media_engine = MediaEngine::default();
        media_engine.register_default_codecs().map_err(|e| MediaError::PeerConnectionFailed(e.to_string()))?;

        let registry = register_default_interceptors(Registry::new(), &mut media_engine)
            .map_err(|e| MediaError::PeerConnectionFailed(e.to_string()))?;

        let config = RTCConfigurationBuilder::new()
            .with_ice_servers(vec![RTCIceServer {
                urls: vec!["stun:stun.l.google.com:19302".to_string()],
                ..Default::default()
            }])
            .build();

        let event_handler = Arc::new(MediaHandlerEventHandler {
            call_id,
            signal_tx: self.signal_tx.clone(),
            ice_candidates: Arc::new(RwLock::new(Vec::new())),
        });

        let runtime = webrtc::runtime::default_runtime()
            .ok_or_else(|| MediaError::PeerConnectionFailed("No async runtime found".to_string()))?;

        let peer_connection = PeerConnectionBuilder::new()
            .with_configuration(config)
            .with_media_engine(media_engine)
            .with_interceptor_registry(registry)
            .with_handler(event_handler)
            .with_runtime(runtime)
            .with_udp_addrs(vec!["0.0.0.0:0".to_string()])
            .build()
            .await
            .map_err(|e| MediaError::PeerConnectionFailed(e.to_string()))?;

        let audio_track = TrackLocalStaticSample::new(
            webrtc::media_stream::track_local_static_sample::RTPCodecCapability::default(),
            "audio".to_string(),
            "ghostwire-audio".to_string(),
        );

        let video_track = TrackLocalStaticSample::new(
            webrtc::media_stream::track_local_static_sample::RTPCodecCapability::default(),
            "video".to_string(),
            "ghostwire-video".to_string(),
        );

        peer_connection
            .add_transceiver_from_track(
                audio_track.clone(),
                Some(RTCRtpTransceiverInit {
                    direction: RTCRtpTransceiverDirection::Sendonly,
                    ..Default::default()
                }),
            )
            .await
            .map_err(|e| MediaError::MediaTrackFailed(e.to_string()))?;

        peer_connection
            .add_transceiver_from_track(
                video_track.clone(),
                Some(RTCRtpTransceiverInit {
                    direction: RTCRtpTransceiverDirection::Sendonly,
                    ..Default::default()
                }),
            )
            .await
            .map_err(|e| MediaError::MediaTrackFailed(e.to_string()))?;

        peer_connection
            .add_transceiver_from_kind(
                RtpCodecKind::Audio,
                Some(RTCRtpTransceiverInit {
                    direction: RTCRtpTransceiverDirection::Recvonly,
                    ..Default::default()
                }),
            )
            .await
            .map_err(|e| MediaError::MediaTrackFailed(e.to_string()))?;

        peer_connection
            .add_transceiver_from_kind(
                RtpCodecKind::Video,
                Some(RTCRtpTransceiverInit {
                    direction: RTCRtpTransceiverDirection::Recvonly,
                    ..Default::default()
                }),
            )
            .await
            .map_err(|e| MediaError::MediaTrackFailed(e.to_string()))?;

        let session = MediaSession {
            call_id,
            peer_connection: peer_connection.clone(),
            state: CallState::Connecting,
            media_state: MediaState::Media,
            local_video_track: Some(video_track),
            local_audio_track: Some(audio_track),
        };

        self.sessions.insert(call_id, session);

        Ok(peer_connection)
    }

    pub async fn handle_sdp(&mut self, call_id: &CallId, sdp: &str, sdp_type: &str) -> Result<(), MediaError> {
        let session = self.sessions.get(call_id)
            .ok_or_else(|| MediaError::SessionNotFound(*call_id))?;

        let session_sdp = if sdp_type == "offer" {
            RTCSessionDescription::offer(sdp.to_string())
        } else {
            RTCSessionDescription::answer(sdp.to_string())
        };

        session.peer_connection
            .set_remote_description(session_sdp)
            .await
            .map_err(|e| MediaError::SDPFailed(e.to_string()))?;

        if sdp_type == "offer" {
            let answer = session.peer_connection
                .create_answer(None)
                .await
                .map_err(|e| MediaError::SDPFailed(e.to_string()))?;

            session.peer_connection
                .set_local_description(answer.clone())
                .await
                .map_err(|e| MediaError::SDPFailed(e.to_string()))?;
        }

        Ok(())
    }

    pub async fn handle_ice_candidate(&mut self, call_id: &CallId, candidate: &str) -> Result<(), MediaError> {
        let session = self.sessions.get(call_id)
            .ok_or_else(|| MediaError::SessionNotFound(*call_id))?;

        let ice_candidate = RTCIceCandidateInit {
            candidate: candidate.to_string(),
            ..Default::default()
        };

        session.peer_connection
            .add_ice_candidate(ice_candidate)
            .await
            .map_err(|e| MediaError::ICEFailed(e.to_string()))?;

        Ok(())
    }

    pub async fn send_audio(&self, call_id: &CallId, data: &[u8], duration: std::time::Duration) -> Result<(), MediaError> {
        let session = self.sessions.get(call_id)
            .ok_or_else(|| MediaError::SessionNotFound(*call_id))?;

        if let Some(track) = &session.local_audio_track {
            let sample = Sample {
                data: data.to_vec(),
                duration,
                ..Default::default()
            };

            track.write_sample(&sample).await
                .map_err(|e| MediaError::MediaTrackFailed(e.to_string()))?;
        }

        Ok(())
    }

    pub async fn send_video(&self, call_id: &CallId, data: &[u8], duration: std::time::Duration) -> Result<(), MediaError> {
        let session = self.sessions.get(call_id)
            .ok_or_else(|| MediaError::SessionNotFound(*call_id))?;

        if let Some(track) = &session.local_video_track {
            let sample = Sample {
                data: data.to_vec(),
                duration,
                ..Default::default()
            };

            track.write_sample(&sample).await
                .map_err(|e| MediaError::MediaTrackFailed(e.to_string()))?;
        }

        Ok(())
    }

    pub async fn end_session(&mut self, call_id: &CallId) -> Result<(), MediaError> {
        if let Some(mut session) = self.sessions.remove(call_id) {
            session.state = CallState::Ended(CallEndReason::Normal);
            session.media_state = MediaState::None;

            session.peer_connection.close().await
                .map_err(|e| MediaError::PeerConnectionFailed(e.to_string()))?;

            info!("Media session ended for call {}", call_id);
        }

        Ok(())
    }

    pub fn get_session(&self, call_id: &CallId) -> Option<&MediaSession> {
        self.sessions.get(call_id)
    }

    pub async fn listen_for_signals(&mut self) -> Option<CallSignal> {
        let mut rx = self.signal_rx.write().await;
        rx.recv().await
    }

    pub async fn handle_received_track(&self, call_id: &CallId, track: Arc<dyn TrackRemote>) -> Result<(), MediaError> {
        let session = self.sessions.get(call_id)
            .ok_or_else(|| MediaError::SessionNotFound(*call_id))?;

        let peer_connection = session.peer_connection.clone();
        let call_id = *call_id;

        tokio::spawn(async move {
            while let Some(event) = track.poll().await {
                match event {
                    TrackRemoteEvent::OnOpen(init) => {
                        debug!("Track opened for call {}: ssrc={} rid={:?}", call_id, init.ssrc, init.rid);
                    }
                    TrackRemoteEvent::OnRtpPacket(pkt) => {
                        debug!("Received RTP packet for call {}: ssrc={} seq={} pt={}", 
                            call_id, pkt.header.ssrc, pkt.header.sequence_number, pkt.header.payload_type);
                    }
                    TrackRemoteEvent::OnEnded => {
                        debug!("Track ended for call {}", call_id);
                        break;
                    }
                    _ => {}
                }
            }
        });

        Ok(())
    }

    pub async fn process_signal(&mut self, signal: CallSignal) -> Result<(), MediaError> {
        match signal {
            CallSignal::CallInitiated { call_id, media_state, .. } => {
                debug!("Creating media session for call {}", call_id);
                let _ = self.create_session(call_id).await?;
            }
            CallSignal::CallAccepted { call_id, .. } => {
                if let Some(session) = self.sessions.get_mut(&call_id) {
                    session.state = CallState::Connecting;
                    session.media_state = MediaState::Media;
                }
            }
            CallSignal::CallEnded { call_id, reason, .. } => {
                self.end_session(&call_id).await?;
            }
            CallSignal::SDP { call_id, sdp, sdp_type } => {
                self.handle_sdp(&call_id, &sdp, &sdp_type).await?;
            }
            CallSignal::ICECandidate { call_id, candidate } => {
                self.handle_ice_candidate(&call_id, &candidate).await?;
            }
            _ => {}
        }
        Ok(())
    }
}

impl Default for MediaHandler {
    fn default() -> Self {
        let machine = Arc::new(Machine::generate());
        Self::new(machine)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::crypto_machine::Machine;

    #[test]
    fn test_media_handler_creation() {
        let machine = Arc::new(Machine::generate());
        let handler = MediaHandler::new(machine);
        assert!(handler.sessions.is_empty());
    }

    #[tokio::test]
    async fn test_create_session() {
        let machine = Arc::new(Machine::generate());
        let mut handler = MediaHandler::new(machine);
        
        let call_id = CallId::generate();
        let result = handler.create_session(call_id).await;
        
        assert!(result.is_ok());
        assert!(handler.sessions.contains_key(&call_id));
    }

    #[tokio::test]
    async fn test_end_session() {
        let machine = Arc::new(Machine::generate());
        let mut handler = MediaHandler::new(machine);
        
        let call_id = CallId::generate();
        let _ = handler.create_session(call_id).await;
        
        let result = handler.end_session(&call_id).await;
        assert!(result.is_ok());
        assert!(!handler.sessions.contains_key(&call_id));
    }
}