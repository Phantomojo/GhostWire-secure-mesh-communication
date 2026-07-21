# 🔧 GHOSTWIRE BACKEND COMMUNICATION TEST (CLI Only)

**No UI Required** - Test backend-to-backend communication directly via API

---

## ✅ CURRENT STATUS

### Node A - LOCAL
```
✅ Backend: RUNNING
   Port: 8080
   
✅ API Endpoints:
   /api/v2/profile ........... OK
   /api/v2/peers ............. OK
   /api/v2/security/dashboard  OK

✅ Network:
   Local IP: <YOUR_LOCAL_IP>
```

### Node B - REMOTE
```
⏳ Status: Needs manual verification
   Configure via: GHOSTWIRE_NODE_B_HOST environment variable
```

---

## 📋 TEST COMMANDS

### **On Node A - Run locally:**

```bash
# 1. Check backend status
curl -s http://127.0.0.1:8080/api/v2/profile | python3 -m json.tool

# 2. Check discovered peers
curl -s "http://127.0.0.1:8080/api/v2/peers?page=1&per_page=20" | python3 -m json.tool

# 3. Check mesh health
curl -s http://127.0.0.1:8080/api/v2/system/mesh-health | python3 -m json.tool
```

### **On Node B - SSH and run:**

```bash
# SSH to remote node
ssh <USER>@<REMOTE_NODE_HOST>

# Run these commands:
cd ~/ghostwire-live-test

# 1. Check if backend running
curl -s http://localhost:8080/api/v2/profile | python3 -m json.tool | head -10

# 2. Check peers
curl -s "http://localhost:8080/api/v2/peers?page=1&per_page=20" | python3 -m json.tool | head -20

# 3. Connect to Node A manually
# Replace <NODE_A_PEER_ID> and <NODE_A_IP> with actual values
curl -X POST http://localhost:8080/api/v2/peers/connect \
  -H "Content-Type: application/json" \
  -d '{"peer_id": "<NODE_A_PEER_ID>", "address": "/ip4/<NODE_A_IP>/tcp/4001"}'

# 4. Verify connection
curl -s "http://localhost:8080/api/v2/peers?page=1&per_page=20" | python3 -c "
import sys,json
d=json.load(sys.stdin)
print('Peers:', len(d['data']))
for p in d['data'][:5]:
    print(' -', p['node_id'][:30])
"
```

---

## 🎯 EXPECTED RESULTS

### **If backends can communicate:**

1. **mDNS Auto-Discovery** (30-60 seconds)
   - Both nodes should discover each other automatically
   - Peer count increases by 1 on both sides

2. **Manual Connection** (immediate)
   - After running curl connect command
   - Peer appears in list immediately
   - Trust level: "First Contact"

3. **API Responses**
   - Both `/api/v2/profile` return valid JSON
   - Both `/api/v2/peers` show peer count > 0
   - Node IDs are different

---

## 🔍 DEBUGGING

### **If remote backend not running:**

```bash
# SSH to remote node
ssh <USER>@<REMOTE_NODE_HOST>

# Start backend
cd ~/ghostwire-live-test
./ghostwire --port 8080 --mesh &

# Verify
curl -s http://localhost:8080/api/v2/profile
```

### **If peers not discovered:**

```bash
# Check mDNS
avahi-browse -a -t 2>/dev/null | grep -i ghostwire

# Check logs
tail -50 /tmp/ghostwire-*.log | grep -i "peer\|discover\|mesh"

# Manual connect (see commands above)
```

### **If API not responding:**

```bash
# Check if process running
ps aux | grep ghostwire | grep -v grep

# Check port
netstat -tlnp | grep 8080

# Restart backend
pkill -f ghostwire
./ghostwire --port 8080 --mesh &
```

---

## 📊 SUCCESS CRITERIA

Backend communication is working if:

- [ ] Both `/api/v2/profile` endpoints return valid JSON
- [ ] Node IDs are different on each machine
- [ ] Peer count > 0 on at least one machine
- [ ] Manual connect command returns success
- [ ] No connection errors in logs

---

## 🚀 QUICK TEST SEQUENCE

**Run these in order:**

1. **Node A:** Check status
   ```bash
   curl -s http://127.0.0.1:8080/api/v2/profile | python3 -c "import sys,json; print('Node:', json.load(sys.stdin)['data']['id'][:20])"
   ```

2. **Node B:** Check status (via SSH)
   ```bash
   # Set environment variable first:
   export GHOSTWIRE_NODE_B_HOST=<your-remote-host>
   export GHOSTWIRE_NODE_B_USER=<your-ssh-user>
   
   # Then run:
   curl -s http://localhost:8080/api/v2/profile | python3 -c "import sys,json; print('Node:', json.load(sys.stdin)['data']['id'][:20])"
   ```

3. **Node B:** Connect to Node A
   ```bash
   # Replace <NODE_A_PEER_ID> and <NODE_A_IP> with actual values
   curl -X POST http://localhost:8080/api/v2/peers/connect -H 'Content-Type: application/json' -d '{"peer_id": "<NODE_A_PEER_ID>", "address": "/ip4/<NODE_A_IP>/tcp/4001"}'
   ```

4. **Both:** Check peer count
   ```bash
   # Node A
   curl -s "http://127.0.0.1:8080/api/v2/peers?page=1&per_page=20" | python3 -c "import sys,json; print('Node A Peers:', len(json.load(sys.stdin)['data']))"
   
   # Node B
   curl -s "http://localhost:8080/api/v2/peers?page=1&per_page=20" | python3 -c "import sys,json; print('Node B Peers:', len(json.load(sys.stdin)['data']))"
   ```

---

**This tests backend communication WITHOUT the UI!** 🎊
