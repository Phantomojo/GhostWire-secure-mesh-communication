// Copyright (C) 2025-2026 Michael Irungu
// SPDX-License-Identifier: AGPL-3.0-or-later

//! # Message Module - Core Communication Structure
//!
//! This module defines the fundamental `Message` struct used throughout GhostWire
//! for representing communication between nodes.
//!
//! ## 🛡️ Security Considerations
//!
//! **IMPORTANT:** This `Message` struct represents **plaintext messages** in memory.
//! When messages are:
//! - ✅ **Stored to disk**: Encrypted via `SecureMessageStore` (XChaCha20-Poly1305)
//! - ✅ **Sent over network**: Encrypted via `EncryptionManager` (ECDH + Kyber)
//! - ⚠️ **In memory**: Plaintext (necessary for processing)
//!
//! ## Architecture
//!
//! ```text
//! ┌─────────────────────────────────────────────────────────────┐
//! │ Message Creation (message.rs)                               │
//! │ ─────────────────────────────────                           │
//! │ Message::new("Alice", "Bob", "Hello")                       │
//! │ Result: Message { content: "Hello", encrypted: false }      │
//! └─────────────────────────────────────────────────────────────┘
//!                              ↓
//! ┌─────────────────────────────────────────────────────────────┐
//! │ Encryption (encryption.rs)                                  │
//! │ ─────────────────────────────                               │
//! │ encrypt_message(&message, &recipient_pubkey)                │
//! │ Result: EncryptedMessage { ciphertext: [gibberish], ... }   │
//! └─────────────────────────────────────────────────────────────┘
//!                              ↓
//! ┌─────────────────────────────────────────────────────────────┐
//! │ Storage OR Network                                          │
//! │ ───────────────────────                                     │
//! │ - Store: Encrypted bytes in Sled DB                         │
//! │ - Network: Encrypted bytes via libp2p                       │
//! └─────────────────────────────────────────────────────────────┘
//! ```
//!
//! ## Design Decisions
//!
//! ### Why Separate Message and EncryptedMessage?
//!
//! 1. **Separation of Concerns**: `Message` is for application logic,
//!    `EncryptedMessage` is for secure transport/storage
//! 2. **Performance**: Avoid encrypting/decrypting during processing
//! 3. **Flexibility**: Can apply different encryption schemes without
//!    changing application logic
//!
//! ### Why Plaintext Fields?
//!
//! - `sender`: Needed for routing and display (encrypted in transit)
//! - `recipient`: Needed for routing (encrypted in transit)
//! - `content`: The actual message (encrypted in transit/storage)
//! - `timestamp`: Needed for ordering (jitter added before sending)
//!
//! ## Future Improvements (TODO)
//!
//! - [ ] Add message priority field
//! - [ ] Add TTL (time-to-live) for auto-expiry
//! - [ ] Add reply-to field for threading
//! - [ ] Add reactions/emoji support
//! - [ ] Add file attachment metadata
//! - [ ] Add message editing support (with edit history)
//! - [ ] Add deletion tombstones (for synced deletion)

use serde::{Deserialize, Serialize};
use uuid::Uuid;

const TIMESTAMP_JITTER_RANGE: u64 = 60; // ±30 seconds

pub const MAX_INLINE_BYTES: usize = 64 * 1024;

/// Type of message being sent
///
/// **Security Note:** Message type is visible in plaintext for routing purposes.
/// Sensitive message types (e.g., `Control`, `Handshake`) should be encrypted
/// at a higher layer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum MessageType {
    /// Standard text message (default)
    #[default]
    Text,
    /// Binary data (file, image, etc.)
    Binary,
    /// System control message
    Control,
    /// Keepalive/heartbeat
    Heartbeat,
    /// Connection handshake
    Handshake,
    /// Encrypted payload (wrapper for EncryptedMessage)
    Encrypted,
}


