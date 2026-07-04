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

/// Type of message being sent
///
/// **Security Note:** Message type is visible in plaintext for routing purposes.
/// Sensitive message types (e.g., `Control`, `Handshake`) should be encrypted
/// at a higher layer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MessageType {
    /// Standard text message (default)
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

impl Default for MessageType {
    fn default() -> Self {
        Self::Text
    }
}

/// Encryption status of the message
///
/// **Security Note:** This is a **local state** indicator, not a security guarantee.
/// Always verify encryption at the transport/storage layer.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum EncryptionStatus {
    /// Message is not encrypted (plaintext in memory)
    None,
    /// Message is being encrypted (in transit)
    InProgress,
    /// Message is fully encrypted (storage or network)
    Full,
    /// Message encryption failed (error state)
    Failed,
}

impl Default for EncryptionStatus {
    fn default() -> Self {
        Self::None
    }
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
    /// Unique message identifier (UUID v4)
    ///
    /// **Purpose:** Deduplication, tracking, reference
    /// **Security:** Random (unpredictable), prevents enumeration
    pub id: Uuid,

    /// Sender's node ID (public key hash)
    ///
    /// **Purpose:** Routing, display, verification
    /// **Security:** Plaintext in memory, encrypted in transit
    pub sender: String,

    /// Recipient's node ID (public key hash)
    ///
    /// **Purpose:** Routing, access control
    /// **Security:** Plaintext in memory, encrypted in transit
    /// **Special Values:** "broadcast" for broadcast messages
    pub recipient: String,

    /// Message content (the actual message)
    ///
    /// **Purpose:** User data
    /// **Security:** ENCRYPT before storage/network transmission
    /// **Size Limit:** 10KB recommended (enforced at API layer)
    pub content: String,

    /// Unix timestamp (seconds since 1970-01-01 UTC)
    ///
    /// **Purpose:** Ordering, display, expiry
    /// **Security:** Add jitter (±5 seconds) before sending to prevent traffic analysis
    /// **Precision:** Seconds (sufficient for messaging)
    pub timestamp: u64,

    /// Legacy encryption flag (deprecated)
    ///
    /// **Status:** DEPRECATED - use `encryption_status` instead
    /// **Backwards Compatibility:** Kept for old code
    /// **Value:** `false` for plaintext, `true` for encrypted
    pub encrypted: bool,

    /// Type of message
    ///
    /// **Purpose:** Routing, handling, display
    /// **Security:** Visible in plaintext for routing
    pub message_type: String,

    /// Current encryption status
    ///
    /// **Purpose:** Track encryption state
    /// **Security:** Local state indicator, not a security guarantee
    pub encryption_status: String,

    /// Hop count for multi-hop relay
    ///
    /// **Purpose:** Track how many hops this message has traversed
    /// **Security:** Prevents infinite loops (max 8 hops via reliability::MAX_HOPS)
    /// **Default:** 0 (originating node)
    pub hops: u8,

    /// Time-to-live in seconds
    ///
    /// Messages are dropped when TTL reaches 0. Decremented on each relay hop.
    pub ttl: u64,

    pub vector_clock: crate::core::vector_clock::VectorClock,
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
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            encrypted: false,
            message_type: "text".to_string(),
            encryption_status: "none".to_string(),
            hops: 0,
            ttl: crate::core::reliability::DEFAULT_MESSAGE_TTL,
            vector_clock: crate::core::vector_clock::VectorClock::new(),
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
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
            encrypted: false,
            message_type: format!("{:?}", message_type),
            encryption_status: "none".to_string(),
            hops: 0,
            ttl: crate::core::reliability::DEFAULT_MESSAGE_TTL,
            vector_clock: crate::core::vector_clock::VectorClock::new(),
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
        assert!(message.age() <= 1);
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
}
