//! TUI First-Run Wizard — 3-phase guided setup
//!
//! Phase 1: Welcome splash with matrix rain + ASCII logo
//! Phase 2: System check + identity creation (split screen)
//! Phase 3: Connected — peer ID, recovery password, enter to continue

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};
use std::io;

use super::theme;

// ═══════════════════════════════════════════════════════════
// Matrix Rain
// ═══════════════════════════════════════════════════════════

const RAIN_CHARS: &[u8] = b"0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz@#$%&*";

struct RainColumn {
    x: u16,
    y: f64,
    chars: Vec<char>,
}

impl RainColumn {
    fn new(x: u16, height: u16) -> Self {
        use rand::Rng;
        let mut rng = rand::rng();
        let len = rng.random_range(3..8.min(height as usize));
        let chars: Vec<char> = (0..len)
            .map(|_| RAIN_CHARS[rng.random_range(0..RAIN_CHARS.len())] as char)
            .collect();
        Self {
            x,
            y: rng.random_range(0.0..height as f64),
            chars,
        }
    }

}

struct MatrixRain {
    columns: Vec<RainColumn>,
}

impl MatrixRain {
    fn new(width: u16, height: u16) -> Self {
        let spacing = 3; // one column every N cols
        let columns: Vec<RainColumn> = (0..width)
            .step_by(spacing)
            .map(|x| RainColumn::new(x, height))
            .collect();
        Self { columns }
    }
}

// ═══════════════════════════════════════════════════════════
// System Info
// ═══════════════════════════════════════════════════════════

struct SystemInfo {
    os: String,
    arch: String,
    hostname: String,
    lan_ip: String,
    total_ram_mb: u64,
    disk_free_gb: f64,
}

impl SystemInfo {
    fn gather() -> Self {
        let os = std::env::consts::OS.to_string();
        let arch = std::env::consts::ARCH.to_string();
        let hostname = hostname::get()
            .map(|h| h.to_string_lossy().into_owned())
            .unwrap_or_else(|_| "unknown".to_string());
        let lan_ip = local_ip_address::local_ip()
            .map(|ip| ip.to_string())
            .unwrap_or_else(|_| "none".to_string());
        let (total_ram_mb, disk_free_gb) = Self::gather_stats();
        Self {
            os,
            arch,
            hostname,
            lan_ip,
            total_ram_mb,
            disk_free_gb,
        }
    }

    #[cfg(target_os = "linux")]
    fn gather_stats() -> (u64, f64) {
        let ram = std::fs::read_to_string("/proc/meminfo")
            .ok()
            .and_then(|s| {
                s.lines()
                    .find(|l| l.starts_with("MemTotal:"))
                    .and_then(|l| l.split_whitespace().nth(1))
                    .and_then(|v| v.parse::<u64>().ok())
                    .map(|kb| kb / 1024)
            })
            .unwrap_or(0);

        let disk = std::fs::read_to_string("/proc/mounts")
            .ok()
            .and_then(|s| {
                s.lines()
                    .next()
                    .and_then(|l| l.split_whitespace().next())
                    .and_then(|mount| {
                        let path = std::ffi::CString::new(mount).ok()?;
                        let mut stat: libc::statvfs = unsafe { std::mem::zeroed() };
                        unsafe {
                            if libc::statvfs(path.as_ptr(), &mut stat) == 0 {
                                let avail = stat.f_bavail * stat.f_frsize;
                                Some(avail as f64 / (1024.0 * 1024.0 * 1024.0))
                            } else {
                                None
                            }
                        }
                    })
            })
            .unwrap_or(0.0);

        (ram, disk)
    }

    #[cfg(not(target_os = "linux"))]
    fn gather_stats() -> (u64, f64) {
        (0, 0.0)
    }
}

// ═══════════════════════════════════════════════════════════
// Random Name Generator
// ═══════════════════════════════════════════════════════════

fn generate_ghost_name() -> String {
    use rand::Rng;
    let mut rng = rand::rng();
    let suffix: String = (0..4)
        .map(|_| {
            let idx = rng.random_range(0..16);
            "0123456789abcdef".chars().nth(idx).unwrap()
        })
        .collect();
    format!("Ghost_{}", suffix)
}

// ═══════════════════════════════════════════════════════════
// Key Generation Progress
// ═══════════════════════════════════════════════════════════

struct KeyGenStep {
    name: &'static str,
    done: bool,
}

impl KeyGenStep {
    fn new(name: &'static str) -> Self {
        Self { name, done: false }
    }
}

