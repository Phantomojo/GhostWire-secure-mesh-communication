# 🌐 GhostWire Cross-PC Communication Guide

## 🚀 Quick Start

### **Option 1: Production Deployment (Recommended)**
```bash
# Deploy on PC 1 (Server)
./deploy.sh

# Deploy on PC 2 (Client)
./deploy.sh

# Access GhostWire
# Web UI: http://localhost:3000
# API: http://localhost:9000
```

### **Option 2: Development Mode**
```bash
# Start on PC 1 (Server)
./launch-ghostwire-working.sh

# Start on PC 2 (Client)
./launch-ghostwire-working.sh
```

### **Option 3: System Service**
```bash
# Install as system service on both PCs
sudo ./systemd/install-service.sh
```

## 🔧 Cross-PC Configuration

### **1. Start GhostWire on PC 1 (Server)**
Choose your deployment option and start the service. Note the IP address displayed.

### **2. Connect from PC 2 (Client)**
1. Open the frontend URL from PC 1 in your browser on PC 2
2. Click the ⚙️ config button in the status bar
3. Set the backend URL to: `http://[PC1_IP]:3000/api` (production) or `http://[PC1_IP]:3001/api` (development)
4. Click "Connect"

## 📋 System Requirements

### **Production Deployment**
- **Docker**: Version 20.10+
- **Docker Compose**: Version 2.0+
- **Network**: Local network connectivity

### **Development Mode**
- **Backend (Rust)**: Rust 1.70+ (optional - mock server available)
- **Frontend (React)**: Node.js 16+, npm 8+
- **Python**: 3.6+ (for mock server)

## 🔧 Configuration

### **Production Configuration**
The production deployment runs on standard ports:
- **Web UI**: Port 3000
- **API**: Port 9000
- **Health Check**: Port 9090

```bash
# Customize ports in docker-compose.yml
ports:
  - "3000:3000"  # Web UI
  - "9000:9000"  # API
```

### **Development Configuration**
The development server runs on dynamic ports:
- **Backend**: Port 3001/3002 (auto-detected)
- **Frontend**: Port 5173/5175 (auto-detected)

```bash
# Customize backend settings
export BACKEND_PORT=3002
export BACKEND_HOST=0.0.0.0
./launch-ghostwire-working.sh
```

### **Frontend Configuration**
The frontend automatically configures itself for cross-PC communication. The configuration panel allows you to:

- Set custom backend URLs
- Test connection status
- Reset to default settings
- Monitor connection health

## 🌐 Network Discovery

### **Automatic Peer Discovery**
GhostWire automatically scans your local network for other GhostWire nodes:

1. Go to the **Network** panel
2. Click **🔍 Scan Network**
3. View discovered peers in the **Discovered Peers** section
4. Click **Connect** to establish communication

### **Manual Peer Registration**
You can manually register with other peers:

1. Get the IP address and port of the target peer
2. Use the configuration panel to set the backend URL
3. The system will automatically register with the peer

### **Production Network Discovery**
In production mode, peer discovery is enhanced with:
- **Health checks** for discovered peers
- **Automatic reconnection** for lost peers
- **Load balancing** across multiple peers
- **Security validation** of peer connections

## 💬 Messaging Features

### **Real-time Communication**
- **WebSocket Support**: Real-time message delivery
- **End-to-End Encryption**: All messages are encrypted
- **Message Status**: Track sent, delivered, and read status
- **Production Monitoring**: Message delivery metrics

### **Message Types**
- **Text Messages**: Standard text communication
- **System Messages**: Automated system notifications
- **Encrypted Messages**: Secure communication
- **Broadcast Messages**: Send to all connected peers

## 🛡️ Security Features

### **Encryption**
- **End-to-End Encryption**: Messages encrypted between peers
- **Public Key Exchange**: Secure key distribution
- **Threat Detection**: Automatic security monitoring
- **Key Rotation**: Automatic encryption key rotation

### **Network Security**
- **Stealth Mode**: Hide network presence
- **Emergency Mode**: Lock down all communications
- **Panic Mode**: Immediate system shutdown
- **Firewall Integration**: Automatic firewall configuration

