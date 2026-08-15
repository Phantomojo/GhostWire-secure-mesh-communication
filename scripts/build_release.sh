#!/bin/bash
set -e

# Colors
GREEN='\033[0;32m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

echo -e "${BLUE}[GhostWire] Starting Clean Release Build...${NC}"

# 1. Build Backend
# The legacy WebUI was removed from the tracked source tree. The daemon still
# serves its authenticated API; embedding a WebUI requires the explicit
# `embedded-webui` feature and a separately supplied, reviewed asset tree.
echo -e "${BLUE}[1/2] Compiling Rust Core (Release Mode - Optimized)...${NC}"
cd ghostwire
cargo build --release
cd ..

# 2. Package Release
echo -e "${BLUE}[2/2] Packaging Release...${NC}"

mkdir -p release
cp ghostwire/target/release/ghostwire release/ghostwire_node
chmod +x release/ghostwire_node

echo -e "${GREEN}✅ Build Complete!${NC}"
echo -e "${GREEN}Binary location: ./release/ghostwire_node${NC}"
echo -e "Run it with: ./release/ghostwire_node --web --port 8080"
