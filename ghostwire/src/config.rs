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
    pub auto_update: bool,
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

#[derive(Clone, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub mesh_enabled: bool,
    pub reticulum_enabled: bool,
    pub briar_enabled: bool,
    pub stealth_tcp_enabled: bool,
    pub max_peers: usize,
    pub connection_timeout_secs: u64,
    #[serde(skip)]
    pub stealth_handshake_secret: Option<String>,
    /// Obfs4 obfuscation settings for DPI evasion
    pub obfs4_enabled: bool,
    /// Obfs4 secret key (hex-encoded, if not provided a random key is generated)
    pub obfs4_secret_key: Option<String>,
}

impl std::fmt::Debug for NetworkConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("NetworkConfig")
            .field("mesh_enabled", &self.mesh_enabled)
            .field("reticulum_enabled", &self.reticulum_enabled)
            .field("briar_enabled", &self.briar_enabled)
            .field("stealth_tcp_enabled", &self.stealth_tcp_enabled)
            .field("max_peers", &self.max_peers)
            .field("connection_timeout_secs", &self.connection_timeout_secs)
            .field(
                "stealth_handshake_secret",
                &self.stealth_handshake_secret.as_ref().map(|_| "********"),
            )
            .field("obfs4_enabled", &self.obfs4_enabled)
            .field(
                "obfs4_secret_key",
                &self.obfs4_secret_key.as_ref().map(|_| "********"),
            )
            .finish()
    }
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
    // AI/ML Features
    pub gemma_2b: bool,      // Gemma 4 E2B (2.3B params, edge-optimized)
    pub gemma_4b: bool,      // Gemma 4 E4B (4B params, balanced)
    pub gemma_26b: bool,     // Gemma 4 MoE (26B total, 4B active)
    pub gemma_31b: bool,     // Gemma 4 Dense (31B, max quality)
    pub ai_routing: bool,    // Enable AI-powered routing decisions
    pub ai_threat_detection: bool,  // Enable AI threat analysis
    // Gemma 4 Configuration
    pub gemma_version: String,          // "3", "3n", or "4" (default: "4")
    pub inference_backend: String,      // "ollama", "litert", or "local" (default: "ollama")
    pub gemma_model_variant: String,    // "e2b", "e4b", "26b", "31b" (auto-detected if empty)
    pub ollama_url: String,             // Ollama server URL (default: "http://localhost:11434")
    pub litert_model_path: String,      // Path to LiteRT-LM .tflite model file
    pub litert_num_threads: usize,      // CPU threads for LiteRT-LM inference
    pub litert_use_gpu: bool,           // Use GPU delegate for LiteRT-LM
    // FunctionGemma Configuration
    pub function_gemma_enabled: bool,   // Enable FunctionGemma tool calling
    pub function_gemma_model: String,   // FunctionGemma model tag (default: "functiongemma:270m")
    pub function_gemma_url: String,     // FunctionGemma backend URL
    // Edge Device Detection
    pub auto_detect_device: bool,       // Auto-detect device class and select model
    pub device_class: String,           // "phone", "tablet", "raspberry_pi", "laptop", "server"
    pub available_ram_mb: u64,          // Available RAM in MB (0 = auto-detect)
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 8080,
                web_enabled: true,
                cli_enabled: true,
                auto_update: true,
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
                stealth_handshake_secret: None,
                obfs4_enabled: true,  // Always-on obfuscation by default
                obfs4_secret_key: None,  // Generated randomly if not provided
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
                // AI/ML: Default to E2B for edge compatibility
                gemma_2b: true,
                gemma_4b: false,
                gemma_26b: false,
                gemma_31b: false,
                ai_routing: true,
                ai_threat_detection: true,
                // Gemma 4 Configuration
                gemma_version: "4".to_string(),
                inference_backend: "ollama".to_string(),
                gemma_model_variant: String::new(), // Auto-detect
                ollama_url: "http://localhost:11434".to_string(),
                litert_model_path: "models/gemma4_e2b_litert.tflite".to_string(),
                litert_num_threads: 4,
                litert_use_gpu: false,
                // FunctionGemma Configuration
                function_gemma_enabled: true,
                function_gemma_model: "functiongemma:270m".to_string(),
                function_gemma_url: "http://localhost:11434".to_string(),
                // Edge Device Detection
                auto_detect_device: true,
                device_class: String::new(), // Auto-detect
                available_ram_mb: 0, // Auto-detect
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

    pub fn load_from_path(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        Self::load_from_file(path)
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

        // Auto-update from environment
        if let Ok(val) = env::var("GHOSTWIRE_AUTO_UPDATE") {
            config.server.auto_update = val.to_lowercase() == "true" || val == "1";
        }

        // Stealth handshake secret from environment
        if let Ok(secret) = env::var("GHOSTWIRE_STEALTH_SECRET") {
            config.network.stealth_handshake_secret = Some(secret);
        }

        // Obfs4 configuration from environment
        if let Ok(val) = env::var("GHOSTWIRE_OBFS4_ENABLED") {
            config.network.obfs4_enabled = val.to_lowercase() == "true" || val == "1";
        }
        if let Ok(secret) = env::var("GHOSTWIRE_OBFS4_SECRET") {
            config.network.obfs4_secret_key = Some(secret);
        }

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
            user: env::var("MATRIX_USER").unwrap_or_else(|_| "@ghostwire:matrix.org".to_string()),
            access_token: env::var("MATRIX_ACCESS_TOKEN").unwrap_or_else(|_| "".to_string()),
        })
    }

    pub fn get_meshtastic_config(&self) -> Option<MeshtasticConfig> {
        if !self.features.meshtastic_bridge {
            return None;
        }

        Some(MeshtasticConfig {
            device_path: env::var("MESHTASTIC_DEVICE_PATH")
                .unwrap_or_else(|_| "/dev/ttyUSB0".to_string()),
            channel: env::var("MESHTASTIC_CHANNEL").unwrap_or_else(|_| "main".to_string()),
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
pub const DEFAULT_MAGIC_BYTES: &[u8; 5] = b"GWSTH";
pub const DEFAULT_HANDSHAKE_TIMEOUT_SECS: u64 = 1;
pub const DEFAULT_MAX_CONN_PER_MIN: usize = 10;
pub const DEFAULT_STEALTH_VERSION: u8 = 0x01;
pub const DEFAULT_MAX_HANDSHAKE_ATTEMPTS: usize = 3;
pub const DEFAULT_CONNECTION_TIMEOUT_SECS: u64 = 30;
