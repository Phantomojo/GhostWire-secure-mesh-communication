#!/bin/bash
# Quick Profile Creation Debug Script
# Usage: ./debug_profile_creation.sh

set -e

echo "=========================================="
echo "  GhostWire Profile Creation Debugger"
echo "=========================================="
echo ""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Check if ghostwire directory exists
echo -n "📂 Checking ~/.ghostwire directory... "
if [ -d "$HOME/.ghostwire" ]; then
    echo -e "${GREEN}Exists${NC}"
    ls -la "$HOME/.ghostwire" | head -10
else
    echo -e "${YELLOW}Does not exist${NC}"
    echo "   Will be created on first profile creation"
fi
echo ""

# Check permissions
echo -n "🔐 Checking permissions... "
if [ -d "$HOME/.ghostwire" ]; then
    if [ -w "$HOME/.ghostwire" ]; then
        echo -e "${GREEN}Writable${NC}"
    else
        echo -e "${RED}NOT writable${NC}"
        echo "   Fix: chmod -R 755 ~/.ghostwire"
    fi
else
    echo -e "${YELLOW}N/A (directory doesn't exist)${NC}"
fi
echo ""

# Check disk space
echo -n "💾 Checking disk space... "
AVAILABLE=$(df -h "$HOME" 2>/dev/null | tail -1 | awk '{print $4}')
echo "${AVAILABLE} available"
echo ""

# Check HOME environment
echo -n "🏠 Checking HOME environment... "
echo "$HOME"
if [ -n "$GHOSTWIRE_HOME" ]; then
    echo -e "${YELLOW}⚠️  GHOSTWIRE_HOME is set to: $GHOSTWIRE_HOME${NC}"
else
    echo -e "${GREEN}Using default home directory${NC}"
fi
echo ""

# Check if ghostwire is running
echo -n "🚀 Checking if ghostwire is running... "
if pgrep -f "ghostwire" > /dev/null; then
    echo -e "${GREEN}Yes (PID: $(pgrep -f ghostwire | head -1))${NC}"
else
    echo -e "${YELLOW}No${NC}"
    echo "   Start with: cargo run --release"
fi
echo ""

# Test profile creation endpoint
echo "🧪 Testing profile creation endpoint..."
TEST_PROFILE="Test_$(date +%s)"
echo "   Attempting to create: $TEST_PROFILE"

RESPONSE=$(curl -s -w "\n%{http_code}" -X POST http://localhost:8080/api/v2/profile \
  -H "Content-Type: application/json" \
  -d "{\"name\": \"$TEST_PROFILE\"}" 2>&1)

HTTP_CODE=$(echo "$RESPONSE" | tail -1)
BODY=$(echo "$RESPONSE" | head -n -1)

if [ "$HTTP_CODE" = "200" ]; then
    echo -e "${GREEN}✅ Success!${NC}"
    echo "   Response: $BODY"
    
    # Clean up test profile
    echo ""
    echo -n "🧹 Cleaning up test profile... "
    curl -s -X DELETE "http://localhost:8080/api/v2/profile/$TEST_PROFILE" > /dev/null 2>&1
    echo -e "${GREEN}Done${NC}"
elif [ "$HTTP_CODE" = "409" ]; then
    echo -e "${YELLOW}⚠️  Profile already exists${NC}"
    echo "   Response: $BODY"
else
    echo -e "${RED}❌ Failed (HTTP $HTTP_CODE)${NC}"
    echo "   Response: $BODY"
    echo ""
    echo "   Check backend logs for details:"
    echo "   tail -f ghostwire.log | grep profile_creation"
fi
echo ""

echo "=========================================="
echo "  Debugging Tips"
echo "=========================================="
echo ""
echo "1. Start with verbose logging:"
echo "   RUST_LOG=info,profile_creation=debug cargo run --release"
echo ""
echo "2. Watch logs in real-time:"
echo "   tail -f ghostwire.log | grep -E 'profile_creation|identity'"
echo ""
echo "3. Check for errors:"
echo "   grep 'CRITICAL\|ERROR' ghostwire.log"
echo ""
echo "4. Common fixes:"
echo "   - Permissions: chmod -R 755 ~/.ghostwire"
echo "   - Remove existing: rm -rf ~/.ghostwire"
echo "   - Set custom path: export GHOSTWIRE_HOME=/path/to/data"
echo ""
echo "=========================================="
