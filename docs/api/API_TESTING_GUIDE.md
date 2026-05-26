# 🧪 GhostWire API Testing Guide

Comprehensive guide to testing the GhostWire API using CLI tools.

## Quick Start

### Option 1: Simple HTTPie Tests (Recommended)
```bash
./test-api.sh localhost:9097
```

### Option 2: Newman (Postman CLI)
```bash
./test-api-newman.sh localhost:9097
```

### Option 3: Manual cURL
```bash
curl -v http://localhost:9097/api/status
```

---

## Prerequisites

### System Requirements
- **curl** (always available)
- **bash** (for scripts)

### For HTTPie Testing
```bash
# Install HTTPie (pip3)
pip3 install httpie

# Or via Homebrew (macOS)
brew install httpie

# Or via APK (Alpine)
apk add httpie
```

### For Newman Testing
```bash
# Install Node.js
# https://nodejs.org/

# Install Newman globally
npm install -g newman
```

---

## Tool Comparison

| Feature | cURL | HTTPie | Newman |
|---------|------|--------|--------|
| Installation | Pre-installed | pip/brew | npm |
| Learning Curve | Steep | Easy | Medium |
| JSON Pretty-Print | Manual | Automatic | Included |
| Postman Integration | No | No | **Yes** |
| Automation | Scripts | Scripts | **Built-in** |
| Reporting | Manual | Manual | **JSON/HTML** |
| Performance Testing | ❌ | ✓ | ✓ |

**Recommendation**: Use **HTTPie** for quick testing, **Newman** for CI/CD.

---

## Starting the API

### Method 1: Direct Cargo
```bash
cd ghostwire
cargo run -- web --port 8080
```

### Method 2: Background Service
```bash
./start-ghostwire-bg.sh
```

### Verify it's running
```bash
curl http://localhost:9097/api/status
```

Expected:
```json
{
  "status": "ok",
  "version": "0.1.0"
}
```

---

## Test 1: HTTPie (Quickest)

### Installation
```bash
pip3 install httpie
```

### Basic Test
```bash
http GET http://localhost:9097/api/status
```

Output:
```
HTTP/1.1 200 OK
Content-Type: application/json

{
    "status": "ok",
    "version": "0.1.0"
}
```

### Run Full Suite
```bash
./test-api.sh localhost:9097
```

This tests:
- ✓ API connectivity
- ✓ Auth & token handling
- ✓ Profile management
- ✓ Identity endpoints
- ✓ Authenticated endpoints
- ✓ Error handling
- ✓ Performance metrics

### Sample Output
```
[12:30:45] ℹ Testing: API Root
[12:30:45] ℹ   GET http://localhost:9097/api/
[12:30:45] ✓ API Root - HTTP 200
[12:30:46] ℹ Testing: System Status
[12:30:46] ✓ System Status - HTTP 200
[12:30:47] ✓ Getting authentication token...
[12:30:47] ✓ Auth Token obtained: eyJhbGci...
...

Summary:
  Passed: 18
  Failed: 0
```

---

## Test 2: Newman (Automated + Reporting)

### Installation
```bash
npm install -g newman
```

### Run Full Suite
```bash
./test-api-newman.sh localhost:9097
```

This creates:
- `results-YYYYMMDD_HHMMSS.json` - Machine-readable results
- `results-YYYYMMDD_HHMMSS.html` - Visual report
- `summary-YYYYMMDD_HHMMSS.txt` - Quick summary
- `test-YYYYMMDD_HHMMSS.log` - Full test log

### Manual Run (with custom options)
```bash
newman run tests/api/GhostWire_API.postman_collection.json \
  --environment results/test-env.json \
  --reporters cli,json,html \
  --reporter-json-export results/results.json \
  --reporter-html-export results/results.html
```

### Sample Report (HTML)
Open `results/results-*.html` in a browser to see:
- ✓ Request/response details
- ✓ Timing information
- ✓ Error messages
- ✓ Assertion results

---

## Test 3: Manual cURL

### Get Status
```bash
curl http://localhost:9097/api/status
```

### Get Dev Token
```bash
TOKEN=$(curl -s http://localhost:9097/api/dev/token | grep -oP '"data":"?\K[^"]+')
echo $TOKEN
```

### Use Token in Request
```bash
curl -H "X-Ghost-Token: $TOKEN" \
  http://localhost:9097/api/v2/profile
```

