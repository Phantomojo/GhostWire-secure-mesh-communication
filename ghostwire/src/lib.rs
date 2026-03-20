// Copyright (C) 2025-2026 Michael Irungu
// SPDX-License-Identifier: AGPL-3.0-or-later

pub mod genesis_constants;
pub mod cli;
pub mod config;
pub mod core;
#[cfg(feature = "web")]
pub mod web;

// API v2
pub mod api_v2_endpoints;
mod api_v2_types;
pub mod controllers;
pub mod v2_routes;

// Re-export key items for convenience
pub use core::Core;
#[cfg(feature = "web")]
pub use web::AppState;