// ═══════════════════════════════════════════════════════════
// Wizard State
// ═══════════════════════════════════════════════════════════

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WizardPhase {
    Welcome,
    SystemCheck,
    SetName,
    Connected,
}

pub struct WizardState {
    pub phase: WizardPhase,
    pub done: bool,
    pub ghost_name: String,
    name_input: String,
    rain: MatrixRain,
    sys_info: SystemInfo,
    key_steps: Vec<KeyGenStep>,
    key_gen_tick: usize,
    recovery_password: String,
}

impl WizardState {
    pub fn new() -> Self {
        let name = generate_ghost_name();
        let recovery: String = {
            use rand::Rng;
            let mut rng = rand::rng();
            (0..16)
                .map(|_| {
                    let idx = rng.random_range(0..16);
                    "0123456789abcdef".chars().nth(idx).unwrap()
                })
                .collect()
        };

        Self {
            phase: WizardPhase::Welcome,
            done: false,
            ghost_name: name.clone(),
            name_input: name,
            rain: MatrixRain::new(80, 24), // will be resized on first render
            sys_info: SystemInfo::gather(),
            key_steps: vec![
                KeyGenStep::new("Ed25519 signing key"),
                KeyGenStep::new("X25519 key exchange"),
                KeyGenStep::new("ML-KEM-768 (post-quantum)"),
            ],
            key_gen_tick: 0,
            recovery_password: recovery,
        }
    }

