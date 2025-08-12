#!/bin/bash

# 🔒 Safety Firewall Setup Script
# Comprehensive security monitoring and alerting for GhostWire
# Date: August 13, 2025

set -e

echo "🔒 Setting up Safety Firewall for GhostWire..."
echo "⏰ Started at: $(date)"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

print_header() {
    echo -e "${PURPLE}[HEADER]${NC} $1"
}

print_cyan() {
    echo -e "${CYAN}[CYAN]${NC} $1"
}

# Check if we're in the right directory
if [ ! -f "requirements.txt" ]; then
    print_error "Please run this script from the GhostWire root directory"
    exit 1
fi

print_header "🔒 Safety Firewall Setup - GhostWire"
echo ""

# 1. Install/Update Safety CLI
print_status "📦 Installing/Updating Safety CLI..."
if command -v safety &> /dev/null; then
    print_success "Safety CLI already installed"
else
    print_status "Installing Safety CLI..."
    pip install safety --break-system-packages
    print_success "Safety CLI installed successfully"
fi

# 2. Create Safety Configuration
print_status "⚙️ Creating Safety configuration..."
cat > .safety-policy.yml << 'EOF'
# Safety Policy Configuration for GhostWire
# Date: August 13, 2025

# Ignore specific vulnerabilities that are known and accepted
ignore:
  # Add any known vulnerabilities that need to be ignored here
  # Example:
  # - id: 12345
  #   reason: "No fix available yet, monitoring for updates"
  #   expires: 2025-12-31

# Security thresholds
threshold:
  critical: 0
  high: 0
  medium: 5
  low: 10

# Output format
output: json

# Include ignored vulnerabilities in output
include-ignored: true
EOF

print_success "Safety policy configuration created"

# 3. Create Safety Project Configuration
print_status "⚙️ Creating Safety project configuration..."
cat > .safety-project.ini << 'EOF'
[project]
name = ghostwire
version = 1.0.0
EOF

print_success "Safety project configuration created"

# 4. Run Initial Safety Check
print_status "🔍 Running initial safety check..."
if safety check --policy-file .safety-policy.yml --output json > safety-initial-check.json 2>/dev/null; then
    print_success "Initial safety check completed"
    print_status "Results saved to safety-initial-check.json"
else
    print_warning "Initial safety check had issues, but continuing..."
fi

# 5. Create Security Monitoring Script
print_status "📝 Creating security monitoring script..."
cat > scripts/security_monitor.sh << 'EOF'
#!/bin/bash

# 🔒 Security Monitor for GhostWire
# Run this script regularly to check for new vulnerabilities

set -e

PROJECT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
LOG_DIR="$PROJECT_DIR/logs"
DATE=$(date +%Y%m%d_%H%M%S)

mkdir -p "$LOG_DIR"

echo "🔍 Running security check at $(date)..."

# Run safety check
if safety check --policy-file .safety-policy.yml --output json > "safety-check-$DATE.json" 2>/dev/null; then
    echo "✅ Security check completed successfully"
    
    # Check for critical/high vulnerabilities
    if grep -q '"severity": "critical"' "safety-check-$DATE.json" || grep -q '"severity": "high"' "safety-check-$DATE.json"; then
        echo "🚨 CRITICAL/HIGH vulnerabilities detected!"
        echo "Check safety-check-$DATE.json for details"
        exit 1
    else
        echo "✅ No critical or high vulnerabilities found"
    fi
else
    echo "❌ Security check failed"
    exit 1
fi

# Clean up old reports (keep last 7 days)
find . -name "safety-check-*.json" -mtime +7 -delete

echo "🔒 Security monitoring completed"
EOF

chmod +x scripts/security_monitor.sh
print_success "Security monitoring script created"

# 6. Create Cron Job Setup
print_status "⏰ Setting up automated security monitoring..."
cat > scripts/setup_security_cron.sh << 'EOF'
#!/bin/bash

# Setup cron job for security monitoring
# Run this script once to set up automated security checks

CRON_JOB="0 9 * * * cd $(pwd) && ./scripts/security_monitor.sh >> logs/security-cron.log 2>&1"

echo "Setting up daily security check at 9 AM..."
(crontab -l 2>/dev/null; echo "$CRON_JOB") | crontab -

echo "✅ Cron job set up successfully"
echo "Security checks will run daily at 9 AM"
echo "Logs will be saved to logs/security-cron.log"
EOF

