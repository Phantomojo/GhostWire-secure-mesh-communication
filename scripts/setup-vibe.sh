#!/bin/bash
# Vibe Transcription Setup Script
# This sets up vibe for offline transcription

set -e

echo "🎤 Setting up Vibe Transcription..."

# Create directories
MODEL_DIR="/home/ph/.local/share/vibe/models"
mkdir -p "$MODEL_DIR"

# Download whisper models
echo "📥 Downloading Whisper models..."

download_model() {
    local model=$1
    local model_dir="$MODEL_DIR/$model"
    
    if [ -d "$model_dir" ] && [ "$(ls -A $model_dir)" ]; then
        echo "✅ $model already downloaded"
    else
        echo "⏳ Downloading $model..."
        python3 -c "from huggingface_hub import snapshot_download; snapshot_download(repo_id='openai/$model', local_dir='$model_dir')"
        echo "✅ $model downloaded"
    fi
}

# Download common models
download_model "whisper-tiny"
download_model "whisper-base"
download_model "whisper-small"

echo ""
echo "✅ Vibe setup complete!"
echo ""
echo "To use vibe:"
echo "1. Run: vibe"
echo "2. Select model from dropdown (tiny, base, small)"
echo "3. Drag & drop audio/video files"
echo "4. Transcription will be saved to same folder as source file"
echo ""
echo "Models location: $MODEL_DIR"
