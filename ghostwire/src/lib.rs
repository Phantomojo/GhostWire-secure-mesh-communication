pub mod config;
pub mod core;
#[cfg(feature = "web")]
pub mod web;
#[cfg(feature = "tui")]
pub mod tui;

// API v2
mod api_v2_types;
pub mod api_v2_endpoints;

// Re-export key items for convenience
pub use core::Core;
#[cfg(feature = "web")]
pub use web::AppState;
