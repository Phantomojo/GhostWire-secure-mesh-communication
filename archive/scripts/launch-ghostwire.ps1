#!/usr/bin/env pwsh

# 🌐 GhostWire - Complete System Launcher (PowerShell)
# Starts backend, frontend, and TUI components with proper error handling

param(
    [switch]$BackendOnly,
    [switch]$FrontendOnly,
    [switch]$TuiOnly,
    [int]$BackendPort = 3001,
    [int]$FrontendPort = 5173,
    [string]$BackendHost = "0.0.0.0",
    [switch]$Help
)

# Colors for output
$Red = "`e[91m"
$Green = "`e[92m"
$Yellow = "`e[93m"
$Blue = "`e[94m"
$Cyan = "`e[96m"
$Purple = "`e[95m"
$NC = "`e[0m"

# ASCII Art
Write-Host $Cyan
@"
██╗    ██╗ ██████╗ ███████╗██╗  ██╗███████╗██╗    ██╗██╗██████╗ ███████╗
██║    ██║██╔════╝ ██╔════╝██║  ██║██╔════╝██║    ██║██║██╔══██╗██╔════╝
██║ █╗ ██║██║  ███╗███████╗███████║█████╗  ██║ █╗ ██║██║██████╔╝█████╗  
██║███╗██║██║   ██║╚════██║██╔══██║██╔══╝  ██║███╗██║██║██╔══██╗██╔══╝  
╚███╔███╔╝╚██████╔╝███████║██║  ██║██║     ╚███╔███╔╝██║██║  ██║███████╗
 ╚══╝╚══╝  ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝      ╚══╝╚══╝ ╚═╝╚═╝  ╚═╝╚══════╝
"@
Write-Host $NC

Write-Host "$Green🌐 GhostWire - Complete System Launcher$NC"
Write-Host "$YellowStarting backend, frontend, and TUI components...$NC"
Write-Host ""

# Show help if requested
if ($Help) {
    Write-Host "Usage: .\launch-ghostwire.ps1 [OPTIONS]"
    Write-Host ""
    Write-Host "Options:"
    Write-Host "  -BackendOnly     Start only the backend"
    Write-Host "  -FrontendOnly    Start only the frontend"
    Write-Host "  -TuiOnly         Start only the TUI"
    Write-Host "  -BackendPort N   Set backend port (default: 3001)"
    Write-Host "  -FrontendPort N  Set frontend port (default: 5173)"
    Write-Host "  -BackendHost H   Set backend host (default: 0.0.0.0)"
    Write-Host "  -Help            Show this help message"
    Write-Host ""
    Write-Host "Environment variables:"
    Write-Host "  BACKEND_PORT       Backend port (default: 3001)"
    Write-Host "  FRONTEND_PORT      Frontend port (default: 5173)"
    Write-Host "  BACKEND_HOST       Backend host (default: 0.0.0.0)"
    exit 0
}

# Determine launch mode
$LaunchMode = "all"
if ($BackendOnly) { $LaunchMode = "backend-only" }
if ($FrontendOnly) { $LaunchMode = "frontend-only" }
if ($TuiOnly) { $LaunchMode = "tui-only" }

# Function to check if command exists
function Test-Command {
    param([string]$Command)
    return [bool](Get-Command $Command -ErrorAction SilentlyContinue)
}

# Function to check if port is available
function Test-Port {
    param([int]$Port)
    try {
        $connection = New-Object System.Net.Sockets.TcpClient
        $connection.Connect("localhost", $Port)
        $connection.Close()
        return $false
    }
    catch {
        return $true
    }
}

# Function to find available port
function Find-AvailablePort {
    param([int]$StartPort)
    $port = $StartPort
    while (-not (Test-Port $port)) {
        $port++
        if ($port -gt ($StartPort + 100)) {
            Write-Host "$RedError: Could not find available port starting from $StartPort$NC" -ForegroundColor Red
            exit 1
        }
    }
    return $port
}

# Function to wait for service to be ready
function Wait-ForService {
    param([string]$Url, [int]$MaxAttempts = 30)
    
    Write-Host "$BlueWaiting for service at $Url...$NC"
    
    for ($attempt = 1; $attempt -le $MaxAttempts; $attempt++) {
        try {
            $response = Invoke-WebRequest -Uri $Url -UseBasicParsing -TimeoutSec 5
            if ($response.StatusCode -eq 200) {
                Write-Host "$Green✓ Service ready at $Url$NC"
                return $true
            }
        }
        catch {
            # Service not ready yet
        }
        
        Write-Host "." -NoNewline
        Start-Sleep -Seconds 1
    }
    
    Write-Host "$Red✗ Service failed to start at $Url$NC" -ForegroundColor Red
    return $false
}

