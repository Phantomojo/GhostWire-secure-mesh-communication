#!/bin/bash
# GhostWire AI Setup - Quick Start Script
# Run this to continue AI integration from where we left off

set -e

echo "🤖 GhostWire AI Integration - Quick Setup"
echo "=========================================="
echo ""

# Check Ollama
echo "📦 Checking Ollama..."
if command -v ollama &> /dev/null; then
    OLLAMA_VERSION=$(ollama --version 2>&1 | grep -oP '\d+\.\d+\.\d+' || echo "unknown")
    echo "✅ Ollama installed: $OLLAMA_VERSION"
else
    echo "❌ Ollama not found. Installing..."
    curl -fsSL https://ollama.com/install.sh | sh
fi

# Check if Ollama is running
echo ""
echo "🔄 Checking Ollama service..."
if curl -s http://localhost:11434/api/tags &> /dev/null; then
    echo "✅ Ollama service is running"
else
    echo "⚠️ Starting Ollama service..."
    ollama serve &
    sleep 3
fi

# Check Gemma 4 models
echo ""
echo "🔍 Checking Gemma 4 models..."
if ollama list 2>/dev/null | grep -q "gemma4"; then
    echo "✅ Gemma 4 models installed:"
    ollama list | grep gemma4
else
    echo "⚠️ No Gemma 4 models found. Pulling recommended model..."
    echo "   This will download ~7.2 GB (gemma4:e2b)"
    ollama pull gemma4:e2b
fi

# Test Gemma 4
echo ""
echo "🧪 Testing Gemma 4..."
if curl -s http://localhost:11434/api/generate -d '{
  "model": "gemma4:e2b",
  "prompt": "Say hello to GhostWire AI in one sentence.",
  "stream": false
}' | grep -q "response"; then
    echo "✅ Gemma 4 is working!"
else
    echo "❌ Gemma 4 test failed. Check Ollama logs."
fi

# Check LiteLLM
echo ""
echo "📦 Checking LiteLLM..."
if command -v litellm &> /dev/null; then
    echo "✅ LiteLLM installed: $(litellm --version 2>&1 || echo 'unknown')"
else
    echo "⚠️ LiteLLM not installed. Install with:"
    echo "   pip install litellm[proxy]"
fi

# Check Rust compilation
echo ""
echo "🔨 Checking GhostWire AI module..."
cd "$(dirname "$0")"
if cargo check 2>&1 | grep -q "ai/"; then
    echo "⚠️ AI module has compilation warnings (check logs)"
else
    echo "✅ AI module compiles cleanly"
fi

echo ""
echo "=========================================="
echo "🎯 Next Steps:"
echo "1. Start Ollama: OLLAMA_HOST=0.0.0.0:11434 ollama serve"
echo "2. Start LiteLLM: litellm --config litellm-config.yaml"
echo "3. Test AI: cargo test ai::gemma_client"
echo "4. Run GhostWire: cargo run"
echo ""
echo "📚 Documentation:"
echo "   - AI_INTEGRATION_STATUS.md (current status)"
echo "   - GEMMA4_INTEGRATION.md (Gemma 4 guide)"
echo "   - QWEN_HANDOFF.md (previous handoff)"
echo ""
