# Vibe Transcription - Complete Setup Guide

**Last Updated:** March 2, 2026  
**Status:** ⚠️ Requires Display (GUI app)

---

## What is Vibe?

[Vibe](https://github.com/thewh1teagle/vibe) is a free, offline audio/video transcription desktop application that uses OpenAI Whisper. It provides a user-friendly GUI for transcribing media files.

**Features:**
- ✅ Completely offline (no internet needed after setup)
- ✅ Multiple Whisper models (tiny, base, small, medium, large)
- ✅ Supports 90+ languages
- ✅ Audio & video formats (mp3, mp4, wav, m4a, mkv, etc.)
- ✅ SRT, VTT, TXT output formats
- ✅ Word-level timestamps
- ✅ Speaker diarization (optional)

---

## Current Setup Status

| Component | Status | Location |
|-----------|--------|----------|
| **vibe binary** | ✅ Installed | `/usr/bin/vibe` |
| **GUI dependencies** | ✅ Installed | GTK3, WebKit2GTK |
| **whisper (CLI)** | ✅ Installed | `/root/.local/bin/whisper` |
| **Models directory** | ⏳ Setting up | `/home/ph/.local/share/vibe/models/` |

---

## ⚠️ Important: GUI Application

**vibe is a GUI-only application.** It requires an X11/Wayland display to run.

**You CANNOT run vibe:**
- In SSH without X11 forwarding
- In a headless terminal
- Without a desktop environment

**You CAN run vibe:**
- In a terminal within your desktop (GNOME, KDE, etc.)
- With X11 forwarding enabled
- In a VNC/RDP session

---

## Setup Instructions

### Step 1: Download Whisper Models

Open a terminal **in your desktop environment** and run:

```bash
# Run the setup script
cd /home/ph/GhostWire-secure-mesh-communication
./scripts/setup-vibe.sh
```

Or manually download models:

```bash
# Using Python
python3 -c "from huggingface_hub import snapshot_download; snapshot_download(repo_id='openai/whisper-base', local_dir='/home/ph/.local/share/vibe/models/whisper-base')"
```

### Step 2: Launch Vibe

```bash
# In a terminal with display access
vibe
```

**If you get "Authorization required" error:**
- Make sure you're in a desktop session
- Check that X11 is running: `echo $DISPLAY`
- Should output: `:0` or similar

### Step 3: Use Vibe

1. **Select Model** - Choose from dropdown (tiny, base, small, medium, large)
2. **Drag & Drop** - Drop audio/video files into the window
3. **Configure** - Select language, output format (TXT, SRT, VTT)
4. **Transcribe** - Click "Transcribe" button
5. **Save** - Output saved to same folder as source file

---

## Model Comparison

| Model | Size | Speed | Quality | Best For |
|-------|------|-------|---------|----------|
| **tiny** | 75 MB | Fastest | Basic | Quick tests, fast transcription |
| **base** | 142 MB | Fast | Good | **Recommended for most uses** |
| **small** | 466 MB | Medium | Better | Better accuracy, still fast |
| **medium** | 1.5 GB | Slow | Best | Professional transcripts |
| **large** | 2.9 GB | Slowest | Best | Maximum accuracy, patient users |

**Recommendation:** Start with `base`. Upgrade to `small` or `medium` if you need better accuracy.

---

## Alternative: Use Whisper CLI (No Display Required)

If you need transcription without a GUI, use the CLI version:

```bash
# Add whisper to PATH
export PATH="$PATH:/root/.local/bin"

# Transcribe a file
whisper "/home/ph/Downloads/audio.mp3" --model base --language en

# Output formats
whisper "audio.mp3" --output_format txt    # Plain text
whisper "audio.mp3" --output_format srt    # Subtitles
whisper "audio.mp3" --output_format vtt    # Web subtitles
whisper "audio.mp3" --output_format all    # All formats

# With word timestamps
whisper "audio.mp3" --word_timestamps True

# Different language
whisper "audio.mp3" --language es  # Spanish
```

**Output:** Text file saved next to source file

---

## Your Audio Files

Ready to transcribe in `/home/ph/Downloads/`:

| File | Size | Format |
|------|------|--------|
| `CERBERUS__The_Digital_Trap.mp4` | 28 MB | Video |
| `Civilization_s_Backup_Plan.mp4` | 55 MB | Video |
| `Closing_the_Cerberus_Uncanny_Valley.m4a` | 25 MB | Audio ✅ DONE |
| `Demystifying_C.mp4` | 33 MB | Video |
| `GhostWire_Builds_An_Unkillable_Mesh_Network.m4a` | 59 MB | Audio |
| `The_IoT_Honeypot_That_Morphs_Under_Attack.m4a` | 27 MB | Audio |

---

## Troubleshooting

### "Authorization required, but no authorization protocol specified"

**Cause:** vibe needs a display (X11/Wayland)

**Solutions:**
1. Run in a desktop environment terminal
2. Enable X11 forwarding in SSH: `ssh -X user@host`
3. Use whisper CLI instead (no display needed)

### "Failed to initialize GTK"

**Cause:** GTK3 libraries missing or display not available

**Solutions:**
```bash
# Install GTK3 (if missing)
sudo pacman -S gtk3 webkit2gtk

# Check display
echo $DISPLAY  # Should show :0 or similar

# Restart X11 session
```

### Models not downloading

**Solution:**
```bash
# Manual download
cd /home/ph/.local/share/vibe/models
python3 -c "from huggingface_hub import snapshot_download; snapshot_download(repo_id='openai/whisper-base', local_dir='whisper-base')"
```

### Transcription is slow

**Solutions:**
1. Use smaller model: `--model base` instead of `medium`
2. Close other applications
3. Ensure good CPU cooling

---

## Quick Commands

### Using Vibe GUI
```bash
# Open vibe (requires display)
vibe

# Then drag/drop files in the GUI
```

### Using Whisper CLI
```bash
# Quick transcription
whisper "/path/to/audio.mp3" --model base

# With specific output
whisper "audio.mp3" --model base --output_dir /output/folder --output_format srt

# Batch process
for f in *.mp3; do whisper "$f" --model base; done
```

### Using Transcribe Script
```bash
cd /home/ph/GhostWire-secure-mesh-communication
./scripts/transcribe.sh "/path/to/audio.mp3"
```

---

## Model Storage

Models are stored in: `/home/ph/.local/share/vibe/models/`

**Directory structure:**
```
models/
├── whisper-tiny/
│   ├── config.json
│   ├── model.safetensors
│   └── tokenizer.json
├── whisper-base/
│   └── ...
└── whisper-small/
    └── ...
```

**Disk space required:**
- tiny: ~150 MB
- base: ~300 MB
- small: ~1 GB
- medium: ~3 GB
- large: ~6 GB

---

## Next Steps

1. ✅ **Models downloading** - Wait for download to complete
2. ✅ **Launch vibe** - In desktop terminal: `vibe`
3. ✅ **Transcribe files** - Drag & drop your audio files
4. ✅ **Review transcripts** - Edit if needed
5. ✅ **Use for GhostWire** - Documentation, blog posts, etc.

---

## Support

- **Vibe GitHub:** https://github.com/thewh1teagle/vibe
- **Vibe Docs:** https://thewh1teagle.github.io/vibe/
- **Whisper CLI:** `whisper --help`

**Contact:** NGAIRURU — ngairuru@proton.me

---

**🎉 You're almost ready! Just need to download models and launch vibe.**
