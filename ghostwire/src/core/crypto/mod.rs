//! # Cryptographic Modules
//!
//! This directory organizes GhostWire's cryptographic functionality:
//!
//! ## Modules
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`armor`] | Traffic obfuscation and armor |
//! | [`dht_signatures`] | DHT request signing |
//! | [`double_ratchet`] | Double Ratchet KDF |
//! | [`encryption`] | Main encryption manager |
//! | [`typed_crypto`] | Type-safe cryptographic primitives |
//!
//! ## Usage
//!
//! ```rust
//! use crate::core::crypto::encryption::EncryptionManager;
//! use crate::core::crypto::double_ratchet::DoubleRatchet;
//! ```

// Re-exports for backward compatibility
// These allow `use crate::core::encryption` to continue working
pub use crate::core::armor;
pub use crate::core::dht_signatures;
pub use crate::core::double_ratchet;
pub use crate::core::encryption;
pub use crate::core::typed_crypto;
