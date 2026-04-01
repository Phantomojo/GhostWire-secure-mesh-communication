# GHOSTWIRE WEBSITE - COMPLETE CONTENT PACK
## Upload This to Kimi AI

**Version:** 1.0  
**Date:** March 2, 2026  
**For:** Kimi AI Agent (or any AI building the GhostWire website)

---

## 📦 WHAT THIS IS

This is a **complete content and instruction pack** for building the official GhostWire website.

Upload this entire folder to Kimi AI along with the brief (`GHOSTWIRE_WEBSITE_AI_BRIEF.md`).

---

## 📁 FOLDER STRUCTURE

```
website-content-pack/
│
├── README.md ← You are here (start here)
│
├── 01_CORE_CONTEXT/
│   ├── what_is_ghostwire.md
│   ├── manifesto_summary.md
│   ├── founder_story.md
│   └── use_cases.md
│
├── 02_DESIGN_INSPIRATION/
│   ├── reference_sites.md
│   ├── visual_mood.md
│   └── color_typography.md
│
├── 03_CONTENT_PAGES/
│   ├── homepage_copy.md
│   ├── about_page.md
│   ├── download_page.md
│   ├── security_page.md
│   └── community_page.md
│
├── 04_TECHNICAL_DOCS/
│   ├── architecture_summary.md
│   ├── cryptography_details.md
│   ├── api_overview.md
│   └── rf_mesh_explained.md
│
├── 05_BRAND_ASSETS/
│   ├── logo_usage.md
│   ├── screenshot_guide.md
│   └── image_style.md
│
├── 06_USER_PERSONAS/
│   └── user_personas.md (created)
│
├── 07_WIREFRAMES/
│   ├── homepage_layout.txt
│   ├── download_layout.txt
│   └── mobile_layout.txt
│
└── 08_AGI_INSTRUCTIONS/
    ├── do_dont_checklist.md (created)
    └── quality_checklist.md
```

---

## 🚀 HOW TO USE THIS PACK

### Step 1: Upload to Kimi AI

1. **Zip this folder** → `ghostwire-website-content-pack.zip`
2. **Upload to Kimi** along with `GHOSTWIRE_WEBSITE_AI_BRIEF.md`
3. **Say:** "Build the GhostWire website using this content pack and the brief."

### Step 2: Provide Additional Context

Tell Kimi:

```
I need you to build the official GhostWire website.

I've uploaded two files:
1. GHOSTWIRE_WEBSITE_AI_BRIEF.md - Complete instructions
2. website-content-pack.zip - All content, copy, and context

Please:
1. Read the brief first (especially Part 1: What GhostWire Is)
2. Review the content pack for exact copy and design guidance
3. Build using Astro + Tailwind CSS
4. Reference sites: torproject.org, signal.org, bitcoin.org

DO NOT build a SaaS landing page. This is infrastructure software.

Key requirements:
- Homepage with mission + downloads + network stats
- Download page with GitHub releases integration
- About page with manifesto
- Security page with cryptography details
- Community page with GitHub + Matrix links

Deadline: [Your deadline]
Questions? Ask me before proceeding.
```

### Step 3: Review Output

When Kimi delivers the website:

1. **Check against checklist** → `08_AGI_INSTRUCTIONS/do_dont_checklist.md`
2. **Test performance** → Run Lighthouse, should be > 90
3. **Test accessibility** → Run axe DevTools, zero critical errors
4. **Test on mobile** → Open on your phone
5. **Verify content** → Does it use the provided copy?

### Step 4: Deploy

1. **Deploy to staging** → Netlify, Vercel, or Cloudflare Pages
2. **Test live** → Share link with community for feedback
3. **Fix issues** → Iterate based on feedback
4. **Deploy to production** → Connect to ghostwire.net domain

---

## 📄 DOCUMENT INDEX

### Essential Reading (For AI Agent)

| Document | Purpose | Priority |
|----------|---------|----------|
| `01_CORE_CONTEXT/what_is_ghostwire.md` | What GhostWire is | CRITICAL |
| `06_USER_PERSONAS/user_personas.md` | Who uses it | CRITICAL |
| `03_CONTENT_PAGES/homepage_copy.md` | Exact copy to use | CRITICAL |
| `08_AGI_INSTRUCTIONS/do_dont_checklist.md` | What to avoid | CRITICAL |
| `02_DESIGN_INSPIRATION/reference_sites.md` | Visual targets | HIGH |

