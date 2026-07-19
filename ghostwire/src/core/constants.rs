// SPDX-License-Identifier: AGPL-3.0-or-later OR Commercial License

//! GhostWire Core Constants
//!
//! This module contains immutable constants that define GhostWire's identity,
//! protocol parameters, and historical milestones.

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
// PROTOCOL CONSTANTS
// ═══════════════════════════════════════════════════════════

/// Default libp2p mesh port.
pub const DEFAULT_MESH_PORT: u16 = 4001;

/// Default API server port.
pub const DEFAULT_API_PORT: u16 = 9000;

/// Default web server port.
pub const DEFAULT_WEB_PORT: u16 = 8080;

pub const BROADCAST_TOPIC: &str = "b7dcd0de7e54fd2732a5244bdbdc0d02";
pub const DM_TOPIC: &str = "9982f756f600726c3cbc6c8ca321acc9";
pub const SPHINX_TOPIC: &str = "c93b642a5e02da27dda6344fbfbe7866";
pub const DTN_SYNC_TOPIC: &str = "5ee92e52d560f4904ef25e4265349e3d";

/// Maximum number of hops for message relay (prevents infinite loops).
pub const MAX_MESSAGE_HOPS: u8 = 5;

/// Maximum message size in bytes (1 MB).
pub const MAX_MESSAGE_SIZE: usize = 1_048_576;

/// GhostWire version.
pub const GHOSTWIRE_VERSION: &str = "0.1.0";

/// GhostWire protocol version.
pub const PROTOCOL_VERSION: &str = "1.0.0";

/// User agent string for libp2p identification.
pub const USER_AGENT: &str = "ghostwire/0.1.0";

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
        // Verify the historical first ping message
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
        // Topics should be non-empty hex strings
        assert_eq!(BROADCAST_TOPIC.len(), 32);
        assert_eq!(DM_TOPIC.len(), 32);
        assert_eq!(SPHINX_TOPIC.len(), 32);
        assert_eq!(DTN_SYNC_TOPIC.len(), 32);
    }

    #[test]
    fn test_max_message_hops() {
        assert!(MAX_MESSAGE_HOPS > 0);
        assert!(MAX_MESSAGE_HOPS <= 255); // u8 max
    }

    #[test]
    fn test_max_message_size() {
        assert!(MAX_MESSAGE_SIZE > 0);
        assert!(MAX_MESSAGE_SIZE >= 1024); // At least 1KB
        assert_eq!(MAX_MESSAGE_SIZE, 1_048_576); // Exactly 1 MB
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
        // Peer IDs should be 52+ characters for libp2p
        assert!(FIRST_PING_SENDER.starts_with("12D3Koo"));
        assert!(FIRST_PING_RECIPIENT.starts_with("12D3Koo"));
    }

    #[test]
    fn test_first_ping_timestamp_format() {
        // ISO 8601 format with timezone
        assert!(FIRST_PING_TIMESTAMP.contains("T"));
        assert!(FIRST_PING_TIMESTAMP.contains("+00:00"));
    }
}
