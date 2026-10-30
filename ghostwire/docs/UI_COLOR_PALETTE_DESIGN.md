# UI Color Palette — Design Decision

## Problem

GhostWire's original palette (phosphor green #00dc9a on near-black #070a0e) is
one of the three default AI-generated looks identified by the Claude
frontend-design skill. It reads as "templated" rather than distinctive.

## Research Sources

- Claude frontend-design skill (anthropics/claude-code): identifies 3 default AI looks
- empire-ui.com: 20 dark-mode palettes for 2026
- colourpilot.com: UI color trends, saturation guidance
- vev.design: 6 dark mode palette archetypes
- skyryedesign.com: semantic color tokens, dark mode contrast ratios

## Key Findings

1. **Avoid default AI looks**: warm cream+terracotta, near-black+acid-green, broadsheet
2. **Dominant colors with sharp accents** outperform timid evenly-distributed palettes
3. **Saturation < 80%** by default (neon becomes aggressive on dark backgrounds)
4. **Avoid pure black** (#000) — use deep slate/navy (#0c1220, #0f172a)
5. **Draw from cultural aesthetics**: IDE themes, maritime, aviation, etc.
6. **Template chrome to avoid**: ALL-CAPS eyebrows, middle-dot meta strings, monospace data labels, "→" on buttons

## Palette Options

### A — Deep Slate + Amber Signal (IMPLEMENTED)
- Base: #0c1220, Surface: #141e30, Raised: #1a2744, Panel: #1e2d4a
- Accent: #f59e0b (amber — naval signal lamp, warmth, distinctiveness)
- Secondary: #14b8a6 (teal — trust/verification)
- Text: #e2e8f0 / rgba(226,232,240,0.75) / 0.45 / 0.2
- Aesthetic: maritime mesh, naval signal lamps, warm amber on dark slate

### B — Zinc + Electric Teal (NOT YET BUILT)
- Base: #09090b, Surface: #18181b, Accent: #14b8a6 (teal)
- Aesthetic: cybersecurity, trust, clean security tools

### C — Obsidian + Warm Copper (NOT YET BUILT)
- Base: #0c0a09, Surface: #1c1917, Accent: #ea580c (copper)
- Aesthetic: resilience, durability, earth warmth

### D — Charcoal + Violet Signal (NOT YET BUILT)
- Base: #0a0a0f, Surface: #16161f, Accent: #8b5cf6 (violet)
- Aesthetic: privacy, shadow, anonymity

## SVG Icon Libraries (for future integration)

| Library | Icons | License | React package |
|---|---|---|---|
| Lucide | 1,900+ | ISC | lucide-react |
| Tabler | 6,200+ | MIT | @tabler/icons-react |
| Heroicons | 1,288 | MIT | @heroicons/react |
| Phosphor | 9,000+ | MIT | phosphor-react |
| Feather | 286 | MIT | react-feather |

Currently no icon library installed — using unicode glyphs (⟁, ⬢, ⌕).

## Decision

Implemented Option A (Deep Slate + Amber Signal). Offers 4 cultural-aesthetic
options. User can switch by changing :root CSS variables. Consider adding a
palette switcher in Settings panel for runtime selection.
EOF
echo "created"