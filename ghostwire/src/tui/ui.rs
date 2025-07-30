//! UI rendering for GhostWire TUI

use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Span, Line},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use ratatui::prelude::Alignment;
use crate::tui::{AppState, InputMode, TuiApp, Tab};

/// Render the main UI with comprehensive features
pub fn render(frame: &mut Frame, app: &mut TuiApp) {
    let size = frame.size();
    
    // Check if terminal is too small
    if size.height < 10 || size.width < 40 {
        // Render minimal interface for small terminals
        let error_text = "Terminal too small. Please resize to at least 40x10.";
        let error_widget = Paragraph::new(error_text)
            .style(Style::default().fg(Color::Red).bold())
            .alignment(Alignment::Center);
        frame.render_widget(error_widget, size);
        return;
    }

    match app.app_state {
        AppState::MainMenu => render_main_menu(frame, app),
        AppState::Dashboard => render_dashboard(frame, app),
        AppState::Chat | AppState::Network | AppState::Config | AppState::Logs => render_tab_interface(frame, app),
        AppState::About => render_about(frame, frame.size(), app),
        AppState::Help => render_help(frame, frame.size(), app),
        AppState::Security => render_security(frame, app),
        AppState::Advanced => render_advanced(frame, app),
        AppState::Diagnostics => render_diagnostics(frame, app),
        AppState::Settings => render_settings(frame, app),
    }

    // Always render status bar and input mode
    render_status_bar(frame, app);
    render_input_mode(frame, app);
    render_notifications(frame, app);
}

/// Render input mode indicator
fn render_input_mode(frame: &mut Frame, app: &TuiApp) {
    let height = frame.size().height;
    let width = frame.size().width;
    
    // Ensure we don't overflow
    if height < 1 {
        return; // Terminal too small
    }
    
    let area = Rect::new(0, height - 1, width, 1);
    
    let mode_text = match app.input_mode {
        InputMode::Normal => "NORMAL",
        InputMode::Insert => "INSERT",
        InputMode::Command => "COMMAND",
        InputMode::Search => "SEARCH",
        InputMode::Menu => "MENU",
    };

    let mode_color = match app.input_mode {
        InputMode::Normal => Color::Rgb(0, 255, 0),    // Green
        InputMode::Insert => Color::Rgb(255, 255, 0),  // Yellow
        InputMode::Command => Color::Rgb(255, 165, 0), // Orange
        InputMode::Search => Color::Rgb(0, 255, 255),  // Cyan
        InputMode::Menu => Color::Rgb(255, 0, 255),    // Magenta
    };

    let mode_widget = Paragraph::new(format!("-- {} --", mode_text))
        .style(Style::default().fg(mode_color).bold())
        .alignment(Alignment::Center);

    frame.render_widget(mode_widget, area);
}

/// Render command buffer
fn render_command_buffer(frame: &mut Frame, app: &TuiApp) {
    if app.input_mode == InputMode::Command && !app.command_buffer.is_empty() {
        let area = Rect::new(0, frame.size().height - 2, frame.size().width, 1);
        
        let command_widget = Paragraph::new(format!(":{}", app.command_buffer))
            .style(Style::default().fg(Color::Rgb(255, 165, 0)).bold());

        frame.render_widget(command_widget, area);
    }
}

/// Render search buffer
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
                .block(Block::default()
                    .title("NOTIFICATION")
                    .borders(Borders::ALL)
                    .border_style(Style::default().fg(Color::Rgb(255, 255, 0))))
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Center);

            frame.render_widget(notification_widget, area);
        } else {
            app.notification = None;
        }
    }
}