### Create Profile
```bash
curl -X POST http://localhost:9097/api/v2/profile \
  -H "Content-Type: application/json" \
  -d '{"name": "test_profile"}'
```

### Send Message
```bash
curl -X POST http://localhost:9097/api/broadcast \
  -H "X-Ghost-Token: $TOKEN" \
  -H "Content-Type: application/json" \
  -d '{"content": "Test message"}'
```

### List Peers
```bash
curl -H "X-Ghost-Token: $TOKEN" \
  http://localhost:9097/api/v2/peers
```

---

## API Endpoints to Test

### Health & Status
| Endpoint | Method | Auth | Purpose |
|----------|--------|------|---------|
| `/api/status` | GET | No | API health check |
| `/api/` | GET | No | API version info |

### Authentication
| Endpoint | Method | Auth | Purpose |
|----------|--------|------|---------|
| `/api/dev/token` | GET | No | Get dev token |
| `/api/auth/login` | POST | No | Login endpoint |
| `/api/auth/verify` | POST | No | Verify token |

### Profile Management
| Endpoint | Method | Auth | Purpose |
|----------|--------|------|---------|
| `/api/v2/profile` | GET | No | List profiles |
| `/api/v2/profile` | POST | No | Create profile |
| `/api/v2/profile` | GET | **Yes** | Get profile details |
| `/api/v2/profile/username` | PUT | **Yes** | Update username |
| `/api/v2/profile/metadata` | PUT | **Yes** | Update metadata |

### Identity
| Endpoint | Method | Auth | Purpose |
|----------|--------|------|---------|
| `/api/v2/profile/public-key` | GET | **Yes** | Get public key |
| `/api/v2/profile/lock` | POST | **Yes** | Lock profile |
| `/api/v2/profile/unlock` | POST | **Yes** | Unlock profile |
| `/api/v2/profile/save` | POST | **Yes** | Save profile |

### Messaging
| Endpoint | Method | Auth | Purpose |
|----------|--------|------|---------|
| `/api/v2/messages` | GET | **Yes** | Get messages |
| `/api/broadcast` | POST | **Yes** | Send broadcast |
| `/api/send_message` | POST | **Yes** | Send direct message |

### Peers & Mesh
| Endpoint | Method | Auth | Purpose |
|----------|--------|------|---------|
| `/api/v2/peers` | GET | **Yes** | List peers |
| `/api/connect_peer` | POST | **Yes** | Connect peer |
| `/api/disconnect_peer` | POST | **Yes** | Disconnect peer |
| `/api/ping_peer` | POST | **Yes** | Ping peer |

### Security
| Endpoint | Method | Auth | Purpose |
|----------|--------|------|---------|
| `/api/v2/security/dashboard` | GET | **Yes** | Security stats |
| `/api/system/audit` | GET | **Yes** | Audit log |
| `/api/system/stats` | GET | **Yes** | System stats |
| `/api/system/wipe` | POST | **Yes** | Emergency wipe ⚠️ |

---

## Testing Workflows

### Workflow 1: Bootstrap (No Auth Required)
```bash
# 1. Check API status
http GET http://localhost:9097/api/status

# 2. List existing profiles
http GET http://localhost:9097/api/v2/profile

# 3. Create new profile
http POST http://localhost:9097/api/v2/profile \
  name=alice_profile

# 4. Get identity status
http GET http://localhost:9097/api/v2/profile
```

### Workflow 2: Get Token & Authenticate
```bash
# 1. Get dev token
TOKEN=$(http GET http://localhost:9097/api/dev/token | grep -oP '"data":"?\K[^"]+')

# 2. Verify token works
http GET http://localhost:9097/api/v2/profile \
  X-Ghost-Token:$TOKEN

# 3. Get security dashboard
http GET http://localhost:9097/api/v2/security/dashboard \
  X-Ghost-Token:$TOKEN
```

### Workflow 3: Send Messages
```bash
# 1. Get token
TOKEN=$(http GET http://localhost:9097/api/dev/token | grep -oP '"data":"?\K[^"]+')

# 2. Get peer list
http GET http://localhost:9097/api/v2/peers \
  X-Ghost-Token:$TOKEN

# 3. Broadcast message
http POST http://localhost:9097/api/broadcast \
  X-Ghost-Token:$TOKEN \
  content='Hello from CLI test'

# 4. Check messages
http GET http://localhost:9097/api/v2/messages \
  X-Ghost-Token:$TOKEN
```

