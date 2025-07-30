#!/bin/bash

# Cursor Update Script for Linux
# This script updates Cursor to the latest version

set -e  # Exit on any error

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

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to get current Cursor version
get_current_version() {
    if command_exists cursor; then
        cursor --version 2>/dev/null | head -n1 || echo "Unknown"
    else
        echo "Not installed"
    fi
}

# Function to download latest version info
get_latest_version() {
    print_status "Fetching latest Cursor version..."
    
    # Try to get latest version from GitHub releases
    LATEST_VERSION=$(curl -s https://api.github.com/repos/getcursor/cursor/releases/latest | grep '"tag_name"' | cut -d'"' -f4)
    
    if [ -z "$LATEST_VERSION" ]; then
        print_warning "Could not fetch latest version from GitHub, using fallback method..."
        # Fallback: try to get from cursor.sh website
        LATEST_VERSION=$(curl -s https://cursor.sh/ | grep -o 'v[0-9]\+\.[0-9]\+\.[0-9]\+' | head -n1)
    fi
    
    echo "$LATEST_VERSION"
}

# Function to check if update is needed
check_update_needed() {
    CURRENT_VERSION=$(get_current_version)
    LATEST_VERSION=$(get_latest_version)
    
    print_status "Current version: $CURRENT_VERSION"
    print_status "Latest version: $LATEST_VERSION"
    
    if [ "$CURRENT_VERSION" = "Not installed" ]; then
        print_warning "Cursor is not installed. This script will install it."
        return 0
    fi
    
    if [ "$CURRENT_VERSION" = "$LATEST_VERSION" ]; then
        print_success "Cursor is already up to date!"
        return 1
    else
        print_status "Update available: $CURRENT_VERSION → $LATEST_VERSION"
        return 0
    fi
}

# Function to install/update via AppImage
install_appimage() {
    print_status "Installing/updating Cursor via AppImage..."
    
    # Create directory for AppImage
    mkdir -p ~/.local/bin
    mkdir -p ~/.local/share/applications
    
    # Download latest AppImage
    DOWNLOAD_URL="https://download.cursor.sh/linux/appImage/x64"
    APPDIR="$HOME/.local/share/cursor"
    APPIMAGE="$APPDIR/cursor.AppImage"
    
    # Create app directory
    mkdir -p "$APPDIR"
    
    # Download AppImage
    print_status "Downloading Cursor AppImage..."
    curl -L -o "$APPIMAGE" "$DOWNLOAD_URL"
    chmod +x "$APPIMAGE"
    
    # Create desktop entry
    cat > ~/.local/share/applications/cursor.desktop << EOF
[Desktop Entry]
Name=Cursor
Comment=AI-first code editor
Exec=$APPIMAGE %U
Terminal=false
Type=Application
Icon=cursor
StartupWMClass=Cursor
MimeType=text/plain;inode/directory;application/x-code-workspace;
Categories=Development;TextEditor;IDE;
EOF
    
    # Create symlink in PATH
    ln -sf "$APPIMAGE" ~/.local/bin/cursor
    
    print_success "Cursor AppImage installed successfully!"
}

# Function to install/update via snap
install_snap() {
    print_status "Installing/updating Cursor via snap..."
    
    if command_exists snap; then
        sudo snap install cursor --classic
        print_success "Cursor snap installed successfully!"
    else
        print_error "Snap is not available on this system"
        return 1
    fi
}

# Function to install/update via flatpak
install_flatpak() {
    print_status "Installing/updating Cursor via flatpak..."
    
    if command_exists flatpak; then
        # Add flathub if not already added
        flatpak remote-add --if-not-exists flathub https://flathub.org/repo/flathub.flatpakrepo
        
        # Install cursor
        flatpak install --user com.cursor.Cursor -y
        print_success "Cursor flatpak installed successfully!"
    else
        print_error "Flatpak is not available on this system"
        return 1
    fi
}

# Function to install/update via package manager
install_package_manager() {
    print_status "Installing/update Cursor via package manager..."
    
    if command_exists apt; then
        # Ubuntu/Debian
        print_status "Detected apt package manager"
        
        # Add Cursor repository
        wget -qO - https://download.cursor.sh/linux/keys/public.asc | sudo apt-key add -
        echo "deb [arch=amd64] https://download.cursor.sh/linux/deb stable main" | sudo tee /etc/apt/sources.list.d/cursor.list
        
        sudo apt update
        sudo apt install cursor -y
        
    elif command_exists dnf; then
        # Fedora
        print_status "Detected dnf package manager"
        sudo dnf install cursor -y
        
    elif command_exists pacman; then
        # Arch Linux
        print_status "Detected pacman package manager"
        sudo pacman -S cursor --noconfirm
        
    else
        print_error "No supported package manager found"
        return 1
    fi
    
    print_success "Cursor installed via package manager successfully!"
}

# Function to detect current installation method
detect_installation() {
    if command_exists cursor; then
        CURSOR_PATH=$(which cursor)
        
        if [[ "$CURSOR_PATH" == *"/snap/"* ]]; then
            echo "snap"
        elif [[ "$CURSOR_PATH" == *"/flatpak/"* ]]; then
            echo "flatpak"
        elif [[ "$CURSOR_PATH" == *"/usr/bin/"* ]] || [[ "$CURSOR_PATH" == *"/usr/local/bin/"* ]]; then
            echo "package"
        elif [[ "$CURSOR_PATH" == *"/.local/bin/"* ]]; then
            echo "appimage"
        else
            echo "unknown"
        fi
    else
        echo "not_installed"
    fi
}

# Function to remove old installation
remove_old_installation() {
    local install_type=$1
    
    case $install_type in
        "snap")
            print_status "Removing old snap installation..."
            sudo snap remove cursor
            ;;
        "flatpak")
            print_status "Removing old flatpak installation..."
            flatpak uninstall com.cursor.Cursor -y
            ;;
        "package")
            print_status "Removing old package installation..."
            if command_exists apt; then
                sudo apt remove cursor -y
            elif command_exists dnf; then
                sudo dnf remove cursor -y
            elif command_exists pacman; then
                sudo pacman -R cursor --noconfirm
            fi
            ;;
        "appimage")
            print_status "Removing old AppImage installation..."
            rm -f ~/.local/bin/cursor
            rm -f ~/.local/share/applications/cursor.desktop
            rm -rf ~/.local/share/cursor
            ;;
    esac
}