/// Render enhanced status bar
fn render_status_bar(frame: &mut Frame, app: &TuiApp) {
    let height = frame.size().height;
    let width = frame.size().width;
    
    // Ensure we don't overflow
    if height < 3 {
        return; // Terminal too small
    }
    
    let area = Rect::new(0, height - 3, width, 2);
    
    let status_info = vec![
        format!("⚡ {}%", app.system_stats.cpu_usage as i32),
        format!("💾 {}%", app.system_stats.memory_usage as i32),
        format!("🌐 {}", if app.network_state.is_connected { "CONNECTED" } else { "DISCONNECTED" }),
        format!("👥 {}", app.network_state.peer_count),
        format!("🆔 GW-{}", if app.identity_id.len() >= 8 { &app.identity_id[..8] } else { &app.identity_id }),
        format!("🔐 {:.0}%", app.system_stats.security_score),
        format!("⏱️  {}s", app.system_stats.uptime_seconds),
    ];

    let status_text = status_info.join(" | ");
    
    let status_widget = Paragraph::new(status_text)
        .block(Block::default()
            .title("SYSTEM STATUS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(0, 255, 255))))
        .style(Style::default().fg(Color::White))
        .alignment(Alignment::Center);

    frame.render_widget(status_widget, area);
}

/// Render the header with ASCII art and title
fn render_header(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let ascii_art = vec![
        "██╗    ██╗ ██████╗ ███████╗██╗  ██╗███████╗██╗    ██╗██╗██████╗ ███████╗",
        "██║    ██║██╔════╝ ██╔════╝██║  ██║██╔════╝██║    ██║██║██╔══██╗██╔════╝",
        "██║ █╗ ██║██║  ███╗███████╗███████║█████╗  ██║ █╗ ██║██║██████╔╝█████╗  ",
        "██║███╗██║██║   ██║╚════██║██╔══██║██╔══╝  ██║███╗██║██║██╔══██╗██╔══╝  ",
        "╚███╔███╔╝╚██████╔╝███████║██║  ██║██║     ╚███╔███╔╝██║██║  ██║███████╗",
        " ╚══╝╚══╝  ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝      ╚══╝╚══╝ ╚═╝╚═╝  ╚═╝╚══════╝",
    ];

    // Render ASCII art with cyan color
    for (i, line) in ascii_art.iter().enumerate() {
        if i < area.height as usize {
            let y = area.y + i as u16;
            let text = Span::styled(
                *line,
                Style::default()
                    .fg(Color::Rgb(0, 255, 255)) // Bright cyan
                    .add_modifier(Modifier::BOLD),
            );
            frame.render_widget(
                Paragraph::new(Line::from(text)),
                Rect::new(area.x, y, area.width, 1),
            );
        }
    }

    // Render subtitle
    let subtitle = "🌐 SECURE MESH NETWORKING TERMINAL";
    let subtitle_style = Style::default()
        .fg(Color::Rgb(255, 255, 0)) // Bright yellow
        .add_modifier(Modifier::BOLD);
    
    let subtitle_y = area.y + ascii_art.len() as u16;
    if subtitle_y < area.y + area.height {
        frame.render_widget(
            Paragraph::new(Line::from(Span::styled(subtitle, subtitle_style))),
            Rect::new(area.x, subtitle_y, area.width, 1),
        );
    }

    // Render tab navigation
    let tabs = vec![
        ("[1] CHAT", app.current_tab == Tab::Chat),
        ("[2] NETWORK", app.current_tab == Tab::Network),
        ("[3] CONFIG", app.current_tab == Tab::Config),
        ("[4] LOGS", app.current_tab == Tab::Logs),
    ];

    let tab_text: Vec<Span> = tabs
        .iter()
        .map(|(text, active)| {
            let style = if *active {
                Style::default()
                    .fg(Color::Rgb(255, 255, 0)) // Bright yellow
                    .add_modifier(Modifier::BOLD)
                    .bg(Color::Rgb(0, 100, 0)) // Dark green background
            } else {
                Style::default()
                    .fg(Color::Rgb(128, 128, 128)) // Gray
            };
            Span::styled(*text, style)
        })
        .collect();

    let tab_y = subtitle_y + 1;
    if tab_y < area.y + area.height {
        frame.render_widget(
            Paragraph::new(Line::from(tab_text)),
            Rect::new(area.x, tab_y, area.width, 1),
        );
    }
}

/// Render the main content area
fn render_main_content(frame: &mut Frame, area: Rect, app: &TuiApp) {
    match app.current_tab {
        Tab::Chat => render_chat_tab(frame, area, app),
        Tab::Network => render_network_tab(frame, area, app),
        Tab::Config => render_config_tab(frame, area, app),
        Tab::Logs => render_logs_tab(frame, area, app),
    }
}

/// Render IRC-style chat interface
fn render_chat_tab(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length(20), // Channel list
            Constraint::Min(40),    // Main chat area
            Constraint::Length(15), // User list
        ])
        .split(area);

    // Left: Channel list
    render_channel_list(frame, chunks[0], app);
    
    // Center: Main chat area
    render_chat_area(frame, chunks[1], app);
    
    // Right: User list
    render_user_list(frame, chunks[2], app);
}

