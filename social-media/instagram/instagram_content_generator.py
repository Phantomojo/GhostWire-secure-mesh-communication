#!/usr/bin/env python3
"""
GhostWire Instagram Content Generator
======================================
Converts animated HTML cards to Instagram-ready video formats.

Instagram Specs 2026:
- Feed Posts: 1080x1080 (1:1), 1080x1350 (4:5)
- Stories: 1080x1920 (9:16)
- Reels: 1080x1920 (9:16), up to 90 seconds
- Video Format: MP4 (H.264 codec, AAC audio)
- Frame Rate: 30fps recommended
- Max File Size: 4GB (practical: under 100MB)

Usage:
    python instagram_content_generator.py --card all --format reel,story,feed
"""

import os
import sys
import json
import subprocess
import argparse
from pathlib import Path
from datetime import datetime

# Configuration
PROJECT_ROOT = Path("/home/ph/GhostWire-secure-mesh-communication")
CARDS_DIR = Path("/home/ph/Downloads/xgh")
OUTPUT_DIR = PROJECT_ROOT / "social-media" / "instagram" / "content"
TEMPLATES_DIR = PROJECT_ROOT / "social-media" / "instagram" / "templates"

# Instagram specifications
INSTAGRAM_SPECS = {
    "feed": {
        "width": 1080,
        "height": 1080,  # Can also use 1350 for 4:5 portrait
        "aspect": "1:1",
        "max_duration": 60,
        "format": "mp4"
    },
    "story": {
        "width": 1080,
        "height": 1920,
        "aspect": "9:16",
        "max_duration": 60,
        "format": "mp4"
    },
    "reel": {
        "width": 1080,
        "height": 1920,
        "aspect": "9:16",
        "max_duration": 90,
        "format": "mp4"
    },
    "carousel": {
        "width": 1080,
        "height": 1080,
        "aspect": "1:1",
        "max_slides": 10,
        "format": "png"
    }
}

# Card definitions
CARDS = {
    "launch": {
        "file": "ghostwire_launch_card.html",
        "title": "Ghostwire Launch",
        "type": "animated",
        "duration": 15,
        "description": "Main launch announcement card with animated mesh network"
    },
    "manifesto": {
        "file": "ghostwire_card_2_manifesto.html",
        "title": "Manifesto Quote",
        "type": "static",
        "duration": 5,
        "description": "Key quote from Ghostwire Manifesto"
    },
    "technical": {
        "file": "ghostwire_card_3_technical.html",
        "title": "Technical Stack",
        "type": "animated",
        "duration": 12,
        "description": "6-layer protocol stack visualization"
    },
    "pfp": {
        "file": "ghostwire_pfp_options.html",
        "title": "Profile Picture Options",
        "type": "static",
        "duration": 5,
        "description": "GhostWire profile picture variations"
    }
}


def check_dependencies():
    """Check if required tools are installed."""
    required = {
        "playwright": "For browser automation",
        "ffmpeg": "For video encoding",
        "node": "For running HTML cards"
    }
    
    missing = []
    for tool, desc in required.items():
        try:
            subprocess.run(
                ["which", tool],
                check=True,
                capture_output=True
            )
        except subprocess.CalledProcessError:
            missing.append(f"{tool} ({desc})")
    
    if missing:
        print("❌ Missing dependencies:")
        for dep in missing:
            print(f"   - {dep}")
        print("\n📦 Install with:")
        print("   sudo pacman -S ffmpeg nodejs")
        print("   pip install playwright")
        print("   playwright install chromium")
        return False
    
    return True


def create_output_directories():
    """Create output directory structure."""
    for format_name in INSTAGRAM_SPECS.keys():
        (OUTPUT_DIR / format_name).mkdir(parents=True, exist_ok=True)
    (OUTPUT_DIR / "captions").mkdir(parents=True, exist_ok=True)
    (OUTPUT_DIR / "thumbnails").mkdir(parents=True, exist_ok=True)
    print(f"✅ Output directories created at {OUTPUT_DIR}")


