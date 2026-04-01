#!/bin/bash
# GhostWire Protocol Test Runner
# Runs all protocol integration tests with Docker

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Configuration
TEST_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
RESULTS_DIR="${TEST_DIR}/test-results"
DOCKER_COMPOSE_FILE="${TEST_DIR}/docker-compose.protocol-test.yml"

# Create results directory
mkdir -p "${RESULTS_DIR}"

echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
echo -e "${BLUE}║   GhostWire Protocol Integration Test Suite           ║${NC}"
echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
echo ""

# Function to print status
print_status() {
    local status=$1
    local message=$2
    
    case $status in
        "PASS")
            echo -e "${GREEN}✅ PASS${NC}: ${message}"
            ;;
        "FAIL")
            echo -e "${RED}❌ FAIL${NC}: ${message}"
            ;;
        "INFO")
            echo -e "${BLUE}ℹ️  INFO${NC}: ${message}"
            ;;
        "WARN")
            echo -e "${YELLOW}⚠️  WARN${NC}: ${message}"
            ;;
    esac
}

# Function to run tests
run_tests() {
    local test_type=$1
    
    echo -e "\n${YELLOW}Running ${test_type} tests...${NC}\n"
    
    case $test_type in
        "protocol")
            # Run protocol integration tests
            docker-compose -f "${DOCKER_COMPOSE_FILE}" run --rm protocol-test
            ;;
        "local")
            # Run tests locally (requires Rust toolchain)
            cd "${TEST_DIR}/ghostwire"
            cargo test --test protocol_integration -- --nocapture
            ;;
        "all")
            # Run all test types
            run_tests "protocol"
            run_tests "local"
            ;;
    esac
}

# Function to cleanup
cleanup() {
    print_status "INFO" "Cleaning up test containers..."
    docker-compose -f "${DOCKER_COMPOSE_FILE}" down --remove-orphans 2>/dev/null || true
}

# Trap for cleanup on exit
trap cleanup EXIT

# Parse command line arguments
TEST_TYPE="protocol"
VERBOSE=false

while [[ $# -gt 0 ]]; do
    case $1 in
        -p|--protocol)
            TEST_TYPE="protocol"
            shift
            ;;
        -l|--local)
            TEST_TYPE="local"
            shift
            ;;
        -a|--all)
            TEST_TYPE="all"
            shift
            ;;
        -v|--verbose)
            VERBOSE=true
            shift
            ;;
        -h|--help)
            echo "Usage: $0 [OPTIONS]"
            echo ""
            echo "Options:"
            echo "  -p, --protocol    Run protocol tests in Docker (default)"
            echo "  -l, --local       Run tests locally (requires Rust)"
            echo "  -a, --all         Run all test types"
            echo "  -v, --verbose     Enable verbose output"
            echo "  -h, --help        Show this help message"
            exit 0
            ;;
        *)
            echo -e "${RED}Unknown option: $1${NC}"
            exit 1
            ;;
    esac
done

# Check prerequisites
check_prerequisites() {
    print_status "INFO" "Checking prerequisites..."
    
    # Check Docker
    if ! command -v docker &> /dev/null; then
        print_status "FAIL" "Docker is not installed"
        exit 1
    fi
    print_status "PASS" "Docker is installed"
    
    # Check Docker Compose
    if ! command -v docker-compose &> /dev/null; then
        print_status "FAIL" "Docker Compose is not installed"
        exit 1
    fi
    print_status "PASS" "Docker Compose is installed"
    
    # Check if Docker daemon is running
    if ! docker info &> /dev/null; then
        print_status "FAIL" "Docker daemon is not running"
        exit 1
    fi
    print_status "PASS" "Docker daemon is running"
    
    echo ""
}

# Main execution
main() {
    print_status "INFO" "Starting GhostWire Protocol Tests"
    print_status "INFO" "Test type: ${TEST_TYPE}"
    print_status "INFO" "Results directory: ${RESULTS_DIR}"
    echo ""
    
    check_prerequisites
    
    # Build test image
    print_status "INFO" "Building test image..."
    docker-compose -f "${DOCKER_COMPOSE_FILE}" build
    
    if [ $? -ne 0 ]; then
        print_status "FAIL" "Failed to build test image"
        exit 1
    fi
    print_status "PASS" "Test image built successfully"
    
    # Run tests
    echo ""
    run_tests "${TEST_TYPE}"
    
    TEST_EXIT_CODE=$?
    
    # Print summary
    echo ""
    echo -e "${BLUE}╔════════════════════════════════════════════════════════╗${NC}"
    echo -e "${BLUE}║                    Test Summary                        ║${NC}"
    echo -e "${BLUE}╚════════════════════════════════════════════════════════╝${NC}"
    echo ""
    
    if [ $TEST_EXIT_CODE -eq 0 ]; then
        print_status "PASS" "All tests passed!"
        echo ""
        echo -e "${GREEN}Test results saved to: ${RESULTS_DIR}/${NC}"
        exit 0
    else
        print_status "FAIL" "Some tests failed"
        echo ""
        echo -e "${RED}Check test logs for details: ${RESULTS_DIR}/${NC}"
        exit 1
    fi
}

# Run main function
main
