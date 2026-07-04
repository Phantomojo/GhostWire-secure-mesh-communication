// LiteRT-LM Backend for GhostWire
// Google's on-device inference framework (evolved from TFLite)
//
// LiteRT-LM runs Gemma 4 and FunctionGemma on Android/iOS/Raspberry Pi
// without requiring Ollama. It uses the Gemma 4 MatFormer architecture
// with per-layer embeddings for efficient edge inference.
//
// Supported backends:
//   - LiteRT-LM: Google's on-device inference (this module)
//   - Ollama: HTTP-based local inference server (existing)
//   - Local: Direct ONNX/runtime inference (future)
//
// This module provides a unified inference interface that works
// with any backend, with LiteRT-LM as the native edge option.

use serde::{Deserialize, Serialize};
use std::path::Path;

use std::time::Duration;

// ============================================================
// Inference Backend Enum
// ============================================================

/// Available inference backends for Gemma 4 / FunctionGemma
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InferenceBackend {
    /// Ollama HTTP server (localhost:11434)
    Ollama,
    /// LiteRT-LM native on-device inference
    LiteRt,
    /// Local ONNX/runtime direct inference
    Local,
}

impl InferenceBackend {
    /// Default port/endpoint for this backend
    pub fn default_endpoint(&self) -> &'static str {
        match self {
            InferenceBackend::Ollama => "http://localhost:11434",
            InferenceBackend::LiteRt => "litert://local",
            InferenceBackend::Local => "local://runtime",
        }
    }

    /// Whether this backend requires network connectivity
    pub fn requires_network(&self) -> bool {
        match self {
            InferenceBackend::Ollama => true,
            InferenceBackend::LiteRt => false,
            InferenceBackend::Local => false,
        }
    }

    /// Human-readable name
    pub fn display_name(&self) -> &'static str {
        match self {
            InferenceBackend::Ollama => "Ollama",
            InferenceBackend::LiteRt => "LiteRT-LM",
            InferenceBackend::Local => "Local Runtime",
        }
    }
}

impl std::fmt::Display for InferenceBackend {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.display_name())
    }
}

// ============================================================
// Gemma 4 Model Variants
// ============================================================

/// Gemma 4 model variant with hardware requirements
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Gemma4Variant {
    /// E2B: 2.3B effective params, ~5GB RAM, phones + Raspberry Pi 5
    E2B,
    /// E4B: ~4B effective params, ~7.5GB RAM, phones + tablets
    E4B,
    /// 26B MoE: 4B active params, ~15GB RAM, laptops
    MoE26B,
    /// 31B Dense: flagship, ~30GB RAM, servers
    Dense31B,
}

impl Gemma4Variant {
    /// Ollama model tag for this variant
    pub fn ollama_tag(&self) -> &'static str {
        match self {
            Gemma4Variant::E2B => "gemma4:e2b",
            Gemma4Variant::E4B => "gemma4:e4b",
            Gemma4Variant::MoE26B => "gemma4:26b",
            Gemma4Variant::Dense31B => "gemma4:31b",
        }
    }

    /// LiteRT-LM model file name pattern
    pub fn litert_model_file(&self) -> &'static str {
        match self {
            Gemma4Variant::E2B => "gemma4_e2b_litert.tflite",
            Gemma4Variant::E4B => "gemma4_e4b_litert.tflite",
            Gemma4Variant::MoE26B => "gemma4_26b_moe_litert.tflite",
            Gemma4Variant::Dense31B => "gemma4_31b_dense_litert.tflite",
        }
    }

    /// Minimum RAM required in MB
    pub fn min_ram_mb(&self) -> u64 {
        match self {
            Gemma4Variant::E2B => 5120,
            Gemma4Variant::E4B => 7680,
            Gemma4Variant::MoE26B => 15360,
            Gemma4Variant::Dense31B => 30720,
        }
    }

    /// Context window size (tokens)
    pub fn context_window(&self) -> usize {
        // All Gemma 4 variants support 128K context
        131_072
    }

    /// Whether this variant supports multimodal input (text/image/audio)
    pub fn supports_multimodal(&self) -> bool {
        // E2B and E4B support multimodal; MoE and Dense are text-only
        matches!(self, Gemma4Variant::E2B | Gemma4Variant::E4B)
    }

    /// Whether this variant uses MatFormer architecture (per-layer embeddings)
    pub fn uses_matformer(&self) -> bool {
        // All Gemma 4 variants use MatFormer
        true
    }

    /// Select the best variant for available RAM
    pub fn select_for_ram(available_ram_mb: u64) -> Self {
        if available_ram_mb >= 30720 {
            Gemma4Variant::Dense31B
        } else if available_ram_mb >= 15360 {
            Gemma4Variant::MoE26B
        } else if available_ram_mb >= 7680 {
            Gemma4Variant::E4B
        } else {
            Gemma4Variant::E2B
        }
    }

    /// Parse from string (e.g., config value)
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "e2b" | "2b" | "2.3b" => Some(Gemma4Variant::E2B),
            "e4b" | "4b" => Some(Gemma4Variant::E4B),
            "26b" | "moe" | "moe26b" | "26b_moe" => Some(Gemma4Variant::MoE26B),
            "31b" | "dense" | "dense31b" | "31b_dense" => Some(Gemma4Variant::Dense31B),
            _ => None,
        }
    }
}

