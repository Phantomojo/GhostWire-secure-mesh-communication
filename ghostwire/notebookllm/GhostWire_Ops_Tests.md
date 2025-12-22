# GhostWire Operations, Configuration, and Tests


# File: Cargo.toml
```toml
[package]
name = "ghostwire"
version = "0.1.0"
edition = "2021"
authors = ["GhostWire Team"]
description = "Secure mesh networking and messaging platform"
license = "AGPL-3.0-or-later"
repository = "https://github.com/ghostwire/ghostwire"
keywords = ["mesh", "networking", "security", "privacy", "p2p"]
categories = ["network-programming", "security", "asynchronous"]

[dependencies]
# Core async runtime
tokio = { version = "1.0", features = ["full"] }

# Web framework
axum = { version = "0.7", features = ["ws"] }
tower = { version = "0.4" }
tower-http = { version = "0.5", features = ["cors"] }

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = { version = "1.0" }
toml = { version = "0.8" }

# Networking
libp2p = { version = "0.53", features = ["tokio", "macros", "tcp", "noise", "yamux", "kad", "gossipsub", "mdns", "identify"] }
reqwest = { version = "0.11", features = ["json"] }
libp2p-tcp = { version = "0.41" }

# Cryptography
ring = { version = "0.17" }
ed25519-dalek = { version = "2.0" }
x25519-dalek = { version = "2.0" }
aes-gcm = { version = "0.10" }
chacha20poly1305 = { version = "0.10" }

# Utilities
anyhow = { version = "1.0" }
thiserror = { version = "1.0" }
uuid = { version = "1.0", features = ["v4", "serde"] }
chrono = { version = "0.4", features = ["serde"] }
base64 = { version = "0.21" }
hex = { version = "0.4" }
bs58 = { version = "0.5" }
lazy_static = { version = "1.4" }
rand = { version = "0.8" }
dashmap = { version = "5.0" }
bytes = { version = "1.5" }
bincode = { version = "1.3" }

# CLI
clap = { version = "4.0", features = ["derive"] }

# Logging
tracing = { version = "0.1" }
tracing-subscriber = { version = "0.3" }

# System
hostname = { version = "0.3" }
local-ip-address = { version = "0.5" }

# Optional features
neli = { version = "0.6" }
async-trait = { version = "0.1" }
futures = { version = "0.3", optional = true }
futures-util = { version = "0.3" }
matrix-sdk = { version = "0.7", optional = true }
serialport = { version = "4.3", optional = true }

# TUI dependencies
ratatui = { version = "0.26" }
crossterm = { version = "0.27" }

[features]
default = ["web", "cli", "mesh", "security", "tui"]
web = []
cli = []
mesh = []
security = []
matrix = []
matrix-bridge = ["matrix-sdk"]
meshtastic = ["serialport"]
reticulum = []
briar = []
stealth = []
tui = []

# Minimal build for embedded systems
minimal = []

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true

[profile.dev]
opt-level = 0
debug = true

[profile.release.package."*"]
opt-level = 3

# Optimize specific crates
[profile.release.package.ring]
opt-level = 3

[profile.release.package.libp2p]
opt-level = 3

[profile.release.package.tokio]
opt-level = 3

```


# File: config.toml
```toml
[server]
host = "0.0.0.0"
port = 3000
web_enabled = true
cli_enabled = false
tui_enabled = false

[security]
max_connections_per_ip = 50
max_messages_per_minute = 200
blacklist_enabled = true
threat_detection_enabled = true
key_rotation_interval_days = 7
encryption_algorithm = "aes-256-gcm"

[network]
mesh_enabled = true
reticulum_enabled = false
briar_enabled = false
stealth_tcp_enabled = false
max_peers = 100
connection_timeout_secs = 30
peer_discovery_interval_secs = 60
heartbeat_interval_secs = 30

[features]
matrix_bridge = false
meshtastic = false
web_enabled = true
tui_enabled = false

[logging]
level = "info"
file_enabled = true
file_path = "/app/data/logs/ghostwire.log"
max_file_size_mb = 100
max_files = 5

[storage]
data_dir = "/app/data"
config_dir = "/app/config"
backup_enabled = true
backup_interval_hours = 24
backup_retention_days = 7

[monitoring]
health_check_enabled = true
metrics_enabled = true
metrics_port = 9090
alerting_enabled = false 
```


