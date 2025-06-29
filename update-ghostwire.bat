@echo off
setlocal

REM --- Update GhostWire (Windows) ---

REM Change to script directory
cd /d %~dp0

REM Pull latest code if .git exists
if exist .git (
    echo Updating code from git...
    git pull
) else (
    echo No git repo found. Skipping code update.
)

REM Check for Rust
where cargo >nul 2>nul
if errorlevel 1 (
    echo Rust not found. Installing Rust...
    powershell -Command "Invoke-WebRequest -Uri https://win.rustup.rs -OutFile rustup-init.exe; Start-Process -Wait .\rustup-init.exe -ArgumentList '-y'; Remove-Item .\rustup-init.exe"
    set PATH=%USERPROFILE%\.cargo\bin;%PATH%
) else (
    echo Rust is installed.
)

REM Build backend
cd ghostwire
cargo build --release
if errorlevel 1 (
    echo Backend build failed!
    exit /b 1
)
cd ..

REM Check for Node.js
where node >nul 2>nul
if errorlevel 1 (
    echo Node.js not found. Please install Node.js from https://nodejs.org/
    exit /b 1
) else (
    echo Node.js is installed.
)

REM Install/update frontend dependencies
cd webui
npm install
if errorlevel 1 (
    echo npm install failed!
    exit /b 1
)

REM Optionally build frontend (uncomment for production)
REM npm run build

cd ..

echo.
echo GhostWire update complete!
echo You can now run the backend and frontend as usual.
endlocal 