impl std::fmt::Display for Gemma4Variant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.ollama_tag())
    }
}

// ============================================================
// FunctionGemma Model
// ============================================================

/// FunctionGemma model configuration
pub struct FunctionGemmaModel {
    /// Model tag for Ollama
    pub ollama_tag: &'static str,
    /// LiteRT-LM model file
    pub litert_model_file: &'static str,
    /// Parameter count (270M)
    pub param_count: u64,
    /// Context window (32K)
    pub context_window: usize,
    /// Minimum RAM (512MB — runs on any phone)
    pub min_ram_mb: u64,
}

impl FunctionGemmaModel {
    pub fn config() -> Self {
        Self {
            ollama_tag: "functiongemma:270m",
            litert_model_file: "functiongemma_270m_litert.tflite",
            param_count: 270_000_000,
            context_window: 32_768,
            min_ram_mb: 512,
        }
    }
}

// ============================================================
// LiteRT-LM Inference Engine
// ============================================================

/// Configuration for LiteRT-LM backend
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LiteRtConfig {
    /// Path to the LiteRT-LM model file (.tflite)
    pub model_path: String,
    /// Number of threads for CPU inference (default: 4)
    pub num_threads: usize,
    /// Whether to use GPU delegate if available
    pub use_gpu_delegate: bool,
    /// GPU delegate library path (optional)
    pub gpu_delegate_path: Option<String>,
    /// Whether to use NNAPI delegate (Android only)
    pub use_nnapi_delegate: bool,
    /// Quantization: "full", "dynamic", "float16", or "none"
    pub quantization: String,
    /// Maximum tokens to generate
    pub max_tokens: usize,
    /// Temperature for sampling
    pub temperature: f32,
}

impl Default for LiteRtConfig {
    fn default() -> Self {
        Self {
            model_path: "models/gemma4_e2b_litert.tflite".to_string(),
            num_threads: 4,
            use_gpu_delegate: false,
            gpu_delegate_path: None,
            use_nnapi_delegate: false,
            quantization: "dynamic".to_string(),
            max_tokens: 4096,
            temperature: 0.1,
        }
    }
}

/// LiteRT-LM inference engine
///
/// This is a placeholder struct that defines the interface for LiteRT-LM.
/// The actual LiteRT-LM integration requires the `litert` crate or FFI bindings
/// to the LiteRT-LM C API. When ready, implement the `generate` method using
/// the LiteRT-LM interpreter.
///
/// Example integration steps:
/// 1. Add `litert-sys` or `tflite` crate to Cargo.toml
/// 2. Load the .tflite model file
/// 3. Create an interpreter with the model
/// 4. Set input tensor (tokenized prompt)
/// 5. Run inference
/// 6. Read output tensor (generated tokens)
/// 7. Decode tokens to text
pub struct LiteRtEngine {
    config: LiteRtConfig,
    /// Whether the engine has been initialized
    initialized: bool,
    /// Model metadata (loaded from file)
    model_info: Option<ModelMetadata>,
}

/// Metadata about a loaded LiteRT-LM model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    pub model_name: String,
    pub input_shape: Vec<i64>,
    pub output_shape: Vec<i64>,
    pub num_params: u64,
    pub quantized: bool,
}

impl LiteRtEngine {
    /// Create a new LiteRT-LM engine with the given config
    pub fn new(config: LiteRtConfig) -> Self {
        Self {
            config,
            initialized: false,
            model_info: None,
        }
    }

