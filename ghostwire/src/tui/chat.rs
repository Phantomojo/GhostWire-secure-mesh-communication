// No change needed yet

use crate::core::message::Message;
use chrono::{DateTime, Utc};
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct ChatState {
    pub messages: VecDeque<ChatMessage>,
    pub input_buffer: String,
    pub cursor_position: usize,
    pub current_channel: String,
    pub nickname: String,
    pub channels: Vec<Channel>,
    pub users: Vec<User>,
    pub is_connected: bool,
    pub server_info: ServerInfo,
    pub message_history: VecDeque<ChatMessage>, // Keep full history
}

#[derive(Debug, Clone)]
pub struct Channel {
    pub name: String,
    pub topic: String,
    pub user_count: usize,
    pub is_joined: bool,
    pub messages: VecDeque<ChatMessage>,
    pub users: Vec<String>, // nicknames in this channel
}

#[derive(Debug, Clone)]
pub struct User {
    pub nickname: String,
    pub username: String,
    pub hostname: String,
    pub realname: String,
    pub is_operator: bool,
    pub is_voice: bool,
    pub is_away: bool,
    pub away_message: Option<String>,
    pub last_seen: DateTime<Utc>,
    pub channels: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ServerInfo {
    pub name: String,
    pub version: String,
    pub uptime: u64,
    pub user_count: usize,
    pub channel_count: usize,
    pub max_users: usize,
    pub motd: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct ChatMessage {
    pub message: Message,
    pub display_name: String,
    pub is_own: bool,
    pub encryption_status: EncryptionStatus,
    pub message_type: MessageType,
    pub timestamp: DateTime<Utc>,
    pub channel: Option<String>,
}

#[derive(Debug, Clone, Copy)]
pub enum EncryptionStatus {
    Encrypted,
    Unencrypted,
    Error,
    Unknown,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    PrivMsg, // Regular message
    Notice,  // Server notice
    Join,    // User joined
    Part,    // User left
    Quit,    // User quit
    Nick,    // Nickname change
    Mode,    // Mode change
    Topic,   // Topic change
    Kick,    // User kicked
    Ban,     // User banned
    System,  // System message
    Action,  // /me action
    Ctcp,    // CTCP request
}

impl ChatState {
    pub fn new() -> Self {
        Self {
            messages: VecDeque::new(),
            input_buffer: String::new(),
            cursor_position: 0,
            current_channel: "#ghostwire".to_string(),
            nickname: "gw-user".to_string(),
            channels: vec![Channel {
                name: "ghostwire".to_string(),
                topic: "Mesh Status".to_string(),
                user_count: 1,
                is_joined: true,
                messages: VecDeque::new(),
                users: vec!["gw-user".to_string()],
            }],
            users: Vec::new(),
            is_connected: true,
            server_info: ServerInfo {
                name: "GhostWire".to_string(),
                version: "1.0.0".to_string(),
                uptime: 0,
                user_count: 1,
                channel_count: 1,
                max_users: 100,
                motd: vec!["Welcome to GhostWire Secure TUI".to_string()],
            },
            message_history: VecDeque::new(),
        }
    }

    /// Ingest a message from the GhostWire Core backend
    pub fn ingest_core_message(&mut self, message: Message) {
        // Map raw GhostWire messages to TUI chat messages
        let msg_type = if message.content.starts_with("[SYSTEM]") {
            MessageType::System
        } else {
            MessageType::PrivMsg
        };

        // Determine channel - for now we default to #ghostwire
        let channel = Some("#ghostwire".to_string());

        self.add_message(message, msg_type, channel);
    }

    pub fn add_message(
        &mut self,
        message: Message,
        msg_type: MessageType,
        channel: Option<String>,
    ) {
        let is_own = message.sender == self.nickname;
        let display_name = message.sender.clone();

        let chat_message = ChatMessage {
            message,
            display_name,
            is_own,
            encryption_status: EncryptionStatus::Encrypted,
            message_type: msg_type,
            timestamp: Utc::now(),
            channel: channel.clone(),
        };

        self.messages.push_back(chat_message.clone());
        self.message_history.push_back(chat_message);

        // Keep only last 100 messages in current view
        if self.messages.len() > 100 {
            self.messages.pop_front();
        }

        // Keep last 1000 messages in history
        if self.message_history.len() > 1000 {
            self.message_history.pop_front();
        }
    }

    pub fn add_input_char(&mut self, c: char) {
        if self.cursor_position < self.input_buffer.len() {
            self.input_buffer.insert(self.cursor_position, c);
        } else {
            self.input_buffer.push(c);
        }
        self.cursor_position += 1;
    }

    pub fn delete_char(&mut self) {
        if self.cursor_position > 0 && self.cursor_position <= self.input_buffer.len() {
            self.input_buffer.remove(self.cursor_position - 1);
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.cursor_position > 0 {
            self.cursor_position -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.cursor_position < self.input_buffer.len() {
            self.cursor_position += 1;
        }
    }

    pub fn clear_input(&mut self) {
        self.input_buffer.clear();
        self.cursor_position = 0;
    }

    pub fn get_input(&self) -> String {
        self.input_buffer.clone()
    }

    pub fn join_channel(&mut self, channel_name: &str) {
        if !channel_name.starts_with('#') {
            return;
        }

        if let Some(channel) = self.channels.iter_mut().find(|c| c.name == channel_name) {
            channel.is_joined = true;
            channel.user_count += 1;
            if !channel.users.contains(&self.nickname) {
                channel.users.push(self.nickname.clone());
            }
        } else {
            // Create new channel
            let new_channel = Channel {
                name: channel_name.to_string(),
                topic: "".to_string(),
                user_count: 1,
                is_joined: true,
                messages: VecDeque::new(),
                users: vec![self.nickname.clone()],
            };
            self.channels.push(new_channel);
        }

        self.current_channel = channel_name.to_string();
    }

    pub fn part_channel(&mut self, channel_name: &str) {
        if let Some(channel) = self.channels.iter_mut().find(|c| c.name == channel_name) {
            channel.is_joined = false;
            channel.user_count = channel.user_count.saturating_sub(1);
            channel.users.retain(|u| u != &self.nickname);
        }

        if self.current_channel == channel_name {
            // Switch to first available channel
            if let Some(first_channel) = self.channels.iter().find(|c| c.is_joined) {
                self.current_channel = first_channel.name.clone();
            }
        }
    }

    pub fn change_nickname(&mut self, new_nick: &str) {
        let old_nick = self.nickname.clone();
        self.nickname = new_nick.to_string();

        // Update in all channels
        for channel in &mut self.channels {
            if let Some(pos) = channel.users.iter().position(|u| u == &old_nick) {
                channel.users[pos] = new_nick.to_string();
            }
        }

        // Update own user record
        if let Some(user) = self.users.iter_mut().find(|u| u.nickname == old_nick) {
            user.nickname = new_nick.to_string();
        }
    }

    pub fn get_current_channel_messages(&self) -> Vec<&ChatMessage> {
        self.messages
            .iter()
            .filter(|msg| msg.channel.as_ref() == Some(&self.current_channel))
            .collect()
    }

    pub fn get_channel_users(&self, channel_name: &str) -> Vec<&User> {
        self.users
            .iter()
            .filter(|user| user.channels.contains(&channel_name.to_string()))
            .collect()
    }

    pub fn find_user(&self, nickname: &str) -> Option<&User> {
        self.users.iter().find(|u| u.nickname == nickname)
    }

    pub fn add_user(&mut self, user: User) {
        if !self.users.iter().any(|u| u.nickname == user.nickname) {
            self.users.push(user);
        }
    }

    pub fn remove_user(&mut self, nickname: &str) {
        self.users.retain(|u| u.nickname != nickname);
        for channel in &mut self.channels {
            channel.users.retain(|u| u != nickname);
        }
    }
}

pub fn handle_input(app: &mut crate::tui::TuiApp, key: crossterm::event::KeyEvent) {
    match key.code {
        crossterm::event::KeyCode::Char(c) => {
            app.chat_state.add_input_char(c);
        }
        crossterm::event::KeyCode::Backspace => {
            app.chat_state.delete_char();
        }
        crossterm::event::KeyCode::Left => {
            app.chat_state.move_cursor_left();
        }
        crossterm::event::KeyCode::Right => {
            app.chat_state.move_cursor_right();
        }
        crossterm::event::KeyCode::Enter => {
            let input = app.chat_state.get_input();
            if !input.is_empty() {
                // Handle IRC commands
                if input.starts_with('/') {
                    handle_irc_command(app, &input);
                } else {
                    // Send regular message
                    send_irc_message(app, &input);
                }
                app.chat_state.clear_input();
            }
        }
        _ => {}
    }
}

fn handle_irc_command(app: &mut crate::tui::TuiApp, command: &str) {
    let parts: Vec<&str> = command.split_whitespace().collect();
    if parts.is_empty() {
        return;
    }

    match parts[0] {
        "/join" | "/j" => {
            if parts.len() > 1 {
                app.chat_state.join_channel(parts[1]);
                add_system_message(app, &format!("Joined channel {}", parts[1]));
            }
        }
        "/part" | "/leave" => {
            if parts.len() > 1 {
                app.chat_state.part_channel(parts[1]);
                add_system_message(app, &format!("Left channel {}", parts[1]));
            } else {
                let current_channel = app.chat_state.current_channel.clone();
                app.chat_state.part_channel(&current_channel);
                add_system_message(app, &format!("Left channel {}", current_channel));
            }
        }
        "/nick" => {
            if parts.len() > 1 {
                let old_nick = app.chat_state.nickname.clone();
                app.chat_state.change_nickname(parts[1]);
                add_system_message(
                    app,
                    &format!("Nickname changed from {} to {}", old_nick, parts[1]),
                );
            }
        }
        "/msg" | "/m" => {
            if parts.len() > 2 {
                let target = parts[1];
                let message = parts[2..].join(" ");
                send_private_message(app, target, &message);
            }
        }
        "/me" => {
            if parts.len() > 1 {
                let action = parts[1..].join(" ");
                send_action_message(app, &action);
            }
        }
        "/topic" => {
            if parts.len() > 1 {
                let topic = parts[1..].join(" ");
                set_channel_topic(app, &topic);
            }
        }
        "/whois" => {
            if parts.len() > 1 {
                show_whois(app, parts[1]);
            }
        }
        "/list" => {
            show_channel_list(app);
        }
        "/users" => {
            show_user_list(app);
        }
        "/help" => {
            show_help(app);
        }
        "/quit" => {
            add_system_message(app, "Disconnecting from server...");
            app.chat_state.is_connected = false;
        }
        "/window" | "/w" => {
            handle_window_command(app, &parts);
        }
        "/set" => {
            handle_set_command(app, &parts);
        }
        "/save" => {
            add_system_message(app, "💾 Configuration and layout saved to disk (Stub)");
        }
        _ => {
            add_system_message(app, &format!("Unknown command: {}", parts[0]));
        }
    }
}
fn handle_window_command(app: &mut crate::tui::TuiApp, parts: &[&str]) {
    use crate::tui::{SplitDirection, TuiLayoutTree};

    if parts.len() < 2 {
        add_system_message(app, "Usage: /window <split|show|hide|list>");
        return;
    }

    match parts[1] {
        "split" => {
            let direction = match parts.get(2) {
                Some(&"v") => SplitDirection::Vertical,
                _ => SplitDirection::Horizontal,
            };

            // Recursive split: current tree becomes a child of a new split, alongside a new window
            let current_tree = app.layout_tree.clone();
            // Default to showing window 1 (Status) in the new pane for now
            let new_pane = TuiLayoutTree::Window(0);

            app.layout_tree = TuiLayoutTree::Split {
                direction,
                ratios: vec![50, 50],
                children: vec![current_tree, new_pane],
            };

            add_system_message(app, &format!("Window split {:?}", direction));
        }
        "show" => {
            if let Some(i_str) = parts.get(2) {
                if let Ok(i) = i_str.parse::<usize>() {
                    if i > 0 && i <= app.windows.len() {
                        app.active_window_index = i - 1;
                        // For non-split mode, we just update active_window_index.
                        // In split mode, this is more complex because we need to know WHICH split to update.
                        // For now, if it's a single window tree, we update it.
                        if let TuiLayoutTree::Window(_) = app.layout_tree {
                            app.layout_tree = TuiLayoutTree::Window(i - 1);
                        }
                        add_system_message(app, &format!("Focused window {}", i));
                    }
                }
            }
        }
        "hide" => {
            // Reset to single window view of active window
            app.layout_tree = TuiLayoutTree::Window(app.active_window_index);
            add_system_message(app, "Layout reset to single window.");
        }
        "list" => {
            add_system_message(app, "Active Windows:");
            let names: Vec<String> = app
                .windows
                .iter()
                .map(|window| match window {
                    crate::tui::TuiWindow::Status => "Status".to_string(),
                    crate::tui::TuiWindow::Channel(c) => format!("#{}", c),
                    crate::tui::TuiWindow::Query(q) => format!("@{}", q),
                    crate::tui::TuiWindow::Network => "Network".to_string(),
                    crate::tui::TuiWindow::Logs => "Logs".to_string(),
                    crate::tui::TuiWindow::Config => "Config".to_string(),
                })
                .collect();

            for (i, name) in names.iter().enumerate() {
                add_system_message(app, &format!("  {}: {}", i + 1, name));
            }
        }
        _ => {
            add_system_message(app, &format!("Unknown window command: {}", parts[1]));
        }
    }
}

fn handle_set_command(app: &mut crate::tui::TuiApp, parts: &[&str]) {
    if parts.len() < 2 {
        add_system_message(app, "Usage: /set <key> [value]");
        return;
    }

    let key = parts[1];
    if parts.len() == 2 {
        // Query setting
        add_system_message(app, &format!("Setting {} = (Current Value Stub)", key));
    } else {
        // Change setting
        let value = parts[2..].join(" ");
        add_system_message(app, &format!("Setting {} changed to {} (Stub)", key, value));
    }
}

fn send_irc_message(app: &mut crate::tui::TuiApp, content: &str) {
    let message = Message {
        id: uuid::Uuid::new_v4(),
        content: content.to_string(),
        sender: app.chat_state.nickname.clone(),
        recipient: app.chat_state.current_channel.clone(),
        timestamp: Utc::now().timestamp() as u64,
        encrypted: true,
        message_type: "irc".to_string(),
        encryption_status: "encrypted".to_string(),
    };

    app.chat_state.add_message(
        message,
        MessageType::PrivMsg,
        Some(app.chat_state.current_channel.clone()),
    );

    // Send via backend
    if let Err(e) = app
        .message_sender
        .send(crate::tui::TuiMessage::SendMessage(content.to_string()))
    {
        eprintln!("Failed to send message to channel: {}", e);
    }
}

fn send_private_message(app: &mut crate::tui::TuiApp, target: &str, content: &str) {
    let message = Message {
        id: uuid::Uuid::new_v4(),
        content: content.to_string(),
        sender: app.chat_state.nickname.clone(),
        recipient: target.to_string(),
        timestamp: Utc::now().timestamp() as u64,
        encrypted: true,
        message_type: "irc".to_string(),
        encryption_status: "encrypted".to_string(),
    };

    app.chat_state
        .add_message(message, MessageType::PrivMsg, None);

    // Send via backend
    if let Err(e) = app
        .message_sender
        .send(crate::tui::TuiMessage::SendMessage(content.to_string()))
    {
        eprintln!("Failed to send private message: {}", e);
    }
}

fn send_action_message(app: &mut crate::tui::TuiApp, action: &str) {
    let message = Message {
        id: uuid::Uuid::new_v4(),
        content: format!("* {} {}", app.chat_state.nickname, action),
        sender: app.chat_state.nickname.clone(),
        recipient: app.chat_state.current_channel.clone(),
        timestamp: Utc::now().timestamp() as u64,
        encrypted: true,
        message_type: "irc".to_string(),
        encryption_status: "encrypted".to_string(),
    };

    app.chat_state.add_message(
        message,
        MessageType::Action,
        Some(app.chat_state.current_channel.clone()),
    );
}

fn add_system_message(app: &mut crate::tui::TuiApp, content: &str) {
    let message = Message {
        id: uuid::Uuid::new_v4(),
        content: content.to_string(),
        sender: "System".to_string(),
        recipient: "all".to_string(),
        timestamp: Utc::now().timestamp() as u64,
        encrypted: false,
        message_type: "irc".to_string(),
        encryption_status: "unencrypted".to_string(),
    };

    app.chat_state.add_message(
        message,
        MessageType::System,
        Some(app.chat_state.current_channel.clone()),
    );
}

fn set_channel_topic(app: &mut crate::tui::TuiApp, topic: &str) {
    if let Some(channel) = app
        .chat_state
        .channels
        .iter_mut()
        .find(|c| c.name == app.chat_state.current_channel)
    {
        channel.topic = topic.to_string();
        add_system_message(app, &format!("Topic changed to: {}", topic));
    }
}

fn show_whois(app: &mut crate::tui::TuiApp, nickname: &str) {
    if let Some(user) = app.chat_state.find_user(nickname) {
        let nickname = user.nickname.clone();
        let username = user.username.clone();
        let hostname = user.hostname.clone();
        let realname = user.realname.clone();
        let channels = user.channels.clone();
        let is_away = user.is_away;
        let away_message = user.away_message.clone();

        add_system_message(
            app,
            &format!(
                "WHOIS {}: {}@{} ({})",
                nickname, username, hostname, realname
            ),
        );
        add_system_message(app, &format!("Channels: {}", channels.join(", ")));
        if is_away {
            add_system_message(
                app,
                &format!(
                    "Away: {}",
                    away_message.as_ref().unwrap_or(&"No message".to_string())
                ),
            );
        }
    } else {
        add_system_message(app, &format!("No such nick: {}", nickname));
    }
}

fn show_channel_list(app: &mut crate::tui::TuiApp) {
    add_system_message(app, "Channel List:");
    let channels = app.chat_state.channels.clone();
    for channel in channels {
        add_system_message(
            app,
            &format!("{} {} {}", channel.name, channel.user_count, channel.topic),
        );
    }
}

fn show_user_list(app: &mut crate::tui::TuiApp) {
    add_system_message(
        app,
        &format!("Users in {}:", app.chat_state.current_channel),
    );
    let current_channel = app.chat_state.current_channel.clone();
    let users = app.chat_state.get_channel_users(&current_channel);
    let user_list: Vec<String> = users
        .iter()
        .map(|user| {
            let prefix = if user.is_operator {
                "@"
            } else if user.is_voice {
                "+"
            } else {
                ""
            };
            format!("{}{}", prefix, user.nickname)
        })
        .collect();

    for user_text in user_list {
        add_system_message(app, &user_text);
    }
}

fn show_help(app: &mut crate::tui::TuiApp) {
    add_system_message(app, "Available commands:");
    add_system_message(app, "/join <channel> - Join a channel");
    add_system_message(app, "/part [channel] - Leave a channel");
    add_system_message(app, "/nick <newnick> - Change nickname");
    add_system_message(app, "/msg <nick> <message> - Send private message");
    add_system_message(app, "/me <action> - Send action message");
    add_system_message(app, "/topic <topic> - Set channel topic");
    add_system_message(app, "/whois <nick> - Get user information");
    add_system_message(app, "/list - Show channel list");
    add_system_message(app, "/users - Show users in current channel");
    add_system_message(app, "/help - Show this help");
    add_system_message(app, "/quit - Disconnect from server");
}

pub fn format_message(msg: &ChatMessage) -> String {
    let timestamp = msg.timestamp.format("%H:%M");

    match msg.message_type {
        MessageType::PrivMsg => {
            if msg.is_own {
                format!(
                    "[{}] <{}> {}",
                    timestamp, msg.display_name, msg.message.content
                )
            } else {
                format!(
                    "[{}] <{}> {}",
                    timestamp, msg.display_name, msg.message.content
                )
            }
        }
        MessageType::Notice => {
            format!(
                "[{}] -{}- {}",
                timestamp, msg.display_name, msg.message.content
            )
        }
        MessageType::Join => {
            format!(
                "[{}] *** {} has joined {}",
                timestamp,
                msg.display_name,
                msg.channel.as_ref().unwrap_or(&"".to_string())
            )
        }
        MessageType::Part => {
            format!(
                "[{}] *** {} has left {}",
                timestamp,
                msg.display_name,
                msg.channel.as_ref().unwrap_or(&"".to_string())
            )
        }
        MessageType::Quit => {
            format!("[{}] *** {} has quit IRC", timestamp, msg.display_name)
        }
        MessageType::Nick => {
            format!(
                "[{}] *** {} is now known as {}",
                timestamp, msg.display_name, msg.message.content
            )
        }
        MessageType::Mode => {
            format!(
                "[{}] *** {} sets mode {}",
                timestamp, msg.display_name, msg.message.content
            )
        }
        MessageType::Topic => {
            format!(
                "[{}] *** {} changes topic to: {}",
                timestamp, msg.display_name, msg.message.content
            )
        }
        MessageType::Kick => {
            format!(
                "[{}] *** {} has been kicked by {}",
                timestamp, msg.message.content, msg.display_name
            )
        }
        MessageType::Ban => {
            format!(
                "[{}] *** {} has been banned by {}",
                timestamp, msg.message.content, msg.display_name
            )
        }
        MessageType::System => {
            format!("[{}] *** {}", timestamp, msg.message.content)
        }
        MessageType::Action => {
            format!(
                "[{}] {} {}",
                timestamp, msg.display_name, msg.message.content
            )
        }
        MessageType::Ctcp => {
            format!(
                "[{}] CTCP {}: {}",
                timestamp, msg.display_name, msg.message.content
            )
        }
    }
}
