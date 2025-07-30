//! Configuration management interface for GhostWire TUI

/// Configuration state
pub struct ConfigState {
    pub current_section: ConfigSection,
    pub selected_field: Option<String>,
    pub edit_mode: bool,
    pub config_values: Vec<ConfigField>,
}

#[derive(Debug, Clone, PartialEq)]
pub enum ConfigSection {
    Network,
    Security,
    Features,
    Advanced,
}

#[derive(Debug, Clone)]
pub struct ConfigField {
    pub name: String,
    pub value: String,
    pub description: String,
    pub field_type: FieldType,
}

#[derive(Debug, Clone)]
pub enum FieldType {
    String,
    Number,
    Boolean,
    List,
}

impl ConfigState {
    pub fn new() -> Self {
        Self {
            current_section: ConfigSection::Network,
            selected_field: None,
            edit_mode: false,
            config_values: vec![
                ConfigField {
                    name: "host".to_string(),
                    value: "127.0.0.1".to_string(),
                    description: "Server host address".to_string(),
                    field_type: FieldType::String,
                },
                ConfigField {
                    name: "port".to_string(),
                    value: "8080".to_string(),
                    description: "Server port".to_string(),
                    field_type: FieldType::Number,
                },
                ConfigField {
                    name: "mesh_enabled".to_string(),
                    value: "true".to_string(),
                    description: "Enable mesh networking".to_string(),
                    field_type: FieldType::Boolean,
                },
            ],
        }
    }

    pub fn get_field(&self, name: &str) -> Option<&ConfigField> {
        self.config_values.iter().find(|f| f.name == name)
    }

    pub fn set_field_value(&mut self, name: &str, value: String) {
        if let Some(field) = self.config_values.iter_mut().find(|f| f.name == name) {
            field.value = value;
        }
    }
}

/// Handle config-specific input
pub fn handle_input(app: &mut crate::tui::TuiApp, key: crossterm::event::KeyEvent) {
    match key.code {
        crossterm::event::KeyCode::Char('s') => {
            // Save configuration
            // TODO: Implement config save
        }
        crossterm::event::KeyCode::Char('r') => {
            // Reload configuration
            // TODO: Implement config reload
        }
        _ => {}
    }
} 