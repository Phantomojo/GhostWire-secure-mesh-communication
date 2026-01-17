#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}[GhostWire] Starting Clean Release Build...${NC}"

# 1. Build Frontend
echo -e "${BLUE}[1/3] Building WebUI (Profile & Assets)...${NC}"
cd webui
npm install --silent
npm run build
cd ..

# 2. Build Backend
echo -e "${BLUE}[2/3] Compiling Rust Core (Release Mode - Optimized)...${NC}"
cd ghostwire
# Ensure the dist folder is picked up by rust-embed
if [ ! -d "../webui/dist" ]; then
    echo "Error: webui/dist not found!"
    exit 1
fi
cargo build --release
cd ..

# 3. Package Release
echo -e "${BLUE}[3/3] Packaging Release...${NC}"
mkdir -p release
cp ghostwire/target/release/ghostwire release/ghostwire_node
chmod +x release/ghostwire_node

echo -e "${GREEN}✅ Build Complete!${NC}"
echo -e "${GREEN}Binary location: ./release/ghostwire_node${NC}"
echo -e "Run it with: ./release/ghostwire_node --web --port 8080"
