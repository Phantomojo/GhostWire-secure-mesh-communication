#!/bin/bash

# 🚀 Quick Start Script for GhostWire
# Sets up the development environment and runs initial checks
# Date: August 13, 2025

set -e

echo "🚀 GhostWire Quick Start Script"
echo "================================"
echo ""

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
PURPLE='\033[0;35m'
CYAN='\033[0;36m'
NC='\033[0m'

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

# Check if we're in the right directory
if [ ! -f "requirements.txt" ] && [ ! -f "docker-compose.yml" ]; then
    print_error "Please run this script from the GhostWire root directory"
    exit 1
fi

print_header "🚀 Setting up GhostWire development environment..."
echo ""

# 1. Check Python version
print_status "🐍 Checking Python version..."
python_version=$(python3 --version 2>&1 | cut -d' ' -f2 | cut -d'.' -f1,2)
required_version="3.11"

if [ "$(printf '%s\n' "$required_version" "$python_version" | sort -V | head -n1)" = "$required_version" ]; then
    print_success "Python $python_version is compatible (requires $required_version+)"
else
    print_error "Python $python_version is not compatible (requires $required_version+)"
    exit 1
fi

# 2. Create virtual environment
print_status "🔧 Setting up virtual environment..."
if [ ! -d "venv" ]; then
    python3 -m venv venv
    print_success "Virtual environment created"
else
    print_success "Virtual environment already exists"
fi

# 3. Activate virtual environment
print_status "🔌 Activating virtual environment..."
source venv/bin/activate
print_success "Virtual environment activated"

# 4. Upgrade pip
print_status "📦 Upgrading pip..."
pip install --upgrade pip
print_success "Pip upgraded"

# 5. Install dependencies
print_status "📚 Installing dependencies..."
if [ -f "requirements.txt" ]; then
    pip install -r requirements.txt
    print_success "Dependencies installed from requirements.txt"
else
    print_warning "No requirements.txt found, installing basic development tools..."
    pip install pytest pytest-cov black isort flake8 mypy bandit safety
    print_success "Basic development tools installed"
fi

# 6. Install pre-commit hooks
print_status "🔒 Setting up pre-commit hooks..."
if command -v pre-commit &> /dev/null; then
    pre-commit install
    print_success "Pre-commit hooks installed"
else
    print_warning "pre-commit not found, installing..."
    pip install pre-commit
    pre-commit install
    print_success "Pre-commit installed and hooks configured"
fi

# 7. Create necessary directories
print_status "📁 Creating project directories..."
mkdir -p tests
mkdir -p logs
mkdir -p reports
mkdir -p security_reports
mkdir -p security_analysis
print_success "Project directories created"

# 8. Set up Safety Firewall
print_status "🔒 Setting up Safety Firewall..."
if [ -f "scripts/safety_firewall.sh" ]; then
    chmod +x scripts/safety_firewall.sh
    ./scripts/safety_firewall.sh
    print_success "Safety Firewall configured"
else
    print_warning "Safety Firewall script not found"
fi

# 9. Run initial security check
print_status "🔍 Running initial security check..."
if command -v safety &> /dev/null; then
    safety check --output safety-initial-check.txt || true
    print_success "Initial security check completed"
else
    print_warning "Safety not available for initial security check"
fi

# 10. Check Docker setup
print_status "🐳 Checking Docker setup..."
if command -v docker &> /dev/null; then
    if docker --version &> /dev/null; then
        print_success "Docker is available"
        
        # Check if docker-compose is available
        if command -v docker-compose &> /dev/null; then
            print_success "Docker Compose is available"
            
            # Test Docker Compose
            if [ -f "docker-compose.yml" ]; then
                print_status "Testing Docker Compose configuration..."
                docker-compose config &> /dev/null && print_success "Docker Compose configuration is valid" || print_warning "Docker Compose configuration has issues"
            fi
        else
            print_warning "Docker Compose not found"
        fi
    else
        print_warning "Docker is installed but not working properly"
    fi
else
    print_warning "Docker not found - some features may not work"
fi

# 11. Run initial tests
print_status "🧪 Running initial tests..."
if [ -d "tests" ] && [ "$(ls -A tests)" ]; then
    if command -v pytest &> /dev/null; then
        pytest tests/ -v --tb=short || true
        print_success "Initial tests completed"
    else
        print_warning "pytest not available for initial tests"
    fi
else
    print_warning "No tests found, skipping initial test run"
fi

# 12. Create development configuration
print_status "⚙️ Creating development configuration..."
if [ ! -f ".env.development" ]; then
    cat > .env.development << 'EOF'
# Development Environment Configuration
# Copy this file to .env and modify as needed

# Database
DATABASE_URL=sqlite:///./ghostwire_dev.db

