//! UI rendering for GhostWire TUI

use crate::tui::{AppState, InputMode, SplitDirection, TuiApp, TuiLayoutTree, TuiWindow};
use ratatui::prelude::Alignment;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame,
};

/// Render the main UI with comprehensive features
pub fn render(frame: &mut Frame, app: &mut TuiApp) {
    let size = frame.size();

    // Check if terminal is too small
    if size.height < 15 || size.width < 60 {
        let error_text = "Terminal too small. Please resize to at least 60x15.";
        let error_widget = Paragraph::new(error_text)
            .style(Style::default().fg(Color::Red).bold())
            .alignment(Alignment::Center);
        frame.render_widget(error_widget, size);
        return;
    }

    // Unified layout shell (Irssi-style ultra-compact)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Window Bar: {1:STATUS} {2:#ghostwire}
            Constraint::Length(1), // Title Bar: --- STATUS ---
            Constraint::Min(0),    // Main Content Area
            Constraint::Length(1), // Status Bar (Compact)
            Constraint::Length(1), // Input Line (Prioritized)
        ])
        .split(size);

    render_window_bar(frame, chunks[0], app);
    render_title_bar(frame, chunks[1], app);

    match app.app_state {
        AppState::MainMenu => render_menu_items(frame, chunks[2], app),
        AppState::Dashboard => render_dashboard_content(frame, chunks[2], app),
        AppState::Chat | AppState::Network | AppState::Config | AppState::Logs => {
            render_main_content(frame, chunks[2], app)
        }
        AppState::About => render_about_content(frame, chunks[2], app),
        AppState::Help => render_help_content(frame, chunks[2], app),
        AppState::Security => render_security_content(frame, chunks[2], app),
        AppState::Advanced => render_advanced_content(frame, chunks[2], app),
        AppState::Diagnostics => render_diagnostics_content(frame, chunks[2], app),
        AppState::Settings => render_settings_content(frame, chunks[2], app),
    }

    render_status_bar_compact(frame, chunks[3], app);
    render_input_bar_compact(frame, chunks[4], app);
    render_notifications(frame, app);
}

/// Render command buffer
#[allow(dead_code)]
fn render_command_buffer(frame: &mut Frame, app: &TuiApp) {
    if app.input_mode == InputMode::Command && !app.command_buffer.is_empty() {
        let area = Rect::new(0, frame.size().height - 2, frame.size().width, 1);

        let command_widget = Paragraph::new(format!(":{}", app.command_buffer))
            .style(Style::default().fg(Color::Rgb(255, 165, 0)).bold());

        frame.render_widget(command_widget, area);
    }
}

/// Render search buffer
#[allow(dead_code)]
fn render_search_buffer(frame: &mut Frame, app: &TuiApp) {
    if app.input_mode == InputMode::Search && !app.search_buffer.is_empty() {
        let area = Rect::new(0, frame.size().height - 2, frame.size().width, 1);

        let search_widget = Paragraph::new(format!("/{}", app.search_buffer))
            .style(Style::default().fg(Color::Rgb(0, 255, 255)).bold());

        frame.render_widget(search_widget, area);
    }
}

/// Render notifications
fn render_notifications(frame: &mut Frame, app: &mut TuiApp) {
    if let Some((message, timestamp)) = &app.notification {
        let elapsed = timestamp.elapsed();
        if elapsed.as_secs() < 3 {
            let area = Rect::new(2, 2, frame.size().width - 4, 3);

            let notification_widget = Paragraph::new(message.clone())
                .block(
                    Block::default()
                        .title("NOTIFICATION")
                        .borders(Borders::ALL)
                        .border_style(Style::default().fg(Color::Rgb(255, 255, 0))),
                )
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Center);

            frame.render_widget(notification_widget, area);
        } else {
            app.notification = None;
        }
    }
}

/// Render compact status bar (Irssi-style)
fn render_status_bar_compact(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let stats = vec![
        Span::styled(
            " GHOSTWIRE ",
            Style::default()
                .fg(Color::Black)
                .bg(Color::Rgb(0, 255, 255))
                .bold(),
        ),
        Span::raw(" | "),
        Span::styled(
            format!("CPU:{}%", app.system_stats.cpu_usage.round()),
            Style::default().fg(Color::Rgb(200, 200, 200)),
        ),
        Span::raw(" "),
        Span::styled(
            format!("MEM:{}%", app.system_stats.memory_usage.round()),
            Style::default().fg(Color::Rgb(200, 200, 200)),
        ),
        Span::raw(" | "),
        Span::styled(
            if app.network_state.is_connected {
                "NET:ON"
            } else {
                "NET:OFF"
            },
            Style::default().fg(if app.network_state.is_connected {
                Color::Green
            } else {
                Color::Red
            }),
        ),
        Span::raw(" | "),
        Span::styled(
            format!("P:{} ", app.network_state.peer_count),
            Style::default().fg(Color::Rgb(255, 255, 0)),
        ),
        Span::styled(
            format!(
                "ID:{} ",
                if app.identity_id.len() >= 8 {
                    &app.identity_id[..8]
                } else {
                    &app.identity_id
                }
            ),
            Style::default().fg(Color::Rgb(150, 150, 150)),
        ),
        Span::raw("| "),
        Span::styled(
            format!("SEC:{:.0}% ", app.system_stats.security_score),
            Style::default().fg(Color::Rgb(0, 255, 0)),
        ),
    ];

    let p = Paragraph::new(Line::from(stats)).style(Style::default().bg(Color::Rgb(20, 20, 20)));

    frame.render_widget(p, area);
}

