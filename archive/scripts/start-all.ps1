# 🌐 GhostWire - Windows Multi-PC Startup Script
# This script automatically configures and starts GhostWire for multi-PC communication

param(
    [int]$BackendPort = 0,
    [int]$FrontendPort = 0,
    [string]$Username = ""
)

# Colors for output
$Red = "Red"
$Green = "Green"
$Yellow = "Yellow"
$Blue = "Blue"
$Cyan = "Cyan"
$Purple = "Magenta"

function Write-ColorOutput {
    param(
        [string]$Message,
        [string]$Color = "White"
    )
    Write-Host $Message -ForegroundColor $Color
}

# ASCII Art
Write-ColorOutput @"
██╗    ██╗ ██████╗ ███████╗██╗  ██╗███████╗██╗    ██╗██╗██████╗ ███████╗
██║    ██║██╔════╝ ██╔════╝██║  ██║██╔════╝██║    ██║██║██╔══██╗██╔════╝
██║ █╗ ██║██║  ███╗███████╗███████║█████╗  ██║ █╗ ██║██║██████╔╝█████╗  
██║███╗██║██║   ██║╚════██║██╔══██║██╔══╝  ██║███╗██║██║██╔══██╗██╔══╝  
╚███╔███╔╝╚██████╔╝███████║██║  ██║██║     ╚███╔███╔╝██║██║  ██║███████╗
 ╚══╝╚══╝  ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝      ╚══╝╚══╝ ╚═╝╚═╝  ╚═╝╚══════╝
"@ $Cyan

Write-ColorOutput "🌐 GhostWire - Secure Messaging Network" $Green
Write-ColorOutput "Windows Multi-PC Startup Script" $Yellow
Write-Host ""

# Function to get local IP address
function Get-LocalIP {
    $ip = (Get-NetIPAddress | Where-Object {$_.AddressFamily -eq "IPv4" -and $_.IPAddress -notlike "127.*" -and $_.IPAddress -notlike "169.*"} | Select-Object -First 1).IPAddress
    if (-not $ip) {
        $ip = "192.168.1.100"  # Fallback
    }
    return $ip
}

# Function to find available port
function Find-AvailablePort {
    param([int]$StartPort)
    
    $port = $StartPort
    while ($true) {
        $connection = Get-NetTCPConnection -LocalPort $port -ErrorAction SilentlyContinue
        if (-not $connection) {
            return $port
        }
        $port++
    }
}

# Function to update frontend configuration
function Update-FrontendConfig {
    param([int]$BackendPort)
    
    Write-ColorOutput "Configuring frontend for port $BackendPort..." $Blue
    
    # Update API configuration
    $apiFile = "webui\src\services\api.ts"
    if (Test-Path $apiFile) {
        $content = Get-Content $apiFile -Raw
        $content = $content -replace "localhost:3001", "localhost:$BackendPort"
        Set-Content $apiFile $content -NoNewline
    }
    
    # Update App.tsx configuration
    $appFile = "webui\src\App.tsx"
    if (Test-Path $appFile) {
        $content = Get-Content $appFile -Raw
        $content = $content -replace "localhost:3001", "localhost:$BackendPort"
        Set-Content $appFile $content -NoNewline
    }
}

# Function to scan for other GhostWire nodes
function Scan-GhostWireNodes {
    Write-ColorOutput "🔍 Scanning network for other GhostWire nodes..." $Cyan
    
    $localIP = Get-LocalIP
    $networkPrefix = $localIP -replace "\.\d+$", ""
    $ports = @(3001, 3002, 3003, 3004, 3005)
    
    foreach ($port in $ports) {
        Write-ColorOutput "Scanning port $port..." $Yellow
        
        for ($i = 1; $i -le 254; $i++) {
            $targetIP = "$networkPrefix.$i"
            if ($targetIP -ne $localIP) {
                try {
                    $response = Invoke-WebRequest -Uri "http://${targetIP}:${port}/api/status" -TimeoutSec 1 -ErrorAction SilentlyContinue
                    if ($response.StatusCode -eq 200) {
                        Write-ColorOutput "✓ Found GhostWire node at ${targetIP}:${port}" $Green
                        
                        # Try to get username
                        try {
                            $userResponse = Invoke-WebRequest -Uri "http://${targetIP}:${port}/api/get_username" -TimeoutSec 2 -ErrorAction SilentlyContinue
                            if ($userResponse.StatusCode -eq 200) {
                                $userData = $userResponse.Content | ConvertFrom-Json
                                if ($userData.data) {
                                    Write-ColorOutput "  Username: $($userData.data)" $Cyan
                                }
                            }
                        } catch {
                            # Username not available
                        }
                    }
                } catch {
                    # Node not found or not responding
                }
            }
        }
    }
}

