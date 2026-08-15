//! GhostWire's stable, privacy-safe public fault catalogue.
//!
//! Internal subsystems may retain richer error sources, but API, WebSocket, CLI,
//! and event boundaries must expose only [`PublicFault`]. In particular, peer
//! identifiers, routes, addresses, message content, keys, and raw source errors
//! are local diagnostic data and must never be serialized by this module.

use serde::{Deserialize, Serialize};
use std::fmt;

/// Broad, stable fault category intended for client behavior and aggregation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
#[non_exhaustive]
pub enum FaultCategory {
    Transport,
    Authorization,
    IdentityCrypto,
    Protocol,
    PrivacyPolicy,
    Resource,
    Storage,
    Configuration,
    ExternalAdapter,
    Internal,
}

impl FaultCategory {
    /// Stable string form retained for existing logging and API consumers.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Transport => "TRANSPORT",
            Self::Authorization => "AUTHORIZATION",
            Self::IdentityCrypto => "IDENTITY_CRYPTO",
            Self::Protocol => "PROTOCOL",
            Self::PrivacyPolicy => "PRIVACY_POLICY",
            Self::Resource => "RESOURCE",
            Self::Storage => "STORAGE",
            Self::Configuration => "CONFIGURATION",
            Self::ExternalAdapter => "EXTERNAL_ADAPTER",
            Self::Internal => "INTERNAL",
        }
    }
}

/// Client-visible retry instruction. Clients must not infer retry policy from
/// display strings or raw underlying errors.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
#[non_exhaustive]
pub enum RetryDirective {
    Never,
    After { delay_ms: u64 },
    QueueUntilRouteAvailable,
    UserAction,
    NewOperation,
}

impl RetryDirective {
    #[must_use]
    pub const fn is_retryable(self) -> bool {
        matches!(self, Self::After { .. } | Self::QueueUntilRouteAvailable)
    }

    #[must_use]
    pub const fn delay_ms(self) -> Option<u64> {
        match self {
            Self::After { delay_ms } => Some(delay_ms),
            Self::Never
            | Self::QueueUntilRouteAvailable
            | Self::UserAction
            | Self::NewOperation => None,
        }
    }
}

/// Stable error codes for GhostWire's public boundaries.
///
/// Code ranges are allocated by `docs/ERROR_INDEX.md`. New codes must include a
/// public-safe description, category, retry directive, and a regression test.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[repr(u16)]
#[non_exhaustive]
pub enum ErrorCode {
    // Connectivity and transport (0x0000-0x00FF)
    ConnectionFailed = 0x0001,
    ConnectionTimeout = 0x0002,
    DnsResolutionFailed = 0x0003,
    TransportCapabilityUnavailable = 0x0004,

    // Local session and authorization (0x0100-0x01FF)
    SessionUnauthenticated = 0x0101,
    CommandForbidden = 0x0102,
    OriginRejected = 0x0103,

    // Identity and cryptography (0x1000-0x10FF)
    HandshakeFailed = 0x1001,
    PeerIdMismatch = 0x1002,
    IdentityRevoked = 0x1003,
    AuthenticationFailed = 0x1004,
    SignatureInvalid = 0x1005,

    // Protocol and message validation (0x2000-0x20FF)
    MessageTimeout = 0x2001,
    ProtocolViolation = 0x2002,
    FrameMalformed = 0x2003,
    FrameExpired = 0x2004,
    FrameReplayed = 0x2005,
    CausalDependencyMissing = 0x2006,
    ProtocolVersionUnsupported = 0x2007,

    // Privacy and delivery policy (0x3000-0x30FF)
    PrivacyRouteRequired = 0x3001,
    PrivacyDowngradeRequiresApproval = 0x3002,
    PrivateDeliveryUnavailable = 0x3003,

    // Resource limits (0x4000-0x40FF)
    ConnectionLimitReached = 0x4001,
    FrameSizeLimit = 0x4002,
    QueueCapacityLimit = 0x4003,
    RateLimited = 0x4004,

