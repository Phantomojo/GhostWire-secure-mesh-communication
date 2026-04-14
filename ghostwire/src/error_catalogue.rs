// GhostWire Error Catalogue - MVP (10 basic error codes)
// File: ghostwire/src/error_catalogue.rs
// 
// This provides standardized error codes for debugging first ping issues.
// Expand based on real errors encountered during testing.

use serde::{Deserialize, Serialize};
use std::fmt;

/// GhostWire error codes - MVP set (10 codes)
/// 
/// Format: 0xCCCC where CCCC is the error code
/// Categories:
///   0x0000-0x00FF: Network errors
///   0x1000-0x10FF: Crypto errors
///   0x2000-0x20FF: Protocol errors
///   0x4000-0x40FF: Resource errors
///   0x6000-0x6FFF: Internal errors
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[repr(u16)]
pub enum ErrorCode {
    // Network errors (0x0000-0x00FF)
    ConnectionFailed = 0x0001,
    ConnectionTimeout = 0x0002,
    DnsResolutionFailed = 0x0003,
    
    // Crypto errors (0x1000-0x10FF)
    HandshakeFailed = 0x1001,
    PeerIdMismatch = 0x1002,
    
    // Protocol errors (0x2000-0x20FF)
    MessageTimeout = 0x2001,
    ProtocolViolation = 0x2002,
    
    // Resource errors (0x4000-0x40FF)
    ConnectionLimitReached = 0x4001,
    
    // Internal errors (0x6000-0x6FFF)
    Unknown = 0x6FFF,
}

impl ErrorCode {
    /// Get human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            ErrorCode::ConnectionFailed => "Failed to establish connection to peer",
            ErrorCode::ConnectionTimeout => "Connection attempt timed out",
            ErrorCode::DnsResolutionFailed => "Failed to resolve hostname",
            ErrorCode::HandshakeFailed => "Noise protocol handshake failed",
            ErrorCode::PeerIdMismatch => "Remote peer ID does not match expected",
            ErrorCode::MessageTimeout => "Message delivery timed out",
            ErrorCode::ProtocolViolation => "Protocol violation detected",
            ErrorCode::ConnectionLimitReached => "Maximum connection limit reached",
            ErrorCode::Unknown => "Unknown error occurred",
        }
    }
    
    /// Get error category
    pub fn category(&self) -> &'static str {
        match self {
            ErrorCode::ConnectionFailed |
            ErrorCode::ConnectionTimeout |
            ErrorCode::DnsResolutionFailed => "NETWORK",
            
            ErrorCode::HandshakeFailed |
            ErrorCode::PeerIdMismatch => "CRYPTO",
            
            ErrorCode::MessageTimeout |
            ErrorCode::ProtocolViolation => "PROTOCOL",
            
            ErrorCode::ConnectionLimitReached => "RESOURCE",
            
            ErrorCode::Unknown => "INTERNAL",
        }
    }
    
    /// Whether this error is retryable
    pub fn is_retryable(&self) -> bool {
        match self {
            ErrorCode::ConnectionFailed |
            ErrorCode::ConnectionTimeout |
            ErrorCode::DnsResolutionFailed |
            ErrorCode::MessageTimeout => true,
            
            ErrorCode::HandshakeFailed |
            ErrorCode::PeerIdMismatch |
            ErrorCode::ProtocolViolation |
            ErrorCode::ConnectionLimitReached |
            ErrorCode::Unknown => false,
        }
    }
    
    /// Suggested retry delay in milliseconds
    pub fn retry_delay_ms(&self) -> Option<u64> {
        match self {
            ErrorCode::ConnectionTimeout => Some(5000),  // 5 seconds
            ErrorCode::ConnectionFailed => Some(2000),   // 2 seconds
            ErrorCode::MessageTimeout => Some(3000),     // 3 seconds
            _ => None,
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?} (0x{:04X})", self, *self as u16)
    }
}

/// GhostWire error with rich context
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GhostWireError {
    /// Machine-readable error code
    pub code: ErrorCode,
    
    /// Human-readable message
    pub message: String,
    
    /// Optional peer ID if relevant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub peer_id: Option<String>,
    
    /// Optional multiaddr if relevant
    #[serde(skip_serializing_if = "Option::is_none")]
    pub multiaddr: Option<String>,
    
    /// Optional stack trace (debug builds only)
    #[cfg(debug_assertions)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub trace: Option<String>,
}

impl GhostWireError {
    /// Create a new error
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
    
    /// Add peer ID context
    pub fn with_peer_id(mut self, peer_id: impl Into<String>) -> Self {
        self.peer_id = Some(peer_id.into());
        self
    }
    
    /// Add multiaddr context
    pub fn with_multiaddr(mut self, addr: impl Into<String>) -> Self {
        self.multiaddr = Some(addr.into());
        self
    }
    
