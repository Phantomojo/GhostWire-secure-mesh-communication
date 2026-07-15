//! TUI Application — main state, event loop, and screen rendering.
//!
//! Connects to the GhostWire daemon via HTTP API (no direct Core access).
//! This means the daemon stays alive when the TUI closes, and multiple
//! TUI instances can connect simultaneously.

use super::daemon_client::{DaemonClient, DaemonContact, DaemonMessage};
use super::theme;
use super::wizard::WizardState;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEventKind},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
    Frame, Terminal,
};
use std::{io, time::Duration};

// ═══════════════════════════════════════════════════════════
// Application State
// ═══════════════════════════════════════════════════════════

/// Which screen/view is active
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum View {
    Chat,
    Contacts,
    Dashboard,
    Wizard,
}

/// TUI application state
pub struct TuiApp {
    /// HTTP client for the daemon
    client: DaemonClient,
    /// Current view
    view: View,
    /// Selected contact index in the sidebar
    selected_contact: Option<usize>,
    /// Contacts list (fetched from daemon)
    contacts: Vec<DaemonContact>,
    /// Messages in the current conversation
    messages: Vec<DaemonMessage>,
    /// Message input buffer
    input: String,
    /// Cursor position in input
    cursor_pos: usize,
    /// Whether we're in input mode (typing)
    input_mode: bool,
    /// Running flag
    running: bool,
    /// Error message
    error: Option<String>,
    /// Status / toast message
    status: Option<String>,
    /// My display name
    my_name: String,
    /// Peer count (fetched from daemon)
    peer_count: usize,
    /// Total unread messages
    total_unread: usize,
    /// Network mode
    network_mode: String,
    /// Encryption type
    encryption_type: String,
    ai_status: String,
    wizard: Option<WizardState>,
    start_in_wizard: bool,
}

impl TuiApp {
    pub fn new(client: DaemonClient, start_in_wizard: bool) -> Self {
        let view = if start_in_wizard {
            View::Wizard
        } else {
            View::Chat
        };
        Self {
            client,
            view,
            selected_contact: None,
            contacts: Vec::new(),
            messages: Vec::new(),
            input: String::new(),
            cursor_pos: 0,
            input_mode: false,
            running: true,
            error: None,
            status: None,
            my_name: String::from("connecting..."),
            peer_count: 0,
            total_unread: 0,
            network_mode: String::from("Mesh P2P"),
            encryption_type: String::from("XChaCha20-Poly1305"),
            ai_status: String::from("Checking..."),
            wizard: if start_in_wizard {
                Some(WizardState::new())
            } else {
                None
            },
            start_in_wizard,
        }
    }

