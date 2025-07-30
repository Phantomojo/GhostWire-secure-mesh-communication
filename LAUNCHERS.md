# 🌐 GhostWire Launchers

This directory contains comprehensive launcher scripts to start the entire GhostWire system with a single command.

## 🚀 Quick Start

### Linux/macOS
```bash
./launch-ghostwire.sh
```

### Windows (Command Prompt)
```cmd
launch-ghostwire.bat
```

### Windows (PowerShell)
```powershell
.\launch-ghostwire.ps1
```

## 📋 What Gets Started

The launchers automatically start all GhostWire components:

1. **Backend (Rust)** - Core networking, encryption, and API services
2. **Frontend (WebUI)** - React-based web interface
3. **TUI** - Terminal User Interface for command-line interaction

## 🎛️ Launch Options

### Start Everything (Default)
```bash
./launch-ghostwire.sh
```

### Start Only Backend
```bash
./launch-ghostwire.sh --backend-only
```

### Start Only Frontend
```bash
./launch-ghostwire.sh --frontend-only
```

### Start Only TUI
```bash
./launch-ghostwire.sh --tui-only
```

### Custom Ports
```bash
./launch-ghostwire.sh --backend-port 3002 --frontend-port 5174
```

## 🔧 Configuration

### Environment Variables
- `BACKEND_PORT` - Backend API port (default: 3001)
- `FRONTEND_PORT` - Frontend web port (default: 5173)
- `BACKEND_HOST` - Backend host (default: 0.0.0.0)
- `LAUNCH_MODE` - Launch mode (default: all)

### Command Line Options
- `--backend-only` - Start only the backend
- `--frontend-only` - Start only the frontend
- `--tui-only` - Start only the TUI
- `--backend-port N` - Set backend port
- `--frontend-port N` - Set frontend port
- `--backend-host H` - Set backend host
- `-h, --help` - Show help message

## 🌐 Access Points

Once started, you can access:

- **Backend API**: `http://localhost:3001/api`
- **Frontend UI**: `http://localhost:5173`
- **TUI**: Running in terminal

## 🔍 Features

### Automatic Port Detection
If the default ports are in use, the launchers automatically find available ports.

### Dependency Checking
- Verifies Rust/Cargo is installed
- Verifies Node.js/npm is installed
- Checks for required directories

### Process Monitoring
- Monitors all running processes
- Automatic cleanup on exit
- Graceful shutdown with Ctrl+C

### Error Handling
- Comprehensive error messages
- Automatic dependency installation
- Build verification

## 🛠️ Prerequisites

### Required Software
- **Rust** - [Install via rustup.rs](https://rustup.rs/)
- **Node.js** - [Install from nodejs.org](https://nodejs.org/)

### Optional Software
- **Git** - For version control
- **Docker** - For containerized deployment

## 📁 File Structure

```
launch-ghostwire.sh      # Linux/macOS launcher
launch-ghostwire.bat     # Windows batch launcher
launch-ghostwire.ps1     # Windows PowerShell launcher
start-all.js            # Node.js launcher (legacy)
start-ghostwire.bat     # Windows launcher (legacy)
```

## 🔄 Legacy Launchers

The following launchers are still available but may have limited functionality:

- `start-all.js` - Node.js-based launcher
- `start-ghostwire.bat` - Basic Windows launcher

## 🐛 Troubleshooting

### Common Issues

**Port Already in Use**
```
Port 3001 is in use, finding available port...
Using backend port: 3002
```
*Solution: The launcher automatically finds available ports.*

**Cargo Not Found**
```
Error: Cargo not found. Please install Rust first.
```
*Solution: Install Rust via [rustup.rs](https://rustup.rs/)*

**npm Not Found**
```
Error: npm not found. Please install Node.js first.
```
*Solution: Install Node.js from [nodejs.org](https://nodejs.org/)*

**Build Failed**
```
Error: Backend build failed
```
*Solution: Check Rust dependencies and try `cargo clean` in the ghostwire directory*

### Debug Mode

For detailed debugging, you can run components individually:

```bash
# Backend only
cd ghostwire && cargo run -- --host 0.0.0.0 --port 3001

# Frontend only
cd webui && npm run dev -- --port 5173

# TUI only
cd ghostwire && cargo run --bin ghostwire -- tui
```

## 🔐 Security Notes

- The backend runs on `0.0.0.0` by default for network accessibility
- For production, consider binding to specific interfaces
- All communication is encrypted by default
- API endpoints are protected with authentication

## 📞 Support

If you encounter issues:

1. Check the prerequisites are installed
2. Verify all dependencies are available
3. Check the troubleshooting section
4. Review the logs in the terminal output
5. Try running components individually for debugging

## 🎯 Next Steps

After launching GhostWire:

1. **Web Interface**: Open `http://localhost:5173` in your browser
2. **API Testing**: Use `http://localhost:3001/api` for programmatic access
3. **TUI**: Use the terminal interface for command-line operations
4. **Documentation**: Check the `/docs` directory for detailed guides

---

**Happy networking with GhostWire! 🌐✨** 