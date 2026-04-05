# Gemma 4 Integration for GhostWire

**Date:** April 4, 2026 (Release Day!)
**Model:** Google Gemma 4 - Just Released April 3, 2026

---

## 🎯 Why Gemma 4 for GhostWire

Gemma 4 is **perfect** for our mesh communication AI routing because:

1. **Edge-Optimized:** E2B (2B params) runs on 8GB RAM - ideal for Raspberry Pi mesh nodes
2. **MoE Architecture:** 26B model with only 4B active params = flagship quality at edge speeds
3. **Function Calling:** Native structured JSON output for routing decisions
4. **128K-256K Context:** Can analyze entire mesh topology in single prompt
5. **Apache 2.0 License:** Free for commercial use
6. **Multi-Modal:** E2B/E4B support audio + image (future threat detection from screenshots)
7. **Thinking Mode:** Configurable chain-of-thought for complex routing analysis

---

## 📊 Gemma 4 Variants for GhostWire

| Variant | Use Case | Hardware | VRAM/RAM | Latency |
|---------|----------|----------|----------|---------|
| **E2B** | Edge nodes (Raspberry Pi) | CPU-only OK | 8GB RAM | ~100ms |
| **E4B** | Relay nodes (laptops) | Mid GPU | 10GB VRAM | ~50ms |
| **26B MoE** | Coordinator node | RTX 3060+ | 20GB RAM | ~80ms |
| **31B Dense** | Central AI server | RTX 3090/4090 | 24GB VRAM | ~120ms |

### Recommended for GhostWire:
- **Mesh Nodes:** `gemma4:e2b` (lightweight, runs on edge devices)
- **Routing AI:** `gemma4:26b` MoE (best quality/size ratio, sparse activation)
- **Threat Detection:** `gemma4:31b` (max accuracy for security analysis)

---

## 🏗️ Integration Architecture

```
┌─────────────────────────────────────────────────────┐
│              GhostWire AI Router                     │
├─────────────────────────────────────────────────────┤
│                                                      │
│  ┌──────────────┐    ┌──────────────────────┐       │
│  │  LiteLLM     │───▶│  Gemma 4 (Local)     │       │
│  │  Proxy       │    │  Ollama @ :11434     │       │
│  │  :4000       │    └──────────────────────┘       │
│  └──────────────┘                                    │
│         │                                            │
│         ├─ Fallback: OpenAI (cloud)                  │
│         ├─ Fallback: Anthropic (cloud)               │
│         └─ Primary: Gemma 4 (local, offline)         │
│                                                      │
└─────────────────────────────────────────────────────┘
```

### Multi-Provider Routing with LiteLLM

```yaml
# litellm-config.yaml
model_list:
  # Primary: Local Gemma 4 (offline-first)
  - model_name: gemma4-local
    litellm_params:
      model: ollama/gemma4:26b
      api_base: http://localhost:11434

  # Fallback 1: OpenAI (when online)
  - model_name: gpt-4o
    litellm_params:
      model: openai/gpt-4o
      api_key: os.environ/OPENAI_API_KEY

  # Fallback 2: Anthropic
  - model_name: claude-sonnet
    litellm_params:
      model: anthropic/claude-sonnet-4-20260226
      api_key: os.environ/ANTHROPIC_API_KEY

router_settings:
  routing_strategy: simple-shuffle
  fallbacks:
    - gemma4-local
    - gpt-4o
    - claude-sonnet
  timeout: 30s
  retry_attempts: 3
```

---

## 🔧 Installation Steps

### 1. Install Ollama (if not already installed)
```bash
curl -fsSL https://ollama.com/install.sh | sh
```

### 2. Pull Gemma 4 Models
```bash
# For edge nodes (Raspberry Pi / low-power)
ollama pull gemma4:e2b

# For relay/coordinator nodes (recommended)
ollama pull gemma4:26b

# For central AI server (max accuracy)
ollama pull gemma4:31b
```

### 3. Start Ollama with Network Access
```bash
# Allow LAN access for mesh nodes to query
OLLAMA_HOST=0.0.0.0:11434 ollama serve &
```

### 4. Install LiteLLM Proxy
```bash
pip install litellm[proxy]

# Start proxy with config
litellm --config /home/ph/GhostWire-secure-mesh-communication/ghostwire/litellm-config.yaml
```

