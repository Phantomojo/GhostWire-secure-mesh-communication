@echo off
setlocal

REM --- Update GhostWire (Windows) ---

REM Change to script directory
cd /d %~dp0

REM Helper: Report error to main PC
set REPORT_URL=http://192.168.100.242:3001/api/report_error
set HOSTNAME=%COMPUTERNAME%
for /f "tokens=2 delims=: " %%a in ('ipconfig ^| findstr /C:"IPv4 Address"') do set LOCALIP=%%a
set OSVER=%OS% %PROCESSOR_ARCHITECTURE%

REM Function to report error
set REPORT_ERROR_POWERSHELL=powershell -Command "$body = @{ error = \"Updater error on %HOSTNAME% (%LOCALIP%) [%OSVER%]: ERROR_MSG\" } | ConvertTo-Json; Invoke-RestMethod -Uri '%REPORT_URL%' -Method Post -ContentType 'application/json' -Body $body"

REM Pull latest code if .git exists
if exist .git (
    echo Updating code from git...
    git pull || (set ERR=Git pull failed! & call :report_error "%ERR%" & exit /b 1)
) else (
    echo No git repo found. Skipping code update.
)

REM Check for Rust
where cargo >nul 2>nul
if errorlevel 1 (
    echo Rust not found. Installing Rust...
    powershell -Command "Invoke-WebRequest -Uri https://win.rustup.rs -OutFile rustup-init.exe; Start-Process -Wait .\rustup-init.exe -ArgumentList '-y'; Remove-Item .\rustup-init.exe" || (set ERR=Rust install failed! & call :report_error "%ERR%" & exit /b 1)
    set PATH=%USERPROFILE%\.cargo\bin;%PATH%
) else (
    echo Rust is installed.
)

REM Build backend
cd ghostwire
cargo build --release || (set ERR=Backend build failed! & call :report_error "%ERR%" & exit /b 1)
cd ..

REM Check for Node.js
where node >nul 2>nul
if errorlevel 1 (
    set ERR=Node.js not found! & call :report_error "%ERR%" & exit /b 1
) else (
    echo Node.js is installed.
)

REM Install/update frontend dependencies
cd webui
npm install || (set ERR=npm install failed! & call :report_error "%ERR%" & exit /b 1)

REM Optionally build frontend (uncomment for production)
REM npm run build || (set ERR=Frontend build failed! & call :report_error "%ERR%" & exit /b 1)

cd ..

echo.
echo GhostWire update complete!
echo You can now run the backend and frontend as usual.
goto :eof

:report_error
set MSG=%~1
setlocal enabledelayedexpansion
set CMD=!REPORT_ERROR_POWERSHELL:ERROR_MSG=%MSG%!
%CMD%
endlocal
exit /b 1 