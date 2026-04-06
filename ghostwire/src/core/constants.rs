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
// cross-machine communication. A message traveled from omen
// to stratsytem via libp2p mesh networking — no central server,
// no internet, just pure P2P.
//
// Node A (omen):      12D3KooWGPrkHB4STH6kyNcQtzW4aMgRkRX7zkdaqncrrdY1A1sf
// Node B (stratsytem): 12D3KooWHUPbjbtx3qidqRRw8nU2zqALB9fEUvt9dZGs4EKNwEVK
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

/// Broadcast topic name for gossipsub.
pub const BROADCAST_TOPIC: &str = "ghostwire-messages";

/// DM topic prefix for direct peer-to-peer messaging.
pub const DM_TOPIC_PREFIX: &str = "ghostwire-dm-";

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