# Function to update frontend API configuration
function Update-FrontendConfig {
    param([int]$BackendPort)
    
    $apiFile = "webui\src\services\api.ts"
    if (Test-Path $apiFile) {
        Write-Host "$BlueUpdating frontend API configuration...$NC"
        $content = Get-Content $apiFile -Raw
        $content = $content -replace 'const API_BASE_URL = .*', "const API_BASE_URL = 'http://localhost:$BackendPort/api';"
        Set-Content $apiFile $content
        Write-Host "$Green✓ Frontend API configuration updated$NC"
    }
    else {
        Write-Host "$YellowWarning: Frontend API file not found at $apiFile$NC"
    }
}

# Check if ports are available
if ($LaunchMode -ne "frontend-only" -and $LaunchMode -ne "tui-only") {
    if (-not (Test-Port $BackendPort)) {
        Write-Host "$YellowPort $BackendPort is in use, finding available port...$NC"
        $BackendPort = Find-AvailablePort $BackendPort
        Write-Host "$BlueUsing backend port: $BackendPort$NC"
    }
}

if ($LaunchMode -ne "backend-only" -and $LaunchMode -ne "tui-only") {
    if (-not (Test-Port $FrontendPort)) {
        Write-Host "$YellowPort $FrontendPort is in use, finding available port...$NC"
        $FrontendPort = Find-AvailablePort $FrontendPort
        Write-Host "$BlueUsing frontend port: $FrontendPort$NC"
    }
}

# Global variables for process tracking
$Global:BackendProcess = $null
$Global:FrontendProcess = $null
$Global:TuiProcess = $null

# Function to start backend
function Start-Backend {
    Write-Host "$BlueStarting GhostWire backend...$NC"
    
    # Check if Rust is installed
    if (-not (Test-Command "cargo")) {
        Write-Host "$RedError: Cargo not found. Please install Rust first.$NC" -ForegroundColor Red
        Write-Host "$YellowVisit: https://rustup.rs/$NC"
        exit 1
    }
    
    # Check if ghostwire directory exists
    if (-not (Test-Path "ghostwire")) {
        Write-Host "$RedError: ghostwire directory not found$NC" -ForegroundColor Red
        exit 1
    }
    
    # Build backend if needed
    Write-Host "$BlueBuilding backend...$NC"
    Push-Location "ghostwire"
    try {
        cargo build --quiet
        if ($LASTEXITCODE -ne 0) {
            Write-Host "$RedError: Backend build failed$NC" -ForegroundColor Red
            exit 1
        }
    }
    finally {
        Pop-Location
    }
    
    # Start backend
    Write-Host "$BlueStarting backend on $BackendHost`:$BackendPort...$NC"
    Push-Location "ghostwire"
    $Global:BackendProcess = Start-Process -FilePath "cargo" -ArgumentList "run", "--", "--host", $BackendHost, "--port", $BackendPort -PassThru -WindowStyle Normal
    Pop-Location
    
    # Wait for backend to start
    Start-Sleep -Seconds 3
    
    # Check if backend is running
    if ($Global:BackendProcess.HasExited) {
        Write-Host "$RedError: Backend failed to start$NC" -ForegroundColor Red
        exit 1
    }
    
    Write-Host "$Green✓ Backend started (PID: $($Global:BackendProcess.Id))$NC"
    
    # Wait for backend API to be ready
    if (-not (Wait-ForService "http://localhost:$BackendPort/api/health" 30)) {
        Write-Host "$YellowWarning: Backend API not responding, but process is running$NC"
    }
}

# Function to start frontend
function Start-Frontend {
    Write-Host "$BlueStarting GhostWire frontend...$NC"
    
    # Check if Node.js is installed
    if (-not (Test-Command "npm")) {
        Write-Host "$RedError: npm not found. Please install Node.js first.$NC" -ForegroundColor Red
        Write-Host "$YellowVisit: https://nodejs.org/$NC"
        exit 1
    }
    
    # Check if webui directory exists
    if (-not (Test-Path "webui")) {
        Write-Host "$RedError: webui directory not found$NC" -ForegroundColor Red
        exit 1
    }
    
    # Install frontend dependencies if needed
    if (-not (Test-Path "webui\node_modules")) {
        Write-Host "$BlueInstalling frontend dependencies...$NC"
        Push-Location "webui"
        npm install
        Pop-Location
    }
    
    # Update frontend configuration
    Update-FrontendConfig $BackendPort
    
    # Start frontend
    Write-Host "$BlueStarting frontend on port $FrontendPort...$NC"
    Push-Location "webui"
    $Global:FrontendProcess = Start-Process -FilePath "npm" -ArgumentList "run", "dev", "--", "--port", $FrontendPort -PassThru -WindowStyle Normal
    Pop-Location
    
    # Wait for frontend to start
    Start-Sleep -Seconds 5
    
    # Check if frontend is running
    if ($Global:FrontendProcess.HasExited) {
        Write-Host "$RedError: Frontend failed to start$NC" -ForegroundColor Red
        exit 1
    }
    
    Write-Host "$Green✓ Frontend started (PID: $($Global:FrontendProcess.Id))$NC"
    
    # Wait for frontend to be ready
    if (-not (Wait-ForService "http://localhost:$FrontendPort" 30)) {
        Write-Host "$YellowWarning: Frontend not responding, but process is running$NC"
    }
}

