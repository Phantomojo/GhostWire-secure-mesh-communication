# 🔧 GHOSTWIRE BACKEND COMMUNICATION TEST (CLI Only)

**No UI Required** - Test backend-to-backend communication directly via API

---

## ✅ CURRENT STATUS

### PC-A (omen) - LOCAL
```
✅ Backend: RUNNING
   Node ID: 88ef8419-e3b2-4d27-90f7-9d22db444a6b
   Profile: PHANTOMOJO
   Port: 8080
   
✅ API Endpoints:
   /api/v2/profile ........... OK
   /api/v2/peers ............. OK (5 peers discovered via mDNS)
   /api/v2/security/dashboard  OK

✅ Network:
   Local IP: 192.168.100.7
   Ping to stratsytem: OK
```

### PC-B (stratsytem) - REMOTE
```
⏳ Status: Needs manual verification
   Expected Node ID: a19cc36b-59d3-4d02-aefa-d31930701243
```

---

## 📋 TEST COMMANDS

### **On PC-A (omen) - Run locally:**

```bash
# 1. Check backend status
curl -s http://127.0.0.1:8080/api/v2/profile | python3 -m json.tool

# 2. Check discovered peers
curl -s "http://127.0.0.1:8080/api/v2/peers?page=1&per_page=20" | python3 -m json.tool

# 3. Check mesh health
curl -s http://127.0.0.1:8080/api/v2/system/mesh-health | python3 -m json.tool
```

### **On PC-B (stratsytem) - SSH and run:**

```bash
# SSH to stratsytem
ssh root@100.105.58.61

# Run these commands:
cd ~/ghostwire-live-test

# 1. Check if backend running
curl -s http://localhost:8080/api/v2/profile | python3 -m json.tool | head -10

# 2. Check peers
curl -s "http://localhost:8080/api/v2/peers?page=1&per_page=20" | python3 -m json.tool | head -20

# 3. Connect to PC-A manually
curl -X POST http://localhost:8080/api/v2/peers/connect \
  -H "Content-Type: application/json" \
  -d '{"peer_id": "88ef8419-e3b2-4d27-90f7-9d22db444a6b", "address": "/ip4/192.168.100.7/tcp/4001"}'

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

### **If stratsytem backend not running:**

```bash
# SSH to stratsytem
ssh root@100.105.58.61

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

1. **PC-A:** Check status
   ```bash
   curl -s http://127.0.0.1:8080/api/v2/profile | python3 -c "import sys,json; print('Node:', json.load(sys.stdin)['data']['id'][:20])"
   ```

2. **PC-B:** Check status (via SSH)
   ```bash
   ssh root@100.105.58.61 "curl -s http://localhost:8080/api/v2/profile | python3 -c 'import sys,json; print(\"Node:\", json.load(sys.stdin)[\"data\"][\"id\"][:20])'"
   ```

3. **PC-B:** Connect to PC-A
   ```bash
   ssh root@100.105.58.61 "curl -X POST http://localhost:8080/api/v2/peers/connect -H 'Content-Type: application/json' -d '{\"peer_id\": \"88ef8419-e3b2-4d27-90f7-9d22db444a6b\", \"address\": \"/ip4/192.168.100.7/tcp/4001\"}'"
   ```

4. **Both:** Check peer count
   ```bash
   # PC-A
   curl -s "http://127.0.0.1:8080/api/v2/peers?page=1&per_page=20" | python3 -c "import sys,json; print('PC-A Peers:', len(json.load(sys.stdin)['data']))"
   
   # PC-B
   ssh root@100.105.58.61 "curl -s 'http://localhost:8080/api/v2/peers?page=1&per_page=20' | python3 -c 'import sys,json; print(\"PC-B Peers:\", len(json.load(sys.stdin)[\"data\"]))'"
   ```

---

**This tests backend communication WITHOUT the UI!** 🎊
