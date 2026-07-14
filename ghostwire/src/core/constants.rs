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
