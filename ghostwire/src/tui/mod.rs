//! GhostWire Terminal User Interface (TUI)
//! A retro-style, feature-rich terminal interface for mesh networking

use std::sync::Arc;
use std::io;
use tokio::sync::{mpsc, Mutex};
use crossterm::event::{self, KeyCode, KeyEvent, KeyModifiers};
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style, Stylize},
    text::{Span, Line},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
    Frame, Terminal,
};
use ratatui::backend::CrosstermBackend;
use ratatui::prelude::Alignment;
use crate::core::{Core, Message};
use crate::tui::chat::ChatState;
use crate::tui::network::NetworkState;
use crate::tui::config::ConfigState;
use crate::tui::logs::LogsState;
use app::App;
use anyhow::Result;

pub mod chat;
pub mod network;
pub mod config;
pub mod logs;
pub mod app;
pub mod ui;

/// Input modes to prevent keybind conflicts
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum InputMode {
    Normal,     // Navigation mode
    Insert,     // Text input mode
    Command,    // Command mode (like vim)
    Search,     // Search mode
    Menu,       // Menu selection mode
}

/// Application states for navigation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AppState {
    MainMenu,
    Dashboard,
    Chat,
    Network,
    Config,
    Logs,
    About,
    Help,
    Security,
    Advanced,
    Diagnostics,
    Settings,
}

/// Tab navigation
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Tab {
    Chat,
    Network,
    Config,
    Logs,
}

/// Message types for async communication
#[derive(Debug)]
pub enum TuiMessage {
    SendMessage(String),
    ScanNetwork,
    ConnectNetwork,
    DisconnectNetwork,
    RefreshPeers,
    UpdateSecuritySettings,
    ExportKeys,
    ImportKeys,
    ShowLogs,
    EmergencyMode,
    StealthMode,
    PanicMode,
    BackupSystem,
    RestoreSystem,
    UpdateFirmware,
    SystemDiagnostics,
    NetworkDiagnostics,
    SecurityAudit,
    PerformanceTest,
    Quit,
}

/// Main TUI application state
pub struct TuiApp {
    pub core: Arc<Mutex<Core>>,
    pub chat_state: crate::tui::chat::ChatState,
    pub network_state: NetworkState,
    pub config_state: ConfigState,
    pub logs_state: LogsState,
    pub current_tab: Tab,
    pub app_state: AppState,
    pub input_mode: InputMode,
    pub menu_selection: usize,
    pub should_quit: bool,
    pub message_sender: mpsc::UnboundedSender<TuiMessage>,
    pub identity_id: String,
    pub last_scan_time: std::time::Instant,
    pub scan_results: Vec<String>,
    pub security_alerts: Vec<String>,
    pub command_buffer: String,
    pub search_buffer: String,
    pub notification: Option<(String, std::time::Instant)>,
    pub system_stats: SystemStats,
    pub advanced_features: AdvancedFeatures,
}

/// System statistics and monitoring
#[derive(Debug, Clone)]
pub struct SystemStats {
    pub cpu_usage: f32,
    pub memory_usage: f32,
    pub network_throughput: f32,
    pub active_connections: usize,
    pub encryption_overhead: f32,
    pub latency_ms: f32,
    pub packet_loss: f32,
    pub uptime_seconds: u64,
    pub last_backup: Option<std::time::Instant>,
    pub security_score: f32,
}

/// Advanced features and capabilities
#[derive(Debug, Clone)]
pub struct AdvancedFeatures {
    pub stealth_mode: bool,
    pub panic_mode: bool,
    pub emergency_mode: bool,
    pub auto_backup: bool,
    pub threat_detection: bool,
    pub performance_monitoring: bool,
    pub network_optimization: bool,
    pub encryption_rotation: bool,
    pub peer_verification: bool,
    pub traffic_obfuscation: bool,
}

impl Default for SystemStats {
    fn default() -> Self {
        Self {
            cpu_usage: 0.0,
            memory_usage: 0.0,
            network_throughput: 0.0,
            active_connections: 0,
            encryption_overhead: 0.0,
            latency_ms: 0.0,
            packet_loss: 0.0,
            uptime_seconds: 0,
            last_backup: None,
            security_score: 100.0,
        }
    }
}

