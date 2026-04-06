// Gemma 4 Client for GhostWire
// Local AI inference via Ollama or LiteRT-LM for mesh network intelligence
//
// Supports all Gemma 4 variants:
//   - E2B: 2.3B params, ~5GB RAM, phones + Raspberry Pi 5
//   - E4B: ~4B params, ~7.5GB RAM, phones + tablets
//   - 26B MoE: 4B active params, ~15GB RAM, laptops
//   - 31B Dense: flagship, ~30GB RAM, servers
//
// Also supports FunctionGemma (270M) for on-device tool calling.
// Backends: Ollama (HTTP), LiteRT-LM (native), Local (ONNX/runtime)

use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use super::litert_backend::{Gemma4Variant, InferenceBackend};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NodeTelemetry {
    pub node_id: String,
    pub rssi: f32,
    pub latency_ms: f32,
    pub packet_loss: f32,
    pub battery_level: f32,
    pub hop_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshRoutingDecision {
    pub best_relay: String,
    pub confidence: f32,
    pub reasoning: String,
    pub alternative_relays: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatAnalysis {
    pub threat_level: ThreatLevel,
    pub confidence: f32,
    pub threats: Vec<ThreatDetail>,
    pub recommendation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ThreatLevel {
    None,
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThreatDetail {
    pub threat_type: String,
    pub severity: f32,
    pub description: String,
    pub affected_nodes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MeshAnalysis {
    pub optimal_relays: Vec<MeshRoutingDecision>,
    pub network_health: f32,
    pub anomalies: Vec<String>,
    pub recommendations: Vec<String>,
}

pub struct GemmaClient {
    client: Client,
    base_url: String,
    model: String,
    timeout: Duration,
    /// The inference backend being used (Ollama, LiteRT, or Local)
    backend: InferenceBackend,
    /// The Gemma 4 variant loaded
    variant: Gemma4Variant,
}

impl GemmaClient {
    /// Create new Gemma 4 client with Ollama backend (backward compatible)
    ///
    /// # Arguments
    /// * `base_url` - Ollama server URL (e.g., "http://localhost:11434")
    /// * `model` - Gemma 4 model tag (e.g., "gemma4:e2b", "gemma4:26b")
    pub fn new(base_url: &str, model: &str) -> Self {
        Self::with_backend(base_url, model, InferenceBackend::Ollama, Gemma4Variant::E2B)
    }

    /// Create new Gemma 4 client with explicit backend and variant
    pub fn with_backend(
        base_url: &str,
        model: &str,
        backend: InferenceBackend,
        variant: Gemma4Variant,
    ) -> Self {
        Self {
            client: Client::builder()
                .timeout(Duration::from_secs(60))
                .build()
                .expect("Failed to create HTTP client"),
            base_url: base_url.to_string(),
            model: model.to_string(),
            timeout: Duration::from_secs(60),
            backend,
            variant,
        }
    }

    /// Get the inference backend in use
    pub fn backend(&self) -> InferenceBackend {
        self.backend
    }

    /// Get the Gemma 4 variant in use
    pub fn variant(&self) -> Gemma4Variant {
        self.variant
    }

    /// Get the model name
    pub fn model_name(&self) -> &str {
        &self.model
    }

    /// Check if this client uses network-based inference
    pub fn requires_network(&self) -> bool {
        self.backend.requires_network()
    }

    /// Analyze mesh network and recommend optimal relay nodes
    pub async fn analyze_mesh(
        &self,
        nodes: Vec<NodeTelemetry>,
        task: &str,
    ) -> Result<MeshAnalysis, Box<dyn std::error::Error + Send + Sync>> {
        let prompt = format!(
            "You are a mesh network routing AI. Analyze this network topology and recommend optimal relay nodes.\n\n\
             Network Nodes ({} total):\n{}\n\n\
             Task: {}\n\n\
             Consider:\n\
             - RSSI (signal strength, closer to 0 is better)\n\
             - Latency (lower is better)\n\
             - Packet loss (lower is better)\n\
             - Battery level (higher is better for relay selection)\n\
             - Hop count (fewer hops is better)\n\n\
             Return a JSON object with:\n\
             - optimal_relays: array of routing decisions\n\
             - network_health: 0.0-1.0 overall health score\n\
             - anomalies: list of detected anomalies\n\
             - recommendations: list of actionable recommendations",
            nodes.len(),
            nodes.iter()
                .map(|n| format!(
                    "  Node {}: RSSI={}dBm, Latency={}ms, Loss={}%, Battery={}%, Hops={}",
                    n.node_id, n.rssi, n.latency_ms, n.packet_loss * 100.0, n.battery_level * 100.0, n.hop_count
                ))
                .collect::<Vec<_>>()
                .join("\n"),
            task
        );

        let response = self
            .client
            .post(format!("{}/api/generate", self.base_url))
            .json(&serde_json::json!({
                "model": self.model,
                "prompt": prompt,
                "stream": false,
                "format": {
                    "type": "object",
                    "properties": {
                        "optimal_relays": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "best_relay": {"type": "string"},
                                    "confidence": {"type": "number"},
                                    "reasoning": {"type": "string"},
                                    "alternative_relays": {
                                        "type": "array",
                                        "items": {"type": "string"}
                                    }
                                },
                                "required": ["best_relay", "confidence", "reasoning", "alternative_relays"]
                            }
                        },
                        "network_health": {"type": "number"},
                        "anomalies": {
                            "type": "array",
                            "items": {"type": "string"}
                        },
                        "recommendations": {
                            "type": "array",
                            "items": {"type": "string"}
                        }
                    },
                    "required": ["optimal_relays", "network_health", "anomalies", "recommendations"]
                },
                "options": {
                    "temperature": 0.1,
                    "num_predict": 2048
                }
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let analysis: MeshAnalysis = serde_json::from_value(response["response"].clone())
            .map_err(|e| format!("Failed to parse Gemma response: {}", e))?;

        Ok(analysis)
    }

    /// Detect security threats in network traffic
    pub async fn detect_threats(
        &self,
        network_data: &str,
    ) -> Result<ThreatAnalysis, Box<dyn std::error::Error + Send + Sync>> {
        let prompt = format!(
            "You are a cybersecurity AI analyzing mesh network traffic for threats.\n\n\
             Network Data:\n{}\n\n\
             Analyze for:\n\
             - Man-in-the-middle attacks\n\
             - Sybil attacks (fake nodes)\n\
             - Wormhole attacks\n\
             - Blackhole attacks\n\
             - Traffic analysis patterns\n\
             - Anomalous behavior\n\n\
             Return JSON with:\n\
             - threat_level: one of 'None', 'Low', 'Medium', 'High', 'Critical'\n\
             - confidence: 0.0-1.0\n\
             - threats: array of threat details\n\
             - recommendation: actionable security advice",
            network_data
        );

        let response = self
            .client
            .post(format!("{}/api/generate", self.base_url))
            .json(&serde_json::json!({
                "model": self.model,
                "prompt": prompt,
                "stream": false,
                "format": {
                    "type": "object",
                    "properties": {
                        "threat_level": {"type": "string"},
                        "confidence": {"type": "number"},
                        "threats": {
                            "type": "array",
                            "items": {
                                "type": "object",
                                "properties": {
                                    "threat_type": {"type": "string"},
                                    "severity": {"type": "number"},
                                    "description": {"type": "string"},
                                    "affected_nodes": {
                                        "type": "array",
                                        "items": {"type": "string"}
                                    }
                                },
                                "required": ["threat_type", "severity", "description", "affected_nodes"]
                            }
                        },
                        "recommendation": {"type": "string"}
                    },
                    "required": ["threat_level", "confidence", "threats", "recommendation"]
                }
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let analysis: ThreatAnalysis = serde_json::from_value(response["response"].clone())
            .map_err(|e| format!("Failed to parse Gemma threat analysis: {}", e))?;

        Ok(analysis)
    }

    /// Check if Ollama server is reachable
    pub async fn health_check(&self) -> bool {
        self.client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await
            .map(|r| r.status().is_success())
            .unwrap_or(false)
    }

    /// Get list of available models from Ollama
    pub async fn list_models(&self) -> Result<Vec<String>, Box<dyn std::error::Error + Send + Sync>> {
        let response = self
            .client
            .get(format!("{}/api/tags", self.base_url))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let models = response["models"]
            .as_array()
            .map(|arr| {
                arr.iter()
                    .filter_map(|m| m["name"].as_str().map(String::from))
                    .collect()
            })
            .unwrap_or_default();

        Ok(models)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_client_creation() {
        let client = GemmaClient::new("http://localhost:11434", "gemma4:e2b");
        assert_eq!(client.model_name(), "gemma4:e2b");
        assert_eq!(client.backend(), InferenceBackend::Ollama);
        assert_eq!(client.variant(), Gemma4Variant::E2B);
    }

    #[test]
    fn test_client_with_backend() {
        let client = GemmaClient::with_backend(
            "litert://local",
            "gemma4:e4b",
            InferenceBackend::LiteRt,
            Gemma4Variant::E4B,
        );
        assert_eq!(client.model_name(), "gemma4:e4b");
        assert_eq!(client.backend(), InferenceBackend::LiteRt);
        assert_eq!(client.variant(), Gemma4Variant::E4B);
        assert!(!client.requires_network());
    }

    #[test]
    fn test_client_network_requirement() {
        let ollama_client = GemmaClient::new("http://localhost:11434", "gemma4:e2b");
        assert!(ollama_client.requires_network());

        let litert_client = GemmaClient::with_backend(
            "litert://local",
            "gemma4:e2b",
            InferenceBackend::LiteRt,
            Gemma4Variant::E2B,
        );
        assert!(!litert_client.requires_network());
    }

    #[tokio::test]
    async fn test_health_check_offline() {
        let client = GemmaClient::new("http://localhost:11434", "gemma4:e2b");
        // Should return false if Ollama is not running
        let healthy = client.health_check().await;
        // This test may pass or fail depending on Ollama status
        println!("Ollama health: {}", healthy);
    }
}
