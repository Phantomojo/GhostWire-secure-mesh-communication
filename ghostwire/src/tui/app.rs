//! TUI Application — main state, event loop, and screen rendering.
//!
//! Connects to the GhostWire daemon via HTTP API (no direct Core access).
//! This means the daemon stays alive when the TUI closes, and multiple
//! TUI instances can connect simultaneously.

use super::daemon_client::{
    fingerprint_from_b64, AuditEntry, DaemonClient, DaemonContact, DaemonMessage, HealthResult,
    PeerRequest, TrustEntry, WsStream,
};
use super::theme;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use futures_util::StreamExt;
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::collections::HashMap;
use std::{io, time::Duration};
use tokio::sync::mpsc;
use tokio_tungstenite::tungstenite::Message as WsMessage;

use serde_json;
use tracing::{debug, error, info, trace, warn};

// ═══════════════════════════════════════════════════════════
// Application State
// ═══════════════════════════════════════════════════════════

/// Which screen/view is active
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum View {
    /// Main chat view (contacts sidebar + conversation + input)
    Chat,
    /// Full contact list / address book
    Contacts,
    /// Dashboard / node status
    Dashboard,
    /// AI/ML Intelligence metrics
    Intelligence,
    /// Knowledge Ark storage
    Ark,
    /// Security dashboard
    Security,
}

/// Which mode of input is active
#[derive(Debug, Clone, PartialEq, Eq)]
enum InputMode {
    /// Normal chat message
    Chat,
    /// Adding a new peer ID
    NewPeer,
    /// Verifying a peer by entering their public key (stores peer_id)
    VerifyPeer(String),
}

/// Connection health state communicated from the background task
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ConnectionHealth {
    Connected,
    Disconnected,
    Reconnecting,
}

/// Complete data state fetched by the background refresh task.
/// Sent through the mpsc channel to the render loop.
#[derive(Debug, Clone)]
pub struct AppState {
    pub connection_health: ConnectionHealth,
    pub my_name: String,
    pub contacts: Vec<DaemonContact>,
    pub messages: Vec<DaemonMessage>,
    pub peer_count: usize,
    pub network_mode: String,
    pub encryption_type: String,
    pub ai_status: String,
    pub ai_analysis: Option<super::daemon_client::MeshAnalysis>,
    pub ai_threats: Option<super::daemon_client::ThreatAnalysis>,
    pub swarm_node_count: usize,
    pub swarm_connected: usize,
    pub swarm_last_refresh: Option<std::time::Instant>,
    /// Per-peer latency cache: node_id → latency_ms
    pub peer_latency: HashMap<String, u32>,
    /// Public key identifier from the daemon
    pub key_id: Option<String>,
    /// Human-readable Ed25519 fingerprint (AA:BB:... format)
    pub key_fingerprint: Option<String>,
    /// Pending inbound connection requests
    pub peer_requests: Vec<PeerRequest>,
    /// Real trust scores fetched from the daemon: peer_id → score (0.0–1.0)
    pub trust_scores: HashMap<String, f32>,
    /// Audit log entries from the daemon's security audit ring buffer
    pub audit_log: Vec<AuditEntry>,
}

impl AppState {
    /// State to display when the daemon is unreachable
    fn disconnected() -> Self {
        Self {
            connection_health: ConnectionHealth::Disconnected,
            my_name: String::from("connecting..."),
            contacts: Vec::new(),
            messages: Vec::new(),
            peer_count: 0,
            network_mode: String::from("Mesh P2P"),
            encryption_type: String::from("XChaCha20-Poly1305"),
            ai_status: String::from("Waiting for AI..."),
            ai_analysis: None,
            ai_threats: None,
            swarm_node_count: 0,
            swarm_connected: 0,
            swarm_last_refresh: None,
            peer_latency: HashMap::new(),
            key_id: None,
            key_fingerprint: None,
            peer_requests: Vec::new(),
            trust_scores: HashMap::new(),
            audit_log: Vec::new(),
        }
    }
}

/// Commands sent from the render loop to the background refresh task
#[derive(Debug, Clone)]
enum BackgroundCommand {
    /// Change which peer's messages to fetch
    SelectPeer(Option<String>),
    /// Send a connection request to a peer by node_id
    ConnectPeer(String),
    /// Accept a pending connection request
    AcceptRequest(String),
    /// Deny a pending connection request
    DenyRequest(String),
    /// Verify a peer's public key via the trust system
    VerifyPeer(String, String),
    /// Block a peer via the trust system
    BlockPeer(String),
    /// Disconnect from a connected peer
    DisconnectPeer(String),
    /// Force a refresh of the audit log
    AuditRefresh,
    /// Gracefully shut down the background task
    Shutdown,
}

/// TUI application state
pub struct TuiApp {
    /// HTTP client for the daemon (kept only for fire-and-forget message sending)
    client: DaemonClient,
    /// Commands channel sender to the background refresh task
    cmd_tx: mpsc::Sender<BackgroundCommand>,
    /// State receiver from the background refresh task
    state_rx: mpsc::Receiver<AppState>,
    /// Current data state (last received from background task)
    state: AppState,
    /// Messages display buffer (updated from state.messages + optimistic pushes)
    messages: Vec<DaemonMessage>,
    /// Current view
    view: View,
    /// Selected contact index in the sidebar
    selected_contact: Option<usize>,
    /// Message input buffer
    input: String,
    /// Cursor position in input
    cursor_pos: usize,
    /// What kind of input we're doing
    input_mode: Option<InputMode>,
    /// Running flag
    running: bool,
    /// Error message
    error: Option<String>,
    /// Status / toast message
    status: Option<String>,
    /// Total unread messages
    total_unread: usize,
    /// Last contact we fetched messages for
    last_fetched_peer: Option<String>,
    /// Scroll offset for conversation
    chat_scroll: usize,
    /// Status message timer (in ticks)
    status_timer: u32,
    selected_request: Option<usize>,
    /// Scroll offset for audit log in security view
    audit_scroll: usize,
    /// Peer_id pending disconnect confirmation
    pending_disconnect: Option<String>,
}

impl TuiApp {
    pub fn new(client: DaemonClient, cmd_tx: mpsc::Sender<BackgroundCommand>, state_rx: mpsc::Receiver<AppState>) -> Self {
        Self {
            client,
            cmd_tx,
            state_rx,
            state: AppState::disconnected(),
            messages: Vec::new(),
            view: View::Chat,
            selected_contact: None,
            input: String::new(),
            cursor_pos: 0,
            input_mode: None,
            running: true,
            error: None,
            status: None,
            total_unread: 0,
            last_fetched_peer: None,
            chat_scroll: 0,
            status_timer: 0,
            selected_request: None,
            audit_scroll: 0,
            pending_disconnect: None,
        }
    }

    /// Run the TUI application
    pub async fn run(&mut self) -> io::Result<()> {
        // Setup terminal
        enable_raw_mode()?;
        let mut stdout = io::stdout();
        execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
        let backend = CrosstermBackend::new(stdout);
        let mut terminal = Terminal::new(backend)?;

        // Run event loop
        let result = self.run_event_loop(&mut terminal).await;

        // Restore terminal
        disable_raw_mode()?;
        execute!(
            terminal.backend_mut(),
            LeaveAlternateScreen,
            DisableMouseCapture
        )?;
        terminal.show_cursor()?;

        result
    }