/// Encryption status of the message
///
/// **Security Note:** This is a **local state** indicator, not a security guarantee.
/// Always verify encryption at the transport/storage layer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
#[derive(Default)]
pub enum EncryptionStatus {
    /// Message is not encrypted (plaintext in memory)
    #[default]
    None,
    /// Message is being encrypted (in transit)
    InProgress,
    /// Message is fully encrypted (storage or network)
    Full,
    /// Message encryption failed (error state)
    Failed,
}


#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MediaMetadata {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub duration_secs: Option<f64>,
    pub codec: Option<String>,
    pub bitrate: Option<u32>,
    pub thumbnail_cid: Option<String>,
    pub file_size: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MessageMetadata {
    pub media: Option<MediaMetadata>,
    pub reply_to: Option<Uuid>,
    pub edit_of: Option<Uuid>,
    pub thread_id: Option<Uuid>,
    pub priority: u8,
    pub expires_at: Option<u64>,
}

/// # Message - Core Communication Structure
///
/// Represents a single message in the GhostWire network.
///
/// ## Fields
///
/// - `id`: Unique identifier (UUID v4) - used for deduplication and tracking
/// - `sender`: Sender's node ID (public key hash) - plaintext for routing
/// - `recipient`: Recipient's node ID - plaintext for routing
/// - `content`: Message content - **ENCRYPTED BEFORE STORAGE/NETWORK**
/// - `timestamp`: Unix timestamp (seconds since 1970) - precise time
/// - `encrypted`: Legacy boolean - use `encryption_status` instead
/// - `message_type`: Type of message (Text, Binary, Control, etc.)
/// - `encryption_status`: Current encryption state
///
/// ## Security Properties
///
/// - ✅ Unique ID prevents replay attacks (when used with deduplication)
/// - ✅ Timestamp enables ordering and expiry
/// - ⚠️ Content is plaintext in memory (encrypt before storage/network)
/// - ⚠️ Metadata (sender, recipient, timestamp) is plaintext in memory
///
/// ## Example Usage
///
/// ```rust
/// // Create a new message
/// let message = Message::new(
///     "Alice".to_string(),
///     "Bob".to_string(),
///     "Hello, World!".to_string(),
/// );
///
/// // Message is plaintext in memory
/// assert_eq!(message.content, "Hello, World!");
/// assert_eq!(message.encrypted, false);
///
/// // Before sending/storing, encrypt via EncryptionManager
/// // let encrypted = encryption.encrypt_message(&message, &recipient_key)?;
/// ```
///
/// ## Performance Characteristics
///
/// - Size: ~100-200 bytes for short messages (in memory)
/// - Clone: Cheap (all fields are Clone)
/// - Serialize: Fast (serde-derived)
/// - Comparison: Use `id` field for equality checks
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Message {
    pub id: Uuid,
    pub sender: String,
    pub recipient: String,
    pub content: String,
    pub content_type: String,
    pub content_bytes: Option<Vec<u8>>,
    pub content_cid: Option<String>,
    pub metadata: Option<MessageMetadata>,
    pub timestamp: u64,
    pub encrypted: bool,
    pub message_type: String,
    pub encryption_status: String,
    pub hops: u8,
    pub ttl: u64,
    pub vector_clock: crate::core::vector_clock::VectorClock,
    pub compressed: bool,
    pub compression_algo: Option<String>,
}

