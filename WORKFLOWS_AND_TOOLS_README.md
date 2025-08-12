# 🚀 GhostWire Workflows and Tools Setup

This document describes the comprehensive workflows, CI/CD pipelines, security tools, and development utilities that have been set up for the GhostWire project, based on enterprise-grade practices from the Vanta-ledger repository.

## 📋 Table of Contents

- [Overview](#overview)
- [GitHub Actions Workflows](#github-actions-workflows)
- [Security Tools](#security-tools)
- [Development Scripts](#development-scripts)
- [Pre-commit Hooks](#pre-commit-hooks)
- [Quick Start Guide](#quick-start-guide)
- [Usage Examples](#usage-examples)
- [Troubleshooting](#troubleshooting)

## 🔍 Overview

GhostWire now includes a comprehensive set of workflows and tools that provide:

- **Automated CI/CD pipelines** with testing, security checks, and deployment
- **Enterprise-grade security monitoring** with vulnerability scanning and reporting
- **Code quality enforcement** through automated linting, formatting, and type checking
- **Comprehensive testing suites** for unit, integration, and security testing
- **Development automation** scripts for quick setup and maintenance

## 🚀 GitHub Actions Workflows

### 1. Main CI/CD Pipeline (`.github/workflows/main.yml`)

**Triggers:** Push to main/master/develop, Pull Requests

**Jobs:**
- **🧪 Test**: Runs tests on Python 3.11 and 3.12
- **🔒 Security**: Performs security scans with Bandit and Safety
- **🏗️ Build**: Builds and packages the application
- **🚀 Deploy Staging**: Deploys to staging environment (develop branch)
- **🚀 Deploy Production**: Deploys to production (main/master branch)

**Features:**
- Multi-Python version testing
- Code formatting checks (Black, isort)
- Linting (Flake8)
- Type checking (MyPy)
- Coverage reporting
- Security vulnerability scanning

### 2. Security Analysis (`.github/workflows/security.yml`)

**Triggers:** Push to main/master/develop, Pull Requests, Weekly schedule (Saturdays 2 AM UTC)

**Jobs:**
- **Security Scan**: Runs Bandit and Semgrep security analysis
- **Dependency Scan**: Checks dependencies with Safety
- **Secrets Detection**: Scans for exposed secrets with TruffleHog
- **Security Report**: Generates comprehensive security reports

**Features:**
- Automated weekly security scans
- Multiple security tools integration
- PR comments with security results
- Artifact retention for 30 days

### 3. Comprehensive Testing (`.github/workflows/testing.yml`)

**Triggers:** Push to main/master/develop, Pull Requests, Daily schedule (6 AM UTC)

**Jobs:**
- **Unit Tests**: Multi-OS, multi-Python version testing
- **Integration Tests**: Database and service integration testing
- **Performance Tests**: Benchmark and load testing
- **Security Tests**: Security-focused testing
- **Test Summary**: Comprehensive test reporting

**Features:**
- PostgreSQL and Redis service containers
- Performance benchmarking
- Security testing integration
- Detailed test summaries

## 🔒 Security Tools

### 1. Safety Firewall (`scripts/safety_firewall.sh`)

A comprehensive security monitoring system that:

- Installs and configures Safety CLI
- Creates security policies and thresholds
- Sets up automated daily security checks
- Generates security reports
- Creates cron jobs for continuous monitoring

**Usage:**
```bash
./scripts/safety_firewall.sh
```

**Features:**
- Configurable vulnerability thresholds
- Automated daily security monitoring
- Security score calculation
- Comprehensive reporting

### 2. Enterprise Security Monitor (`scripts/enterprise_security_monitor.sh`)

Enterprise-grade security analysis tool that:

- Runs comprehensive security scans
- Monitors system health
- Generates detailed security reports
- Provides alerting capabilities
- Calculates security scores

**Usage:**
```bash
./scripts/enterprise_security_monitor.sh
```

**Features:**
- Multiple security tool integration
- System health monitoring
- Configurable alerting
- Detailed reporting and analysis

### 3. Security Report Generator (`scripts/generate_security_report.py`)

Python script that generates comprehensive security reports from scan results.

**Usage:**
```bash
python scripts/generate_security_report.py
```

## 🛠️ Development Scripts

### 1. Quick Start (`scripts/quick_start.sh`)

Sets up the complete development environment:

- Python virtual environment
- Dependencies installation
- Pre-commit hooks setup
- Safety Firewall configuration
- Basic test structure
- Development configuration

**Usage:**
```bash
./scripts/quick_start.sh
```

**Features:**
- Automated environment setup
- Dependency verification
- Docker setup checking
- Test structure creation
- Useful aliases creation

### 2. Comprehensive Test Suite (`scripts/run_comprehensive_tests.py`)

Python-based testing framework that runs:

- Dependency checks
- Unit tests
- Code quality checks
- Security checks
- Integration tests
- Comprehensive reporting

**Usage:**
```bash
python scripts/run_comprehensive_tests.py
```

**Features:**
- Automated test execution
- Multiple tool integration
- Detailed logging
- HTML and markdown reports
- Performance metrics

## 🔧 Pre-commit Hooks

### Configuration (`.pre-commit-config.yaml`)

Automated code quality enforcement through:

- **Secret Detection**: detect-secrets, TruffleHog, git-secrets
- **Security Scanning**: Bandit security analysis
- **Code Formatting**: Black, isort
- **Linting**: Flake8, MyPy
- **File Validation**: YAML, JSON, Markdown validation
- **Shell Scripting**: ShellCheck, Dockerfile linting

**Setup:**
```bash
pip install pre-commit
pre-commit install
```

## 🚀 Quick Start Guide

### 1. Initial Setup

```bash
# Clone the repository
git clone <your-repo-url>
cd GhostWire-secure-mesh-communication

# Run the quick start script
./scripts/quick_start.sh
```

### 2. Activate Development Environment

```bash
# Activate virtual environment
source venv/bin/activate

# Or use the alias
ghostwire-dev
```

### 3. Run Tests

```bash
# Run basic tests
pytest tests/ -v

# Run comprehensive test suite
python scripts/run_comprehensive_tests.py

# Or use the alias
ghostwire-test
```

### 4. Security Checks

```bash
# Run security monitoring
./scripts/security_monitor.sh

# Run enterprise security monitor
./scripts/enterprise_security_monitor.sh

# Generate security report
python scripts/generate_security_report.py

# Or use the alias
ghostwire-security
```

### 5. Code Quality

```bash
# Format code
black .
isort .

# Lint code
flake8 .
mypy ghostwire/

# Run pre-commit hooks
pre-commit run --all-files
```

## 📊 Usage Examples

### Daily Development Workflow

```bash
# 1. Start development session
ghostwire-dev

# 2. Make code changes
# ... edit files ...

# 3. Run tests
ghostwire-test

# 4. Check security
ghostwire-security

# 5. Commit changes (pre-commit hooks run automatically)
git add .
git commit -m "Feature: Add new functionality"
```

### Security Monitoring

```bash
# Set up daily security monitoring
./scripts/setup_security_cron.sh

# Run manual security check
./scripts/security_monitor.sh

# Generate detailed security report
python scripts/generate_security_report.py
```

### CI/CD Pipeline

The GitHub Actions workflows automatically run on:

- **Push to main/master**: Full pipeline with production deployment
- **Push to develop**: Full pipeline with staging deployment
- **Pull Requests**: Full testing and security checks
- **Scheduled**: Daily testing, weekly security scans

## 🔧 Troubleshooting

### Common Issues

#### 1. Pre-commit Hooks Fail

```bash
# Update pre-commit hooks
pre-commit autoupdate

# Run manually to see detailed errors
pre-commit run --all-files
```

#### 2. Safety Checks Fail

```bash
# Check Safety installation
safety --version

# Reinstall if needed
pip install --upgrade safety

# Run with verbose output
safety check --verbose
```

#### 3. Tests Fail

```bash
# Check dependencies
pip list

# Reinstall test dependencies
pip install -r requirements.txt
pip install pytest pytest-cov black isort flake8 mypy

# Run tests with verbose output
pytest tests/ -v --tb=long
```

#### 4. Docker Issues

```bash
# Check Docker status
docker --version
docker-compose --version

# Test Docker Compose
docker-compose config

# Restart Docker service if needed
sudo systemctl restart docker
```

### Getting Help

1. **Check logs**: Look in `logs/` directory for detailed error logs
2. **Review reports**: Check `reports/` and `security_reports/` directories
3. **GitHub Actions**: Check workflow runs in the Actions tab
4. **Script help**: Most scripts have `--help` or `-h` options

## 📚 Additional Resources

### Documentation

- [GitHub Actions Documentation](https://docs.github.com/en/actions)
- [Pre-commit Documentation](https://pre-commit.com/)
- [Safety Documentation](https://pyup.io/safety/)
- [Bandit Documentation](https://bandit.readthedocs.io/)
- [Pytest Documentation](https://docs.pytest.org/)

### Security Resources

- [OWASP Top 10](https://owasp.org/www-project-top-ten/)
- [Python Security Best Practices](https://python-security.readthedocs.io/)
- [GitHub Security](https://security.github.com/)

## 🎯 Next Steps

1. **Customize Workflows**: Modify GitHub Actions workflows for your specific needs
2. **Configure Alerts**: Set up Slack, email, or webhook notifications
3. **Add Tests**: Create comprehensive test suites for your code
4. **Security Policies**: Customize security thresholds and policies
5. **Monitoring**: Set up continuous monitoring and alerting

---

**🔒 Your GhostWire project is now protected by enterprise-grade security and automation tools!**

For questions or issues, check the logs and reports generated by these tools, or refer to the troubleshooting section above.
