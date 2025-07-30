# 🌐 GhostWire Cross-PC Communication Guide

## 🚀 Quick Start

### 1. Start GhostWire on PC 1 (Server)
```bash
./start-ghostwire-complete.sh
```

This will start both frontend and backend servers and display the access URLs.

### 2. Connect from PC 2 (Client)
1. Open the frontend URL from PC 1 in your browser on PC 2
2. Click the ⚙️ config button in the status bar
3. Set the backend URL to: `http://[PC1_IP]:3001/api`
4. Click "Connect"

## 📋 System Requirements

### Backend (Rust)
- Rust 1.70+ (optional - mock server available)
- Python 3.6+ (for mock server)

### Frontend (React)
- Node.js 16+
- npm 8+

## 🔧 Configuration

### Backend Configuration
The backend server runs on port 3001 by default. You can change this by setting environment variables:

```bash
export BACKEND_PORT=3002
export BACKEND_HOST=0.0.0.0
./start-ghostwire-complete.sh
```

### Frontend Configuration
The frontend automatically configures itself for cross-PC communication. The configuration panel allows you to:

- Set custom backend URLs
- Test connection status
- Reset to default settings

## 🌐 Network Discovery

### Automatic Peer Discovery
GhostWire automatically scans your local network for other GhostWire nodes:

1. Go to the **Network** panel
2. Click **🔍 Scan Network**
3. View discovered peers in the **Discovered Peers** section
4. Click **Connect** to establish communication

### Manual Peer Registration
You can manually register with other peers:

1. Get the IP address and port of the target peer
2. Use the configuration panel to set the backend URL
3. The system will automatically register with the peer

## 💬 Messaging Features

### Real-time Communication
- **WebSocket Support**: Real-time message delivery
- **End-to-End Encryption**: All messages are encrypted
- **Message Status**: Track sent, delivered, and read status

### Message Types
- **Text Messages**: Standard text communication
- **System Messages**: Automated system notifications
- **Encrypted Messages**: Secure communication

## 🛡️ Security Features

### Encryption
- **End-to-End Encryption**: Messages encrypted between peers
- **Public Key Exchange**: Secure key distribution
- **Threat Detection**: Automatic security monitoring

### Network Security
- **Stealth Mode**: Hide network presence
- **Emergency Mode**: Lock down all communications
- **Panic Mode**: Immediate system shutdown

## 🔍 Troubleshooting

### Connection Issues
1. **Check Firewall**: Ensure ports 3001 and 5173 are open
2. **Verify IP Address**: Use the correct IP address of the server PC
3. **Test Connectivity**: Try pinging the server PC from the client PC

### Backend Issues
1. **Rust Compilation**: If Rust fails to compile, the system will use a mock server
2. **Port Conflicts**: The launcher automatically finds available ports
3. **Network Binding**: Ensure the backend binds to `0.0.0.0` for network access

### Frontend Issues
1. **Browser Compatibility**: Use modern browsers (Chrome, Firefox, Safari, Edge)
2. **JavaScript Errors**: Check browser console for error messages
3. **CORS Issues**: The backend includes CORS headers for cross-origin requests

## 📊 Monitoring

### System Status
The status bar shows:
- **CPU Usage**: Real-time CPU monitoring
- **Memory Usage**: Memory consumption tracking
- **Network Status**: Connection quality and throughput
- **Security Score**: Overall system security rating

### Network Monitoring
- **Connected Nodes**: List of active connections
- **Network Topology**: Visual representation of the network
- **Peer Statistics**: Connection quality and latency

## 🎯 Advanced Features

### Network Scanning
The system automatically scans for:
- Other GhostWire nodes on the local network
- Available ports and services
- Network topology and routing

### Peer Management
- **Connect/Disconnect**: Manual peer management
- **Ping**: Test connection latency
- **Status Monitoring**: Real-time peer status

### System Controls
- **Emergency Controls**: Quick access to security modes
- **System Monitoring**: Real-time system metrics
- **Configuration Management**: Easy system configuration

## 🔄 Updates and Maintenance

### Automatic Updates
The system includes:
- **Real-time Updates**: Live system status updates
- **Configuration Persistence**: Settings saved between sessions
- **Error Reporting**: Automatic error logging and reporting

### Manual Maintenance
- **Log Monitoring**: View system logs in the System panel
- **Configuration Backup**: Export/import system settings
- **System Restart**: Quick system restart functionality

## 📞 Support

### Getting Help
1. Check the browser console for error messages
2. Review the system logs in the System panel
3. Test network connectivity between PCs
4. Verify firewall and network settings

### Common Issues
- **"Backend Disconnected"**: Check backend URL configuration
- **"Network Scan Failed"**: Verify network permissions and firewall
- **"Message Send Failed"**: Check peer connection status

## 🎉 Success Indicators

When everything is working correctly, you should see:
- ✅ **Backend Connected** in the status bar
- ✅ **Network scan** finds other peers
- ✅ **Messages** can be sent and received
- ✅ **Real-time updates** in the interface

---

**Happy communicating with GhostWire! 🚀** 