/// Render compact input bar (Irssi-style)
fn render_input_bar_compact(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let current_channel = match app.windows.get(app.active_window_index) {
        Some(TuiWindow::Channel(c)) => format!("#{}", c),
        Some(TuiWindow::Query(q)) => format!("@{}", q),
        Some(TuiWindow::Status) => "system".to_string(),
        Some(TuiWindow::Network) => "net".to_string(),
        Some(TuiWindow::Logs) => "logs".to_string(),
        Some(TuiWindow::Config) => "config".to_string(),
        _ => "main".to_string(),
    };

    let prompt = Span::styled(
        format!("[{}] {} > ", app.chat_state.nickname, current_channel),
        Style::default().fg(Color::Rgb(255, 0, 255)).bold(),
    );

    let input_text = Span::raw(&app.chat_state.input_buffer);

    let line = Line::from(vec![prompt, input_text]);
    let widget = Paragraph::new(line);

    frame.render_widget(widget, area);
}

/// Render the header with ASCII art and title
/// Render the window navigation bar (Top line)
fn render_window_bar(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let window_spans: Vec<Span> = app
        .windows
        .iter()
        .enumerate()
        .map(|(i, window)| {
            let active = i == app.active_window_index;
            let name = match window {
                TuiWindow::Status => "STATUS",
                TuiWindow::Channel(c) => c,
                TuiWindow::Query(q) => q,
                TuiWindow::Network => "NETWORK",
                TuiWindow::Logs => "LOGS",
                TuiWindow::Config => "CONFIG",
            };

            let text = format!(" {}:{} ", i + 1, name);
            if active {
                Span::styled(
                    text,
                    Style::default()
                        .fg(Color::Black)
                        .bg(Color::Rgb(0, 255, 255)) // Cyan highlight
                        .add_modifier(Modifier::BOLD),
                )
            } else {
                Span::styled(text, Style::default().fg(Color::Rgb(120, 120, 120)))
            }
        })
        .collect();

    // Add separator
    let mut final_spans = Vec::new();
    for span in window_spans {
        final_spans.push(span);
        final_spans.push(Span::raw(" | "));
    }

    frame.render_widget(Paragraph::new(Line::from(final_spans)), area);
}

/// Render a condensed, elegant header
fn render_title_bar(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let title = match app.windows.get(app.active_window_index) {
        Some(TuiWindow::Status) => "DASHBOARD",
        Some(TuiWindow::Channel(c)) => c,
        Some(TuiWindow::Query(q)) => q,
        Some(TuiWindow::Network) => "NETWORK TOPOLOGY",
        Some(TuiWindow::Logs) => "SYSTEM LOGS",
        Some(TuiWindow::Config) => "CONFIGURATION",
        _ => "GHOSTWIRE",
    };

    let title_line = format!("--- {} ---", title.to_uppercase());
    let widget = Paragraph::new(title_line)
        .style(
            Style::default()
                .fg(Color::Rgb(0, 150, 255))
                .add_modifier(Modifier::BOLD),
        )
        .alignment(Alignment::Center);

    frame.render_widget(widget, area);
}

/// Render the main content area (recursive layout tree)
fn render_main_content(frame: &mut Frame, area: Rect, app: &TuiApp) {
    render_layout_tree(frame, area, app, &app.layout_tree);
}

/// Recursively render the layout tree
fn render_layout_tree(frame: &mut Frame, area: Rect, app: &TuiApp, node: &TuiLayoutTree) {
    match node {
        TuiLayoutTree::Window(index) => {
            if let Some(window) = app.windows.get(*index) {
                match window {
                    TuiWindow::Status => render_dashboard_content(frame, area, app),
                    TuiWindow::Channel(_) | TuiWindow::Query(_) => {
                        render_chat_tab_for_window(frame, area, app, *index)
                    }
                    TuiWindow::Network => render_network_tab(frame, area, app),
                    TuiWindow::Config => render_config_tab(frame, area, app),
                    TuiWindow::Logs => render_logs_tab(frame, area, app),
                }
            }
        }
        TuiLayoutTree::Split {
            direction,
            ratios,
            children,
        } => {
            let ratatui_direction = match direction {
                SplitDirection::Horizontal => Direction::Horizontal,
                SplitDirection::Vertical => Direction::Vertical,
            };

            let constraints: Vec<Constraint> =
                ratios.iter().map(|&r| Constraint::Percentage(r)).collect();

            let chunks = Layout::default()
                .direction(ratatui_direction)
                .constraints(constraints)
                .split(area);

            for (i, child) in children.iter().enumerate() {
                if let Some(chunk) = chunks.get(i) {
                    render_layout_tree(frame, *chunk, app, child);
                }
            }
        }
    }
}

/// Render IRC-style chat interface for a specific window index
fn render_chat_tab_for_window(frame: &mut Frame, area: Rect, app: &TuiApp, window_index: usize) {
    let window = app.windows.get(window_index);
    let target = match window {
        Some(TuiWindow::Channel(c)) => c.as_str(),
        Some(TuiWindow::Query(q)) => q.as_str(),
        _ => return,
    };

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(25), // Side bar (Channel list)
            Constraint::Min(40),    // Main chat area
            Constraint::Length(20), // User list (Irssi-style right side)
        ])
        .split(area);

    // Left: Channel/Window list
    render_window_list_sidebar(frame, chunks[0], app);

    // Center: Main chat area
    render_chat_area_with_target(frame, chunks[1], app, target);

    // Right: User list for this target
    render_user_list_with_target(frame, chunks[2], app, target);
}

