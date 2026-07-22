// SPDX-License-Identifier: AGPL-3.0-or-later OR Commercial License

//! GhostWire Core Constants
//!
//! This module contains immutable constants that define GhostWire's identity,
//! protocol parameters, and historical milestones.
//!
//! ## Organization
//!
//! - [`time`] — Time constants (seconds, minutes, hours, etc.)
//! - [`buffer`] — Buffer size constants
//! - [`network`] — Network and protocol constants
//! - [`security`] — Security and crypto constants
//! - [`mesh`] — Mesh networking constants
//! - [`history`] — Historical first-ping constants

// ═══════════════════════════════════════════════════════════
// ⏱️ TIME CONSTANTS
// ═══════════════════════════════════════════════════════════
/// Time constants in seconds
pub mod time {
    /// One second in seconds
    pub const SECOND: u64 = 1;
    /// One minute in seconds
    pub const MINUTE: u64 = 60;
    /// One hour in seconds
    pub const HOUR: u64 = 3600;
    /// One day in seconds
    pub const DAY: u64 = 86400;
    /// One week in seconds
    pub const WEEK: u64 = 604800;
    /// One month (30 days) in seconds
    pub const MONTH: u64 = 30 * DAY;

    // Common timeouts (in seconds)
    /// Default connection timeout
    pub const CONNECTION_TIMEOUT: u64 = 30;
    /// Default heartbeat interval
    pub const HEARTBEAT_INTERVAL: u64 = 60;
    /// Default lockout duration (5 minutes)
    pub const DEFAULT_LOCKOUT: u64 = 5 * MINUTE;
    /// Blacklist TTL (1 hour)
    pub const BLACKLIST_TTL: u64 = HOUR;
    /// Key rotation interval (7 days)
    pub const KEY_ROTATION_INTERVAL: u64 = 7 * DAY;
    /// Anonymous intro expiry (48 hours)
    pub const ANON_INTRO_EXPIRY: u64 = 48 * HOUR;
    /// Dead man's switch interval (24 hours)
    pub const DEAD_MAN_INTERVAL: u64 = 24 * HOUR;
}

// ═══════════════════════════════════════════════════════════
// 📦 BUFFER SIZE CONSTANTS
// ═══════════════════════════════════════════════════════════
/// Buffer and frame size constants
pub mod buffer {
    /// Small buffer size (256 bytes)
    pub const SMALL: usize = 256;
    /// Default/medium buffer size (4096 bytes)
    pub const DEFAULT: usize = 4096;
    /// Large buffer size (65,536 bytes)
    pub const LARGE: usize = 65_536;
    /// Maximum buffer size (1 MB)
    pub const MAX: usize = 1024 * 1024;
    /// Block size for storage (256 KB)
    pub const BLOCK_SIZE: usize = 256 * 1024;
    /// Wire protocol frame size
    pub const WIRE_FRAME: usize = LARGE;
    /// Traffic armor padding block size
    pub const PAD_BLOCK: usize = 512;
    /// Maximum padded message size
    pub const MAX_PADDED: usize = 65_536;
}

// ═══════════════════════════════════════════════════════════
// 🌐 NETWORK CONSTANTS
// ═══════════════════════════════════════════════════════════
/// Network and protocol constants
pub mod network {
    use super::time::*;

    /// Maximum port number
    pub const MAX_PORT: u16 = 65535;
    /// Default libp2p mesh port
    pub const DEFAULT_MESH_PORT: u16 = 4001;
    /// Default API server port
    pub const DEFAULT_API_PORT: u16 = 9000;
    /// Default web server port
    pub const DEFAULT_WEB_PORT: u16 = 8080;
    /// Default connection timeout in seconds
    pub const DEFAULT_TIMEOUT_SECS: u64 = CONNECTION_TIMEOUT;
    /// Maximum number of peers
    pub const MAX_PEERS: usize = 50;
    /// Maximum connections per IP
    pub const MAX_CONNECTIONS_PER_IP: usize = 10;
    /// Maximum messages per minute
    pub const MAX_MESSAGES_PER_MINUTE: usize = 100;
    /// Mailbox maximum size
    pub const MAX_MAILBOX_SIZE: usize = 100;
}

