//! TUI — Terminal User Interface for GhostWire
//!
//! Modern chat-style terminal interface with:
//! - First-run wizard for new users (display name, key generation)
//! - 3-panel layout (contacts sidebar | conversation | input)
//! - Rounded borders, color-coded status, trust visualization
//! - Display names instead of raw peer IDs
//! - Context-sensitive help bar
//!
//! Modules:
//! - `theme` — color palette, styles, logo rendering, trust bars
//! - `first_run` — guided setup wizard for new users
//! - `app` — main application state, event loop, screen rendering

mod app;
mod first_run;
mod theme;

pub use app::run_tui;
