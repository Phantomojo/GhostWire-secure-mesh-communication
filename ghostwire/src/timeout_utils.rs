// Timeout utilities for GhostWire operations
// File: ghostwire/src/timeout_utils.rs
//
// Provides timeout wrappers for common operations to prevent hanging.

use crate::error_catalogue::{ErrorCode, GhostWireError};
use tokio::time::{timeout, Duration};
use anyhow::Result;
use std::future::Future;

/// Default timeout for connection attempts
pub const DEFAULT_CONNECTION_TIMEOUT: Duration = Duration::from_secs(30);

/// Default timeout for handshake operations
pub const DEFAULT_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);

/// Default timeout for message delivery
pub const DEFAULT_MESSAGE_TIMEOUT: Duration = Duration::from_secs(60);

/// Default timeout for peer discovery
pub const DEFAULT_DISCOVERY_TIMEOUT: Duration = Duration::from_secs(30);

/// Execute an async operation with a timeout
/// 
/// # Example
/// ```rust
/// let result = with_timeout(
///     DEFAULT_CONNECTION_TIMEOUT,
///     || async { connect_to_peer(peer_addr).await },
///     ErrorCode::ConnectionTimeout,
///     "Connection attempt timed out"
/// ).await?;
/// ```
pub async fn with_timeout<T, F, Fut>(
    timeout_duration: Duration,
    operation: F,
    error_code: ErrorCode,
    error_message: impl Into<String>,
) -> Result<T, GhostWireError>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<T, GhostWireError>>,
{
    match timeout(timeout_duration, operation()).await {
        Ok(result) => result,
        Err(_) => Err(GhostWireError::new(error_code, error_message)),
    }
}

/// Execute a connection attempt with timeout
pub async fn connect_with_timeout<F, Fut>(
    connect_fn: F,
    timeout_duration: Option<Duration>,
) -> Result<(), GhostWireError>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<(), GhostWireError>>,
{
    let duration = timeout_duration.unwrap_or(DEFAULT_CONNECTION_TIMEOUT);
    
    with_timeout(
        duration,
        connect_fn,
        ErrorCode::ConnectionTimeout,
        format!("Connection attempt timed out after {}s", duration.as_secs()),
    ).await
}

/// Execute handshake with timeout
pub async fn handshake_with_timeout<F, Fut, T>(
    handshake_fn: F,
    timeout_duration: Option<Duration>,
) -> Result<T, GhostWireError>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<T, GhostWireError>>,
{
    let duration = timeout_duration.unwrap_or(DEFAULT_HANDSHAKE_TIMEOUT);
    
    with_timeout(
        duration,
        handshake_fn,
        ErrorCode::HandshakeFailed,
        format!("Handshake timed out after {}s", duration.as_secs()),
    ).await
}

/// Execute message send with timeout
pub async fn send_message_with_timeout<F, Fut, T>(
    send_fn: F,
    timeout_duration: Option<Duration>,
) -> Result<T, GhostWireError>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<T, GhostWireError>>,
{
    let duration = timeout_duration.unwrap_or(DEFAULT_MESSAGE_TIMEOUT);
    
    with_timeout(
        duration,
        send_fn,
        ErrorCode::MessageTimeout,
        format!("Message delivery timed out after {}s", duration.as_secs()),
    ).await
}

/// Execute peer discovery with timeout
pub async fn discover_peer_with_timeout<F, Fut, T>(
    discover_fn: F,
    timeout_duration: Option<Duration>,
) -> Result<T, GhostWireError>
where
    F: FnOnce() -> Fut,
    Fut: Future<Output = Result<T, GhostWireError>>,
{
    let duration = timeout_duration.unwrap_or(DEFAULT_DISCOVERY_TIMEOUT);
    
    with_timeout(
        duration,
        discover_fn,
        ErrorCode::ConnectionTimeout,
        format!("Peer discovery timed out after {}s", duration.as_secs()),
    ).await
}

/// Retry an operation with exponential backoff
/// 
/// # Arguments
/// * `max_retries` - Maximum number of retry attempts
/// * `initial_delay` - Initial delay between retries
/// * `max_delay` - Maximum delay between retries
/// * `operation` - The async operation to retry
/// 
/// # Example
/// ```rust
/// let result = retry_with_backoff(
///     3,
///     Duration::from_millis(100),
///     Duration::from_secs(5),
///     || async { connect_to_peer(peer_addr).await }
/// ).await?;
/// ```
pub async fn retry_with_backoff<T, F, Fut>(
    max_retries: u32,
    initial_delay: Duration,
    max_delay: Duration,
    mut operation: F,
) -> Result<T, GhostWireError>
where
    F: FnMut() -> Fut,
    Fut: Future<Output = Result<T, GhostWireError>>,
{
    let mut last_error: Option<GhostWireError> = None;
    let mut delay = initial_delay;
    
    for attempt in 0..=max_retries {
        match operation().await {
            Ok(result) => return Ok(result),
            Err(e) => {
                if !e.code.is_retryable() {
                    return Err(e);
                }
                
                last_error = Some(e);
                
                if attempt < max_retries {
                    tracing::warn!(
                        target: "ghostwire::retry",
                        attempt = attempt + 1,
                        max_retries = max_retries,
                        delay_ms = delay.as_millis(),
                        "Operation failed, retrying..."
                    );
                    
                    tokio::time::sleep(delay).await;
                    
                    // Exponential backoff with max cap
                    delay = (delay * 2).min(max_delay);
                }
            }
        }
    }
    
    Err(last_error.unwrap_or_else(|| {
        GhostWireError::new(
            ErrorCode::Unknown,
            "Operation failed after all retries"
        )
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::time::sleep;
    
    #[tokio::test]
    async fn test_with_timeout_success() {
        let result = with_timeout(
            Duration::from_millis(100),
            || async { Ok::<_, GhostWireError>("success".to_string()) },
            ErrorCode::ConnectionTimeout,
            "Should not timeout"
        ).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "success");
    }
    
    #[tokio::test]
    async fn test_with_timeout_failure() {
        let result = with_timeout(
            Duration::from_millis(50),
            || async {
                sleep(Duration::from_millis(100)).await;
                Ok::<_, GhostWireError>("too slow".to_string())
            },
            ErrorCode::ConnectionTimeout,
            "Operation timed out"
        ).await;
        
        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, ErrorCode::ConnectionTimeout);
    }
    
    #[tokio::test]
    async fn test_retry_with_backoff_success() {
        let mut attempts = 0;
        
        let result = retry_with_backoff(
            3,
            Duration::from_millis(10),
            Duration::from_millis(100),
            || {
                attempts += 1;
                async move {
                    if attempts < 2 {
                        Err(GhostWireError::new(
                            ErrorCode::ConnectionFailed,
                            "Temporary failure"
                        ))
                    } else {
                        Ok("success")
                    }
                }
            }
        ).await;
        
        assert!(result.is_ok());
        assert_eq!(attempts, 2);
    }
    
    #[tokio::test]
    async fn test_retry_with_backoff_non_retryable() {
        let result: Result<(), GhostWireError> = retry_with_backoff(
            3,
            Duration::from_millis(10),
            Duration::from_millis(100),
            || async {
                Err(GhostWireError::new(
                    ErrorCode::HandshakeFailed,  // Non-retryable
                    "Permanent failure"
                ))
            }
        ).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().code, ErrorCode::HandshakeFailed);
    }
}