fn render_channel_list(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let title = "CHANNELS";
    let title_widget = Paragraph::new(title)
        .block(Block::default()
            .title("📺 CHANNELS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(0, 255, 255))))
        .style(Style::default().fg(Color::Rgb(255, 255, 0)).bold())
        .alignment(Alignment::Center);

    frame.render_widget(title_widget, Rect::new(area.x, area.y, area.width, 1));

    let channel_area = Rect::new(area.x, area.y + 1, area.width, area.height - 1);
    
    let mut channel_items = Vec::new();
    for channel in &app.chat_state.channels {
        let prefix = if channel.is_joined { "●" } else { "○" };
        let current = if channel.name == app.chat_state.current_channel { ">" } else { " " };
        let style = if channel.is_joined {
            Style::default().fg(Color::Rgb(0, 255, 0))
        } else {
            Style::default().fg(Color::Rgb(128, 128, 128))
        };
        
        let text = format!("{} {} {} ({})", current, prefix, channel.name, channel.user_count);
        channel_items.push(Line::from(Span::styled(text, style)));
    }

    let channels_widget = Paragraph::new(channel_items)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White));

    frame.render_widget(channels_widget, channel_area);
}

fn render_chat_area(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Channel header
            Constraint::Min(10),    // Messages
            Constraint::Length(1),  // Separator
        ])
        .split(area);

    // Channel header with topic
    let current_channel = &app.chat_state.current_channel;
    let channel_info = app.chat_state.channels.iter()
        .find(|c| c.name == *current_channel);
    
    let default_topic = "".to_string();
    let topic = channel_info.map(|c| &c.topic).unwrap_or(&default_topic);
    let user_count = channel_info.map(|c| c.user_count).unwrap_or(0);
    
    let header_text = format!("💬 {} - {} ({})", current_channel, topic, user_count);
    let header_widget = Paragraph::new(header_text)
        .block(Block::default()
            .title("CHAT")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(0, 255, 255))))
        .style(Style::default().fg(Color::Rgb(255, 255, 0)).bold())
        .alignment(Alignment::Left);

    frame.render_widget(header_widget, chunks[0]);

    // Messages area
    let messages = app.chat_state.get_current_channel_messages();
    let mut message_lines = Vec::new();
    
    for msg in messages.iter().take(50) { // Show last 50 messages
        let formatted = crate::tui::chat::format_message(msg);
        let style = if msg.is_own {
            Style::default().fg(Color::Rgb(255, 255, 0)) // Yellow for own messages
        } else if msg.message_type == crate::tui::chat::MessageType::System {
            Style::default().fg(Color::Rgb(255, 165, 0)) // Orange for system messages
        } else if msg.message_type == crate::tui::chat::MessageType::Action {
            Style::default().fg(Color::Rgb(255, 192, 203)) // Pink for actions
        } else {
            Style::default().fg(Color::White) // White for regular messages
        };
        
        message_lines.push(Line::from(Span::styled(formatted, style)));
    }

    let messages_widget = Paragraph::new(message_lines)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White))
        .scroll((app.chat_state.messages.len().saturating_sub(20) as u16, 0));

    frame.render_widget(messages_widget, chunks[1]);

    // Input area separator
    let separator = Line::from(Span::styled("─".repeat(area.width as usize), Style::default().fg(Color::Rgb(128, 128, 128))));
    let separator_widget = Paragraph::new(separator);
    frame.render_widget(separator_widget, chunks[2]);
}

