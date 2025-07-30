//! System logs and monitoring interface for GhostWire TUI

use std::collections::VecDeque;
use chrono::{DateTime, Utc};

/// Log entry with comprehensive information
#[derive(Debug, Clone)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub module: String,
    pub message: String,
    pub details: Option<String>,
    pub source: LogSource,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogLevel {
    Debug,
    Info,
    Warning,
    Error,
    Critical,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum LogSource {
    System,
    Network,
    Security,
    Encryption,
    Mesh,
    User,
    External,
}

/// Logs state and management
pub struct LogsState {
    pub entries: VecDeque<LogEntry>,
    pub max_entries: usize,
    pub filter_level: LogLevel,
    pub filter_module: Option<String>,
    pub auto_scroll: bool,
    pub search_term: String,
    pub selected_entry: Option<usize>,
}

impl LogsState {
    pub fn new() -> Self {
        let mut logs = Self {
            entries: VecDeque::new(),
            max_entries: 1000,
            filter_level: LogLevel::Info,
            filter_module: None,
            auto_scroll: true,
            search_term: String::new(),
            selected_entry: None,
        };

        // Add some sample logs
        logs.add_entry(LogLevel::Info, "System", "GhostWire TUI initialized successfully", None, LogSource::System);
        logs.add_entry(LogLevel::Info, "Network", "Mesh network ready for connections", None, LogSource::Network);
        logs.add_entry(LogLevel::Warning, "Security", "New peer detected - verification pending", Some("Peer ID: node1"), LogSource::Security);
        logs.add_entry(LogLevel::Info, "Encryption", "Key rotation completed", None, LogSource::Encryption);
        logs.add_entry(LogLevel::Debug, "Mesh", "Topology update: 3 nodes connected", None, LogSource::Mesh);

        logs
    }

    pub fn add_entry(&mut self, level: LogLevel, module: &str, message: &str, details: Option<&str>, source: LogSource) {
        let entry = LogEntry {
            timestamp: Utc::now(),
            level,
            module: module.to_string(),
            message: message.to_string(),
            details: details.map(|s| s.to_string()),
            source,
        };

        self.entries.push_back(entry);

        // Keep only max_entries
        if self.entries.len() > self.max_entries {
            self.entries.pop_front();
        }
    }

    pub fn clear(&mut self) {
        self.entries.clear();
    }

    pub fn set_filter_level(&mut self, level: LogLevel) {
        self.filter_level = level;
    }

    pub fn set_filter_module(&mut self, module: Option<String>) {
        self.filter_module = module;
    }

    pub fn toggle_auto_scroll(&mut self) {
        self.auto_scroll = !self.auto_scroll;
    }

    pub fn set_search_term(&mut self, term: String) {
        self.search_term = term;
    }

    pub fn get_filtered_entries(&self) -> Vec<&LogEntry> {
        self.entries
            .iter()
            .filter(|entry| {
                // Level filter
                if !self.matches_level(entry) {
                    return false;
                }

                // Module filter
                if let Some(ref filter_module) = self.filter_module {
                    if entry.module != *filter_module {
                        return false;
                    }
                }

                // Search filter
                if !self.search_term.is_empty() {
                    if !entry.message.to_lowercase().contains(&self.search_term.to_lowercase()) {
                        return false;
                    }
                }

                true
            })
            .collect()
    }

    fn matches_level(&self, entry: &LogEntry) -> bool {
        match (self.filter_level, &entry.level) {
            (LogLevel::Debug, _) => true,
            (LogLevel::Info, LogLevel::Info | LogLevel::Warning | LogLevel::Error | LogLevel::Critical) => true,
            (LogLevel::Warning, LogLevel::Warning | LogLevel::Error | LogLevel::Critical) => true,
            (LogLevel::Error, LogLevel::Error | LogLevel::Critical) => true,
            (LogLevel::Critical, LogLevel::Critical) => true,
            _ => false,
        }
    }

    pub fn export_logs(&self, format: &str) -> String {
        match format {
            "json" => self.export_json(),
            "csv" => self.export_csv(),
            "text" => self.export_text(),
            _ => self.export_text(),
        }
    }

    fn export_json(&self) -> String {
        // Simplified JSON export
        let entries: Vec<String> = self.entries
            .iter()
            .map(|entry| {
                format!(
                    r#"{{"timestamp":"{}","level":"{:?}","module":"{}","message":"{}"}}"#,
                    entry.timestamp, entry.level, entry.module, entry.message
                )
            })
            .collect();
        format!("[\n{}\n]", entries.join(",\n"))
    }

    fn export_csv(&self) -> String {
        let mut csv = "timestamp,level,module,message,source\n".to_string();
        for entry in &self.entries {
            csv.push_str(&format!(
                "{},{:?},{},{},{:?}\n",
                entry.timestamp, entry.level, entry.module, entry.message, entry.source
            ));
        }
        csv
    }

    fn export_text(&self) -> String {
        self.entries
            .iter()
            .map(|entry| {
                format!(
                    "[{}] {:?} {}: {}",
                    entry.timestamp.format("%Y-%m-%d %H:%M:%S"),
                    entry.level,
                    entry.module,
                    entry.message
                )
            })
            .collect::<Vec<String>>()
            .join("\n")
    }
}

/// Handle logs-specific input
pub fn handle_input(app: &mut crate::tui::TuiApp, key: crossterm::event::KeyEvent) {
    match key.code {
        crossterm::event::KeyCode::Char('c') if key.modifiers.contains(crossterm::event::KeyModifiers::CONTROL) => {
            // Ctrl+C - Clear logs
            app.logs_state.clear();
        }
        crossterm::event::KeyCode::Char('e') => {
            // Export logs
            let json_logs = app.logs_state.export_logs("json");
            // In a real implementation, this would save to file
            app.notification = Some(("Logs exported to logs.json".to_string(), std::time::Instant::now()));
        }
        crossterm::event::KeyCode::Char('f') => {
            // Filter logs
            app.input_mode = crate::tui::InputMode::Search;
        }
        crossterm::event::KeyCode::Char('a') => {
            // Toggle auto-scroll
            app.logs_state.toggle_auto_scroll();
        }
        crossterm::event::KeyCode::Char('r') => {
            // Refresh logs
            app.logs_state.add_entry(
                crate::tui::logs::LogLevel::Info,
                "User",
                "Logs refreshed manually",
                None,
                crate::tui::logs::LogSource::User,
            );
        }
        _ => {}
    }
} 