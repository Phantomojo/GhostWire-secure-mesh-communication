// Copyright (C) 2025-2026 Michael Irungu
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod cli;
pub mod config;
pub mod core;
pub mod daemon;
pub mod genesis_constants;
pub mod startup;
pub mod tui;
#[cfg(feature = "web")]
pub mod web;

// API v2
pub mod api_v2_endpoints;
mod api_v2_types;
pub mod controllers;
pub mod v2_routes;

// Error handling
pub mod error_catalogue;
pub mod timeout_utils;

// Re-export key items for convenience
pub use config::Config;
pub use core::identity::Identity;
pub use core::message::Message;
pub use core::Core;
pub use error_catalogue::{ErrorCode, GhostWireError};
#[cfg(feature = "web")]
pub use web::AppState;