fn render_user_list(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let title = "USERS";
    let title_widget = Paragraph::new(title)
        .block(Block::default()
            .title("👥 USERS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(0, 255, 255))))
        .style(Style::default().fg(Color::Rgb(255, 255, 0)).bold())
        .alignment(Alignment::Center);

    frame.render_widget(title_widget, Rect::new(area.x, area.y, area.width, 1));

    let user_area = Rect::new(area.x, area.y + 1, area.width, area.height - 1);
    
    let users = app.chat_state.get_channel_users(&app.chat_state.current_channel);
    let mut user_items = Vec::new();
    
    for user in users {
        let prefix = if user.is_operator { "@" } else if user.is_voice { "+" } else { "" };
        let away_indicator = if user.is_away { " (away)" } else { "" };
        let style = if user.nickname == app.chat_state.nickname {
            Style::default().fg(Color::Rgb(255, 255, 0)).bold() // Yellow for self
        } else if user.is_operator {
            Style::default().fg(Color::Rgb(255, 0, 0)) // Red for operators
        } else if user.is_voice {
            Style::default().fg(Color::Rgb(0, 255, 0)) // Green for voiced users
        } else if user.is_away {
            Style::default().fg(Color::Rgb(128, 128, 128)) // Gray for away users
        } else {
            Style::default().fg(Color::White) // White for regular users
        };
        
        let text = format!("{}{}{}", prefix, user.nickname, away_indicator);
        user_items.push(Line::from(Span::styled(text, style)));
    }

    let users_widget = Paragraph::new(user_items)
        .block(Block::default().borders(Borders::NONE))
        .style(Style::default().fg(Color::White));

    frame.render_widget(users_widget, user_area);
}

/// Render network tab with comprehensive network information
fn render_network_tab(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Length(8),  // Network Status
            Constraint::Length(8),  // Peer List
            Constraint::Length(6),  // Actions
            Constraint::Min(0),     // Scan Results
        ])
        .split(area);

    // Network header
    let header = Paragraph::new("🌐 NETWORK MONITORING")
        .style(Style::default().fg(Color::Rgb(0, 255, 255)).bold())
        .alignment(Alignment::Center);
    frame.render_widget(header, chunks[0]);

    // Network Status
    let status_info = vec![
        format!("📡 Connection: {}", if app.network_state.is_connected { "🟢 CONNECTED" } else { "🔴 DISCONNECTED" }),
        format!("👥 Active Peers: {}", app.network_state.peer_count),
        format!("📨 Total Messages: {}", app.network_state.message_count),
        format!("⏱️  Network Uptime: {}s", app.network_state.uptime_seconds),
        format!("🌐 Mesh Status: {}", if app.network_state.is_connected { "🟢 ACTIVE" } else { "🔴 INACTIVE" }),
        format!("📡 Transport: {}", if app.network_state.is_connected { "TCP/Stealth" } else { "None" }),
        format!("🔐 Encryption: AES-256-GCM + Ed25519"),
        format!("🛡️  Security: Multi-layer protection"),
    ];

    let status_block = Paragraph::new(status_info.join("\n"))
        .block(Block::default()
            .title("📊 NETWORK STATUS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(0, 255, 255))))
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
        app.network_state.peers.iter().map(|peer| {
            format!("✅ {} - Online - Last seen: Now", peer.name)
        }).collect()
    };

    let peer_block = Paragraph::new(peer_info.join("\n"))
        .block(Block::default()
            .title("👥 CONNECTED PEERS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(0, 255, 0))))
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
        .block(Block::default()
            .title("⚡ NETWORK ACTIONS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(255, 255, 0))))
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
            content.push(format!("... and {} more results", app.scan_results.len() - 5));
        }
        content
    };

    let scan_block = Paragraph::new(scan_content.join("\n"))
        .block(Block::default()
            .title("🔍 SCAN RESULTS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(255, 0, 255))))
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
                .title_style(Style::default().fg(Color::Rgb(255, 255, 0)).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(0, 255, 255)))
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
                .title_style(Style::default().fg(Color::Rgb(255, 255, 0)).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(0, 255, 255)))
        )
        .style(Style::default().fg(Color::Rgb(255, 255, 255)))
        .wrap(Wrap { trim: true });

    frame.render_widget(logs_widget, area);
}

/// Render IRC-style input bar
fn render_input_bar(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let input_text = if app.chat_state.input_buffer.is_empty() {
        format!("[{}] {}: ", app.chat_state.nickname, app.chat_state.current_channel)
    } else {
        format!("[{}] {}: {}", app.chat_state.nickname, app.chat_state.current_channel, app.chat_state.input_buffer)
    };

    let input_widget = Paragraph::new(input_text)
        .block(Block::default()
            .title("💬 INPUT")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(0, 255, 255))))
        .style(Style::default().fg(Color::Rgb(255, 255, 255)).bold())
        .alignment(Alignment::Left);

    frame.render_widget(input_widget, area);
} 

