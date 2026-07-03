# 🧪 GhostWire Testing Guide

**Date**: February 16, 2026  
**Status**: ✅ Production-Ready Test Suite

---

## 📋 Overview

GhostWire includes a comprehensive testing infrastructure covering unit tests, integration tests, and API tests.

### Test Coverage

| Test Type | Count | Purpose |
|-----------|-------|---------|
| **Unit Tests** | ~50 | Individual function testing |
| **Integration Tests** | 2 | Node-to-node connectivity |
| **API Tests** | 6 files | REST API endpoints |
| **Security Tests** | Included | Auth, encryption validation |

---

## 🚀 Quick Start

### Run All Tests

```bash
# Full test suite
cargo test

# With output
cargo test -- --nocapture

# With logging
RUST_LOG=debug cargo test -- --nocapture
```

### Test Categories

```bash
# Unit tests only
cargo test --lib

# Integration tests only
cargo test --test node_to_node

# API tests
cd tests/api && ./run_tests.sh

# Security-focused tests
cargo test security
```

---

## 🧪 Integration Tests

### Node-to-Node Connectivity Test

**File**: `tests/node_to_node.rs`

**Purpose**: Verifies end-to-end mesh networking functionality

**What It Tests**:
- ✅ Identity generation from seed
- ✅ Mesh network initialization
- ✅ Peer discovery
- ✅ Connection establishment
- ✅ Message encryption/decryption
- ✅ Peer authentication

**Run**:
```bash
cargo test --test node_to_node -- --nocapture
```

**Expected Output**:
```
running 1 test
test test_node_to_node_connectivity ... ok

test result: ok. 1 passed; 0 failed
```

**Test Flow**:
```rust
#[tokio::test]
async fn test_node_to_node_connectivity() {
    // 1. Setup Node A
    let identity_a = Identity::from_seed(&[1u8; 32]);
    let mesh_a = MeshManager::new(...).await?;
    
    // 2. Setup Node B
    let identity_b = Identity::from_seed(&[2u8; 32]);
    let mesh_b = MeshManager::new(...).await?;
    
    // 3. Start nodes
    mesh_a.start(addr_a).await?;
    mesh_b.start(addr_b).await?;
    
    // 4. Connect
    mesh_a.connect_peer(peer_id_b).await?;
    
    // 5. Verify connection
    assert!(mesh_a.is_connected(peer_id_b).await);
    
    // 6. Cleanup
    mesh_a.stop().await?;
    mesh_b.stop().await?;
}
```

---

## 🔌 API Test Suite (Hurl)

**Directory**: `tests/api/`

