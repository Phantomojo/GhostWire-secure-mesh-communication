#!/bin/bash

# 🔒 Enterprise-Grade Security Monitor for GhostWire
# Comprehensive security analysis, monitoring, and reporting
# Date: August 13, 2025

set -e

# Configuration
PROJECT_DIR="${PROJECT_DIR:-$(pwd)}"
LOG_DIR="${LOG_DIR:-$PROJECT_DIR/logs}"
REPORT_DIR="${REPORT_DIR:-$PROJECT_DIR/security_reports}"
ANALYSIS_DIR="${ANALYSIS_DIR:-$PROJECT_DIR/security_analysis}"
DATE=$(date +%Y%m%d_%H%M%S)
TIMESTAMP=$(date -Iseconds)

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

# Create directories
mkdir -p "$LOG_DIR"
mkdir -p "$REPORT_DIR"
mkdir -p "$ANALYSIS_DIR"

# Function to log messages
log_message() {
    echo "[$(date)] $1" | tee -a "$LOG_DIR/enterprise_security.log"
}

# Function to send alert
send_alert() {
    local message="$1"
    local level="$2"
    local channel="$3"
    
    case $level in
        "critical")
            echo -e "${RED}🚨 CRITICAL ALERT: $message${NC}"
            ;;
        "high")
            echo -e "${YELLOW}⚠️ HIGH ALERT: $message${NC}"
            ;;
        "medium")
            echo -e "${YELLOW}⚠️ MEDIUM ALERT: $message${NC}"
            ;;
        "low")
            echo -e "${BLUE}ℹ️ LOW ALERT: $message${NC}"
            ;;
        *)
            echo -e "${GREEN}ℹ️ INFO: $message${NC}"
            ;;
    esac
    
    log_message "[$level] $message"
    
    # TODO: Implement actual alert channels (Slack, email, webhook, etc.)
    if [ "$channel" = "slack" ]; then
        # Slack notification would go here
        echo "Slack notification: $message" >> "$LOG_DIR/alerts.log"
    elif [ "$channel" = "email" ]; then
        # Email notification would go here
        echo "Email notification: $message" >> "$LOG_DIR/alerts.log"
    fi
}

# Function to generate security score
calculate_security_score() {
    local critical_vulns=$1
    local high_vulns=$2
    local medium_vulns=$3
    local low_vulns=$4
    
    # Base score starts at 100
    local score=100
    
    # Deduct points based on vulnerabilities
    score=$((score - critical_vulns * 20))
    score=$((score - high_vulns * 10))
    score=$((score - medium_vulns * 5))
    score=$((score - low_vulns * 1))
    
    # Ensure score doesn't go below 0
    if [ $score -lt 0 ]; then
        score=0
    fi
    
    echo $score
}

# Function to analyze dependencies
analyze_dependencies() {
    log_message "Analyzing dependencies..."
    
    cd "$PROJECT_DIR"
    
    # Run safety check
    if command -v safety &> /dev/null; then
        log_message "Running Safety vulnerability check..."
        safety check --json --output "$ANALYSIS_DIR/safety-results-$DATE.json" || true
        
        # Parse results
        if [ -f "$ANALYSIS_DIR/safety-results-$DATE.json" ]; then
            local critical_vulns=$(grep -c '"severity": "critical"' "$ANALYSIS_DIR/safety-results-$DATE.json" || echo "0")
            local high_vulns=$(grep -c '"severity": "high"' "$ANALYSIS_DIR/safety-results-$DATE.json" || echo "0")
            local medium_vulns=$(grep -c '"severity": "medium"' "$ANALYSIS_DIR/safety-results-$DATE.json" || echo "0")
            local low_vulns=$(grep -c '"severity": "low"' "$ANALYSIS_DIR/safety-results-$DATE.json" || echo "0")
            
            # Send alerts for critical and high vulnerabilities
            if [ "$critical_vulns" -gt 0 ]; then
                send_alert "Critical vulnerabilities detected: $critical_vulns" "critical" "all"
            fi
            
            if [ "$high_vulns" -gt 0 ]; then
                send_alert "High vulnerabilities detected: $high_vulns" "high" "all"
            fi
            
            # Calculate security score
            local score=$(calculate_security_score "$critical_vulns" "$high_vulns" "$medium_vulns" "$low_vulns")
            echo "$score" > "$ANALYSIS_DIR/security-score-$DATE.txt"
            
            log_message "Security score: $score/100"
        fi
    else
        log_message "Safety not installed, skipping dependency analysis"
    fi
}

# Function to run security scans
run_security_scans() {
    log_message "Running security scans..."
    
    cd "$PROJECT_DIR"
    
    # Run Bandit if available
    if command -v bandit &> /dev/null; then
        log_message "Running Bandit security scan..."
        bandit -r ghostwire/ -f json -o "$ANALYSIS_DIR/bandit-results-$DATE.json" || true
        bandit -r ghostwire/ -f txt > "$ANALYSIS_DIR/bandit-summary-$DATE.txt" || true
    fi
    
    # Run Semgrep if available
    if command -v semgrep &> /dev/null; then
        log_message "Running Semgrep security scan..."
        semgrep --config=auto ghostwire/ --json --output="$ANALYSIS_DIR/semgrep-results-$DATE.json" || true
        semgrep --config=auto ghostwire/ --text --output="$ANALYSIS_DIR/semgrep-summary-$DATE.txt" || true
    fi
    
    # Run TruffleHog if available
    if command -v trufflehog &> /dev/null; then
        log_message "Running TruffleHog secrets scan..."
        trufflehog --json --output="$ANALYSIS_DIR/trufflehog-results-$DATE.json" . || true
    fi
}