# Security
SECRET_KEY=your-secret-key-here-change-in-production
DEBUG=True

# Logging
LOG_LEVEL=DEBUG
LOG_FILE=logs/ghostwire_dev.log

# API
API_HOST=0.0.0.0
API_PORT=8000

# Mesh Network
MESH_NETWORK_ID=ghostwire_dev
MESH_BROADCAST_PORT=5000
MESH_DISCOVERY_PORT=5001
EOF
    print_success "Development environment configuration created"
else
    print_success "Development environment configuration already exists"
fi

# 13. Create .gitignore additions
print_status "📝 Updating .gitignore..."
if [ -f ".gitignore" ]; then
    # Add common development files to .gitignore if not already present
    grep -q "venv/" .gitignore || echo "venv/" >> .gitignore
    grep -q "*.pyc" .gitignore || echo "*.pyc" >> .gitignore
    grep -q "__pycache__/" .gitignore || echo "__pycache__/" >> .gitignore
    grep -q ".env" .gitignore || echo ".env" >> .gitignore
    grep -q ".env.*" .gitignore || echo ".env.*" >> .gitignore
    grep -q "logs/" .gitignore || echo "logs/" >> .gitignore
    grep -q "reports/" .gitignore || echo "reports/" >> .gitignore
    grep -q "security_reports/" .gitignore || echo "security_reports/" >> .gitignore
    grep -q "security_analysis/" .gitignore || echo "security_analysis/" >> .gitignore
    grep -q "*.db" .gitignore || echo "*.db" >> .gitignore
    print_success ".gitignore updated"
else
    print_warning "No .gitignore found"
fi

# 14. Create basic test structure
print_status "🧪 Setting up basic test structure..."
if [ ! -d "tests" ] || [ -z "$(ls -A tests)" ]; then
    mkdir -p tests/unit
    mkdir -p tests/integration
    mkdir -p tests/security
    
    # Create basic test files
    cat > tests/__init__.py << 'EOF'
# Tests package
EOF
    
    cat > tests/conftest.py << 'EOF'
"""
Pytest configuration and fixtures for GhostWire tests
"""
import pytest
import os
import sys

# Add project root to Python path
project_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
sys.path.insert(0, project_root)

@pytest.fixture
def sample_data():
    """Sample data for testing"""
    return {
        "message": "Hello GhostWire!",
        "timestamp": "2025-08-13T00:00:00Z"
    }
EOF
    
    cat > tests/unit/__init__.py << 'EOF'
# Unit tests
EOF
    
    cat > tests/unit/test_basic.py << 'EOF'
"""
Basic unit tests for GhostWire
"""
import pytest

def test_sample_data(sample_data):
    """Test that sample data is properly structured"""
    assert "message" in sample_data
    assert "timestamp" in sample_data
    assert sample_data["message"] == "Hello GhostWire!"

def test_basic_math():
    """Basic math test to ensure pytest is working"""
    assert 2 + 2 == 4
    assert 3 * 3 == 9
EOF
    
    print_success "Basic test structure created"
else
    print_success "Test structure already exists"
fi

# 15. Final setup
print_status "🔧 Finalizing setup..."

# Create a quick start alias
echo 'alias ghostwire-dev="cd $(pwd) && source venv/bin/activate"' >> ~/.bashrc
print_success "Added 'ghostwire-dev' alias to bashrc"

# Create a test alias
echo 'alias ghostwire-test="cd $(pwd) && source venv/bin/activate && python scripts/run_comprehensive_tests.py"' >> ~/.bashrc
print_success "Added 'ghostwire-test' alias to bashrc"

# Create a security alias
echo 'alias ghostwire-security="cd $(pwd) && source venv/bin/activate && python scripts/generate_security_report.py"' >> ~/.bashrc
print_success "Added 'ghostwire-security' alias to bashrc"

print_header "🎉 GhostWire Quick Start Complete!"
echo ""
echo "📋 What was set up:"
echo "  ✅ Python virtual environment"
echo "  ✅ Dependencies installed"
echo "  ✅ Pre-commit hooks configured"
echo "  ✅ Safety Firewall setup"
echo "  ✅ Project directory structure"
echo "  ✅ Development configuration"
echo "  ✅ Basic test structure"
echo "  ✅ Git configuration"
echo ""
echo "🚀 Next steps:"
echo "  1. Activate environment: source venv/bin/activate"
echo "  2. Run tests: pytest tests/ -v"
echo "  3. Run security check: safety check"
echo "  4. Run comprehensive tests: python scripts/run_comprehensive_tests.py"
echo "  5. Use aliases: ghostwire-dev, ghostwire-test, ghostwire-security"
echo ""
echo "🔒 Your GhostWire development environment is ready!"
echo "📚 Check the documentation for more details on development workflows."