# Main execution
try {
    # Check if we're in the right directory
    if (-not (Test-Path "ghostwire\Cargo.toml")) {
        Write-ColorOutput "Please run this script from the Obsidian directory" $Yellow
        Read-Host "Press Enter to exit"
        exit 1
    }
    
    # Get network information
    $localIP = Get-LocalIP
    
    # Find available ports if not specified
    if ($BackendPort -eq 0) {
        $BackendPort = Find-AvailablePort 3001
    }
    if ($FrontendPort -eq 0) {
        $FrontendPort = Find-AvailablePort 5173
    }
    
    Write-ColorOutput "🌐 GhostWire Multi-PC Setup" $Cyan
    Write-ColorOutput "Local IP: $localIP" $Blue
    Write-ColorOutput "Backend Port: $BackendPort" $Blue
    Write-ColorOutput "Frontend Port: $FrontendPort" $Blue
    Write-Host ""
    
    # Update frontend configuration
    Update-FrontendConfig $BackendPort
    
    # Start backend
    Write-ColorOutput "Starting backend server on port $BackendPort..." $Blue
    Start-Process -FilePath "cmd" -ArgumentList "/k", "cd ghostwire && cargo run -- --host 0.0.0.0 --port $BackendPort" -WindowStyle Normal
    
    # Wait for backend to start
    Start-Sleep -Seconds 3
    
    # Start frontend
    Write-ColorOutput "Starting frontend server on port $FrontendPort..." $Blue
    Start-Process -FilePath "cmd" -ArgumentList "/k", "cd webui && npm run dev -- --port $FrontendPort" -WindowStyle Normal
    
    Write-Host ""
    Write-ColorOutput "✓ GhostWire started successfully!" $Green
    Write-Host ""
    Write-ColorOutput "🌐 Network Information:" $Purple
    Write-ColorOutput "Backend API: http://${localIP}:${BackendPort}" $Yellow
    Write-ColorOutput "Frontend UI: http://${localIP}:${FrontendPort}" $Yellow
    Write-ColorOutput "Local Backend: http://localhost:${BackendPort}" $Yellow
    Write-ColorOutput "Local Frontend: http://localhost:${FrontendPort}" $Yellow
    Write-Host ""
    Write-ColorOutput "🔍 Multi-PC Instructions:" $Cyan
    Write-ColorOutput "1. Other PCs can connect to: http://${localIP}:${FrontendPort}" $Blue
    Write-ColorOutput "2. Use the Peers tab to scan for other GhostWire nodes" $Blue
    Write-ColorOutput "3. Set your username in the Peers tab" $Blue
    Write-Host ""
    
    # Ask if user wants to scan for peers
    $scanPeers = Read-Host "Would you like to scan for other GhostWire nodes? (y/n)"
    if ($scanPeers -eq "y" -or $scanPeers -eq "Y") {
        Scan-GhostWireNodes
    }
    
    Write-Host ""
    Write-ColorOutput "🌐 GhostWire is now running!" $Green
    Write-ColorOutput "Open http://localhost:${FrontendPort} in your browser" $Blue
    
} catch {
    Write-ColorOutput "Error: $($_.Exception.Message)" $Red
    Read-Host "Press Enter to exit"
    exit 1
}

Read-Host "Press Enter to exit" 