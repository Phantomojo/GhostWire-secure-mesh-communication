//! # Network Modules
//!
//! This directory organizes GhostWire's networking functionality:
//!
//! ## Modules
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`briar`] | Briar protocol implementation |
//! | [`mesh`] | Main mesh networking logic |
//! | [`mesh_health`] | Mesh health monitoring |
//! | [`monitor`] | Network monitoring |
//! | [`presence_obfuscation`] | Presence obfuscation |
//! | [`reticulum`] | Reticulum protocol |
//! | [`transport`] | Transport abstraction layer |
//! | [`quic_transport`] | QUIC transport implementation |
//! | [`stealth_tcp`] | Stealth TCP transport |
//!
//! ## Architecture
//!
//! ```text
//! Application Layer
//!      ↓
//! Mesh Logic (mesh.rs)
//!      ↓
//! Transport Abstraction (transport.rs)
//!      ↓
//! Transport Implementations (quic_transport, stealth_tcp, etc.)
//! ```

pub mod presence_obfuscation;

// Re-exports for backward compatibility
#[cfg(feature = "briar")]
pub use crate::core::briar;
pub use crate::core::mesh;
pub use crate::core::mesh_health;
pub use crate::core::monitor;
#[cfg(feature = "reticulum")]
pub use crate::core::reticulum;
pub use crate::core::transport;
pub use crate::core::quic_transport;
pub use crate::core::stealth_tcp;