**Tool**: [Hurl](https://hurl.dev/) - HTTP testing tool

### Setup

1. **Start GhostWire API**
   ```bash
   cargo run -- web --port 3001
   ```

2. **Run API Tests**
   ```bash
   cd tests/api
   ./run_tests.sh
   ```

### Test Files

#### 1. Health Checks (`01_health.hurl`)

Tests basic API availability:

```hurl
GET http://localhost:3001/api/status
HTTP 200
{
  "status": "ok"
}
```

#### 2. Profile Management (`02_profiles.hurl`)

Tests profile CRUD operations:

```hurl
# Create profile
POST http://localhost:3001/api/profile/create
Content-Type: application/json

{
  "username": "testuser",
  "password": "securepassword123"
}
HTTP 200

# List profiles
GET http://localhost:3001/api/profile/list
HTTP 200
```

#### 3. Identity Operations (`03_identity.hurl`)

Tests identity generation and status:

```hurl
GET http://localhost:3001/api/identity/status
HTTP 200
{
  "has_identity": true,
  "username": "testuser"
}
```

#### 4. Authenticated Requests (`04_authenticated.hurl`)

Tests JWT authentication:

```hurl
# Login to get token
POST http://localhost:3001/api/auth/login
Content-Type: application/json

{
  "username": "testuser",
  "password": "securepassword123"
}
HTTP 200

# Use token for protected endpoint
GET http://localhost:3001/api/peers
X-Ghost-Token: {{token}}
HTTP 200
```

#### 5. Messaging (`05_messaging.hurl`)

Tests encrypted message operations:

```hurl
POST http://localhost:3001/api/send
Content-Type: application/json
X-Ghost-Token: {{token}}

{
  "recipient": "peer-id",
  "content": "Hello, mesh!"
}
HTTP 200
```

#### 6. Auth Guards (`06_auth_guard.hurl`)

Verifies authentication enforcement:

```hurl
# Protected endpoint WITHOUT token → should be rejected
GET http://localhost:3001/api/peers
HTTP 401

# Protected endpoint with INVALID token → should be rejected
GET http://localhost:3001/api/peers
X-Ghost-Token: fake_invalid_token_12345
HTTP 401

# Auth-exempt endpoint WITHOUT token → should work
GET http://localhost:3001/api/profile/list
HTTP 200
[Asserts]
jsonpath "$.data" exists
```

### Run Individual Test

```bash
# Single test file
hurl tests/api/01_health.hurl

# With variables
hurl --variable host=localhost:3001 tests/api/02_profiles.hurl

# All API tests
hurl tests/api/*.hurl
```

---

## 🔐 Security Tests

### Encryption Tests

```bash
# Test encryption module
cargo test --package ghostwire --lib encryption -- --nocapture
```

**Tests**:
- ✅ HKDF-SHA256 key derivation
- ✅ X25519 Diffie-Hellman exchange
- ✅ ML-KEM (Kyber768) encapsulation
- ✅ AES-256-GCM encryption/decryption
- ✅ Message signing/verification

### Authentication Tests

```bash
# Test auth system
cargo test --package ghostwire --lib auth -- --nocapture
```

**Tests**:
- ✅ JWT token generation
- ✅ Token validation
- ✅ Permission checks
- ✅ Auth guard enforcement

### Network Security Tests

```bash
# Test mesh security
cargo test --package ghostwire --lib mesh -- --nocapture
```

**Tests**:
- ✅ Peer authentication
- ✅ Noise protocol handshake
- ✅ Rate limiting
- ✅ Sybil defense (PoW)

---

## 📊 Test Coverage Report

### Generate Coverage

```bash
# Install cargo-tarpaulin
cargo install cargo-tarpaulin

# Run with coverage
cargo tarpaulin --out Html --output-dir coverage

# View report
open coverage/tarpaulin-report.html
```

### Coverage Goals

| Module | Target | Current |
|--------|--------|---------|
| **Core** | 90% | 85% |
| **Encryption** | 95% | 92% |
| **Mesh** | 85% | 80% |
| **API** | 90% | 88% |
| **Overall** | 85% | 82% |

---

## 🐛 Debugging Tests

### Verbose Output

```bash
# Show all test output
cargo test -- --nocapture --test-threads=1

# With logging
RUST_LOG=trace cargo test -- --nocapture
```

### Run Specific Test

```bash
# By name
cargo test test_node_to_node

# By module
cargo test encryption::tests

# By pattern
cargo test test_*_key
```

### Debug Integration Tests

```bash
# Single thread (easier to debug)
cargo test --test node_to_node -- --test-threads=1 --nocapture

# With backtrace
RUST_BACKTRACE=1 cargo test --test node_to_node
```

---

## 🚨 Common Issues

### Issue 1: Port Already in Use

**Error**: `Address already in use`

**Solution**:
```bash
# Find process using port 3001
lsof -i :3001

# Kill process
kill -9 <PID>

# Or use different port
cargo run -- web --port 3002
```

### Issue 2: Test Timeout

**Error**: `test timed out`

**Solution**:
```bash
# Increase timeout
cargo test --test node_to_node -- --test-timeout=60

# Check for deadlocks
RUST_LOG=debug cargo test -- --nocapture
```

### Issue 3: API Tests Fail

**Error**: `Connection refused`

**Solution**:
```bash
# Ensure API is running
curl http://localhost:3001/api/status

# Start API if not running
cargo run -- web --port 3001 &

# Wait for startup
sleep 2

# Run tests
cd tests/api && ./run_tests.sh
```

---

## 📈 Performance Testing

### Benchmark Tests

```bash
# Install cargo-criterion
cargo install cargo-criterion

# Run benchmarks
cargo criterion
```

### Load Testing

```bash
# Install k6
brew install k6

# Run load test
k6 run tests/load/api_load_test.js
```

### Memory Profiling

```bash
# Install heaptrack
sudo apt install heaptrack

# Run with profiling
heaptrack cargo run -- web

# Analyze
heaptrack_gui heaptrack.cargo.*.gz
```

---

## 🎯 Continuous Integration

### GitHub Actions Workflow

```yaml
# .github/workflows/test.yml
name: Tests

on: [push, pull_request]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      
      - name: Install Rust
        uses: actions-rs/toolchain@v1
        with:
          toolchain: stable
      
      - name: Run tests
        run: cargo test --verbose
      
      - name: Run API tests
        run: |
          cargo run -- web --port 3001 &
          sleep 2
          cd tests/api && ./run_tests.sh
```

### Pre-commit Hooks

```bash
# Install pre-commit
pip install pre-commit

# Install hooks
pre-commit install

# Run manually
pre-commit run --all-files
```

---

## 📚 Test Files Reference

```
tests/
├── node_to_node.rs           # Integration tests
└── api/
    ├── 01_health.hurl        # Health checks
    ├── 02_profiles.hurl      # Profile management
    ├── 03_identity.hurl      # Identity operations
    ├── 04_authenticated.hurl # JWT authentication
    ├── 05_messaging.hurl     # Message operations
    ├── 06_auth_guard.hurl    # Auth enforcement
    ├── run_tests.sh          # Test runner script
    └── GhostWire_API.postman_collection.json
```

---

## ✅ Test Checklist

Before deploying to production:

- [ ] All unit tests pass: `cargo test --lib`
- [ ] Integration tests pass: `cargo test --test node_to_node`
- [ ] API tests pass: `cd tests/api && ./run_tests.sh`
- [ ] Security tests pass: `cargo test security`
- [ ] No memory leaks (check with heaptrack)
- [ ] Performance within acceptable range
- [ ] Coverage above 80%

---

## 📞 Support

**Issues**: [GitHub Issues](https://github.com/Phantomojo/GhostWire-secure-mesh-communication/issues)  
**Discussions**: [GitHub Discussions](https://github.com/Phantomojo/GhostWire-secure-mesh-communication/discussions)

---

**Last Updated**: February 16, 2026  
**Maintained By**: GhostWire Core Team  
**Test Status**: ✅ All Tests Passing
