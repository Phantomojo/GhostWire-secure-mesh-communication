use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum CallState {
    Idle,
    Inviting,
    Ringing,
    Connected,
    Disconnected,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallInvite {
    pub call_id: Uuid,
    pub caller: String,
    pub callee: String,
    pub media_type: MediaType,
    pub ice_ufrag: String,
    pub ice_pwd: String,
    pub sdp_offer: Option<String>,
    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallAccept {
    pub call_id: Uuid,
    pub ice_ufrag: String,
    pub ice_pwd: String,
    pub sdp_answer: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallReject {
    pub call_id: Uuid,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallEnd {
    pub call_id: Uuid,
    pub duration_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IceCandidate {
    pub call_id: Uuid,
    pub candidate: String,
    pub sdp_mid: String,
    pub sdp_m_line_index: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum MediaType {
    Audio,
    Video,
    AudioVideo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum CallSignal {
    Invite(CallInvite),
    Accept(CallAccept),
    Reject(CallReject),
    End(CallEnd),
    IceCandidate(IceCandidate),
}

/// Wrapper that enforces call signals are encrypted before transmission.
/// Contains serialized + encrypted payload with original signal type for routing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptedCallSignal {
    pub signal_type: String,
    pub ciphertext: Vec<u8>,
    pub nonce: [u8; 24],
}

impl EncryptedCallSignal {
    pub fn encrypt(signal: &CallSignal, key: &[u8; 32]) -> Result<Self, anyhow::Error> {
        use chacha20poly1305::aead::Aead;
        use chacha20poly1305::{ChaCha20Poly1305, KeyInit, Nonce};
        use rand::RngCore;

        let plaintext = bincode::serialize(signal)
            .map_err(|e| anyhow::anyhow!("Failed to serialize call signal: {}", e))?;

        let cipher = ChaCha20Poly1305::new(key.into());
        let mut nonce_bytes = [0u8; 24];
        rand::rng().fill_bytes(&mut nonce_bytes);
        let nonce = Nonce::from_slice(&nonce_bytes);

        let ciphertext = cipher
            .encrypt(nonce, plaintext.as_ref())
            .map_err(|e| anyhow::anyhow!("Failed to encrypt call signal: {}", e))?;

        let signal_type = match signal {
            CallSignal::Invite(_) => "Invite",
            CallSignal::Accept(_) => "Accept",
            CallSignal::Reject(_) => "Reject",
            CallSignal::End(_) => "End",
            CallSignal::IceCandidate(_) => "IceCandidate",
        }
        .to_string();

        Ok(Self {
            signal_type,
            ciphertext,
            nonce: nonce_bytes,
        })
    }

    pub fn decrypt(&self, key: &[u8; 32]) -> Result<CallSignal, anyhow::Error> {
        use chacha20poly1305::aead::Aead;
        use chacha20poly1305::{ChaCha20Poly1305, KeyInit, Nonce};

        let cipher = ChaCha20Poly1305::new(key.into());
        let nonce = Nonce::from_slice(&self.nonce);

        let plaintext = cipher
            .decrypt(nonce, self.ciphertext.as_ref())
            .map_err(|e| anyhow::anyhow!("Failed to decrypt call signal: {}", e))?;

        bincode::deserialize(&plaintext)
            .map_err(|e| anyhow::anyhow!("Failed to deserialize call signal: {}", e))
    }
}

pub struct ActiveCall {
    pub call_id: Uuid,
    pub peer: String,
    pub media_type: MediaType,
    pub state: CallState,
    pub local_ufrag: String,
    pub local_pwd: String,
    pub started_at: u64,
}

impl ActiveCall {
    pub fn new_invite(
        caller: String,
        callee: String,
        media_type: MediaType,
    ) -> (Self, CallInvite) {
        let call_id = Uuid::new_v4();
        let local_ufrag = generate_ufrag();
        let local_pwd = generate_pwd();

        let invite = CallInvite {
            call_id,
            caller: caller.clone(),
            callee: callee.clone(),
            media_type: media_type.clone(),
            ice_ufrag: local_ufrag.clone(),
            ice_pwd: local_pwd.clone(),
            sdp_offer: None,
            timestamp: now_secs(),
        };

        let call = Self {
            call_id,
            peer: callee,
            media_type,
            state: CallState::Inviting,
            local_ufrag,
            local_pwd,
            started_at: now_secs(),
        };

        (call, invite)
    }

    pub fn from_invite(invite: &CallInvite, local_ufrag: String, local_pwd: String) -> Self {
        Self {
            call_id: invite.call_id,
            peer: invite.caller.clone(),
            media_type: invite.media_type.clone(),
            state: CallState::Ringing,
            local_ufrag,
            local_pwd,
            started_at: invite.timestamp,
        }
    }

    pub fn accept(&mut self) -> CallAccept {
        self.state = CallState::Connected;
        CallAccept {
            call_id: self.call_id,
            ice_ufrag: self.local_ufrag.clone(),
            ice_pwd: self.local_pwd.clone(),
            sdp_answer: None,
        }
    }

    pub fn reject(reason: String, call_id: Uuid) -> CallReject {
        CallReject { call_id, reason }
    }

    pub fn end(&mut self) -> CallEnd {
        let duration = now_secs().saturating_sub(self.started_at);
        self.state = CallState::Disconnected;
        CallEnd {
            call_id: self.call_id,
            duration_secs: duration,
        }
    }
}

fn generate_ufrag() -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(4);
    for b in rand::random::<[u8; 4]>() {
        let _ = write!(s, "{:02x}", b);
    }
    s
}

fn generate_pwd() -> String {
    use std::fmt::Write;
    let mut s = String::with_capacity(24);
    for b in rand::random::<[u8; 16]>() {
        let _ = write!(s, "{:02x}", b);
    }
    s
}

fn now_secs() -> u64 {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

pub fn generate_sdp_offer(
    ufrag: &str,
    pwd: &str,
    media_type: &MediaType,
    ice_candidates: &[String],
) -> String {
    let media_line = match media_type {
        MediaType::Audio => "audio",
        MediaType::Video => "video",
        MediaType::AudioVideo => "audio",
    };

    let mut sdp = format!(
        "v=0\r\n\
         o=- 0 0 IN IP4 127.0.0.1\r\n\
         s=GhostWire Call\r\n\
         t=0 0\r\n\
         m={} 9 UDP/TLS/RTP/SAVPF 111\r\n\
         c=IN IP4 0.0.0.0\r\n\
         a=ice-ufrag:{}\r\n\
         a=ice-pwd:{}\r\n\
         a=rtcp-mux\r\n\
         a=rtpmap:111 opus/48000/2\r\n",
        media_line, ufrag, pwd
    );

    for candidate in ice_candidates {
        sdp.push_str(&format!("a=candidate:{}\r\n", candidate));
    }

    sdp
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_lifecycle() {
        let (mut caller, invite) =
            ActiveCall::new_invite("alice".into(), "bob".into(), MediaType::Audio);

        assert_eq!(caller.state, CallState::Inviting);
        assert_eq!(invite.caller, "alice");
        assert_eq!(invite.callee, "bob");

        let mut callee = ActiveCall::from_invite(&invite, "ufrag_b".into(), "pwd_b".into());
        assert_eq!(callee.state, CallState::Ringing);

        let accept = callee.accept();
        assert_eq!(callee.state, CallState::Connected);
        assert_eq!(accept.call_id, invite.call_id);

        caller.state = CallState::Connected;
        let end = caller.end();
        assert_eq!(caller.state, CallState::Disconnected);
        assert_eq!(end.call_id, invite.call_id);
    }

    #[test]
    fn test_reject() {
        let reject = ActiveCall::reject("busy".into(), Uuid::new_v4());
        assert_eq!(reject.reason, "busy");
    }

    #[test]
    fn test_sdp_offer() {
        let sdp = generate_sdp_offer("aabb", "ccdd", &MediaType::Audio, &[]);
        assert!(sdp.contains("a=ice-ufrag:aabb"));
        assert!(sdp.contains("m=audio"));
    }

    #[test]
    fn test_media_types() {
        for mt in [MediaType::Audio, MediaType::Video, MediaType::AudioVideo] {
            let (call, _) =
                ActiveCall::new_invite("a".into(), "b".into(), mt.clone());
            assert_eq!(call.media_type, mt);
        }
    }
}
