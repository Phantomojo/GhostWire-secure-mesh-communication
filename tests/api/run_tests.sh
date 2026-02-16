#!/bin/bash
# ============================================================
# GhostWire API Test Runner
# ============================================================
# Runs all hurl API tests against a live backend.
# Usage: ./tests/api/run_tests.sh [base_url]
# Default: http://localhost:3001
# ============================================================

set -e

BASE_URL="${1:-http://localhost:3001}"
SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
PASS=0
FAIL=0
TOTAL=0

echo "╔══════════════════════════════════════════════════╗"
echo "║      GhostWire API Test Suite (hurl)             ║"
echo "╠══════════════════════════════════════════════════╣"
echo "║  Target: $BASE_URL"
echo "╚══════════════════════════════════════════════════╝"
echo ""

# Check if hurl is installed
if ! command -v hurl &> /dev/null; then
    echo "❌ hurl is not installed. Install with: sudo pacman -S hurl"
    exit 1
fi

# Check if backend is running
if ! curl -s "${BASE_URL}/api/" > /dev/null 2>&1; then
    echo "❌ Backend not reachable at ${BASE_URL}"
    echo "   Start it with: cargo run -- start"
    exit 1
fi

echo "🔍 Backend is reachable. Running tests..."
echo ""

# Run each test file in order
for test_file in "$SCRIPT_DIR"/*.hurl; do
    test_name=$(basename "$test_file" .hurl)
    TOTAL=$((TOTAL + 1))
    
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    echo "🧪 Running: $test_name"
    echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
    
    if hurl --test --color "$test_file" 2>&1; then
        echo "✅ $test_name PASSED"
        PASS=$((PASS + 1))
    else
        echo "❌ $test_name FAILED"
        FAIL=$((FAIL + 1))
    fi
    echo ""
done

# Summary
echo "╔══════════════════════════════════════════════════╗"
echo "║                 TEST SUMMARY                     ║"
echo "╠══════════════════════════════════════════════════╣"
echo "║  Total:  $TOTAL"
echo "║  Passed: $PASS ✅"
echo "║  Failed: $FAIL ❌"
echo "╚══════════════════════════════════════════════════╝"

if [ $FAIL -gt 0 ]; then
    exit 1
fi
