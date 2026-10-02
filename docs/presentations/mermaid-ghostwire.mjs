// GhostWire mermaid theme — mirrors docs/BRAND_GUIDELINES.md
// GhostWire Green #00DC9A on Deep Space Blue #0F172A
// Accents: Sunrise Orange #F97316, Trust Blue #3B82F6, Safeguard Yellow #FBBF24

export const ghostwireTheme = `%%{init: {
  'theme': 'base',
  'themeVariables': {
    'background': '#FFFFFF',
    'primaryColor': '#ECFDF5',
    'primaryTextColor': '#0F172A',
    'primaryBorderColor': '#00DC9A',
    'secondaryColor': '#EFF6FF',
    'secondaryTextColor': '#0F172A',
    'secondaryBorderColor': '#3B82F6',
    'tertiaryColor': '#FFF7ED',
    'tertiaryTextColor': '#0F172A',
    'tertiaryBorderColor': '#F97316',
    'lineColor': '#64748B',
    'textColor': '#0F172A',
    'mainBkg': '#ECFDF5',
    'nodeBorder': '#00DC9A',
    'clusterBkg': '#F8FAFC',
    'clusterBorder': '#CBD5E1',
    'titleColor': '#0F172A',
    'edgeLabelBackground': '#FFFFFF',
    'fontFamily': 'DejaVu Sans, Helvetica, Arial, sans-serif',
    'fontSize': '15px'
  },
  'flowchart': { 'curve': 'basis', 'padding': 18, 'nodeSpacing': 45, 'rankSpacing': 55, 'useMaxWidth': true },
  'sequence': { 'useMaxWidth': true, 'actorFontSize': 14, 'noteFontSize': 13, 'boxMargin': 10 },
  'themeCSS': '.label { font-family: "DejaVu Sans", sans-serif; } .node rect, .node polygon { rx: 6; ry: 6; }'
}}%%`;

// Mermaid renders the init directive only if it is the first line of the source.
// This helper guarantees that regardless of how the block was indented.
export function withTheme(diagramSource) {
  const src = diagramSource.replace(/^\s*```mermaid\s*/i, '').replace(/```\s*$/, '');
  if (src.trimStart().startsWith('%%{')) return src;
  return `${ghostwireTheme}\n${src}`;
}