fn render_window_list_sidebar(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let block = Block::default()
        .title("🗔 WINDOWS")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(0, 255, 255)));

    let mut items = Vec::new();
    for (i, window) in app.windows.iter().enumerate() {
        let active = i == app.active_window_index;
        let prefix = if active { ">" } else { " " };
        let name = match window {
            TuiWindow::Status => "Status".to_string(),
            TuiWindow::Channel(c) => format!("#{}", c),
            TuiWindow::Query(q) => format!("@{}", q),
            TuiWindow::Network => "Network".to_string(),
            TuiWindow::Logs => "Logs".to_string(),
            TuiWindow::Config => "Config".to_string(),
        };

        let style = if active {
            Style::default().fg(Color::Rgb(255, 255, 0)).bold()
        } else {
            Style::default().fg(Color::White)
        };

        items.push(Line::from(Span::styled(
            format!("{} {}:{}", prefix, i + 1, name),
            style,
        )));
    }

    let widget = Paragraph::new(items).block(block);
    frame.render_widget(widget, area);
}

fn render_chat_area_with_target(frame: &mut Frame, area: Rect, app: &TuiApp, target: &str) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Messages
        ])
        .split(area);

    // Window Header
    let header_widget = Paragraph::new(format!("💬 target: {}", target))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(0, 255, 255))),
        )
        .style(Style::default().fg(Color::Rgb(255, 255, 0)).bold());
    frame.render_widget(header_widget, chunks[0]);

    // Messages
    let messages = app
        .chat_state
        .messages
        .iter()
        .filter(|m| {
            m.channel.as_deref() == Some(target)
                || (target.starts_with('@') && m.display_name == target[1..])
        })
        .collect::<Vec<_>>();

    let mut message_lines = Vec::new();
    for msg in messages.iter().rev().take(area.height as usize) {
        let formatted = crate::tui::chat::format_message(msg);
        let style = if msg.is_own {
            Style::default().fg(Color::Rgb(255, 255, 0))
        } else {
            Style::default().fg(Color::White)
        };
        message_lines.push(Line::from(Span::styled(formatted, style)));
    }
    message_lines.reverse();

    let messages_widget = Paragraph::new(message_lines);
    frame.render_widget(messages_widget, chunks[1]);
}

fn render_user_list_with_target(frame: &mut Frame, area: Rect, app: &TuiApp, target: &str) {
    let block = Block::default()
        .title("👥 USERS")
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Rgb(0, 255, 255)));

    // Placeholder users for now
    let users = app.chat_state.get_channel_users(target);
    let mut user_items = Vec::new();
    for user in users {
        user_items.push(Line::from(Span::raw(format!(" {}", user.nickname))));
    }

    let widget = Paragraph::new(user_items).block(block);
    frame.render_widget(widget, area);
}

/// Render network tab with comprehensive network information
fn render_network_tab(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Length(8), // Network Status
            Constraint::Length(8), // Peer List
            Constraint::Length(6), // Actions
            Constraint::Min(0),    // Scan Results
        ])
        .split(area);

    // Network header
    let header = Paragraph::new("🌐 NETWORK MONITORING")
        .style(Style::default().fg(Color::Rgb(0, 255, 255)).bold())
        .alignment(Alignment::Center);
    frame.render_widget(header, chunks[0]);

    // Network Status
    let status_info = vec![
        format!(
            "📡 Connection: {}",
            if app.network_state.is_connected {
                "🟢 CONNECTED"
            } else {
                "🔴 DISCONNECTED"
            }
        ),
        format!("👥 Active Peers: {}", app.network_state.peer_count),
        format!("📨 Total Messages: {}", app.network_state.message_count),
        format!("⏱️  Network Uptime: {}s", app.network_state.uptime_seconds),
        format!(
            "🌐 Mesh Status: {}",
            if app.network_state.is_connected {
                "🟢 ACTIVE"
            } else {
                "🔴 INACTIVE"
            }
        ),
        format!(
            "📡 Transport: {}",
            if app.network_state.is_connected {
                "TCP/Stealth"
            } else {
                "None"
            }
        ),
        format!("🔐 Encryption: AES-256-GCM + Ed25519"),
        format!("🛡️  Security: Multi-layer protection"),
    ];

    let status_block = Paragraph::new(status_info.join("\n"))
        .block(
            Block::default()
                .title("📊 NETWORK STATUS")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(0, 255, 255))),
        )
        .style(Style::default().fg(Color::White));
    frame.render_widget(status_block, chunks[1]);

    // Peer List
    let peer_info = if app.network_state.peers.is_empty() {
        vec![
            "👥 NO PEERS CONNECTED".to_string(),
            "".to_string(),
            "Press 's' to scan for peers".to_string(),
            "Press 'c' to connect to network".to_string(),
        ]
    } else {
        app.network_state
            .peers
            .iter()
            .map(|peer| format!("✅ {} - Online - Last seen: Now", peer.name))
            .collect()
    };

    let peer_block = Paragraph::new(peer_info.join("\n"))
        .block(
            Block::default()
                .title("👥 CONNECTED PEERS")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(0, 255, 0))),
        )
        .style(Style::default().fg(Color::White));
    frame.render_widget(peer_block, chunks[2]);

    // Network Actions
    let actions = vec![
        "[S] 🔍 Scan Network".to_string(),
        "[C] 🔌 Connect/Disconnect".to_string(),
        "[R] 🔄 Refresh Peers".to_string(),
        "[M] 🌐 Mesh Status".to_string(),
        "[T] 📡 Transport Info".to_string(),
        "[L] 📋 Network Logs".to_string(),
    ];

    let actions_block = Paragraph::new(actions.join("\n"))
        .block(
            Block::default()
                .title("⚡ NETWORK ACTIONS")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(255, 255, 0))),
        )
        .style(Style::default().fg(Color::White));
    frame.render_widget(actions_block, chunks[3]);

    // Scan Results
    let scan_content = if app.scan_results.is_empty() {
        vec![
            "🔍 NETWORK SCAN RESULTS".to_string(),
            "".to_string(),
            "No recent scan performed".to_string(),
            "Press 'S' to scan for nearby devices".to_string(),
        ]
    } else {
        let mut content = vec!["🔍 RECENT SCAN RESULTS:".to_string(), "".to_string()];
        content.extend(app.scan_results.iter().take(5).cloned());
        if app.scan_results.len() > 5 {
            content.push(format!(
                "... and {} more results",
                app.scan_results.len() - 5
            ));
        }
        content
    };

    let scan_block = Paragraph::new(scan_content.join("\n"))
        .block(
            Block::default()
                .title("🔍 SCAN RESULTS")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(255, 0, 255))),
        )
        .style(Style::default().fg(Color::White));
    frame.render_widget(scan_block, chunks[4]);
}

