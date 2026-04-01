pub mod core_identity;
pub mod encryption;
pub mod recovery;
#[cfg(test)]
mod tests;

pub use core_identity::*;
pub use encryption::{decrypt_identity, encrypt_identity, EncryptedIdentity};
pub use recovery::*;

pub mod coercion;
pub mod panic;
pub mod anonymous_intro;

// Re-export device_pairing — it lives in identity_pairing/ but some code
// imports it from identity/ directly
pub use crate::core::identity_pairing::device_pairing;