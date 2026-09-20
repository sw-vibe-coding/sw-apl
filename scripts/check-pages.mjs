// The browser demo, checked in a browser.
//
// The page, the worker and the WebAssembly are one thing served as
// three files, and a browser caches them separately. What this
// checks is what that costs: a visitor holding an older copy of one
// of them must still get a session, or must be told plainly why not
// -- never a prompt that silently ignores typing.
//
// It is not part of `just test`: it needs a browser, and a checkout
// without one should still be able to run the suite. `just
// check-pages` runs it, and says what to install if it cannot.
//
//   npm install playwright-core      (uses the Chrome already here)
//
// Usage: node scripts/check-pages.mjs [pages-dir]

import { createServer } from 'node:http';
import { readFile } from 'node:fs/promises';
import { extname, join, normalize } from 'node:path';
import process from 'node:process';

const ROOT = process.argv[2] ?? 'pages';

const TYPES = {
  '.json': 'application/json',
  '.html': 'text/html',
  '.js': 'text/javascript',
  '.mjs': 'text/javascript',
  '.wasm': 'application/wasm',
  '.svg': 'image/svg+xml',
  '.txt': 'text/plain',
  '.ts': 'text/plain',
};

// A static host, as plain as the one the demo is published on.
//
// `serving` is what a visitor's browser has rather than what the
// checkout holds: put a path in it and that is what is served, which
// is how an older cached file is imitated. It has to be done here
// and not with the browser's own interception, because a worker's
// script is fetched by the browser and not by the page.
const serving = new Map();

// `isolated` sends the two headers a real static host will not, as
// `just pages-serve` does. The default is without them, because that
// is what a published demo faces and what the service worker is for.
const ISOLATION = {
  'cross-origin-opener-policy': 'same-origin',
  'cross-origin-embedder-policy': 'require-corp',
  'cross-origin-resource-policy': 'cross-origin',
};

function host(dir, isolated = false) {
  const extra = isolated ? ISOLATION : {};
  const server = createServer(async (req, res) => {
    const path = normalize(new URL(req.url, 'http://x').pathname);
    const held = serving.get(path);
    if (held !== undefined) {
      res.writeHead(200, { 'content-type': 'text/javascript', ...extra });
      return res.end(held);
    }
    const file = join(dir, path.endsWith('/') ? `${path}index.html` : path);
    try {
      const body = await readFile(file);
      res.writeHead(200, {
        'content-type': TYPES[extname(file)] ?? 'application/octet-stream',
        ...extra,
      });
      res.end(body);
    } catch {
      res.writeHead(404).end('not found');
    }
  });
  return new Promise((ok) => server.listen(0, '127.0.0.1', () => ok(server)));
}

let chromium;
try {
  ({ chromium } = await import('playwright-core'));
} catch {
  console.log('check-pages: playwright-core is not installed; skipping.');
  console.log('check-pages:   npm install playwright-core');
  process.exit(0);
}

const CHROME = process.env.CHROME_PATH
  ?? '/Applications/Google Chrome.app/Contents/MacOS/Google Chrome';

const server = await host(ROOT);
const url = `http://127.0.0.1:${server.address().port}/`;
const browser = await chromium.launch({ executablePath: CHROME, headless: true });

let failed = 0;
const check = (name, ok, saw) => {
  console.log(`${ok ? 'ok  ' : 'FAIL'} ${name}${ok ? '' : `\n       saw: ${saw}`}`);
  if (!ok) failed += 1;
};

// Wait until the page is either taking typing or has said why not.
async function settle(page, seconds = 20) {
  for (let i = 0; i < seconds * 4; i++) {
    const done = await page.evaluate(() => {
      const line = document.getElementById('line');
      return !line.classList.contains('waiting')
        || document.getElementById('paper').textContent.trim().length > 0;
    });
    if (done) return;
    await page.waitForTimeout(250);
  }
}

async function visit() {
  const page = await (await browser.newContext()).newPage();
  await page.goto(url, { waitUntil: 'load' });
  await page.waitForTimeout(3000); // the isolation worker installs and reloads
  await settle(page);
  return page;
}

async function send(page, text) {
  await page.evaluate((t) => {
    const data = new DataTransfer();
    data.setData('text', t);
    dispatchEvent(new ClipboardEvent('paste', { clipboardData: data, bubbles: true, cancelable: true }));
    dispatchEvent(new KeyboardEvent('keydown', { key: 'Enter', bubbles: true }));
  }, text);
  await page.waitForTimeout(900);
}

