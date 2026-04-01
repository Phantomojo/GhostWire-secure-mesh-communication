# 🚀 STRATSYTEM DEPLOYMENT INSTRUCTIONS

## ⚠️ SSH Connection Issues Detected

Automated deployment to stratsytem is failing due to SSH timeouts.
Please deploy manually using these instructions:

---

## 📋 MANUAL DEPLOYMENT STEPS

### **Step 1: SSH to Stratsytem**

```bash
# Option A: Regular SSH
ssh root@100.105.58.61

# Option B: Tailscale SSH (if configured)
tailscale ssh stratsytem
```

### **Step 2: Stop Existing Process**

```bash
pkill -f ghostwire
```

### **Step 3: Copy New Binary from Omen**

**From stratsytem, run:**
```bash
scp root@100.88.234.14:/home/ph/GhostWire-secure-mesh-communication/ghostwire/target/release/ghostwire ~/ghostwire-live-test/
```

**Or from omen, run:**
```bash
scp /home/ph/GhostWire-secure-mesh-communication/ghostwire/target/release/ghostwire root@100.105.58.61:~/ghostwire-live-test/
```

### **Step 4: Start GhostWire on Stratsytem**

```bash
cd ~/ghostwire-live-test
chmod +x ghostwire
nohup ./ghostwire --port 8080 --mesh > /tmp/ghostwire-strat.log 2>&1 &
```

### **Step 5: Verify Deployment**

```bash
# Check process is running
ps aux | grep ghostwire | grep -v grep

# Check UI is being served
curl -s http://localhost:8080/ | grep -o "index-.*\.js"
# Should show: index-DYexBQoL.js

# Check API is responding
curl -s http://localhost:8080/api/v2/profile | python3 -m json.tool | head -10
```

---

## ✅ VERIFICATION CHECKLIST

After deployment, verify:

- [ ] Process running: `ps aux | grep ghostwire`
- [ ] UI file correct: `index-DYexBQoL.js`
- [ ] API responding: `/api/v2/profile` returns JSON
- [ ] CORS headers: `curl -v http://localhost:8080/api/v2/profile | grep access-control`
- [ ] No errors in log: `tail -20 /tmp/ghostwire-strat.log`

---

## 🔧 DEBUGGING

If issues occur:

### **Check Logs**
```bash
tail -50 /tmp/ghostwire-strat.log | grep -i "error\|panic"
```

### **Check Port**
```bash
netstat -tlnp | grep 8080
```

### **Restart Service**
```bash
pkill -f ghostwire
sleep 2
cd ~/ghostwire-live-test
./ghostwire --port 8080 --mesh &
```

---

## 📊 EXPECTED STATUS AFTER DEPLOYMENT

| Metric | Expected Value |
|--------|----------------|
| Process | Running |
| UI File | `index-DYexBQoL.js` |
| Node ID | `a19cc36b-59d3-4d02-aefa-d31930701243` |
| Profile | strat (or create new) |
| Port | 8080 |
| CORS | Enabled |

---

## 🎯 POST-DEPLOYMENT TEST

Once stratsytem is updated:

1. **Open browser on stratsytem:** `http://localhost:8080`
2. **Or via SSH tunnel from omen:**
   ```bash
   ssh -L 8081:localhost:8080 root@100.105.58.61 -N &
   # Then open: http://localhost:8081
   ```
3. **Create/load profile**
4. **Wait 30 seconds for peer discovery**
5. **Check peer count shows 1 on both nodes**
6. **Test messaging between nodes**

---

**Run these commands on stratsytem now!** 🚀