    async fn run_event_loop(
        &mut self,
        terminal: &mut Terminal<CrosstermBackend<io::Stdout>>,
    ) -> io::Result<()> {
        loop {
            if !self.running {
                break;
            }

            // Draw UI
            terminal.draw(|f| self.ui(f))?;

            // Poll for events
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }
                    self.handle_key(key);
                }
            }

            // Try to receive latest state from background task (non-blocking)
            if let Ok(new_state) = self.state_rx.try_recv() {
                self.apply_state(new_state);
            }

            // Decrement status timer
            if self.status_timer > 0 {
                self.status_timer -= 1;
                if self.status_timer == 0 {
                    self.status = None;
                }
            }
        }

        Ok(())
    }

    // ═══════════════════════════════════════════════════
    // State Application (from background task)
    // ═══════════════════════════════════════════════════

    /// Apply a fresh AppState received from the background task to the UI fields.
    /// This is the ONLY place that copies data from the background to the render layer.
    fn apply_state(&mut self, new_state: AppState) {
        self.state = new_state;

        if self.state.contacts.is_empty() {
            self.state.contacts.push(DaemonContact {
                display_name: "Searching for mesh peers...".to_string(),
                peer_id: String::new(),
                online: false,
                trust_score: 0.0,
                unread_count: 0,
                last_seen_secs: 0,
            });
        }

        self.total_unread = self.state.contacts.iter().map(|c| c.unread_count).sum();

        let selected_peer = self.selected_contact
            .and_then(|idx| self.state.contacts.get(idx))
            .and_then(|c| if c.peer_id.is_empty() { None } else { Some(c.peer_id.clone()) });

        if let Some(ref pid) = selected_peer {
            if self.last_fetched_peer.as_ref() != Some(pid) {
                self.messages.clear();
                self.chat_scroll = 0;
                self.last_fetched_peer = Some(pid.clone());
            }
            if !self.state.messages.is_empty() {
                self.messages = self.state.messages.clone();
            }
        }

        if let Some(req_idx) = self.selected_request {
            if req_idx >= self.state.peer_requests.len() {
                self.selected_request = None;
            }
        }

        // Handle connection health transitions (set error/status toasts)
        match self.state.connection_health {
            ConnectionHealth::Connected => {
                if self.error.as_deref() == Some("Connection to daemon lost") {
                    self.error = None;
                    self.status = Some("Reconnected to daemon".to_string());
                    self.status_timer = 50;
                }
            }
            ConnectionHealth::Disconnected => {
                if self.error.as_deref() != Some("Connection to daemon lost") {
                    self.error = Some("Connection to daemon lost".to_string());
                }
            }
            ConnectionHealth::Reconnecting => {}
        }
    }

    // ═══════════════════════════════════════════════════
    // Input Handling
    // ═══════════════════════════════════════════════════

    /// Notify the background task which peer's messages to fetch.
    fn notify_selected_peer(&self) {
        let peer_id = self.selected_contact
            .and_then(|idx| self.state.contacts.get(idx))
            .and_then(|c| if c.peer_id.is_empty() { None } else { Some(c.peer_id.clone()) });
        let _ = self.cmd_tx.try_send(BackgroundCommand::SelectPeer(peer_id));
    }

    fn handle_key(&mut self, key: event::KeyEvent) {
        if let Some(mode) = self.input_mode.as_ref() {
            self.handle_input_key(key, mode.clone());
        } else {
            self.handle_normal_key(key);
        }
    }

    fn handle_normal_key(&mut self, key: event::KeyEvent) {
        // Pending disconnect confirmation takes priority
        if self.pending_disconnect.is_some() {
            match key.code {
                KeyCode::Char('y') | KeyCode::Char('Y') => {
                    if let Some(peer_id) = self.pending_disconnect.take() {
                        let _ = self.cmd_tx.try_send(BackgroundCommand::DisconnectPeer(peer_id));
                        self.status = Some("Disconnecting peer...".to_string());
                        self.status_timer = 40;
                    }
                }
                KeyCode::Char('n') | KeyCode::Char('N') | KeyCode::Esc => {
                    self.pending_disconnect = None;
                    self.status = Some("Disconnect cancelled".to_string());
                    self.status_timer = 30;
                }
                _ => {}
            }
            return;
        }

        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.running = false;
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                // Enter input mode
                if self.selected_contact.is_some() {
                    self.input_mode = Some(InputMode::Chat);
                    self.cursor_pos = self.input.len();
                } else {
                    self.error = Some("Select a contact first (or press 'n' for new)".to_string());
                }
            }
            KeyCode::Char('n') | KeyCode::Char('N') => {
                // New peer chat
                self.input_mode = Some(InputMode::NewPeer);
                self.input.clear();
                self.cursor_pos = 0;
            }
            KeyCode::Char('1') => self.view = View::Chat,
            KeyCode::Char('2') => self.view = View::Contacts,
            KeyCode::Char('3') => self.view = View::Dashboard,
            KeyCode::Char('4') => self.view = View::Intelligence,
            KeyCode::Char('5') => self.view = View::Ark,
            KeyCode::Char('6') => self.view = View::Security,
            KeyCode::Char('s') | KeyCode::Char('S') => self.view = View::Security,
            KeyCode::Tab => {
                if self.view == View::Security {
                    // In security view, Tab toggles between none (just stay in Security)
                    // Let Esc/1-6/other views handle navigation out
                } else {
                    // Cycle views
                    self.view = match self.view {
                        View::Chat => View::Contacts,
                        View::Contacts => View::Dashboard,
                        View::Dashboard => View::Intelligence,
                        View::Intelligence => View::Ark,
                        View::Ark => View::Security,
                        View::Security => View::Chat,
                    };
                }
            }
            KeyCode::Char('a') | KeyCode::Char('A') => {
                if self.view == View::Contacts {
                    if let Some(req_idx) = self.selected_request {
                        if let Some(req) = self.state.peer_requests.get(req_idx) {
                            let _ = self.cmd_tx.try_send(BackgroundCommand::AcceptRequest(req.request_id.clone()));
                            self.selected_request = None;
                            self.status = Some("Connection request accepted".to_string());
                            self.status_timer = 40;
                        }
                    }
                }
            }
            KeyCode::Char('v') | KeyCode::Char('V') => {
                if self.view == View::Contacts {
                    if let Some(cont_idx) = self.selected_contact {
                        if let Some(contact) = self.state.contacts.get(cont_idx) {
                            if !contact.peer_id.is_empty() {
                                self.input.clear();
                                self.cursor_pos = 0;
                                self.input_mode = Some(InputMode::VerifyPeer(contact.peer_id.clone()));
                                self.status = Some(format!("Enter public key hex for {}", contact.display_name));
                                self.status_timer = 80;
                            }
                        }
                    }
                }
            }
            KeyCode::Char('b') | KeyCode::Char('B') => {
                if self.view == View::Contacts {
                    if let Some(cont_idx) = self.selected_contact {
                        if let Some(contact) = self.state.contacts.get(cont_idx) {
                            if !contact.peer_id.is_empty() {
                                let _ = self.cmd_tx.try_send(BackgroundCommand::BlockPeer(contact.peer_id.clone()));
                                self.status = Some(format!("Block request sent for {}", contact.display_name));
                                self.status_timer = 40;
                            }
                        }
                    }
                }
            }
            KeyCode::Char('r') | KeyCode::Char('R') => {
                if self.view == View::Security {
                    let _ = self.cmd_tx.try_send(BackgroundCommand::AuditRefresh);
                    self.status = Some("Audit log refreshed".to_string());
                    self.status_timer = 40;
                }
            }
            KeyCode::Char('d') | KeyCode::Char('D') => {
                if self.view == View::Contacts {
                    if let Some(req_idx) = self.selected_request {
                        if let Some(req) = self.state.peer_requests.get(req_idx) {
                            let _ = self.cmd_tx.try_send(BackgroundCommand::DenyRequest(req.request_id.clone()));
                            self.selected_request = None;
                            self.status = Some("Connection request declined".to_string());
                            self.status_timer = 40;
                        }
                    } else if let Some(cont_idx) = self.selected_contact {
                        if let Some(contact) = self.state.contacts.get(cont_idx) {
                            if contact.online && !contact.peer_id.is_empty() {
                                self.pending_disconnect = Some(contact.peer_id.clone());
                                self.status = Some(format!(
                                    "Disconnect from {}? Press y to confirm, n to cancel",
                                    contact.display_name
                                ));
                                self.status_timer = 0;
                            }
                        }
                    }
                }
            }
            KeyCode::Up => {
                if self.view == View::Security {
                    if self.audit_scroll > 0 {
                        self.audit_scroll -= 1;
                    }
                } else if self.view == View::Contacts && !self.state.peer_requests.is_empty() {
                    let num_reqs = self.state.peer_requests.len();
                    if let Some(req_idx) = self.selected_request {
                        self.selected_request = if req_idx > 0 { Some(req_idx - 1) } else { Some(0) };
                    } else if let Some(cont_idx) = self.selected_contact {
                        if cont_idx == 0 {
                            self.selected_request = Some(num_reqs - 1);
                            self.selected_contact = None;
                        } else {
                            self.selected_contact = Some(cont_idx - 1);
                        }
                    } else {
                        self.selected_request = Some(num_reqs - 1);
                    }
                } else {
                    if let Some(idx) = self.selected_contact {
                        if idx > 0 {
                            self.selected_contact = Some(idx - 1);
                            self.notify_selected_peer();
                        }
                    } else if !self.state.contacts.is_empty() {
                        self.selected_contact = Some(0);
                        self.notify_selected_peer();
                    }
                }
            }
            KeyCode::Down => {
                if self.view == View::Security {
                    let max_scroll = self.state.audit_log.len().saturating_sub(1);
                    if self.audit_scroll < max_scroll {
                        self.audit_scroll += 1;
                    }
                } else if self.view == View::Contacts && !self.state.peer_requests.is_empty() {
                    let num_reqs = self.state.peer_requests.len();
                    if let Some(req_idx) = self.selected_request {
                        if req_idx + 1 < num_reqs {
                            self.selected_request = Some(req_idx + 1);
                        } else {
                            self.selected_request = None;
                            self.selected_contact = if self.state.contacts.is_empty() { None } else { Some(0) };
                        }
                    } else if let Some(cont_idx) = self.selected_contact {
                        if cont_idx + 1 < self.state.contacts.len() {
                            self.selected_contact = Some(cont_idx + 1);
                        }
                    } else if !self.state.contacts.is_empty() {
                        self.selected_contact = Some(0);
                        self.notify_selected_peer();
                    }
                } else {
                    if let Some(idx) = self.selected_contact {
                        if idx + 1 < self.state.contacts.len() {
                            self.selected_contact = Some(idx + 1);
                            self.notify_selected_peer();
                        }
                    } else if !self.state.contacts.is_empty() {
                        self.selected_contact = Some(0);
                        self.notify_selected_peer();
                    }
                }
            }
            KeyCode::PageUp => {
                if self.chat_scroll > 5 {
                    self.chat_scroll -= 5;
                } else {
                    self.chat_scroll = 0;
                }
            }
            KeyCode::PageDown => {
                self.chat_scroll += 5;
            }
            KeyCode::Enter => {
                // Open selected conversation
                if let Some(idx) = self.selected_contact {
                    if idx < self.state.contacts.len() {
                        let contact = &self.state.contacts[idx];
                        if !contact.peer_id.is_empty() {
                            self.selected_contact = Some(idx);
                            self.input_mode = Some(InputMode::Chat);
                            self.cursor_pos = self.input.len();
                        }
                    }
                }
            }
            KeyCode::Esc => {
                self.error = None;
                self.status = None;
                if self.view == View::Security {
                    self.view = View::Chat;
                    self.audit_scroll = 0;
                }
            }
            _ => {}
        }
    }

    fn handle_input_key(&mut self, key: event::KeyEvent, mode: InputMode) {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = None;
                self.error = None;
            }
            KeyCode::Enter => {
                if !self.input.trim().is_empty() {
                    match mode {
                        InputMode::Chat => self.send_message(),
                        InputMode::NewPeer => self.start_new_chat(),
                        InputMode::VerifyPeer(peer_id) => self.verify_peer(&peer_id),
                    }
                }
            }
            KeyCode::Backspace => {
                if self.cursor_pos > 0 {
                    self.input.remove(self.cursor_pos - 1);
                    self.cursor_pos -= 1;
                }
            }
            KeyCode::Delete => {
                if self.cursor_pos < self.input.len() {
                    self.input.remove(self.cursor_pos);
                }
            }
            KeyCode::Left => {
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_pos < self.input.len() {
                    self.cursor_pos += 1;
                }
            }
            KeyCode::Char(c) => {
                self.input.insert(self.cursor_pos, c);
                self.cursor_pos += 1;
            }
            _ => {}
        }
    }

    // ═══════════════════════════════════════════════════
    // Message Sending (via daemon API)
    // ═══════════════════════════════════════════════════

    fn send_message(&mut self) {
        let message = self.input.clone();
        let recipient_idx = self.selected_contact;

        if message.trim().is_empty() {
            self.error = Some("Message cannot be empty".to_string());
            return;
        }

        let Some(idx) = recipient_idx else {
            self.error = Some("Select a contact first".to_string());
            return;
        };

        if idx >= self.state.contacts.len() {
            self.error = Some("Invalid contact selection".to_string());
            return;
        }

        let contact = &self.state.contacts[idx];
        if contact.peer_id.is_empty() {
            self.error = Some("No valid peer selected".to_string());
            return;
        }

        let recipient = contact.peer_id.clone();
        let my_name = self.state.my_name.clone();
        let client = self.client.clone_for_send();

        // Add to local message list immediately (optimistic UI)
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        self.messages.push(DaemonMessage {
            sender: my_name.clone(),
            content: message.clone(),
            timestamp: now,
            is_mine: true,
        });

        // Send via daemon API (async, fire-and-forget)
        tokio::spawn(async move {
            match client.send_message(&recipient, &message).await {
                Ok(()) => tracing::info!("Message sent via daemon"),
                Err(e) => tracing::error!("Failed to send message: {}", e),
            }
        });

        self.status = Some(format!("Sent to {}", contact.display_name));
        self.status_timer = 50; // Clear after ~5 seconds
        self.input.clear();
        self.cursor_pos = 0;
        self.input_mode = None;
    }

    fn start_new_chat(&mut self) {
        let peer_id = self.input.trim().to_string();
        if peer_id.is_empty() {
            return;
        }

        if let Some(idx) = self.state.contacts.iter().position(|c| c.peer_id == peer_id) {
            self.selected_contact = Some(idx);
            self.input.clear();
            self.cursor_pos = 0;
            self.input_mode = Some(InputMode::Chat);
            self.notify_selected_peer();
            return;
        }

        let _ = self.cmd_tx.try_send(BackgroundCommand::ConnectPeer(peer_id.clone()));

        self.input.clear();
        self.cursor_pos = 0;
        self.input_mode = None;
        self.status = Some(format!("Connection request sent to {}", peer_id));
        self.status_timer = 80;
    }

    fn verify_peer(&mut self, peer_id: &str) {
        let key_hex = self.input.trim().to_string();
        if key_hex.is_empty() {
            return;
        }
        let _ = self.cmd_tx.try_send(BackgroundCommand::VerifyPeer(peer_id.to_string(), key_hex));
        self.input.clear();
        self.cursor_pos = 0;
        self.input_mode = None;
        self.status = Some(format!("Verification sent for {}", peer_id));
        self.status_timer = 40;
    }

    // ═══════════════════════════════════════════════════
    // UI Rendering
    // ═══════════════════════════════════════════════════

    fn ui(&self, f: &mut Frame) {
        let area = f.area();
        let main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
                Constraint::Length(1),
            ])
            .split(area);

        match self.view {
            View::Chat => self.render_chat_view(f),
            View::Contacts => self.render_contacts_view(f),
            View::Dashboard => {
                self.render_dashboard_view(f);
            }
            View::Intelligence => {
                super::intelligence_view::render(
                    f,
                    main[1],
                    &self.state.ai_status,
                    &self.state.ai_analysis,
                    &self.state.ai_threats,
                    self.state.swarm_node_count,
                    self.state.swarm_connected,
                    self.state.swarm_last_refresh,
                );
            }
            View::Ark => {
                let para = Paragraph::new("\n\n  Knowledge Ark View Coming Soon...").block(Block::default().borders(Borders::ALL).title(" Knowledge Ark "));
                f.render_widget(para, main[1]);
            }
            View::Security => {
                self.render_security_view(f);
            }
        }

        // Overlay error/status if present
        if let Some(ref err) = self.error {
            self.render_error_popup(f, err);
        } else if let Some(ref status) = self.status {
            self.render_status_popup(f, status);
        }
    }

    fn render_error_popup(&self, f: &mut Frame, err: &str) {
        let area = f.area();
        let popup_area = centered_rect(60, 20, area);
        let block = Block::default()
            .title(" Error ")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(theme::error_style());
        let paragraph = Paragraph::new(err)
            .block(block)
            .style(theme::error_style())
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(ratatui::widgets::Clear, popup_area); // Clear background
        f.render_widget(paragraph, popup_area);
    }

    fn render_status_popup(&self, f: &mut Frame, status: &str) {
        let area = f.area();
        let popup_area = centered_rect(40, 10, area);
        let block = Block::default()
            .title(" Status ")
            .borders(Borders::ALL)
            .border_type(ratatui::widgets::BorderType::Rounded)
            .border_style(theme::primary_bold());
        let paragraph = Paragraph::new(status)
            .block(block)
            .style(theme::text_style())
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(ratatui::widgets::Clear, popup_area);
        f.render_widget(paragraph, popup_area);
    }

    fn render_chat_view(&self, f: &mut Frame) {
        let area = f.area();

        // Main layout: header | sidebar+content | input | help
        let main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(8),    // Main content (sidebar + conversation)
                Constraint::Length(3), // Input area
                Constraint::Length(3), // Help bar
            ])
            .split(area);

        // Header
        self.render_header(f, main[0]);

        // Split main into sidebar + conversation
        let content = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Percentage(25), // Sidebar (contacts)
                Constraint::Percentage(75), // Conversation
            ])
            .split(main[1]);

        // Sidebar
        self.render_sidebar(f, content[0]);

        // Conversation area
        self.render_conversation(f, content[1]);

        // Input area
        self.render_input_area(f, main[2]);

        // Help bar
        self.render_help_bar(f, main[3]);
    }

    fn render_header(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let logo = theme::logo_header();
        let status_text = Line::from(vec![
            theme::status_dot(self.state.peer_count > 0),
            Span::raw(" "),
            Span::styled(&self.state.my_name, theme::primary_bold()),
            Span::raw(" | "),
            Span::styled(format!("{} Peers", self.state.peer_count), theme::text_style()),
            Span::raw(" | "),
            Span::styled(&self.state.network_mode, theme::muted_style()),
        ]);

        let header = Paragraph::new(vec![logo, status_text])
            .block(theme::header_block())
            .alignment(ratatui::layout::Alignment::Center);
        f.render_widget(header, area);
    }

    fn render_sidebar(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let items: Vec<ListItem> = self
            .state.contacts
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let is_selected = self.selected_contact == Some(i);
                let unread_indicator = if c.unread_count > 0 {
                    format!(" ({})", c.unread_count)
                } else {
                    String::new()
                };

                let style = if is_selected {
                    theme::highlight_style()
                } else {
                    theme::text_style()
                };

                let latency_span = match self.state.peer_latency.get(&c.peer_id) {
                    Some(lat) if *lat < 32 => {
                        Span::styled(format!(" {}ms", lat), Style::default().fg(Color::Green))
                    }
                    Some(lat) if *lat <= 100 => {
                        Span::styled(format!(" {}ms", lat), Style::default().fg(Color::Yellow))
                    }
                    Some(lat) => {
                        Span::styled(format!(" {}ms", lat), Style::default().fg(Color::Red))
                    }
                    None => Span::styled("  —", theme::muted_style()),
                };

                let name_line = Line::from(vec![
                    theme::status_dot(c.online),
                    Span::raw(" "),
                    Span::styled(&c.display_name, style),
                    latency_span,
                    Span::styled(unread_indicator, theme::warning_style()),
                ]);

                let trust_score = self.state.trust_scores.get(&c.peer_id).copied().unwrap_or(0.5);
                let trust_line = theme::trust_bar_line(trust_score as f64, &c.display_name);

                ListItem::new(vec![name_line, trust_line])
            })
            .collect();

        let contact_list = List::new(items).block(theme::themed_block("Contacts"));
        f.render_widget(contact_list, area);
    }

    fn render_conversation(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let contact_name = self
            .selected_contact
            .and_then(|i| self.state.contacts.get(i))
            .map(|c| c.display_name.as_str())
            .unwrap_or("Select a contact");

        if self.messages.is_empty() {
            let welcome = Paragraph::new(vec![
                Line::from(""),
                Line::from(vec![Span::styled(
                    "No messages yet",
                    Style::default()
                        .fg(Color::White)
                        .add_modifier(Modifier::BOLD),
                )]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    "Select a contact and press 'i' to start chatting.",
                    theme::muted_style(),
                )]),
                Line::from(""),
                Line::from(vec![Span::styled(
                    "Messages are end-to-end encrypted.",
                    theme::muted_style(),
                )]),
            ])
            .block(theme::themed_block(contact_name));
            f.render_widget(welcome, area);
        } else {
            let mut msg_lines: Vec<Line> = Vec::new();
            
            for m in &self.messages {
                let time = format_time(m.timestamp);
                let sender_style = if m.is_mine {
                    theme::sent_style()
                } else {
                    theme::primary_bold()
                };
                let prefix = if m.is_mine { "→" } else { "←" };

                // Truncate sender name if it's a long PeerID
                let sender_display = if m.sender.len() > 12 {
                    format!("{}...", &m.sender[0..8])
                } else {
                    m.sender.clone()
                };

                let alignment = if m.is_mine {
                    ratatui::layout::Alignment::Right
                } else {
                    ratatui::layout::Alignment::Left
                };

                if m.is_mine {
                    msg_lines.push(Line::from(vec![
                        Span::styled(sender_display, sender_style),
                        Span::raw(" "),
                        Span::raw(time),
                        Span::raw(" "),
                        Span::styled(prefix, theme::muted_style()),
                    ]).alignment(alignment));
                } else {
                    msg_lines.push(Line::from(vec![
                        Span::styled(prefix, theme::muted_style()),
                        Span::raw(" "),
                        Span::raw(time),
                        Span::raw(" "),
                        Span::styled(sender_display, sender_style),
                    ]).alignment(alignment));
                }

                msg_lines.push(Line::from(vec![
                    Span::raw("   "),
                    Span::styled(m.content.clone(), theme::text_style()),
                ]).alignment(alignment));
                msg_lines.push(Line::from("")); // spacing between messages
            }

            // Calculate max scroll
            let total_lines = msg_lines.len();
            let visible_height = area.height.saturating_sub(2) as usize; // account for borders
            
            let scroll = if total_lines > visible_height {
                self.chat_scroll.min(total_lines.saturating_sub(visible_height))
            } else {
                0
            };

            let conversation = Paragraph::new(msg_lines)
                .block(theme::themed_block(contact_name))
                .scroll((scroll as u16, 0));
            
            f.render_widget(conversation, area);
            
            // Render scroll indicator if needed
            if total_lines > visible_height {
                let percent = (scroll as f32 / (total_lines - visible_height) as f32 * 100.0) as usize;
                let indicator = format!(" {}% ", percent);
                let indicator_area = ratatui::layout::Rect {
                    x: area.x + area.width.saturating_sub(indicator.len() as u16 + 2),
                    y: area.y,
                    width: indicator.len() as u16,
                    height: 1,
                };
                f.render_widget(Paragraph::new(indicator).style(theme::muted_style()), indicator_area);
            }
        }
    }

    fn render_input_area(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let input_text = if self.input_mode.is_some() {
            // Show cursor
            let before = &self.input[..self.cursor_pos];
            let after = &self.input[self.cursor_pos..];
            format!(" {before}█{after}")
        } else if self.input.is_empty() {
            " Press 'i' to type, 'n' for new chat...".to_string()
        } else {
            format!(" {}", self.input)
        };

        let border_style = if self.input_mode.is_some() {
            theme::primary_bold()
        } else {
            Style::default().fg(theme::MUTED)
        };

        let title = match self.input_mode {
            Some(InputMode::Chat) => " Message ",
            Some(InputMode::NewPeer) => " Enter Peer ID ",
            Some(InputMode::VerifyPeer(_)) => " Enter Public Key Hex ",
            None => " Type to chat ",
        };

        let input_widget = Paragraph::new(input_text)
            .style(if self.input_mode.is_some() {
                theme::text_style()
            } else {
                theme::muted_style()
            })
            .block(
                Block::default()
                    .title(title)
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(border_style),
            );
        f.render_widget(input_widget, area);
    }

    fn render_help_bar(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let hints = match self.input_mode {
            Some(InputMode::Chat) => {
                theme::help_bar(&[("Enter", "Send"), ("Esc", "Cancel"), ("←→", "Move cursor")])
            }
            Some(InputMode::NewPeer) => {
                theme::help_bar(&[("Enter", "Start Chat"), ("Esc", "Cancel")])
            }
            Some(InputMode::VerifyPeer(_)) => {
                theme::help_bar(&[("Enter", "Verify"), ("Esc", "Cancel")])
            }
            None => {
                if self.view == View::Security {
                    theme::help_bar(&[
                        ("↑↓", "Scroll log"),
                        ("r", "Refresh"),
                        ("1-6", "Views"),
                        ("s", "Security"),
                        ("Esc", "Back"),
                        ("q", "Quit"),
                    ])
                } else {
                    theme::help_bar(&[
                        ("1-6", "Views"),
                        ("↑↓", "Navigate"),
                        ("Enter", "Open"),
                        ("i", "Type"),
                        ("n", "New"),
                        ("q", "Quit"),
                    ])
                }
            }
        };
        f.render_widget(hints, area);
    }

    fn render_contacts_view(&self, f: &mut Frame) {
        let area = f.area();
        let main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(8),
                Constraint::Length(3),
            ])
            .split(area);

        self.render_header(f, main[0]);

        let has_requests = !self.state.peer_requests.is_empty();

        if has_requests {
            let req_height = self.state.peer_requests.len() as u16 * 3 + 2;
            let content = Layout::default()
                .direction(Direction::Vertical)
                .constraints([
                    Constraint::Length(req_height.min(12)),
                    Constraint::Min(1),
                ])
                .split(main[1]);

            let request_items: Vec<ListItem> = self
                .state.peer_requests
                .iter()
                .enumerate()
                .map(|(i, r)| {
                    let is_selected = self.selected_request == Some(i);
                    let style = if is_selected {
                        theme::highlight_style()
                    } else {
                        theme::text_style()
                    };

                    let from = if r.from_nickname.is_empty() {
                        format!("{}...", &r.from_node_id[..8.min(r.from_node_id.len())])
                    } else {
                        format!("{} ({})", r.from_nickname, &r.from_node_id[..8.min(r.from_node_id.len())])
                    };

                    let action_hint = if is_selected {
                        "  [a] Accept  [d] Deny"
                    } else {
                        ""
                    };

                    let req_line = Line::from(vec![
                        Span::styled("  ✦ ", Style::default().fg(Color::Yellow)),
                        Span::styled(from, style),
                        Span::styled(action_hint, theme::warning_style()),
                    ]);
                    let msg_line = match &r.message {
                        Some(msg) => Line::from(vec![
                            Span::styled("     ", theme::muted_style()),
                            Span::styled(msg, theme::muted_style()),
                        ]),
                        None => Line::from(vec![]),
                    };

                    ListItem::new(vec![req_line, msg_line])
                })
                .collect();

            let req_list = List::new(request_items)
                .block(theme::themed_block("Pending Connection Requests"))
                .highlight_style(theme::highlight_style());
            f.render_widget(req_list, content[0]);

            let contact_items: Vec<ListItem> = self.build_contact_items();
            let contact_list = List::new(contact_items)
                .block(theme::themed_block("All Contacts"))
                .highlight_style(theme::highlight_style());
            f.render_widget(contact_list, content[1]);
        } else {
            let contact_items: Vec<ListItem> = self.build_contact_items();
            let contact_list = List::new(contact_items).block(theme::themed_block("All Contacts"));
            f.render_widget(contact_list, main[1]);
        }

        let help_items: &[(&str, &str)] = if self.selected_request.is_some() {
            &[("↑↓", "Navigate"), ("a", "Accept"), ("d", "Deny"), ("Enter", "Message"), ("v", "Verify"), ("b", "Block"), ("Tab", "Switch view"), ("q", "Quit")]
        } else if self.selected_contact.is_some() {
            &[("↑↓", "Navigate"), ("v", "Verify"), ("b", "Block"), ("Enter", "Message"), ("Tab", "Switch view"), ("q", "Quit")]
        } else {
            &[("↑↓", "Navigate"), ("Enter", "Message"), ("Tab", "Switch view"), ("q", "Quit")]
        };
        let help = theme::help_bar(help_items);
        f.render_widget(help, main[2]);
    }

    fn build_contact_items(&self) -> Vec<ListItem<'_>> {
        self
            .state.contacts
            .iter()
            .map(|c| {
                let status_text = if c.online {
                    "online".to_string()
                } else {
                    format!("last seen {} ago", format_duration(c.last_seen_secs))
                };
                let latency_span = match self.state.peer_latency.get(&c.peer_id) {
                    Some(lat) if *lat < 32 => {
                        Span::styled(format!(" {}ms", lat), Style::default().fg(Color::Green))
                    }
                    Some(lat) if *lat <= 100 => {
                        Span::styled(format!(" {}ms", lat), Style::default().fg(Color::Yellow))
                    }
                    Some(lat) => {
                        Span::styled(format!(" {}ms", lat), Style::default().fg(Color::Red))
                    }
                    None => Span::styled("  —", theme::muted_style()),
                };

                let trust_score = self.state.trust_scores.get(&c.peer_id).copied().unwrap_or(0.5);

                let name_line = Line::from(vec![
                    theme::status_dot(c.online),
                    Span::raw(" "),
                    Span::styled(&c.display_name, theme::text_style()),
                    latency_span,
                    Span::raw("  "),
                    Span::styled(status_text, theme::muted_style()),
                ]);
                let trust_line = theme::trust_bar_line(trust_score as f64, &c.display_name);

                ListItem::new(vec![name_line, trust_line])
            })
            .collect()
    }

    fn render_dashboard_view(&self, f: &mut Frame) {
        let area = f.area();
        let main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(8),
                Constraint::Length(3),
            ])
            .split(area);

        self.render_header(f, main[0]);

        // Dashboard content
        let content = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(5), // Node info
                Constraint::Length(4), // Key fingerprint
                Constraint::Length(5), // Network
                Constraint::Min(1),    // Spacer
            ])
            .split(main[1]);

        let node_info = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Name:      ", theme::muted_style()),
                Span::styled(&self.state.my_name, theme::primary_bold()),
            ]),
            Line::from(vec![
                Span::styled("Status:    ", theme::muted_style()),
                Span::styled("🟢 Running", theme::online_style()),
            ]),
            Line::from(vec![
                Span::styled("Encryption:", theme::muted_style()),
                Span::styled(" XChaCha20-Poly1305", theme::text_style()),
            ]),
        ])
        .block(theme::themed_block("Your Node"));
        f.render_widget(node_info, content[0]);

        // Key identity section
        let finger_print = match &self.state.key_fingerprint {
            Some(fp) => Span::styled(fp, Style::default().fg(Color::Cyan)),
            None => Span::styled("—", theme::muted_style()),
        };
        let key_id_line = match &self.state.key_id {
            Some(kid) => Span::styled(kid, theme::muted_style()),
            None => Span::styled("—", theme::muted_style()),
        };
        let key_block = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Key ID:    ", theme::muted_style()),
                key_id_line,
            ]),
            Line::from(vec![
                Span::styled("Fingerprint:", theme::muted_style()),
                Span::raw(" "),
                finger_print,
            ]),
        ])
        .block(theme::themed_block("Identity Key"));
        f.render_widget(key_block, content[1]);

        let network = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Mode:      ", theme::muted_style()),
                Span::styled(&self.state.network_mode, theme::text_style()),
            ]),
            Line::from(vec![
                Span::styled("Peers:     ", theme::muted_style()),
                Span::styled(
                    format!("{} connected", self.state.peer_count),
                    theme::online_style(),
                ),
            ]),
            Line::from(vec![
                Span::styled("AI Layer:  ", theme::muted_style()),
                Span::styled("L1 Active (91μs)", theme::text_style()),
            ]),
        ])
        .block(theme::themed_block("Network"));
        f.render_widget(network, content[2]);

        let help = theme::help_bar(&[
            ("Tab", "Switch view"),
            ("1/2/3", "Chat/Contacts/Dashboard"),
            ("q", "Quit"),
        ]);
        f.render_widget(help, main[2]);
    }

    /// Render the Security Dashboard view.
    ///
    /// Three vertical sections:
    ///   1. Status bar — key fingerprint, auth status, peer/blocked counts
    ///   2. Peer security summary — scrollable peer list with trust bars
    ///   3. Audit log — time-ordered event list with color coding
    fn render_security_view(&self, f: &mut Frame) {
        let area = f.area();
        let main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
                Constraint::Length(3),
            ])
            .split(area);

        self.render_header(f, main[0]);

        // Content: status bar (top), peer summary (middle), audit log (bottom)
        let content = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(4), // Node security status
                Constraint::Length(6), // Peer security summary (compact)
                Constraint::Min(1),    // Audit log
            ])
            .split(main[1]);

        // ── Section 1: Status Bar ──
        let fingerprint = match &self.state.key_fingerprint {
            Some(fp) => Span::styled(fp, Style::default().fg(Color::Cyan)),
            None => Span::styled("—", theme::muted_style()),
        };
        let auth_status = Span::styled("Authenticated", theme::online_style());
        let connected_count = self.state.peer_count;
        let blocked_count = self
            .state
            .audit_log
            .iter()
            .filter(|e| e.event_type == "PeerBlocked")
            .count();

        let status_lines = vec![
            Line::from(vec![
                Span::styled("Key: ", theme::muted_style()),
                fingerprint,
            ]),
            Line::from(vec![
                Span::styled("Auth: ", theme::muted_style()),
                auth_status,
                Span::raw("  |  "),
                Span::styled("Connected: ", theme::muted_style()),
                Span::styled(format!("{}", connected_count), theme::online_style()),
                Span::raw("  |  "),
                Span::styled("Blocked: ", theme::muted_style()),
                Span::styled(format!("{}", blocked_count), theme::warning_style()),
            ]),
        ];
        let status_block = Paragraph::new(status_lines)
            .block(theme::themed_block("Node Security Status"));
        f.render_widget(status_block, content[0]);

        // ── Section 2: Peer Security Summary ──
        let peer_items: Vec<ListItem> = self
            .state
            .contacts
            .iter()
            .filter(|c| !c.peer_id.is_empty())
            .map(|c| {
                let trust_score = self.state.trust_scores.get(&c.peer_id).copied().unwrap_or(0.5);
                let truncated = if c.peer_id.len() > 12 {
                    format!("{}…", &c.peer_id[..12])
                } else {
                    c.peer_id.clone()
                };

                // Badge: Verified / Blocked / Unknown
                let trust_level_str = self
                    .state
                    .audit_log
                    .iter()
                    .find(|e| e.peer_id.as_deref() == Some(&c.peer_id))
                    .map(|e| e.event_type.as_str())
                    .unwrap_or("");
                let badge = if trust_level_str == "PeerBlocked" {
                    Span::styled(" BLOCKED ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD))
                } else if trust_level_str == "PeerVerified" {
                    Span::styled(" VERIFIED ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD))
                } else {
                    Span::styled(" unknown ", theme::muted_style())
                };

                let id_line = Line::from(vec![
                    theme::status_dot(c.online),
                    Span::raw(" "),
                    Span::styled(truncated, theme::text_style()),
                    Span::raw(" "),
                    badge,
                ]);
                let trust_line = theme::trust_bar_line(trust_score as f64, &c.display_name);

                ListItem::new(vec![id_line, trust_line])
            })
            .collect();

        let peer_list = if peer_items.is_empty() {
            List::new(vec![ListItem::new(Line::from(vec![
                Span::styled("  No peers connected", theme::muted_style()),
            ]))])
            .block(theme::themed_block("Peer Security Summary"))
        } else {
            List::new(peer_items).block(theme::themed_block("Peer Security Summary"))
        };
        f.render_widget(peer_list, content[1]);

        // ── Section 3: Audit Log ──
        let audit_lines: Vec<Line> = self
            .state
            .audit_log
            .iter()
            .enumerate()
            .map(|(i, entry)| {
                // Color coding
                let style = match entry.event_type.as_str() {
                    "AuthFailure" | "SecurityLockdown" => {
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
                    }
                    "PeerBlocked" => Style::default().fg(Color::Yellow),
                    "PeerVerified" => Style::default().fg(Color::Green),
                    "SystemStartup" => theme::muted_style(),
                    _ => theme::text_style(),
                };

                // Short timestamp: only HH:MM:SS portion from RFC3339
                let ts = entry.timestamp.as_str();
                let short_ts = if ts.len() > 19 { &ts[11..19] } else { ts };

                let peer_info = match &entry.peer_id {
                    Some(pid) if !pid.is_empty() => {
                        if pid.len() > 12 {
                            format!(" {}…", &pid[..12])
                        } else {
                            format!(" {}", pid)
                        }
                    }
                    _ => String::new(),
                };

                let row = format!("{} {}{} {}", short_ts, entry.event_type, peer_info, entry.detail);
                Line::from(Span::styled(row, style))
            })
            .collect();

        let total_entries = audit_lines.len();
        let visible_height = content[2].height.saturating_sub(2) as usize;
        let scroll = if total_entries > visible_height {
            self.audit_scroll.min(total_entries.saturating_sub(visible_height))
        } else {
            0
        };

        let audit_paragraph = Paragraph::new(audit_lines)
            .block(theme::themed_block("Audit Log"))
            .scroll((scroll as u16, 0));
        f.render_widget(audit_paragraph, content[2]);

        // Scroll indicator
        if total_entries > visible_height {
            let max_scroll = total_entries - visible_height;
            let percent = if max_scroll > 0 {
                (scroll as f32 / max_scroll as f32 * 100.0) as usize
            } else {
                0
            };
            let indicator = format!(" {}% ", percent);
            let indicator_area = ratatui::layout::Rect {
                x: content[2].x + content[2].width.saturating_sub(indicator.len() as u16 + 2),
                y: content[2].y,
                width: indicator.len() as u16,
                height: 1,
            };
            f.render_widget(
                Paragraph::new(indicator).style(theme::muted_style()),
                indicator_area,
            );
        }
    }
}