/// Render the config tab with retro styling
fn render_config_tab(frame: &mut Frame, area: Rect, _app: &TuiApp) {
    let config_info = "⚙️  CONFIGURATION\n\
                      \n\
                      🔧 This tab will show configuration options.\n\
                      \n\
                      🎮 CONTROLS:\n\
                      S - Save config\n\
                      R - Reload config\n\
                      E - Edit settings\n\
                      \n\
                      📁 Current config: default.toml";

    let config_widget = Paragraph::new(config_info)
        .block(
            Block::default()
                .title("⚙️  CONFIGURATION")
                .title_style(
                    Style::default()
                        .fg(Color::Rgb(255, 255, 0))
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(0, 255, 255))),
        )
        .style(Style::default().fg(Color::Rgb(255, 255, 255)))
        .wrap(Wrap { trim: true });

    frame.render_widget(config_widget, area);
}

/// Render the logs tab with retro styling
fn render_logs_tab(frame: &mut Frame, area: Rect, _app: &TuiApp) {
    let logs_info = "📋 SYSTEM LOGS\n\
                    \n\
                    📝 This tab will show system logs.\n\
                    \n\
                    🎮 CONTROLS:\n\
                    F - Filter logs\n\
                    C - Clear logs\n\
                    S - Save logs\n\
                    \n\
                    🔍 Log level: INFO";

    let logs_widget = Paragraph::new(logs_info)
        .block(
            Block::default()
                .title("📋 LOGS")
                .title_style(
                    Style::default()
                        .fg(Color::Rgb(255, 255, 0))
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(0, 255, 255))),
        )
        .style(Style::default().fg(Color::Rgb(255, 255, 255)))
        .wrap(Wrap { trim: true });

    frame.render_widget(logs_widget, area);
}

/// Render menu items
fn render_menu_items(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let menu_items = vec![
        "🚀 DASHBOARD - System Overview & Quick Actions",
        "💬 CHAT - Secure Messaging & Communication",
        "🌐 NETWORK - Mesh Network Management",
        "⚙️  CONFIG - Settings & Configuration",
        "📋 LOGS - System Logs & Monitoring",
        "ℹ️  ABOUT - Project Information",
        "❓ HELP - Usage Guide & Commands",
    ];

    let menu_items: Vec<ListItem> = menu_items
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let style = if i == app.menu_selection {
                Style::default()
                    .fg(Color::Rgb(255, 255, 0)) // Bright yellow for selected
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Rgb(0, 100, 0)) // Dark green background
            } else {
                Style::default().fg(Color::Rgb(255, 255, 255)) // White for others
            };
            ListItem::new(*item).style(style)
        })
        .collect();

    let menu_widget = List::new(menu_items).block(
        Block::default()
            .title("🎮 MAIN MENU - Select an option")
            .title_style(
                Style::default()
                    .fg(Color::Rgb(255, 255, 0))
                    .add_modifier(Modifier::BOLD),
            )
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(0, 255, 255))),
    );

    frame.render_widget(menu_widget, area);
}

/// Render the main dashboard with comprehensive backend features
fn render_dashboard_content(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(60), // Status Info
            Constraint::Percentage(40), // Quick Actions & Alerts
        ])
        .split(area);

    // Left column - Network & Security
    render_dashboard_left_column(frame, chunks[0], app);

    // Right column - Actions & Status
    render_dashboard_right_column(frame, chunks[1], app);
}

