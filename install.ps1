# GhostWire Installer for Windows
# PowerShell Script - Run as Administrator for system-wide installation
# Version: 1.0.0

param(
    [switch]$System,
    [switch]$User,
    [string]$InstallDir = "",
    [switch]$Service,
    [switch]$NoService,
    [switch]$Silent,
    [string]$Username = "",
    [switch]$Help
)

# Script configuration
$ErrorActionPreference = "Stop"
$ProgressPreference = "SilentlyContinue"

# Colors for output
function Write-Info { Write-Host "[INFO] $args" -ForegroundColor Blue }
function Write-Success { Write-Host "[✓] $args" -ForegroundColor Green }
function Write-Warning { Write-Host "[!] $args" -ForegroundColor Yellow }
function Write-Error { Write-Host "[✗] $args" -ForegroundColor Red }

# Banner
function Show-Banner {
    Write-Host @"

╔═══════════════════════════════════════════════════════════════╗
║                                                               ║
║   ██████╗ ██╗  ██╗ ██████╗ ███████╗████████╗██╗    ██╗██╗██████╗ ███████╗  ║
║  ██╔════╝ ██║  ██║██╔═══██╗██╔════╝╚══██╔══╝██║    ██║██║██╔══██╗██╔════╝  ║
║  ██║  ███╗███████║██║   ██║███████╗   ██║   ██║ █╗ ██║██║██████╔╝█████╗    ║
║  ██║   ██║██╔══██║██║   ██║╚════██║   ██║   ██║███╗██║██║██╔══██╗██╔══╝    ║
║  ╚██████╔╝██║  ██║╚██████╔╝███████║   ██║   ╚███╔███╔╝██║██║  ██║███████╗  ║
║   ╚═════╝ ╚═╝  ╚═╝ ╚═════╝ ╚══════╝   ╚═╝    ╚══╝╚══╝ ╚═╝╚═╝  ╚═╝╚══════╝  ║
║                                                               ║
║              Secure Mesh Networking & Communication          ║
║                    Windows Installer v1.0                    ║
╚═══════════════════════════════════════════════════════════════╝

"@ -ForegroundColor Cyan
}

