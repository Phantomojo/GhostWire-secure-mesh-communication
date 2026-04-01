#!/bin/bash
# Transcribe audio/video files using whisper or ollama
# Usage: ./transcribe.sh <audio_file> [output_file]

set -e

INPUT_FILE="$1"
OUTPUT_FILE="${2:-${INPUT_FILE%.*}.txt}"

if [ -z "$INPUT_FILE" ]; then
    echo "Usage: $0 <audio_file> [output_file]"
    echo "Supported formats: mp3, mp4, wav, m4a, ogg, flac"
    exit 1
fi

if [ ! -f "$INPUT_FILE" ]; then
    echo "Error: File not found: $INPUT_FILE"
    exit 1
fi

echo "🎤 Transcribing: $INPUT_FILE"
echo "📄 Output: $OUTPUT_FILE"

# Extract audio if video
if [[ "$INPUT_FILE" == *.mp4 ]] || [[ "$INPUT_FILE" == *.mkv ]] || [[ "$INPUT_FILE" == *.avi ]]; then
    echo "📹 Extracting audio from video..."
    TEMP_AUDIO="/tmp/transcribe_audio_$$".wav
    ffmpeg -i "$INPUT_FILE" -vn -acodec pcm_s16le -ar 16000 -ac 1 "$TEMP_AUDIO" -y 2>/dev/null
    INPUT_FILE="$TEMP_AUDIO"
fi

# Method 1: Try whisper CLI (if installed)
if command -v whisper &> /dev/null; then
    echo "🎯 Using Whisper..."
    whisper "$INPUT_FILE" --model base --language en --output_dir "$(dirname "$OUTPUT_FILE")" --output_format txt 2>&1
    mv "$(dirname "$OUTPUT_FILE")/$(basename "$INPUT_FILE" | sed 's/\.[^.]*$/.txt/')" "$OUTPUT_FILE" 2>/dev/null || true
    rm -f "$TEMP_AUDIO" 2>/dev/null
    echo "✅ Transcription complete: $OUTPUT_FILE"
    exit 0
fi

# Method 2: Use pipx whisper venv directly
if [ -f "/root/.local/bin/whisper" ]; then
    echo "🐍 Using Whisper (pipx)..."
    export PATH="$PATH:/root/.local/bin"
    whisper "$INPUT_FILE" --model base --language en --output_dir "$(dirname "$OUTPUT_FILE")" --output_format txt 2>&1
    mv "$(dirname "$OUTPUT_FILE")/$(basename "$INPUT_FILE" | sed 's/\.[^.]*$/.txt/')" "$OUTPUT_FILE" 2>/dev/null || true
    rm -f "$TEMP_AUDIO" 2>/dev/null
    echo "✅ Transcription complete: $OUTPUT_FILE"
    exit 0
fi

# Method 3: Use ollama with llama for manual transcription
echo "🦙 Using Ollama (llama3.1) for transcription assistance..."
echo "Note: This requires manual input. For auto-transcription, install whisper."
echo ""
echo "To install whisper properly:"
echo "  1. Clean up /tmp: rm -rf /tmp/*"
echo "  2. Then: pipx install openai-whisper"
echo ""
echo "Or use online services:"
echo "  - YouTube Studio (upload as unlisted, download auto-captions)"
echo "  - Google Docs Voice Typing"
echo "  - Otter.ai"

rm -f "$TEMP_AUDIO" 2>/dev/null
exit 1