    // Storage and lifecycle (0x5000-0x50FF)
    EncryptedPersistenceFailed = 0x5001,
    MessageLifecycleConflict = 0x5002,

    // Configuration (0x6000-0x60FF)
    FeatureUnavailable = 0x6001,
    ConfigurationInvalid = 0x6002,

    // Optional external adapters (0x7000-0x70FF)
    DataPlaneOfferInvalid = 0x7001,
    ReticulumBridgeUnavailable = 0x7002,

    // Internal fallback codes.
    InternalInvariant = 0x7F00,
    Unknown = 0x6FFF,
}

impl ErrorCode {
    /// Every code currently allocated by this module. This supports uniqueness
    /// and metadata-completeness tests without requiring callers to match an
    /// extensible public enum exhaustively.
    pub const ALL: &'static [Self] = &[
        Self::ConnectionFailed,
        Self::ConnectionTimeout,
        Self::DnsResolutionFailed,
        Self::TransportCapabilityUnavailable,
        Self::SessionUnauthenticated,
        Self::CommandForbidden,
        Self::OriginRejected,
        Self::HandshakeFailed,
        Self::PeerIdMismatch,
        Self::IdentityRevoked,
        Self::AuthenticationFailed,
        Self::SignatureInvalid,
        Self::MessageTimeout,
        Self::ProtocolViolation,
        Self::FrameMalformed,
        Self::FrameExpired,
        Self::FrameReplayed,
        Self::CausalDependencyMissing,
        Self::ProtocolVersionUnsupported,
        Self::PrivacyRouteRequired,
        Self::PrivacyDowngradeRequiresApproval,
        Self::PrivateDeliveryUnavailable,
        Self::ConnectionLimitReached,
        Self::FrameSizeLimit,
        Self::QueueCapacityLimit,
        Self::RateLimited,
        Self::EncryptedPersistenceFailed,
        Self::MessageLifecycleConflict,
        Self::FeatureUnavailable,
        Self::ConfigurationInvalid,
        Self::DataPlaneOfferInvalid,
        Self::ReticulumBridgeUnavailable,
        Self::InternalInvariant,
        Self::Unknown,
    ];

    /// Public-safe description. It must not disclose topology, identities,
    /// secrets, content, routes, payment data, or implementation detail.
    #[must_use]
    pub const fn description(self) -> &'static str {
        match self {
            Self::ConnectionFailed => "Failed to establish a transport connection",
            Self::ConnectionTimeout => "The transport attempt timed out",
            Self::DnsResolutionFailed => "Failed to resolve the requested host",
            Self::TransportCapabilityUnavailable => {
                "No transport satisfies this delivery requirement"
            }
            Self::SessionUnauthenticated => "Authentication is required",
            Self::CommandForbidden => "This command is not permitted",
            Self::OriginRejected => "This client origin is not permitted",
            Self::HandshakeFailed => "The secure handshake failed",
            Self::PeerIdMismatch => "The remote identity did not match the expected identity",
            Self::IdentityRevoked => "The identity is no longer trusted",
            Self::AuthenticationFailed => "Message authentication failed",
            Self::SignatureInvalid => "Message signature verification failed",
            Self::MessageTimeout => "Message delivery timed out",
            Self::ProtocolViolation => "Protocol validation failed",
            Self::FrameMalformed => "The message format is invalid",
            Self::FrameExpired => "The message has expired",
            Self::FrameReplayed => "This message was already processed",
            Self::CausalDependencyMissing => "The message is waiting for earlier messages",
            Self::ProtocolVersionUnsupported => "This protocol version is not supported",
            Self::PrivacyRouteRequired => "No route meets the required privacy level",
            Self::PrivacyDowngradeRequiresApproval => {
                "A less private route requires explicit approval"
            }
            Self::PrivateDeliveryUnavailable => "Private delivery is temporarily unavailable",
            Self::ConnectionLimitReached => "The connection limit has been reached",
            Self::FrameSizeLimit => "The message exceeds the permitted size",
            Self::QueueCapacityLimit => "The delivery queue is at capacity",
            Self::RateLimited => "The operation is temporarily rate limited",
            Self::EncryptedPersistenceFailed => "The message could not be stored securely",
            Self::MessageLifecycleConflict => "The message state could not be updated",
            Self::FeatureUnavailable => "This feature is not enabled",
            Self::ConfigurationInvalid => "The local configuration is invalid",
            Self::DataPlaneOfferInvalid => "The data-transfer offer was rejected",
            Self::ReticulumBridgeUnavailable => "The Reticulum bridge is unavailable",
            Self::InternalInvariant => "An internal operation failed",
            Self::Unknown => "An unknown operation failed",
        }
    }

    #[must_use]
    pub const fn fault_category(self) -> FaultCategory {
        match self {
            Self::ConnectionFailed
            | Self::ConnectionTimeout
            | Self::DnsResolutionFailed
            | Self::TransportCapabilityUnavailable => FaultCategory::Transport,
            Self::SessionUnauthenticated | Self::CommandForbidden | Self::OriginRejected => {
                FaultCategory::Authorization
            }
            Self::HandshakeFailed
            | Self::PeerIdMismatch
            | Self::IdentityRevoked
            | Self::AuthenticationFailed
            | Self::SignatureInvalid => FaultCategory::IdentityCrypto,
            Self::MessageTimeout
            | Self::ProtocolViolation
            | Self::FrameMalformed
            | Self::FrameExpired
            | Self::FrameReplayed
            | Self::CausalDependencyMissing
            | Self::ProtocolVersionUnsupported => FaultCategory::Protocol,
            Self::PrivacyRouteRequired
            | Self::PrivacyDowngradeRequiresApproval
            | Self::PrivateDeliveryUnavailable => FaultCategory::PrivacyPolicy,
            Self::ConnectionLimitReached
            | Self::FrameSizeLimit
            | Self::QueueCapacityLimit
            | Self::RateLimited => FaultCategory::Resource,
            Self::EncryptedPersistenceFailed | Self::MessageLifecycleConflict => {
                FaultCategory::Storage
            }
            Self::FeatureUnavailable | Self::ConfigurationInvalid => FaultCategory::Configuration,
            Self::DataPlaneOfferInvalid | Self::ReticulumBridgeUnavailable => {
                FaultCategory::ExternalAdapter
            }
            Self::InternalInvariant | Self::Unknown => FaultCategory::Internal,
        }
    }

    /// Stable string form retained for existing callers.
    #[must_use]
    pub const fn category(self) -> &'static str {
        self.fault_category().as_str()
    }

    #[must_use]
    pub const fn retry_directive(self) -> RetryDirective {
        match self {
            Self::ConnectionFailed => RetryDirective::After { delay_ms: 2_000 },
            Self::ConnectionTimeout => RetryDirective::After { delay_ms: 5_000 },
            Self::DnsResolutionFailed => RetryDirective::After { delay_ms: 5_000 },
            Self::MessageTimeout => RetryDirective::After { delay_ms: 3_000 },
            Self::RateLimited => RetryDirective::After { delay_ms: 5_000 },
            Self::TransportCapabilityUnavailable
            | Self::PrivacyRouteRequired
            | Self::PrivateDeliveryUnavailable => RetryDirective::QueueUntilRouteAvailable,
            Self::SessionUnauthenticated
            | Self::PrivacyDowngradeRequiresApproval
            | Self::ConfigurationInvalid => RetryDirective::UserAction,
            Self::FrameExpired => RetryDirective::NewOperation,
            Self::CommandForbidden
            | Self::OriginRejected
            | Self::HandshakeFailed
            | Self::PeerIdMismatch
            | Self::IdentityRevoked
            | Self::AuthenticationFailed
            | Self::SignatureInvalid
            | Self::ProtocolViolation
            | Self::FrameMalformed
            | Self::FrameReplayed
            | Self::CausalDependencyMissing
            | Self::ProtocolVersionUnsupported
            | Self::ConnectionLimitReached
            | Self::FrameSizeLimit
            | Self::QueueCapacityLimit
            | Self::EncryptedPersistenceFailed
            | Self::MessageLifecycleConflict
            | Self::FeatureUnavailable
            | Self::DataPlaneOfferInvalid
            | Self::ReticulumBridgeUnavailable
            | Self::InternalInvariant
            | Self::Unknown => RetryDirective::Never,
        }
    }

    #[must_use]
    pub const fn is_retryable(self) -> bool {
        self.retry_directive().is_retryable()
    }

    #[must_use]
    pub const fn retry_delay_ms(self) -> Option<u64> {
        self.retry_directive().delay_ms()
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} (0x{:04X})", self, *self as u16)
    }
}