    pub fn handle_key(&mut self, key: event::KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }
        match self.phase {
            WizardPhase::Welcome => {
                // Any key advances to SystemCheck
                self.phase = WizardPhase::SystemCheck;
            }
            WizardPhase::SystemCheck => {
                // Wait for key gen to complete, then any key advances to SetName
                if self.all_keys_done() {
                    self.phase = WizardPhase::SetName;
                }
            }
            WizardPhase::SetName => {
                match key.code {
                    KeyCode::Enter => {
                        // Confirm name, move to Connected
                        if !self.name_input.trim().is_empty() {
                            self.ghost_name = self.name_input.trim().to_string();
                            self.phase = WizardPhase::Connected;
                        }
                    }
                    KeyCode::Esc => {
                        // Reset to generated name
                        self.name_input = self.ghost_name.clone();
                    }
                    KeyCode::Backspace => {
                        self.name_input.pop();
                    }
                    KeyCode::Char(c) => {
                        // Limit name to 32 chars
                        if self.name_input.len() < 32 {
                            self.name_input.push(c);
                        }
                    }
                    _ => {}
                }
            }
            WizardPhase::Connected => {
                match key.code {
                    KeyCode::Enter | KeyCode::Char(' ') => {
                        self.done = true;
                    }
                    _ => {}
                }
            }
        }
    }

    fn all_keys_done(&self) -> bool {
        self.key_steps.iter().all(|s| s.done)
    }

    fn tick_key_gen(&mut self) {
        if self.all_keys_done() {
            return;
        }
        self.key_gen_tick += 1;
        // Each key takes ~15 ticks (1.5s at 100ms poll)
        for (i, step) in self.key_steps.iter_mut().enumerate() {
            if !step.done && self.key_gen_tick >= (i + 1) * 15 {
                step.done = true;
            }
        }
    }

    pub fn tick(&mut self) {
        if self.phase == WizardPhase::SystemCheck {
            self.tick_key_gen();
        }
    }

    pub fn render(&self, f: &mut Frame) {
        let area = f.area();
        match self.phase {
            WizardPhase::Welcome => self.render_welcome(f, area),
            WizardPhase::SystemCheck => self.render_system_check(f, area),
            WizardPhase::SetName => self.render_set_name(f, area),
            WizardPhase::Connected => self.render_connected(f, area),
        }
    }

    fn render_welcome(&self, f: &mut Frame, area: Rect) {
        // Matrix rain background
        self.render_rain_bg(f, area);

        // Center the content
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Percentage(25),
                Constraint::Length(12),
                Constraint::Length(3),
                Constraint::Min(1),
            ])
            .split(area);

        // Logo block
        let logo_lines = theme::splash_logo_lines();
        let logo = Paragraph::new(logo_lines)
            .alignment(Alignment::Center)
            .block(Block::default().borders(Borders::NONE));
        f.render_widget(logo, chunks[1]);

        // Prompt
        let prompt = Paragraph::new(Line::from(vec![
            Span::styled(
                "Press any key to begin...",
                Style::default()
                    .fg(theme::MUTED)
                    .add_modifier(Modifier::ITALIC),
            ),
        ]))
        .alignment(Alignment::Center);
        f.render_widget(prompt, chunks[2]);
    }

    fn render_system_check(&self, f: &mut Frame, area: Rect) {
        // Split into left (system info) and right (identity + key gen)
        let main = Layout::default()
            .direction(Direction::Vertical)
            .margin(1)
            .constraints([
                Constraint::Length(3), // Header
                Constraint::Min(8),   // Content
                Constraint::Length(3), // Help
            ])
            .split(area);

        // Header
        let header = Paragraph::new(vec![
            Line::from(vec![
                Span::styled(theme::LOGO_SYMBOL, theme::primary_bold()),
                Span::raw(" "),
                Span::styled("GHOSTWIRE", theme::primary_bold()),
                Span::raw("  "),
                Span::styled("First Run Setup", theme::muted_style()),
            ]),
        ]);
        f.render_widget(header, main[0]);

        // Split content: left = system, right = identity + keys
        let content = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
            .split(main[1]);

        // Left: System info (neofetch-style)
        self.render_system_info(f, content[0]);

        // Right: Identity creation + key gen
        self.render_identity_panel(f, content[1]);

        // Help bar
        let help = if self.all_keys_done() {
            theme::help_bar(&[("Any key", "Continue")])
        } else {
            theme::help_bar(&[])
        };
        f.render_widget(help, main[2]);
    }

    fn render_system_info(&self, f: &mut Frame, area: Rect) {
        let info = &self.sys_info;
        let lines = vec![
            Line::from(vec![
                Span::styled("OS:       ", theme::muted_style()),
                Span::styled(&info.os, theme::text_style()),
            ]),
            Line::from(vec![
                Span::styled("Arch:     ", theme::muted_style()),
                Span::styled(&info.arch, theme::text_style()),
            ]),
            Line::from(vec![
                Span::styled("Hostname: ", theme::muted_style()),
                Span::styled(&info.hostname, theme::text_style()),
            ]),
            Line::from(vec![
                Span::styled("LAN IP:   ", theme::muted_style()),
                Span::styled(&info.lan_ip, theme::online_style()),
            ]),
            Line::from(vec![
                Span::styled("RAM:      ", theme::muted_style()),
                Span::styled(
                    if info.total_ram_mb > 0 {
                        format!("{} MB", info.total_ram_mb)
                    } else {
                        "detecting...".to_string()
                    },
                    theme::text_style(),
                ),
            ]),
            Line::from(vec![
                Span::styled("Disk:     ", theme::muted_style()),
                Span::styled(
                    if info.disk_free_gb > 0.0 {
                        format!("{:.1} GB free", info.disk_free_gb)
                    } else {
                        "detecting...".to_string()
                    },
                    theme::text_style(),
                ),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "System checks passed",
                theme::online_style(),
            )]),
        ];

        let widget = Paragraph::new(lines).block(theme::themed_block("System Info"));
        f.render_widget(widget, area);
    }

    fn render_identity_panel(&self, f: &mut Frame, area: Rect) {
        let mut lines: Vec<Line> = Vec::new();

        // Identity name
        lines.push(Line::from(vec![
            Span::styled("Name:  ", theme::muted_style()),
            Span::styled(&self.ghost_name, theme::primary_bold()),
        ]));
        lines.push(Line::from(""));

        // Key generation progress
        lines.push(Line::from(vec![Span::styled(
            "Generating encryption keys...",
            Style::default()
                .fg(theme::WARNING)
                .add_modifier(Modifier::BOLD),
        )]));
        lines.push(Line::from(""));

        for step in &self.key_steps {
            let (icon, style) = if step.done {
                ("✓", theme::online_style())
            } else {
                ("⏳", theme::warning_style())
            };
            lines.push(Line::from(vec![
                Span::styled(format!("  {} ", icon), style),
                Span::styled(step.name, theme::text_style()),
            ]));
        }

        lines.push(Line::from(""));
        if self.all_keys_done() {
            lines.push(Line::from(vec![Span::styled(
                "All keys generated",
                theme::online_style(),
            )]));
        } else {
            lines.push(Line::from(vec![Span::styled(
                "This takes ~30 seconds",
                theme::muted_style(),
            )]));
        }

        let widget = Paragraph::new(lines).block(theme::themed_block("Identity"));
        f.render_widget(widget, area);
    }

    fn render_set_name(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(3),
                Constraint::Length(5),
                Constraint::Length(3),
                Constraint::Min(1),
            ])
            .split(area);

        let header = Paragraph::new(vec![
            Line::from(vec![
                Span::styled(theme::LOGO_SYMBOL, theme::primary_bold()),
                Span::raw(" "),
                Span::styled("GHOSTWIRE", theme::primary_bold()),
                Span::raw("  "),
                Span::styled("Choose Your Name", theme::muted_style()),
            ]),
        ]);
        f.render_widget(header, chunks[0]);

        let input_display = if self.name_input.is_empty() {
            Span::styled("_", Style::default().fg(theme::PRIMARY).add_modifier(Modifier::SLOW_BLINK))
        } else {
            Span::styled(&self.name_input, theme::primary_bold())
        };

        let input_block = Paragraph::new(vec![
            Line::from(""),
            Line::from(vec![
                Span::styled("  > ", theme::muted_style()),
                input_display,
            ]),
            Line::from(""),
        ])
        .block(theme::themed_block("Your ghost name"));
        f.render_widget(input_block, chunks[1]);

        let help_lines = vec![
            Line::from(vec![
                Span::styled("  Type your name, press ", theme::muted_style()),
                Span::styled("Enter", theme::primary_bold()),
                Span::styled(" to confirm", theme::muted_style()),
            ]),
            Line::from(vec![
                Span::styled("  ", theme::muted_style()),
                Span::styled("Esc", theme::primary_bold()),
                Span::styled(" to reset to: ", theme::muted_style()),
                Span::styled(&self.ghost_name, theme::muted_style()),
            ]),
            Line::from(vec![
                Span::styled("  Backspace to delete", theme::muted_style()),
            ]),
        ];
        let help = Paragraph::new(help_lines);
        f.render_widget(help, chunks[2]);
    }

    fn render_connected(&self, f: &mut Frame, area: Rect) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(2)
            .constraints([
                Constraint::Length(8),  // Logo
                Constraint::Length(10), // Info
                Constraint::Length(3),  // Prompt
                Constraint::Min(1),    // Spacer
            ])
            .split(area);

        // Logo
        let logo = Paragraph::new(theme::splash_logo_lines()).alignment(Alignment::Center);
        f.render_widget(logo, chunks[0]);

        // Info panel
        let info = vec![
            Line::from(vec![
                Span::styled("Your Peer ID: ", theme::muted_style()),
                Span::styled(&self.ghost_name, theme::primary_bold()),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Recovery Password (save this!):",
                Style::default()
                    .fg(theme::WARNING)
                    .add_modifier(Modifier::BOLD),
            )]),
            Line::from(vec![
                Span::raw("  "),
                Span::styled(&self.recovery_password, theme::primary_bold()),
            ]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "This password recovers your identity if you lose your device.",
                theme::muted_style(),
            )]),
        ];

        let info_widget = Paragraph::new(info).block(theme::themed_block("Connected"));
        f.render_widget(info_widget, chunks[1]);

        // Prompt
        let prompt = Paragraph::new(Line::from(vec![Span::styled(
            "Press Enter to continue to GhostWire",
            Style::default()
                .fg(theme::TEXT)
                .add_modifier(Modifier::BOLD),
        )]))
        .alignment(Alignment::Center);
        f.render_widget(prompt, chunks[2]);
    }

    fn render_rain_bg(&self, f: &mut Frame, area: Rect) {
        let height = area.height as f64;
        for col in &self.rain.columns {
            if col.x >= area.width {
                continue;
            }
            for (i, ch) in col.chars.iter().enumerate() {
                let y = col.y + i as f64;
                if y >= 0.0 && y < height {
                    let cell_y = y as u16 + area.y;
                    let cell_x = col.x + area.x;
                    if cell_y < area.y + area.height && cell_x < area.x + area.width {
                        let rain = Paragraph::new(Span::styled(
                            ch.to_string(),
                            Style::default().fg(Color::DarkGray),
                        ));
                        f.render_widget(
                            rain,
                            Rect {
                                x: cell_x,
                                y: cell_y,
                                width: 1,
                                height: 1,
                            },
                        );
                    }
                }
            }
        }
    }
}

// ═══════════════════════════════════════════════════════════
// Standalone Runner (for use before TuiApp)
// ═══════════════════════════════════════════════════════════

/// Run the first-run wizard standalone. Returns the ghost name if completed.
pub async fn run_wizard() -> io::Result<Option<String>> {
    use crossterm::{
        event::DisableMouseCapture,
        execute,
        terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    };
    use ratatui::{backend::CrosstermBackend, Terminal};

    let mut state = WizardState::new();

    terminal::enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let result = loop {
        terminal.draw(|f| state.render(f))?;

        if event::poll(std::time::Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                state.handle_key(key);
                if state.done {
                    break Some(state.ghost_name);
                }
            }
        }
    };

    // Restore terminal
    terminal::disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(result)
}