# File: ghostwire.toml.example
```toml
# GhostWire Configuration File
# Copy this to ghostwire.toml and modify as needed

[server]
host = "127.0.0.1"
port = 8080
web_enabled = true
cli_enabled = true

[security]
max_connections_per_ip = 10
max_messages_per_minute = 100
blacklist_enabled = true
threat_detection_enabled = true
allowed_ips = []
blocked_ips = []
key_rotation_interval_days = 7

[network]
mesh_enabled = true
reticulum_enabled = false
briar_enabled = false
stealth_tcp_enabled = false
max_peers = 50
connection_timeout_secs = 30

[encryption]
algorithm = "AES-256-GCM"
key_size = 32
session_timeout_hours = 1

[features]
matrix_bridge = false
meshtastic_bridge = false
web_ui = true
cli = true
api = true

# Environment variables for sensitive data:
# MATRIX_HOMESERVER=https://matrix.org
# MATRIX_USER=@youruser:matrix.org
# MATRIX_ACCESS_TOKEN=your_access_token_here
# MESHTASTIC_DEVICE_PATH=/dev/ttyUSB0
# MESHTASTIC_CHANNEL=main 
```


# File: build-optimized.sh
```bash
#!/bin/bash

# GhostWire Optimized Build Script
# This script builds different versions of GhostWire with varying feature sets

set -e

echo "🔧 GhostWire Optimized Build Script"
echo "=================================="

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to get binary size
get_binary_size() {
    local binary_path="$1"
    if [ -f "$binary_path" ]; then
        du -h "$binary_path" | cut -f1
    else
        echo "N/A"
    fi
}

# Clean previous builds
print_status "Cleaning previous builds..."
cargo clean

# Build minimal version (core functionality only)
print_status "Building minimal version..."
cargo build --release --no-default-features --features minimal
MINIMAL_SIZE=$(get_binary_size "target/release/ghostwire")
print_success "Minimal build complete: ${MINIMAL_SIZE}"

# Build security-focused version
print_status "Building security-focused version..."
cargo build --release --no-default-features --features "minimal,security"
SECURITY_SIZE=$(get_binary_size "target/release/ghostwire")
print_success "Security build complete: ${SECURITY_SIZE}"

# Build CLI version
print_status "Building CLI version..."
cargo build --release --no-default-features --features "minimal,security,cli"
CLI_SIZE=$(get_binary_size "target/release/ghostwire")
print_success "CLI build complete: ${CLI_SIZE}"

# Build web version
print_status "Building web version..."
cargo build --release --no-default-features --features "minimal,security,web"
WEB_SIZE=$(get_binary_size "target/release/ghostwire")
print_success "Web build complete: ${WEB_SIZE}"

# Build mesh version
print_status "Building mesh version..."
cargo build --release --no-default-features --features "minimal,security,cli,mesh"
MESH_SIZE=$(get_binary_size "target/release/ghostwire")
print_success "Mesh build complete: ${MESH_SIZE}"

# Build full version
print_status "Building full version..."
cargo build --release
FULL_SIZE=$(get_binary_size "target/release/ghostwire")
print_success "Full build complete: ${FULL_SIZE}"

# Create optimized builds with different names
print_status "Creating optimized builds..."

# Minimal binary
cp target/release/ghostwire target/release/ghostwire-minimal

# CLI binary
cargo build --release --no-default-features --features "minimal,security,cli"
cp target/release/ghostwire target/release/ghostwire-cli

# Web binary
cargo build --release --no-default-features --features "minimal,security,web"
cp target/release/ghostwire target/release/ghostwire-web

# Mesh binary
cargo build --release --no-default-features --features "minimal,security,cli,mesh"
cp target/release/ghostwire target/release/ghostwire-mesh

# Full binary
cargo build --release
cp target/release/ghostwire target/release/ghostwire-full

# Print summary
echo ""
echo "📊 Build Summary"
echo "================"
echo "Minimal (core only):     ${MINIMAL_SIZE}"
echo "Security-focused:        ${SECURITY_SIZE}"
echo "CLI version:             ${CLI_SIZE}"
echo "Web version:             ${WEB_SIZE}"
echo "Mesh version:            ${MESH_SIZE}"
echo "Full version:            ${FULL_SIZE}"
echo ""

# List all binaries
print_status "Available binaries:"
ls -lh target/release/ghostwire-*

print_success "Build process complete!"
print_warning "Remember to test each binary before deployment" 
```