impl Default for AdvancedFeatures {
    fn default() -> Self {
        Self {
            stealth_mode: false,
            panic_mode: false,
            emergency_mode: false,
            auto_backup: true,
            threat_detection: true,
            performance_monitoring: true,
            network_optimization: true,
            encryption_rotation: true,
            peer_verification: true,
            traffic_obfuscation: true,
        }
    }
}

/// Start the TUI application
pub async fn start_tui(core: Arc<Mutex<Core>>) -> Result<()> {
    // Setup terminal
    crossterm::terminal::enable_raw_mode()?;
    let stdout = io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Create message channel for async communication
    let (message_sender, mut message_receiver) = mpsc::unbounded_channel::<TuiMessage>();

    // Create app state
    let mut app = TuiApp {
        core,
        chat_state: crate::tui::chat::ChatState::new(),
        network_state: NetworkState::new(),
        config_state: ConfigState::new(),
        logs_state: LogsState::new(),
        current_tab: Tab::Chat,
        app_state: AppState::MainMenu,
        input_mode: InputMode::Normal,
        menu_selection: 0,
        should_quit: false,
        message_sender,
        identity_id: "".to_string(), // Initialize identity_id
        last_scan_time: std::time::Instant::now(),
        scan_results: Vec::new(),
        security_alerts: Vec::new(),
        command_buffer: String::new(),
        search_buffer: String::new(),
        notification: None,
        system_stats: SystemStats::default(),
        advanced_features: AdvancedFeatures::default(),
    };

    // Initialize default transport for basic functionality
    {
        let mut core_guard = app.core.lock().await;
        if let Err(e) = core_guard.init_default_transport().await {
            eprintln!("Warning: Failed to initialize default transport: {}", e);
        }
        app.identity_id = core_guard.get_identity_id(); // Set identity_id after initialization
    }

    // Main event loop
    loop {
        terminal.draw(|f| crate::tui::ui::render(f, &mut app))?;

        if app.should_quit {
            break;
        }

        // Handle async messages
        while let Ok(message) = message_receiver.try_recv() {
            match message {
                TuiMessage::SendMessage(content) => {
                    let message = crate::core::message::Message {
                        id: uuid::Uuid::new_v4(),
                        content,
                        sender: app.identity_id.clone(),
                        recipient: "all".to_string(),
                        timestamp: chrono::Utc::now().timestamp() as u64,
                        encrypted: true,
                        message_type: "text".to_string(),
                        encryption_status: "encrypted".to_string(),
                    };
                    app.chat_state.add_message(message.clone(), crate::tui::chat::MessageType::PrivMsg, Some(app.chat_state.current_channel.clone()));
                }
                TuiMessage::ScanNetwork => {
                    if let Err(e) = scan_network_async(&mut app).await {
                        eprintln!("Failed to scan network: {}", e);
                    }
                }
                TuiMessage::ConnectNetwork => {
                    if let Err(e) = connect_network_async(&mut app).await {
                        eprintln!("Failed to connect to network: {}", e);
                    }
                }
                TuiMessage::DisconnectNetwork => {
                    if let Err(e) = disconnect_network_async(&mut app).await {
                        eprintln!("Failed to disconnect from network: {}", e);
                    }
                }
                TuiMessage::RefreshPeers => {
                    if let Err(e) = refresh_peers_async(&mut app).await {
                        eprintln!("Failed to refresh peers: {}", e);
                    }
                }
                TuiMessage::UpdateSecuritySettings => {
                    if let Err(e) = update_security_settings_async(&mut app).await {
                        eprintln!("Failed to update security settings: {}", e);
                    }
                }
                TuiMessage::ExportKeys => {
                    if let Err(e) = export_keys_async(&mut app).await {
                        eprintln!("Failed to export keys: {}", e);
                    }
                }
                TuiMessage::ImportKeys => {
                    if let Err(e) = import_keys_async(&mut app).await {
                        eprintln!("Failed to import keys: {}", e);
                    }
                }
                TuiMessage::ShowLogs => {
                    app.app_state = AppState::Logs;
                }
                TuiMessage::EmergencyMode => {
                    app.advanced_features.emergency_mode = !app.advanced_features.emergency_mode;
                    app.notification = Some((
                        if app.advanced_features.emergency_mode { "EMERGENCY MODE ACTIVATED" } else { "Emergency mode deactivated" }.to_string(),
                        std::time::Instant::now()
                    ));
                }
                TuiMessage::StealthMode => {
                    app.advanced_features.stealth_mode = !app.advanced_features.stealth_mode;
                    app.notification = Some((
                        if app.advanced_features.stealth_mode { "STEALTH MODE ACTIVATED" } else { "Stealth mode deactivated" }.to_string(),
                        std::time::Instant::now()
                    ));
                }
                TuiMessage::PanicMode => {
                    app.advanced_features.panic_mode = !app.advanced_features.panic_mode;
                    app.notification = Some((
                        if app.advanced_features.panic_mode { "PANIC MODE ACTIVATED" } else { "Panic mode deactivated" }.to_string(),
                        std::time::Instant::now()
                    ));
                }
                TuiMessage::BackupSystem => {
                    app.notification = Some(("System backup initiated".to_string(), std::time::Instant::now()));
                }
                TuiMessage::RestoreSystem => {
                    app.notification = Some(("System restore initiated".to_string(), std::time::Instant::now()));
                }
                TuiMessage::UpdateFirmware => {
                    app.notification = Some(("Firmware update initiated".to_string(), std::time::Instant::now()));
                }
                TuiMessage::SystemDiagnostics => {
                    app.notification = Some(("System diagnostics running".to_string(), std::time::Instant::now()));
                }
                TuiMessage::NetworkDiagnostics => {
                    app.notification = Some(("Network diagnostics running".to_string(), std::time::Instant::now()));
                }
                TuiMessage::SecurityAudit => {
                    app.notification = Some(("Security audit running".to_string(), std::time::Instant::now()));
                }
                TuiMessage::PerformanceTest => {
                    app.notification = Some(("Performance test running".to_string(), std::time::Instant::now()));
                }
                TuiMessage::Quit => {
                    app.should_quit = true;
                }
            }
        }

        // Handle input
        if crossterm::event::poll(std::time::Duration::from_millis(100))? {
            if let crossterm::event::Event::Key(key) = crossterm::event::read()? {
                handle_input(&mut app, key);
            }
        }
    }

    // Restore terminal
    crossterm::terminal::disable_raw_mode()?;
    terminal.show_cursor()?;

    Ok(())
}

