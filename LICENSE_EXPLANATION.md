# 📜 GhostWire Licensing Strategy

<div align="center">

**Dual Licensing Model: AGPL-3.0 + Commercial License**

*Balancing open source collaboration with commercial sustainability and abuse prevention*

</div>

---

## 🎯 Executive Summary

GhostWire operates under a **dual licensing model**:

1. **AGPL-3.0-or-later** - For open source, community, and non-commercial use
2. **Commercial License** - For proprietary, closed-source, or SaaS deployments

This approach:
- ✅ Keeps the software freely available for community use
- ✅ Prevents commercial exploitation without contribution back
- ✅ Provides a revenue path for sustainable development
- ✅ Maintains control over how the software is used commercially

---

## ⚠️ Why NOT Commons Clause?

We researched adding a "Commons Clause" restriction but found:

### **Legal Incompatibility**
- Commons Clause **conflicts with AGPL-3.0** terms
- AGPL grants freedom to use for any purpose; Commons Clause restricts commercial use
- These are **legally incompatible** - you cannot add Commons Clause to AGPL software
- See: [OpenSource StackExchange - Can Commons Clause be used along with AGPL?](https://opensource.stackexchange.com/questions/10045/can-commons-clause-be-used-along-with-agpl)

### **Community Trust Issues**
- Commons Clause is considered "open washing" - appearing open source while restricting freedoms
- Not OSI-approved; violates the Open Source Definition
- Damages community trust and adoption

### **Better Alternative: Dual Licensing**
- **Legally sound** - Used successfully by MongoDB, Elastic, iText, and others
- **Clear separation** - AGPL for open use, commercial for proprietary use
- **Revenue model** - Companies pay for closed-source deployments
- **Community friendly** - Full AGPL rights for open source users

---

## 📋 License Breakdown

### **AGPL-3.0-or-later** (Default)

**You CAN:**
- ✅ Use GhostWire for personal, research, educational purposes
- ✅ Modify and distribute the source code
- ✅ Deploy GhostWire internally within your organization
- ✅ Contribute improvements back to the community
- ✅ Create derivative works (must remain AGPL-3.0)

**You MUST:**
- 📌 Make source code available if you deploy GhostWire as a service (network use = distribution)
- 📌 Release modifications under AGPL-3.0
- 📌 Include copyright notices and license text
- 📌 Disclose source code to anyone who interacts with your modified version over a network

**You CANNOT (without commercial license):**
- ❌ Use GhostWire in proprietary, closed-source products
- ❌ Offer GhostWire as a SaaS without releasing your modifications
- ❌ Combine GhostWire with proprietary software without AGPL compliance
- ❌ Remove or alter copyright notices

### **Commercial License**

**Available for:**
- 🏢 Proprietary product integration
- 🏢 SaaS deployments without source disclosure requirements
- 🏢 Closed-source derivative works
- 🏢 Custom licensing terms for enterprise customers

**Contact:** licensing@ghostwire.cc (or create a GitHub issue for inquiries)

---

## 🛡️ Abuse Prevention Strategy

### **What We're Protecting Against**

1. **Commercial Exploitation**
   - Companies selling GhostWire-based services without contributing back
   - SaaS providers using AGPL loopholes to avoid source disclosure
   - Reselling without permission or revenue sharing

2. **Brand Misuse**
   - Unauthorized use of GhostWire name/trademarks
   - Misleading claims about affiliation or endorsement
   - Competing products using GhostWire branding

3. **Security Risks**
   - Malicious actors deploying vulnerable modified versions
   - Forks that remove security features
   - Unauthorized hosting that could enable attacks

### **How Dual Licensing Prevents Abuse**

| Threat | AGPL-3.0 Protection | Commercial License Control |
|--------|---------------------|---------------------------|
| **SaaS without contribution** | Requires source disclosure | Must pay for closed-source SaaS |
| **Proprietary integration** | Not allowed without compliance | Requires commercial license |
| **Reselling** | Must comply with AGPL terms | Explicit permission required |
| **Brand misuse** | Trademark law + license terms | Contract terms in commercial license |
| **Security vulnerabilities** | Community can audit code | Commercial support SLA available |

### **Additional Controls**

#### **1. GitHub Repository Settings**
- Make repository **public** for visibility
- **Fork prevention**: Monitor fork creation (GitHub doesn't prevent forks of public repos)
- **Issue templates**: Require license acknowledgment for commercial use questions
- **PR templates**: Require Contributor License Agreement (CLA) for contributions

#### **2. Contributor License Agreement (CLA)**
- All contributors must sign CLA
- Ensures we have rights to use and dual-license contributions
- Prevents future legal disputes
- Stored in: `docs/CLA.md`

#### **3. Trademark Protection**
- "GhostWire" name and logo are trademarked
- Commercial use requires explicit permission
- Community use allowed with attribution

#### **4. Monitoring & Enforcement**
- GitHub monitors public forks and usage
- Regular searches for unauthorized commercial use
- DMCA takedowns for violations
- Legal action for willful violations

---

## 🔧 GitHub CLI Setup for Public Repository

### **Make Repository Public**

```bash
# Navigate to repository
cd /path/to/GhostWire-secure-mesh-communication

# Make repository public (requires gh CLI authentication)
gh repo edit --accept-visibility-change-consequences --visibility public

# Verify the change
gh repo view --json name,visibility,isFork
```

### **Repository Settings to Configure**

```bash
# Disable forks if desired (note: this is controversial in open source)
# gh repo edit --disable-fork true

# Set default branch
gh repo edit --default-branch main

# Enable issues and projects
gh repo edit --enable-issues true
gh repo edit --enable-projects true

# Set homepage
gh repo edit --homepage "https://ghostwire.com"
```

### **Post-Public Verification**

```bash
# Check repository visibility
gh repo view | grep -i visibility

# List all collaborators
gh api repos/Phantomojo/GhostWire-secure-mesh-communication/collaborators

# Verify license is visible
curl -s https://github.com/Phantomojo/GhostWire-secure-mesh-communication | grep -i license
```

---

## 📄 Required Documentation Files

### **✅ Core License Files**
- `LICENSE` - AGPL-3.0 full text (already exists)
- `LICENSE_EXPLANATION.md` - This file, explaining dual licensing
- `COMMERCIAL_LICENSE.md` - Commercial license terms and contact

### **✅ Community Files**
- `README.md` - Updated with license badges and clarity
- `CONTRIBUTING.md` - Updated with CLA requirements
- `CODE_OF_CONDUCT.md` - Community standards (to be created)
- `SUPPORT.md` - Support contact information (to be created)

### **✅ Security & Legal**
- `SECURITY.md` - Updated with commercial license contact
- `docs/CLA.md` - Contributor License Agreement template
- `docs/TRADEMARK.md` - Trademark usage guidelines

---

## 🎯 License Enforcement Workflow

### **For Community Users**
1. Download from GitHub (AGPL-3.0)
2. Use, modify, distribute under AGPL terms
3. If deploying as service → disclose source code
4. Contribute improvements back (optional but encouraged)

### **For Commercial Users**
1. Evaluate AGPL-3.0 terms
2. If AGPL compliance is not feasible → contact for commercial license
3. Negotiate terms based on use case
4. Sign commercial license agreement
5. Deploy without AGPL restrictions

### **For Contributors**
1. Fork repository (AGPL-3.0)
2. Make changes
3. Sign CLA (one-time)
4. Submit pull request
5. Changes dual-licensed under AGPL + commercial

### **For Violators**
1. Detection (monitoring, reports, searches)
2. Cease and desist notice
3. Compliance deadline (typically 30 days)
4. DMCA takedown if unresolved
5. Legal action for willful violations

---

## 📊 Comparison: Commons Clause vs Dual Licensing

| Aspect | Commons Clause + AGPL | Dual Licensing (AGPL + Commercial) |
|--------|----------------------|-----------------------------------|
| **Legal Compatibility** | ❌ Incompatible with AGPL | ✅ Legally sound |
| **OSI Approval** | ❌ Not open source | ✅ AGPL is OSI-approved |
| **Community Trust** | ❌ Damaged (open washing) | ✅ Maintained |
| **Commercial Control** | ✅ Restricts selling | ✅ Requires commercial license |
| **Revenue Model** | ❌ Unclear | ✅ Clear commercial path |
| **Adoption Examples** | ❌ Few, controversial | ✅ MongoDB, Elastic, iText |
| **Enforcement** | ⚠️ Legally questionable | ✅ Well-established |
| **Recommendation** | ❌ **DO NOT USE** | ✅ **RECOMMENDED** |

---

## 🚀 Implementation Checklist

- [x] Research licensing options and best practices
- [x] Create LICENSE_EXPLANATION.md (this file)
- [ ] Update root LICENSE file with dual licensing notice
- [ ] Create COMMERCIAL_LICENSE.md
- [ ] Create CODE_OF_CONDUCT.md
- [ ] Create SUPPORT.md
- [ ] Update README.md with license clarity
- [ ] Update CONTRIBUTING.md with CLA requirements
- [ ] Update SECURITY.md with commercial contact
- [ ] Create docs/CLA.md (Contributor License Agreement)
- [ ] Create docs/TRADEMARK.md (Trademark Guidelines)
- [ ] Make repository public via GitHub CLI
- [ ] Configure GitHub repository settings
- [ ] Announce licensing model to community

---

## 📞 Questions?

### **For AGPL-3.0 Use**
- Check the [LICENSE](LICENSE) file
- Read the [AGPL-3.0 FAQ](https://www.gnu.org/licenses/agpl-3.0-faq.html)
- Open a GitHub issue for clarification

### **For Commercial Licensing**
- Email: licensing@ghostwire.cc
- GitHub Issue: Create a "Commercial License Inquiry" issue
- Website: [ghostwire.com/licensing](https://ghostwire.com/licensing) (when available)

### **For Legal Questions**
- Consult with legal counsel
- Contact: legal@ghostwire.com

---

<div align="center">

**GhostWire is committed to open source while ensuring sustainable development through responsible commercial licensing.**

*Last updated: March 2026*

</div>
