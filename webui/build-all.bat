@echo off
chcp 65001 >nul

REM Build React frontend
cd /d %~dp0

echo === Building React frontend ===
npm run build
if %errorlevel% neq 0 (
    echo Frontend build failed!
    exit /b %errorlevel%
)

echo === Building Tauri app ===
cd src-tauri
cargo tauri build
if %errorlevel% neq 0 (
    echo Tauri build failed!
    exit /b %errorlevel%
)

echo.
echo ✓ All builds completed successfully!
echo Output: src-tauri\target\release\bundle
cd .. 