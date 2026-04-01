#!/bin/bash

##############################################################################
# GhostWire Comprehensive API Test Suite
# Tests all endpoints using HTTPie (friendly curl alternative)
# Run: ./test-api.sh [host:port]
##############################################################################

set -e

# Configuration
API_HOST="${1:-localhost:8080}"
API_BASE="http://$API_HOST/api"
RESULTS_DIR="./results"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)
TOKEN=""

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Setup
mkdir -p "$RESULTS_DIR"
LOG_FILE="$RESULTS_DIR/test_${TIMESTAMP}.log"

##############################################################################
# Helper Functions
##############################################################################

log() {
    echo "[$(date +'%H:%M:%S')] $*" | tee -a "$LOG_FILE"
}

success() {
    echo -e "${GREEN}✓ $*${NC}" | tee -a "$LOG_FILE"
}

error() {
    echo -e "${RED}✗ $*${NC}" | tee -a "$LOG_FILE"
}

info() {
    echo -e "${BLUE}ℹ $*${NC}" | tee -a "$LOG_FILE"
}

warning() {
    echo -e "${YELLOW}⚠ $*${NC}" | tee -a "$LOG_FILE"
}

test_endpoint() {
    local method=$1
    local endpoint=$2
    local name=$3
    local data=$4
    local expected_code=${5:-200}
    
    info "Testing: $name"
    info "  $method $API_BASE$endpoint"
    
    local cmd="http --print=HhBb --timeout=10"
    
    if [[ -n "$TOKEN" ]]; then
        cmd="$cmd --header=X-Ghost-Token:$TOKEN"
    fi
    
    cmd="$cmd $method $API_BASE$endpoint"
    
    if [[ -n "$data" ]]; then
        cmd="$cmd $data"
    fi
    
    local response=$(eval "$cmd" 2>&1 || true)
    local http_code=$(echo "$response" | grep -oP '(?<=HTTP/1.1 )\d+' | tail -1)
    
    if [[ "$http_code" == "$expected_code" ]]; then
        success "$name - HTTP $http_code"
        echo "$response" >> "$LOG_FILE"
        return 0
    else
        error "$name - Expected $expected_code, got $http_code"
        echo "$response" >> "$LOG_FILE"
        return 1
    fi
}

##############################################################################
# Main Test Suite
##############################################################################

main() {
    clear
    log "=========================================="
    log "GhostWire API Test Suite"
    log "=========================================="
    log "Target: $API_BASE"
    log "Log: $LOG_FILE"
    log ""
    
    # Use curl (always available)
    info "Using curl for HTTP requests"
    
    # Verify API is running
    log "Verifying API connectivity..."
    if ! curl -s -m 5 "$API_BASE/" > /dev/null 2>&1; then
        error "API not reachable at $API_BASE"
        error "Start the API with: cargo run -- web --port 8080"
        exit 1
    fi
    success "API is online"
    log ""
    
    # ========== SECTION 1: Auth & Bootstrap ==========
    log "========== Section 1: Auth & Bootstrap =========="
    
    # Get API version
    test_endpoint "GET" "/" "API Root" "" 200
    
    # Get status
    test_endpoint "GET" "/status" "System Status" "" 200
    
    # Get dev token
    info "Getting authentication token..."
    TOKEN_RESPONSE=$(curl -s "$API_BASE/dev/token")
    TOKEN=$(echo "$TOKEN_RESPONSE" | grep -oP '"data":"?\K[^"]+' | head -1 || echo "")
    
    if [[ -z "$TOKEN" ]]; then
        warning "Could not extract token. Some authenticated tests will fail."
    else
        success "Auth Token obtained: ${TOKEN:0:20}..."
    fi
    log ""
    
    # ========== SECTION 2: Profile Management ==========
    log "========== Section 2: Profile Management =========="
    
    # List profiles
    test_endpoint "GET" "/v2/profile" "List Profiles" "" 200
    
    # Create new profile
    test_endpoint "POST" "/v2/profile" "Create Profile" \
        "name=test_profile_${TIMESTAMP}" 200 || warning "Profile creation may require different endpoint"
    
    log ""
    
    # ========== SECTION 3: Identity Management ==========
    log "========== Section 3: Identity Management =========="
    
    # Get identity status
    test_endpoint "GET" "/v2/profile" "Identity Status" "" 200
    
    # Get public key
    test_endpoint "GET" "/v2/profile/public-key" "Get Public Key" "" 200
    
    log ""
    
    # ========== SECTION 4: Authenticated Endpoints ==========
    if [[ -n "$TOKEN" ]]; then
        log "========== Section 4: Authenticated Endpoints =========="
        
        # Get profile (authenticated)
        test_endpoint "GET" "/v2/profile" "Get Profile (Authenticated)" "" 200
        
        # Get security dashboard
        test_endpoint "GET" "/v2/security/dashboard" "Security Dashboard" "" 200
        
        # List peers
        test_endpoint "GET" "/v2/peers" "List Peers" "" 200
        
        # Get messages
        test_endpoint "GET" "/v2/messages" "Get Messages" "" 200
        
        # Send broadcast message
        test_endpoint "POST" "/broadcast" "Broadcast Message" \
            "content='Test message from API test suite'" 200 || true
        
        log ""
    else
        warning "Skipping authenticated endpoints - no token available"
    fi
    
    # ========== SECTION 5: Error Handling ==========
    log "========== Section 5: Error Handling & Edge Cases =========="
    
    # Invalid endpoint
    warning "Testing 404 error handling..."
    test_endpoint "GET" "/invalid/endpoint" "Invalid Endpoint" "" 404 || true
    
    # Invalid method
    warning "Testing invalid method..."
    test_endpoint "DELETE" "/" "Invalid Method" "" || true
    
    # Missing auth (protected endpoint)
    warning "Testing auth requirement..."
    HTTP_CODE=$(curl -s -w "%{http_code}" -o /dev/null "$API_BASE/v2/security/dashboard")
    if [[ "$HTTP_CODE" != "200" ]]; then
        success "Auth protection working - returned $HTTP_CODE"
    fi
    
    log ""
    
    # ========== SECTION 6: Performance ==========
    log "========== Section 6: Performance Testing =========="
    
    info "Testing response times..."
    
    for endpoint in "/" "/v2/peers" "/v2/profile"; do
        local start=$(date +%s%N)
        curl -s "$API_BASE$endpoint" > /dev/null 2>&1 || true
        local end=$(date +%s%N)
        local duration=$(( (end - start) / 1000000 ))
        
        if [[ $duration -lt 100 ]]; then
            success "$endpoint - ${duration}ms"
        elif [[ $duration -lt 500 ]]; then
            warning "$endpoint - ${duration}ms (acceptable)"
        else
            error "$endpoint - ${duration}ms (slow)"
        fi
    done
    
    log ""
    
    # ========== Summary ==========
    log "=========================================="
    log "Test Suite Complete"
    log "=========================================="
    log "Results saved to: $LOG_FILE"
    log ""
    
    # Count results
    local passed=$(grep -c "✓" "$LOG_FILE" || true)
    local failed=$(grep -c "✗" "$LOG_FILE" || true)
    
    echo ""
    echo -e "${BLUE}Summary:${NC}"
    echo -e "  ${GREEN}Passed: $passed${NC}"
    echo -e "  ${RED}Failed: $failed${NC}"
    echo ""
    
    if [[ $failed -eq 0 ]]; then
        success "All tests passed!"
        exit 0
    else
        error "Some tests failed. Review log for details."
        exit 1
    fi
}

# Run main
main "$@"
