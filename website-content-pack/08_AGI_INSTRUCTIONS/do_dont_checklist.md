# GHOSTWIRE WEBSITE - AI AGENT INSTRUCTIONS
## Do's, Don'ts & Quality Checklist

**For:** Kimi AI or any AI agent building the website  
**Read This Before Starting**

---

## ✅ DO

### Philosophy & Approach

- [ ] **Build a PROJECT SITE** (like torproject.org, signal.org)
- [ ] **Prioritize downloads** — Most important CTA on homepage
- [ ] **Lead with mission** — "Infrastructure for Human Dignity"
- [ ] **Write for crisis users** — Journalists under threat, activists in hostile regimes
- [ ] **Be honest about limitations** — What GhostWire does NOT protect against
- [ ] **Make security prominent** — Cryptography details easy to find
- [ ] **Design mobile-first** — 70%+ users on phones, many on 3G
- [ ] **Optimize for speed** — < 500KB total, < 1.5s load time
- [ ] **Use real screenshots** — From actual GhostWire webui
- [ ] **Link to GitHub everywhere** — Source code, releases, contributors

### Content

- [ ] **Use provided copy** — See `03_CONTENT_PAGES/` for exact text
- [ ] **Keep it technical but accessible** — Engineers AND activists read this
- [ ] **Show use cases** — Journalists, activists, communities, disaster zones
- [ ] **Include network stats** — Even if static initially (nodes, messages, countries)
- [ ] **Link to manifesto** — `/docs/GHOSTWIRE_MANIFESTO.md`
- [ ] **Document cryptography** — ChaCha20-Poly1305, Ed25519, X25519
- [ ] **Add threat model** — Who GhostWire protects against

### Design