# Function to start TUI
function Start-Tui {
    Write-Host "$BlueStarting GhostWire TUI...$NC"
    
    # Check if backend is running
    if ($Global:BackendProcess -and -not $Global:BackendProcess.HasExited) {
        Write-Host "$Green✓ Backend is running, TUI can connect$NC"
    }
    else {
        Write-Host "$YellowWarning: Backend not running, TUI may not work properly$NC"
    }
    
    # Start TUI in a new terminal
    Write-Host "$BlueStarting TUI in new terminal...$NC"
    Push-Location "ghostwire"
    $Global:TuiProcess = Start-Process -FilePath "cargo" -ArgumentList "run", "--bin", "ghostwire", "--", "tui" -PassThru -WindowStyle Normal
    Pop-Location
    
    Write-Host "$Green✓ TUI started (PID: $($Global:TuiProcess.Id))$NC"
}

# Function to cleanup on exit
function Stop-AllServices {
    Write-Host "`n$YellowShutting down GhostWire...$NC"
    
    if ($Global:BackendProcess -and -not $Global:BackendProcess.HasExited) {
        Write-Host "$BlueStopping backend (PID: $($Global:BackendProcess.Id))...$NC"
        Stop-Process -Id $Global:BackendProcess.Id -Force -ErrorAction SilentlyContinue
    }
    
    if ($Global:FrontendProcess -and -not $Global:FrontendProcess.HasExited) {
        Write-Host "$BlueStopping frontend (PID: $($Global:FrontendProcess.Id))...$NC"
        Stop-Process -Id $Global:FrontendProcess.Id -Force -ErrorAction SilentlyContinue
    }
    
    if ($Global:TuiProcess -and -not $Global:TuiProcess.HasExited) {
        Write-Host "$BlueStopping TUI (PID: $($Global:TuiProcess.Id))...$NC"
        Stop-Process -Id $Global:TuiProcess.Id -Force -ErrorAction SilentlyContinue
    }
    
    Write-Host "$Green✓ GhostWire shutdown complete$NC"
    exit 0
}

# Set up signal handlers
Register-EngineEvent PowerShell.Exiting -Action { Stop-AllServices }

# Start components based on launch mode
switch ($LaunchMode) {
    "backend-only" { Start-Backend }
    "frontend-only" { Start-Frontend }
    "tui-only" { Start-Tui }
    "all" {
        Start-Backend
        Start-Frontend
        Start-Tui
    }
}

# Print status information
Write-Host ""
Write-Host "$Green==============================$NC"
Write-Host "$Green✓ GhostWire started successfully!$NC"
if ($LaunchMode -ne "frontend-only" -and $LaunchMode -ne "tui-only") {
    Write-Host "$CyanBackend API:   http://localhost:$BackendPort/api$NC"
}
if ($LaunchMode -ne "backend-only" -and $LaunchMode -ne "tui-only") {
    Write-Host "$CyanFrontend UI:   http://localhost:$FrontendPort$NC"
}
if ($LaunchMode -ne "backend-only" -and $LaunchMode -ne "frontend-only") {
    Write-Host "$CyanTUI:           Running in terminal$NC"
}
Write-Host "$Green==============================$NC"
Write-Host ""
Write-Host "$YellowPress Ctrl+C to stop all services$NC"
Write-Host ""

# Keep script running and monitor processes
try {
    while ($true) {
        # Check if any process has died
        if ($Global:BackendProcess -and $Global:BackendProcess.HasExited) {
            Write-Host "$RedBackend process died unexpectedly$NC" -ForegroundColor Red
            Stop-AllServices
        }
        
        if ($Global:FrontendProcess -and $Global:FrontendProcess.HasExited) {
            Write-Host "$RedFrontend process died unexpectedly$NC" -ForegroundColor Red
            Stop-AllServices
        }
        
        if ($Global:TuiProcess -and $Global:TuiProcess.HasExited) {
            Write-Host "$YellowTUI process ended$NC"
            $Global:TuiProcess = $null
        }
        
        Start-Sleep -Seconds 5
    }
}
catch {
    Stop-AllServices
} 