use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub security: SecurityConfig,
    pub network: NetworkConfig,
    pub encryption: EncryptionConfig,
    pub features: FeatureFlags,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub web_enabled: bool,
    pub cli_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub max_connections_per_ip: usize,
    pub max_messages_per_minute: usize,
    pub blacklist_enabled: bool,
    pub threat_detection_enabled: bool,
    pub allowed_ips: Vec<String>,
    pub blocked_ips: Vec<String>,
    pub key_rotation_interval_days: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub mesh_enabled: bool,
    pub reticulum_enabled: bool,
    pub briar_enabled: bool,
    pub stealth_tcp_enabled: bool,
    pub max_peers: usize,
    pub connection_timeout_secs: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EncryptionConfig {
    pub algorithm: String,
    pub key_size: usize,
    pub session_timeout_hours: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureFlags {
    pub matrix_bridge: bool,
    pub meshtastic_bridge: bool,
    pub web_ui: bool,
    pub cli: bool,
    pub api: bool,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                web_enabled: true,
                cli_enabled: true,
            },
            security: SecurityConfig {
                max_connections_per_ip: 10,
                max_messages_per_minute: 100,
                blacklist_enabled: true,
                threat_detection_enabled: true,
                allowed_ips: vec![],
                blocked_ips: vec![],
                key_rotation_interval_days: 7,
            },
            network: NetworkConfig {
                mesh_enabled: true,
                reticulum_enabled: false,
                briar_enabled: false,
                stealth_tcp_enabled: false,
                max_peers: 50,
                connection_timeout_secs: 30,
            },
            encryption: EncryptionConfig {
                algorithm: "AES-256-GCM".to_string(),
                key_size: 32,
                session_timeout_hours: 1,
            },
            features: FeatureFlags {
                matrix_bridge: false,
                meshtastic_bridge: false,
                web_ui: true,
                cli: true,
                api: true,
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        // Try to load from config file first
        let config_paths = vec![
            "ghostwire.toml",
            "config/ghostwire.toml",
            "~/.config/ghostwire/config.toml",
        ];

        for path in config_paths {
            if let Ok(config) = Self::load_from_file(path) {
                return Ok(config);
            }
        }

        // Fall back to environment variables
        Ok(Self::from_env())
    }

    fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let expanded_path = if path.starts_with("~/") {
            let home = env::var("HOME")?;
            path.replacen("~/", &format!("{}/", home), 1)
        } else {
            path.to_string()
        };

        if !Path::new(&expanded_path).exists() {
            return Err("Config file not found".into());
        }

        let content = fs::read_to_string(&expanded_path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }

    fn from_env() -> Self {
        let mut config = Config::default();

        // Server config from environment
        if let Ok(host) = env::var("GHOSTWIRE_HOST") {
            config.server.host = host;
        }
        if let Ok(port) = env::var("GHOSTWIRE_PORT") {
            if let Ok(port_num) = port.parse() {
                config.server.port = port_num;
            }
        }

        // Security config from environment
        if let Ok(max_conn) = env::var("GHOSTWIRE_MAX_CONNECTIONS_PER_IP") {
            if let Ok(max_conn_num) = max_conn.parse() {
                config.security.max_connections_per_ip = max_conn_num;
            }
        }

        // Feature flags from environment
        config.features.matrix_bridge = env::var("GHOSTWIRE_MATRIX_BRIDGE").is_ok();
        config.features.meshtastic_bridge = env::var("GHOSTWIRE_MESHTASTIC_BRIDGE").is_ok();

        config
    }

    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string_pretty(self)?;
        fs::write(path, content)?;
        Ok(())
    }

    pub fn get_matrix_config(&self) -> Option<MatrixConfig> {
        if !self.features.matrix_bridge {
            return None;
        }

        Some(MatrixConfig {
            homeserver: env::var("MATRIX_HOMESERVER")
                .unwrap_or_else(|_| "https://matrix.org".to_string()),
            user: env::var("MATRIX_USER")
                .unwrap_or_else(|_| "@ghostwire:matrix.org".to_string()),
            access_token: env::var("MATRIX_ACCESS_TOKEN")
                .unwrap_or_else(|_| "".to_string()),
        })
    }

    pub fn get_meshtastic_config(&self) -> Option<MeshtasticConfig> {
        if !self.features.meshtastic_bridge {
            return None;
        }

        Some(MeshtasticConfig {
            device_path: env::var("MESHTASTIC_DEVICE_PATH")
                .unwrap_or_else(|_| "/dev/ttyUSB0".to_string()),
            channel: env::var("MESHTASTIC_CHANNEL")
                .unwrap_or_else(|_| "main".to_string()),
        })
    }
}

#[derive(Debug, Clone)]
pub struct MatrixConfig {
    pub homeserver: String,
    pub user: String,
    pub access_token: String,
}

#[derive(Debug, Clone)]
pub struct MeshtasticConfig {
    pub device_path: String,
    pub channel: String,
}

// Constants that can be overridden by config
pub const DEFAULT_MAGIC_BYTES: &[u8] = b"GWSTH";
pub const DEFAULT_HANDSHAKE_TIMEOUT_SECS: u64 = 1;
pub const DEFAULT_MAX_CONN_PER_MIN: usize = 10;
pub const DEFAULT_STEALTH_VERSION: u8 = 0x01;
pub const DEFAULT_MAX_HANDSHAKE_ATTEMPTS: usize = 3;
pub const DEFAULT_CONNECTION_TIMEOUT_SECS: u64 = 30; 