# File: install.sh
```bash
#!/bin/bash

# GhostWire Installation Script
# Supports on-demand feature compilation and installation

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${CYAN}================================${NC}"
    echo -e "${CYAN}  GHOSTWIRE INSTALLATION SCRIPT${NC}"
    echo -e "${CYAN}================================${NC}"
}

# Function to show help
show_help() {
    echo "Usage: $0 [OPTIONS]"
    echo ""
    echo "Options:"
    echo "  --minimal          Build minimal version (core only)"
    echo "  --cli              Build with CLI interface"
    echo "  --tui              Build with TUI interface"
    echo "  --web              Build with web interface"
    echo "  --mesh             Build with mesh networking"
    echo "  --matrix           Build with Matrix bridge"
    echo "  --meshtastic       Build with Meshtastic bridge"
    echo "  --security         Build with advanced security"
    echo "  --full             Build with all features"
    echo "  --features FEAT    Build with specific features (comma-separated)"
    echo "  --install          Install to system (requires sudo)"
    echo "  --clean            Clean previous builds"
    echo "  --help             Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 --minimal                    # Minimal build"
    echo "  $0 --features cli,tui,mesh      # Custom feature set"
    echo "  $0 --full --install             # Full build and install"
    echo "  $0 --tui --install              # TUI build and install"
}

# Function to check dependencies
check_dependencies() {
    print_status "Checking system dependencies..."
    
    # Check for Rust
    if ! command -v cargo &> /dev/null; then
        print_error "Rust/Cargo not found. Please install Rust first:"
        echo "  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh"
        exit 1
    fi
    
    # Check for required system packages
    local missing_packages=()
    
    if ! pkg-config --exists openssl 2>/dev/null; then
        missing_packages+=("libssl-dev")
    fi
    
    if ! pkg-config --exists pkg-config 2>/dev/null; then
        missing_packages+=("pkg-config")
    fi
    
    if [ ${#missing_packages[@]} -ne 0 ]; then
        print_warning "Missing system packages: ${missing_packages[*]}"
        echo "Please install them with your package manager:"
        echo "  Ubuntu/Debian: sudo apt install ${missing_packages[*]}"
        echo "  CentOS/RHEL: sudo yum install ${missing_packages[*]}"
        echo "  macOS: brew install ${missing_packages[*]}"
    fi
    
    print_success "Dependencies check completed"
}

# Function to build with features
build_with_features() {
    local features="$1"
    local profile="$2"
    
    print_status "Building with features: $features"
    print_status "Profile: $profile"
    
    # Clean previous build
    if [ "$CLEAN_BUILD" = true ]; then
        print_status "Cleaning previous build..."
        cargo clean
    fi
    
    # Build command
    local build_cmd="cargo build"
    
    if [ "$profile" = "release" ]; then
        build_cmd="$build_cmd --release"
    fi
    
    if [ -n "$features" ]; then
        if [ "$features" = "minimal" ]; then
            build_cmd="$build_cmd --no-default-features --features minimal"
        else
            build_cmd="$build_cmd --features $features"
        fi
    fi
    
    print_status "Running: $build_cmd"
    
    # Execute build
    if eval "$build_cmd"; then
        print_success "Build completed successfully"
        
        # Show binary size
        if [ -f "target/$profile/ghostwire" ]; then
            local size=$(du -h "target/$profile/ghostwire" | cut -f1)
            print_status "Binary size: $size"
        fi
    else
        print_error "Build failed"
        exit 1
    fi
}

# Function to install binary
install_binary() {
    if [ "$INSTALL" != true ]; then
        return
    fi
    
    print_status "Installing GhostWire to system..."
    
    local binary_path="target/release/ghostwire"
    local install_path="/usr/local/bin/ghostwire"
    
    if [ ! -f "$binary_path" ]; then
        print_error "Binary not found at $binary_path"
        print_error "Please build first with --release"
        exit 1
    fi
    
    # Check if we have sudo access
    if ! sudo -n true 2>/dev/null; then
        print_warning "Sudo access required for installation"
        echo "Please enter your password when prompted"
    fi
    
    # Install binary
    sudo cp "$binary_path" "$install_path"
    sudo chmod +x "$install_path"
    
    print_success "GhostWire installed to $install_path"
    
    # Create configuration directory
    sudo mkdir -p /etc/ghostwire
    if [ ! -f "/etc/ghostwire/config.toml" ]; then
        sudo cp ghostwire.toml.example /etc/ghostwire/config.toml
        print_status "Configuration template installed to /etc/ghostwire/config.toml"
    fi
    
    # Create user configuration directory
    mkdir -p ~/.config/ghostwire
    if [ ! -f ~/.config/ghostwire/config.toml ]; then
        cp ghostwire.toml.example ~/.config/ghostwire/config.toml
        print_status "User configuration template installed to ~/.config/ghostwire/config.toml"
    fi
}

# Function to create different builds
create_builds() {
    print_status "Creating optimized builds..."
    
    local build_dir="builds"
    mkdir -p "$build_dir"
    
    # Minimal build
    if [ "$BUILD_MINIMAL" = true ]; then
        build_with_features "minimal" "release"
        cp target/release/ghostwire "$build_dir/ghostwire-minimal"
        print_success "Minimal build: $build_dir/ghostwire-minimal"
    fi
    
    # CLI build
    if [ "$BUILD_CLI" = true ]; then
        build_with_features "cli" "release"
        cp target/release/ghostwire "$build_dir/ghostwire-cli"
        print_success "CLI build: $build_dir/ghostwire-cli"
    fi
    
    # TUI build
    if [ "$BUILD_TUI" = true ]; then
        build_with_features "tui" "release"
        cp target/release/ghostwire "$build_dir/ghostwire-tui"
        print_success "TUI build: $build_dir/ghostwire-tui"
    fi
    
    # Web build
    if [ "$BUILD_WEB" = true ]; then
        build_with_features "web" "release"
        cp target/release/ghostwire "$build_dir/ghostwire-web"
        print_success "Web build: $build_dir/ghostwire-web"
    fi
    
    # Mesh build
    if [ "$BUILD_MESH" = true ]; then
        build_with_features "mesh" "release"
        cp target/release/ghostwire "$build_dir/ghostwire-mesh"
        print_success "Mesh build: $build_dir/ghostwire-mesh"
    fi
    
    # Full build
    if [ "$BUILD_FULL" = true ]; then
        build_with_features "" "release"
        cp target/release/ghostwire "$build_dir/ghostwire-full"
        print_success "Full build: $build_dir/ghostwire-full"
    fi
    
    # Show all builds
    if [ -d "$build_dir" ]; then
        echo ""
        print_status "Available builds:"
        ls -lh "$build_dir"/
    fi
}

# Main script
main() {
    print_header
    
    # Parse command line arguments
    BUILD_MINIMAL=false
    BUILD_CLI=false
    BUILD_TUI=false
    BUILD_WEB=false
    BUILD_MESH=false
    BUILD_MATRIX=false
    BUILD_MESHTASTIC=false
    BUILD_SECURITY=false
    BUILD_FULL=false
    INSTALL=false
    CLEAN_BUILD=false
    CUSTOM_FEATURES=""
    
    while [[ $# -gt 0 ]]; do
        case $1 in
            --minimal)
                BUILD_MINIMAL=true
                shift
                ;;
            --cli)
                BUILD_CLI=true
                shift
                ;;
            --tui)
                BUILD_TUI=true
                shift
                ;;
            --web)
                BUILD_WEB=true
                shift
                ;;
            --mesh)
                BUILD_MESH=true
                shift
                ;;
            --matrix)
                BUILD_MATRIX=true
                shift
                ;;
            --meshtastic)
                BUILD_MESHTASTIC=true
                shift
                ;;
            --security)
                BUILD_SECURITY=true
                shift
                ;;
            --full)
                BUILD_FULL=true
                shift
                ;;
            --features)
                CUSTOM_FEATURES="$2"
                shift 2
                ;;
            --install)
                INSTALL=true
                shift
                ;;
            --clean)
                CLEAN_BUILD=true
                shift
                ;;
            --help)
                show_help
                exit 0
                ;;
            *)
                print_error "Unknown option: $1"
                show_help
                exit 1
                ;;
        esac
    done
    
    # If no specific build requested, default to full
    if [ "$BUILD_MINIMAL" = false ] && [ "$BUILD_CLI" = false ] && [ "$BUILD_TUI" = false ] && [ "$BUILD_WEB" = false ] && [ "$BUILD_MESH" = false ] && [ "$BUILD_MATRIX" = false ] && [ "$BUILD_MESHTASTIC" = false ] && [ "$BUILD_SECURITY" = false ] && [ "$BUILD_FULL" = false ] && [ -z "$CUSTOM_FEATURES" ]; then
        BUILD_FULL=true
    fi
    
    # Check dependencies
    check_dependencies
    
    # Build based on options
    if [ -n "$CUSTOM_FEATURES" ]; then
        build_with_features "$CUSTOM_FEATURES" "release"
        install_binary
    elif [ "$BUILD_FULL" = true ]; then
        build_with_features "" "release"
        install_binary
    else
        create_builds
        if [ "$INSTALL" = true ]; then
            # Install the last built binary (usually the most feature-rich)
            install_binary
        fi
    fi
    
    print_success "Installation completed successfully!"
    echo ""
    echo "Next steps:"
    echo "1. Configure GhostWire: edit ~/.config/ghostwire/config.toml"
    echo "2. Start GhostWire: ghostwire --tui"
    echo "3. View help: ghostwire --help"
    echo ""
    echo "For more information, see the documentation at:"
    echo "  https://github.com/ghostwire/ghostwire"
}

# Run main function
main "$@" 
```


