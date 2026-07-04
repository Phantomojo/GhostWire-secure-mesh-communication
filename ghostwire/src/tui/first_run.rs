//! TUI First-Run Wizard
//!
//! Guided setup experience for new GhostWire users.
//! Collects display name, generates identity keys, saves profile.

use crate::core::Core;
use crossterm::{
    event::{self, KeyCode},
    execute,
};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::io;

use super::theme;

/// First-run wizard state
pub struct FirstRunWizard {
    /// Display name being typed
    name_input: String,
    /// Cursor position in name input
    cursor_pos: usize,
    /// Whether setup is complete
    done: bool,
    /// Error message to display
    error: Option<String>,
    /// Setup phase: 0=welcome, 1=name input, 2=generating, 3=done
    phase: usize,
}

impl FirstRunWizard {
    pub fn new() -> Self {
        Self {
            name_input: String::new(),
            cursor_pos: 0,
            done: false,
            error: None,
            phase: 0, // Start at welcome
        }
    }

    pub fn is_done(&self) -> bool {
        self.done
    }

    pub fn display_name(&self) -> &str {
        &self.name_input
    }

    /// Handle key input during wizard
    pub fn handle_key(&mut self, key: crossterm::event::KeyEvent) {
        match self.phase {
            0 => {
                // Welcome screen — any key advances
                self.phase = 1;
            }
            1 => {
                // Name input phase
                match key.code {
                    KeyCode::Enter => {
                        let name = self.name_input.trim().to_string();
                        if name.is_empty() {
                            self.error = Some("Please enter a display name".to_string());
                            return;
                        }
                        if name.len() > 32 {
                            self.error = Some("Name must be 32 characters or less".to_string());
                            return;
                        }
                        self.phase = 2; // Move to generating
                    }
                    KeyCode::Esc => {
                        // Don't allow exit — they must pick a name
                    }
                    KeyCode::Char(c) => {
                        if self.name_input.len() < 32 {
                            self.name_input.insert(self.cursor_pos, c);
                            self.cursor_pos += 1;
                            self.error = None;
                        }
                    }
                    KeyCode::Backspace => {
                        if self.cursor_pos > 0 {
                            self.name_input.remove(self.cursor_pos - 1);
                            self.cursor_pos -= 1;
                            self.error = None;
                        }
                    }
                    KeyCode::Delete => {
                        if self.cursor_pos < self.name_input.len() {
                            self.name_input.remove(self.cursor_pos);
                        }
                    }
                    KeyCode::Left => {
                        if self.cursor_pos > 0 {
                            self.cursor_pos -= 1;
                        }
                    }
                    KeyCode::Right => {
                        if self.cursor_pos < self.name_input.len() {
                            self.cursor_pos += 1;
                        }
                    }
                    _ => {}
                }
            }
            2 => {
                // Generating phase — no input, just wait
            }
            3 => {
                // Done phase — any key continues to main TUI
                self.done = true;
            }
            _ => {}
        }
    }