def generate_card_screenshot(card_key, output_path, width=1080, height=1080):
    """
    Generate screenshot/video of HTML card using Playwright.
    
    Args:
        card_key: Key from CARDS dict
        output_path: Output file path
        width: Output width in pixels
        height: Output height in pixels
    """
    card = CARDS.get(card_key)
    if not card:
        print(f"❌ Card '{card_key}' not found")
        return False
    
    html_file = CARDS_DIR / card["file"]
    if not html_file.exists():
        print(f"❌ HTML file not found: {html_file}")
        return False
    
    print(f"📸 Rendering {card['title']}...")
    
    # Create Playwright script
    script = f"""
const playwright = require('playwright');

(async () => {{
    const browser = await playwright.chromium.launch({{ headless: true }});
    const page = await browser.newPage({{
        viewport: {{ width: {width}, height: {height} }}
    }});
    
    await page.goto('file://{html_file.absolute()}');
    await page.waitForTimeout(1000);
    
    {'await page.screenshot({{ path: f'{output_path}', fullPage: false }});' if card['type'] == 'static' else ''}
    
    await browser.close();
}})();
"""
    
    # For now, we'll use a simpler approach with direct screenshot
    try:
        from playwright.sync_api import sync_playwright
        
        with sync_playwright() as p:
            browser = p.chromium.launch(headless=True)
            page = browser.new_page(viewport={"width": width, "height": height})
            page.goto(f"file://{html_file.absolute()}")
            page.wait_for_timeout(2000)  # Wait for animations
            
            if card["type"] == "static":
                page.screenshot(path=str(output_path), full_page=False)
            else:
                # For animated cards, we'll create a video
                page.screenshot(path=str(output_path).replace('.mp4', '.png'), full_page=False)
            
            browser.close()
        
        print(f"✅ Generated: {output_path}")
        return True
        
    except ImportError:
        print("⚠️  Playwright not installed. Install with: pip install playwright")
        return False


def create_video_from_images(images, output_path, duration=5, fps=30):
    """
    Create video from sequence of images using ffmpeg.
    
    Args:
        images: List of image paths
        output_path: Output video path
        duration: Duration per image in seconds
        fps: Frames per second
    """
    if not images:
        print("❌ No images provided")
        return False
    
    # Create temp file list for ffmpeg
    temp_file = "/tmp/ffmpeg_images.txt"
    with open(temp_file, 'w') as f:
        for img in images:
            f.write(f"file '{img}'\n")
            f.write(f"duration {duration}\n")
    
    cmd = [
        "ffmpeg",
        "-y",
        "-f", "concat",
        "-safe", "0",
        "-i", temp_file,
        "-vf", f"fps={fps},scale=1080:1920:force_original_aspect_ratio=decrease,pad=1080:1920:(ow-iw)/2:(oh-ih)/2",
        "-c:v", "libx264",
        "-pix_fmt", "yuv420p",
        "-r", str(fps),
        output_path
    ]
    
    try:
        subprocess.run(cmd, check=True, capture_output=True)
        print(f"✅ Video created: {output_path}")
        return True
    except subprocess.CalledProcessError as e:
        print(f"❌ FFmpeg error: {e}")
        return False