const paper = (page) => page.evaluate(() => document.getElementById('paper').textContent);
const typing = (page) => page.evaluate(() =>
  !document.getElementById('line').classList.contains('waiting'));

// 1. A first visit: the session starts and the shipped library is there.
{
  const page = await visit();
  check('a first visit takes typing', await typing(page), 'the prompt never came');
  await send(page, ')LIB 1');
  const shown = await paper(page);
  check('library 1 lists what sw-apl ships',
    ['EDIT', 'LIFE', 'RACE'].every((n) => shown.includes(n)), JSON.stringify(shown.slice(-120)));
  await page.context().close();
}

// 2. A returning visit holding the worker from an older bundle. This
//    is the bug: start() gained an argument, the cached worker does
//    not pass it, and the page showed a prompt that did nothing.
{
  const stale = `import init, { start } from "./wasm/apl_wasm.js";
self.onmessage = async (event) => { self.onmessage = null; await init(); start(event.data); };`;
  serving.set('/worker.js', stale);
  const page = await visit();
  check('a worker from an older bundle still starts a session',
    await typing(page), JSON.stringify((await paper(page)).slice(-200)));
  await send(page, ')LIB 1');
  const shown = await paper(page);
  check('and that session answers',
    ['EDIT', 'LIFE', 'RACE'].every((n) => shown.includes(n)), JSON.stringify(shown.slice(-200)));
  await page.context().close();
  serving.delete('/worker.js');
}

// 3. A worker that never answers at all. Whatever is left unpaired
//    must say so on the paper rather than looking like a slow load.
{
  serving.set('/worker.js', 'self.onmessage = () => {};');
  const page = await visit();
  await page.waitForTimeout(12000);
  const said = await paper(page);
  check('a session that never starts says so', said.trim().length > 0, '(the paper stayed blank)');
  check('and says what to do about it', /SITE DATA|RELOAD/i.test(said), JSON.stringify(said.slice(-200)));
  await page.context().close();
  serving.delete('/worker.js');
}

// 4. The board: a reader with no APL keyboard, and no keyboard at
//    all. Entering a glyph by tapping is the whole point of it.
{
  const page = await visit();
  await page.setViewportSize({ width: 390, height: 844 }); // a phone
  await page.click('#show-board');
  check('the board is drawn from the keymap',
    (await page.locator('#board .key').count()) > 40, 'too few keys');

  // ")LIB 1" by tapping alone, and no physical key touched. ")" is
  // the quote key's glyph face, where a 2741 put it; the letters and
  // the digit come from the plain layer.
  const layer = '#board .controls .key.wide:not([aria-label^="return"])';
  const tapKey = (cap) => page.click(`#board .key[data-plain="${cap}"]`);
  await tapKey("'");                 // ) -- the APL layer is the default
  await page.click(layer);           // ABC
  for (const cap of ['L', 'I', 'B']) await tapKey(cap);
  await page.click('#board .key.space');
  await tapKey('1');
  await page.click('#board .controls .key[aria-label^="return"]');
  await page.waitForTimeout(900);
  const shown = await paper(page);
  check('a line entered by tapping alone runs',
    ['EDIT', 'LIFE', 'RACE'].every((n) => shown.includes(n)), JSON.stringify(shown.slice(-200)));

  // The line being typed must never be off screen on a phone.
  const onscreen = await page.evaluate(() => {
    const box = document.getElementById('line').getBoundingClientRect();
    return box.bottom <= innerHeight + 1 && box.top >= 0;
  });
  check('the line being typed is on screen at phone width', onscreen, 'it is not');
  await page.context().close();
}

// 5. A host that sends the isolation headers itself, which is what
//    `just demo` and `just pages-serve` do. There is nothing for the
//    service worker to forge, so the page must be isolated on the
//    first response and must not need a second load.
{
  const direct = await host(ROOT, true);
  const at = `http://127.0.0.1:${direct.address().port}/`;
  const page = await (await browser.newContext()).newPage();
  let loads = 0;
  page.on('load', () => { loads += 1; });
  await page.goto(at, { waitUntil: 'load' });
  await settle(page);
  check('a host that sends the headers needs no service worker',
    await page.evaluate(() => self.crossOriginIsolated), 'not isolated');
  check('and the page loads once', loads === 1, `loaded ${loads} times`);
  check('and the session starts', await typing(page), 'the prompt never came');
  await page.context().close();
  direct.close();
}

await browser.close();
server.close();
console.log(failed ? `check-pages: ${failed} failed` : 'check-pages: all passed');
process.exit(failed ? 1 : 0);
