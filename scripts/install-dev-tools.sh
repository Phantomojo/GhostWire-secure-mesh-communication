#!/bin/bash
set -e

echo "🔧 Installing GhostWire Developer Tools..."
echo ""

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

# Function to print section headers
print_header() {
    echo -e "${BLUE}========================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}========================================${NC}"
}

# Function to print success
print_success() {
    echo -e "${GREEN}✓ $1${NC}"
}

# Function to print warning
print_warning() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

# Track failures
FAILED=()

# Install function with error handling
install_tool() {
    local name=$1
    local cmd=$2
    
    echo -n "Installing $name... "
    if eval "$cmd" > /dev/null 2>&1; then
        print_success "$name installed"
    else
        print_warning "$name installation failed (may already be installed)"
        FAILED+=("$name")
    fi
}

print_header "📦 Security Tools"

install_tool "cargo-audit" "cargo install cargo-audit"
install_tool "cargo-deny" "cargo install cargo-deny"
install_tool "cargo-geiger" "cargo install cargo-geiger"

print_header "🧪 Testing Tools"

install_tool "cargo-nextest" "cargo install --locked cargo-nextest"
install_tool "cargo-llvm-cov" "cargo install cargo-llvm-cov"

print_header "📚 Documentation Tools"

print_warning "utoipa: Add to Cargo.toml manually (see docs/DEV_TOOLS.md)"

print_header "🛠️ Development Tools"

install_tool "cargo-watch" "cargo install cargo-watch"
install_tool "cargo-outdated" "cargo install cargo-outdated"
install_tool "cargo-expand" "cargo install cargo-expand"
install_tool "cargo-make" "cargo install cargo-make"

print_header "🌐 API Testing Tools"

# Check if Bruno can be installed
if command -v npm &> /dev/null; then
    echo -n "Installing Bruno (via npm)... "
    if npm install -g @usebruno/cli > /dev/null 2>&1; then
        print_success "Bruno CLI installed"
    else
        print_warning "Bruno CLI: Install from https://www.usebruno.com/downloads"
        FAILED+=("Bruno")
    fi
else
    print_warning "npm not found - Install Bruno manually from https://www.usebruno.com/downloads"
    FAILED+=("Bruno")
fi

# Summary
echo ""
print_header "📊 Installation Summary"

if [ ${#FAILED[@]} -eq 0 ]; then
    echo -e "${GREEN}✅ All tools installed successfully!${NC}"
else
    echo -e "${YELLOW}⚠ Some tools failed to install (may already be installed):${NC}"
    for tool in "${FAILED[@]}"; do
        echo "  - $tool"
    done
    echo ""
    echo "This is usually fine - they may already be installed."
fi

echo ""
echo "Next steps:"
echo "1. Run 'cargo deny init' to initialize security policies"
echo "2. Run 'cargo make init' to initialize task automation"
echo "3. Install Bruno GUI from: https://www.usebruno.com/downloads"
echo "4. Review docs/DEV_TOOLS.md for usage guide"
echo ""
echo "Verify installations:"
echo "  cargo audit --version"
echo "  cargo nextest --version"
echo "  cargo watch --version"
echo "  bruno --version (if GUI installed)"
echo ""
