#!/bin/bash
# ═══════════════════════════════════════════════════════════════════
# GhostWire Instagram Content Renderer
# ═══════════════════════════════════════════════════════════════════
#
# One-command rendering for all Instagram content
#
# Usage:
#   ./render_instagram_content.sh [card] [format]
#
# Examples:
#   ./render_instagram_content.sh                    # Render all cards, all formats
#   ./render_instagram_content.sh launch reel        # Render launch card as Reel
#   ./render_instagram_content.sh all story          # Render all cards as Stories
#
# ═══════════════════════════════════════════════════════════════════

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Paths
SCRIPT_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
PROJECT_ROOT="$(dirname "$(dirname "$SCRIPT_DIR")")"
CARDS_DIR="/home/ph/Downloads/xgh"
OUTPUT_DIR="$SCRIPT_DIR/content"
PYTHON_SCRIPT="$SCRIPT_DIR/instagram_content_generator.py"

# Default values
CARD="${1:-all}"
FORMAT="${2:-all}"

echo -e "${BLUE}╔══════════════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║     GhostWire Instagram Content Renderer                         ║${NC}"
echo -e "${BLUE}╚══════════════════════════════════════════════════════════════════╝${NC}"
echo ""

# Function to check dependencies
check_dependencies() {
    echo -e "${YELLOW}Checking dependencies...${NC}"
    
    MISSING=()
    
    # Check Python
    if ! command -v python3 &> /dev/null; then
        MISSING+=("python3")
    fi
    
    # Check Playwright
    if ! python3 -c "import playwright" &> /dev/null; then
        MISSING+=("playwright (pip install playwright)")
    fi
    
    # Check FFmpeg
    if ! command -v ffmpeg &> /dev/null; then
        MISSING+=("ffmpeg (sudo pacman -S ffmpeg)")
    fi
    
    # Check Node.js
    if ! command -v node &> /dev/null; then
        MISSING+=("node (sudo pacman -S nodejs)")
    fi
    
    if [ ${#MISSING[@]} -gt 0 ]; then
        echo -e "${RED}❌ Missing dependencies:${NC}"
        for dep in "${MISSING[@]}"; do
            echo -e "${RED}   - $dep${NC}"
        done
        echo ""
        echo -e "${YELLOW}Install with:${NC}"
        echo "   sudo pacman -S ffmpeg nodejs"
        echo "   pip install playwright"
        echo "   playwright install chromium"
        exit 1
    fi
    
    echo -e "${GREEN}✅ All dependencies found${NC}"
    echo ""
}

# Function to create output directories
create_directories() {
    echo -e "${YELLOW}Creating output directories...${NC}"
    mkdir -p "$OUTPUT_DIR"/{reel,story,feed,carousel,captions,thumbnails}
    echo -e "${GREEN}✅ Directories created at $OUTPUT_DIR${NC}"
    echo ""
}

# Function to render cards
render_cards() {
    echo -e "${YELLOW}Rendering cards...${NC}"
    echo "   Card: $CARD"
    echo "   Format: $FORMAT"
    echo ""
    
    cd "$SCRIPT_DIR"
    
    if [ "$CARD" = "all" ] && [ "$FORMAT" = "all" ]; then
        echo -e "${BLUE}Rendering ALL cards in ALL formats...${NC}"
        python3 "$PYTHON_SCRIPT" --card all --format all
    elif [ "$FORMAT" = "all" ]; then
        echo -e "${BLUE}Rendering $CARD card in ALL formats...${NC}"
        python3 "$PYTHON_SCRIPT" --card "$CARD" --format all
    elif [ "$CARD" = "all" ]; then
        echo -e "${BLUE}Rendering ALL cards in $FORMAT format...${NC}"
        python3 "$PYTHON_SCRIPT" --card all --format "$FORMAT"
    else
        echo -e "${BLUE}Rendering $CARD card in $FORMAT format...${NC}"
        python3 "$PYTHON_SCRIPT" --card "$CARD" --format "$FORMAT"
    fi
}

# Function to show summary
show_summary() {
    echo ""
    echo -e "${GREEN}╔══════════════════════════════════════════════════════════════════╗${NC}"
    echo -e "${GREEN}║     ✅ Rendering Complete!                                       ║${NC}"
    echo -e "${GREEN}╚══════════════════════════════════════════════════════════════════╝${NC}"
    echo ""
    echo -e "${BLUE}📁 Output Location:${NC}"
    echo "   $OUTPUT_DIR"
    echo ""
    echo -e "${BLUE}📊 Generated Files:${NC}"
    
    if [ -d "$OUTPUT_DIR/reel" ]; then
        REEL_COUNT=$(ls -1 "$OUTPUT_DIR/reel"/*.mp4 2>/dev/null | wc -l)
        echo "   Reels: $REEL_COUNT videos"
    fi
    
    if [ -d "$OUTPUT_DIR/story" ]; then
        STORY_COUNT=$(ls -1 "$OUTPUT_DIR/story"/*.mp4 2>/dev/null | wc -l)
        echo "   Stories: $STORY_COUNT videos"
    fi
    
    if [ -d "$OUTPUT_DIR/feed" ]; then
        FEED_COUNT=$(ls -1 "$OUTPUT_DIR/feed"/*.mp4 2>/dev/null | wc -l)
        echo "   Feed Posts: $FEED_COUNT videos"
    fi
    
    if [ -d "$OUTPUT_DIR/captions" ]; then
        CAPTION_COUNT=$(ls -1 "$OUTPUT_DIR/captions"/*.txt 2>/dev/null | wc -l)
        echo "   Captions: $CAPTION_COUNT files"
    fi
    
    echo ""
    echo -e "${BLUE}📝 Next Steps:${NC}"
    echo "   1. Review generated content in $OUTPUT_DIR"
    echo "   2. Read captions: cat $OUTPUT_DIR/captions/[filename]_caption.txt"
    echo "   3. Upload to Instagram manually or use scheduling tool"
    echo ""
    echo -e "${YELLOW}📖 Full strategy guide:${NC}"
    echo "   $SCRIPT_DIR/INSTAGRAM_CONTENT_STRATEGY.md"
    echo ""
}

# Main execution
main() {
    check_dependencies
    create_directories
    render_cards
    show_summary
}

# Run main function
main