impl Message {
    /// Create a new plaintext message
    ///
    /// ## Parameters
    ///
    /// - `sender`: Sender's node ID (must be valid peer ID format)
    /// - `recipient`: Recipient's node ID (or "broadcast" for broadcast)
    /// - `content`: Message content (should be < 10KB)
    ///
    /// ## Returns
    ///
    /// A new `Message` instance with:
    /// - Unique UUID v4 ID
    /// - Current timestamp
    /// - `encrypted: false`
    /// - `encryption_status: "none"`
    /// - `message_type: "text"`
    ///
    /// ## Security Notes
    ///
    /// - Message is **plaintext** - encrypt before storage/network
    /// - No validation on sender/recipient (done at API layer)
    /// - No size limit on content (enforce at API layer)
    /// - Timestamp is precise (add jitter before sending)
    ///
    /// ## Example
    ///
    /// ```rust
    /// let message = Message::new(
    ///     "Alice".to_string(),
    ///     "Bob".to_string(),
    ///     "Secret meeting at noon".to_string(),
    /// );
    ///
    /// // Encrypt before sending
    /// // let encrypted = encryption.encrypt_message(&message, &bob_pubkey)?;
    /// ```
    pub fn new(sender: String, recipient: String, content: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender,
            recipient,
            content,
            content_type: "text/plain".to_string(),
            content_bytes: None,
            content_cid: None,
            metadata: None,
            timestamp: {
                let base = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let jitter = rand::random::<u64>() % TIMESTAMP_JITTER_RANGE;
                base + jitter - (TIMESTAMP_JITTER_RANGE / 2)
            },
            encrypted: false,
            message_type: "text".to_string(),
            encryption_status: "none".to_string(),
            hops: 0,
            ttl: crate::core::reliability::DEFAULT_MESSAGE_TTL,
            vector_clock: crate::core::vector_clock::VectorClock::new(),
            compressed: false,
            compression_algo: None,
        }
    }

    /// Create a new message with custom type
    ///
    /// ## Parameters
    ///
    /// - `sender`: Sender's node ID
    /// - `recipient`: Recipient's node ID
    /// - `content`: Message content
    /// - `message_type`: Type of message (Text, Binary, Control, etc.)
    ///
    /// ## Returns
    ///
    /// A new `Message` instance with specified type
    ///
    /// ## Example
    ///
    /// ```rust
    /// let control_msg = Message::with_type(
    ///     "System".to_string(),
    ///     "All".to_string(),
    ///     "SHUTDOWN".to_string(),
    ///     MessageType::Control,
    /// );
    /// ```
    pub fn with_type(
        sender: String,
        recipient: String,
        content: String,
        message_type: MessageType,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            sender,
            recipient,
            content,
            content_type: "text/plain".to_string(),
            content_bytes: None,
            content_cid: None,
            metadata: None,
            timestamp: {
                let base = std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap_or_default()
                    .as_secs();
                let jitter = rand::random::<u64>() % TIMESTAMP_JITTER_RANGE;
                base + jitter - (TIMESTAMP_JITTER_RANGE / 2)
            },
            encrypted: false,
            message_type: format!("{:?}", message_type),
            encryption_status: "none".to_string(),
            hops: 0,
            ttl: crate::core::reliability::DEFAULT_MESSAGE_TTL,
            vector_clock: crate::core::vector_clock::VectorClock::new(),
            compressed: false,
            compression_algo: None,
        }
    }

    /// Mark message as encrypted
    ///
    /// **Usage:** Call after encryption to track status
    /// **Security:** Does NOT actually encrypt - just updates status field
    pub fn mark_encrypted(&mut self) {
        self.encrypted = true;
        self.encryption_status = "full".to_string();
    }

    /// Check if message is a broadcast
    ///
    /// **Returns:** `true` if recipient is "broadcast" or empty
    pub fn is_broadcast(&self) -> bool {
        self.recipient == "broadcast" || self.recipient.is_empty()
    }

    /// Get message age in seconds
    ///
    /// **Returns:** Seconds since message was created
    /// **Usage:** Expiry checks, display ("5 minutes ago")
    pub fn age(&self) -> u64 {
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        now.saturating_sub(self.timestamp)
    }

    /// Check if message has expired
    ///
    /// ## Parameters
    ///
    /// - `ttl`: Time-to-live in seconds
    ///
    /// **Returns:** `true` if message age exceeds TTL
    /// **Usage:** Auto-delete old messages
    pub fn is_expired(&self, ttl: u64) -> bool {
        self.age() >= ttl
    }

    pub fn with_binary(
        sender: String,
        recipient: String,
        content_bytes: Vec<u8>,
        content_type: String,
    ) -> Self {
        let mut msg = Self::new(sender, recipient, String::new());
        msg.content_type = content_type;
        msg.content_bytes = Some(content_bytes);
        msg.message_type = "binary".to_string();
        msg
    }

    pub fn with_cid(
        sender: String,
        recipient: String,
        cid: String,
        content_type: String,
    ) -> Self {
        let mut msg = Self::new(sender, recipient, String::new());
        msg.content_type = content_type;
        msg.content_cid = Some(cid);
        msg.message_type = "binary".to_string();
        msg
    }

    pub fn inline_bytes(&self) -> Option<&[u8]> {
        self.content_bytes.as_deref()
    }

    pub fn has_cid(&self) -> bool {
        self.content_cid.is_some()
    }

    pub fn compress(&mut self, codec: crate::core::compression::Codec) -> Result<(), anyhow::Error> {
        if self.compressed {
            return Ok(());
        }
        if let Some(ref bytes) = self.content_bytes {
            let compressed = crate::core::compression::compress(bytes, codec)?;
            self.content_bytes = Some(compressed);
            self.compressed = true;
            self.compression_algo = Some(format!("{:?}", codec));
            return Ok(());
        }
        if !self.content.is_empty() {
            let compressed = crate::core::compression::compress(self.content.as_bytes(), codec)?;
            self.content_bytes = Some(compressed);
            self.compressed = true;
            self.compression_algo = Some(format!("{:?}", codec));
            return Ok(());
        }
        Ok(())
    }

    pub fn decompress(&mut self) -> Result<(), anyhow::Error> {
        if !self.compressed {
            return Ok(());
        }
        let codec = self.compression_algo.as_deref().map(|s| match s {
            "Zstd" => crate::core::compression::Codec::Zstd,
            "Lz4" => crate::core::compression::Codec::Lz4,
            _ => crate::core::compression::Codec::None,
        }).unwrap_or(crate::core::compression::Codec::None);

        if let Some(ref bytes) = self.content_bytes {
            let decompressed = crate::core::compression::decompress(bytes, codec)?;
            if self.message_type == "text" || self.content.is_empty() {
                self.content = String::from_utf8_lossy(&decompressed).to_string();
            }
            self.content_bytes = Some(decompressed);
            self.compressed = false;
            self.compression_algo = None;
        }
        Ok(())
    }
}

