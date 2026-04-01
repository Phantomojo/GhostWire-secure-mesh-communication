# 📱 GhostWire Instagram Content System

> Convert animated HTML cards to Instagram-ready video formats

---

## 🎯 What This Does

This system takes your **animated HTML/CSS cards** and converts them into **upload-ready Instagram formats**:

- ✅ **Reels** (1080×1920, 9:16, up to 90s)
- ✅ **Stories** (1080×1920, 9:16, up to 60s)
- ✅ **Feed Posts** (1080×1080, 1:1 or 1080×1350, 4:5)
- ✅ **Carousels** (up to 10 slides, 1080×1080)

---

## 📁 File Structure

```
social-media/instagram/
├── templates/
│   ├── instagram_reel_launch.html    # Animated launch card (Reel format)
│   ├── instagram_story_manifesto.html # Manifesto quote (Story format)
│   └── instagram_feed_stack.html      # Tech stack (Feed format)
├── content/
│   ├── reel/                          # Rendered Reels videos
│   ├── story/                         # Rendered Stories videos
│   ├── feed/                          # Rendered Feed posts
│   ├── carousel/                      # Carousel slide images
│   ├── captions/                      # Pre-written captions
│   └── thumbnails/                    # Video thumbnails
├── instagram_content_generator.py     # Main rendering script
├── INSTAGRAM_CONTENT_STRATEGY.md      # Full content strategy
└── README.md                          # This file
```

---

## 🚀 Quick Start

### 1. Install Dependencies

```bash
# System packages
sudo pacman -S ffmpeg nodejs npm

# Python packages
pip install playwright

# Install browser for Playwright
playwright install chromium
```

### 2. Render Your First Card

```bash
cd /home/ph/GhostWire-secure-mesh-communication/social-media/instagram

# Render all cards in all formats
python instagram_content_generator.py --card all --format all

# Render specific card
python instagram_content_generator.py --card launch --format reel

# Render for Stories only
python instagram_content_generator.py --card manifesto --format story
```

### 3. Find Your Content

```bash
# Open output directory
ls -la content/reel/
ls -la content/story/
ls -la content/feed/

# View captions
cat content/captions/launch_reel_caption.txt
```

---

## 📋 Available Cards

| Card | File | Type | Duration | Description |
|------|------|------|----------|-------------|
| **launch** | ghostwire_launch_card.html | Animated | 15s | Main launch with mesh animation |
| **manifesto** | ghostwire_card_2_manifesto.html | Static | 5s | Quote from manifesto |
| **technical** | ghostwire_card_3_technical.html | Animated | 12s | 6-layer protocol stack |
| **pfp** | ghostwire_pfp_options.html | Static | 5s | Profile picture options |

---

## 🎬 Rendering Process

### Step 1: HTML → Screenshot/Video

The script uses **Playwright** (headless Chromium) to:
1. Load the HTML card
2. Wait for animations to initialize
3. Capture screenshot(s) or record video

### Step 2: Resize & Encode

**FFmpeg** handles:
- Resizing to Instagram specs
- H.264 encoding (MP4)
- Frame rate optimization (30fps)
- File size compression

### Step 3: Generate Captions

Pre-written captions are saved with:
- Hook (first line grabs attention)
- Body (main message)
- CTA (call-to-action)
- Hashtags (relevant tags)

---

## 📐 Instagram Specifications 2026

### Reels
```
Resolution: 1080 × 1920 px
Aspect Ratio: 9:16 (vertical)
Duration: 3-90 seconds
Format: MP4 (H.264, AAC)
Frame Rate: 30fps
File Size: <100MB recommended
```

### Stories
```
Resolution: 1080 × 1920 px
Aspect Ratio: 9:16 (vertical)
Duration: Up to 60 seconds
Format: MP4 or PNG
Safe Zone: Keep text away from top 250px and bottom 250px (UI elements)
```

### Feed Posts
```
Resolution: 1080 × 1080 px (square) or 1080 × 1350 px (portrait)
Aspect Ratio: 1:1 or 4:5
Duration: Up to 60 seconds (video)
Format: MP4 or PNG
```

### Carousels
```
Slides: 2-10 images/videos
Resolution: 1080 × 1080 px
Format: PNG or MP4
```

---

## 🎨 Creating New Cards

### Template Structure

```html
<!DOCTYPE html>
<html>
<head>
  <style>
    /* Instagram-safe dimensions */
    .instagram-card {
      width: 1080px;
      height: 1920px; /* For Reels/Stories */
      /* or 1080px × 1080px for Feed */
    }
    
    /* Safe zone for UI elements */
    .safe-zone {
      top: 200px;
      bottom: 300px;
      left: 60px;
      right: 60px;
    }
  </style>
</head>
<body>
  <!-- Your content here -->
</body>
</html>
```