/// Render the main menu
fn render_main_menu(frame: &mut Frame, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),  // Header with ASCII art
            Constraint::Length(3),  // Status bar
            Constraint::Min(0),     // Menu items
            Constraint::Length(3),  // Footer
        ])
        .split(frame.size());

    render_header(frame, chunks[0], app);
    render_status_bar(frame, app);
    render_menu_items(frame, chunks[2], app);
    render_footer(frame, chunks[3], app);
}

/// Render the dashboard
fn render_dashboard(frame: &mut Frame, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),  // Header
            Constraint::Length(3),  // Status bar
            Constraint::Min(0),     // Dashboard content
            Constraint::Length(3),  // Footer
        ])
        .split(frame.size());

    render_header(frame, chunks[0], app);
    render_status_bar(frame, app);
    render_dashboard_content(frame, chunks[2], app);
    render_footer(frame, chunks[3], app);
}

/// Render the tab interface (original chat/network/config/logs)
fn render_tab_interface(frame: &mut Frame, app: &mut TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),  // Header with ASCII art
            Constraint::Length(3),  // Status bar
            Constraint::Min(0),     // Main content
            Constraint::Length(3),  // Input bar
        ])
        .split(frame.size());

    render_header(frame, chunks[0], app);
    render_status_bar(frame, app);
    render_main_content(frame, chunks[2], app);
    render_input_bar(frame, chunks[3], app);
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
                Style::default()
                    .fg(Color::Rgb(255, 255, 255)) // White for others
            };
            ListItem::new(*item).style(style)
        })
        .collect();

    let menu_widget = List::new(menu_items)
        .block(
            Block::default()
                .title("🎮 MAIN MENU - Select an option")
                .title_style(Style::default().fg(Color::Rgb(255, 255, 0)).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(0, 255, 255)))
        );

    frame.render_widget(menu_widget, area);
}

/// Render the main dashboard with comprehensive backend features
fn render_dashboard_content(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Content
        ])
        .split(area);

    // Dashboard header
    let header = Paragraph::new("🌐 GHOSTWIRE DASHBOARD")
        .style(Style::default().fg(Color::Rgb(255, 255, 0)).bold())
        .alignment(Alignment::Center);
    frame.render_widget(header, chunks[0]);

    // Dashboard content in scrollable area
    let content_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50), // Left column
            Constraint::Percentage(50), // Right column
        ])
        .split(chunks[1]);

    // Left column - Network & Security
    render_dashboard_left_column(frame, content_chunks[0], app);
    
    // Right column - Actions & Status
    render_dashboard_right_column(frame, content_chunks[1], app);
}

