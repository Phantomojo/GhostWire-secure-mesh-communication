//! Network monitoring interface for GhostWire TUI

/// Network state and monitoring
pub struct NetworkState {
    pub peer_count: usize,
    pub message_count: usize,
    pub is_connected: bool,
    pub uptime_seconds: u64,
    pub selected_peer: Option<String>,
    pub peers: Vec<PeerInfo>,
}

/// Peer information for display
pub struct PeerInfo {
    pub id: String,
    pub name: String,
    pub is_online: bool,
    pub last_seen: u64,
    pub message_count: usize,
}

impl NetworkState {
    pub fn new() -> Self {
        Self {
            peer_count: 0,
            message_count: 0,
            is_connected: false,
            uptime_seconds: 0,
            selected_peer: None,
            peers: vec![],
        }
    }

    pub fn update_peer_count(&mut self, count: usize) {
        self.peer_count = count;
    }

    pub fn update_message_count(&mut self, count: usize) {
        self.message_count = count;
    }

    pub fn set_connection_status(&mut self, connected: bool) {
        self.is_connected = connected;
    }

    pub fn add_peer(&mut self, peer: PeerInfo) {
        self.peers.push(peer);
    }

    pub fn remove_peer(&mut self, peer_id: &str) {
        self.peers.retain(|p| p.id != peer_id);
    }
}

/// Handle network-specific input
pub fn handle_input(app: &mut crate::tui::TuiApp, key: crossterm::event::KeyEvent) {
    match key.code {
        crossterm::event::KeyCode::Char('s') | crossterm::event::KeyCode::Char('S') => {
            // Scan network
            if let Err(e) = app.message_sender.send(crate::tui::TuiMessage::ScanNetwork) {
                eprintln!("Failed to send scan message: {}", e);
            }
        }
        crossterm::event::KeyCode::Char('c') | crossterm::event::KeyCode::Char('C') => {
            // Connect/disconnect
            if app.network_state.is_connected {
                if let Err(e) = app.message_sender.send(crate::tui::TuiMessage::DisconnectNetwork) {
                    eprintln!("Failed to send disconnect message: {}", e);
                }
            } else {
                if let Err(e) = app.message_sender.send(crate::tui::TuiMessage::ConnectNetwork) {
                    eprintln!("Failed to send connect message: {}", e);
                }
            }
        }
        crossterm::event::KeyCode::Char('r') | crossterm::event::KeyCode::Char('R') => {
            // Refresh network status
            if let Err(e) = app.message_sender.send(crate::tui::TuiMessage::RefreshPeers) {
                eprintln!("Failed to send refresh message: {}", e);
            }
        }
        crossterm::event::KeyCode::Char('m') | crossterm::event::KeyCode::Char('M') => {
            // Show mesh status (already in network tab)
            // Could add specific mesh information here
        }
        crossterm::event::KeyCode::Char('t') | crossterm::event::KeyCode::Char('T') => {
            // Show transport info
            // Could add specific transport information here
        }
        crossterm::event::KeyCode::Char('l') | crossterm::event::KeyCode::Char('L') => {
            // Show network logs
            app.app_state = crate::tui::AppState::Logs;
        }
        _ => {}
    }
} 