// ═══════════════════════════════════════════════════════════
// 🔒 SECURITY CONSTANTS
// ═══════════════════════════════════════════════════════════
/// Security and cryptography constants
pub mod security {
    use super::time::*;

    /// Default CRL TTL (30 days)
    pub const DEFAULT_CRL_TTL: u64 = 30 * DAY;
    /// CRL garbage collection interval (1 hour)
    pub const CRL_GC_INTERVAL_SECS: u64 = HOUR;
    /// Default ACK timeout (5 seconds)
    pub const DEFAULT_ACK_TIMEOUT_SECS: u64 = 5;
    /// Default initial retry delay (1 second)
    pub const DEFAULT_RETRY_DELAY_SECS: u64 = 1;
    /// Default maximum retry delay (30 seconds)
    pub const DEFAULT_MAX_RETRY_DELAY_SECS: u64 = 30;
    /// Default maximum retries
    pub const DEFAULT_MAX_RETRIES: u8 = 5;
    /// Default backoff multiplier
    pub const DEFAULT_BACKOFF_MULTIPLIER: f64 = 2.0;
    /// Default maximum pending ACKs
    pub const DEFAULT_MAX_PENDING_ACKS: usize = 1000;
    /// Default session timeout (1 hour)
    pub const DEFAULT_SESSION_TIMEOUT_HOURS: u64 = 1;
    /// Maximum wantlist size per peer
    pub const MAX_WANTLIST_PER_PEER: usize = 16;
    /// Rate limit: max connections per minute
    pub const MAX_CONN_PER_MIN: usize = 10;
    /// Maximum handshake attempts
    pub const MAX_HANDSHAKE_ATTEMPTS: usize = 3;
    /// Maximum skip for Double Ratchet
    pub const MAX_SKIP: u32 = 200;
    /// Default key size (256 bits)
    pub const KEY_SIZE: usize = 32;
}

// ═══════════════════════════════════════════════════════════
// 📡 MESH NETWORKING CONSTANTS
// ═══════════════════════════════════════════════════════════
/// Mesh networking constants
pub mod mesh {
    use super::buffer;
    use super::network::*;
    use super::time::*;

    /// Maximum message hops
    pub const MAX_HOPS: u8 = 5;
    /// Maximum DTN hops
    pub const DTN_MAX_HOPS: u8 = 8;
    /// Maximum message size
    pub const MAX_MESSAGE_SIZE: usize = buffer::MAX;
    /// In-memory message cache size
    pub const IN_MEMORY_MSGS_MAX: usize = 5000;
    /// Seen messages cache size
    pub const SEEN_MSGS_MAX: usize = 10000;
    /// DTN message TTL (1 hour)
    pub const DTN_MESSAGE_TTL_SECS: u64 = HOUR;
    /// Proof of work difficulty (bits)
    pub const POW_DIFFICULTY_BITS: u8 = 4;
    /// Sliding window quota limit
    pub const SLIDING_WINDOW_QUOTA: usize = 2;
    /// Blacklist maximum entries
    pub const BLACKLIST_MAX_ENTRIES: usize = 10000;
}

