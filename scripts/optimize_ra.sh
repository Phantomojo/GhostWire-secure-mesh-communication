#!/bin/bash

# GhostWire: Rust-Analyzer Optimization Script

# 1. Project-level Optimization
# We've already created rust-analyzer.json in the project root.
# This script helps apply system-level caps.

echo "🚀 Optimizing GhostWire Rust-Analyzer Environment..."

# Define memory caps
RA_MEMORY_MAX="4G"
RA_MEMORY_HIGH="3.5G"

echo "📍 Memory Limit: $RA_MEMORY_MAX (High: $RA_MEMORY_HIGH)"

# 2. Systemd Scope for Memory Capping
# This command runs rust-analyzer in a managed systemd scope
echo ""
echo "💡 To start your IDE with a hard memory cap on rust-analyzer (Linux/systemd):"
echo "   alias code-cap='systemd-run --user --scope -p MemoryMax=$RA_MEMORY_MAX -p MemoryHigh=$RA_MEMORY_HIGH code .'"
echo ""

# 3. Monitoring
echo "📊 To monitor rust-analyzer memory usage, run:"
echo "   watch -n 5 'ps -u $USER -o pid,rss,command | grep rust-analyzer | grep -v grep'"
echo ""

# 4. Check for .vscode
if [ -d ".vscode" ]; then
    echo "✅ .vscode directory found."
    echo "   You can manually copy the settings from rust-analyzer.json into .vscode/settings.json if needed."
else
    echo "ℹ️ .vscode directory not found. Using root-level rust-analyzer.json."
fi

echo ""
echo "✨ Optimization configuration complete."
