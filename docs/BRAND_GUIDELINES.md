# 🎨 GhostWire Brand Guidelines

**Version:** 1.0  
**Date:** April 1, 2026  
**Status:** Official Brand Identity

---

## 📋 **Overview**

This document defines the visual identity and brand guidelines for GhostWire. Use these assets consistently across all documentation, presentations, and communications.

---

## 🎨 **Logo**

### **Primary Logo**

![GhostWire Logo](../webui/public/ghostwire-logo.png)

**File:** `webui/public/ghostwire-logo.png`  
**Format:** PNG (transparent background available in SVG)  
**Usage:** GitHub profile, website, presentations, social media

### **Logo Usage**

✅ **Do:**
- Use the official logo from `webui/public/ghostwire-logo.png`
- Maintain clear space around the logo (minimum ½ logo height)
- Use on appropriate backgrounds (mint green or white)

❌ **Don't:**
- Stretch or distort the logo
- Change the colors
- Place on busy backgrounds
- Use low-resolution versions

---

## 🎨 **Color Palette**

### **Primary Colors**

| Color | Hex | RGB | Usage |
|-------|-----|-----|-------|
| **GhostWire Green** | `#00DC9A` | rgb(0, 220, 154) | Primary brand color |
| **Deep Space Blue** | `#0F172A` | rgb(15, 23, 42) | Text, headers |
| **Pure White** | `#FFFFFF` | rgb(255, 255, 255) | Backgrounds |

### **Accent Colors**

| Color | Hex | RGB | Usage |
|-------|-----|-----|-------|
| **Sunrise Orange** | `#F97316` | rgb(249, 115, 22) | Call-to-action, highlights |
| **Trust Blue** | `#3B82F6` | rgb(59, 130, 246) | Links, SDG alignment |
| **Safeguard Yellow** | `#FBBF24` | rgb(251, 191, 36) | Warnings, notes |

### **CSS Variables**

```css
:root {
  --ghostwire-green: #00DC9A;
  --deep-space-blue: #0F172A;
  --pure-white: #FFFFFF;
  --sunrise-orange: #F97316;
  --trust-blue: #3B82F6;
  --safeguard-yellow: #FBBF24;
}
```

---

## 📝 **Typography**

### **Primary Font**

**Segoe UI** (Windows)  
**San Francisco** (macOS)  
**Arial** (fallback)

### **Font Sizes**

| Usage | Size | Weight |
|-------|------|--------|
| Headings | 48-72px | Bold (700) |
| Subheadings | 24-36px | Semi-Bold (600) |
| Body Text | 16-20px | Regular (400) |
| Small Text | 12-14px | Regular (400) |

---

## 🖼️ **Visual Assets**

### **Location**

All visual assets are stored in:
- `webui/public/` - Web and UI assets
- `docs/assets/` - Documentation assets

### **Available Assets**

| Asset | File | Usage |
|-------|------|-------|
| Logo (PNG) | `webui/public/ghostwire-logo.png` | General use |
| Logo (SVG) | `docs/assets/ghostwire_logo_wordmark.svg` | Scalable graphics |

---

## 📋 **Brand Voice**

### **Tone**

- ✅ **Professional but approachable**
- ✅ **Technical but accessible**
- ✅ **Confident but humble**
- ✅ **African innovation perspective**

### **Language Guidelines**

| Use | Instead of |
|-----|------------|
| "Connectivity resilience" | "Censorship resistance" |
| "Communities with limited connectivity" | "Activists in censored regions" |
| "Privacy-enhancing" | "Stealth" |
| "Real-time monitoring" | "Tactical" |
| "Industry-standard encryption" | "Military-grade encryption" |

---

## 📧 **Email Signature**

```
─────────────────────────────────────────
[Your Name]
[Your Role], Team GhostWire
Open University of Kenya

📧 [your email]
🌐 ghostwire.cc
🔗 github.com/Phantomojo/GhostWire-secure-mesh-communication

[GhostWire Logo]
─────────────────────────────────────────
```

---

## 🖥️ **Website/UI Usage**

### **Favicon**

Use `webui/public/ghostwire-logo.png` as favicon.

### **Header Logo**

```html
<img src="/ghostwire-logo.png" alt="GhostWire Logo" height="50" />
```

---

## 📊 **Presentation Template**

### **Title Slide**

```
┌────────────────────────────────────────┐
│                                        │
│    [GhostWire Logo]                    │
│                                        │
│    GhostWire                           │
│    [Subtitle]                          │
│                                        │
│    ───────────────────────────         │
│                                        │
│    Team GhostWire                      │
│    Open University of Kenya            │
│                                        │
└────────────────────────────────────────┘
```

**Template File:** `./presentation_title_slide.html`

---

## 🎬 **Video Assets**

### **Intro Frame**

Use `video_intro_frame.html` for video submissions.

**Duration:** 5 seconds  
**Format:** 1920×1080px (Full HD)  
**Animation:** Fade in logo → text → tagline

---

## 🖨️ **Print Materials**

### **Exhibition Poster**

**Size:** A1 (594 × 841 mm)  
**Resolution:** 300 DPI minimum  
**Format:** PDF (print-ready)

**Template:** `./exhibition_poster_A1.html`

---

## ✅ **Brand Checklist**

Before publishing any materials:

- [ ] Logo is from official source (`webui/public/ghostwire-logo.png`)
- [ ] Colors match brand palette
- [ ] Typography follows guidelines
- [ ] Language is professional and accessible
- [ ] Team name is "Team GhostWire"
- [ ] Institution is "Open University of Kenya"

---

## 📞 **Questions?**

Refer to:
- `VISUAL_ASSETS_COMPLETE.md` - Complete asset guide

---

<div align="center">

**GhostWire — Connecting the disconnected**

*Built for connection. Designed for resilience.*

**Team GhostWire | Open University of Kenya**

</div>
