#!/usr/bin/env bash
# Run once during setup — eliminates Google Fonts CDN dependency
set -e
mkdir -p public/fonts
BASE="https://raw.githubusercontent.com/JetBrains/JetBrainsMono/master/fonts/webfonts"
curl -fsSL "$BASE/JetBrainsMono-Regular.woff2" -o public/fonts/JetBrainsMono-Regular.woff2
curl -fsSL "$BASE/JetBrainsMono-Medium.woff2"  -o public/fonts/JetBrainsMono-Medium.woff2
curl -fsSL "$BASE/JetBrainsMono-Bold.woff2"    -o public/fonts/JetBrainsMono-Bold.woff2
echo "Fonts ready. No Google CDN dependency."