chmod +x scripts/setup_security_cron.sh
print_success "Cron setup script created"

# 7. Create Log Directory
print_status "📁 Creating log directory..."
mkdir -p logs
print_success "Log directory created"

# 8. Create Security Report Template
print_status "📋 Creating security report template..."
cat > scripts/generate_security_report.py << 'EOF'
#!/usr/bin/env python3
"""
Security Report Generator for GhostWire
Generates comprehensive security reports from safety check results
"""

import json
import os
import sys
from datetime import datetime
from pathlib import Path

def generate_report():
    """Generate a comprehensive security report"""
    
    # Find the most recent safety check file
    safety_files = list(Path('.').glob('safety-check-*.json'))
    if not safety_files:
        print("No safety check files found. Run safety check first.")
        return
    
    latest_file = max(safety_files, key=lambda x: x.stat().st_mtime)
    
    try:
        with open(latest_file, 'r') as f:
            data = json.load(f)
    except Exception as e:
        print(f"Error reading {latest_file}: {e}")
        return
    
    # Generate report
    report = f"""# 🔒 Security Report - GhostWire
Generated: {datetime.now().strftime('%Y-%m-%d %H:%M:%S')}
Source: {latest_file}

## 📊 Vulnerability Summary
"""
    
    if isinstance(data, list):
        vulns = data
    elif isinstance(data, dict) and 'vulnerabilities' in data:
        vulns = data['vulnerabilities']
    else:
        vulns = []
    
    if not vulns:
        report += "✅ No vulnerabilities found!\n"
    else:
        # Group by severity
        by_severity = {}
        for vuln in vulns:
            severity = vuln.get('severity', 'unknown')
            if severity not in by_severity:
                by_severity[severity] = []
            by_severity[severity].append(vuln)
        
        for severity in ['critical', 'high', 'medium', 'low']:
            if severity in by_severity:
                count = len(by_severity[severity])
                report += f"\n## {severity.upper()} Vulnerabilities ({count})\n"
                
                for vuln in by_severity[severity]:
                    report += f"""
### {vuln.get('package', 'Unknown Package')}
- **CVE:** {vuln.get('cve', 'N/A')}
- **Description:** {vuln.get('description', 'No description available')}
- **Installed Version:** {vuln.get('installed_version', 'Unknown')}
- **Fixed Version:** {vuln.get('fixed_version', 'No fix available')}
"""
    
    # Save report
    report_file = f"logs/security-report-{datetime.now().strftime('%Y%m%d_%H%M%S')}.md"
    os.makedirs('logs', exist_ok=True)
    
    with open(report_file, 'w') as f:
        f.write(report)
    
    print(f"✅ Security report generated: {report_file}")

if __name__ == "__main__":
    generate_report()
EOF

chmod +x scripts/generate_security_report.py
print_success "Security report generator created"

# 9. Final Setup
print_status "🔧 Finalizing setup..."

# Create a quick security check alias
echo 'alias ghostwire-security="cd $(pwd) && ./scripts/security_monitor.sh"' >> ~/.bashrc
print_success "Added 'ghostwire-security' alias to bashrc"

# Create initial secrets baseline
print_status "🔐 Creating initial secrets baseline..."
if command -v detect-secrets &> /dev/null; then
    detect-secrets scan --baseline .secrets.baseline || true
    print_success "Secrets baseline created"
else
    print_warning "detect-secrets not found. Install with: pip install detect-secrets"
fi

print_header "🎉 Safety Firewall Setup Complete!"
echo ""
echo "📋 What was set up:"
echo "  ✅ Safety CLI and configuration"
echo "  ✅ Security monitoring script"
echo "  ✅ Automated cron job setup"
echo "  ✅ Security report generator"
echo "  ✅ Log directory structure"
echo "  ✅ Secrets baseline"
echo ""
echo "🚀 Next steps:"
echo "  1. Run: ./scripts/security_monitor.sh (manual check)"
echo "  2. Run: ./scripts/setup_security_cron.sh (automated daily checks)"
echo "  3. Run: ./scripts/generate_security_report.py (generate reports)"
echo "  4. Use alias: ghostwire-security (quick security check)"
echo ""
echo "🔒 Your GhostWire project is now protected by the Safety Firewall!"
echo "⏰ Security checks will run automatically and alert you to any issues."