/// Render left column of dashboard
fn render_dashboard_left_column(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8), // Network Status
            Constraint::Length(8), // Security Status
            Constraint::Min(0),    // Scan Results
        ])
        .split(area);

    // Network Status (High density text)
    let network_status = vec![
        Line::from(vec![
            Span::styled("NETWORK: ", Style::default().fg(Color::Rgb(150, 150, 150))),
            if app.network_state.is_connected {
                Span::styled("CONNECTED", Style::default().fg(Color::Green).bold())
            } else {
                Span::styled("DISCONNECTED", Style::default().fg(Color::Red).bold())
            },
        ]),
        Line::from(vec![
            Span::styled("PEERS:   ", Style::default().fg(Color::Rgb(150, 150, 150))),
            Span::styled(
                app.network_state.peer_count.to_string(),
                Style::default().fg(Color::Cyan),
            ),
        ]),
        Line::from(vec![
            Span::styled("IDENT:   ", Style::default().fg(Color::Rgb(150, 150, 150))),
            Span::styled(
                format!("GW-{}", &app.identity_id[..8]),
                Style::default().fg(Color::Rgb(255, 255, 0)),
            ),
        ]),
        Line::from(vec![
            Span::styled("UPTIME:  ", Style::default().fg(Color::Rgb(150, 150, 150))),
            Span::styled(
                format!("{}s", app.network_state.uptime_seconds),
                Style::default().fg(Color::White),
            ),
        ]),
    ];

    let network_p = Paragraph::new(network_status).block(
        Block::default()
            .title(" [ Status ] ")
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::Rgb(80, 80, 80))),
    );
    frame.render_widget(network_p, chunks[0]);

    // Security Status (Clean text)
    let security_content = vec![
        Line::from(vec![
            Span::styled("SECURITY: ", Style::default().fg(Color::Rgb(150, 150, 150))),
            Span::styled("ACTIVE", Style::default().fg(Color::Green).bold()),
        ]),
        Line::from(vec![
            Span::styled("ENCRYPT:  ", Style::default().fg(Color::Rgb(150, 150, 150))),
            Span::styled(
                "AES-256-GCM",
                Style::default().fg(Color::Rgb(200, 200, 200)),
            ),
        ]),
        Line::from(vec![
            Span::styled("ALERTS:   ", Style::default().fg(Color::Rgb(150, 150, 150))),
            Span::styled(
                app.security_alerts.len().to_string(),
                Style::default().fg(if app.security_alerts.is_empty() {
                    Color::Green
                } else {
                    Color::Red
                }),
            ),
        ]),
    ];

    let security_p = Paragraph::new(security_content).block(
        Block::default()
            .title(" [ Security ] ")
            .borders(Borders::TOP)
            .border_style(Style::default().fg(Color::Rgb(80, 80, 80))),
    );
    frame.render_widget(security_p, chunks[1]);

    // Scan Results
    let scan_content = if app.scan_results.is_empty() {
        vec!["🔍 No recent network scan".to_string()]
    } else {
        app.scan_results.clone()
    };

    // Scan Results
    let scan_block = Paragraph::new(scan_content.join("\n"))
        .block(
            Block::default()
                .title(" [ Scan Results ] ")
                .borders(Borders::TOP)
                .border_style(Style::default().fg(Color::Rgb(80, 80, 80))),
        )
        .style(Style::default().fg(Color::Rgb(150, 150, 150)));
    frame.render_widget(scan_block, chunks[2]);
}

/// Render right column of dashboard
fn render_dashboard_right_column(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12), // Quick Actions
            Constraint::Length(8),  // Security Alerts
            Constraint::Min(0),     // System Info
        ])
        .split(area);

    // Quick Actions
    let actions = vec![
        "[1] 🔍 Scan Network".to_string(),
        "[2] 🔌 Connect/Disconnect".to_string(),
        "[3] 👥 Refresh Peers".to_string(),
        "[4] 🔑 Export Keys".to_string(),
        "[5] 🔒 Security Settings".to_string(),
        "[6] 📋 View Logs".to_string(),
        "[7] 🌐 Mesh Status".to_string(),
        "[8] 📡 Reticulum Status".to_string(),
        "[9] 🔄 Update Settings".to_string(),
        "[0] 🚨 Emergency Mode".to_string(),
    ];

    let actions_block = Paragraph::new(actions.join("\n"))
        .block(
            Block::default()
                .title(" [ Quick Actions ] ")
                .borders(Borders::TOP)
                .border_style(Style::default().fg(Color::Rgb(80, 80, 80))),
        )
        .style(Style::default().fg(Color::Rgb(200, 200, 200)));
    frame.render_widget(actions_block, chunks[0]);

    // Security Alerts
    let alerts_content = if app.security_alerts.is_empty() {
        vec!["✅ No security alerts".to_string()]
    } else {
        app.security_alerts.iter().take(5).cloned().collect()
    };

    let alerts_block = Paragraph::new(alerts_content.join("\n"))
        .block(
            Block::default()
                .title("🚨 SECURITY ALERTS")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(255, 0, 0))),
        )
        .style(Style::default().fg(Color::White));
    frame.render_widget(alerts_block, chunks[1]);

    // System Information
    let system_info = vec![
        "🖥️  Platform: Linux x86_64".to_string(),
        "🔧 Rust: 1.70+".to_string(),
        "🌐 libp2p: Latest".to_string(),
        "🔐 Crypto: ring, ed25519-dalek".to_string(),
        "📡 Transports: TCP, Stealth, Mesh".to_string(),
        "🛡️  Security: Multi-layer".to_string(),
    ];

    let system_block = Paragraph::new(system_info.join("\n"))
        .block(
            Block::default()
                .title("💻 SYSTEM INFO")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(0, 255, 255))),
        )
        .style(Style::default().fg(Color::White));
    frame.render_widget(system_block, chunks[2]);
}

