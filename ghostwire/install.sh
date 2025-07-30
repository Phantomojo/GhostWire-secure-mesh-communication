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