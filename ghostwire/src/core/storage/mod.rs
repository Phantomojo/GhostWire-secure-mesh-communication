//! # Storage Modules
//!
//! This directory organizes GhostWire's storage and persistence functionality:
//!
//! ## Modules
//!
//! | Module | Description |
//! |--------|-------------|
//! | [`blockstore`] | DHT-like content-addressed storage |
//! | [`encrypted_store`] | Encrypted key-value storage |
//! | [`mailbox`] | Message mailbox storage |
//! | [`persistence_manager`] | Persistence coordination |
//! | [`store`] | Basic key-value store |
//!
//! ## Architecture
//!
//! The storage stack provides multiple layers:
//!
//! ```text
//! High-level API
//!      ↓
//! Persistence Manager (coordinates)
//!      ↓
//! Encrypted Store (security layer)
//!      ↓
//! Block Store / Mailbox (data layer)
//! ```

// Re-exports for backward compatibility
pub use crate::core::blockstore;
pub use crate::core::encrypted_store;
pub use crate::core::mailbox;
pub use crate::core::persistence_manager;
pub use crate::core::store;