/// Public, serializable fault safe to return across API, WebSocket, CLI, and
/// event boundaries. `diagnostic_ref` is a local opaque correlation handle; it
/// must not be derived from a peer, route, message, conversation, or secret.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PublicFault {
    pub code: ErrorCode,
    pub category: FaultCategory,
    pub retry: RetryDirective,
    pub message: &'static str,
    pub diagnostic_ref: Option<String>,
}

impl PublicFault {
    #[must_use]
    pub fn from_code(code: ErrorCode) -> Self {
        Self {
            code,
            category: code.fault_category(),
            retry: code.retry_directive(),
            message: code.description(),
            diagnostic_ref: None,
        }
    }

    #[must_use]
    pub fn with_diagnostic_ref(mut self, diagnostic_ref: impl Into<String>) -> Self {
        self.diagnostic_ref = Some(diagnostic_ref.into());
        self
    }
}

/// Internal fault container. The optional peer and address values are retained
/// for protected local diagnostics only and are deliberately skipped during
/// serialization and display.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostWireError {
    pub code: ErrorCode,
    pub message: String,
    #[serde(skip)]
    pub peer_id: Option<String>,
    #[serde(skip)]
    pub multiaddr: Option<String>,
    #[cfg(debug_assertions)]
    #[serde(skip)]
    pub trace: Option<String>,
}

