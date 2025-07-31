#!/bin/bash

# GhostWire Health Check Script
# This script monitors the health of GhostWire services

set -e

# Configuration
GHOSTWIRE_HOST="localhost"
GHOSTWIRE_PORT="3000"
API_ENDPOINT="http://${GHOSTWIRE_HOST}:${GHOSTWIRE_PORT}/api/status"
LOG_FILE="/var/log/ghostwire-health.log"
ALERT_EMAIL="admin@example.com"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to log messages
log_message() {
    echo "$(date '+%Y-%m-%d %H:%M:%S') - $1" | tee -a "$LOG_FILE"
}

# Function to check service health
check_health() {
    local response
    local status_code
    
    # Check if service is responding
    if response=$(curl -s -w "%{http_code}" "$API_ENDPOINT" 2>/dev/null); then
        status_code="${response: -3}"
        response_body="${response%???}"
        
        if [ "$status_code" = "200" ]; then
            log_message "✅ Service is healthy (HTTP $status_code)"
            return 0
        else
            log_message "⚠️  Service responded with HTTP $status_code"
            return 1
        fi
    else
        log_message "❌ Service is not responding"
        return 1
    fi
}

# Function to check Docker container
check_docker() {
    if docker ps --filter "name=ghostwire" --format "table {{.Names}}\t{{.Status}}" | grep -q "ghostwire"; then
        log_message "✅ Docker container is running"
        return 0
    else
        log_message "❌ Docker container is not running"
        return 1
    fi
}

# Function to check system resources
check_resources() {
    local cpu_usage
    local memory_usage
    local disk_usage
    
    # Check CPU usage
    cpu_usage=$(top -bn1 | grep "Cpu(s)" | awk '{print $2}' | cut -d'%' -f1)
    
    # Check memory usage
    memory_usage=$(free | grep Mem | awk '{printf "%.1f", $3/$2 * 100.0}')
    
    # Check disk usage
    disk_usage=$(df / | tail -1 | awk '{print $5}' | cut -d'%' -f1)
    
    log_message "📊 System Resources - CPU: ${cpu_usage}%, Memory: ${memory_usage}%, Disk: ${disk_usage}%"
    
    # Alert if resources are high
    if (( $(echo "$cpu_usage > 80" | bc -l) )); then
        log_message "⚠️  High CPU usage: ${cpu_usage}%"
    fi
    
    if (( $(echo "$memory_usage > 80" | bc -l) )); then
        log_message "⚠️  High memory usage: ${memory_usage}%"
    fi
    
    if [ "$disk_usage" -gt 80 ]; then
        log_message "⚠️  High disk usage: ${disk_usage}%"
    fi
}

# Function to send alert
send_alert() {
    local message="$1"
    log_message "🚨 ALERT: $message"
    
    # Send email alert (requires mail command)
    if command -v mail >/dev/null 2>&1; then
        echo "$message" | mail -s "GhostWire Health Alert" "$ALERT_EMAIL"
    fi
}

# Main health check
main() {
    log_message "🔍 Starting GhostWire health check..."
    
    local health_status=0
    local docker_status=0
    
    # Check Docker container
    if ! check_docker; then
        docker_status=1
        health_status=1
    fi
    
    # Check service health
    if ! check_health; then
        health_status=1
    fi
    
    # Check system resources
    check_resources
    
    # Send alert if service is down
    if [ $health_status -eq 1 ]; then
        send_alert "GhostWire service is not responding properly"
    fi
    
    if [ $docker_status -eq 1 ]; then
        send_alert "GhostWire Docker container is not running"
    fi
    
    log_message "🏁 Health check completed with status: $health_status"
    
    exit $health_status
}

# Run main function
main "$@" 