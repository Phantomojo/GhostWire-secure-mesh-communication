@echo off
cd /d %~dp0
chcp 65001 >nul

REM 🌐 GhostWire - Windows Multi-PC Startup
setlocal

REM === CONFIGURATION ===
set BACKEND_PORT=3001
set FRONTEND_PORT=5173

REM === Update frontend config for backend port ===
powershell -Command "(Get-Content webui\src\services\api.ts) -replace 'localhost:3001', 'localhost:%BACKEND_PORT%' | Set-Content webui\src\services\api.ts"

REM === Start backend ===
echo Starting GhostWire backend on port %BACKEND_PORT% ...
start "GhostWire Backend" cmd /k "cd ghostwire && cargo run -- --host 0.0.0.0 --port %BACKEND_PORT%"

REM Wait for backend to start
ping 127.0.0.1 -n 4 >nul

REM === Start frontend ===
echo Starting GhostWire frontend on port %FRONTEND_PORT% ...
start "GhostWire Frontend" cmd /k "cd webui && npm run dev -- --port %FRONTEND_PORT%"

REM === Print info ===
echo.
echo ✓ GhostWire started successfully!
echo Backend API: http://localhost:%BACKEND_PORT%
echo Frontend UI: http://localhost:%FRONTEND_PORT%
echo.
echo To connect from another PC, use your local IP address instead of localhost.
echo.
pause
endlocal 