/// Render about screen
fn render_about_content(frame: &mut Frame, area: Rect, _app: &TuiApp) {
    let about_text = format!(
        "🌐 GHOSTWIRE - SECURE MESH NETWORKING\n\
         \n\
         🔒 A decentralized, encrypted mesh networking platform\n\
         built with Rust for maximum security and performance.\n\
         \n\
         🚀 FEATURES\n\
         • End-to-end encryption (AES-256-GCM)\n\
         • Decentralized mesh networking\n\
         • Anonymous communication\n\
         • Threat detection & mitigation\n\
         • Cross-platform compatibility\n\
         \n\
         🛠️  TECHNOLOGY\n\
         • Backend: Rust, Tokio, libp2p\n\
         • Frontend: React, TypeScript\n\
         • Encryption: Ring, ChaCha20-Poly1305\n\
         • Networking: P2P, Kademlia, Gossipsub\n\
         \n\
         📄 LICENSE\n\
         MIT License - Open Source\n\
         \n\
         🔗 GITHUB\n\
         https://github.com/ghostwire-project\n\
         \n\
         Press Q to return to menu"
    );

    let about_widget = Paragraph::new(about_text)
        .block(
            Block::default()
                .title("ℹ️  ABOUT GHOSTWIRE")
                .title_style(
                    Style::default()
                        .fg(Color::Rgb(255, 255, 0))
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(0, 255, 255))),
        )
        .style(Style::default().fg(Color::Rgb(255, 255, 255)))
        .wrap(Wrap { trim: true });

    frame.render_widget(about_widget, area);
}

/// Render help screen
fn render_help_content(frame: &mut Frame, area: Rect, _app: &TuiApp) {
    let help_text = format!(
        "🎮 GHOSTWIRE TUI - CONTROLS & NAVIGATION\n\
         \n\
         🏠 MAIN MENU\n\
         ↑↓ / jk - Navigate menu items\n\
         Enter - Select highlighted option\n\
         Q / ESC - Quit application\n\
         \n\
         📊 DASHBOARD\n\
         1-4 - Quick jump to sections\n\
         H - Show this help\n\
         Q - Return to main menu\n\
         \n\
         💬 CHAT MODE\n\
         Type message and press Enter\n\
         Tab - Switch between tabs\n\
         1-4 - Direct tab access\n\
         Q - Return to menu\n\
         \n\
         🌐 NETWORK MODE\n\
         R - Refresh network status\n\
         C - Connect/disconnect\n\
         S - Scan for peers\n\
         \n\
         ⚙️  CONFIG MODE\n\
         S - Save configuration\n\
         R - Reload configuration\n\
         E - Edit settings\n\
         \n\
         📋 LOGS MODE\n\
         F - Filter logs\n\
         C - Clear logs\n\
         S - Save logs\n\
         \n\
         🔧 GENERAL\n\
         ESC - Go back/return to menu\n\
         H - Show help (this screen)\n\
         D - Go to dashboard\n\
         Q - Quit application\n\
         \n\
         Press Q to return to menu"
    );

    let help_widget = Paragraph::new(help_text)
        .block(
            Block::default()
                .title("❓ HELP & CONTROLS")
                .title_style(
                    Style::default()
                        .fg(Color::Rgb(255, 255, 0))
                        .add_modifier(Modifier::BOLD),
                )
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(0, 255, 255))),
        )
        .style(Style::default().fg(Color::Rgb(255, 255, 255)))
        .wrap(Wrap { trim: true });

    frame.render_widget(help_widget, area);
}

/// Render security interface
fn render_security_content(frame: &mut Frame, area: Rect, app: &TuiApp) {
    render_security_subcontent(frame, area, app);
}