// ═══════════════════════════════════════════════════════════
// Helpers
// ═══════════════════════════════════════════════════════════

fn format_time(timestamp: u64) -> String {
    let secs = timestamp % 86400;
    let hours = secs / 3600;
    let mins = (secs % 3600) / 60;
    format!("{:02}:{:02}", hours, mins)
}

fn format_duration(secs: u64) -> String {
    if secs < 60 {
        format!("{}s", secs)
    } else if secs < 3600 {
        format!("{}m", secs / 60)
    } else if secs < 86400 {
        format!("{}h", secs / 3600)
    } else {
        format!("{}d", secs / 86400)
    }
}

fn centered_rect(percent_x: u16, percent_y: u16, r: ratatui::layout::Rect) -> ratatui::layout::Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

// ═══════════════════════════════════════════════════════════
// Background Refresh Task
// ═══════════════════════════════════════════════════════════

/// Background task that owns the DaemonClient and periodically refreshes mesh data.
///
/// Health check every 2s — detects daemon restarts / crashes.
/// Full refresh every 5s — safety net (WebSocket handles real-time deltas
/// when connected, polling takes over transparently when disconnected).
/// WebSocket connection to the daemon's /ws event bus receives CoreEvent
/// JSON messages for immediate updates without polling.
/// When WebSocket drops: retry WS every 10s, polling covers the gap.
async fn background_worker(
    client: DaemonClient,
    state_tx: mpsc::Sender<AppState>,
    mut cmd_rx: mpsc::Receiver<BackgroundCommand>,
) {
    let mut selected_peer_id: Option<String> = None;
    let mut connected = client.health_check().await;
    let mut ws_stream: Option<WsStream> = None;
    let mut ws_reconnect_timer = tokio::time::Instant::now();
    let mut last_full_state: Option<AppState> = None;

    // Initial load
    match &connected {
        HealthResult::Connected => {
            // Try initial WebSocket connection if daemon is up
            match client.connect_websocket().await {
                Ok(ws) => {
                    info!("WebSocket connected to daemon event bus");
                    ws_stream = Some(ws);
                }
                Err(e) => {
                    warn!("Initial WebSocket connection failed (falling back to polling): {}", e);
                }
            }
            let state = build_full_state(&client, &selected_peer_id).await;
            last_full_state = Some(state.clone());
            let _ = state_tx.try_send(state);
        }
        _ => {
            let _ = state_tx.try_send(AppState::disconnected());
        }
    }

    let mut health_interval = tokio::time::interval(Duration::from_secs(2));
    let mut refresh_interval = tokio::time::interval(Duration::from_secs(5));

    // Skip first ticks; we already did initial load
    health_interval.tick().await;
    refresh_interval.tick().await;

    loop {
        // Attempt WebSocket reconnect every 10s if stream is down but daemon is reachable
        let now = tokio::time::Instant::now();
        if ws_stream.is_none()
            && matches!(connected, HealthResult::Connected)
            && now >= ws_reconnect_timer
        {
            ws_reconnect_timer = now + Duration::from_secs(10);
            match client.connect_websocket().await {
                Ok(ws) => {
                    info!("WebSocket reconnected to daemon event bus");
                    ws_stream = Some(ws);
                    // Do a full refresh to sync state after reconnect
                    let state = build_full_state(&client, &selected_peer_id).await;
                    last_full_state = Some(state.clone());
                    let _ = state_tx.try_send(state);
                }
                Err(e) => {
                    debug!("WebSocket reconnect attempt failed: {}", e);
                }
            }
        }

        // Build a future that polls the WebSocket if connected, or never completes
        let ws_next = async {
            if let Some(ws) = ws_stream.as_mut() {
                ws.next().await
            } else {
                std::future::pending().await
            }
        };

        tokio::select! {
            ws_msg = ws_next => {
                match ws_msg {
                    Some(Ok(WsMessage::Text(text))) => {
                        handle_ws_event(&client, &text, &selected_peer_id, &mut last_full_state, &state_tx).await;
                    }
                    Some(Ok(WsMessage::Close(_))) | None => {
                        warn!("WebSocket event bus connection closed — falling back to polling");
                        ws_stream = None;
                        // Signal reconnecting state in-place (keep previous data visible)
                        if let Some(ref prev) = last_full_state {
                            let _ = state_tx.try_send(AppState {
                                connection_health: ConnectionHealth::Reconnecting,
                                ..prev.clone()
                            });
                        }
                    }
                    Some(Err(e)) => {
                        error!("WebSocket event bus error: {}", e);
                        ws_stream = None;
                    }
                    _ => {}
                }
            }
            _ = health_interval.tick() => {
                let health = client.health_check().await;
                let was_connected = matches!(connected, HealthResult::Connected);
                connected = health;
                let is_connected = matches!(connected, HealthResult::Connected);

                if !is_connected {
                    let _ = state_tx.try_send(AppState::disconnected());
                } else if !was_connected && is_connected {
                    // Reconnected to daemon — do full refresh
                    let state = build_full_state(&client, &selected_peer_id).await;
                    last_full_state = Some(state.clone());
                    let _ = state_tx.try_send(state);
                }
            }
            _ = refresh_interval.tick() => {
                if matches!(connected, HealthResult::Connected) {
                    let state = build_full_state(&client, &selected_peer_id).await;
                    last_full_state = Some(state.clone());
                    let _ = state_tx.try_send(state);
                }
            }
            cmd = cmd_rx.recv() => {
                match cmd {
                    Some(BackgroundCommand::SelectPeer(pid)) => {
                        selected_peer_id = pid;
                        if matches!(connected, HealthResult::Connected) {
                            let state = build_full_state(&client, &selected_peer_id).await;
                            last_full_state = Some(state.clone());
                            let _ = state_tx.try_send(state);
                        }
                    }
                    Some(BackgroundCommand::ConnectPeer(node_id)) => {
                        info!("Sending connection request to {}", node_id);
                        client.connect_peer(&node_id).await;
                        if matches!(connected, HealthResult::Connected) {
                            let state = build_full_state(&client, &selected_peer_id).await;
                            last_full_state = Some(state.clone());
                            let _ = state_tx.try_send(state);
                        }
                    }
                    Some(BackgroundCommand::AcceptRequest(request_id)) => {
                        info!("Accepting connection request {}", request_id);
                        client.respond_to_request(&request_id, true).await;
                        if matches!(connected, HealthResult::Connected) {
                            let state = build_full_state(&client, &selected_peer_id).await;
                            last_full_state = Some(state.clone());
                            let _ = state_tx.try_send(state);
                        }
                    }
                    Some(BackgroundCommand::DenyRequest(request_id)) => {
                        info!("Denying connection request {}", request_id);
                        client.respond_to_request(&request_id, false).await;
                        if matches!(connected, HealthResult::Connected) {
                            let state = build_full_state(&client, &selected_peer_id).await;
                            last_full_state = Some(state.clone());
                            let _ = state_tx.try_send(state);
                        }
                    }
                    Some(BackgroundCommand::VerifyPeer(peer_id, key_hex)) => {
                        info!("Verifying peer {} via trust system", peer_id);
                        client.verify_peer(&peer_id, &key_hex).await;
                        if matches!(connected, HealthResult::Connected) {
                            let state = build_full_state(&client, &selected_peer_id).await;
                            last_full_state = Some(state.clone());
                            let _ = state_tx.try_send(state);
                        }
                    }
                    Some(BackgroundCommand::BlockPeer(peer_id)) => {
                        info!("Blocking peer {} via trust system", peer_id);
                        client.block_peer_trust(&peer_id).await;
                        if matches!(connected, HealthResult::Connected) {
                            let state = build_full_state(&client, &selected_peer_id).await;
                            last_full_state = Some(state.clone());
                            let _ = state_tx.try_send(state);
                        }
                    }
                    Some(BackgroundCommand::DisconnectPeer(peer_id)) => {
                        if matches!(connected, HealthResult::Connected) {
                            let _ = client.disconnect_peer(&peer_id).await;
                            let new_state = build_full_state(&client, &selected_peer_id).await;
                            last_full_state = Some(new_state.clone());
                            let _ = state_tx.try_send(new_state);
                        }
                    }
                    Some(BackgroundCommand::AuditRefresh) => {
                        if matches!(connected, HealthResult::Connected) {
                            if let Some(ref mut prev) = last_full_state {
                                prev.audit_log = client.get_audit_log().await;
                                let _ = state_tx.try_send(prev.clone());
                            }
                        }
                    }
                    None | Some(BackgroundCommand::Shutdown) => break,
                }
            }
        }
    }
}

