#!/usr/bin/env bash
set -e

# Colors for output
green='\033[0;32m'
red='\033[0;31m'
reset='\033[0m'

function info() {
  echo -e "${green}[INFO] $1${reset}"
}

function error() {
  echo -e "${red}[ERROR] $1${reset}" >&2
}

info "Updating system packages..."
sudo apt update && sudo apt upgrade -y

info "Installing essential build tools and utilities..."
sudo apt install -y build-essential curl wget git unzip zip tar pkg-config libssl-dev ca-certificates software-properties-common apt-transport-https

info "Installing Python 3 and pip..."
sudo apt install -y python3 python3-pip python3-venv

info "Installing Node.js (LTS) and npm..."
if ! command -v node >/dev/null 2>&1; then
  curl -fsSL https://deb.nodesource.com/setup_lts.x | sudo -E bash -
  sudo apt install -y nodejs
else
  info "Node.js already installed."
fi

info "Installing global npm tools (yarn, pnpm)..."
sudo npm install -g yarn pnpm

info "Installing Rust (via rustup)..."
if ! command -v rustup >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
  source "$HOME/.cargo/env"
else
  info "Rust already installed."
fi

info "Installing Tauri dependencies..."
sudo apt install -y libwebkit2gtk-4.0-dev libgtk-3-dev libayatana-appindicator3-dev librsvg2-dev

info "Installing Docker (optional, but recommended)..."
if ! command -v docker >/dev/null 2>&1; then
  curl -fsSL https://get.docker.com -o get-docker.sh
  sh get-docker.sh
  sudo usermod -aG docker $USER
  rm get-docker.sh
else
  info "Docker already installed."
fi

info "Installing Docker Compose..."
sudo apt install -y docker-compose

info "Installing VS Code (optional)..."
if ! command -v code >/dev/null 2>&1; then
  wget -qO- https://packages.microsoft.com/keys/microsoft.asc | gpg --dearmor > packages.microsoft.gpg
  sudo install -o root -g root -m 644 packages.microsoft.gpg /usr/share/keyrings/
  sudo sh -c 'echo "deb [arch=amd64 signed-by=/usr/share/keyrings/packages.microsoft.gpg] https://packages.microsoft.com/repos/vscode stable main" > /etc/apt/sources.list.d/vscode.list'
  sudo apt update
  sudo apt install -y code
  rm packages.microsoft.gpg
else
  info "VS Code already installed."
fi

info "All done! Please restart your terminal or run 'source ~/.cargo/env' to update your environment."
info "If you installed Docker, you may need to log out and back in for group changes to take effect." 