### Workflow 4: Full Automated Test
```bash
# Run complete test suite
./test-api.sh localhost:9097

# Or with Newman for reporting
./test-api-newman.sh localhost:9097
```

---

## Expected Test Results

### Success Indicators
```
✓ API is online
✓ Auth Token obtained: eyJhbGci...
✓ API Root - HTTP 200
✓ System Status - HTTP 200
✓ List Profiles - HTTP 200
✓ Get Profile (Authenticated) - HTTP 200
✓ Get Public Key - HTTP 200
✓ Security Dashboard - HTTP 200
✓ List Peers - HTTP 200
✓ Get Messages - HTTP 200

Summary:
  Passed: 18
  Failed: 0
```

### Common Failures & Fixes

#### "API not reachable"
```bash
# Start the API first
cargo run -- web --port 8080
```

#### "401 Unauthorized"
```bash
# Get a new token
TOKEN=$(curl -s http://localhost:9097/api/dev/token | grep -oP '"data":"?\K[^"]+')
echo $TOKEN
```

#### "CORS error"
- ✓ Fixed in previous commit (added DELETE, OPTIONS methods)
- Re-test with: `./test-api.sh`

#### "404 Not Found"
- Check endpoint path
- Verify API version (v2 vs v1)
- Check documentation

---

## Performance Benchmarking

### Check Response Times
```bash
# Measure endpoint latency
http --verbose --timeout=10 \
  GET http://localhost:9097/api/v2/peers
```

### Stress Testing
```bash
# Install Apache Bench
apt-get install apache2-utils

# Run 100 requests
ab -n 100 -c 10 http://localhost:9097/api/status

# Or use wrk
brew install wrk
wrk -t4 -c100 -d30s http://localhost:9097/api/status
```

### Expected Performance
| Endpoint | Response Time |
|----------|----------------|
| `/status` | < 10ms |
| `/v2/profile` | < 50ms |
| `/v2/peers` | < 100ms |
| `/v2/messages` | < 100ms |

---

## Continuous Integration

### GitHub Actions Example
```yaml
name: API Tests

on: [push]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v2
      
      - name: Start API
        run: cargo run -- web --port 8080 &
      
      - name: Wait for API
        run: sleep 5
      
      - name: Run Tests
        run: ./test-api.sh localhost:9097
      
      - name: Upload Results
        uses: actions/upload-artifact@v2
        with:
          name: test-results
          path: results/
```

---

## Debugging

### Enable Verbose Output
```bash
# HTTPie verbose
http --verbose --print=HhBb \
  GET http://localhost:9097/api/status

# cURL verbose
curl -v http://localhost:9097/api/status

# Newman verbose
newman run tests/api/GhostWire_API.postman_collection.json \
  --verbose
```

### Inspect Network
```bash
# Monitor HTTP traffic (macOS)
tcpdump -i lo0 -n 'tcp port 8080'

# Or use Wireshark
wireshark
```

### Check API Logs
```bash
# View API logs
tail -f logs/ghostwire.log

# Filter by level
grep "ERROR\|WARN" logs/ghostwire.log
```

---

## Best Practices

✓ **Always start the API first**
```bash
cargo run -- web --port 8080
```

✓ **Use environment variables**
```bash
export API_HOST=localhost:9097
./test-api.sh $API_HOST
```

✓ **Save test results**
```bash
./test-api.sh > test-results.txt 2>&1
```

✓ **Test both authenticated and unauthenticated endpoints**
- Some endpoints require auth
- Some endpoints are public (for bootstrapping)

✓ **Check error responses**
```bash
curl -w "\nStatus: %{http_code}\n" \
  http://localhost:9097/api/invalid
```

---

## Resources

- **Postman Collection**: `tests/api/GhostWire_API.postman_collection.json`
- **API Documentation**: `docs/POSTMAN_API_GUIDE.md`
- **HTTPie Docs**: https://httpie.io/
- **Newman Docs**: https://learning.postman.com/docs/running-collections/using-newman-cli/

---

## Support

For issues or questions:
1. Check `results/*.log` for error details
2. Review API logs: `tail -f logs/ghostwire.log`
3. Test with cURL: `curl -v http://localhost:9097/api/status`
4. Open GitHub issue with test results

---

**Last Updated**: 2026-02-18  
**Status**: ✅ All testing tools configured and ready
