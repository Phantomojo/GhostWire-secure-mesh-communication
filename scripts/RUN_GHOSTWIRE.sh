#!/bin/bash
# GhostWire - Secure Mesh Communication
# One-click launcher - No bash knowledge needed!

clear
echo "╔══════════════════════════════════════╗"
echo "║     🛡️  GHOSTWIRE LAUNCHER 🛡️       ║"
echo "║   Secure Mesh Communication App     ║"
echo "╚══════════════════════════════════════╝"
echo ""

cd "$(dirname "$0")/.."

# Check if backend is already running
if lsof -Pi :8080 -sTCP:LISTEN -t >/dev/null 2>&1 ; then
    echo "✅ Backend already running"
else
    echo "🔧 Starting backend..."
    cd ghostwire
    cargo run --release --bin ghostwire -- --web --api-host 0.0.0.0 --port 8080 > /tmp/ghostwire-backend.log 2>&1 &
    BACKEND_PID=$!
    echo "   Backend PID: $BACKEND_PID"
    cd ..
    sleep 3
fi

# Check if frontend is already running  
if lsof -Pi :5173 -sTCP:LISTEN -t >/dev/null 2>&1 ; then
    echo "✅ Frontend already running"
    echo ""
    echo "🌐 GhostWire is ready!"
    echo "   Open: http://localhost:5173"
else
    echo "🎨 Starting web interface..."
    cd webui
    
    # Check if node_modules exists
    if [ ! -d "node_modules" ]; then
        echo "📦 Installing dependencies (first time only)..."
        npm install
    fi
    
    npm run dev > /tmp/ghostwire-frontend.log 2>&1 &
    FRONTEND_PID=$!
    echo "   Frontend PID: $FRONTEND_PID"
    
    echo ""
    echo "⏳ Waiting for services to initialize..."
    
    # Wait for backend to be ready (port 8080)
    MAX_RETRIES=60
    COUNT=0
    until lsof -Pi :8080 -sTCP:LISTEN -t >/dev/null 2>&1 || [ $COUNT -eq $MAX_RETRIES ]; do
        printf "."
        sleep 2
        let COUNT+=1
    done
    
    if [ $COUNT -eq $MAX_RETRIES ]; then
        echo ""
        echo "❌ Backend failed to start in time. Check logs: /tmp/ghostwire-backend.log"
    else
        echo ""
        echo "✨ GhostWire is ready!"
        echo ""
        echo "🌐 Opening in your browser..."
        echo "   URL: http://localhost:5173"
        
        # Try to open browser
        if command -v xdg-open &> /dev/null; then
            xdg-open "http://localhost:5173" 2>/dev/null &
        elif command -v firefox &> /dev/null; then
            firefox "http://localhost:5173" 2>/dev/null &
        elif command -v google-chrome &> /dev/null; then
            google-chrome "http://localhost:5173" 2>/dev/null &
        fi
    fi
fi

echo ""
echo "💡 Tips:"
echo "   • The app will open in your browser"
echo "   • Close this window to stop GhostWire"
echo "   • Check logs: /tmp/ghostwire-*.log"
echo ""
echo "Press Ctrl+C to stop GhostWire"

# Keep script running
tail -f /dev/null