- [ ] **Use reference sites** — torproject.org, signal.org, bitcoin.org
- [ ] **Follow color palette** — Black (#0a0a0a), white, purple accent (#7e6ee3)
- [ ] **Use Inter font** — Clean, readable, professional
- [ ] **Make it accessible** — WCAG 2.1 AA minimum
- [ ] **Respect reduced motion** — `prefers-reduced-motion` media query
- [ ] **Progressive enhancement** — Works without JavaScript
- [ ] **Real imagery** — Actual GhostWire UI, hardware photos, network diagrams

### Technical

- [ ] **Use Astro** — Static site, minimal JS, best performance
- [ ] **Integrate GitHub API** — Show real releases, stars, contributors
- [ ] **Enable easy updates** — Markdown for blog, docs, security advisories
- [ ] **No tracking** — Or use privacy-respecting (Plausible, Fathom)
- [ ] **No cookies** — Unless essential
- [ ] **HTTPS only** — Enforce secure connections
- [ ] **Set up redirects** — Old URLs → new URLs if restructuring

---

## ❌ DON'T

### Critical Mistakes (DO NOT MAKE THESE)

- [ ] **Build a SaaS landing page** — This is infrastructure, not a product
- [ ] **Add a pricing page** — GhostWire is FREE, not freemium
- [ ] **Use "Get Started" CTAs** — Vague. Use "Download" or "Read Docs"
- [ ] **Add testimonials** — Feels fake for infrastructure
- [ ] **Use stock photos** — No "hackers in hoodies", no generic cybersecurity clipart
- [ ] **Add email capture** — Don't collect emails unless necessary
- [ ] **Add cookie banner** — Use no tracking instead
- [ ] **Over-engineer animations** — No parallax, no WebGL, no magnetic buttons
- [ ] **Make it heavy** — No 5MB bundles, no 50+ npm packages
- [ ] **Hide downloads** — Should be visible in < 3 seconds

### Design Mistakes

- [ ] **Don't use gradient backgrounds everywhere** — Looks like crypto scam
- [ ] **Don't use 3D illustrations** — No isometric people, no floating elements
- [ ] **Don't add a chat widget** — Use community chat links instead
- [ ] **Don't create "feature comparison tables"** — Not competing with products
- [ ] **Don't add exit-intent popups** — Just don't
- [ ] **Don't use hamburger menu on desktop** — Use a nav bar
- [ ] **Don't make modal popups** — Especially not for "subscribe to newsletter"

### Content Mistakes

- [ ] **Don't use marketing buzzwords** — "Revolutionary", "cutting-edge", "synergy"
- [ ] **Don't say "high-performance platform"** — Say what it actually does
- [ ] **Don't write "join the movement"** — Sounds cult-ish
- [ ] **Don't hype** — Just facts, no exaggeration
- [ ] **Don't claim unproven security** — If not audited, say "not yet audited"
- [ ] **Don't promise anonymity** — GhostWire provides privacy, not full anonymity

### Technical Mistakes

- [ ] **Don't use WordPress** — Too heavy, security concerns
- [ ] **Don't use Wix/Squarespace** — No control, poor performance
- [ ] **Don't add Google Analytics** — Use Plausible or none
- [ ] **Don't load 50 fonts** — Use 1-2 fonts max
- [ ] **Don't inline all CSS** — Use proper CSS files
- [ ] **Don't block rendering** — Load critical CSS first

---

## 📋 QUALITY CHECKLIST

### Before Submission, Verify ALL:

#### Content

- [ ] Homepage has clear mission statement (first 5 seconds)
- [ ] Download button visible without scrolling
- [ ] Network stats shown (even if placeholder)
- [ ] Use cases section (journalists, activists, etc.)
- [ ] Security page exists with cryptography details
- [ ] About page links to manifesto
- [ ] Community page has GitHub + Matrix links
- [ ] No pricing information anywhere
- [ ] No "Get Started" or "Learn More" vague CTAs
- [ ] All copy uses provided text from `03_CONTENT_PAGES/`

#### Design

- [ ] Looks like torproject.org / signal.org (not Stripe.com)
- [ ] Color palette matches spec (black, white, purple accent)
- [ ] Typography is Inter (or similar clean sans-serif)
- [ ] No stock photos
- [ ] Real GhostWire screenshots used
- [ ] Mobile responsive (test on phone)
- [ ] Accessible (run axe DevTools, fix all errors)
- [ ] Reduced motion respected
- [ ] Works with JavaScript disabled (progressive enhancement)

#### Performance

- [ ] Lighthouse Performance > 90
- [ ] First Contentful Paint < 1.5s
- [ ] Time to Interactive < 3s
- [ ] Total bundle size < 500KB
- [ ] Images optimized (WebP, AVIF)
- [ ] No unused JavaScript
- [ ] No render-blocking resources
- [ ] Gzip/brotli compression enabled

#### Technical

- [ ] Built with Astro (or approved alternative)
- [ ] GitHub API integration works (shows real data)
- [ ] All links work (no 404s)
- [ ] No console errors
- [ ] No tracking scripts (or privacy-respecting only)
- [ ] No cookies (or essential only)
- [ ] HTTPS enforced
- [ ] Builds without errors
- [ ] Deploys successfully

#### Accessibility

- [ ] All images have alt text
- [ ] All interactive elements keyboard-accessible
- [ ] Focus states visible
- [ ] Color contrast > 4.5:1 (text), > 3:1 (UI)
- [ ] Screen reader tested (NVDA or VoiceOver)
- [ ] Skip-to-content link present
- [ ] Proper heading hierarchy (H1 → H2 → H3)
- [ ] Form labels present (if any forms)
- [ ] Error messages clear and actionable

---

## 🧪 TESTING CHECKLIST

### Test On Devices

- [ ] Desktop (1920x1080)
- [ ] Laptop (1366x768)
- [ ] iPhone or Android phone
- [ ] iPad or Android tablet
- [ ] Old/slow device (if available)

### Test On Browsers

- [ ] Firefox (latest)
- [ ] Chrome (latest)
- [ ] Safari (latest)
- [ ] Brave (latest)
- [ ] Tor Browser (if applicable)

### Test On Connections

- [ ] Fast WiFi
- [ ] 4G mobile
- [ ] 3G mobile (throttle in DevTools)
- [ ] Offline (what works?)

### Test With Tools

- [ ] Lighthouse (Chrome DevTools)
- [ ] axe DevTools (accessibility)
- [ ] WAVE (web accessibility)
- [ ] WebPageTest (performance)
- [ ] GTmetrix (performance)

---

## 📊 ACCEPTANCE CRITERIA

### Must Have (Required)

1. Homepage with mission + downloads + stats
2. Download page with GitHub releases integration
3. About page with manifesto link
4. Security page with cryptography details
5. Community page with GitHub + Matrix links
6. Mobile responsive
7. Accessible (WCAG 2.1 AA)
8. Performance (Lighthouse > 90)
9. No critical bugs
10. Deploys successfully

### Should Have (Important)

1. Blog section (Markdown-based)
2. Documentation section (or link to external docs)
3. Network stats (live or static)
4. Donate page (OpenCollective integration)
5. Press kit (logos, screenshots)
6. Multiple language support structure

### Nice to Have (Optional)

1. Live network map
2. Contributor showcase
3. Release notes auto-sync from GitHub
4. Status page
5. Community forum integration

---

## 🚀 SUBMISSION GUIDE

### What to Submit

1. **Source code** — Full repository
2. **Build output** — `dist/` folder
3. **Deployment instructions** — How to deploy
4. **Environment variables** — What's needed (GitHub token, etc.)
5. **Known issues** — What's not done yet
6. **Future improvements** — What could be better

### How to Submit

```
Submit as:
1. GitHub repository (preferred)
   OR
2. ZIP file with full source + build

Include:
- README.md (setup instructions)
- .env.example (required env vars)
- DEPLOY.md (deployment guide)
- KNOWN_ISSUES.md (what's broken)
```

### Review Process

1. **First pass** — Does it meet all "Must Have" criteria?
2. **Second pass** — Test on mobile, run Lighthouse
3. **Third pass** — Accessibility audit (axe DevTools)
4. **Fourth pass** — Content review (copy, tone, accuracy)
5. **Final** — Deploy to staging, test live

### Feedback Loop

```
Submit → Review → Feedback → Revise → Resubmit → Approve
   │         │          │          │          │
   │         │          │          │          └─→ Deploy to production
   │         │          │          └─→ Fix issues
   │         │          └─→ List what needs to change
   │         └─→ Check against checklist
   └─→ AI agent submits completed work
```

---

## 🆘 IF YOU GET STUCK

### Common Questions

**Q: Should I use React or Astro?**  
A: Astro. It's faster, simpler, and better for content sites.

**Q: Should I add a blog?**  
A: Yes, but keep it simple. Markdown files, no CMS needed.

**Q: Should I add user accounts?**  
A: No. GhostWire doesn't have user accounts.

**Q: Should I add analytics?**  
A: Only if privacy-respecting (Plausible, Fathom). No Google Analytics.

**Q: Should I add a newsletter signup?**  
A: No. Don't collect emails unless absolutely necessary.

**Q: What if I can't get GitHub API working?**  
A: Use static data for now. Add a TODO comment.

**Q: What if the design doesn't match reference sites?**  
A: That's okay. Focus on function over form. Content over style.

**Q: What if I'm not sure about something?**  
A: Ask. Or make a reasonable assumption and document it.

### Contact

If stuck, contact:
- **NGAIRURU** — ngairuru@proton.me
- **GitHub** — github.com/phantomojo/GhostWire-secure-mesh-communication

---

## 🎯 SUCCESS METRICS

Your website is successful if:

1. **Amina the Journalist** can download and set up GhostWire in < 10 minutes
2. **Lighthouse score** > 90 on all metrics
3. **Accessibility** — Zero critical axe errors
4. **Performance** — < 1.5s load time on 4G
5. **Community** — People start contributing via GitHub links
6. **Trust** — Security researchers take it seriously
7. **Mission** — Visitors understand what GhostWire is in < 5 seconds

---

**Remember:** You're not building a marketing site. You're building **infrastructure for human dignity**. Design accordingly.