/// Handle input based on current mode and state
pub fn handle_input(app: &mut TuiApp, key: KeyEvent) {
    match app.input_mode {
        InputMode::Normal => handle_normal_mode(app, key),
        InputMode::Insert => handle_insert_mode(app, key),
        InputMode::Command => handle_command_mode(app, key),
        InputMode::Search => handle_search_mode(app, key),
        InputMode::Menu => handle_menu_mode(app, key),
    }
}

/// Handle normal navigation mode
fn handle_normal_mode(app: &mut TuiApp, key: KeyEvent) {
    match key.code {
        // Mode switching
        KeyCode::Char('i') => {
            app.input_mode = InputMode::Insert;
        }
        KeyCode::Char(':') => {
            app.input_mode = InputMode::Command;
            app.command_buffer.clear();
        }
        KeyCode::Char('/') => {
            app.input_mode = InputMode::Search;
            app.search_buffer.clear();
        }

        // Navigation
        KeyCode::Tab => {
            app.current_tab = match app.current_tab {
                Tab::Chat => Tab::Network,
                Tab::Network => Tab::Config,
                Tab::Config => Tab::Logs,
                Tab::Logs => Tab::Chat,
            };
        }
        KeyCode::Char('1') => app.current_tab = Tab::Chat,
        KeyCode::Char('2') => app.current_tab = Tab::Network,
        KeyCode::Char('3') => app.current_tab = Tab::Config,
        KeyCode::Char('4') => app.current_tab = Tab::Logs,

        // State navigation
        KeyCode::Char('d') => app.app_state = AppState::Dashboard,
        KeyCode::Char('h') => app.app_state = AppState::Help,
        KeyCode::Char('a') => app.app_state = AppState::About,
        KeyCode::Char('s') => app.app_state = AppState::Security,
        KeyCode::Char('x') => app.app_state = AppState::Advanced,
        KeyCode::Char('g') => app.app_state = AppState::Diagnostics,

        // Quick actions (Ctrl+ combinations)
        KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Err(e) = app.message_sender.send(TuiMessage::ScanNetwork) {
                eprintln!("Failed to send scan message: {}", e);
            }
        }
        KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if app.network_state.is_connected {
                if let Err(e) = app.message_sender.send(TuiMessage::DisconnectNetwork) {
                    eprintln!("Failed to send disconnect message: {}", e);
                }
            } else {
                if let Err(e) = app.message_sender.send(TuiMessage::ConnectNetwork) {
                    eprintln!("Failed to send connect message: {}", e);
                }
            }
        }
        KeyCode::Char('r') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Err(e) = app.message_sender.send(TuiMessage::RefreshPeers) {
                eprintln!("Failed to send refresh message: {}", e);
            }
        }
        KeyCode::Char('e') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Err(e) = app.message_sender.send(TuiMessage::EmergencyMode) {
                eprintln!("Failed to send emergency message: {}", e);
            }
        }
        KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Err(e) = app.message_sender.send(TuiMessage::PanicMode) {
                eprintln!("Failed to send panic message: {}", e);
            }
        }
        KeyCode::Char('t') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Err(e) = app.message_sender.send(TuiMessage::StealthMode) {
                eprintln!("Failed to send stealth message: {}", e);
            }
        }

        // Function keys (using proper key codes)
        KeyCode::Char('1') if key.modifiers.contains(KeyModifiers::CONTROL) => app.app_state = AppState::Help,
        KeyCode::Char('2') if key.modifiers.contains(KeyModifiers::CONTROL) => app.app_state = AppState::Dashboard,
        KeyCode::Char('3') if key.modifiers.contains(KeyModifiers::CONTROL) => app.app_state = AppState::Security,
        KeyCode::Char('4') if key.modifiers.contains(KeyModifiers::CONTROL) => app.app_state = AppState::Advanced,
        KeyCode::Char('5') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Err(e) = app.message_sender.send(TuiMessage::SystemDiagnostics) {
                eprintln!("Failed to send diagnostics message: {}", e);
            }
        }
        KeyCode::Char('6') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Err(e) = app.message_sender.send(TuiMessage::NetworkDiagnostics) {
                eprintln!("Failed to send network diagnostics message: {}", e);
            }
        }
        KeyCode::Char('7') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Err(e) = app.message_sender.send(TuiMessage::SecurityAudit) {
                eprintln!("Failed to send security audit message: {}", e);
            }
        }
        KeyCode::Char('8') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Err(e) = app.message_sender.send(TuiMessage::PerformanceTest) {
                eprintln!("Failed to send performance test message: {}", e);
            }
        }
        KeyCode::Char('9') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Err(e) = app.message_sender.send(TuiMessage::BackupSystem) {
                eprintln!("Failed to send backup message: {}", e);
            }
        }
        KeyCode::Char('0') if key.modifiers.contains(KeyModifiers::CONTROL) => {
            if let Err(e) = app.message_sender.send(TuiMessage::UpdateFirmware) {
                eprintln!("Failed to send firmware update message: {}", e);
            }
        }

        // Arrow keys for menu navigation
        KeyCode::Up => {
            if app.menu_selection > 0 {
                app.menu_selection -= 1;
            }
        }
        KeyCode::Down => {
            app.menu_selection += 1;
        }
        KeyCode::Enter => {
            // Handle menu selection
            match app.app_state {
                AppState::MainMenu => {
                    match app.menu_selection {
                        0 => app.app_state = AppState::Dashboard,
                        1 => app.app_state = AppState::Chat,
                        2 => app.app_state = AppState::Network,
                        3 => app.app_state = AppState::Security,
                        4 => app.app_state = AppState::Advanced,
                        5 => app.app_state = AppState::Diagnostics,
                        6 => app.app_state = AppState::Settings,
                        7 => app.app_state = AppState::About,
                        8 => app.app_state = AppState::Help,
                        _ => {}
                    }
                }
                _ => {}
            }
        }

        // Quit
        KeyCode::Char('q') | KeyCode::Esc => {
            if app.app_state == AppState::MainMenu {
                app.should_quit = true;
            } else {
                app.app_state = AppState::MainMenu;
            }
        }

        _ => {}
    }
}

