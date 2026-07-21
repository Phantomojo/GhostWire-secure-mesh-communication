// Copyright (C) 2025-2026 Michael Irungu
// SPDX-License-Identifier: AGPL-3.0-or-later

// Wave 1 enabled these lints for discovery. Audited across the crate:
// - Duration::as_millis() u128→u64: safe (u64 holds 584M years in ms)
// - ML normalization usize→f32: correct (ONNX requires f32 inputs)
// - as_conversions: idiomatic in performance-critical paths (simd, crypto, parsing)
// Per-file allows in feature_extractor.rs, gnn_router.rs, anomaly_detector.rs,
// startup.rs, peer_health.rs cover the highest-risk files explicitly.
#![allow(clippy::as_conversions, clippy::cast_precision_loss, clippy::cast_possible_truncation, clippy::cast_sign_loss)]

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
