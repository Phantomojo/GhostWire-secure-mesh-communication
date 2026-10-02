#!/usr/bin/env node
/**
 * Render every ```mermaid block in the Bible chapters to SVG, using the
 * GhostWire brand theme, then rewrite the Markdown to reference the SVG.
 *
 * pandoc does not render mermaid, so without this step every diagram reaches
 * the PDF as raw syntax. This runs before pandoc and points each block at a
 * real image.
 *
 * Usage: node render_diagrams.mjs [--force]
 */
import { readFile, writeFile, mkdir, readdir, unlink } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import { withTheme } from './mermaid-ghostwire-dark.mjs';

// GW_DARK=1 renders the dark variant into diagrams-dark/ and rewrites the
// Markdown image paths to match. Light mode is the default and unchanged.
const DARK = process.env.GW_DARK === '1';

const execFileAsync = promisify(execFile);
const HERE = path.dirname(fileURLToPath(import.meta.url));
const OUTDIR = path.join(HERE, DARK ? 'diagrams-dark' : 'diagrams');
const MMC = '/home/phantomojo/.hermes/tools/mermaid/node_modules/.bin/mmdc';

const PUPPETEER_CONFIG = path.join(HERE, 'puppeteer-config.json');

const force = process.argv.includes('--force') || DARK;
const only = (() => {
  const i = process.argv.indexOf('--only');
  return i > -1 ? process.argv[i + 1] : null;
})();

async function main() {
  if (!existsSync(MMC)) {
    console.error(`✗ mermaid-cli not found at ${MMC}`);
    process.exit(1);
  }
  await mkdir(OUTDIR, { recursive: true });

  const files = (await readdir(HERE))
    .filter((f) => /^\d\d[a-z]?_.*\.md$/.test(f))
    .filter((f) => !only || f.startsWith(only))
    .sort();

  let rendered = 0;
  let reused = 0;
  let failed = 0;
  const failures = [];

  for (const file of files) {
    const full = path.join(HERE, file);
    let text = await readFile(full, 'utf8');
    const blocks = [...text.matchAll(/```mermaid\s*\n([\s\S]*?)```/g)];

    if (blocks.length === 0) continue;
    console.log(`\n${file} — ${blocks.length} diagram(s)`);

    let index = 0;
    let changed = false;
    for (const block of blocks) {
      index += 1;
      const stem = `${path.basename(file, '.md')}--${index}`;
      const svg = path.join(OUTDIR, `${stem}.svg`);
      const png = path.join(OUTDIR, `${stem}.png`);

      if (!force && existsSync(svg) && existsSync(png)) {
        reused += 1;
      } else {
        const themed = withTheme(block[1]);
        const tmp = path.join(OUTDIR, `${stem}.mmd`);
        await writeFile(tmp, themed, 'utf8');
        try {
          await execFileAsync(MMC, [
            '-i', tmp,
            '-o', svg,
            '-b', 'transparent',
            '-p', PUPPETEER_CONFIG,
          ], { timeout: 120000, maxBuffer: 32 * 1024 * 1024 });

          // A PNG fallback keeps the diagram visible if a downstream tool
          // cannot place the SVG, and survives PDF viewers that block SVG.
          // Background must match the page, or each diagram shows as a
          // bright block on a dark ground (and glares).
          await execFileAsync(MMC, [
            '-i', tmp,
            '-o', png,
            '-b', DARK ? '#0F172A' : 'white',
            '-s', '2',
            '-p', PUPPETEER_CONFIG,
          ], { timeout: 120000, maxBuffer: 32 * 1024 * 1024 });

          // Keep the .mmd source: a second theme needs it to re-render, and
          // the Markdown no longer contains the mermaid block once converted.
          rendered += 1;
          console.log(`  ✓ ${stem}`);
        } catch (err) {
          failed += 1;
          failures.push({ file, index, error: String(err.message).slice(0, 300) });
          console.log(`  ✗ ${stem} — ${String(err.message).split('\n')[0].slice(0, 120)}`);
          continue;
        }
      }

      // Swap the mermaid fence for an image reference pandoc understands.
      const rel = path.relative(path.dirname(full), png);
      // No "Diagram N:" prefix — pandoc numbers figures itself via the caption,
      // and prefixing here produces "Diagram 1: Diagram 1: ..." in the PDF.
      const label = file.replace(/^\d\d_/, '').replace(/\.md$/, '').replace(/_/g, ' ');
      const replacement = `![${label}](${rel})`;
      text = text.replace(block[0], replacement);
      changed = true;
    }

    if (changed) {
      await writeFile(full, text, 'utf8');
      console.log(`  → ${file} updated`);
    }
  }

  console.log(`\nrendered ${rendered} · reused ${reused} · failed ${failed}`);
  if (failures.length) {
    console.log('\nFailures (source left as a code block):');
    for (const f of failures) console.log(`  ${f.file} #${f.index}: ${f.error}`);
    process.exitCode = 1;
  }
}

main().catch((e) => {
  console.error(e);
  process.exit(1);
});