# Function to check system health
check_system_health() {
    log_message "Checking system health..."
    
    # Check disk space
    local disk_usage=$(df -h . | awk 'NR==2 {print $5}' | sed 's/%//')
    if [ "$disk_usage" -gt 90 ]; then
        send_alert "Disk usage is high: ${disk_usage}%" "high" "all"
    fi
    
    # Check memory usage
    local mem_usage=$(free | awk 'NR==2{printf "%.0f", $3*100/$2}')
    if [ "$mem_usage" -gt 90 ]; then
        send_alert "Memory usage is high: ${mem_usage}%" "high" "all"
    fi
    
    # Check if critical services are running
    if [ -f "docker-compose.yml" ]; then
        if ! docker-compose ps | grep -q "Up"; then
            send_alert "Docker services are not running properly" "high" "all"
        fi
    fi
}

# Function to generate comprehensive report
generate_report() {
    log_message "Generating comprehensive security report..."
    
    local report_file="$REPORT_DIR/enterprise-security-report-$DATE.md"
    
    cat > "$report_file" << EOF
# 🔒 Enterprise Security Report - GhostWire
Generated: $(date)
Timestamp: $TIMESTAMP

## 📊 Executive Summary
- **Project:** GhostWire Secure Mesh Communication
- **Report Date:** $(date)
- **Security Score:** $(cat "$ANALYSIS_DIR/security-score-$DATE.txt" 2>/dev/null || echo "N/A")/100

## 🔍 Security Analysis Results

### Dependency Vulnerabilities
EOF
    
    # Add safety results
    if [ -f "$ANALYSIS_DIR/safety-results-$DATE.json" ]; then
        echo "#### Safety Check Results" >> "$report_file"
        echo '```json' >> "$report_file"
        head -50 "$ANALYSIS_DIR/safety-results-$DATE.json" >> "$report_file"
        echo '```' >> "$report_file"
        echo "" >> "$report_file"
    fi
    
    # Add Bandit results
    if [ -f "$ANALYSIS_DIR/bandit-summary-$DATE.txt" ]; then
        echo "#### Bandit Security Scan" >> "$report_file"
        echo '```' >> "$report_file"
        head -30 "$ANALYSIS_DIR/bandit-summary-$DATE.txt" >> "$report_file"
        echo '```' >> "$report_file"
        echo "" >> "$report_file"
    fi
    
    # Add Semgrep results
    if [ -f "$ANALYSIS_DIR/semgrep-summary-$DATE.txt" ]; then
        echo "#### Semgrep Security Scan" >> "$report_file"
        echo '```' >> "$report_file"
        head -30 "$ANALYSIS_DIR/semgrep-summary-$DATE.txt" >> "$report_file"
        echo '```' >> "$report_file"
        echo "" >> "$report_file"
    fi
    
    # Add system health
    echo "## 🖥️ System Health" >> "$report_file"
    echo "- **Disk Usage:** $(df -h . | awk 'NR==2 {print $5}')" >> "$report_file"
    echo "- **Memory Usage:** $(free | awk 'NR==2{printf "%.1f%%", $3*100/$2}')" >> "$report_file"
    echo "" >> "$report_file"
    
    # Add recommendations
    echo "## 🎯 Recommendations" >> "$report_file"
    echo "1. Review and address critical vulnerabilities immediately" >> "$report_file"
    echo "2. Update dependencies to latest secure versions" >> "$report_file"
    echo "3. Implement security best practices in code" >> "$report_file"
    echo "4. Regular security training for development team" >> "$report_file"
    echo "" >> "$report_file"
    
    echo "## 📁 Generated Files" >> "$report_file"
    echo "- Safety Results: \`$ANALYSIS_DIR/safety-results-$DATE.json\`" >> "$report_file"
    echo "- Bandit Results: \`$ANALYSIS_DIR/bandit-results-$DATE.json\`" >> "$report_file"
    echo "- Semgrep Results: \`$ANALYSIS_DIR/semgrep-results-$DATE.json\`" >> "$report_file"
    echo "- Security Score: \`$ANALYSIS_DIR/security-score-$DATE.txt\`" >> "$report_file"
    
    log_message "Report generated: $report_file"
}

# Function to cleanup old files
cleanup_old_files() {
    log_message "Cleaning up old analysis files..."
    
    # Keep files for 30 days
    find "$ANALYSIS_DIR" -name "*-results-*.json" -mtime +30 -delete
    find "$ANALYSIS_DIR" -name "*-summary-*.txt" -mtime +30 -delete
    find "$REPORT_DIR" -name "enterprise-security-report-*.md" -mtime +30 -delete
    
    log_message "Cleanup completed"
}

# Main execution
main() {
    log_message "Starting Enterprise Security Monitor..."
    
    # Check if we're in the right directory
    if [ ! -f "requirements.txt" ] && [ ! -f "docker-compose.yml" ]; then
        echo "Error: Please run this script from the GhostWire root directory"
        exit 1
    fi
    
    # Run all security checks
    analyze_dependencies
    run_security_scans
    check_system_health
    
    # Generate report
    generate_report
    
    # Cleanup
    cleanup_old_files
    
    log_message "Enterprise Security Monitor completed successfully"
    
    # Display summary
    echo ""
    echo "🔒 Enterprise Security Monitor - Complete!"
    echo "📊 Security Score: $(cat "$ANALYSIS_DIR/security-score-$DATE.txt" 2>/dev/null || echo "N/A")/100"
    echo "📁 Reports: $REPORT_DIR/"
    echo "📁 Analysis: $ANALYSIS_DIR/"
    echo "📁 Logs: $LOG_DIR/"
    echo ""
    echo "🚨 Check the generated report for detailed analysis and recommendations."
}

# Run main function
main "$@"