/// Helper for security (Refactored)
fn render_security_subcontent(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Content
        ])
        .split(area);

    // Security header
    let header = Paragraph::new("🛡️ SECURITY CENTER")
        .style(Style::default().fg(Color::Rgb(255, 0, 0)).bold())
        .alignment(Alignment::Center);
    frame.render_widget(header, chunks[0]);

    // Security content
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Left column
            Constraint::Percentage(50), // Right column
        ])
        .split(chunks[1]);

    // Left column - Security Status
    let security_status = vec![
        format!("🔐 Encryption: AES-256-GCM + Ed25519"),
        format!(
            "🛡️ Threat Detection: {}",
            if app.advanced_features.threat_detection {
                "ACTIVE"
            } else {
                "INACTIVE"
            }
        ),
        format!(
            "🔑 Key Rotation: {}",
            if app.advanced_features.encryption_rotation {
                "ENABLED"
            } else {
                "DISABLED"
            }
        ),
        format!(
            "👥 Peer Verification: {}",
            if app.advanced_features.peer_verification {
                "ENABLED"
            } else {
                "DISABLED"
            }
        ),
        format!("🚨 Security Score: {:.0}%", app.system_stats.security_score),
        format!("🚨 Alerts: {}", app.security_alerts.len()),
        format!(
            "🔒 Stealth Mode: {}",
            if app.advanced_features.stealth_mode {
                "ACTIVE"
            } else {
                "INACTIVE"
            }
        ),
        format!(
            "🚨 Panic Mode: {}",
            if app.advanced_features.panic_mode {
                "ACTIVE"
            } else {
                "INACTIVE"
            }
        ),
    ];

    let security_block = Paragraph::new(security_status.join("\n"))
        .block(
            Block::default()
                .title("🛡️ SECURITY STATUS")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(255, 0, 0))),
        )
        .style(Style::default().fg(Color::White));
    frame.render_widget(security_block, content_chunks[0]);

    // Right column - Security Actions
    let security_actions = vec![
        "[A] 🔍 Security Audit".to_string(),
        "[T] 🛡️ Toggle Threat Detection".to_string(),
        "[K] 🔑 Rotate Keys".to_string(),
        "[S] 🔒 Toggle Stealth Mode".to_string(),
        "[P] 🚨 Activate Panic Mode".to_string(),
        "[E] 🚨 Emergency Mode".to_string(),
        "[V] 👥 Verify All Peers".to_string(),
        "[B] 💾 Backup Security Keys".to_string(),
        "[R] 🔄 Restore Security Keys".to_string(),
        "[L] 📋 Security Logs".to_string(),
    ];

    let actions_block = Paragraph::new(security_actions.join("\n"))
        .block(
            Block::default()
                .title("⚡ SECURITY ACTIONS")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(255, 0, 0))),
        )
        .style(Style::default().fg(Color::White));
    frame.render_widget(actions_block, content_chunks[1]);
}

/// Render advanced features interface
fn render_advanced_content(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Content
        ])
        .split(area);

    // Advanced header
    let header = Paragraph::new("⚡ ADVANCED FEATURES")
        .style(Style::default().fg(Color::Rgb(255, 165, 0)).bold())
        .alignment(Alignment::Center);
    frame.render_widget(header, chunks[0]);

    // Advanced content
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Left column
            Constraint::Percentage(50), // Right column
        ])
        .split(chunks[1]);

    // Left column - Feature Status
    let feature_status = vec![
        format!(
            "🔒 Stealth Mode: {}",
            if app.advanced_features.stealth_mode {
                "ACTIVE"
            } else {
                "INACTIVE"
            }
        ),
        format!(
            "🚨 Panic Mode: {}",
            if app.advanced_features.panic_mode {
                "ACTIVE"
            } else {
                "INACTIVE"
            }
        ),
        format!(
            "🚨 Emergency Mode: {}",
            if app.advanced_features.emergency_mode {
                "ACTIVE"
            } else {
                "INACTIVE"
            }
        ),
        format!(
            "💾 Auto Backup: {}",
            if app.advanced_features.auto_backup {
                "ENABLED"
            } else {
                "DISABLED"
            }
        ),
        format!(
            "🛡️ Threat Detection: {}",
            if app.advanced_features.threat_detection {
                "ACTIVE"
            } else {
                "INACTIVE"
            }
        ),
        format!(
            "📊 Performance Monitoring: {}",
            if app.advanced_features.performance_monitoring {
                "ACTIVE"
            } else {
                "INACTIVE"
            }
        ),
        format!(
            "🌐 Network Optimization: {}",
            if app.advanced_features.network_optimization {
                "ENABLED"
            } else {
                "DISABLED"
            }
        ),
        format!(
            "🔑 Encryption Rotation: {}",
            if app.advanced_features.encryption_rotation {
                "ENABLED"
            } else {
                "DISABLED"
            }
        ),
        format!(
            "👥 Peer Verification: {}",
            if app.advanced_features.peer_verification {
                "ENABLED"
            } else {
                "DISABLED"
            }
        ),
        format!(
            "🌫️ Traffic Obfuscation: {}",
            if app.advanced_features.traffic_obfuscation {
                "ACTIVE"
            } else {
                "INACTIVE"
            }
        ),
    ];

    let features_block = Paragraph::new(feature_status.join("\n"))
        .block(
            Block::default()
                .title("⚡ FEATURE STATUS")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(255, 165, 0))),
        )
        .style(Style::default().fg(Color::White));
    frame.render_widget(features_block, content_chunks[0]);

    // Right column - Advanced Actions
    let advanced_actions = vec![
        "[S] 🔒 Toggle Stealth Mode".to_string(),
        "[P] 🚨 Toggle Panic Mode".to_string(),
        "[E] 🚨 Toggle Emergency Mode".to_string(),
        "[B] 💾 Toggle Auto Backup".to_string(),
        "[T] 🛡️ Toggle Threat Detection".to_string(),
        "[M] 📊 Toggle Performance Monitoring".to_string(),
        "[N] 🌐 Toggle Network Optimization".to_string(),
        "[K] 🔑 Toggle Encryption Rotation".to_string(),
        "[V] 👥 Toggle Peer Verification".to_string(),
        "[O] 🌫️ Toggle Traffic Obfuscation".to_string(),
    ];

    let actions_block = Paragraph::new(advanced_actions.join("\n"))
        .block(
            Block::default()
                .title("🎛️ ADVANCED CONTROLS")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(255, 165, 0))),
        )
        .style(Style::default().fg(Color::White));
    frame.render_widget(actions_block, content_chunks[1]);
}

