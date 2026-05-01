#!/bin/bash
# GhostWire Test Runner
# One-command test execution with log collection and reporting
#
# Usage:
#   ./run_tests.sh              # Run all tests
#   ./run_tests.sh --all        # Same as above
#   ./run_tests.sh --quick      # Quick smoke test (connectivity + messaging)
#   ./run_tests.sh --stress     # Stress test only
#   ./run_tests.sh --full       # All tests + network simulation + benchmarks
#   ./run_tests.sh --deploy     # Just deploy and start nodes
#   ./run_tests.sh --status     # Check node status

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
GHOSTWIRE_DIR="$PROJECT_DIR/ghostwire"
TEST_SCRIPT="$SCRIPT_DIR/scripts/test_orchestrator.py"
DEPLOY_SCRIPT="$SCRIPT_DIR/scripts/deploy.sh"
NETWORK_SIM="$SCRIPT_DIR/scripts/network_sim.sh"
LOG_DIR="$SCRIPT_DIR/logs"
RESULT_DIR="$SCRIPT_DIR/results"
TIMESTAMP=$(date +%Y%m%d_%H%M%S)

# Colors
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

banner() {
    echo -e "\n${CYAN}╔══════════════════════════════════════════════════════════╗${NC}"
    echo -e "${CYAN}║  GhostWire Test Suite${NC}"
    echo -e "${CYAN}║  $(date '+%Y-%m-%d %H:%M:%S')${NC}"
    echo -e "${CYAN}╚══════════════════════════════════════════════════════════╝${NC}\n"
}

step() {
    echo -e "\n${BLUE}▶ $1${NC}"
}

ok() {
    echo -e "${GREEN}✓ $1${NC}"
}

warn() {
    echo -e "${YELLOW}⚠ $1${NC}"
}

err() {
    echo -e "${RED}✗ $1${NC}"
}

# Build if needed
build_if_needed() {
    if [ ! -f "$GHOSTWIRE_DIR/target/release/ghostwire" ]; then
        step "Building GhostWire (release mode)..."
        cd "$GHOSTWIRE_DIR"
        cargo build --release 2>&1 | tail -5
        ok "Build complete"
    else
        ok "Binary exists (skipping build)"
    fi
}

# Deploy and start nodes
deploy_and_start() {
    step "Deploying binary to nodes..."
    bash "$DEPLOY_SCRIPT" deploy

    step "Starting GhostWire nodes..."
    bash "$DEPLOY_SCRIPT" start

    step "Checking node status..."
    bash "$DEPLOY_SCRIPT" status
}

# Run Python test orchestrator
run_tests() {
    local test_args=$1

    step "Running tests: $test_args"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    python3 "$TEST_SCRIPT" $test_args

    echo -e "\n${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"
}

# Collect and display results
show_results() {
    step "Test Results"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    # Find latest result files
    local latest_results=$(ls -t "$RESULT_DIR"/*.json 2>/dev/null | head -5)

    if [ -z "$latest_results" ]; then
        warn "No result files found"
        return
    fi

    for f in $latest_results; do
        echo -e "\n${YELLOW}📄 $(basename $f)${NC}"
        python3 -c "
import json
with open('$f') as fp:
    data = json.load(fp)
    s = data.get('summary', {})
    print(f\"  Suite: {s.get('suite', 'unknown')}\")
    print(f\"  Passed: {s.get('passed', 0)}/{s.get('total', 0)} ({s.get('pass_rate', 'N/A')})\")
    print(f\"  Duration: {s.get('duration_secs', 0)}s\")
    for r in data.get('results', []):
        status = '✅' if r['passed'] else '❌'
        print(f\"  {status} {r['name']}: {r.get('details', '')[:80]}\")
" 2>/dev/null || echo "  (parse error)"
    done
}

# Show logs
show_logs() {
    step "Recent Logs"
    echo -e "${BLUE}━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━${NC}"

    bash "$DEPLOY_SCRIPT" logs 20
}

# Main
banner

case "${1:---all}" in
    --deploy)
        build_if_needed
        deploy_and_start
        ;;
    --quick)
        build_if_needed
        deploy_and_start
        sleep 5
        run_tests "--connectivity --messaging --api"
        show_results
        show_logs
        ;;
    --stress)
        build_if_needed
        deploy_and_start
        sleep 5
        run_tests "--stress --count 200"
        show_results
        show_logs
        ;;
    --resilience)
        build_if_needed
        deploy_and_start
        sleep 5
        run_tests "--resilience"
        show_results
        show_logs
        ;;
    --network)
        build_if_needed
        deploy_and_start
        sleep 5
        run_tests "--network-sim"
        show_results
        show_logs
        ;;
    --benchmark)
        build_if_needed
        deploy_and_start
        sleep 5
        run_tests "--benchmark"
        show_results
        show_logs
        ;;
    --full)
        build_if_needed
        deploy_and_start
        sleep 5
        run_tests "--all"
        show_results
        show_logs
        ;;
    --status)
        bash "$DEPLOY_SCRIPT" status
        ;;
    --logs)
        bash "$DEPLOY_SCRIPT" logs ${2:-50} ${3:-""}
        ;;
    --all|*)
        build_if_needed
        deploy_and_start
        sleep 5
        run_tests "--all"
        show_results
        show_logs
        ;;
esac

echo -e "\n${GREEN}══════════════════════════════════════════════════════════${NC}"
echo -e "${GREEN}  Test run complete. Results saved to: $RESULT_DIR${NC}"
echo -e "${GREEN}  Logs saved to: $LOG_DIR${NC}"
echo -e "${GREEN}══════════════════════════════════════════════════════════${NC}\n"
