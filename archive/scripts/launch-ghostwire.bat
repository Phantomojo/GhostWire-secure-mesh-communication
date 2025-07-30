@echo off
setlocal enabledelayedexpansion

REM 🌐 GhostWire - Complete System Launcher (Windows)
REM Starts backend, frontend, and TUI components with proper error handling

REM Colors for output (Windows 10+)
set "RED=[91m"
set "GREEN=[92m"
set "YELLOW=[93m"
set "BLUE=[94m"
set "CYAN=[96m"
set "PURPLE=[95m"
set "NC=[0m"

REM ASCII Art
echo %CYAN%
echo ██╗    ██╗ ██████╗ ███████╗██╗  ██╗███████╗██╗    ██╗██╗██████╗ ███████╗
echo ██║    ██║██╔════╝ ██╔════╝██║  ██║██╔════╝██║    ██║██║██╔══██╗██╔════╝
echo ██║ █╗ ██║██║  ███╗███████╗███████║█████╗  ██║ █╗ ██║██║██████╔╝█████╗  
echo ██║███╗██║██║   ██║╚════██║██╔══██║██╔══╝  ██║███╗██║██║██╔══██╗██╔══╝  
echo ╚███╔███╔╝╚██████╔╝███████║██║  ██║██║     ╚███╔███╔╝██║██║  ██║███████╗
echo  ╚══╝╚══╝  ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝      ╚══╝╚══╝ ╚═╝╚═╝  ╚═╝╚══════╝
echo %NC%

echo %GREEN%🌐 GhostWire - Complete System Launcher%NC%
echo %YELLOW%Starting backend, frontend, and TUI components...%NC%
echo.

REM Configuration
set "BACKEND_PORT=3001"
set "FRONTEND_PORT=5173"
set "BACKEND_HOST=0.0.0.0"
set "LAUNCH_MODE=all"

REM Parse command line arguments
:parse_args
if "%~1"=="" goto :end_parse
if "%~1"=="--backend-only" set "LAUNCH_MODE=backend-only" & shift & goto :parse_args
if "%~1"=="--frontend-only" set "LAUNCH_MODE=frontend-only" & shift & goto :parse_args
if "%~1"=="--tui-only" set "LAUNCH_MODE=tui-only" & shift & goto :parse_args
if "%~1"=="--backend-port" set "BACKEND_PORT=%~2" & shift & shift & goto :parse_args
if "%~1"=="--frontend-port" set "FRONTEND_PORT=%~2" & shift & shift & goto :parse_args
if "%~1"=="--backend-host" set "BACKEND_HOST=%~2" & shift & shift & goto :parse_args
if "%~1"=="-h" goto :show_help
if "%~1"=="--help" goto :show_help
echo %RED%Unknown option: %~1%NC% >&2
echo Use --help for usage information
exit /b 1
:end_parse

REM Check if ports are available
echo %BLUE%Checking port availability...%NC%
netstat -an | find ":%BACKEND_PORT% " >nul 2>&1
if %errorlevel% equ 0 (
    echo %YELLOW%Port %BACKEND_PORT% is in use, finding available port...%NC%
    for /l %%i in (%BACKEND_PORT%,1,%BACKEND_PORT%+100) do (
        netstat -an | find ":%%i " >nul 2>&1
        if !errorlevel! neq 0 (
            set "BACKEND_PORT=%%i"
            echo %BLUE%Using backend port: !BACKEND_PORT!%NC%
            goto :port_check_done
        )
    )
    echo %RED%Error: Could not find available port starting from %BACKEND_PORT%%NC% >&2
    exit /b 1
)

:port_check_done
netstat -an | find ":%FRONTEND_PORT% " >nul 2>&1
if %errorlevel% equ 0 (
    echo %YELLOW%Port %FRONTEND_PORT% is in use, finding available port...%NC%
    for /l %%i in (%FRONTEND_PORT%,1,%FRONTEND_PORT%+100) do (
        netstat -an | find ":%%i " >nul 2>&1
        if !errorlevel! neq 0 (
            set "FRONTEND_PORT=%%i"
            echo %BLUE%Using frontend port: !FRONTEND_PORT!%NC%
            goto :start_services
        )
    )
    echo %RED%Error: Could not find available port starting from %FRONTEND_PORT%%NC% >&2
    exit /b 1
)

:start_services
REM Start components based on launch mode
if "%LAUNCH_MODE%"=="backend-only" goto :start_backend
if "%LAUNCH_MODE%"=="frontend-only" goto :start_frontend
if "%LAUNCH_MODE%"=="tui-only" goto :start_tui
goto :start_all

:start_backend
call :start_backend_service
goto :monitor_services

:start_frontend
call :start_frontend_service
goto :monitor_services

:start_tui
call :start_tui_service
goto :monitor_services

:start_all
call :start_backend_service
call :start_frontend_service
call :start_tui_service
goto :monitor_services

:start_backend_service
echo %BLUE%Starting GhostWire backend...%NC%