/// Render diagnostics interface
fn render_diagnostics_content(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Content
        ])
        .split(area);

    // Diagnostics header
    let header = Paragraph::new("🔧 SYSTEM DIAGNOSTICS")
        .style(Style::default().fg(Color::Rgb(0, 255, 255)).bold())
        .alignment(Alignment::Center);
    frame.render_widget(header, chunks[0]);

    // Diagnostics content
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Left column
            Constraint::Percentage(50), // Right column
        ])
        .split(chunks[1]);

    // Left column - System Metrics
    let system_metrics = vec![
        format!("⚡ CPU Usage: {:.1}%", app.system_stats.cpu_usage),
        format!("💾 Memory Usage: {:.1}%", app.system_stats.memory_usage),
        format!(
            "🌐 Network Throughput: {:.1} MB/s",
            app.system_stats.network_throughput
        ),
        format!(
            "🔗 Active Connections: {}",
            app.system_stats.active_connections
        ),
        format!(
            "🔐 Encryption Overhead: {:.1}%",
            app.system_stats.encryption_overhead
        ),
        format!("⏱️ Latency: {:.1}ms", app.system_stats.latency_ms),
        format!("📦 Packet Loss: {:.1}%", app.system_stats.packet_loss),
        format!("⏱️ Uptime: {}s", app.system_stats.uptime_seconds),
        format!("🔐 Security Score: {:.0}%", app.system_stats.security_score),
        format!(
            "💾 Last Backup: {}",
            if let Some(backup) = app.system_stats.last_backup {
                format!("{}s ago", backup.elapsed().as_secs())
            } else {
                "Never".to_string()
            }
        ),
    ];

    let metrics_block = Paragraph::new(system_metrics.join("\n"))
        .block(
            Block::default()
                .title("📊 SYSTEM METRICS")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(0, 255, 255))),
        )
        .style(Style::default().fg(Color::White));
    frame.render_widget(metrics_block, content_chunks[0]);

    // Right column - Diagnostic Actions
    let diagnostic_actions = vec![
        "[S] 🔧 System Diagnostics".to_string(),
        "[N] 🌐 Network Diagnostics".to_string(),
        "[A] 🛡️ Security Audit".to_string(),
        "[P] ⚡ Performance Test".to_string(),
        "[B] 💾 Backup System".to_string(),
        "[R] 🔄 Restore System".to_string(),
        "[U] 🔧 Update Firmware".to_string(),
        "[L] 📋 View Logs".to_string(),
        "[C] 🧹 Clear Cache".to_string(),
        "[T] 🔄 Reset System".to_string(),
    ];

    let actions_block = Paragraph::new(diagnostic_actions.join("\n"))
        .block(
            Block::default()
                .title("🔧 DIAGNOSTIC TOOLS")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(0, 255, 255))),
        )
        .style(Style::default().fg(Color::White));
    frame.render_widget(actions_block, content_chunks[1]);
}

/// Render settings interface
fn render_settings_content(frame: &mut Frame, area: Rect, _app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Content
        ])
        .split(area);

    // Settings header
    let header = Paragraph::new("⚙️ SYSTEM SETTINGS")
        .style(Style::default().fg(Color::Rgb(255, 255, 0)).bold())
        .alignment(Alignment::Center);
    frame.render_widget(header, chunks[0]);

    // Settings content
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Left column
            Constraint::Percentage(50), // Right column
        ])
        .split(chunks[1]);

    // Left column - Current Settings
    let current_settings = vec![
        "🔧 Network Settings:".to_string(),
        "  • Auto-connect: Enabled".to_string(),
        "  • Port: 3000".to_string(),
        "  • Timeout: 30s".to_string(),
        "".to_string(),
        "🔐 Security Settings:".to_string(),
        "  • Encryption: AES-256-GCM".to_string(),
        "  • Key Rotation: 24h".to_string(),
        "  • Threat Detection: Active".to_string(),
        "".to_string(),
        "💾 Storage Settings:".to_string(),
        "  • Auto-backup: Enabled".to_string(),
        "  • Backup Interval: 6h".to_string(),
        "  • Max Log Size: 100MB".to_string(),
    ];

    let settings_block = Paragraph::new(current_settings.join("\n"))
        .block(
            Block::default()
                .title("⚙️ CURRENT SETTINGS")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(255, 255, 0))),
        )
        .style(Style::default().fg(Color::White));
    frame.render_widget(settings_block, content_chunks[0]);

    // Right column - Settings Actions
    let settings_actions = vec![
        "[N] 🌐 Network Settings".to_string(),
        "[S] 🔐 Security Settings".to_string(),
        "[D] 💾 Data Settings".to_string(),
        "[I] 🔧 Interface Settings".to_string(),
        "[L] 📋 Logging Settings".to_string(),
        "[B] 💾 Backup Settings".to_string(),
        "[R] 🔄 Reset to Defaults".to_string(),
        "[E] 📤 Export Settings".to_string(),
        "[I] 📥 Import Settings".to_string(),
        "[V] 🔍 Validate Settings".to_string(),
    ];

    let actions_block = Paragraph::new(settings_actions.join("\n"))
        .block(
            Block::default()
                .title("🎛️ SETTINGS CONTROLS")
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(255, 255, 0))),
        )
        .style(Style::default().fg(Color::White));
    frame.render_widget(actions_block, content_chunks[1]);
}
