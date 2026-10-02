// GhostWire mermaid theme — DARK variant
// Mirrors BRAND_GUIDELINES.md: Deep Space Blue #0F172A ground, GhostWire Green
// #00DC9A accent, Sunrise Orange / Trust Blue / Safeguard Yellow on top.
//
// The light theme in mermaid-ghostwire.mjs renders white-background PNGs, which
// glare on a dark page. These colours keep the same hierarchy while sitting
// correctly on #0F172A.

export const ghostwireDarkTheme = `%%{init: {
  'theme': 'base',
  'themeVariables': {
    'background': '#0F172A',
    'primaryColor': '#14342C',
    'primaryTextColor': '#E2E8F0',
    'primaryBorderColor': '#00DC9A',
    'secondaryColor': '#142B45',
    'secondaryTextColor': '#E2E8F0',
    'secondaryBorderColor': '#3B82F6',
    'tertiaryColor': '#3A2416',
    'tertiaryTextColor': '#E2E8F0',
    'tertiaryBorderColor': '#F97316',
    'lineColor': '#94A3B8',
    'textColor': '#E2E8F0',
    'mainBkg': '#14342C',
    'nodeBorder': '#00DC9A',
    'clusterBkg': '#111C33',
    'clusterBorder': '#334155',
    'titleColor': '#E2E8F0',
    'edgeLabelBackground': '#0F172A',
    'fontFamily': 'DejaVu Sans, Helvetica, Arial, sans-serif',
    'fontSize': '15px'
  },
  'flowchart': { 'curve': 'basis', 'padding': 18, 'nodeSpacing': 45, 'rankSpacing': 55, 'useMaxWidth': true },
  'sequence': { 'useMaxWidth': true, 'actorFontSize': 14, 'noteFontSize': 13, 'boxMargin': 10 },
  'themeCSS': '.label { font-family: "DejaVu Sans", sans-serif; fill: #E2E8F0; } .node rect, .node polygon { rx: 6; ry: 6; } .edgeLabel { fill: #E2E8F0; }'
}}%%`;

// Mermaid only honours the init directive if it is the first line of the source.
export function withTheme(src) {
  const body = src.replace(/^\s*```mermaid\s*/i, '').replace(/```\s*$/, '');
  const directive = DARK ? ghostwireDarkTheme : '';
  if (body.trimStart().startsWith('%%{')) {
    // Source already carries a directive: strip it, ours must come first.
    return directive + '\n' + body.replace(/^\s*%%\{[\s\S]*?\}%%\s*/, '');
  }
  return directive + '\n' + body;
}

export const DARK = process.env.GW_DARK === '1';
