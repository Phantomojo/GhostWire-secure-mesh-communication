#!/bin/bash

# 🎨 GhostWire Branding Demo Script
# This script launches GhostWire so you can see the new Afrofuturistic branding in action

echo "🛡️  Starting GhostWire with Afrofuturistic Branding..."
echo ""

# Navigate to project root
cd "$(dirname "$0")"

# Start the backend
echo "🔧 Starting GhostWire backend..."
cd ghostwire
cargo build --release --bin ghostwire 2>&1 | tail -n 5 &
BACKEND_PID=$!

# Give backend time to compile
sleep 3

# Start the backend server
echo "🚀 Launching backend server..."
./target/release/ghostwire --web --api-host 127.0.0.1 --port 8080 &
SERVER_PID=$!

# Wait for backend to be ready
echo "⏳ Waiting for backend to be ready..."
sleep 5

# Start the frontend
echo "🎨 Starting frontend with branding..."
cd ../webui
npm run dev &
FRONTEND_PID=$!

echo ""
echo "✅ GhostWire is starting!"
echo ""
echo "📍 View the branding at:"
echo "   🌐 http://localhost:5173"
echo ""
echo "🎨 What you'll see:"
echo "   • Glowing panther logo in the header"
echo "   • Purple/gold Vibranium color scheme"
echo "   • Afrofuturistic design elements"
echo "   • Feature icons throughout the UI"
echo ""
echo "Press Ctrl+C to stop all services"
echo ""

# Wait for user interrupt
trap "kill $BACKEND_PID $SERVER_PID $FRONTEND_PID 2>/dev/null; exit" INT TERM
wait
