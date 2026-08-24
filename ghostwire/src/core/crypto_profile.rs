//! Authenticated transport-independent crypto profile negotiation primitives.
//!
//! This module deliberately does not encrypt application data yet. It provides
//! the bounded, canonical, downgrade-resistant negotiation layer that every
//! transport integration must use before selecting an envelope codec.

use serde::{Deserialize, Serialize};
use std::fmt;

pub const MAX_CAPABILITY_PROFILES: usize = 32;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u16)]
pub enum CryptoProfileId {
    Gw0Session = 0x0100,
}

impl CryptoProfileId {
    pub const fn security_rank(self) -> u8 {
        match self {
            Self::Gw0Session => 1,
        }
    }

    pub const fn wire_id(self) -> u16 {
        self as u16
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum PostQuantumMode {
    Classical = 0,
    Hybrid = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u8)]
pub enum NonceMode {
    Counter = 0,
    Random = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct CryptoProfile {
    pub id: CryptoProfileId,
    pub protocol_version: u16,
    pub max_plaintext_fragment: u32,
    pub replay_window: u32,
    pub pq_mode: PostQuantumMode,
    pub nonce_mode: NonceMode,
    pub requires_fragment_authentication: bool,
}

impl CryptoProfile {
    pub const GW0_SESSION: Self = Self {
        id: CryptoProfileId::Gw0Session,
        protocol_version: 1,
        max_plaintext_fragment: 60 * 1024,
        replay_window: 128,
        pq_mode: PostQuantumMode::Classical,
        nonce_mode: NonceMode::Counter,
        requires_fragment_authentication: true,
    };
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CryptoCapabilities {
    pub protocol_min: u16,
    pub protocol_max: u16,
    pub profiles: Vec<CryptoProfileId>,
    pub max_frame_size: u32,
    pub supports_fragmentation: bool,
}

impl CryptoCapabilities {
    pub fn new(
        mut profiles: Vec<CryptoProfileId>,
        max_frame_size: u32,
        supports_fragmentation: bool,
    ) -> Result<Self, NegotiationError> {
        if profiles.is_empty() || profiles.len() > MAX_CAPABILITY_PROFILES {
            return Err(NegotiationError::InvalidCapabilities(
                "profile count out of bounds",
            ));
        }
        profiles.sort_by_key(|profile| profile.wire_id());
        profiles.dedup();
        if max_frame_size == 0 {
            return Err(NegotiationError::InvalidCapabilities(
                "frame size must be non-zero",
            ));
        }
        Ok(Self {
            protocol_min: 1,
            protocol_max: 1,
            profiles,
            max_frame_size,
            supports_fragmentation,
        })
    }

    /// Canonical bytes must be included in the authenticated handshake transcript.
    pub fn canonical_bytes(&self) -> Vec<u8> {
        let mut out = Vec::with_capacity(16 + self.profiles.len() * 2);
        out.extend_from_slice(&self.protocol_min.to_be_bytes());
        out.extend_from_slice(&self.protocol_max.to_be_bytes());
        out.extend_from_slice(&self.max_frame_size.to_be_bytes());
        out.push(u8::from(self.supports_fragmentation));
        out.extend_from_slice(&(self.profiles.len() as u16).to_be_bytes());
        for profile in &self.profiles {
            out.extend_from_slice(&profile.wire_id().to_be_bytes());
        }
        out
    }

    /// Decode the exact canonical capability representation used on the wire.
    /// Unknown fields, profiles, truncation, and trailing bytes are rejected.
    pub fn from_canonical_bytes(bytes: &[u8]) -> Result<Self, NegotiationError> {
        fn take<'a>(
            bytes: &'a [u8],
            offset: &mut usize,
            len: usize,
        ) -> Result<&'a [u8], NegotiationError> {
            let end = offset
                .checked_add(len)
                .ok_or(NegotiationError::InvalidCapabilities(
                    "capability length overflow",
                ))?;
            if end > bytes.len() {
                return Err(NegotiationError::InvalidCapabilities(
                    "truncated capability encoding",
                ));
            }
            let value = &bytes[*offset..end];
            *offset = end;
            Ok(value)
        }

        let mut offset = 0;
        let protocol_min = u16::from_be_bytes(
            take(bytes, &mut offset, 2)?
                .try_into()
                .map_err(|_| NegotiationError::InvalidCapabilities("invalid protocol minimum"))?,
        );
        let protocol_max = u16::from_be_bytes(
            take(bytes, &mut offset, 2)?
                .try_into()
                .map_err(|_| NegotiationError::InvalidCapabilities("invalid protocol maximum"))?,
        );
        let max_frame_size = u32::from_be_bytes(
            take(bytes, &mut offset, 4)?
                .try_into()
                .map_err(|_| NegotiationError::InvalidCapabilities("invalid frame size"))?,
        );
        let supports_fragmentation = match take(bytes, &mut offset, 1)?[0] {
            0 => false,
            1 => true,
            _ => {
                return Err(NegotiationError::InvalidCapabilities(
                    "invalid fragmentation flag",
                ))
            }
        };
        let profile_count = u16::from_be_bytes(
            take(bytes, &mut offset, 2)?
                .try_into()
                .map_err(|_| NegotiationError::InvalidCapabilities("invalid profile count"))?,
        ) as usize;
        if profile_count == 0 || profile_count > MAX_CAPABILITY_PROFILES {
            return Err(NegotiationError::InvalidCapabilities(
                "profile count out of bounds",
            ));
        }

