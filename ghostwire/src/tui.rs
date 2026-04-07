//! TUI — Terminal User Interface for GhostWire
//!
//! Interactive terminal interface for managing GhostWire nodes.
//! Provides real-time mesh status, peer list, message history, and message input.

use crate::core::Core;
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
    widgets::{Block, Borders, List, ListItem, Paragraph, Tabs},
    Frame, Terminal,
};
use std::{io, sync::Arc, time::Duration};

/// TUI application state
pub struct TuiApp {
    core: Arc<Core>,
    /// Current tab (0=Dashboard, 1=Peers, 2=Messages, 3=Compose)
    tab: usize,
    /// Message input buffer
    input: String,
    /// Recipient input buffer
    recipient: String,
    /// Whether we're currently in input mode
    input_mode: bool,
    /// Which field is focused (0=recipient, 1=message)
    input_focus: usize,
    /// Cursor position in current input field
    cursor_pos: usize,
    /// Running flag
    running: bool,
    /// Last error message
    error: Option<String>,
    /// Status message
    status: Option<String>,
}

impl TuiApp {
    pub fn new(core: Arc<Core>) -> Self {
        Self {
            core,
            tab: 0,
            input: String::new(),
            recipient: String::new(),
            input_mode: false,
            input_focus: 0,
            cursor_pos: 0,
            running: true,
            error: None,
            status: Some("GhostWire TUI — Press 'q' to quit, Tab to switch tabs, 'i' to input".to_string()),
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

    async fn run_event_loop(&mut self, terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> io::Result<()> {
        loop {
            if !self.running {
                break;
            }

            // Draw UI
            terminal.draw(|f| self.ui(f))?;

            // Poll for events with timeout
            if event::poll(Duration::from_millis(100))? {
                if let Event::Key(key) = event::read()? {
                    if key.kind != KeyEventKind::Press {
                        continue;
                    }

                    if self.input_mode {
                        self.handle_input(key);
                    } else {
                        self.handle_normal(key);
                    }
                }
            }
        }

        Ok(())
    }

    fn handle_normal(&mut self, key: event::KeyEvent) {
        match key.code {
            KeyCode::Char('q') | KeyCode::Char('Q') => {
                self.running = false;
            }
            KeyCode::Tab => {
                self.tab = (self.tab + 1) % 4;
            }
            KeyCode::Char('i') | KeyCode::Char('I') => {
                self.input_mode = true;
                self.input_focus = 0;
                self.cursor_pos = self.recipient.len();
            }
            KeyCode::Char('1') => self.tab = 0,
            KeyCode::Char('2') => self.tab = 1,
            KeyCode::Char('3') => self.tab = 2,
            KeyCode::Char('4') => self.tab = 3,
            _ => {}
        }
    }

    fn handle_input(&mut self, key: event::KeyEvent) {
        let current_input = if self.input_focus == 0 {
            &mut self.recipient
        } else {
            &mut self.input
        };

        match key.code {
            KeyCode::Esc => {
                self.input_mode = false;
                self.error = None;
            }
            KeyCode::Enter => {
                if self.input_focus == 0 {
                    // Move to message input
                    self.input_focus = 1;
                    self.cursor_pos = self.input.len();
                } else {
                    // Send message
                    self.send_message();
                }
            }
            KeyCode::Tab => {
                self.input_focus = if self.input_focus == 0 { 1 } else { 0 };
                let current = if self.input_focus == 0 {
                    &self.recipient
                } else {
                    &self.input
                };
                self.cursor_pos = current.len();
            }
            KeyCode::Backspace => {
                if self.cursor_pos > 0 {
                    current_input.remove(self.cursor_pos - 1);
                    self.cursor_pos -= 1;
                }
            }
            KeyCode::Delete => {
                if self.cursor_pos < current_input.len() {
                    current_input.remove(self.cursor_pos);
                }
            }
            KeyCode::Left => {
                if self.cursor_pos > 0 {
                    self.cursor_pos -= 1;
                }
            }
            KeyCode::Right => {
                if self.cursor_pos < current_input.len() {
                    self.cursor_pos += 1;
                }
            }
            KeyCode::Char(c) => {
                current_input.insert(self.cursor_pos, c);
                self.cursor_pos += 1;
            }
            _ => {}
        }
    }

    fn send_message(&mut self) {
        let recipient = self.recipient.clone();
        let message = self.input.clone();

        if recipient.is_empty() || message.is_empty() {
            self.error = Some("Recipient and message cannot be empty".to_string());
            return;
        }

        let core = self.core.clone();
        let recipient_for_spawn = recipient.clone();
        let message_for_spawn = message.clone();

        // Spawn async send
        tokio::spawn(async move {
            use crate::core::Message;
            let msg = Message::new(
                core.get_identity_id().await,
                recipient_for_spawn.clone(),
                message_for_spawn.clone(),
            );
            match core.send_direct_message(recipient_for_spawn, &msg).await {
                Ok(_) => tracing::info!("Message sent"),
                Err(e) => tracing::error!("Failed to send message: {}", e),
            }
        });

        self.status = Some(format!("Message sent to {}", recipient));
        self.input.clear();
        self.cursor_pos = 0;
        self.input_focus = 0;
        self.input_mode = false;
    }

    fn ui(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),  // Header
                Constraint::Length(3),  // Tabs
                Constraint::Min(10),    // Main content
                Constraint::Length(3),  // Status bar
            ])
            .split(f.area());

        // Header
        let header = Paragraph::new("GhostWire — Secure Mesh Network")
            .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
            .block(Block::default().borders(Borders::ALL).title(" GhostWire TUI "));
        f.render_widget(header, chunks[0]);

        // Tabs
        let tabs = Tabs::new(vec!["Dashboard", "Peers", "Messages", "Compose"])
            .select(self.tab)
            .style(Style::default().fg(Color::White))
            .highlight_style(
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .divider(Span::raw(" | "));
        f.render_widget(tabs, chunks[1]);

        // Main content
        match self.tab {
            0 => self.render_dashboard(f, chunks[2]),
            1 => self.render_peers(f, chunks[2]),
            2 => self.render_messages(f, chunks[2]),
            3 => self.render_compose(f, chunks[2]),
            _ => {}
        }

        // Status bar
        let status_text = if let Some(ref err) = self.error {
            Line::from(vec![
                Span::styled("ERROR: ", Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)),
                Span::raw(err),
            ])
        } else if let Some(ref status) = self.status {
            Line::from(vec![
                Span::styled("STATUS: ", Style::default().fg(Color::Green).add_modifier(Modifier::BOLD)),
                Span::raw(status),
            ])
        } else {
            Line::from("")
        };

        let status_bar = Paragraph::new(status_text)
            .style(Style::default().fg(Color::White))
            .block(Block::default().borders(Borders::ALL).title(" Status "));
        f.render_widget(status_bar, chunks[3]);

        // Show cursor in input mode
        if self.input_mode {
            f.set_cursor_position((
                chunks[2].x + self.cursor_pos as u16 + 2,
                if self.input_focus == 0 {
                    chunks[2].y + 2
                } else {
                    chunks[2].y + 4
                },
            ));
        }
    }