# Check if running as Administrator
function Test-Administrator {
    $currentUser = New-Object Security.Principal.WindowsPrincipal([Security.Principal.WindowsIdentity]::GetCurrent())
    return $currentUser.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

# Check dependencies
function Test-Dependencies {
    Write-Info "Checking dependencies..."
    
    $missing = @()
    
    # Check Rust
    if (Get-Command rustc -ErrorAction SilentlyContinue) {
        $rustVersion = rustc --version
        Write-Success "Rust found: $rustVersion"
    } else {
        Write-Warning "Rust not found"
        $missing += "rust"
    }
    
    # Check Cargo
    if (Get-Command cargo -ErrorAction SilentlyContinue) {
        $cargoVersion = cargo --version
        Write-Success "Cargo found: $cargoVersion"
    } else {
        Write-Warning "Cargo not found"
        $missing += "cargo"
    }
    
    # Check Node.js
    if (Get-Command node -ErrorAction SilentlyContinue) {
        $nodeVersion = node --version
        Write-Success "Node.js found: $nodeVersion"
    } else {
        Write-Warning "Node.js not found"
        $missing += "node"
    }
    
    # Check npm
    if (Get-Command npm -ErrorAction SilentlyContinue) {
        $npmVersion = npm --version
        Write-Success "npm found: $npmVersion"
    } else {
        Write-Warning "npm not found"
        $missing += "npm"
    }
    
    if ($missing.Count -gt 0) {
        Write-Error "Missing dependencies: $($missing -join ', ')"
        Write-Host ""
        Write-Info "Would you like to install missing dependencies? [Y/N]"
        
        if (-not $Silent) {
            $response = Read-Host
            if ($response -match '^[Yy]$') {
                Install-Dependencies
            } else {
                Write-Error "Please install dependencies manually and re-run this installer."
                Write-Info "Run: .\scripts\install-deps.ps1"
                exit 1
            }
        } else {
            Write-Error "Silent mode: Cannot install dependencies automatically."
            Write-Info "Please run: .\scripts\install-deps.ps1"
            exit 1
        }
    }
}

# Get installation directory
function Get-InstallDirectory {
    if ($InstallDir) {
        return $InstallDir
    }
    
    if ($System) {
        return "$env:ProgramFiles\GhostWire"
    } else {
        return "$env:LOCALAPPDATA\GhostWire"
    }
}

# Prompt for username
function Get-UserName {
    if ($Username) {
        return $Username
    }
    
    if (-not $Silent) {
        Write-Host ""
        Write-Info "Enter your username for GhostWire (e.g., alice, bob@home, node-alpha)"
        $user = Read-Host "Username"
        
        if ([string]::IsNullOrWhiteSpace($user)) {
            Write-Warning "No username provided. Using default: $env:USERNAME-node"
            return "$env:USERNAME-node"
        }
        return $user
    } else {
        return "$env:USERNAME-node"
    }
}

# Build backend
function Build-Backend {
    Write-Info "Building GhostWire backend (Rust)..."
    Write-Info "This may take several minutes on first build..."
    
    Push-Location ghostwire
    
    try {
        if (Test-Path "vendor") {
            Write-Info "Using vendored dependencies..."
            $env:CARGO_HOME = "$(Get-Location)\.cargo"
            cargo build --release --offline
        } else {
            cargo build --release
        }
        
        Write-Success "Backend built successfully"
    } catch {
        Write-Error "Backend build failed: $_"
        exit 1
    } finally {
        Pop-Location
    }
}

# Setup frontend
function Setup-Frontend {
    Write-Info "Setting up GhostWire frontend (Node.js)..."
    
    Push-Location webui
    
    try {
        npm install --silent
        Write-Success "Frontend setup complete"
    } catch {
        Write-Error "Frontend setup failed: $_"
        exit 1
    } finally {
        Pop-Location
    }
}

# Create installation directories
function New-InstallDirectories {
    param([string]$baseDir)
    
    Write-Info "Creating installation directories..."
    
    New-Item -ItemType Directory -Force -Path "$baseDir\bin" | Out-Null
    New-Item -ItemType Directory -Force -Path "$baseDir\config" | Out-Null
    New-Item -ItemType Directory -Force -Path "$baseDir\data" | Out-Null
    New-Item -ItemType Directory -Force -Path "$baseDir\logs" | Out-Null
    New-Item -ItemType Directory -Force -Path "$baseDir\webui" | Out-Null
    New-Item -ItemType Directory -Force -Path "$baseDir\scripts" | Out-Null
    New-Item -ItemType Directory -Force -Path "$baseDir\docs" | Out-Null
    
    Write-Success "Directories created"
}

# Copy files
function Copy-InstallFiles {
    param([string]$baseDir)
    
    Write-Info "Copying files to installation directory..."
    
    # Copy backend binary
    Copy-Item "ghostwire\target\release\ghostwire.exe" "$baseDir\bin\" -Force
    
    # Copy frontend
    Copy-Item "webui\*" "$baseDir\webui\" -Recurse -Force
    
    # Copy scripts
    Copy-Item "scripts\*" "$baseDir\scripts\" -Recurse -Force
    
    # Copy configs if they exist
    if (Test-Path "config") {
        Copy-Item "config\*" "$baseDir\config\" -Recurse -Force
    }
    
    # Copy documentation
    if (Test-Path "README.md") { Copy-Item "README.md" "$baseDir\docs\" -Force }
    if (Test-Path "INSTALL.md") { Copy-Item "INSTALL.md" "$baseDir\docs\" -Force }
    
    Write-Success "Files copied"
}

# Create configuration files
function New-ConfigFiles {
    param(
        [string]$baseDir,
        [string]$userName
    )
    
    Write-Info "Creating configuration files..."
    
    # Identity config
    $identityConfig = @"
# GhostWire Identity Configuration
# Generated on $(Get-Date)

[identity]
username = "$userName"
display_name = "$userName"

# Note: Keypair will be auto-generated on first run
# Public key and PeerID will be saved here automatically
"@
    $identityConfig | Out-File "$baseDir\config\identity.toml" -Encoding UTF8
    
    # Network config
    $networkConfig = @"
# GhostWire Network Configuration

[network]
# Listen address for P2P
listen_addr = "/ip4/0.0.0.0/tcp/4001"

# Enable NAT traversal for internet connectivity
enable_relay = true
enable_autonat = true
enable_dcutr = true

# Relay servers for NAT traversal
relay_servers = []

# Bootstrap nodes for peer discovery across the internet
bootstrap_nodes = [
    "/dnsaddr/bootstrap.libp2p.io/p2p/QmNnooDu7bfjPFoTZYxMNLWUQJyrVwtbZg5gBMjTezGAJN",
    "/dnsaddr/bootstrap.libp2p.io/p2p/QmQCU2EcMqAqQPR2i9bChDtGNJchTbq5TbXJJ16u19uLTa",
]

# STUN servers for NAT hole punching
stun_servers = ["stun:stun.l.google.com:19302"]

[web]
# Web UI host and port
host = "0.0.0.0"
port = 3001

[security]
# Security settings
enable_encryption = true
enable_stealth = true
"@
    $networkConfig | Out-File "$baseDir\config\network.toml" -Encoding UTF8
    
    Write-Success "Configuration files created"
}

# Create launcher scripts
function New-LauncherScripts {
    param([string]$baseDir)
    
    Write-Info "Creating launcher scripts..."
    
    # PowerShell launcher
    $psLauncher = @"
# GhostWire Launcher Script
`$installDir = Split-Path -Parent `$PSScriptRoot

# Set environment
`$env:GHOSTWIRE_CONFIG_DIR = "`$installDir\config"
`$env:GHOSTWIRE_DATA_DIR = "`$installDir\data"
`$env:GHOSTWIRE_LOG_DIR = "`$installDir\logs"

# Launch backend
& "`$installDir\bin\ghostwire.exe" --config "`$installDir\config\network.toml" `$args
"@
    $psLauncher | Out-File "$baseDir\bin\ghostwire-launch.ps1" -Encoding UTF8
    
    # Batch launcher
    $batLauncher = @"
@echo off
set GHOSTWIRE_CONFIG_DIR=%~dp0..\config
set GHOSTWIRE_DATA_DIR=%~dp0..\data
set GHOSTWIRE_LOG_DIR=%~dp0..\logs
"%~dp0ghostwire.exe" --config "%~dp0..\config\network.toml" %*
"@
    $batLauncher | Out-File "$baseDir\bin\ghostwire-launch.bat" -Encoding ASCII
    
    Write-Success "Launcher scripts created"
}

# Add to PATH
function Add-ToPath {
    param([string]$baseDir)
    
    Write-Info "Adding GhostWire to PATH..."
    
    $binPath = "$baseDir\bin"
    $userPath = [Environment]::GetEnvironmentVariable("Path", "User")
    
    if ($userPath -notlike "*$binPath*") {
        [Environment]::SetEnvironmentVariable("Path", "$userPath;$binPath", "User")
        $env:Path += ";$binPath"
        Write-Success "Added to PATH"
        Write-Warning "You may need to restart your terminal for PATH changes to take effect"
    } else {
        Write-Info "Already in PATH"
    }
}

# Add Windows Firewall rules
function Add-FirewallRules {
    Write-Info "Adding Windows Firewall rules..."
    
    try {
        # Allow inbound on port 4001 (P2P)
        New-NetFirewallRule -DisplayName "GhostWire P2P" -Direction Inbound -LocalPort 4001 -Protocol TCP -Action Allow -ErrorAction SilentlyContinue | Out-Null
        
        # Allow inbound on port 3001 (Web UI)
        New-NetFirewallRule -DisplayName "GhostWire Web UI" -Direction Inbound -LocalPort 3001 -Protocol TCP -Action Allow -ErrorAction SilentlyContinue | Out-Null
        
        Write-Success "Firewall rules added"
    } catch {
        Write-Warning "Unable to add firewall rules. You may need to do this manually."
    }
}

# Create desktop shortcut
function New-DesktopShortcut {
    param([string]$baseDir)
    
    Write-Info "Creating desktop shortcut..."
    
    $WshShell = New-Object -ComObject WScript.Shell
    $Shortcut = $WshShell.CreateShortcut("$env:USERPROFILE\Desktop\GhostWire.lnk")
    $Shortcut.TargetPath = "$baseDir\bin\ghostwire-launch.bat"
    $Shortcut.WorkingDirectory = "$baseDir"
    $Shortcut.Description = "GhostWire Secure Mesh Network"
    $Shortcut.Save()
    
    Write-Success "Desktop shortcut created"
}

# Print summary
function Show-Summary {
    param(
        [string]$installDir,
        [string]$userName
    )
    
    Write-Host ""
    Write-Host "╔═══════════════════════════════════════════════════════════════╗" -ForegroundColor Green
    Write-Host "║                                                               ║" -ForegroundColor Green
    Write-Host "║  🎉  GhostWire Installation Complete!  🎉                    ║" -ForegroundColor Green
    Write-Host "║                                                               ║" -ForegroundColor Green
    Write-Host "╚═══════════════════════════════════════════════════════════════╝" -ForegroundColor Green
    Write-Host ""
    Write-Host "📦 Installation Details:" -ForegroundColor Cyan
    Write-Host "  Location: " -NoNewline; Write-Host $installDir -ForegroundColor Green
    Write-Host "  Username: " -NoNewline; Write-Host $userName -ForegroundColor Green
    Write-Host ""
    Write-Host "🚀 Quick Start:" -ForegroundColor Cyan
    Write-Host "  1. Start GhostWire:"
    Write-Host "     $installDir\bin\ghostwire-launch.bat" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "  2. Or double-click the desktop shortcut"
    Write-Host ""
    Write-Host "🌐 Access Points:" -ForegroundColor Cyan
    Write-Host "  Web UI: " -NoNewline; Write-Host "http://localhost:3001" -ForegroundColor Green
    Write-Host "  API: " -NoNewline; Write-Host "http://localhost:3001/api" -ForegroundColor Green
    Write-Host ""
    Write-Host "🔧 Management:" -ForegroundColor Cyan
    Write-Host "  View logs: $installDir\logs" -ForegroundColor Yellow
    Write-Host "  Edit config: $installDir\config\network.toml" -ForegroundColor Yellow
    Write-Host ""
    Write-Host "💡 Note: Other nodes can discover you by your username: " -NoNewline -ForegroundColor Magenta
    Write-Host $userName -ForegroundColor Green
    Write-Host ""
}

# Show help
function Show-Help {
    Write-Host @"
GhostWire Installer for Windows

Usage: .\install.ps1 [OPTIONS]

Options:
    -System             Install system-wide (requires Administrator)
    -User               Install for current user (default)
    -InstallDir DIR     Custom installation directory
    -Service            Install as Windows Service
    -NoService          Don't install Windows Service (default)
    -Silent             Silent mode (no prompts)
    -Username NAME      Set username for identity
    -Help               Show this help message

Examples:
    # User installation (default)
    .\install.ps1

    # System-wide installation
    .\install.ps1 -System

    # Custom directory
    .\install.ps1 -InstallDir "C:\Custom\Path"

    # Silent installation
    .\install.ps1 -Silent -Username alice

"@
}

# Main installation flow
function Main {
    if ($Help) {
        Show-Help
        exit 0
    }
    
    Show-Banner
    
    Write-Info "Starting GhostWire installation..."
    Write-Host ""
    
    # Check admin if system install
    if ($System -and -not (Test-Administrator)) {
        Write-Error "System-wide installation requires Administrator privileges"
        Write-Info "Please run PowerShell as Administrator"
        exit 1
    }
    
    Test-Dependencies
    
    $installDir = Get-InstallDirectory
    $userName = Get-UserName
    
    Write-Info "Installation directory: $installDir"
    Write-Success "Username set to: $userName"
    
    Write-Host ""
    Write-Info "Building GhostWire..."
    Build-Backend
    Setup-Frontend
    
    Write-Host ""
    Write-Info "Installing GhostWire to $installDir..."
    New-InstallDirectories -baseDir $installDir
    Copy-InstallFiles -baseDir $installDir
    New-ConfigFiles -baseDir $installDir -userName $userName
    New-LauncherScripts -baseDir $installDir
    Add-ToPath -baseDir $installDir
    Add-FirewallRules
    New-DesktopShortcut -baseDir $installDir
    
    Show-Summary -installDir $installDir -userName $userName
}

# Run main
Main