# File: start-backend.sh
```bash
#!/bin/bash

# GhostWire Backend Launcher
# This script starts the backend server for cross-PC communication

set -e

# Colors for output
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}🚀 Starting GhostWire Backend Server${NC}"

# Configuration
BACKEND_HOST=${BACKEND_HOST:-"0.0.0.0"}
BACKEND_PORT=${BACKEND_PORT:-3001}

# Check if we have a pre-built binary
if [ -f "target/release/ghostwire" ]; then
    echo -e "${GREEN}✓ Found pre-built binary${NC}"
    echo -e "${YELLOW}Starting backend on ${BACKEND_HOST}:${BACKEND_PORT}${NC}"
    ./target/release/ghostwire web --host $BACKEND_HOST --port $BACKEND_PORT
elif [ -f "target/debug/ghostwire" ]; then
    echo -e "${GREEN}✓ Found debug binary${NC}"
    echo -e "${YELLOW}Starting backend on ${BACKEND_HOST}:${BACKEND_PORT}${NC}"
    ./target/debug/ghostwire web --host $BACKEND_HOST --port $BACKEND_PORT
else
    echo -e "${RED}❌ No binary found. Attempting to build...${NC}"
    
    # Try to fix rustup issue
    export RUSTUP_TOOLCHAIN="stable"
    unset CARGO_PROXY
    unset RUSTUP_PROXY
    
    # Try building with system rustc
    if command -v rustc >/dev/null 2>&1; then
        echo -e "${YELLOW}Using system rustc...${NC}"
        RUSTC=$(which rustc) cargo build --release
    else
        echo -e "${RED}❌ Rust compiler not found${NC}"
        echo -e "${YELLOW}Please install Rust: https://rustup.rs/${NC}"
        exit 1
    fi
    
    if [ -f "target/release/ghostwire" ]; then
        echo -e "${GREEN}✓ Build successful${NC}"
        echo -e "${YELLOW}Starting backend on ${BACKEND_HOST}:${BACKEND_PORT}${NC}"
        ./target/release/ghostwire web --host $BACKEND_HOST --port $BACKEND_PORT
    else
        echo -e "${RED}❌ Build failed${NC}"
        echo -e "${YELLOW}Starting mock backend server...${NC}"
        
        # Start a simple mock server for testing
        python3 -m http.server $BACKEND_PORT --bind $BACKEND_HOST &
        MOCK_PID=$!
        echo -e "${GREEN}✓ Mock server started on ${BACKEND_HOST}:${BACKEND_PORT}${NC}"
        echo -e "${YELLOW}Press Ctrl+C to stop${NC}"
        trap "kill $MOCK_PID" EXIT
        wait $MOCK_PID
    fi
fi 
```


# File: tests/api_integration.rs
```rust
/// Integration test: send a message via /api/send_message and check response.
#[tokio::test]
async fn test_send_message_api() {
    // TODO: Start server in test mode (or assume running)
    let client = reqwest::Client::new();
    let res = client.post("http://localhost:8080/api/send_message")
        .json(&serde_json::json!({
            "recipient": "peer1",
            "message": "Hello, GhostWire!"
        }))
        .send()
        .await
        .expect("Failed to send request");
    assert!(res.status().is_success());
    // TODO: Check response body for success
} 
```