### Best Practices

1. **Use Safe Zones** - Keep important text away from edges
2. **High Contrast** - Ensure readability on mobile screens
3. **Large Text** - Minimum 18px for body, 36px+ for headlines
4. **Short Duration** - 5-15 seconds for static, 15-30s for animated
5. **Vertical Format** - 9:16 performs best on Instagram

---

## 📝 Caption Writing Guide

### Structure

```
[HOOK - Grabs attention in 1st line]

[BODY - Main message, 2-5 lines]

[CTA - Call-to-action]

[HASHTAGS - 3-10 relevant tags]

---
[Branding line]
```

### Example

```
🌐 The Internet Was Supposed To Be Ungovernable

GhostWire is here.

Sovereign mesh communication.
No servers. No surveillance. No permission.

🔗 Link in bio to join the mesh

#GhostWire #MeshNetwork #Decentralized #Privacy #Rust

---
GhostWire | Sovereign mesh communication
```

### Hashtag Strategy

**Mix of sizes:**
- 2-3 large (1M+ posts): #Privacy #Tech
- 3-5 medium (100k-1M): #Rust #OpenSource
- 2-3 niche (10-100k): #MeshNetwork #Libp2p

---

## 📊 Content Calendar

See `INSTAGRAM_CONTENT_STRATEGY.md` for:
- 4-week launch calendar
- Daily posting schedule
- Content pillars
- Engagement strategy
- KPI tracking

---

## 🛠️ Advanced Usage

### Custom Rendering

```python
# Render at custom resolution
generate_card_screenshot(
    "launch",
    output_path="custom.png",
    width=1080,
    height=1350  # 4:5 portrait for feed
)

# Create video from images
create_video_from_images(
    images=["frame1.png", "frame2.png", "frame3.png"],
    output_path="output.mp4",
    duration=3,  # seconds per image
    fps=30
)
```

### FFmpeg Commands

```bash
# Convert PNG sequence to video
ffmpeg -framerate 30 -i frame_%03d.png -c:v libx264 -pix_fmt yuv420p output.mp4

# Resize video
ffmpeg -i input.mp4 -vf "scale=1080:1920" output_1080p.mp4

# Compress video
ffmpeg -i input.mp4 -c:v libx264 -crf 23 -c:a aac -b:a 128k compressed.mp4

# Add audio track
ffmpeg -i video.mp4 -i audio.mp3 -c:v copy -c:a aac -shortest output.mp4
```

---

## 🐛 Troubleshooting

### "Playwright not found"
```bash
pip install playwright
playwright install chromium
```

### "FFmpeg not found"
```bash
sudo pacman -S ffmpeg
```

### "HTML file not found"
Check that CARDS_DIR points to correct location:
```python
CARDS_DIR = Path("/home/ph/Downloads/xgh")
```

### Video quality issues
Increase bitrate in FFmpeg:
```bash
ffmpeg -i input.mp4 -b:v 5M output_high_quality.mp4
```

---

## 📈 Analytics & Tracking

### UTM Parameters

Add to your link-in-bio URL:
```
github.com/Phantomojo/GhostWire?utm_source=instagram&utm_medium=reel&utm_campaign=launch
```

### Track Metrics

- **Reach** - Unique accounts who saw content
- **Engagement** - Likes, comments, shares, saves
- **Profile Visits** - Clicks to your profile
- **Link Clicks** - Clicks to GitHub (via link-in-bio)
- **Followers** - New followers gained

### Tools

- **Instagram Insights** - Native analytics
- **Meta Business Suite** - Cross-platform tracking
- **Bitly** - Link tracking with custom domains

---

## 🎯 Launch Checklist

### Pre-Launch (Week Before)
- [ ] 10+ posts rendered and ready
- [ ] Captions written for all posts
- [ ] Hashtag research completed
- [ ] Link-in-bio tool set up (Linktree/Carrd)
- [ ] Instagram Business account created
- [ ] Content calendar populated

### Launch Day
- [ ] Main launch Reel posted at 11 AM
- [ ] Shared to Stories immediately
- [ ] Respond to all comments in first hour
- [ ] Post 2-3 Stories throughout day
- [ ] Go live for Q&A (optional)
- [ ] Thank you story at end of day

### Post-Launch
- [ ] Monitor GitHub stars/issues
- [ ] Engage with community DMs
- [ ] Track analytics weekly
- [ ] Adjust content based on performance
- [ ] Plan Week 2 content

---

## 📞 Support

**Issues?** Create an issue in the GhostWire repo.

**Questions?** DM @phantomojo on Instagram.

---

**Last Updated:** March 28, 2026  
**Maintained By:** GhostWire Team  
**Status:** ✅ Ready for launch