# Main function
main() {
    print_status "Starting Cursor update process..."
    
    # Check if update is needed
    if ! check_update_needed; then
        exit 0
    fi
    
    # Detect current installation
    CURRENT_INSTALL=$(detect_installation)
    print_status "Current installation type: $CURRENT_INSTALL"
    
    # Ask user for preferred installation method
    echo
    print_status "Choose installation method:"
    echo "1) AppImage (recommended - portable, no system-wide installation)"
    echo "2) Snap (Ubuntu/other Linux)"
    echo "3) Flatpak (universal Linux)"
    echo "4) Package manager (apt/dnf/pacman)"
    echo "5) Auto-detect best method"
    
    read -p "Enter choice (1-5) [default: 5]: " choice
    choice=${choice:-5}
    
    # Remove old installation if different method chosen
    if [ "$choice" != "5" ] && [ "$CURRENT_INSTALL" != "not_installed" ]; then
        case $choice in
            1) NEW_METHOD="appimage" ;;
            2) NEW_METHOD="snap" ;;
            3) NEW_METHOD="flatpak" ;;
            4) NEW_METHOD="package" ;;
        esac
        
        if [ "$CURRENT_INSTALL" != "$NEW_METHOD" ]; then
            print_warning "Removing old installation to switch to new method..."
            remove_old_installation "$CURRENT_INSTALL"
        fi
    fi
    
    # Install using chosen method
    case $choice in
        1)
            install_appimage
            ;;
        2)
            install_snap
            ;;
        3)
            install_flatpak
            ;;
        4)
            install_package_manager
            ;;
        5)
            # Auto-detect best method
            if command_exists snap; then
                install_snap
            elif command_exists flatpak; then
                install_flatpak
            elif command_exists apt || command_exists dnf || command_exists pacman; then
                install_package_manager
            else
                install_appimage
            fi
            ;;
        *)
            print_error "Invalid choice"
            exit 1
            ;;
    esac
    
    # Verify installation
    print_status "Verifying installation..."
    if command_exists cursor; then
        NEW_VERSION=$(get_current_version)
        print_success "Cursor updated successfully! New version: $NEW_VERSION"
        
        # Add to PATH if not already there
        if [[ ":$PATH:" != *":$HOME/.local/bin:"* ]]; then
            print_warning "Adding ~/.local/bin to PATH..."
            echo 'export PATH="$HOME/.local/bin:$PATH"' >> ~/.bashrc
            print_status "Please restart your terminal or run: source ~/.bashrc"
        fi
    else
        print_error "Installation verification failed"
        exit 1
    fi
}

# Check if running as root
if [ "$EUID" -eq 0 ]; then
    print_error "Please do not run this script as root"
    exit 1
fi

# Check for required tools
if ! command_exists curl; then
    print_error "curl is required but not installed. Please install it first."
    exit 1
fi

# Run main function
main "$@" 