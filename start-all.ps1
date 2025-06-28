# PowerShell script to start both GhostWire backend and frontend

# Start the Rust backend
Start-Process powershell -ArgumentList "cd Obsidian/ghostwire; cargo run"
 
# Start the React frontend
Start-Process powershell -ArgumentList "cd Obsidian/webui; npm run dev" 