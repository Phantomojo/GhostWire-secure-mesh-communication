//! TUI Theme — GhostWire visual identity
//!
//! Centralized color palette, styles, border types, and logo rendering.
//! Every widget in the TUI uses these styles for consistency.

use ratatui::{
    layout::Alignment,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Paragraph},
};

// ═══════════════════════════════════════════════════════════
// Color Palette
// ═══════════════════════════════════════════════════════════

/// Primary brand color — used for logo, headers, active elements
pub const PRIMARY: Color = Color::Cyan;

/// Online / connected / success
pub const ONLINE: Color = Color::Green;

/// Warning / degraded / pending
pub const WARNING: Color = Color::Yellow;

/// Offline / disconnected / inactive
pub const OFFLINE: Color = Color::DarkGray;

/// Error / failure / security alert
pub const ERROR: Color = Color::Red;

/// Primary text content
pub const TEXT: Color = Color::White;

/// Muted text — timestamps, help, secondary info
pub const MUTED: Color = Color::Gray;

/// Your (sent) messages — distinct from received
pub const SENT: Color = Color::Magenta;

/// Background for selected/highlighted items
pub const HIGHLIGHT_BG: Color = Color::DarkGray;

// ═══════════════════════════════════════════════════════════
// Style Constructors
// ═══════════════════════════════════════════════════════════

pub fn primary_bold() -> Style {
    Style::default().fg(PRIMARY).add_modifier(Modifier::BOLD)
}

pub fn text_style() -> Style {
    Style::default().fg(TEXT)
}

pub fn muted_style() -> Style {
    Style::default().fg(MUTED)
}

pub fn online_style() -> Style {
    Style::default().fg(ONLINE)
}

pub fn warning_style() -> Style {
    Style::default().fg(WARNING)
}

pub fn offline_style() -> Style {
    Style::default().fg(OFFLINE)
}

pub fn error_style() -> Style {
    Style::default().fg(ERROR).add_modifier(Modifier::BOLD)
}

pub fn sent_style() -> Style {
    Style::default().fg(SENT)
}

pub fn highlight_style() -> Style {
    Style::default()
        .fg(PRIMARY)
        .bg(HIGHLIGHT_BG)
        .add_modifier(Modifier::BOLD)
}

// ═══════════════════════════════════════════════════════════
// Block Styles
// ═══════════════════════════════════════════════════════════

/// Standard block with rounded corners — used everywhere
pub fn themed_block<'a>(title: &'a str) -> Block<'a> {
    Block::default()
        .title(format!(" {title} "))
        .borders(ratatui::widgets::Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(MUTED))
}

/// Block with primary (cyan) border — for the main header
pub fn header_block<'a>() -> Block<'a> {
    Block::default()
        .borders(ratatui::widgets::Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(primary_bold())
}

// ═══════════════════════════════════════════════════════════
// Logo Rendering
// ═══════════════════════════════════════════════════════════

/// The GhostWire brand mark — a hexagonal mesh node symbol
pub const LOGO_SYMBOL: &str = "\u{2B21}"; // ⬡

/// Compact header logo: "⬡ GHOSTWIRE"
pub fn logo_header() -> Line<'static> {
    Line::from(vec![
        Span::styled(LOGO_SYMBOL, primary_bold()),
        Span::raw(" "),
        Span::styled("GHOSTWIRE", primary_bold()),
    ])
}

/// Full splash logo (for first-run / welcome screen)
///
/// ```text
///       ╱│╲
///       ──┼──   GhostWire v0.1.0
///       ╲│╱
///
///   Infrastructure for Human Dignity
/// ```
pub fn splash_logo_lines() -> Vec<Line<'static>> {
    vec![
        Line::from(vec![
            Span::raw("      "),
            Span::styled("╱│╲", primary_bold()),
        ]),
        Line::from(vec![
            Span::raw("      "),
            Span::styled("──┼──", primary_bold()),
            Span::raw("   "),
            Span::styled("GhostWire v0.1.0", primary_bold()),
        ]),
        Line::from(vec![
            Span::raw("      "),
            Span::styled("╲│╱", primary_bold()),
        ]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "Infrastructure for Human Dignity",
            Style::default().fg(MUTED).add_modifier(Modifier::ITALIC),
        )]),
    ]
}

// ═══════════════════════════════════════════════════════════
// Status Indicators
// ═══════════════════════════════════════════════════════════

/// Online status dot: "🟢" (green circle)
pub fn status_online() -> Span<'static> {
    Span::styled("🟢", online_style())
}

/// Warning/degraded status dot: "🟡" (yellow circle)
pub fn status_warning() -> Span<'static> {
    Span::styled("🟡", warning_style())
}

/// Offline status dot: "⚫" (dark circle)
pub fn status_offline() -> Span<'static> {
    Span::styled("⚫", offline_style())
}

/// Status dot based on a boolean
pub fn status_dot(online: bool) -> Span<'static> {
    if online {
        status_online()
    } else {
        status_offline()
    }
}

// ═══════════════════════════════════════════════════════════
// Trust Score Visualization
// ═══════════════════════════════════════════════════════════

/// Visual trust bar: "████████░░░░" (12 chars)
///
/// `score` is 0.0–1.0
pub fn trust_bar(score: f64) -> String {
    let filled = (score * 12.0).round() as usize;
    let filled = filled.clamp(0, 12);
    let empty = 12 - filled;
    format!("{}{}", "█".repeat(filled), "░".repeat(empty))
}

/// Styled trust bar line with label
pub fn trust_bar_line(score: f64, label: &str) -> Line<'_> {
    let bar = trust_bar(score);
    let bar_color = if score >= 0.7 {
        ONLINE
    } else if score >= 0.4 {
        WARNING
    } else {
        ERROR
    };
    let trust_label = if score >= 0.7 {
        "High"
    } else if score >= 0.4 {
        "Medium"
    } else {
        "Low"
    };

    let label_owned = label.to_string();
    Line::from(vec![
        Span::raw("   "),
        Span::styled(bar, Style::default().fg(bar_color)),
        Span::raw("  "),
        Span::styled(trust_label, Style::default().fg(bar_color)),
        Span::raw(" "),
        Span::styled(label_owned, muted_style()),
    ])
}

// ═══════════════════════════════════════════════════════════
// Help Bar
// ═══════════════════════════════════════════════════════════

/// Context-sensitive help bar at the bottom of each screen
pub fn help_bar<'a>(hints: &[(&'a str, &'a str)]) -> Paragraph<'a> {
    let spans: Vec<Span> = hints
        .iter()
        .enumerate()
        .flat_map(|(i, (key, action))| {
            let mut parts = vec![
                Span::styled(*key, primary_bold()),
                Span::styled(*action, muted_style()),
            ];
            if i < hints.len() - 1 {
                parts.push(Span::styled("  ", muted_style()));
            }
            parts
        })
        .collect();

    Paragraph::new(Line::from(spans))
        .alignment(Alignment::Center)
        .block(themed_block(""))
}