impl PartialEq for Message {
    /// Compare messages by ID only
    ///
    /// **Rationale:** ID is unique, comparing content is expensive
    /// **Usage:** Deduplication, equality checks
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Message {}

impl std::hash::Hash for Message {
    /// Hash by ID only
    ///
    /// **Usage:** Store in HashSet/HashMap
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl std::fmt::Display for Message {
    /// Format message for display/logging
    ///
    /// **Output:** `Message[id=..., sender=..., content_preview=...]`
    /// **Security:** Truncates content to 50 chars (prevent log flooding)
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let content_preview = if self.content.len() > 50 {
            format!("{}...", &self.content[..50])
        } else {
            self.content.clone()
        };

        write!(
            f,
            "Message[id={}, sender={}, recipient={}, content={}, type={}]",
            self.id, self.sender, self.recipient, content_preview, self.message_type
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_creation() {
        let message = Message::new("Alice".to_string(), "Bob".to_string(), "Hello!".to_string());

        assert_eq!(message.sender, "Alice");
        assert_eq!(message.recipient, "Bob");
        assert_eq!(message.content, "Hello!");
        assert_eq!(message.encrypted, false);
        assert_eq!(message.encryption_status, "none");
        assert_eq!(message.message_type, "text");
    }

    #[test]
    fn test_message_with_type() {
        let message = Message::with_type(
            "System".to_string(),
            "All".to_string(),
            "SHUTDOWN".to_string(),
            MessageType::Control,
        );

        assert_eq!(message.message_type, "Control");
    }

    #[test]
    fn test_message_mark_encrypted() {
        let mut message =
            Message::new("Alice".to_string(), "Bob".to_string(), "Secret".to_string());

        assert_eq!(message.encrypted, false);

        message.mark_encrypted();

        assert_eq!(message.encrypted, true);
        assert_eq!(message.encryption_status, "full");
    }

    #[test]
    fn test_message_is_broadcast() {
        let broadcast = Message::new(
            "Alice".to_string(),
            "broadcast".to_string(),
            "Hello all!".to_string(),
        );
        assert!(broadcast.is_broadcast());

        let empty_recipient =
            Message::new("Alice".to_string(), "".to_string(), "Hello!".to_string());
        assert!(empty_recipient.is_broadcast());

        let direct = Message::new("Alice".to_string(), "Bob".to_string(), "Hey".to_string());
        assert!(!direct.is_broadcast());
    }

    #[test]
    fn test_message_age() {
        let message = Message::new("Alice".to_string(), "Bob".to_string(), "Test".to_string());

        // Message just created, age should be 0 or 1 second
        assert!(message.age() <= 60);
    }

    #[test]
    fn test_message_expiry() {
        let message = Message::new("Alice".to_string(), "Bob".to_string(), "Test".to_string());

        // Not expired with 1 hour TTL
        assert!(!message.is_expired(3600));

        // Expired with 0 TTL
        assert!(message.is_expired(0));
    }

    #[test]
    fn test_message_equality() {
        let message1 = Message::new("Alice".to_string(), "Bob".to_string(), "Hello!".to_string());

        let message2 = Message::new("Alice".to_string(), "Bob".to_string(), "Hello!".to_string());

        // Different IDs, not equal
        assert_ne!(message1, message2);

        // Same ID (clone), equal
        let message1_clone = message1.clone();
        assert_eq!(message1, message1_clone);
    }

    #[test]
    fn test_message_display() {
        let message = Message::new(
            "Alice".to_string(),
            "Bob".to_string(),
            "This is a very long message that should be truncated in display format".to_string(),
        );

        let display = format!("{}", message);

        assert!(display.contains("Message["));
        assert!(display.contains("Alice"));
        assert!(display.contains("Bob"));
        assert!(display.contains("...")); // Truncated
        assert!(display.len() < 200); // Reasonable length
    }

    #[test]
    fn test_message_hash() {
        use std::collections::HashSet;

        let message1 = Message::new("Alice".to_string(), "Bob".to_string(), "Hello!".to_string());

        let message2 = message1.clone(); // Same ID
        let message3 = Message::new(
            "Alice".to_string(),
            "Bob".to_string(),
            "Different!".to_string(),
        );

        let mut set = HashSet::new();
        set.insert(message1.clone());
        set.insert(message2.clone()); // Should not add (same ID)
        set.insert(message3);

        assert_eq!(set.len(), 2); // Only 2 unique messages
    }

    #[test]
    fn test_compress_roundtrip() {
        use crate::core::compression::Codec;

        let mut msg = Message::new(
            "Alice".to_string(),
            "Bob".to_string(),
            "x".repeat(1024),
        );
        msg.compress(Codec::Zstd).unwrap();
        assert!(msg.compressed);
        assert!(msg.content_bytes.is_some());

        msg.decompress().unwrap();
        assert!(!msg.compressed);
        assert_eq!(msg.content, "x".repeat(1024));
    }

    #[test]
    fn test_compress_binary() {
        use crate::core::compression::Codec;

        let data = vec![0xAB; 2048];
        let mut msg = Message::with_binary(
            "Alice".to_string(),
            "Bob".to_string(),
            data.clone(),
            "application/octet-stream".to_string(),
        );
        msg.compress(Codec::Lz4).unwrap();
        assert!(msg.compressed);

        msg.decompress().unwrap();
        assert_eq!(msg.content_bytes, Some(data));
    }
}