### **Production Security**
- **Container Security**: Non-root container execution
- **Network Isolation**: Docker network isolation
- **Health Monitoring**: Security health checks
- **Audit Logging**: Comprehensive security logs

## 🔍 Troubleshooting

### **Production Deployment Issues**

#### **Container Communication Problems**
```bash
# Check container status
docker-compose ps

# Check container logs
docker-compose logs ghostwire

# Check network connectivity
docker network ls
docker network inspect ghostwire_ghostwire_network

# Restart containers
docker-compose restart
```

#### **Health Check Failures**
```bash
# Run health check manually
./monitoring/health-check.sh

# Check API endpoints
curl http://localhost:3000/api/status
curl http://localhost:9000/api/peers

# Check container health
docker inspect ghostwire | grep -A 10 "Health"
```

### **Development Mode Issues**

#### **Connection Issues**
1. **Check Firewall**: Ensure ports 3001 and 5173 are open
2. **Verify IP Address**: Use the correct IP address of the server PC
3. **Test Connectivity**: Try pinging the server PC from the client PC

#### **Backend Issues**
1. **Rust Compilation**: If Rust fails to compile, the system will use a mock server
2. **Port Conflicts**: The launcher automatically finds available ports
3. **Network Binding**: Ensure the backend binds to `0.0.0.0` for network access

#### **Frontend Issues**
1. **Browser Compatibility**: Use modern browsers (Chrome, Firefox, Safari, Edge)
2. **CORS Issues**: Check browser console for CORS errors
3. **WebSocket Issues**: Verify WebSocket connections are allowed

### **Cross-PC Communication Issues**

#### **Peers Can't Discover Each Other**
```bash
# Check network connectivity
ping [PC1_IP]
ping [PC2_IP]

# Check port accessibility
telnet [PC1_IP] 3000  # Production
telnet [PC1_IP] 3001  # Development

# Check firewall settings
sudo ufw status
sudo iptables -L
```

#### **Connection Requests Fail**
1. **Verify Backend URLs**: Ensure correct IP and port
2. **Check API Endpoints**: Test API endpoints directly
3. **Review Logs**: Check both frontend and backend logs
4. **Network Configuration**: Verify network settings

## 📊 Monitoring & Health

### **Production Monitoring**
```bash
# Check service health
./monitoring/health-check.sh

# Monitor resource usage
docker stats ghostwire

# View logs
docker-compose logs -f

# Check system service status
sudo systemctl status ghostwire
```

### **Cross-PC Health Monitoring**
- **Connection Status**: Real-time connection monitoring
- **Message Delivery**: Track message delivery success rates
- **Peer Health**: Monitor health of connected peers
- **Network Performance**: Track network latency and throughput

## 🚀 Advanced Configuration

### **Multiple PC Setup (3+ Computers)**
1. Deploy GhostWire on all PCs using `./deploy.sh`
2. Configure one PC as the primary hub
3. Connect all other PCs to the hub
4. Test peer discovery and messaging
5. Monitor network health across all instances

### **Load Balancing**
For high-traffic scenarios:
- Deploy multiple GhostWire instances
- Use load balancer for frontend traffic
- Configure peer distribution across instances
- Monitor load distribution and performance

### **High Availability**
For critical deployments:
- Deploy multiple instances on different servers
- Configure automatic failover
- Set up monitoring and alerting
- Implement backup and recovery procedures

## 🎯 Success Criteria

### **Functional Success:**
- ✅ PCs can discover each other automatically
- ✅ Connection requests are sent and accepted
- ✅ Messages are delivered between peers
- ✅ Real-time updates work across all PCs
- ✅ All 46 buttons work on all connected PCs

### **Production Success:**
- ✅ Containers communicate across network
- ✅ Health checks pass on all instances
- ✅ Resource usage stays within limits
- ✅ Security features work across network
- ✅ Monitoring shows healthy status

## 🚀 Ready to Deploy!

Choose your deployment option and start cross-PC communication:

### **For Production:**
```bash
./deploy.sh
```

### **For Development:**
```bash
./launch-ghostwire-working.sh
```

**🎉 Your GhostWire system is now ready for cross-PC communication with production deployment capabilities! 🎉** 