### 5. Test Local Gemma 4
```bash
curl http://localhost:11434/api/generate -d '{
  "model": "gemma4:26b",
  "prompt": "Analyze this mesh network: 3 nodes, RSSI [-45, -67, -82], latency [12ms, 45ms, 120ms]. Which node should be the relay?",
  "stream": false,
  "format": {
    "type": "object",
    "properties": {
      "best_relay": {"type": "integer"},
      "confidence": {"type": "number"},
      "reasoning": {"type": "string"}
    },
    "required": ["best_relay", "confidence", "reasoning"]
  }
}'
```

---

## 💻 GhostWire Rust Integration

### Add to `Cargo.toml`
```toml
[dependencies]
reqwest = { version = "0.12", features = ["json"] }
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tokio = { version = "1.0", features = ["full"] }
```

### New File: `src/core/ai/gemma_client.rs`
```rust
use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize)]
pub struct MeshAnalysisRequest {
    pub nodes: Vec<NodeTelemetry>,
    pub task: String,
}

#[derive(Debug, Serialize)]
pub struct NodeTelemetry {
    pub node_id: u32,
    pub rssi: f32,
    pub latency_ms: f32,
    pub packet_loss: f32,
    pub battery_level: f32,
}

#[derive(Debug, Deserialize)]
pub struct RoutingDecision {
    pub best_relay: u32,
    pub confidence: f32,
    pub reasoning: String,
}

pub struct GemmaClient {
    client: Client,
    base_url: String,
    model: String,
}

impl GemmaClient {
    pub fn new(base_url: &str, model: &str) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.to_string(),
            model: model.to_string(),
        }
    }

    pub async fn analyze_mesh(
        &self,
        request: MeshAnalysisRequest,
    ) -> Result<RoutingDecision, Box<dyn std::error::Error>> {
        let prompt = format!(
            "Analyze this mesh network and select the best relay node.\n\n\
             Nodes: {:?}\n\n\
             Task: {}\n\n\
             Return JSON with best_relay (node_id), confidence (0-1), and reasoning.",
            request.nodes, request.task
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
                        "best_relay": {"type": "integer"},
                        "confidence": {"type": "number"},
                        "reasoning": {"type": "string"}
                    },
                    "required": ["best_relay", "confidence", "reasoning"]
                }
            }))
            .send()
            .await?
            .json::<serde_json::Value>()
            .await?;

        let decision: RoutingDecision = serde_json::from_value(response["response"].clone())?;
        Ok(decision)
    }

    pub async fn detect_threats(
        &self,
        network_data: &str,
    ) -> Result<ThreatAnalysis, Box<dyn std::error::Error>> {
        let prompt = format!(
            "Analyze this network traffic for security threats.\n\n\
             Data: {}\n\n\
             Identify any anomalies, potential attacks, or suspicious patterns.",
            network_data
        );

        // Similar implementation with threat analysis schema
        todo!("Implement threat detection")
    }
}
```

---

## 📋 Implementation Checklist

- [ ] Install Ollama v0.20.0+
- [ ] Pull `gemma4:26b` (recommended) or `gemma4:e2b` (edge)
- [ ] Start Ollama with `OLLAMA_HOST=0.0.0.0:11434`
- [ ] Install LiteLLM proxy
- [ ] Create `litellm-config.yaml` with fallbacks
- [ ] Create `src/core/ai/gemma_client.rs`
- [ ] Wire Gemma client into routing classifier
- [ ] Test mesh analysis endpoint
- [ ] Benchmark latency vs accuracy
- [ ] Add thinking mode toggle (`"think": true`)
- [ ] Implement threat detection prompts
- [ ] Add offline fallback (use PRoPHET when Gemma unavailable)

---

## 🚀 Next Steps

1. **Check hardware:** Run `nvidia-smi` or `free -h` to determine which Gemma variant to use
2. **Install Ollama + pull model:** ~5 minutes
3. **Test local inference:** Verify Gemma 4 works
4. **Integrate with Rust client:** Wire into GhostWire core
5. **Benchmark:** Compare Gemma 4 routing decisions vs PRoPHET baseline

---

## ⚠️ Known Issues (April 4, 2026)

- **Ollama v0.20.0 MLX acceleration:** May have bugs on Apple Silicon (HN reports issues)
- **vLLM nightly required:** Need `--pre` flag for pip install
- **26B MoE on CPU:** Works but slow (~5-10 tokens/sec vs ~50 tokens/sec on GPU)
- **Function calling:** Still maturing, may need prompt engineering for reliable JSON output

---

*Gemma 4 is the game-changer we needed for offline-first AI routing in mesh networks.*