/// Render left column of dashboard
fn render_dashboard_left_column(frame: &mut Frame, area: Rect, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(8),  // Network Status
            Constraint::Length(8),  // Security Status
            Constraint::Min(0),     // Scan Results
        ])
        .split(area);

    // Network Status
    let network_status = vec![
        format!("📡 Status: {}", if app.network_state.is_connected { "🟢 CONNECTED" } else { "🔴 DISCONNECTED" }),
        format!("👥 Peers: {}", app.network_state.peer_count),
        format!("📨 Messages: {}", app.network_state.message_count),
        format!("⏱️  Uptime: {}s", app.network_state.uptime_seconds),
        format!("🆔 Key ID: GW-{}", &app.identity_id[..8]),
        format!("🔑 Transport: {}", if app.network_state.is_connected { "Active" } else { "None" }),
    ];

    let network_block = Paragraph::new(network_status.join("\n"))
        .block(Block::default()
            .title("🌐 NETWORK STATUS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(0, 255, 255))))
        .style(Style::default().fg(Color::White));
    frame.render_widget(network_block, chunks[0]);

    // Security Status
    let security_status = vec![
        "🔒 Encryption: AES-256-GCM".to_string(),
        "🛡️  Authentication: Ed25519".to_string(),
        "⚡ Perfect Forward Secrecy: Enabled".to_string(),
        format!("🚨 Alerts: {}", app.security_alerts.len()),
        "🛡️  Threat Detection: Active".to_string(),
        "🔐 Zero-Knowledge: Enabled".to_string(),
    ];

    let security_block = Paragraph::new(security_status.join("\n"))
        .block(Block::default()
            .title("🛡️ SECURITY STATUS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(255, 0, 0))))
        .style(Style::default().fg(Color::White));
    frame.render_widget(security_block, chunks[1]);

    // Scan Results
    let scan_content = if app.scan_results.is_empty() {
        vec!["🔍 No recent network scan".to_string()]
    } else {
        app.scan_results.clone()
    };

    let scan_block = Paragraph::new(scan_content.join("\n"))
        .block(Block::default()
            .title("🔍 NETWORK SCAN RESULTS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(0, 255, 0))))
        .style(Style::default().fg(Color::White));
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
        .block(Block::default()
            .title("⚡ QUICK ACTIONS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(255, 255, 0))))
        .style(Style::default().fg(Color::White));
    frame.render_widget(actions_block, chunks[0]);

    // Security Alerts
    let alerts_content = if app.security_alerts.is_empty() {
        vec!["✅ No security alerts".to_string()]
    } else {
        app.security_alerts.iter().take(5).cloned().collect()
    };

    let alerts_block = Paragraph::new(alerts_content.join("\n"))
        .block(Block::default()
            .title("🚨 SECURITY ALERTS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(255, 0, 0))))
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
        .block(Block::default()
            .title("💻 SYSTEM INFO")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(0, 255, 255))))
        .style(Style::default().fg(Color::White));
    frame.render_widget(system_block, chunks[2]);
}

/// Render footer with navigation info
fn render_footer(frame: &mut Frame, area: Rect, _app: &TuiApp) {
    let footer_text = "↑↓ Navigate | Enter Select | Q Quit | ESC Back | H Help";
    let footer_widget = Paragraph::new(footer_text)
        .block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(0, 255, 255)))
        )
        .style(
            Style::default()
                .fg(Color::Rgb(128, 128, 128))
                .add_modifier(Modifier::BOLD)
        );

    frame.render_widget(footer_widget, area);
}

/// Render about screen
fn render_about(frame: &mut Frame, area: Rect, _app: &TuiApp) {
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
                .title_style(Style::default().fg(Color::Rgb(255, 255, 0)).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(0, 255, 255)))
        )
        .style(Style::default().fg(Color::Rgb(255, 255, 255)))
        .wrap(Wrap { trim: true });

    frame.render_widget(about_widget, area);
}

/// Render help screen
fn render_help(frame: &mut Frame, area: Rect, _app: &TuiApp) {
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
                .title_style(Style::default().fg(Color::Rgb(255, 255, 0)).add_modifier(Modifier::BOLD))
                .borders(Borders::ALL)
                .border_style(Style::default().fg(Color::Rgb(0, 255, 255)))
        )
        .style(Style::default().fg(Color::Rgb(255, 255, 255)))
        .wrap(Wrap { trim: true });

    frame.render_widget(help_widget, area);
} 

/// Render security interface
fn render_security(frame: &mut Frame, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Content
        ])
        .split(frame.size());

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
        format!("🛡️ Threat Detection: {}", if app.advanced_features.threat_detection { "ACTIVE" } else { "INACTIVE" }),
        format!("🔑 Key Rotation: {}", if app.advanced_features.encryption_rotation { "ENABLED" } else { "DISABLED" }),
        format!("👥 Peer Verification: {}", if app.advanced_features.peer_verification { "ENABLED" } else { "DISABLED" }),
        format!("🚨 Security Score: {:.0}%", app.system_stats.security_score),
        format!("🚨 Alerts: {}", app.security_alerts.len()),
        format!("🔒 Stealth Mode: {}", if app.advanced_features.stealth_mode { "ACTIVE" } else { "INACTIVE" }),
        format!("🚨 Panic Mode: {}", if app.advanced_features.panic_mode { "ACTIVE" } else { "INACTIVE" }),
    ];

    let security_block = Paragraph::new(security_status.join("\n"))
        .block(Block::default()
            .title("🛡️ SECURITY STATUS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(255, 0, 0))))
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
        .block(Block::default()
            .title("⚡ SECURITY ACTIONS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(255, 0, 0))))
        .style(Style::default().fg(Color::White));
    frame.render_widget(actions_block, content_chunks[1]);
}

/// Render advanced features interface
fn render_advanced(frame: &mut Frame, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Content
        ])
        .split(frame.size());

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
        format!("🔒 Stealth Mode: {}", if app.advanced_features.stealth_mode { "ACTIVE" } else { "INACTIVE" }),
        format!("🚨 Panic Mode: {}", if app.advanced_features.panic_mode { "ACTIVE" } else { "INACTIVE" }),
        format!("🚨 Emergency Mode: {}", if app.advanced_features.emergency_mode { "ACTIVE" } else { "INACTIVE" }),
        format!("💾 Auto Backup: {}", if app.advanced_features.auto_backup { "ENABLED" } else { "DISABLED" }),
        format!("🛡️ Threat Detection: {}", if app.advanced_features.threat_detection { "ACTIVE" } else { "INACTIVE" }),
        format!("📊 Performance Monitoring: {}", if app.advanced_features.performance_monitoring { "ACTIVE" } else { "INACTIVE" }),
        format!("🌐 Network Optimization: {}", if app.advanced_features.network_optimization { "ENABLED" } else { "DISABLED" }),
        format!("🔑 Encryption Rotation: {}", if app.advanced_features.encryption_rotation { "ENABLED" } else { "DISABLED" }),
        format!("👥 Peer Verification: {}", if app.advanced_features.peer_verification { "ENABLED" } else { "DISABLED" }),
        format!("🌫️ Traffic Obfuscation: {}", if app.advanced_features.traffic_obfuscation { "ACTIVE" } else { "INACTIVE" }),
    ];

    let features_block = Paragraph::new(feature_status.join("\n"))
        .block(Block::default()
            .title("⚡ FEATURE STATUS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(255, 165, 0))))
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
        .block(Block::default()
            .title("🎛️ ADVANCED CONTROLS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(255, 165, 0))))
        .style(Style::default().fg(Color::White));
    frame.render_widget(actions_block, content_chunks[1]);
}

/// Render diagnostics interface
fn render_diagnostics(frame: &mut Frame, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Content
        ])
        .split(frame.size());

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
        format!("🌐 Network Throughput: {:.1} MB/s", app.system_stats.network_throughput),
        format!("🔗 Active Connections: {}", app.system_stats.active_connections),
        format!("🔐 Encryption Overhead: {:.1}%", app.system_stats.encryption_overhead),
        format!("⏱️ Latency: {:.1}ms", app.system_stats.latency_ms),
        format!("📦 Packet Loss: {:.1}%", app.system_stats.packet_loss),
        format!("⏱️ Uptime: {}s", app.system_stats.uptime_seconds),
        format!("🔐 Security Score: {:.0}%", app.system_stats.security_score),
        format!("💾 Last Backup: {}", if let Some(backup) = app.system_stats.last_backup {
            format!("{}s ago", backup.elapsed().as_secs())
        } else {
            "Never".to_string()
        }),
    ];

    let metrics_block = Paragraph::new(system_metrics.join("\n"))
        .block(Block::default()
            .title("📊 SYSTEM METRICS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(0, 255, 255))))
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
        .block(Block::default()
            .title("🔧 DIAGNOSTIC TOOLS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(0, 255, 255))))
        .style(Style::default().fg(Color::White));
    frame.render_widget(actions_block, content_chunks[1]);
}

/// Render settings interface
fn render_settings(frame: &mut Frame, app: &TuiApp) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Content
        ])
        .split(frame.size());

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
        .block(Block::default()
            .title("⚙️ CURRENT SETTINGS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(255, 255, 0))))
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
        .block(Block::default()
            .title("🎛️ SETTINGS CONTROLS")
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Rgb(255, 255, 0))))
        .style(Style::default().fg(Color::White));
    frame.render_widget(actions_block, content_chunks[1]);
} 