impl GhostWireError {
    #[must_use]
    pub fn new(code: ErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
            peer_id: None,
            multiaddr: None,
            #[cfg(debug_assertions)]
            trace: None,
        }
    }

    /// Attach protected local context. Do not serialize this type directly at
    /// an API/client boundary; use [`Self::to_public`] instead.
    #[must_use]
    pub fn with_peer_id(mut self, peer_id: impl Into<String>) -> Self {
        self.peer_id = Some(peer_id.into());
        self
    }

    /// Attach protected local context. Do not serialize this type directly at
    /// an API/client boundary; use [`Self::to_public`] instead.
    #[must_use]
    pub fn with_multiaddr(mut self, addr: impl Into<String>) -> Self {
        self.multiaddr = Some(addr.into());
        self
    }

    #[cfg(debug_assertions)]
    #[must_use]
    pub fn with_trace(mut self, trace: impl Into<String>) -> Self {
        self.trace = Some(trace.into());
        self
    }

    #[must_use]
    pub fn to_public(&self) -> PublicFault {
        PublicFault::from_code(self.code)
    }

    /// Log a privacy-safe classification. Operators can correlate protected
    /// context through dedicated local diagnostics, not general-purpose logs.
    pub fn log(&self) {
        let peer_context_present = self.peer_id.is_some();
        let route_context_present = self.multiaddr.is_some();
        let retry = self.code.retry_directive();
        match self.code.fault_category() {
            FaultCategory::IdentityCrypto | FaultCategory::Protocol => tracing::warn!(
                target: "ghostwire::error",
                code = format_args!("0x{:04X}", self.code as u16),
                category = self.code.category(),
                peer_context_present,
                route_context_present,
                retryable = retry.is_retryable(),
                "{}",
                self.code.description()
            ),
            FaultCategory::Internal => tracing::error!(
                target: "ghostwire::error",
                code = format_args!("0x{:04X}", self.code as u16),
                category = self.code.category(),
                peer_context_present,
                route_context_present,
                "{}",
                self.code.description()
            ),
            _ => tracing::info!(
                target: "ghostwire::error",
                code = format_args!("0x{:04X}", self.code as u16),
                category = self.code.category(),
                peer_context_present,
                route_context_present,
                retryable = retry.is_retryable(),
                "{}",
                self.code.description()
            ),
        }
    }
}