    /// Add debug trace
    #[cfg(debug_assertions)]
    pub fn with_trace(mut self, trace: impl Into<String>) -> Self {
        self.trace = Some(trace.into());
        self
    }
    
    /// Log the error with appropriate level and capture in Sentry if configured
    pub fn log(&self) {
        match self.code {
            ErrorCode::HandshakeFailed |
            ErrorCode::PeerIdMismatch |
            ErrorCode::ProtocolViolation => {
                tracing::error!(
                    target: "ghostwire::error",
                    code = format!("0x{:04X}", self.code as u16),
                    category = self.code.category(),
                    peer_id = ?self.peer_id,
                    "{}: {}",
                    self.code,
                    self.message
                );
                // Capture in Sentry with context
                sentry::with_scope(
                    |scope| {
                        scope.set_tag("error.category", self.code.category());
                        scope.set_tag("error.code", format!("0x{:04X}", self.code as u16));
                        scope.set_tag("error.retryable", self.code.is_retryable());
                        if let Some(ref peer_id) = self.peer_id {
                            scope.set_tag("error.peer_id", peer_id);
                        }
                        if let Some(ref addr) = self.multiaddr {
                            scope.set_tag("error.multiaddr", addr);
                        }
                    },
                    || sentry::capture_message(&self.to_string(), sentry::Level::Error),
                );
            }

            ErrorCode::ConnectionFailed |
            ErrorCode::ConnectionTimeout |
            ErrorCode::MessageTimeout => {
                tracing::warn!(
                    target: "ghostwire::error",
                    code = format!("0x{:04X}", self.code as u16),
                    category = self.code.category(),
                    retryable = self.code.is_retryable(),
                    "{}: {}",
                    self.code,
                    self.message
                );
                // Capture in Sentry as warning
                sentry::with_scope(
                    |scope| {
                        scope.set_tag("error.category", self.code.category());
                        scope.set_tag("error.code", format!("0x{:04X}", self.code as u16));
                        scope.set_tag("error.retryable", self.code.is_retryable());
                        if let Some(ref peer_id) = self.peer_id {
                            scope.set_tag("error.peer_id", peer_id);
                        }
                    },
                    || sentry::capture_message(&self.to_string(), sentry::Level::Warning),
                );
            }

            _ => {
                tracing::info!(
                    target: "ghostwire::error",
                    code = format!("0x{:04X}", self.code as u16),
                    category = self.code.category(),
                    "{}: {}",
                    self.code,
                    self.message
                );
                // Capture in Sentry as info
                sentry::with_scope(
                    |scope| {
                        scope.set_tag("error.category", self.code.category());
                        scope.set_tag("error.code", format!("0x{:04X}", self.code as u16));
                    },
                    || sentry::capture_message(&self.to_string(), sentry::Level::Info),
                );
            }
        }
    }
}

impl fmt::Display for GhostWireError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {}: {}", self.code.category(), self.code, self.message)?;
        
        if let Some(ref peer_id) = self.peer_id {
            write!(f, " (peer: {})", peer_id)?;
        }
        
        if let Some(ref addr) = self.multiaddr {
            write!(f, " (addr: {})", addr)?;
        }
        
        Ok(())
    }
}

impl std::error::Error for GhostWireError {}

// Convenience conversion functions
impl From<std::io::Error> for GhostWireError {
    fn from(err: std::io::Error) -> Self {
        let code = match err.kind() {
            std::io::ErrorKind::TimedOut => ErrorCode::ConnectionTimeout,
            std::io::ErrorKind::ConnectionRefused |
            std::io::ErrorKind::ConnectionReset |
            std::io::ErrorKind::ConnectionAborted => ErrorCode::ConnectionFailed,
            std::io::ErrorKind::WouldBlock => ErrorCode::ConnectionTimeout,
            _ => ErrorCode::Unknown,
        };
        
        GhostWireError::new(code, err.to_string())
    }
}

impl From<tokio::time::error::Elapsed> for GhostWireError {
    fn from(_: tokio::time::error::Elapsed) -> Self {
        GhostWireError::new(
            ErrorCode::ConnectionTimeout,
            "Operation timed out"
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_code_display() {
        assert_eq!(
            format!("{}", ErrorCode::ConnectionFailed),
            "ConnectionFailed (0x0001)"
        );
    }
    
    #[test]
    fn test_error_code_retryable() {
        assert!(ErrorCode::ConnectionTimeout.is_retryable());
        assert!(!ErrorCode::HandshakeFailed.is_retryable());
    }
    
    #[test]
    fn test_error_with_context() {
        let err = GhostWireError::new(ErrorCode::ConnectionFailed, "Test error")
            .with_peer_id("QmTest123")
            .with_multiaddr("/ip4/127.0.0.1/tcp/4001");
        
        assert_eq!(err.peer_id, Some("QmTest123".to_string()));
        assert_eq!(err.multiaddr, Some("/ip4/127.0.0.1/tcp/4001".to_string()));
    }
}