/// Handle a single CoreEvent JSON message received over the WebSocket.
///
/// Performs targeted updates (fetching only what changed) instead of a full
/// state refresh. Falls back to build_full_state for events that touch
/// multiple systems (peer connect/disconnect).
async fn handle_ws_event(
    client: &DaemonClient,
    text: &str,
    selected_peer_id: &Option<String>,
    last_full_state: &mut Option<AppState>,
    state_tx: &mpsc::Sender<AppState>,
) {
    // Parse as generic JSON (we don't share the server-side CoreEvent type)
    let value: serde_json::Value = match serde_json::from_str(text) {
        Ok(v) => v,
        Err(e) => {
            debug!("Failed to parse WebSocket message: {}", e);
            return;
        }
    };

    // Externally-tagged serde enum: {"VariantName": data}
    let obj = match value.as_object() {
        Some(o) => o,
        None => return,
    };
    let (variant, data) = match obj.iter().next() {
        Some(pair) => pair,
        None => return,
    };

    match variant.as_str() {
        "NewMessage" => {
            // Extract sender peer ID from the message envelope
            let sender = match data.get("sender").and_then(|s| s.as_str()) {
                Some(s) => s.to_string(),
                None => return,
            };
            // If the message is for the currently viewed peer, fetch new messages
            if selected_peer_id.as_deref() == Some(&sender) {
                let state = build_full_state(client, selected_peer_id).await;
                *last_full_state = Some(state.clone());
                let _ = state_tx.try_send(state);
            } else {
                // For a different peer, just refresh contacts (unread count updates)
                if let Some(ref prev) = last_full_state {
                    let mut updated = prev.clone();
                    updated.contacts = client.get_contacts().await;
                    updated.swarm_last_refresh = Some(std::time::Instant::now());
                    *last_full_state = Some(updated.clone());
                    let _ = state_tx.try_send(updated);
                }
            }
        }
        "PeerConnected" | "PeerDisconnected" => {
            // Refresh contacts and network status
            let state = build_full_state(client, selected_peer_id).await;
            *last_full_state = Some(state.clone());
            let _ = state_tx.try_send(state);
        }
        "NetworkUpdate" => {
            // Targeted update: only touch peer_count / network fields
            if let Some(ref prev) = last_full_state {
                let mut updated = prev.clone();
                if let Some(peers) = data.get("active_peers").and_then(|v| v.as_u64()) {
                    updated.peer_count = peers as usize;
                    updated.swarm_node_count = peers as usize;
                }
                updated.swarm_last_refresh = Some(std::time::Instant::now());
                *last_full_state = Some(updated.clone());
                let _ = state_tx.try_send(updated);
            }
        }
        "SystemAlert" => {
            if let Some(msg) = data.as_str() {
                warn!("System alert from daemon: {}", msg);
            }
        }
        "SecurityLockdown" => {
            if let Some(locked) = data.as_bool() {
                if locked {
                    warn!("Security lockdown activated by daemon");
                }
            }
        }
        _ => {
            trace!("Unhandled CoreEvent variant: {}", variant);
        }
    }
}