// ═══════════════════════════════════════════════════════════
// 📜 PROTOCOL STRINGS
// ═══════════════════════════════════════════════════════════
/// Protocol identifiers and strings
pub mod protocol {
    /// GhostWire version
    pub const VERSION: &str = "0.1.0";
    /// Protocol version
    pub const PROTOCOL_VERSION: &str = "1.0.0";
    /// User agent string
    pub const USER_AGENT: &str = "ghostwire/0.1.0";
    /// Libp2p identify protocol
    pub const IDENTIFY_PROTOCOL: &str = "/ghostwire/mesh/2.0.0";
    /// Kademlia protocol path
    pub const KAD_PROTOCOL: &str = "/ghostwire/kad/shard/global";
    /// Chat protocol path
    pub const CHAT_PROTOCOL: &str = "/ghostwire/chat/1.0.0";
    /// DTN bundle protocol
    pub const DTN_PROTOCOL: &str = "/ghostwire/dtn/1.0.0";
    /// TLS hostname for QUIC
    pub const TLS_HOSTNAME: &str = "ghostwire.local";
    /// TOFU fingerprint encryption domain
    pub const TOFU_SALT: &str = "GHOSTWIRE-TOFU-FP-SALT";
    /// Recovery KDF domain
    pub const RECOVERY_KDF_DOMAIN: &str = "ghostwire-recovery-v1";
}

// ═══════════════════════════════════════════════════════════
// 🎯 TOPIC IDENTIFIERS
// ═══════════════════════════════════════════════════════════
/// GossipSub topic identifiers (anonymized hashes)
pub mod topics {
    /// Broadcast messages topic
    pub const BROADCAST: &str = "b7dcd0de7e54fd2732a5244bdbdc0d02";
    /// Direct messages topic
    pub const DM: &str = "9982f756f600726c3cbc6c8ca321acc9";
    /// Sphinx routing topic
    pub const SPHINX: &str = "c93b642a5e02da27dda6344fbfbe7866";
    /// DTN sync topic
    pub const DTN_SYNC: &str = "5ee92e52d560f4904ef25e4265349e3d";
    /// Default mesh messages topic
    pub const MESSAGES: &str = "ghostwire-messages";
}

// ═══════════════════════════════════════════════════════════
// 🎯 FIRST PING — April 5, 2026
// ═══════════════════════════════════════════════════════════
//
// At 22:50:37.125669222 UTC, GhostWire achieved its first
// cross-machine communication. A message traveled from node A
// to node B via libp2p mesh networking — no central server,
// just peer-to-peer gossipsub routing.
//
// Node A:      <PEER_ID_A>
// Node B:      <PEER_ID_B>
// Message: "FINAL TEST - should work now!"
// Protocol: libp2p/gossipsub
// Topic: ghostwire-messages
// Latency: ~18ms
//
// This is GhostWire's equivalent of ARPANET's "LO" (Oct 29, 1969).
// See docs/FIRST_PING.md for the full historical record.
// ═══════════════════════════════════════════════════════════

/// The exact UTC timestamp of GhostWire's first cross-machine ping.
/// Format: ISO 8601 with nanosecond precision.
pub const FIRST_PING_TIMESTAMP: &str = "2026-04-05T22:50:37.125669222+00:00";

/// The Unix epoch seconds of the first ping message content.
pub const FIRST_PING_EPOCH: u64 = 1775429437;

/// The content of the first message ever sent through GhostWire.
pub const FIRST_PING_MESSAGE: &str = "FINAL TEST - should work now!";

/// The PeerId of the sending node (omen).
pub const FIRST_PING_SENDER: &str = "12D3KooWGPrkHB4STH6kyNcQtzW4aMgRkRX7zkdaqncrrdY1A1sf";

/// The PeerId of the receiving node (stratsytem).
pub const FIRST_PING_RECIPIENT: &str = "12D3KooWHUPbjbtx3qidqRRw8nU2zqALB9fEUvt9dZGs4EKNwEVK";

/// The libp2p gossipsub topic used for the first ping.
pub const FIRST_PING_TOPIC: &str = "ghostwire-messages";

/// The message UUID of the first ping.
pub const FIRST_PING_MESSAGE_ID: &str = "eab6bb98-a168-43c9-a0a9-a009946830bc";