    /// Render the wizard screen
    pub fn render(&self, f: &mut Frame) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Min(8),    // Logo + content
                Constraint::Length(3), // Help bar
            ])
            .split(f.area());

        match self.phase {
            0 => self.render_welcome(f, chunks[0]),
            1 => self.render_name_input(f, chunks[0]),
            2 => self.render_generating(f, chunks[0]),
            3 => self.render_done(f, chunks[0]),
            _ => {}
        }

        // Help bar
        let help = match self.phase {
            0 => theme::help_bar(&[("Any key", "Continue")]),
            1 => theme::help_bar(&[("Type", "Your name"), ("Enter", "Continue")]),
            2 => theme::help_bar(&[]),
            3 => theme::help_bar(&[("Any key", "Enter GhostWire")]),
            _ => theme::help_bar(&[]),
        };
        f.render_widget(help, chunks[1]);
    }

    fn render_welcome(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let logo_lines = theme::splash_logo_lines();
        let logo = Paragraph::new(logo_lines).block(Block::default().borders(Borders::NONE));
        f.render_widget(logo, area);
    }

    fn render_name_input(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(6), // Logo (smaller)
                Constraint::Length(3), // Prompt
                Constraint::Length(3), // Input field
                Constraint::Length(1), // Error
                Constraint::Min(1),    // Spacer
            ])
            .split(area);

        // Small logo
        let logo = Paragraph::new(vec![
            Line::from(vec![
                Span::styled(theme::LOGO_SYMBOL, theme::primary_bold()),
                Span::raw(" "),
                Span::styled("GHOSTWIRE", theme::primary_bold()),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Secure Mesh Network",
                theme::muted_style(),
            )]),
        ]);
        f.render_widget(logo, chunks[0]);

        // Prompt
        let prompt = Paragraph::new(vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "Choose how others will see you on the mesh:",
                Style::default().fg(Color::White),
            )]),
        ]);
        f.render_widget(prompt, chunks[1]);

        // Input field
        let cursor_char = if self.cursor_pos >= self.name_input.len() {
            "█".to_string()
        } else {
            format!("█{}", &self.name_input[self.cursor_pos..])
        };
        let before_cursor = &self.name_input[..self.cursor_pos];

        let input_text = if self.cursor_pos >= self.name_input.len() {
            format!(" {} ", self.name_input)
        } else {
            format!(" {before_cursor}{cursor_char} ")
        };

        let input_widget = Paragraph::new(input_text)
            .style(Style::default().fg(Color::White))
            .block(
                Block::default()
                    .title(" Display Name ")
                    .borders(Borders::ALL)
                    .border_type(ratatui::widgets::BorderType::Rounded)
                    .border_style(theme::primary_bold()),
            );
        f.render_widget(input_widget, chunks[2]);

        // Error message
        if let Some(ref err) = self.error {
            let error_text = Paragraph::new(vec![Line::from(vec![
                Span::styled("⚠ ", theme::error_style()),
                Span::styled(err, theme::error_style()),
            ])])
            .style(theme::error_style());
            f.render_widget(error_text, chunks[3]);
        }

        // Hint text
        let hint = Paragraph::new(vec![Line::from(vec![Span::styled(
            "This is your display name. You can change it later. Use a name people will recognize.",
            theme::muted_style(),
        )])]);
        f.render_widget(hint, chunks[4]);
    }

    fn render_generating(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(6), Constraint::Min(1)])
            .split(area);

        let logo = Paragraph::new(vec![
            Line::from(vec![
                Span::styled(theme::LOGO_SYMBOL, theme::primary_bold()),
                Span::raw(" "),
                Span::styled("GHOSTWIRE", theme::primary_bold()),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Secure Mesh Network",
                theme::muted_style(),
            )]),
        ]);
        f.render_widget(logo, chunks[0]);

        let generating = Paragraph::new(vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "🔑  Generating your encryption keys...",
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "This creates your unique identity on the mesh.",
                theme::muted_style(),
            )]),
            Line::from(vec![Span::styled(
                "Your keys are saved locally and never leave this device.",
                theme::muted_style(),
            )]),
        ]);
        f.render_widget(generating, chunks[1]);
    }

    fn render_done(&self, f: &mut Frame, area: ratatui::layout::Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([Constraint::Length(6), Constraint::Min(1)])
            .split(area);

        let logo = Paragraph::new(vec![
            Line::from(vec![
                Span::styled(theme::LOGO_SYMBOL, theme::primary_bold()),
                Span::raw(" "),
                Span::styled("GHOSTWIRE", theme::primary_bold()),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Secure Mesh Network",
                theme::muted_style(),
            )]),
        ]);
        f.render_widget(logo, chunks[0]);

        let done = Paragraph::new(vec![
            Line::from(""),
            Line::from(vec![Span::styled(
                "✅  Welcome to GhostWire!",
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(""),
            Line::from(vec![
                Span::styled("Your name: ", theme::muted_style()),
                Span::styled(&self.name_input, theme::primary_bold()),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Your identity is ready. Press any key to enter the mesh.",
                theme::muted_style(),
            )]),
        ]);
        f.render_widget(done, chunks[1]);
    }

    /// Complete setup — call after phase 2 async work is done
    pub fn mark_generating_done(&mut self) {
        self.phase = 3;
    }
}

/// Run the first-run wizard, returning the chosen display name.
/// Returns None if the user already has a profile.
pub async fn run_first_run(core: &Core) -> io::Result<Option<String>> {
    // Check if profile already exists
    if core.has_profile().await {
        return Ok(None);
    }

    use crossterm::{
        event::{Event, KeyEventKind},
        terminal,
    };
    use ratatui::{backend::CrosstermBackend, Terminal};
    use std::io::stdout;

    let mut wizard = FirstRunWizard::new();

    // Setup terminal
    terminal::enable_raw_mode()?;
    let mut stdout = stdout();
    execute!(
        stdout,
        terminal::EnterAlternateScreen,
        event::EnableMouseCapture
    )?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    loop {
        terminal.draw(|f| wizard.render(f))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind != KeyEventKind::Press {
                    continue;
                }

                // If we're in the generating phase, actually save the profile
                if wizard.phase == 2 && !wizard.display_name().is_empty() {
                    let name = wizard.display_name().trim().to_string();
                    if let Err(e) = core.save_profile(&name).await {
                        tracing::error!("Failed to save profile: {}", e);
                    }
                    wizard.mark_generating_done();
                    continue;
                }

                wizard.handle_key(key);

                if wizard.is_done() {
                    break;
                }
            }
        }
    }

    // Restore terminal
    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        terminal::LeaveAlternateScreen,
        event::DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(Some(wizard.display_name().to_string()))
}
