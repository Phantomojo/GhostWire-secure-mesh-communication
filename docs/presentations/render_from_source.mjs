#!/usr/bin/env node
/**
 * Render every diagram from its cached .mmd source into a theme-specific
 * directory. Needed because the Markdown holds image references (not mermaid
 * blocks) once a diagram has been rendered, so switching themes requires
 * re-rendering from source.
 *
 *   node render_from_source.mjs           # light -> diagrams/
 *   GW_DARK=1 node render_from_source.mjs  # dark  -> diagrams-dark/
 */
import { readdir, mkdir, rm } from 'node:fs/promises';
import { existsSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';
import { execFile } from 'node:child_process';
import { promisify } from 'node:util';
import { readFile, writeFile } from 'node:fs/promises';
import { withTheme, DARK } from './mermaid-ghostwire-dark.mjs';

const execFileAsync = promisify(execFile);
const HERE = path.dirname(fileURLToPath(import.meta.url));
const SRC = path.join(HERE, 'diagrams');
const OUT = path.join(HERE, DARK ? 'diagrams-dark' : 'diagrams');
const MMC = '/home/phantomojo/.hermes/tools/mermaid/node_modules/.bin/mmdc';
const PUP = path.join(HERE, 'puppeteer-config.json');
const BG = DARK ? '#0F172A' : 'white';

if (!existsSync(SRC)) {
  console.error(`✗ no diagram sources at ${SRC}`);
  process.exit(1);
}
await mkdir(OUT, { recursive: true });

const files = (await readdir(SRC)).filter((f) => f.endsWith('.mmd')).sort();
console.log(`→ ${files.length} diagram source(s) → ${path.basename(OUT)}`);

let ok = 0;
const failed = [];

for (const f of files) {
  const stem = f.replace(/\.mmd$/, '');
  const raw = await readFile(path.join(SRC, f), 'utf8');
  const tmp = path.join(OUT, `${stem}.mmd`);
  await writeFile(tmp, withTheme(raw), 'utf8');

  try {
    await execFileAsync(MMC, ['-i', tmp, '-o', path.join(OUT, `${stem}.svg`),
      '-b', 'transparent', '-p', PUP],
      { timeout: 120000, maxBuffer: 32 * 1024 * 1024 });
    await execFileAsync(MMC, ['-i', tmp, '-o', path.join(OUT, `${stem}.png`),
      '-b', BG, '-s', '2', '-p', PUP],
      { timeout: 120000, maxBuffer: 32 * 1024 * 1024 });
    await rm(tmp, { force: true });
    ok += 1;
    process.stdout.write('.');
  } catch (err) {
    failed.push({ stem, msg: String(err.message).split('\n')[0].slice(0, 160) });
    process.stdout.write('x');
  }
}

console.log(`\nrendered ${ok} · failed ${failed.length}`);
for (const f of failed) console.log(`  ${f.stem}: ${f.msg}`);
if (failed.length) process.exitCode = 1;