// ═══════════════════════════════════════════════════════════
// DEPRECATED: Legacy flat constants (use module-based constants above)
// ═══════════════════════════════════════════════════════════
/// Default libp2p mesh port. Use: `network::DEFAULT_MESH_PORT`
#[deprecated(since = "0.2.0", note = "Use constants::network::DEFAULT_MESH_PORT")]
pub const DEFAULT_MESH_PORT: u16 = network::DEFAULT_MESH_PORT;

/// Default API server port. Use: `network::DEFAULT_API_PORT`
#[deprecated(since = "0.2.0", note = "Use constants::network::DEFAULT_API_PORT")]
pub const DEFAULT_API_PORT: u16 = network::DEFAULT_API_PORT;

/// Default web server port. Use: `network::DEFAULT_WEB_PORT`
#[deprecated(since = "0.2.0", note = "Use constants::network::DEFAULT_WEB_PORT")]
pub const DEFAULT_WEB_PORT: u16 = network::DEFAULT_WEB_PORT;

/// Broadcast topic. Use: `topics::BROADCAST`
#[deprecated(since = "0.2.0", note = "Use constants::topics::BROADCAST")]
pub const BROADCAST_TOPIC: &str = topics::BROADCAST;

/// Direct message topic. Use: `topics::DM`
#[deprecated(since = "0.2.0", note = "Use constants::topics::DM")]
pub const DM_TOPIC: &str = topics::DM;

/// Sphinx routing topic. Use: `topics::SPHINX`
#[deprecated(since = "0.2.0", note = "Use constants::topics::SPHINX")]
pub const SPHINX_TOPIC: &str = topics::SPHINX;

/// DTN sync topic. Use: `topics::DTN_SYNC`
#[deprecated(since = "0.2.0", note = "Use constants::topics::DTN_SYNC")]
pub const DTN_SYNC_TOPIC: &str = topics::DTN_SYNC;

/// Maximum message hops. Use: `mesh::MAX_HOPS`
#[deprecated(since = "0.2.0", note = "Use constants::mesh::MAX_HOPS")]
pub const MAX_MESSAGE_HOPS: u8 = mesh::MAX_HOPS;

/// Maximum message size. Use: `mesh::MAX_MESSAGE_SIZE`
#[deprecated(since = "0.2.0", note = "Use constants::mesh::MAX_MESSAGE_SIZE")]
pub const MAX_MESSAGE_SIZE: usize = mesh::MAX_MESSAGE_SIZE;

/// GhostWire version. Use: `protocol::VERSION`
#[deprecated(since = "0.2.0", note = "Use constants::protocol::VERSION")]
pub const GHOSTWIRE_VERSION: &str = protocol::VERSION;

/// GhostWire protocol version. Use: `protocol::PROTOCOL_VERSION`
#[deprecated(since = "0.2.0", note = "Use constants::protocol::PROTOCOL_VERSION")]
pub const PROTOCOL_VERSION: &str = protocol::PROTOCOL_VERSION;