    /// Initialize the engine by loading the model file
    ///
    /// This is where the actual LiteRT-LM model loading happens.
    /// Currently returns an error since the LiteRT-LM crate is not yet integrated.
    pub fn initialize(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let model_path = Path::new(&self.config.model_path);

        if !model_path.exists() {
            return Err(format!(
                "LiteRT-LM model not found at: {}. Download the Gemma 4 LiteRT model first.",
                self.config.model_path
            )
            .into());
        }

        // TODO: Implement actual LiteRT-LM loading
        // Example with tflite crate:
        //   let model = tflite::FlatBufferModel::build_from_file(&self.config.model_path)?;
        //   let mut interpreter = tflite::Interpreter::from_model(model)?;
        //   interpreter.allocate_tensors()?;
        //   self.interpreter = Some(interpreter);

        tracing::info!(
            "LiteRT-LM engine configured (model: {})",
            self.config.model_path
        );

        // Mark as configured (not fully initialized until LiteRT crate is available)
        self.initialized = true;
        self.model_info = Some(ModelMetadata {
            model_name: self.config.model_path.clone(),
            input_shape: vec![1, 1], // [batch, seq_len] — dynamic
            output_shape: vec![1, 1],
            num_params: 0, // Would read from model
            quantized: self.config.quantization != "none",
        });

        Ok(())
    }

    /// Generate text from a prompt using LiteRT-LM
    ///
    /// Returns the generated text and token count.
    pub fn generate(
        &self,
        prompt: &str,
        max_tokens: Option<usize>,
    ) -> Result<GenerationResult, Box<dyn std::error::Error + Send + Sync>> {
        if !self.initialized {
            return Err("LiteRT-LM engine not initialized. Call initialize() first.".into());
        }

        // TODO: Implement actual LiteRT-LM inference
        // Example flow:
        //   1. Tokenize prompt (using SentencePiece or similar)
        //   2. Set input tensor with token IDs
        //   3. Run interpreter.invoke()
        //   4. Read output tensor (logits)
        //   5. Sample from logits (temperature-based)
        //   6. Decode tokens to text
        //   7. Loop until EOS or max_tokens

        let max_tok = max_tokens.unwrap_or(self.config.max_tokens);

        tracing::warn!(
            "LiteRT-LM generate called (prompt_len={}, max_tokens={}) — stub implementation",
            prompt.len(),
            max_tok
        );

        Ok(GenerationResult {
            text: String::new(),
            tokens_generated: 0,
            prompt_tokens: 0,
            latency_ms: 0,
        })
    }

    /// Generate with structured output (for FunctionGemma tool calling)
    ///
    /// Enforces JSON output format for function calling.
    pub fn generate_structured(
        &self,
        prompt: &str,
        json_schema: &serde_json::Value,
    ) -> Result<serde_json::Value, Box<dyn std::error::Error + Send + Sync>> {
        if !self.initialized {
            return Err("LiteRT-LM engine not initialized.".into());
        }

        // TODO: Implement structured generation with JSON schema enforcement
        // This would use grammar-constrained decoding to ensure valid JSON output
        // matching the provided schema.

        let _ = (prompt, json_schema);
        tracing::warn!("LiteRT-LM structured generate — stub implementation");

        Ok(serde_json::json!({}))
    }

    /// Check if the engine is ready
    pub fn is_ready(&self) -> bool {
        self.initialized
    }

    /// Get model metadata
    pub fn model_info(&self) -> Option<&ModelMetadata> {
        self.model_info.as_ref()
    }

    /// Get the config
    pub fn config(&self) -> &LiteRtConfig {
        &self.config
    }
}

// ============================================================
// Unified Inference Interface
// ============================================================

/// Result from a text generation call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GenerationResult {
    pub text: String,
    pub tokens_generated: usize,
    pub prompt_tokens: usize,
    pub latency_ms: u64,
}

/// Unified inference request
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceRequest {
    pub prompt: String,
    pub max_tokens: Option<usize>,
    pub temperature: Option<f32>,
    pub structured_output: Option<serde_json::Value>,
}

/// Unified inference response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InferenceResponse {
    pub text: String,
    pub backend: String,
    pub model: String,
    pub tokens_generated: usize,
    pub latency_ms: u64,
}

/// Unified inference engine trait
///
/// Implemented by both OllamaClient (HTTP) and LiteRtEngine (native).
pub trait InferenceEngine: Send + Sync {
    /// Generate text from a prompt
    fn generate(
        &self,
        request: InferenceRequest,
    ) -> impl std::future::Future<
        Output = Result<InferenceResponse, Box<dyn std::error::Error + Send + Sync>>,
    > + Send;

