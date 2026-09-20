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

// A GitHub project page is served from /<repo>/, never from the
// root. Every URL the page and the worker fetch has to be relative
// for that to work, and the service worker's scope has to cover
// them -- so the checks run under a prefix as well as at the root.
const PREFIX = '/sw-apl/';

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

function host(dir, isolated = false, prefix = '/') {
  const extra = isolated ? ISOLATION : {};
  const server = createServer(async (req, res) => {
    let path = normalize(new URL(req.url, 'http://x').pathname);
    if (prefix !== '/') {
      if (!path.startsWith(prefix)) return res.writeHead(404).end('not found');
      path = path.slice(prefix.length - 1) || '/';
    }
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

// 5. Where the board sits and how big it is, kept across a reload.
//    A reader who has made the keys small on a phone must not have
//    to do it again every visit.
{
  const context = await browser.newContext();
  const page = await context.newPage();
  await page.goto(url, { waitUntil: 'load' });
  await page.waitForTimeout(3000);
  await settle(page);
  await page.setViewportSize({ width: 390, height: 844 });
  await page.click('#show-board');
  await page.click('#board .key.pin');                       // to the top
  await page.click('#board .key.small[aria-label="smaller keys"]');
  const was = await page.evaluate(() => {
    const board = document.getElementById('board');
    return { edge: board.dataset.edge, size: board.dataset.size };
  });
  check('the board moves to the top', was.edge === 'top', was.edge);

  const again = await context.newPage();
  await again.goto(url, { waitUntil: 'load' });
  await again.waitForTimeout(3000);
  await settle(again);
  const now = await again.evaluate(() => {
    const board = document.getElementById('board');
    return {
      edge: board.dataset.edge,
      size: board.dataset.size,
      shown: !board.hidden,
      // Pinned to the top means the board really is above the paper.
      above: board.compareDocumentPosition(document.getElementById('paper'))
        & Node.DOCUMENT_POSITION_FOLLOWING,
    };
  });
  check('and is found where it was left', now.shown && now.edge === was.edge && now.above > 0,
    JSON.stringify(now));
  check('and at the size it was left', now.size === was.size, `${now.size} was ${was.size}`);
  await context.close();
}

// 6. A host that sends the isolation headers itself, which is what
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

// 7. The colophon: below the fold, and not one pixel of the session
//    given up for it. The build facts come from build-info.json, so
//    a page that has them proves the whole path.
{
  const page = await visit();
  await page.setViewportSize({ width: 390, height: 844 });
  await page.waitForTimeout(500);
  const fold = await page.evaluate(() => {
    const session = document.getElementById('session');
    const foot = document.querySelector('.colophon');
    return {
      fills: Math.abs(session.getBoundingClientRect().height - innerHeight) <= 1,
      below: foot.getBoundingClientRect().top >= innerHeight - 1,
      scrolls: document.documentElement.scrollHeight > innerHeight,
    };
  });
  check('the session still fills the window exactly', fold.fills, JSON.stringify(fold));
  check('and the colophon is below the fold', fold.below && fold.scrolls, JSON.stringify(fold));

  await page.evaluate(() => document.querySelector('.colophon').scrollIntoView());
  await page.waitForTimeout(300);
  const said = await page.evaluate(() => ({
    built: document.getElementById('built').textContent,
    repo: [...document.querySelectorAll('.colophon a')].map((a) => a.href).join(' '),
  }));
  check('it names the build it is running',
    /Built \w+.* at \d{4}-/.test(said.built), said.built);
  check('and the repository and the licence',
    said.repo.includes('github.com/sw-vibe-coding/sw-apl') && said.repo.includes('LICENSE'),
    said.repo);

  // The board pins to the top inside the session, not into the
  // colophon: its ground moved when the session became a wrapper.
  await page.click('#show-board');
  await page.click('#board .key.pin');
  const inside = await page.evaluate(() =>
    document.getElementById('board').parentElement.id === 'session');
  check('and the board still pins inside the session', inside, 'it escaped');
  await page.context().close();
}

// 8. Espanso, and any other OS-level expander. It watches the
//    keystrokes before the browser sees them, so the 2741 map
//    cannot hide a trigger from it -- but it replaces what was
//    typed either by sending backspaces and the glyph as a key, or
//    by pasting, and the page has to accept both. Backtick is not
//    in the keymap, so the trigger passes through untouched.
{
  const page = await visit();
  const line = () => page.evaluate(() =>
    document.getElementById('before').textContent + document.getElementById('after').textContent);

  await page.keyboard.press('`');
  await page.keyboard.press('r');
  check('an expander trigger reaches the line untouched', (await line()) === '`R', await line());

  await page.keyboard.press('Backspace');
  await page.keyboard.press('Backspace');
  await page.evaluate(() => dispatchEvent(new KeyboardEvent('keydown', { key: '⍴', bubbles: true })));
  check('a glyph sent as a keystroke arrives as itself', (await line()) === '⍴', await line());

  await page.evaluate(() => {
    const d = new DataTransfer();
    d.setData('text', '⍳5');
    dispatchEvent(new ClipboardEvent('paste', { clipboardData: d, bubbles: true, cancelable: true }));
  });
  check('and a glyph pasted in arrives as itself', (await line()) === '⍴⍳5', await line());
  await page.context().close();
}

// 9. Under a sub-path, which is where GitHub Pages actually serves
//    a project page from. Nothing must be fetched from the root.
{
  const sub = await host(ROOT, false, PREFIX);
  const at = `http://127.0.0.1:${sub.address().port}${PREFIX}`;
  const page = await (await browser.newContext()).newPage();
  const missed = [];
  page.on('response', (r) => { if (r.status() >= 400) missed.push(r.url()); });
  await page.goto(at, { waitUntil: 'load' });
  await page.waitForTimeout(3500);
  await settle(page);
  check('the page works under a sub-path', await typing(page),
    JSON.stringify((await paper(page)).slice(-200)));
  check('and fetches nothing from the root',
    missed.filter((u) => !u.endsWith('favicon.ico')).length === 0, missed.join(' '));
  await send(page, ')LIB 1');
  const shown = await paper(page);
  check('and runs a line there',
    ['EDIT', 'LIFE', 'RACE'].every((n) => shown.includes(n)), JSON.stringify(shown.slice(-160)));
  await page.context().close();
  sub.close();
}

await browser.close();
server.close();
console.log(failed ? `check-pages: ${failed} failed` : 'check-pages: all passed');
process.exit(failed ? 1 : 0);