### Reference Documents

| Document | Purpose | Priority |
|----------|---------|----------|
| `01_CORE_CONTEXT/manifesto_summary.md` | Philosophy | HIGH |
| `03_CONTENT_PAGES/download_page.md` | Download page copy | HIGH |
| `04_TECHNICAL_DOCS/architecture_summary.md` | Technical details | MEDIUM |
| `04_TECHNICAL_DOCS/cryptography_details.md` | Security info | MEDIUM |
| `05_BRAND_ASSETS/screenshot_guide.md` | Visual assets | MEDIUM |

### Optional Documents

| Document | Purpose | Priority |
|----------|---------|----------|
| `01_CORE_CONTEXT/founder_story.md` | About founder | LOW |
| `01_CORE_CONTEXT/use_cases.md` | Use case details | LOW |
| `02_DESIGN_INSPIRATION/visual_mood.md` | Design details | LOW |
| `05_BRAND_ASSETS/logo_usage.md` | Logo guidelines | LOW |
| `07_WIREFRAMES/homepage_layout.txt` | Layout guide | LOW |

---

## ✅ CHECKLIST FOR AI AGENT

Before starting, confirm you have read:

- [ ] `GHOSTWIRE_WEBSITE_AI_BRIEF.md` (main brief)
- [ ] `01_CORE_CONTEXT/what_is_ghostwire.md` (what GhostWire is)
- [ ] `06_USER_PERSONAS/user_personas.md` (who uses it)
- [ ] `02_DESIGN_INSPIRATION/reference_sites.md` (visual targets)
- [ ] `08_AGI_INSTRUCTIONS/do_dont_checklist.md` (what to avoid)

Before submitting, verify:

- [ ] Homepage uses provided copy from `03_CONTENT_PAGES/homepage_copy.md`
- [ ] Download button is visible without scrolling
- [ ] No pricing page exists
- [ ] No "Get Started" vague CTAs
- [ ] Security page has cryptography details
- [ ] About page links to manifesto
- [ ] Mobile responsive
- [ ] Lighthouse score > 90
- [ ] Accessibility passes (axe DevTools)
- [ ] Bundle size < 500KB

---

## 🎯 SUCCESS CRITERIA

Your website is ready when:

1. **Amina the Journalist** (see `user_personas.md`) can:
   - Understand what GhostWire is in < 5 seconds
   - Find download button in < 3 seconds
   - Install and run in < 10 minutes

2. **Performance:**
   - Lighthouse Performance > 90
   - First Contentful Paint < 1.5s
   - Total bundle < 500KB

3. **Accessibility:**
   - WCAG 2.1 AA compliant
   - Zero critical axe errors
   - Works with screen reader

4. **Content:**
   - Uses provided copy (no marketing fluff)
   - Links to manifesto
   - Security details accurate
   - GitHub integration working

5. **Design:**
   - Looks like torproject.org / signal.org
   - Not like a SaaS landing page
   - Mobile responsive
   - Real screenshots (not stock photos)

---

## 📞 CONTACT

For questions or clarifications:

**NGAIRURU**  
- Email: ngairuru@proton.me  
- GitHub: github.com/phantomojo  
- Location: Nairobi, Kenya

---

## 🔄 VERSIONS

| Version | Date | Changes |
|---------|------|---------|
| 1.0 | March 2, 2026 | Initial release |

---

## 📌 QUICK START FOR KIMI AI

```
1. Read GHOSTWIRE_WEBSITE_AI_BRIEF.md first
2. Read 01_CORE_CONTEXT/what_is_ghostwire.md
3. Read 06_USER_PERSONAS/user_personas.md
4. Review 02_DESIGN_INSPIRATION/reference_sites.md
5. Use copy from 03_CONTENT_PAGES/homepage_copy.md
6. Follow 08_AGI_INSTRUCTIONS/do_dont_checklist.md
7. Build with Astro + Tailwind CSS
8. Test against quality checklist
9. Submit for review
```

**Good luck. Build something that matters.**

---

*This pack was created to prevent the mistakes identified in `KIMI_AGENT_WEBSITE_AUDIT.md`. Learn from those errors.*