    /// Check if the engine is healthy/reachable
    fn health_check(&self) -> impl std::future::Future<Output = bool> + Send;

    /// Get the backend name
    fn backend_name(&self) -> &str;

    /// Get the model name
    fn model_name(&self) -> &str;
}

// ============================================================
// Model Version Detection
// ============================================================

/// Detect which Gemma model version is available locally
pub async fn detect_available_models(
    ollama_url: &str,
) -> Result<Vec<Gemma4Variant>, Box<dyn std::error::Error + Send + Sync>> {
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    let response = client
        .get(format!("{}/api/tags", ollama_url))
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;

    let mut available = Vec::new();

    if let Some(models) = response["models"].as_array() {
        for model in models {
            if let Some(name) = model["name"].as_str() {
                let name_lower = name.to_lowercase();
                if name_lower.starts_with("gemma4") || name_lower.starts_with("functiongemma") {
                    if let Some(variant) = parse_model_tag(name) {
                        available.push(variant);
                    }
                }
            }
        }
    }

    Ok(available)
}

/// Parse an Ollama model tag into a Gemma4Variant
fn parse_model_tag(tag: &str) -> Option<Gemma4Variant> {
    let tag_lower = tag.to_lowercase();

    if tag_lower.contains("functiongemma") {
        // FunctionGemma is separate, not a Gemma4Variant
        return None;
    }

    if tag_lower.contains("31b") || tag_lower.contains("dense") {
        Some(Gemma4Variant::Dense31B)
    } else if tag_lower.contains("26b") || tag_lower.contains("moe") {
        Some(Gemma4Variant::MoE26B)
    } else if tag_lower.contains("e4b") || tag_lower.contains(":4b") {
        Some(Gemma4Variant::E4B)
    } else if tag_lower.contains("e2b") || tag_lower.contains(":2b") {
        Some(Gemma4Variant::E2B)
    } else {
        None
    }
}

/// Check if a specific model is available in Ollama
pub async fn is_model_available(ollama_url: &str, variant: Gemma4Variant) -> bool {
    match detect_available_models(ollama_url).await {
        Ok(models) => models.contains(&variant),
        Err(_) => false,
    }
}

/// Select the best available model based on RAM
pub async fn select_best_model(
    ollama_url: &str,
    available_ram_mb: u64,
) -> Result<Gemma4Variant, Box<dyn std::error::Error + Send + Sync>> {
    let available = detect_available_models(ollama_url).await?;

    if available.is_empty() {
        // No Gemma 4 models available — recommend downloading
        let recommended = Gemma4Variant::select_for_ram(available_ram_mb);
        tracing::warn!(
            "No Gemma 4 models found. Run: ollama pull {}",
            recommended.ollama_tag()
        );
        return Ok(recommended);
    }

    // Filter by RAM and pick the best available
    let mut candidates: Vec<_> = available
        .into_iter()
        .filter(|v| v.min_ram_mb() <= available_ram_mb)
        .collect();

    candidates.sort_by_key(|v| std::cmp::Reverse(v.min_ram_mb()));

    if candidates.is_empty() {
        // Even the smallest model won't fit — use E2B anyway with warning
        tracing::warn!(
            "Insufficient RAM for any available Gemma 4 model. {} MB available, {} MB minimum.",
            available_ram_mb,
            Gemma4Variant::E2B.min_ram_mb()
        );
        return Ok(Gemma4Variant::E2B);
    }

    // Return the largest model that fits
    Ok(candidates[0])
}

