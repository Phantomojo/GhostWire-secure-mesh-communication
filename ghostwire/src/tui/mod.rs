//! TUI — Terminal User Interface for GhostWire
//!
//! Modern chat-style terminal interface that connects to the GhostWire daemon
//! via HTTP API. The daemon owns the database and mesh connections; the TUI
//! is a thin client that displays data and sends commands.
//!
//! Architecture:
//!   ghostwire start   →  daemon (background, owns sled DB + mesh)
//!   ghostwire tui     →  TUI client → HTTP API → daemon
//!   ghostwire stop    →  stops the daemon
//!
//! Modules:
//! - `theme` — color palette, styles, logo rendering, trust bars
//! - `wizard` — guided first-run setup wizard for new users
//! - `daemon_client` — HTTP client that talks to the daemon API
//! - `app` — main application state, event loop, screen rendering

mod app;
mod daemon_client;
mod theme;
mod wizard;

pub use app::run_tui;
pub use wizard::run_wizard;