REM Check if Rust is installed
where cargo >nul 2>&1
if %errorlevel% neq 0 (
    echo %RED%Error: Cargo not found. Please install Rust first.%NC% >&2
    echo %YELLOW%Visit: https://rustup.rs/%NC%
    exit /b 1
)

REM Check if ghostwire directory exists
if not exist "ghostwire" (
    echo %RED%Error: ghostwire directory not found%NC% >&2
    exit /b 1
)

REM Build backend if needed
echo %BLUE%Building backend...%NC%
cd ghostwire
cargo build --quiet
if %errorlevel% neq 0 (
    echo %RED%Error: Backend build failed%NC% >&2
    exit /b 1
)
cd ..

REM Start backend
echo %BLUE%Starting backend on %BACKEND_HOST%:%BACKEND_PORT%...%NC%
cd ghostwire
start "GhostWire Backend" cmd /k "cargo run -- --host %BACKEND_HOST% --port %BACKEND_PORT%"
cd ..
set "BACKEND_PID=%errorlevel%"

REM Wait for backend to start
timeout /t 3 /nobreak >nul

echo %GREEN%✓ Backend started%NC%
goto :eof

:start_frontend_service
echo %BLUE%Starting GhostWire frontend...%NC%

REM Check if Node.js is installed
where npm >nul 2>&1
if %errorlevel% neq 0 (
    echo %RED%Error: npm not found. Please install Node.js first.%NC% >&2
    echo %YELLOW%Visit: https://nodejs.org/%NC%
    exit /b 1
)

REM Check if webui directory exists
if not exist "webui" (
    echo %RED%Error: webui directory not found%NC% >&2
    exit /b 1
)

REM Install frontend dependencies if needed
if not exist "webui\node_modules" (
    echo %BLUE%Installing frontend dependencies...%NC%
    cd webui
    npm install
    cd ..
)

REM Update frontend configuration
echo %BLUE%Updating frontend API configuration...%NC%
powershell -Command "(Get-Content webui\src\services\api.ts) -replace 'const API_BASE_URL = .*', 'const API_BASE_URL = \"http://localhost:%BACKEND_PORT%/api\";' | Set-Content webui\src\services\api.ts"
echo %GREEN%✓ Frontend API configuration updated%NC%

REM Start frontend
echo %BLUE%Starting frontend on port %FRONTEND_PORT%...%NC%
cd webui
start "GhostWire Frontend" cmd /k "npm run dev -- --port %FRONTEND_PORT%"
cd ..
set "FRONTEND_PID=%errorlevel%"

REM Wait for frontend to start
timeout /t 5 /nobreak >nul

echo %GREEN%✓ Frontend started%NC%
goto :eof

:start_tui_service
echo %BLUE%Starting GhostWire TUI...%NC%

REM Start TUI in a new terminal
echo %BLUE%Starting TUI in new terminal...%NC%
cd ghostwire
start "GhostWire TUI" cmd /k "cargo run --bin ghostwire -- tui"
cd ..
set "TUI_PID=%errorlevel%"

echo %GREEN%✓ TUI started%NC%
goto :eof

:monitor_services
REM Print status information
echo.
echo %GREEN%==============================%NC%
echo %GREEN%✓ GhostWire started successfully!%NC%
if not "%LAUNCH_MODE%"=="frontend-only" if not "%LAUNCH_MODE%"=="tui-only" (
    echo %CYAN%Backend API:   http://localhost:%BACKEND_PORT%/api%NC%
)
if not "%LAUNCH_MODE%"=="backend-only" if not "%LAUNCH_MODE%"=="tui-only" (
    echo %CYAN%Frontend UI:   http://localhost:%FRONTEND_PORT%%NC%
)
if not "%LAUNCH_MODE%"=="backend-only" if not "%LAUNCH_MODE%"=="frontend-only" (
    echo %CYAN%TUI:           Running in terminal%NC%
)
echo %GREEN%==============================%NC%
echo.
echo %YELLOW%Press Ctrl+C to stop all services%NC%
echo.

REM Keep script running and monitor processes
:monitor_loop
timeout /t 5 /nobreak >nul
goto :monitor_loop

:show_help
echo Usage: %~nx0 [OPTIONS]
echo.
echo Options:
echo   --backend-only     Start only the backend
echo   --frontend-only    Start only the frontend
echo   --tui-only         Start only the TUI
echo   --backend-port N   Set backend port (default: 3001)
echo   --frontend-port N  Set frontend port (default: 5173)
echo   --backend-host H   Set backend host (default: 0.0.0.0)
echo   -h, --help         Show this help message
echo.
echo Environment variables:
echo   BACKEND_PORT       Backend port (default: 3001)
echo   FRONTEND_PORT      Frontend port (default: 5173)
echo   BACKEND_HOST       Backend host (default: 0.0.0.0)
echo   LAUNCH_MODE        Launch mode (default: all)
exit /b 0 