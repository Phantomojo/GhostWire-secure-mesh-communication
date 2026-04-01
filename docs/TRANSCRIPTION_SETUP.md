# Audio/Video Transcription Setup Guide

**Last Updated:** March 2, 2026  
**Status:** ✅ FULLY READY

---

## Current Setup

| Component | Status | Notes |
|-----------|--------|-------|
| **vibe** | ⚠️ Installed (GUI only) | Needs X11/Wayland display |
| **ollama** | ✅ Running | PID 1136175 |
| **ffmpeg** | ✅ Installed | For audio extraction |
| **whisper** | ✅ INSTALLED | pipx, base model ready |
| **transcribe.sh** | ✅ Created | `/home/ph/GhostWire-secure-mesh-communication/scripts/` |

---

## Quick Start

### Option 1: Use the Script (Recommended)

```bash
cd /home/ph/GhostWire-secure-mesh-communication

# Transcribe a file
./scripts/transcribe.sh "/home/ph/Downloads/your_audio.mp3"

# Output will be: /home/ph/Downloads/your_audio.txt
```

### Option 2: Use whisper directly

```bash
# Add whisper to PATH
export PATH="$PATH:/root/.local/bin"

# Transcribe a file
whisper "/home/ph/Downloads/audio.mp3" --model base --language en

# Output: /home/ph/Downloads/audio.txt
```

### Option 3: Use vibe GUI (if you have display)

```bash
# In a terminal with X11/Wayland access
vibe

# Then drag/drop your audio file
```

---

## Supported Files

| Format | Extension | Notes |
|--------|-----------|-------|
| Audio | `.mp3`, `.wav`, `.m4a`, `.ogg`, `.flac` | Direct support |
| Video | `.mp4`, `.mkv`, `.avi` | Audio extracted automatically |

---

## Your Audio Files Ready for Transcription

Found in `/home/ph/Downloads/`:

1. `CERBERUS__The_Digital_Trap.mp4` (28 MB)
2. `Civilization_s_Backup_Plan.mp4` (55 MB)
3. `Closing_the_Cerberus_Uncanny_Valley.m4a` (25 MB)
4. `Demystifying_C.mp4` (33 MB)
5. `GhostWire_Builds_An_Unkillable_Mesh_Network.m4a` (59 MB)
6. `The_IoT_Honeypot_That_Morphs_Under_Attack.m4a` (27 MB)

---

## Setup Steps (If Not Complete)

### 1. Wait for whisper model download

```bash
# Check if download is complete
ollama list | grep whisper
```

Expected output:
```
whisper-large-v3    xxxxx    3.1 GB    Recently
```

### 2. Test transcription

```bash
./scripts/transcribe.sh "/home/ph/Downloads/Closing_the_Cerberus_Uncanny_Valley.m4a"
```

### 3. Check output

```bash
cat "/home/ph/Downloads/Closing_the_Cerberus_Uncanny_Valley.txt"
```

---

## Troubleshooting

### Problem: vibe won't start

**Error:** `Authorization required, but no authorization protocol specified`

**Solution:** vibe is a GUI app. Use the script instead:
```bash
./scripts/transcribe.sh <file>
```

Or run vibe in a proper desktop environment.

### Problem: ollama whisper model not found

**Solution:** Download it:
```bash
ollama pull whisper-large-v3
```

### Problem: Transcription is slow

**Solution:** Use a smaller model:
```bash
# Edit transcribe.sh, change:
ollama run whisper-large-v3
# To:
ollama run whisper-base
```

### Problem: Poor transcription quality

**Solutions:**
1. Use `whisper-large-v3` (best quality, slower)
2. Ensure audio is clear (reduce background noise)
3. Specify language if not English: `--language es` for Spanish

---

## Alternative: Install whisper directly

If ollama whisper doesn't work well:

```bash
# Using pipx (recommended)
pipx install openai-whisper

# Or with pip
pip install --break-system-packages openai-whisper

# Then use directly
whisper "/path/to/audio.mp3" --model base --language en
```

---

## Batch Transcription

To transcribe all audio files in Downloads:

```bash
cd /home/ph/Downloads
for file in *.mp3 *.m4a *.wav; do
    [ -f "$file" ] || continue
    echo "Transcribing: $file"
    /home/ph/GhostWire-secure-mesh-communication/scripts/transcribe.sh "$file"
done
```

---

## Contact

Issues? Contact: NGAIRURU — ngairuru@proton.me
