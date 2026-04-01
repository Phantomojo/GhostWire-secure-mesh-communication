#!/bin/bash

##############################################################################
# GhostWire API Testing with Newman (Postman CLI)
# Comprehensive test runner for the Postman collection
##############################################################################

set -e

# Configuration
API_HOST="${1:-localhost:8080}"
API_URL="http://$API_HOST/api"
COLLECTION="tests/api/GhostWire_API.postman_collection.json"
RESULTS_DIR="./results"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

##############################################################################
# Helper Functions
##############################################################################

log() {
    echo "[$(date +'%H:%M:%S')] $*"
}

success() {
    echo -e "${GREEN}✓ $*${NC}"
}

error() {
    echo -e "${RED}✗ $*${NC}"
}

info() {
    echo -e "${BLUE}ℹ $*${NC}"
}

warning() {
    echo -e "${YELLOW}⚠ $*${NC}"
}

check_requirements() {
    info "Checking requirements..."
    
    # Check Node.js
    if ! command -v node &> /dev/null; then
        error "Node.js not found"
        info "Install from: https://nodejs.org/"
        exit 1
    fi
    success "Node.js: $(node --version)"
    
    # Check Newman
    if ! command -v newman &> /dev/null; then
        warning "Newman not found. Installing..."
        npm install -g newman || {
            error "Failed to install Newman"
            exit 1
        }
    fi
    success "Newman: $(newman --version)"
    
    # Check collection exists
    if [[ ! -f "$COLLECTION" ]]; then
        error "Collection not found: $COLLECTION"
        exit 1
    fi
    success "Postman Collection: $COLLECTION"
}

check_api() {
    info "Checking API connectivity at $API_URL..."
    
    if curl -s -m 5 "$API_URL/" > /dev/null 2>&1; then
        success "API is online"
        return 0
    else
        error "API not reachable at $API_URL"
        error "Start with: cargo run -- web --port 8080"
        exit 1
    fi
}

setup_environment() {
    info "Creating test environment..."
    
    mkdir -p "$RESULTS_DIR"
    
    # Create environment JSON
    cat > "$RESULTS_DIR/test-env-${TIMESTAMP}.json" <<EOF
{
  "id": "ghostwire-test-env",
  "name": "GhostWire Test Environment",
  "values": [
    {
      "key": "base_url",
      "value": "$API_URL",
      "type": "string",
      "enabled": true
    },
    {
      "key": "auth_token",
      "value": "",
      "type": "string",
      "enabled": true
    }
  ]
}
EOF
    
    success "Environment created"
    echo "$RESULTS_DIR/test-env-${TIMESTAMP}.json"
}

run_tests() {
    local env_file=$1
    
    info "Running tests..."
    info "  Collection: $COLLECTION"
    info "  Environment: $env_file"
    info "  API URL: $API_URL"
    
    echo ""
    
    newman run "$COLLECTION" \
        --environment "$env_file" \
        --reporters cli,json,html \
        --reporter-json-export "$RESULTS_DIR/results-${TIMESTAMP}.json" \
        --reporter-html-export "$RESULTS_DIR/results-${TIMESTAMP}.html" \
        --timeout 10000 \
        --timeout-request 5000 \
        --delay-request 500 \
        --bail \
        --verbose 2>&1 | tee "$RESULTS_DIR/test-${TIMESTAMP}.log"
    
    local exit_code=$?
    return $exit_code
}

generate_report() {
    local json_file="$RESULTS_DIR/results-${TIMESTAMP}.json"
    local html_file="$RESULTS_DIR/results-${TIMESTAMP}.html"
    
    if [[ ! -f "$json_file" ]]; then
        return
    fi
    
    info "Generating reports..."
    
    # Extract statistics from JSON
    local total_requests=$(grep -o '"request"' "$json_file" | wc -l)
    local failures=$(grep -o '"error"' "$json_file" | wc -l)
    local passed=$((total_requests - failures))
    
    # Create summary
    cat > "$RESULTS_DIR/summary-${TIMESTAMP}.txt" <<EOF
========================================
GhostWire API Test Summary
========================================

Test Date: $(date)
Test Duration: $(date -d @$(grep -oP '"responseTime":\K\d+' "$json_file" | awk '{s+=$1} END {print int(s/1000)}') 2>/dev/null || echo "N/A")

Results:
--------
Total Requests: $total_requests
Passed: $passed
Failed: $failures
Pass Rate: $((passed * 100 / total_requests))%

Reports:
--------
JSON: $json_file
HTML: $html_file
Log: $RESULTS_DIR/test-${TIMESTAMP}.log

EOF
    
    success "Reports generated:"
    echo "  JSON: $json_file"
    echo "  HTML: $html_file"
    echo "  Summary: $RESULTS_DIR/summary-${TIMESTAMP}.txt"
}

##############################################################################
# Main
##############################################################################

main() {
    clear
    log "=========================================="
    log "GhostWire API Test Suite (Newman)"
    log "=========================================="
    log ""
    
    # Setup
    check_requirements
    echo ""
    check_api
    echo ""
    
    # Create environment
    env_file=$(setup_environment)
    echo ""
    
    # Run tests
    if run_tests "$env_file"; then
        success "All tests passed!"
        generate_report
        exit 0
    else
        error "Some tests failed"
        generate_report
        exit 1
    fi
}

main "$@"
