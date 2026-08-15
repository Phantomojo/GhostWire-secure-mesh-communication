use thiserror::Error;

#[derive(Error, Debug)]
pub enum GhostError {
    #[error("Network error: {0}")]
    Network(String),

    #[error("Security error: {0}")]
    Security(String),

    #[error("Storage error: {0}")]
    Storage(String),

    #[error("Identity error: {0}")]
    Identity(String),

    #[error(transparent)]
    IdentityCrypto(#[from] crate::core::identity::encryption::IdentityCryptoError),

    #[error("AI error: {0}")]
    Ai(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),

    #[error("Libp2p error: {0}")]
    Libp2p(String),

    #[error("Encryption error: {0}")]
    Encryption(String),

    #[error("Revocation error: {0}")]
    Revocation(String),

    #[error("Ordering error: {0}")]
    Ordering(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}

pub type Result<T> = std::result::Result<T, GhostError>;