    fn render_dashboard(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(5),  // Node info
                Constraint::Length(5),  // Network status
                Constraint::Min(1),     // Help
            ])
            .split(area);

        // Node info
        let identity_id = tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current().block_on(self.core.get_identity_id())
        });

        let node_info = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Node ID: ", Style::default().fg(Color::Yellow)),
                Span::raw(&identity_id),
            ]),
            Line::from(vec![
                Span::styled("Status: ", Style::default().fg(Color::Yellow)),
                Span::styled("Running", Style::default().fg(Color::Green)),
            ]),
        ])
        .block(Block::default().borders(Borders::ALL).title(" Node Information "));
        f.render_widget(node_info, chunks[0]);

        // Network status
        let network_status = Paragraph::new(vec![
            Line::from(vec![
                Span::styled("Mode: ", Style::default().fg(Color::Yellow)),
                Span::raw("Mesh P2P"),
            ]),
            Line::from(vec![
                Span::styled("Encryption: ", Style::default().fg(Color::Yellow)),
                Span::raw("XChaCha20-Poly1305 + X25519"),
            ]),
        ])
        .block(Block::default().borders(Borders::ALL).title(" Network Status "));
        f.render_widget(network_status, chunks[1]);

        // Help
        let help = Paragraph::new("Tab: Switch tabs | i: Input mode | q: Quit | 1-4: Jump to tab")
            .style(Style::default().fg(Color::DarkGray))
            .block(Block::default().borders(Borders::ALL).title(" Help "));
        f.render_widget(help, chunks[2]);
    }

    fn render_peers(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let peer_list = Paragraph::new(vec![
            Line::from("Peer list will appear here when peers connect."),
            Line::from(""),
            Line::from("Use the mesh network to discover and connect to peers."),
        ])
        .block(Block::default().borders(Borders::ALL).title(" Connected Peers "));
        f.render_widget(peer_list, area);
    }

    fn render_messages(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let msg_list = Paragraph::new(vec![
            Line::from("Message history will appear here."),
            Line::from(""),
            Line::from("Messages are encrypted end-to-end and stored locally."),
        ])
        .block(Block::default().borders(Borders::ALL).title(" Message History "));
        f.render_widget(msg_list, area);
    }

    fn render_compose(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3),  // Recipient
                Constraint::Length(3),  // Message
                Constraint::Min(1),     // Instructions
            ])
            .split(area);

        // Recipient field
        let recipient_style = if self.input_mode && self.input_focus == 0 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };

        let recipient_text = if self.input_mode && self.input_focus == 0 {
            format!("{}|", self.recipient)
        } else {
            format!("Recipient: {}", self.recipient)
        };

        let recipient_widget = Paragraph::new(recipient_text)
            .style(recipient_style)
            .block(Block::default().borders(Borders::ALL).title(" Recipient "));
        f.render_widget(recipient_widget, chunks[0]);

        // Message field
        let message_style = if self.input_mode && self.input_focus == 1 {
            Style::default().fg(Color::Yellow)
        } else {
            Style::default().fg(Color::White)
        };

        let message_text = if self.input_mode && self.input_focus == 1 {
            format!("{}|", self.input)
        } else {
            format!("Message: {}", self.input)
        };

        let message_widget = Paragraph::new(message_text)
            .style(message_style)
            .block(Block::default().borders(Borders::ALL).title(" Message "));
        f.render_widget(message_widget, chunks[1]);

        // Instructions
        let instructions = if self.input_mode {
            "Enter: Next field / Send | Tab: Switch fields | Esc: Cancel"
        } else {
            "Press 'i' to start composing a message"
        };

        let help = Paragraph::new(instructions)
            .style(Style::default().fg(Color::DarkGray))
            .block(Block::default().borders(Borders::ALL).title(" Instructions "));
        f.render_widget(help, chunks[2]);
    }
}

/// Run the TUI with an existing Core
pub async fn run_tui(core: Arc<Core>) -> io::Result<()> {
    let mut app = TuiApp::new(core);
    app.run().await
}