// ============================================================
// Tests
// ============================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gemma4_variant_tags() {
        assert_eq!(Gemma4Variant::E2B.ollama_tag(), "gemma4:e2b");
        assert_eq!(Gemma4Variant::E4B.ollama_tag(), "gemma4:e4b");
        assert_eq!(Gemma4Variant::MoE26B.ollama_tag(), "gemma4:26b");
        assert_eq!(Gemma4Variant::Dense31B.ollama_tag(), "gemma4:31b");
    }

    #[test]
    fn test_gemma4_variant_ram_requirements() {
        assert_eq!(Gemma4Variant::E2B.min_ram_mb(), 5120);
        assert_eq!(Gemma4Variant::E4B.min_ram_mb(), 7680);
        assert_eq!(Gemma4Variant::MoE26B.min_ram_mb(), 15360);
        assert_eq!(Gemma4Variant::Dense31B.min_ram_mb(), 30720);
    }

    #[test]
    fn test_gemma4_variant_context_window() {
        // All Gemma 4 variants support 128K context
        assert_eq!(Gemma4Variant::E2B.context_window(), 131_072);
        assert_eq!(Gemma4Variant::E4B.context_window(), 131_072);
        assert_eq!(Gemma4Variant::MoE26B.context_window(), 131_072);
        assert_eq!(Gemma4Variant::Dense31B.context_window(), 131_072);
    }

    #[test]
    fn test_gemma4_variant_multimodal() {
        assert!(Gemma4Variant::E2B.supports_multimodal());
        assert!(Gemma4Variant::E4B.supports_multimodal());
        assert!(!Gemma4Variant::MoE26B.supports_multimodal());
        assert!(!Gemma4Variant::Dense31B.supports_multimodal());
    }

    #[test]
    fn test_gemma4_variant_matformer() {
        // All Gemma 4 variants use MatFormer architecture
        assert!(Gemma4Variant::E2B.uses_matformer());
        assert!(Gemma4Variant::E4B.uses_matformer());
        assert!(Gemma4Variant::MoE26B.uses_matformer());
        assert!(Gemma4Variant::Dense31B.uses_matformer());
    }

    #[test]
    fn test_select_for_ram() {
        assert_eq!(Gemma4Variant::select_for_ram(4096), Gemma4Variant::E2B);
        assert_eq!(Gemma4Variant::select_for_ram(6144), Gemma4Variant::E2B);
        assert_eq!(Gemma4Variant::select_for_ram(8192), Gemma4Variant::E4B);
        assert_eq!(Gemma4Variant::select_for_ram(16384), Gemma4Variant::MoE26B);
        assert_eq!(
            Gemma4Variant::select_for_ram(65536),
            Gemma4Variant::Dense31B
        );
    }

    #[test]
    fn test_parse_model_tag() {
        assert_eq!(parse_model_tag("gemma4:e2b"), Some(Gemma4Variant::E2B));
        assert_eq!(parse_model_tag("gemma4:e4b"), Some(Gemma4Variant::E4B));
        assert_eq!(parse_model_tag("gemma4:26b"), Some(Gemma4Variant::MoE26B));
        assert_eq!(parse_model_tag("gemma4:31b"), Some(Gemma4Variant::Dense31B));
        assert_eq!(parse_model_tag("functiongemma:270m"), None);
        assert_eq!(parse_model_tag("llama3.1:8b"), None);
    }

    #[test]
    fn test_function_gemma_model() {
        let config = FunctionGemmaModel::config();
        assert_eq!(config.ollama_tag, "functiongemma:270m");
        assert_eq!(config.param_count, 270_000_000);
        assert_eq!(config.context_window, 32_768);
        assert_eq!(config.min_ram_mb, 512);
    }

    #[test]
    fn test_inference_backend_defaults() {
        assert_eq!(
            InferenceBackend::Ollama.default_endpoint(),
            "http://localhost:11434"
        );
        assert!(InferenceBackend::Ollama.requires_network());
        assert!(!InferenceBackend::LiteRt.requires_network());
        assert!(!InferenceBackend::Local.requires_network());
    }

    #[test]
    fn test_litert_config_defaults() {
        let config = LiteRtConfig::default();
        assert_eq!(config.num_threads, 4);
        assert!(!config.use_gpu_delegate);
        assert_eq!(config.quantization, "dynamic");
        assert_eq!(config.max_tokens, 4096);
        assert!((config.temperature - 0.1).abs() < 0.001);
    }

    #[test]
    fn test_gemma4_variant_from_str() {
        assert_eq!(Gemma4Variant::from_str("e2b"), Some(Gemma4Variant::E2B));
        assert_eq!(Gemma4Variant::from_str("2.3b"), Some(Gemma4Variant::E2B));
        assert_eq!(Gemma4Variant::from_str("e4b"), Some(Gemma4Variant::E4B));
        assert_eq!(Gemma4Variant::from_str("26b"), Some(Gemma4Variant::MoE26B));
        assert_eq!(Gemma4Variant::from_str("moe"), Some(Gemma4Variant::MoE26B));
        assert_eq!(
            Gemma4Variant::from_str("31b"),
            Some(Gemma4Variant::Dense31B)
        );
        assert_eq!(
            Gemma4Variant::from_str("dense"),
            Some(Gemma4Variant::Dense31B)
        );
        assert_eq!(Gemma4Variant::from_str("unknown"), None);
    }
}
