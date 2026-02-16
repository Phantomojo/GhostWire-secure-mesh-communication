pub mod cli;
pub mod config;
pub mod core;
#[cfg(feature = "web")]
pub mod web;

// API v2
pub mod api_v2_endpoints;
mod api_v2_types;
pub mod controllers;

// Re-export key items for convenience
pub use core::Core;
#[cfg(feature = "web")]
pub use web::AppState;