    /// Run the TUI application
    pub async fn run(&mut self) -> io::Result<()> {
        if !self.start_in_wizard {
            if !self.client.health_check().await {
                return Err(io::Error::new(
                    io::ErrorKind::ConnectionRefused,
                    format!(
                        "Cannot connect to GhostWire daemon on port {}.\n\
                         Run 'ghostwire start' first, or check 'ghostwire status'.",
                        self.client.port()
                    ),
                ));
            }
            self.refresh_data().await;
        }

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
        let mut refresh_tick = 0;

        loop {
            if !self.running {
                break;
            }

            if self.view == View::Wizard {
                if let Some(ref mut wizard) = self.wizard {
                    wizard.tick();
                }
            } else {
                refresh_tick += 1;
                if refresh_tick >= 30 {
                    refresh_tick = 0;
                    self.refresh_data().await;
                }
            }

            terminal.draw(|f| self.ui(f))?;

            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }
                    self.handle_key(key);
                }
            }
        }

        Ok(())
    }

    // ═══════════════════════════════════════════════════
    // Data Refresh (via daemon API)
    // ═══════════════════════════════════════════════════

    async fn refresh_data(&mut self) {
        let identity_fut = self.client.get_identity();
        let contacts_fut = self.client.get_contacts();
        let net_fut = self.client.get_network_status();
        let ai_fut = self.client.get_ai_status_summary();

        let (identity, contacts, net, ai_status) =
            tokio::join!(identity_fut, contacts_fut, net_fut, ai_fut);

        if let Some(identity) = identity {
            self.my_name = if identity.nickname.is_empty() {
                identity.id
            } else {
                identity.nickname
            };
        }

        self.contacts = contacts;
        self.total_unread = self.contacts.iter().map(|c| c.unread_count).sum();

        self.peer_count = net.peer_count;
        self.network_mode = net.mode;
        self.encryption_type = net.encryption;

        self.ai_status = ai_status;

        if self.contacts.is_empty() {
            self.contacts.push(DaemonContact {
                display_name: "Waiting for mesh peers...".to_string(),
                peer_id: String::new(),
                online: false,
                trust_score: 0.0,
                unread_count: 0,
                last_seen_secs: 0,
            });
        }

        if let Some(idx) = self.selected_contact {
            if idx < self.contacts.len() {
                let peer_id = self.contacts[idx].peer_id.clone();
                if !peer_id.is_empty() {
                    self.messages = self.client.get_messages(&peer_id).await;
                }
            }
        }
    }

    // ═══════════════════════════════════════════════════
    // Input Handling
    // ═══════════════════════════════════════════════════

    fn handle_key(&mut self, key: event::KeyEvent) {
        if self.view == View::Wizard {
            if let Some(ref mut wizard) = self.wizard {
                wizard.handle_key(key);
                if wizard.done {
                    self.view = View::Chat;
                    self.wizard = None;
                }
            }
            return;
        }
        if self.input_mode {
            self.handle_input_key(key);
        } else {
            self.handle_normal_key(key);
        }
    }

    fn handle_normal_key(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.running = false;
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                // Enter input mode
                self.input_mode = true;
                self.cursor_pos = self.input.len();
            }
            KeyCode::Char('1') => self.view = View::Chat,
            KeyCode::Char('2') => self.view = View::Contacts,
            KeyCode::Char('3') => self.view = View::Dashboard,
            KeyCode::Tab => {
                // Cycle views
                self.view = match self.view {
                    View::Chat => View::Contacts,
                    View::Contacts => View::Dashboard,
                    View::Dashboard => View::Chat,
                    View::Wizard => View::Chat,
                };
            }
            KeyCode::Up => {
                if let Some(idx) = self.selected_contact {
                    if idx > 0 {
                        self.selected_contact = Some(idx - 1);
                    }
                } else if !self.contacts.is_empty() {
                    self.selected_contact = Some(0);
                }
            }
            KeyCode::Down => {
                if let Some(idx) = self.selected_contact {
                    if idx + 1 < self.contacts.len() {
                        self.selected_contact = Some(idx + 1);
                    }
                } else if !self.contacts.is_empty() {
                    self.selected_contact = Some(0);
                }
            }
            KeyCode::Enter => {
                if let Some(idx) = self.selected_contact {
                    if idx < self.contacts.len() {
                        let contact = &self.contacts[idx];
                        if !contact.peer_id.is_empty() {
                            self.selected_contact = Some(idx);
                            self.input_mode = true;
                            self.cursor_pos = self.input.len();
                            let peer_id = contact.peer_id.clone();
                            let client = self.client.clone_for_send();
                            let rt = tokio::runtime::Handle::current();
                            let messages =
                                rt.block_on(async { client.get_messages(&peer_id).await });
                            self.messages = messages;
                        }
                    }
                }
            }
            KeyCode::Esc => {
                self.error = None;
                self.status = None;
            }
            _ => {}
        }
    }

    fn handle_input_key(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Esc => {
                self.input_mode = false;
                self.error = None;
            }
            KeyCode::Enter => {
                if !self.input.trim().is_empty() {
                    self.send_message();
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

        if idx >= self.contacts.len() {
            self.error = Some("Invalid contact selection".to_string());
            return;
        }

        let contact = &self.contacts[idx];
        if contact.peer_id.is_empty() {
            self.error = Some("No valid peer selected".to_string());
            return;
        }

        let recipient = contact.peer_id.clone();
        let my_name = self.my_name.clone();
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
        self.input.clear();
        self.cursor_pos = 0;
        self.input_mode = false;
    }

    // ═══════════════════════════════════════════════════
    // UI Rendering
    // ═══════════════════════════════════════════════════

    fn ui(&self, f: &mut Frame) {
        match self.view {
            View::Chat => self.render_chat_view(f),
            View::Contacts => self.render_contacts_view(f),
            View::Dashboard => self.render_dashboard_view(f),
            View::Wizard => {
                if let Some(ref wizard) = self.wizard {
                    wizard.render(f);
                }
            }
        }
    }

    fn render_chat_view(&self, f: &mut Frame) {
        let area = f.area();

        // Main layout: status bar | sidebar+content | input | help
        let main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1), // Status bar
                Constraint::Min(8),    // Main content (sidebar + conversation)
                Constraint::Length(3), // Input area
                Constraint::Length(3), // Help bar
            ])
            .split(area);

        // Status bar
        self.render_status_bar(f, main[0]);

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

    fn render_status_bar(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let status_text = Line::from(vec![
            Span::styled(theme::LOGO_SYMBOL, theme::primary_bold()),
            Span::raw(" "),
            Span::styled(&self.my_name, theme::primary_bold()),
            Span::raw("  "),
            theme::status_dot(self.peer_count > 0),
            Span::raw(" "),
            Span::raw(format!("{} peer", self.peer_count)),
            if self.peer_count != 1 {
                Span::raw("s")
            } else {
                Span::raw("")
            },
            Span::raw("  "),
            if self.total_unread > 0 {
                Span::styled(
                    format!("{} unread", self.total_unread),
                    Style::default()
                        .fg(Color::Yellow)
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled("0 unread", theme::muted_style())
            },
        ]);

        let status = Paragraph::new(status_text).block(Block::default().borders(Borders::NONE));
        f.render_widget(status, area);
    }

    fn render_sidebar(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let items: Vec<ListItem> = self
            .contacts
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

                let name_line = Line::from(vec![
                    theme::status_dot(c.online),
                    Span::raw(" "),
                    Span::styled(&c.display_name, style),
                    Span::styled(unread_indicator, theme::warning_style()),
                ]);

                ListItem::new(name_line)
            })
            .collect();

        let contact_list = List::new(items).block(theme::themed_block("Contacts"));
        f.render_widget(contact_list, area);
    }

    fn render_conversation(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let contact_name = self
            .selected_contact
            .and_then(|i| self.contacts.get(i))
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
            let msg_lines: Vec<Line> = self
                .messages
                .iter()
                .flat_map(|m| {
                    let time = format_time(m.timestamp);
                    let sender_style = if m.is_mine {
                        theme::sent_style()
                    } else {
                        theme::primary_bold()
                    };
                    let prefix = if m.is_mine { "→" } else { "←" };

                    vec![
                        Line::from(vec![
                            Span::styled(prefix, theme::muted_style()),
                            Span::raw(" "),
                            Span::raw(time),
                            Span::raw(" "),
                            Span::styled(m.sender.clone(), sender_style),
                        ]),
                        Line::from(vec![
                            Span::raw("   "),
                            Span::styled(m.content.clone(), theme::text_style()),
                        ]),
                        Line::from(""), // spacing between messages
                    ]
                })
                .collect();

            let conversation = Paragraph::new(msg_lines).block(theme::themed_block(contact_name));
            f.render_widget(conversation, area);
        }
    }

    fn render_input_area(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let input_text = if self.input_mode {
            // Show cursor
            let before = &self.input[..self.cursor_pos];
            let after = &self.input[self.cursor_pos..];
            format!(" {before}█{after}")
        } else if self.input.is_empty() {
            " Press 'i' to type a message...".to_string()
        } else {
            format!(" {}", self.input)
        };

        let border_style = if self.input_mode {
            theme::primary_bold()
        } else {
            Style::default().fg(theme::MUTED)
        };

        let input_widget = Paragraph::new(input_text)
            .style(if self.input_mode {
                theme::text_style()
            } else {
                theme::muted_style()
            })
            .block(
                Block::default()
                    .title(if self.input_mode {
                        " Message "
                    } else {
                        " Type to chat "
                    })
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(border_style),
            );
        f.render_widget(input_widget, area);
    }

    fn render_help_bar(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let hints = if self.input_mode {
            theme::help_bar(&[("Enter", "Send"), ("Esc", "Cancel"), ("←→", "Move cursor")])
        } else {
            theme::help_bar(&[
                ("↑↓", "Navigate"),
                ("Enter", "Open chat"),
                ("i", "Type message"),
                ("Tab", "Switch view"),
                ("1/2/3", "Chat/Contacts/Dashboard"),
                ("q", "Quit"),
            ])
        };
        f.render_widget(hints, area);
    }

    fn render_contacts_view(&self, f: &mut Frame) {
        let area = f.area();
        let main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(8),
                Constraint::Length(3),
            ])
            .split(area);

        self.render_status_bar(f, main[0]);

        // Contact list with details
        let contact_items: Vec<ListItem> = self
            .contacts
            .iter()
            .map(|c| {
                let status_text = if c.online {
                    "online".to_string()
                } else {
                    format!("last seen {} ago", format_duration(c.last_seen_secs))
                };
                let name_line = Line::from(vec![
                    theme::status_dot(c.online),
                    Span::raw(" "),
                    Span::styled(&c.display_name, theme::text_style()),
                    Span::raw("  "),
                    Span::styled(status_text, theme::muted_style()),
                ]);
                let trust_line = theme::trust_bar_line(c.trust_score, &c.display_name);

                ListItem::new(vec![name_line, trust_line])
            })
            .collect();

        let list = List::new(contact_items).block(theme::themed_block("All Contacts"));
        f.render_widget(list, main[1]);

        let help = theme::help_bar(&[
            ("↑↓", "Navigate"),
            ("Enter", "Message"),
            ("Tab", "Switch view"),
            ("q", "Quit"),
        ]);
        f.render_widget(help, main[2]);
    }

    fn render_dashboard_view(&self, f: &mut Frame) {
        let area = f.area();
        let main = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(1),
                Constraint::Min(8),
                Constraint::Length(3),
            ])
            .split(area);

        self.render_status_bar(f, main[0]);

        // Dashboard content
        let content = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(5), // Node info
                Constraint::Length(5), // Network
                Constraint::Min(1),    // Spacer
            ])
            .split(main[1]);

        let node_info = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Name:      ", theme::muted_style()),
                Span::styled(&self.my_name, theme::primary_bold()),
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

        let network = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Mode:      ", theme::muted_style()),
                Span::styled(&self.network_mode, theme::text_style()),
            ]),
            Line::from(vec![
                Span::styled("Peers:     ", theme::muted_style()),
                Span::styled(
                    format!("{} connected", self.peer_count),
                    theme::online_style(),
                ),
            ]),
            Line::from(vec![
                Span::styled("AI Layer:  ", theme::muted_style()),
                Span::styled(&self.ai_status, theme::text_style()),
            ]),
        ])
        .block(theme::themed_block("Network"));
        f.render_widget(network, content[1]);

        let help = theme::help_bar(&[
            ("Tab", "Switch view"),
            ("1/2/3", "Chat/Contacts/Dashboard"),
            ("q", "Quit"),
        ]);
        f.render_widget(help, main[2]);
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

// ═══════════════════════════════════════════════════════════
// Public Entry Point
// ═══════════════════════════════════════════════════════════

pub async fn run_tui(port: u16, start_in_wizard: bool) -> io::Result<()> {
    let client = DaemonClient::new(port);
    let mut app = TuiApp::new(client, start_in_wizard);
    app.run().await
}
