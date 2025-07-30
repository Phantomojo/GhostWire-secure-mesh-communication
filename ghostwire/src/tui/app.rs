//! Application state management for GhostWire TUI

/// Main application state
pub struct App {
    pub running: bool,
}

impl App {
    pub fn new() -> Self {
        Self { running: true }
    }
} 