impl fmt::Display for GhostWireError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}] {}: {}",
            self.code.category(),
            self.code,
            self.code.description()
        )
    }
}

impl std::error::Error for GhostWireError {}

impl From<std::io::Error> for GhostWireError {
    fn from(err: std::io::Error) -> Self {
        let code = match err.kind() {
            std::io::ErrorKind::TimedOut => ErrorCode::ConnectionTimeout,
            std::io::ErrorKind::ConnectionRefused
            | std::io::ErrorKind::ConnectionReset
            | std::io::ErrorKind::ConnectionAborted => ErrorCode::ConnectionFailed,
            std::io::ErrorKind::WouldBlock => ErrorCode::ConnectionTimeout,
            _ => ErrorCode::Unknown,
        };
        Self::new(code, err.to_string())
    }
}

impl From<tokio::time::error::Elapsed> for GhostWireError {
    fn from(_: tokio::time::error::Elapsed) -> Self {
        Self::new(ErrorCode::ConnectionTimeout, "Operation timed out")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn error_code_display_is_stable() {
        assert_eq!(
            ErrorCode::ConnectionFailed.to_string(),
            "ConnectionFailed (0x0001)"
        );
    }

    #[test]
    fn retry_directives_are_explicit() {
        assert_eq!(
            ErrorCode::ConnectionTimeout.retry_directive(),
            RetryDirective::After { delay_ms: 5_000 }
        );
        assert_eq!(
            ErrorCode::PrivacyRouteRequired.retry_directive(),
            RetryDirective::QueueUntilRouteAvailable
        );
        assert!(!ErrorCode::IdentityRevoked.is_retryable());
    }

    #[test]
    fn registry_has_unique_codes_and_complete_public_metadata() {
        let mut unique = HashSet::new();
        for code in ErrorCode::ALL {
            assert!(unique.insert(*code as u16), "duplicate code: {code:?}");
            assert!(!code.description().is_empty());
            assert!(!code.category().is_empty());
        }
    }

    #[test]
    fn public_serialization_omits_sensitive_context() {
        let error = GhostWireError::new(ErrorCode::ConnectionFailed, "raw source detail")
            .with_peer_id("peer-secret")
            .with_multiaddr("/ip4/203.0.113.7/tcp/4001");
        let internal_json = serde_json::to_string(&error).unwrap();
        let public_json = serde_json::to_string(&error.to_public()).unwrap();

        assert!(!internal_json.contains("peer-secret"));
        assert!(!internal_json.contains("203.0.113.7"));
        assert!(!public_json.contains("raw source detail"));
        assert!(!public_json.contains("peer-secret"));
        assert!(!public_json.contains("203.0.113.7"));
    }

    #[test]
    fn public_fault_uses_stable_code_metadata() {
        let fault =
            PublicFault::from_code(ErrorCode::FrameSizeLimit).with_diagnostic_ref("local-ref-123");
        assert_eq!(fault.category, FaultCategory::Resource);
        assert_eq!(fault.retry, RetryDirective::Never);
        assert_eq!(fault.message, "The message exceeds the permitted size");
        assert_eq!(fault.diagnostic_ref.as_deref(), Some("local-ref-123"));
    }
}
