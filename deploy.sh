#!/bin/bash

# GhostWire Small-Scale Deployment Script
# This script deploys GhostWire for small-scale production use

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# ASCII Art
echo -e "${BLUE}"
cat << "EOF"
██╗    ██╗ ██████╗ ███████╗██╗  ██╗███████╗██╗    ██╗██╗██████╗ ███████╗
██║    ██║██╔════╝ ██╔════╝██║  ██║██╔════╝██║    ██║██║██╔══██╗██╔════╝
██║ █╗ ██║██║  ███╗███████╗███████║█████╗  ██║ █╗ ██║██║██████╔╝█████╗  
██║███╗██║██║   ██║╚════██║██╔══██║██╔══╝  ██║███╗██║██║██╔══██╗██╔══╝  
╚███╔███╔╝╚██████╔╝███████║██║  ██║██║     ╚███╔███╔╝██║██║  ██║███████╗
 ╚══╝╚══╝  ╚═════╝ ╚══════╝╚═╝  ╚═╝╚═╝      ╚══╝╚══╝ ╚═╝╚═╝ ═╝╚══════╝
EOF
echo -e "${NC}"

echo -e "${GREEN}🚀 GhostWire Small-Scale Deployment${NC}"
echo -e "${YELLOW}Preparing for production deployment...${NC}"
echo ""

# Function to check if command exists
command_exists() {
    command -v "$1" >/dev/null 2>&1
}

# Function to get local IP
get_local_ip() {
    if command_exists ip; then
        ip route get 1.1.1.1 | grep -oP 'src \K\S+' 2>/dev/null || echo "127.0.0.1"
    elif command_exists ifconfig; then
        ifconfig | grep "inet " | grep -v 127.0.0.1 | awk '{print $2}' | head -n1 || echo "127.0.0.1"
    else
        echo "127.0.0.1"
    fi
}

# Check prerequisites
echo -e "${BLUE}🔍 Checking prerequisites...${NC}"

if ! command_exists docker; then
    echo -e "${RED}❌ Docker not found. Please install Docker first.${NC}"
    echo -e "${YELLOW}Install Docker: https://docs.docker.com/get-docker/${NC}"
    exit 1
fi

if ! command_exists docker-compose; then
    echo -e "${RED}❌ Docker Compose not found. Please install Docker Compose first.${NC}"
    echo -e "${YELLOW}Install Docker Compose: https://docs.docker.com/compose/install/${NC}"
    exit 1
fi

echo -e "${GREEN}✓ Docker and Docker Compose found${NC}"

# Check if we're in the right directory
if [ ! -f "Dockerfile" ] || [ ! -f "docker-compose.yml" ]; then
    echo -e "${RED}❌ Please run this script from the GhostWire root directory${NC}"
    exit 1
fi

# Get local IP
LOCAL_IP=$(get_local_ip)
echo -e "${GREEN}✓ Local IP: ${LOCAL_IP}${NC}"

# Check port availability
echo -e "${BLUE}🔍 Checking port availability...${NC}"

PORTS=(3000 9000)
for port in "${PORTS[@]}"; do
    if lsof -Pi :$port -sTCP:LISTEN -t >/dev/null 2>&1; then
        echo -e "${YELLOW}⚠️  Port $port is in use${NC}"
        echo -e "${YELLOW}Please stop the service using port $port or modify docker-compose.yml${NC}"
        exit 1
    else
        echo -e "${GREEN}✓ Port $port available${NC}"
    fi
done

# Create necessary directories
echo -e "${BLUE}📁 Creating directories...${NC}"
mkdir -p data/logs
mkdir -p config
chmod 755 data config

# Build and start services
echo -e "${BLUE}🔨 Building GhostWire...${NC}"
docker-compose build --no-cache

echo -e "${BLUE}🚀 Starting GhostWire...${NC}"
docker-compose up -d

# Wait for service to start
echo -e "${BLUE}⏳ Waiting for service to start...${NC}"
sleep 10

# Check service health
echo -e "${BLUE}🏥 Checking service health...${NC}"
for i in {1..30}; do
    if curl -f http://localhost:3000/api/status >/dev/null 2>&1; then
        echo -e "${GREEN}✓ Service is healthy${NC}"
        break
    else
        echo -e "${YELLOW}⏳ Waiting for service to be ready... (attempt $i/30)${NC}"
        sleep 2
    fi
done

if [ $i -eq 30 ]; then
    echo -e "${RED}❌ Service failed to start properly${NC}"
    echo -e "${YELLOW}Checking logs...${NC}"
    docker-compose logs
    exit 1
fi

# Display deployment information
echo ""
echo -e "${GREEN}🎉 GhostWire deployed successfully!${NC}"
echo ""
echo -e "${BLUE}📋 Deployment Information:${NC}"
echo -e "  • Web UI: http://${LOCAL_IP}:3000"
echo -e "  • API: http://${LOCAL_IP}:9000"
echo -e "  • Local Access: http://localhost:3000"
echo ""
echo -e "${BLUE}🔧 Management Commands:${NC}"
echo -e "  • View logs: docker-compose logs -f"
echo -e "  • Stop service: docker-compose down"
echo -e "  • Restart service: docker-compose restart"
echo -e "  • Update service: docker-compose pull && docker-compose up -d"
echo ""
echo -e "${BLUE}📊 Monitoring:${NC}"
echo -e "  • Container status: docker-compose ps"
echo -e "  • Resource usage: docker stats ghostwire"
echo -e "  • Health check: curl http://localhost:3000/api/status"
echo ""
echo -e "${GREEN}✅ GhostWire is now running and ready for small-scale use!${NC}" 