async fn build_full_state(client: &DaemonClient, selected_peer_id: &Option<String>) -> AppState {
    let my_name = client
        .get_identity()
        .await
        .map(|ident| {
            if ident.nickname.is_empty() || ident.nickname == "unnamed" {
                ident.id
            } else {
                ident.nickname
            }
        })
        .unwrap_or_else(|| String::from("connecting..."));

    let contacts = client.get_contacts().await;

    // Ping all connected peers concurrently with 3s timeout per peer
    let connected_peers: Vec<&DaemonContact> = contacts.iter().filter(|c| c.online).collect();
    let mut peer_latency = HashMap::new();
    if !connected_peers.is_empty() {
        let results: Vec<(String, Option<u32>)> = futures_util::future::join_all(
            connected_peers.iter().map(|c| {
                let peer_id = c.peer_id.clone();
                async {
                    match tokio::time::timeout(Duration::from_secs(3), client.ping_peer(&peer_id)).await {
                        Ok(Some(lat)) => (peer_id, Some(lat)),
                        _ => (peer_id, None),
                    }
                }
            }),
        )
        .await;

        for (pid, latency) in results {
            if let Some(lat) = latency {
                peer_latency.insert(pid, lat);
            }
        }
    }

    let net = client.get_network_status().await;

    let ai_status = client.get_ai_status().await;

    let ai_analysis = client.get_mesh_analysis().await;
    let ai_threats = client.get_threat_analysis().await;

    let swarm_node_count = contacts.len();
    let swarm_connected = contacts.iter().filter(|c| c.online).count();
    let swarm_last_refresh = Some(std::time::Instant::now());

    let messages = match selected_peer_id {
        Some(ref pid) if !pid.is_empty() => client.get_messages(pid).await,
        _ => Vec::new(),
    };

    let peer_requests = client.get_peer_requests().await;

    // Fetch trust scores for all known peers
    let trust_scores: HashMap<String, f32> = client
        .get_trust_list()
        .await
        .into_iter()
        .map(|e| (e.peer_id.clone(), e.trust_score()))
        .collect();

    // Fetch public key and compute key fingerprint
    let (key_id, key_fingerprint) = match client.get_public_key().await {
        Some((kid, b64)) => {
            let fp = fingerprint_from_b64(&b64);
            (Some(kid), fp)
        }
        None => (None, None),
    };

    // Fetch audit log
    let audit_log = client.get_audit_log().await;

    AppState {
        connection_health: ConnectionHealth::Connected,
        my_name,
        contacts,
        messages,
        peer_count: net.peer_count,
        network_mode: net.mode,
        encryption_type: net.encryption,
        ai_status,
        ai_analysis,
        ai_threats,
        swarm_node_count,
        swarm_connected,
        swarm_last_refresh,
        peer_latency,
        key_id,
        key_fingerprint,
        peer_requests,
        trust_scores,
        audit_log,
    }
}

// ═══════════════════════════════════════════════════════════
// Public Entry Point
// ═══════════════════════════════════════════════════════════

/// Run the TUI, connecting to the daemon on the given port
pub async fn run_tui(port: u16) -> io::Result<()> {
    let client = DaemonClient::new(port);

    let (state_tx, state_rx) = mpsc::channel::<AppState>(1);
    let (cmd_tx, cmd_rx) = mpsc::channel::<BackgroundCommand>(8);

    let bg_client = client.clone_for_send();

    tokio::spawn(async move {
        background_worker(bg_client, state_tx, cmd_rx).await;
    });

    let mut app = TuiApp::new(client, cmd_tx, state_rx);
    app.run().await
}