/// User agent string. Use: `protocol::USER_AGENT`
#[deprecated(since = "0.2.0", note = "Use constants::protocol::USER_AGENT")]
pub const USER_AGENT: &str = protocol::USER_AGENT;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_first_ping_constants() {
        assert!(!FIRST_PING_TIMESTAMP.is_empty());
        assert!(FIRST_PING_EPOCH > 0);
        assert!(!FIRST_PING_MESSAGE.is_empty());
        assert!(!FIRST_PING_SENDER.is_empty());
        assert!(!FIRST_PING_RECIPIENT.is_empty());
        assert!(!FIRST_PING_TOPIC.is_empty());
        assert!(!FIRST_PING_MESSAGE_ID.is_empty());
    }

    #[test]
    fn test_first_ping_message_content() {
        assert_eq!(FIRST_PING_MESSAGE, "FINAL TEST - should work now!");
    }

    #[test]
    fn test_protocol_constants() {
        assert!(DEFAULT_MESH_PORT > 0);
        assert!(DEFAULT_API_PORT > 0);
        assert!(DEFAULT_WEB_PORT > 0);
        assert_ne!(DEFAULT_MESH_PORT, DEFAULT_API_PORT);
        assert_ne!(DEFAULT_API_PORT, DEFAULT_WEB_PORT);
        assert_ne!(DEFAULT_MESH_PORT, DEFAULT_WEB_PORT);
    }

    #[test]
    fn test_topics_are_valid_hashes() {
        assert_eq!(BROADCAST_TOPIC.len(), 32);
        assert_eq!(DM_TOPIC.len(), 32);
        assert_eq!(SPHINX_TOPIC.len(), 32);
        assert_eq!(DTN_SYNC_TOPIC.len(), 32);
    }

    #[test]
    fn test_max_message_hops() {
        assert!(MAX_MESSAGE_HOPS > 0);
        assert!(MAX_MESSAGE_HOPS <= 255);
    }

    #[test]
    fn test_max_message_size() {
        assert!(MAX_MESSAGE_SIZE > 0);
        assert!(MAX_MESSAGE_SIZE >= 1024);
        assert_eq!(MAX_MESSAGE_SIZE, 1_048_576);
    }

    #[test]
    fn test_version_constants() {
        assert!(!GHOSTWIRE_VERSION.is_empty());
        assert!(!PROTOCOL_VERSION.is_empty());
        assert!(!USER_AGENT.is_empty());
    }

    #[test]
    fn test_user_agent_format() {
        assert!(USER_AGENT.starts_with("ghostwire/"));
    }

    #[test]
    fn test_first_ping_peer_ids_are_valid() {
        assert!(FIRST_PING_SENDER.starts_with("12D3Koo"));
        assert!(FIRST_PING_RECIPIENT.starts_with("12D3Koo"));
    }

    #[test]
    fn test_first_ping_timestamp_format() {
        assert!(FIRST_PING_TIMESTAMP.contains("T"));
        assert!(FIRST_PING_TIMESTAMP.contains("+00:00"));
    }

    // Tests for new module-based constants
    #[test]
    fn test_time_constants() {
        assert_eq!(time::MINUTE, 60);
        assert_eq!(time::HOUR, 3600);
        assert_eq!(time::DAY, 86400);
        assert_eq!(time::WEEK, 604800);
    }

    #[test]
    fn test_buffer_constants() {
        assert_eq!(buffer::SMALL, 256);
        assert_eq!(buffer::DEFAULT, 4096);
        assert_eq!(buffer::LARGE, 65_536);
        assert_eq!(buffer::MAX, 1024 * 1024);
    }

    #[test]
    fn test_network_constants() {
        assert_eq!(network::MAX_PORT, 65535);
        assert_eq!(network::DEFAULT_MESH_PORT, 4001);
        assert_eq!(network::MAX_PEERS, 50);
    }

    #[test]
    fn test_security_constants() {
        assert_eq!(security::MAX_SKIP, 200);
        assert_eq!(security::DEFAULT_MAX_RETRIES, 5);
        assert_eq!(security::MAX_CONN_PER_MIN, 10);
    }

    #[test]
    fn test_mesh_constants() {
        assert_eq!(mesh::MAX_HOPS, 5);
        assert_eq!(mesh::DTN_MAX_HOPS, 8);
        assert_eq!(mesh::IN_MEMORY_MSGS_MAX, 5000);
    }

    #[test]
    fn test_protocol_constants_module() {
        assert_eq!(protocol::VERSION, "0.1.0");
        assert!(protocol::IDENTIFY_PROTOCOL.starts_with("/ghostwire"));
        assert_eq!(protocol::TLS_HOSTNAME, "ghostwire.local");
    }

    #[test]
    fn test_topics_constants_module() {
        assert_eq!(topics::BROADCAST.len(), 32);
        assert_eq!(topics::MESSAGES, "ghostwire-messages");
    }
}