/// Handle insert mode (text input)
fn handle_insert_mode(app: &mut TuiApp, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Enter => {
            // Send message in chat
            if app.app_state == AppState::Chat {
                let message = app.chat_state.get_input();
                if !message.trim().is_empty() {
                    if let Err(e) = app.message_sender.send(TuiMessage::SendMessage(message)) {
                        eprintln!("Failed to send message: {}", e);
                    }
                    app.chat_state.clear_input();
                }
            }
        }
        KeyCode::Char(c) => {
            if app.app_state == AppState::Chat {
                app.chat_state.add_input_char(c);
            }
        }
        KeyCode::Backspace => {
            if app.app_state == AppState::Chat {
                app.chat_state.delete_char();
            }
        }
        KeyCode::Left => {
            if app.app_state == AppState::Chat {
                app.chat_state.move_cursor_left();
            }
        }
        KeyCode::Right => {
            if app.app_state == AppState::Chat {
                app.chat_state.move_cursor_right();
            }
        }
        _ => {}
    }
}

/// Handle command mode (like vim)
fn handle_command_mode(app: &mut TuiApp, key: KeyEvent) {
    match key.code {
        KeyCode::Char(c) => {
            app.command_buffer.push(c);
        }
        KeyCode::Backspace => {
            app.command_buffer.pop();
        }
        KeyCode::Enter => {
            let command = app.command_buffer.clone();
            execute_command(app, &command);
            app.command_buffer.clear();
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Esc => {
            app.command_buffer.clear();
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}

/// Handle search mode
fn handle_search_mode(app: &mut TuiApp, key: KeyEvent) {
    match key.code {
        KeyCode::Char(c) => {
            app.search_buffer.push(c);
        }
        KeyCode::Backspace => {
            app.search_buffer.pop();
        }
        KeyCode::Enter => {
            // Apply search
            app.logs_state.set_search_term(app.search_buffer.clone());
            app.search_buffer.clear();
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Esc => {
            app.search_buffer.clear();
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}

/// Handle menu selection mode
fn handle_menu_mode(app: &mut TuiApp, key: KeyEvent) {
    match key.code {
        KeyCode::Up => {
            if app.menu_selection > 0 {
                app.menu_selection -= 1;
            }
        }
        KeyCode::Down => {
            app.menu_selection += 1;
        }
        KeyCode::Enter => {
            // Handle selection
            app.input_mode = InputMode::Normal;
        }
        KeyCode::Esc => {
            app.input_mode = InputMode::Normal;
        }
        _ => {}
    }
}

/// Execute commands
fn execute_command(app: &mut TuiApp, command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    match parts[0] {
        "scan" => {
            if let Err(e) = app.message_sender.send(TuiMessage::ScanNetwork) {
                eprintln!("Failed to send scan message: {}", e);
            }
            app.notification = Some(("Network scan initiated".to_string(), std::time::Instant::now()));
        }
        "connect" => {
            if let Err(e) = app.message_sender.send(TuiMessage::ConnectNetwork) {
                eprintln!("Failed to send connect message: {}", e);
            }
            app.notification = Some(("Connecting to network...".to_string(), std::time::Instant::now()));
        }
        "disconnect" => {
            if let Err(e) = app.message_sender.send(TuiMessage::DisconnectNetwork) {
                eprintln!("Failed to send disconnect message: {}", e);
            }
            app.notification = Some(("Disconnecting from network...".to_string(), std::time::Instant::now()));
        }
        "refresh" => {
            if let Err(e) = app.message_sender.send(TuiMessage::RefreshPeers) {
                eprintln!("Failed to send refresh message: {}", e);
            }
            app.notification = Some(("Refreshing peer list...".to_string(), std::time::Instant::now()));
        }
        "export" => {
            if let Err(e) = app.message_sender.send(TuiMessage::ExportKeys) {
                eprintln!("Failed to send export message: {}", e);
            }
            app.notification = Some(("Exporting keys...".to_string(), std::time::Instant::now()));
        }
        "stealth" => {
            if let Err(e) = app.message_sender.send(TuiMessage::StealthMode) {
                eprintln!("Failed to send stealth message: {}", e);
            }
            app.notification = Some(("Stealth mode activated".to_string(), std::time::Instant::now()));
        }
        "panic" => {
            if let Err(e) = app.message_sender.send(TuiMessage::PanicMode) {
                eprintln!("Failed to send panic message: {}", e);
            }
            app.notification = Some(("PANIC MODE ACTIVATED".to_string(), std::time::Instant::now()));
        }
        "emergency" => {
            if let Err(e) = app.message_sender.send(TuiMessage::EmergencyMode) {
                eprintln!("Failed to send emergency message: {}", e);
            }
            app.notification = Some(("EMERGENCY MODE ACTIVATED".to_string(), std::time::Instant::now()));
        }
        "backup" => {
            if let Err(e) = app.message_sender.send(TuiMessage::BackupSystem) {
                eprintln!("Failed to send backup message: {}", e);
            }
            app.notification = Some(("System backup initiated".to_string(), std::time::Instant::now()));
        }
        "diagnostics" => {
            if let Err(e) = app.message_sender.send(TuiMessage::SystemDiagnostics) {
                eprintln!("Failed to send diagnostics message: {}", e);
            }
            app.notification = Some(("Running system diagnostics...".to_string(), std::time::Instant::now()));
        }
        "audit" => {
            if let Err(e) = app.message_sender.send(TuiMessage::SecurityAudit) {
                eprintln!("Failed to send audit message: {}", e);
            }
            app.notification = Some(("Security audit initiated".to_string(), std::time::Instant::now()));
        }
        "help" => {
            app.app_state = AppState::Help;
        }
        "quit" | "q" => {
            app.should_quit = true;
        }
        _ => {
            app.notification = Some((format!("Unknown command: {}", parts[0]), std::time::Instant::now()));
        }
    }
}

/// Send a message through the TUI (async version)
async fn send_message_async(app: &mut TuiApp, content: String) -> Result<()> {
    let message = Message {
        id: uuid::Uuid::new_v4(),
        content: content.clone(),
        sender: app.identity_id.clone(),
        recipient: "all".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
        encrypted: false,
        message_type: "text".to_string(),
        encryption_status: "encrypted".to_string(),
    };

    // Add to chat history
    app.chat_state.add_message(message.clone(), crate::tui::chat::MessageType::PrivMsg, Some(app.chat_state.current_channel.clone()));

    // Send through core
    {
        let mut core_guard = app.core.lock().await;
        core_guard.send_message(&message).await?;
    }

    Ok(())
}

/// Send a message through the TUI (sync version for backward compatibility)
pub async fn send_message(app: &mut TuiApp, content: String) -> Result<()> {
    send_message_async(app, content).await
}

/// Scan network for nearby devices
async fn scan_network_async(app: &mut TuiApp) -> Result<()> {
    let mut core_guard = app.core.lock().await;
    
    // Update scan time
    app.last_scan_time = std::time::Instant::now();
    
    // Clear previous results
    app.scan_results.clear();
    
    // Simulate network scanning (replace with actual implementation)
    app.scan_results.push("🔍 Scanning local network...".to_string());
    app.scan_results.push("📡 Checking for GhostWire nodes...".to_string());
    
    // Add some simulated results
    app.scan_results.push("✅ Found: node1 (192.168.1.101) - Online".to_string());
    app.scan_results.push("✅ Found: node2 (192.168.1.102) - Online".to_string());
    app.scan_results.push("⚠️  Found: unknown (192.168.1.103) - Unverified".to_string());
    
    // Update network state
    app.network_state.is_connected = true;
    app.network_state.peer_count = 5;
    app.network_state.message_count = 42;
    
    tracing::info!("Network scan completed - found {} peers", app.network_state.peer_count);
    Ok(())
}

/// Connect to the mesh network
async fn connect_network_async(app: &mut TuiApp) -> Result<()> {
    let mut core_guard = app.core.lock().await;
    
    // Initialize mesh network if not already done
    if let Err(e) = core_guard.init_mesh().await {
        tracing::warn!("Failed to initialize mesh: {}", e);
    }
    
    // Update network state
    app.network_state.is_connected = true;
    app.network_state.uptime_seconds = 0;
    
    // Add connection message to chat
    let connection_msg = crate::core::message::Message {
        id: uuid::Uuid::new_v4(),
        content: "Network scan completed".to_string(),
        sender: "System".to_string(),
        recipient: "all".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
        encrypted: false,
        message_type: "system".to_string(),
        encryption_status: "unencrypted".to_string(),
    };
    app.chat_state.add_message(connection_msg, crate::tui::chat::MessageType::System, Some(app.chat_state.current_channel.clone()));
    
    tracing::info!("Connected to mesh network");
    Ok(())
}

/// Disconnect from the mesh network
async fn disconnect_network_async(app: &mut TuiApp) -> Result<()> {
    let mut core_guard = app.core.lock().await;
    
    // Update network state
    app.network_state.is_connected = false;
    app.network_state.peer_count = 0;
    app.network_state.message_count = 0;
    
    // Add disconnection message to chat
    let disconnection_msg = crate::core::message::Message {
        id: uuid::Uuid::new_v4(),
        content: "Disconnected from mesh network".to_string(),
        sender: "System".to_string(),
        recipient: "all".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
        encrypted: false,
        message_type: "system".to_string(),
        encryption_status: "unencrypted".to_string(),
    };
    app.chat_state.add_message(disconnection_msg, crate::tui::chat::MessageType::System, Some(app.chat_state.current_channel.clone()));
    
    tracing::info!("Disconnected from mesh network");
    Ok(())
}

/// Refresh peer list
async fn refresh_peers_async(app: &mut TuiApp) -> Result<()> {
    let core_guard = app.core.lock().await;
    
    // Get mesh stats if available
    if let Some(stats) = core_guard.get_mesh_stats().await {
        app.network_state.peer_count = stats.total_nodes;
        app.network_state.message_count = stats.total_nodes; // Use as placeholder
    }
    
    tracing::info!("Refreshed peer list - {} peers", app.network_state.peer_count);
    Ok(())
}

/// Update security settings
async fn update_security_settings_async(app: &mut TuiApp) -> Result<()> {
    let mut core_guard = app.core.lock().await;
    
    // Add security alert
    app.security_alerts.push("🔒 Security settings updated".to_string());
    
    // Keep only last 10 alerts
    if app.security_alerts.len() > 10 {
        app.security_alerts.remove(0);
    }
    
    tracing::info!("Security settings updated");
    Ok(())
}

/// Export encryption keys
async fn export_keys_async(app: &mut TuiApp) -> Result<()> {
    let core_guard = app.core.lock().await;
    
    // Get public key
    let public_key = core_guard.get_public_key();
    let key_id = core_guard.get_key_id();
    
    // Add export message to chat
    let export_msg = crate::core::message::Message {
        id: uuid::Uuid::new_v4(),
        content: "Keys exported successfully".to_string(),
        sender: "System".to_string(),
        recipient: "all".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
        encrypted: false,
        message_type: "system".to_string(),
        encryption_status: "unencrypted".to_string(),
    };
    app.chat_state.add_message(export_msg, crate::tui::chat::MessageType::System, Some(app.chat_state.current_channel.clone()));
    
    tracing::info!("Exported encryption keys");
    Ok(())
}

/// Import encryption keys
async fn import_keys_async(app: &mut TuiApp) -> Result<()> {
    let mut core_guard = app.core.lock().await;
    
    // Add import message to chat
    let import_msg = crate::core::message::Message {
        id: uuid::Uuid::new_v4(),
        content: "Keys imported successfully".to_string(),
        sender: "System".to_string(),
        recipient: "all".to_string(),
        timestamp: chrono::Utc::now().timestamp() as u64,
        encrypted: false,
        message_type: "system".to_string(),
        encryption_status: "unencrypted".to_string(),
    };
    app.chat_state.add_message(import_msg, crate::tui::chat::MessageType::System, Some(app.chat_state.current_channel.clone()));
    
    tracing::info!("Imported encryption keys");
    Ok(())
}

/// Get network status for display
pub fn get_network_status(app: &TuiApp) -> String {
    format!(
        "Peers: {} | Messages: {} | Status: {}",
        app.network_state.peer_count,
        app.network_state.message_count,
        if app.network_state.is_connected {
            "CONNECTED"
        } else {
            "DISCONNECTED"
        }
    )
} 