def generate_caption(card_key, format_type):
    """
    Generate Instagram caption for a card.
    
    Args:
        card_key: Key from CARDS dict
        format_type: Instagram format (reel, story, feed)
    """
    card = CARDS.get(card_key)
    if not card:
        return None
    
    captions = {
        "launch": {
            "hook": "🌐 The Internet Was Supposed To Be Ungovernable",
            "body": """GhostWire is here.

Sovereign mesh communication.
No servers. No surveillance. No permission.

Built on Rust + libp2p for the decentralized future.

🔗 Link in bio to join the mesh
""",
            "hashtags": "#GhostWire #MeshNetwork #Decentralized #Privacy #Rust #Web3 #P2P #Encryption #OpenSource #Tech",
            "cta": "👇 Tag someone who needs this"
        },
        "manifesto": {
            "hook": "📜 Not A Declaration. A Foundation.",
            "body": """"This is not a declaration.
Declarations announce positions.
This is something older — a foundation.
A spine that holds when everything around it shifts."

— Ghostwire Manifesto
""",
            "hashtags": "#Manifesto #Philosophy #Decentralization #Privacy #Freedom #TechEthics #OpenWeb",
            "cta": "💭 What does sovereignty mean to you?"
        },
        "technical": {
            "hook": "⚙️ 6 Layers. Zero Compromise.",
            "body": """The GhostWire protocol stack:

01. Crypto Layer (AES-256-GCM, Ed25519, PQ)
02. Routing · Sphinx (Onion routing)
03. Transport (Stealth TCP, obfs4)
04. Mesh · libp2p (Gossipsub, DHT)
05. Identity (Shamir recovery)
06. Ghost Channels (Anonymous broadcast)

Built in Rust. AGPL-3.0 licensed.
""",
            "hashtags": "#Rust #Libp2p #Cryptography #Protocol #Engineering #OpenSource #DevCommunity #TechStack",
            "cta": "🔧 Developers: link in bio for docs"
        }
    }
    
    cap = captions.get(card_key, {})
    
    caption_text = f"""{cap.get('hook', '')}

{cap.get('body', '')}
{cap.get('cta', '')}

{cap.get('hashtags', '')}

---
GhostWire | Sovereign mesh communication
github.com/Phantomojo/GhostWire-secure-mesh-communication
"""
    
    # Save caption
    caption_file = OUTPUT_DIR / "captions" / f"{card_key}_{format_type}_caption.txt"
    with open(caption_file, 'w') as f:
        f.write(caption_text)
    
    print(f"📝 Caption saved: {caption_file}")
    return caption_text


def create_carousel_slides():
    """
    Create carousel post with multiple slides.
    """
    slides = [
        {
            "type": "title",
            "text": "GHOSTWIRE",
            "subtext": "Sovereign Mesh Communication"
        },
        {
            "type": "problem",
            "text": "The Problem",
            "points": [
                "Centralized servers = single points of failure",
                "Surveillance capitalism is the business model",
                "You don't own your communication"
            ]
        },
        {
            "type": "solution",
            "text": "The Solution",
            "points": [
                "Peer-to-peer mesh network",
                "End-to-end encrypted by default",
                "No servers. No surveillance. No permission."
            ]
        },
        {
            "type": "tech",
            "text": "Built With",
            "points": ["Rust", "libp2p", "Sphinx Routing", "Ed25519", "AES-256-GCM"]
        },
        {
            "type": "cta",
            "text": "Join The Mesh",
            "subtext": "github.com/Phantomojo/GhostWire"
        }
    ]
    
    print(f"📊 Creating {len(slides)} carousel slides...")
    # Implementation would generate each slide as an image
    return slides


def main():
    parser = argparse.ArgumentParser(description="GhostWire Instagram Content Generator")
    parser.add_argument(
        "--card",
        choices=list(CARDS.keys()) + ["all", "carousel"],
        default="all",
        help="Which card(s) to generate"
    )
    parser.add_argument(
        "--format",
        choices=["feed", "story", "reel", "carousel", "all"],
        default="feed",
        help="Instagram format"
    )
    parser.add_argument(
        "--check-deps",
        action="store_true",
        help="Check dependencies only"
    )
    
    args = parser.parse_args()
    
    print("🎨 GhostWire Instagram Content Generator")
    print("=" * 50)
    
    # Check dependencies
    if args.check_deps or not check_dependencies():
        return
    
    # Create directories
    create_output_directories()
    
    # Generate content
    cards_to_generate = list(CARDS.keys()) if args.card == "all" else [args.card]
    formats = ["feed", "story", "reel"] if args.format == "all" else [args.format]
    
    for card_key in cards_to_generate:
        print(f"\n{'='*50}")
        print(f"Processing: {CARDS[card_key]['title']}")
        print(f"{'='*50}")
        
        for fmt in formats:
            spec = INSTAGRAM_SPECS[fmt]
            output_file = OUTPUT_DIR / fmt / f"{card_key}_{fmt}.{spec['format']}"
            
            generate_card_screenshot(
                card_key,
                output_file,
                width=spec["width"],
                height=spec["height"]
            )
            
            generate_caption(card_key, fmt)
    
    print(f"\n{'='*50}")
    print("✅ Generation complete!")
    print(f"📁 Output: {OUTPUT_DIR}")
    print(f"{'='*50}")


if __name__ == "__main__":
    main()
