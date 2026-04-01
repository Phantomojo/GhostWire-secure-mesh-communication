# GHOSTWIRE USER PERSONAS
## Who Uses GhostWire & Why

**For:** AI Agent building the website  
**Use:** Design for these specific people, not "everyone"

---

## PERSONA 1: Investigative Journalist

### Profile

| Attribute | Value |
|-----------|-------|
| **Name** | Amina Mohamed |
| **Location** | Nairobi, Kenya |
| **Age** | 34 |
| **Work** | Investigative journalist, corruption beat |
| **Tech Skill** | High (uses Signal, Tor, encrypted email) |
| **Threat Model** | Government surveillance, source protection |

### Story

Amina investigates government corruption. Her sources are insiders who risk their lives to share information. She has been summoned by intelligence services twice. Her phone was seized at the airport.

She needs GhostWire to:
- Communicate with sources without revealing their identity
- Continue working during internet shutdowns
- Protect her notes and contacts if device is seized

### Website Needs

From the website, Amina needs to see:
1. **Security details upfront** — What encryption? What's the threat model?
2. **Journalist-specific guides** — "Using GhostWire for Source Protection"
3. **Verification** — PGP signatures, reproducible builds
4. **Emergency contacts** — Who to call if arrested?

### Design For Her

- ✅ Security page prominent
- ✅ Threat model clearly documented
- ✅ No-nonsense tone (she's busy, under pressure)
- ✅ Works on her old ThinkPad (not resource-heavy)

---

## PERSONA 2: Climate Activist

### Profile

| Attribute | Value |
|-----------|-------|
| **Name** | Marcus Chen |
| **Location** | Manila, Philippines |
| **Age** | 26 |
| **Work** | Climate justice organizer |
| **Tech Skill** | Medium (uses WhatsApp, Instagram) |
| **Threat Model** | Corporate surveillance, police infiltration |

### Story

Marcus organizes protests against coal plants. Corporate security monitors activist groups. Police have infiltrated organizing chats. Activists have been arrested before protests.

He needs GhostWire to:
- Coordinate actions without police knowing
- Create disposable identities for high-risk communications
- Work during protests (no internet, just mesh)

### Website Needs

From the website, Marcus needs to see:
1. **Simple setup** — "5 minutes to secure communications"
2. **Group coordination features** — How to create secure group chats
3. **Offline mode** — How to use without internet
4. **Legal support info** — What to do if arrested

### Design For Him

- ✅ Clear, simple onboarding
- ✅ Mobile-first (he's on his phone)
- ✅ Works on 3G (protest sites have bad coverage)
- ✅ Battery-efficient (long protest days)

---

## PERSONA 3: Community Network Operator

### Profile

| Attribute | Value |
|-----------|-------|
| **Name** | Thabo Mbeki |
| **Location** | Eastern Cape, South Africa |
| **Age** | 42 |
| **Work** | Community organizer, runs local mesh network |
| **Tech Skill** | Very high (runs Raspberry Pi nodes, knows networking) |
| **Threat Model** | ISP censorship, infrastructure control |

### Story

Thabo's community has no reliable ISP. He built a mesh network with Raspberry Pi nodes and LoRa radios. The community uses it for messaging, local services, emergency alerts.

He needs GhostWire to:
- Integrate with his existing mesh infrastructure
- Run on low-power hardware (Raspberry Pi, ESP32)
- Support hundreds of users in his community

### Website Needs

From the website, Thabo needs to see:
1. **Technical documentation** — API, configuration, deployment
2. **Hardware requirements** — What hardware is supported?
3. **Scaling guides** — How to support 100+ users
4. **Community forum** — Connect with other operators

### Design For Him

- ✅ Deep technical docs
- ✅ Hardware compatibility list
- ✅ Configuration examples
- ✅ Community support channels

---

## PERSONA 4: Disaster Response Coordinator

### Profile

| Attribute | Value |
|-----------|-------|
| **Name** | Sarah Williams |
| **Location** | Queensland, Australia |
| **Age** | 38 |
| **Work** | Emergency response coordinator |
| **Tech Skill** | Medium (uses radio, some digital tools) |
| **Threat Model** | Infrastructure failure, communication blackout |

### Story

Sarah coordinates disaster response (floods, bushfires). When disasters hit, cell towers fail. Emergency services can't communicate with affected communities. People die because they can't call for help.

She needs GhostWire to:
- Work when cell towers are down
- Connect emergency services with affected communities
- Run on battery/solar power
- Be simple enough for anyone to use

### Website Needs

From the website, Sarah needs to see:
1. **Disaster deployment guide** — "Set up in 10 minutes"
2. **Interoperability** — Does it work with existing emergency radio?
3. **Training materials** — How to teach volunteers
4. **Case studies** — Where has this been used in disasters?

### Design For Her

- ✅ Emergency setup guide prominent
- ✅ Works offline (critical)
- ✅ Simple enough for volunteers
- ✅ Rugged/hardware-agnostic

---

## PERSONA 5: Whistleblower

### Profile

| Attribute | Value |
|-----------|-------|
| **Name** | Anonymous |
| **Location** | Unknown (likely US or EU) |
| **Age** | Unknown |
| **Work** | Government/Corporate insider |
| **Tech Skill** | Varies (may be high, may be novice) |
| **Threat Model** | Retaliation, identification, prosecution |

### Story

This person has evidence of serious wrongdoing (war crimes, corporate fraud, government abuse). They need to share it with journalists without revealing their identity. Their career, freedom, and safety are at stake.

They need GhostWire to:
- Send documents anonymously
- Leave no trace of communication
- Work without phone number or ID
- Be truly secure (not "marketing secure")

### Website Needs

From the website, this person needs to see:
1. **Anonymity guarantees** — No logs, no ID, no phone number
2. **Secure drop instructions** — How to send documents safely
3. **Threat model** — What GhostWire protects against (and what it doesn't)
4. **Tor integration** — Can I use this over Tor?

### Design For Them

- ✅ No tracking on website (they may be monitored)
- ✅ Clear security limitations (honesty builds trust)
- ✅ Tor .onion mirror linked
- ✅ No email collection (they can't risk it)

---

## PERSONA 6: Privacy-Conscious Developer

### Profile

| Attribute | Value |
|-----------|-------|
| **Name** | Alex Petrov |
| **Location** | Berlin, Germany |
| **Age** | 29 |
| **Work** | Software developer |
| **Tech Skill** | Very high (contributes to open source) |
| **Threat Model** | Corporate surveillance, data harvesting |

### Story

Alex is tired of surveillance capitalism. They use Linux, self-host services, avoid Big Tech. They want to support and contribute to GhostWire because it aligns with their values.

They need GhostWire to:
- Be truly open source (not "open source washing")
- Have a clear contribution process
- Be technically sound (good architecture)

### Website Needs

From the website, Alex needs to see:
1. **GitHub link** — Is the code actually open?
2. **Architecture docs** — Is this well-built?
3. **Contribution guide** — How can I help?
4. **Security audits** — Has this been reviewed?

### Design For Them

- ✅ GitHub repo prominent
- ✅ Technical blog/architecture docs
- ✅ Clear contribution process
- ✅ Security audit reports

---

## SUMMARY: WHAT ALL PERSONAS NEED

| Need | Journalist | Activist | Operator | Responder | Whistleblower | Developer |
|------|------------|----------|----------|-----------|---------------|-----------|
| **Downloads** | ✅ | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Security info** | ✅ | ⚠️ | ✅ | ⚠️ | ✅ | ✅ |
| **Setup guide** | ⚠️ | ✅ | ✅ | ✅ | ⚠️ | ✅ |
| **Technical docs** | ❌ | ❌ | ✅ | ⚠️ | ❌ | ✅ |
| **Community** | ⚠️ | ✅ | ✅ | ⚠️ | ❌ | ✅ |
| **Donate info** | ❌ | ❌ | ⚠️ | ❌ | ❌ | ✅ |
| **Threat model** | ✅ | ⚠️ | ✅ | ⚠️ | ✅ | ✅ |
| **Use cases** | ✅ | ✅ | ✅ | ✅ | ✅ | ❌ |

**Legend:** ✅ Critical, ⚠️ Nice to have, ❌ Not needed

---

## DESIGN IMPLICATIONS

### Homepage Must Answer (In Order)

1. **What is this?** (5 seconds to understand)
2. **Is it safe?** (Security info visible)
3. **How do I get it?** (Download button obvious)
4. **Is it for me?** (Use cases clear)
5. **Can I trust it?** (Community, audits, transparency)

### Navigation Priority

1. Download (most important CTA)
2. Documentation (how to use it)
3. Security (why trust it)
4. Community (support, contribute)
5. About (mission, manifesto)

### Content Priority

1. Downloads + installation
2. Quick start guide
3. Security/threat model
4. Use cases (who is this for)
5. Technical documentation
6. Community/contribution
7. About/manifesto
8. Donate

---

## MOBILE CONSIDERATIONS

**70%+ of users will be on phones:**
- Activists (protests)
- Journalists (in the field)
- Disaster responders (emergency sites)
- Community users (no desktop)

**Design mobile-first:**
- Downloads accessible on mobile
- Setup guides readable on small screens
- Touch-friendly (gloves, stress)
- Low bandwidth (3G, expensive data)
- Battery efficient (no power for long)

---

## ACCESSIBILITY CONSIDERATIONS

**Users may have:**
- Disabilities (visual, motor, cognitive)
- Stress/trauma (crisis situations)
- Language barriers (not native English speakers)
- Low tech literacy (first time using secure tools)

**Design for:**
- Screen reader compatibility
- High contrast mode
- Simple language (avoid jargon)
- Multiple languages (eventually)
- Clear, calm design (not chaotic)

---

**For AI Agent:** Design for **Amina the Journalist** first. If it works for her (under pressure, high stakes, high skill), it will work for everyone else.
