# 🚀 GhostWire Small-Scale Deployment Guide

This guide provides step-by-step instructions for deploying GhostWire in a small-scale production environment.

## 📋 Prerequisites

### System Requirements
- **OS**: Linux (Ubuntu 20.04+ recommended), macOS, or Windows with WSL
- **RAM**: Minimum 2GB, Recommended 4GB+
- **Storage**: Minimum 10GB free space
- **Network**: Internet connection for initial setup

### Software Requirements
- **Docker**: Version 20.10+
- **Docker Compose**: Version 2.0+
- **Git**: For cloning the repository

## 🚀 Quick Deployment

### Option 1: Automated Deployment (Recommended)

1. **Clone the repository**
   ```bash
   git clone https://github.com/Phantomojo/GhostWire-secure-mesh-communication.git
   cd GhostWire-secure-mesh-communication
   ```

2. **Run the deployment script**
   ```bash
   ./deploy.sh
   ```

3. **Access GhostWire**
   - Web UI: http://localhost:3000
   - API: http://localhost:9000

### Option 2: Manual Docker Deployment

1. **Build and start with Docker Compose**
   ```bash
   docker-compose up -d
   ```

2. **Check service status**
   ```bash
   docker-compose ps
   docker-compose logs -f
   ```

### Option 3: System Service Deployment

1. **Install as system service**
   ```bash
   sudo ./systemd/install-service.sh
   ```

2. **Manage the service**
   ```bash
   sudo systemctl start ghostwire
   sudo systemctl status ghostwire
   sudo systemctl stop ghostwire
   ```

## 🔧 Configuration

### Environment Variables

You can customize the deployment by setting environment variables in `docker-compose.yml`:

```yaml
environment:
  - RUST_LOG=info
  - GHOSTWIRE_HOST=0.0.0.0
  - GHOSTWIRE_PORT=3000
  - GHOSTWIRE_MAX_PEERS=100
  - GHOSTWIRE_ENCRYPTION_ALGORITHM=aes-256-gcm
```

### Configuration File

The main configuration is in `ghostwire/config.toml`:

```toml
[server]
host = "0.0.0.0"
port = 3000
web_enabled = true

[security]
max_connections_per_ip = 50
max_messages_per_minute = 200
key_rotation_interval_days = 7

[network]
max_peers = 100
connection_timeout_secs = 30
```

## 📊 Monitoring & Health Checks

### Built-in Health Check

The Docker container includes a health check that monitors the service:

```bash
# Check container health
docker ps

# View health check logs
docker inspect ghostwire | grep -A 10 "Health"
```

### Manual Health Check

Use the monitoring script to check service health:

```bash
# Run health check
./monitoring/health-check.sh

# Set up automated monitoring (every 5 minutes)
echo "*/5 * * * * /path/to/ghostwire/monitoring/health-check.sh" | crontab -
```

### Log Monitoring

```bash
# View real-time logs
docker-compose logs -f

# View system service logs
sudo journalctl -u ghostwire -f

# Check log files
tail -f /opt/ghostwire/data/logs/ghostwire.log
```

## 🔒 Security Considerations

### Network Security

1. **Firewall Configuration**
   ```bash
   # Allow only necessary ports
   sudo ufw allow 3000/tcp  # Web UI
   sudo ufw allow 9000/tcp  # API
   sudo ufw enable
   ```

2. **Reverse Proxy (Optional)**
   ```nginx
   server {
       listen 80;
       server_name your-domain.com;
       
       location / {
           proxy_pass http://localhost:3000;
           proxy_set_header Host $host;
           proxy_set_header X-Real-IP $remote_addr;
       }
   }
   ```

### Access Control

1. **Change default configuration**
   - Update `config.toml` with secure settings
   - Use strong encryption keys
   - Enable threat detection

2. **Regular updates**
   ```bash
   # Update GhostWire
   git pull
   docker-compose down
   docker-compose build --no-cache
   docker-compose up -d
   ```

## 📈 Scaling Considerations

### Resource Limits

For small-scale deployment, the default settings are sufficient. For larger deployments, adjust:

```yaml
# In docker-compose.yml
services:
  ghostwire:
    deploy:
      resources:
        limits:
          memory: 2G
          cpus: '1.0'
        reservations:
          memory: 512M
          cpus: '0.5'
```

### Performance Tuning

1. **Database Optimization** (if using external database)
2. **Caching** (Redis can be enabled in docker-compose.yml)
3. **Load Balancing** (for multiple instances)

## 🛠️ Troubleshooting

### Common Issues

1. **Port Already in Use**
   ```bash
   # Check what's using the port
   sudo lsof -i :3000
   
   # Kill the process or change port in docker-compose.yml
   ```

2. **Container Won't Start**
   ```bash
   # Check logs
   docker-compose logs ghostwire
   
   # Check disk space
   df -h
   
   # Check Docker daemon
   sudo systemctl status docker
   ```

3. **Service Not Responding**
   ```bash
   # Check if container is running
   docker ps
   
   # Restart the service
   docker-compose restart
   
   # Check health endpoint
   curl http://localhost:3000/api/status
   ```

### Recovery Procedures

1. **Service Recovery**
   ```bash
   # Stop and restart
   docker-compose down
   docker-compose up -d
   
   # Or restart system service
   sudo systemctl restart ghostwire
   ```

2. **Data Recovery**
   ```bash
   # Backup data
   docker run --rm -v ghostwire_data:/data -v $(pwd):/backup alpine tar czf /backup/ghostwire-backup.tar.gz -C /data .
   
   # Restore data
   docker run --rm -v ghostwire_data:/data -v $(pwd):/backup alpine tar xzf /backup/ghostwire-backup.tar.gz -C /data
   ```

## 📚 Maintenance

### Regular Maintenance Tasks

1. **Daily**
   - Check service status
   - Review logs for errors
   - Monitor resource usage

2. **Weekly**
   - Update GhostWire
   - Backup configuration and data
   - Review security logs

3. **Monthly**
   - Update system packages
   - Review and rotate encryption keys
   - Performance analysis

### Backup Strategy

```bash
# Create backup script
cat > backup-ghostwire.sh << 'EOF'
#!/bin/bash
BACKUP_DIR="/backup/ghostwire"
DATE=$(date +%Y%m%d_%H%M%S)

mkdir -p "$BACKUP_DIR"

# Backup data
docker run --rm -v ghostwire_data:/data -v "$BACKUP_DIR":/backup alpine tar czf "/backup/data_$DATE.tar.gz" -C /data .

# Backup configuration
cp -r config "$BACKUP_DIR/config_$DATE"

# Clean old backups (keep last 7 days)
find "$BACKUP_DIR" -name "*.tar.gz" -mtime +7 -delete
find "$BACKUP_DIR" -name "config_*" -mtime +7 -exec rm -rf {} \;
EOF

chmod +x backup-ghostwire.sh
```

## 🎯 Success Metrics

### Key Performance Indicators

- **Uptime**: >99.5%
- **Response Time**: <100ms for API calls
- **Error Rate**: <0.1%
- **Resource Usage**: <80% CPU, <80% Memory

### Monitoring Commands

```bash
# Check uptime
uptime

# Check resource usage
docker stats ghostwire

# Check response time
curl -w "@-" -o /dev/null -s "http://localhost:3000/api/status" << 'EOF'
     time_namelookup:  %{time_namelookup}\n
        time_connect:  %{time_connect}\n
     time_appconnect:  %{time_appconnect}\n
    time_pretransfer:  %{time_pretransfer}\n
       time_redirect:  %{time_redirect}\n
  time_starttransfer:  %{time_starttransfer}\n
                     ----------\n
          time_total:  %{time_total}\n
EOF
```

## 📞 Support

### Getting Help

1. **Documentation**: Check the main README.md
2. **Issues**: Report bugs on GitHub
3. **Community**: Join discussions on GitHub
4. **Logs**: Check logs for error details

### Emergency Contacts

- **Service Down**: Check health check script
- **Data Loss**: Use backup recovery procedures
- **Security Issues**: Review security logs and rotate keys

---

**🎉 Congratulations!** Your GhostWire deployment is now ready for small-scale production use. The system is secure, monitored, and maintainable. 