        let mut profiles = Vec::with_capacity(profile_count);
        for _ in 0..profile_count {
            let wire_id = u16::from_be_bytes(
                take(bytes, &mut offset, 2)?
                    .try_into()
                    .map_err(|_| NegotiationError::InvalidCapabilities("invalid profile id"))?,
            );
            let profile = match wire_id {
                value if value == CryptoProfileId::Gw0Session.wire_id() => {
                    CryptoProfileId::Gw0Session
                }
                _ => return Err(NegotiationError::InvalidCapabilities("unknown profile id")),
            };
            profiles.push(profile);
        }
        if offset != bytes.len() {
            return Err(NegotiationError::InvalidCapabilities(
                "trailing capability bytes",
            ));
        }

        let capabilities = Self::new(profiles, max_frame_size, supports_fragmentation)?;
        if capabilities.protocol_min != protocol_min || capabilities.protocol_max != protocol_max {
            return Err(NegotiationError::InvalidCapabilities(
                "unsupported protocol range",
            ));
        }
        Ok(capabilities)
    }

    pub fn supports(&self, profile: CryptoProfileId) -> bool {
        self.profiles
            .binary_search_by_key(&profile.wire_id(), |candidate| candidate.wire_id())
            .is_ok()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NegotiatedProfile {
    pub profile: CryptoProfile,
    pub max_frame_size: u32,
    pub fragmentation_required: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NegotiationError {
    InvalidCapabilities(&'static str),
    NoCommonProfile,
    ProtocolVersionMismatch,
    DowngradeRejected,
    FrameTooSmall,
}

impl fmt::Display for NegotiationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidCapabilities(reason) => write!(f, "invalid crypto capabilities: {reason}"),
            Self::NoCommonProfile => f.write_str("no common crypto profile"),
            Self::ProtocolVersionMismatch => f.write_str("no common crypto protocol version"),
            Self::DowngradeRejected => f.write_str("crypto profile downgrade rejected"),
            Self::FrameTooSmall => {
                f.write_str("transport frame is too small for the selected profile")
            }
        }
    }
}

impl std::error::Error for NegotiationError {}

/// Select the strongest mutually supported profile. `minimum_rank` is a local
/// policy floor, never a transport-driven fallback.
pub fn negotiate(
    local: &CryptoCapabilities,
    remote: &CryptoCapabilities,
    minimum_rank: u8,
) -> Result<NegotiatedProfile, NegotiationError> {
    let protocol_min = local.protocol_min.max(remote.protocol_min);
    let protocol_max = local.protocol_max.min(remote.protocol_max);
    if protocol_min > protocol_max {
        return Err(NegotiationError::ProtocolVersionMismatch);
    }

    let selected = local
        .profiles
        .iter()
        .copied()
        .filter(|profile| remote.supports(*profile))
        .filter(|profile| profile.security_rank() >= minimum_rank)
        .max_by_key(|profile| profile.security_rank())
        .ok_or(NegotiationError::NoCommonProfile)?;

    let profile = match selected {
        CryptoProfileId::Gw0Session => CryptoProfile::GW0_SESSION,
    };
    if profile.id.security_rank() < minimum_rank {
        return Err(NegotiationError::DowngradeRejected);
    }

    let max_frame_size = local.max_frame_size.min(remote.max_frame_size);
    let fragmentation_required = max_frame_size < profile.max_plaintext_fragment;
    if fragmentation_required && !(local.supports_fragmentation && remote.supports_fragmentation) {
        return Err(NegotiationError::FrameTooSmall);
    }

    Ok(NegotiatedProfile {
        profile,
        max_frame_size,
        fragmentation_required,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    fn caps(profiles: Vec<CryptoProfileId>, frame: u32, fragments: bool) -> CryptoCapabilities {
        CryptoCapabilities::new(profiles, frame, fragments).unwrap()
    }

    #[test]
    fn canonical_encoding_is_deterministic() {
        let a = caps(vec![CryptoProfileId::Gw0Session], 1200, true);
        let b = caps(vec![CryptoProfileId::Gw0Session], 1200, true);
        assert_eq!(a.canonical_bytes(), b.canonical_bytes());
    }

    #[test]
    fn canonical_encoding_round_trips_strictly() {
        let original = caps(vec![CryptoProfileId::Gw0Session], 1200, true);
        let decoded =
            CryptoCapabilities::from_canonical_bytes(&original.canonical_bytes()).unwrap();
        assert_eq!(decoded, original);
    }

    #[test]
    fn negotiation_requires_common_profile() {
        let local = caps(vec![CryptoProfileId::Gw0Session], 1200, true);
        let remote = caps(vec![CryptoProfileId::Gw0Session], 1200, true);
        let negotiated = negotiate(&local, &remote, 1).unwrap();
        assert!(negotiated.fragmentation_required);
    }

    #[test]
    fn small_transport_requires_both_sides_to_fragment() {
        let local = caps(vec![CryptoProfileId::Gw0Session], 1200, true);
        let remote = caps(vec![CryptoProfileId::Gw0Session], 1200, false);
        assert_eq!(
            negotiate(&local, &remote, 1),
            Err(NegotiationError::FrameTooSmall)
        );
    }

    #[test]
    fn minimum_rank_rejects_unsupported_downgrade() {
        let local = caps(vec![CryptoProfileId::Gw0Session], 64 * 1024, true);
        let remote = caps(vec![CryptoProfileId::Gw0Session], 64 * 1024, true);
        assert_eq!(
            negotiate(&local, &remote, 2),
            Err(NegotiationError::NoCommonProfile)
        );
    }
}
