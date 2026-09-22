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
    ['BIRDS', 'EDIT', 'LIFE', 'RACE'].every((n) => shown.includes(n)), JSON.stringify(shown.slice(-120)));
  // BIRDS is baked into the bundle like the rest, and its birds fly
  // here as they do at the CLI -- the Starling included, which reaches
  // DYAD through a local of its own by dynamic scope.
  await send(page, ')LOAD 1 BIRDS');
  await send(page, "'+⌽' S ⍳5");
  check('BIRDS loads in the browser and its birds fly',
    (await paper(page)).trim().endsWith('6 6 6 6 6'), JSON.stringify((await paper(page)).slice(-60)));
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
  check('it is the 2741 picture, drawn inline, with a hit region per key',
    (await page.locator('#board .picture .face-shifted').count()) === 44
      && (await page.locator('#board .hit:not(.wide)').count()) === 45,
    `${await page.locator('#board .hit:not(.wide)').count()} hit regions`);

  // The face a key sends is drawn boldly and the other dimmed; Caps
  // Lock is lit in APL and dim in ABC, and switches between them.
  const faces = () => page.evaluate(() => {
    const op = (sel) => Number(getComputedStyle(document.querySelector(sel)).opacity);
    return {
      mode: document.getElementById('board').dataset.mode,
      shifted: op('#board .face-shifted'), normal: op('#board .face-normal'),
      caps: getComputedStyle(document.querySelector('#board .caps-lock-face')).fill,
      capsOpacity: op('#board .caps-lock-edge'),
    };
  });
  const apl = await faces();
  check('in APL the glyphs are bold and the letters dim',
    apl.mode === 'apl' && apl.shifted === 1 && apl.normal < 0.5, JSON.stringify(apl));
  check('and Caps Lock is lit', apl.caps === 'rgb(245, 179, 1)', apl.caps);
  await page.click('#board .hit.caps');
  await page.waitForTimeout(250);
  const abc = await faces();
  check('Caps Lock switches to ABC', abc.mode === 'abc', abc.mode);
  check('where the letters are bold and the glyphs dim',
    abc.normal === 1 && abc.shifted < 0.5, JSON.stringify(abc));
  check('and Caps Lock is dim', abc.capsOpacity < 1 && abc.caps !== 'rgb(245, 179, 1)', JSON.stringify(abc));
  await page.click('#board .hit.caps');
  await page.waitForTimeout(250);
  check('and switches back', (await faces()).mode === 'apl', 'it did not');

  // Tab and the Shifts do nothing, so they look inert in either mode.
  const inert = () => page.evaluate(() => [...document.querySelectorAll('#board .inert')]
    .map((el) => Number(getComputedStyle(el).opacity)));
  const inApl = await inert();
  await page.click('#board .hit.caps');
  await page.waitForTimeout(250);
  const inAbc = await inert();
  await page.click('#board .hit.caps');
  check('Tab and the Shifts are greyed in both modes',
    inApl.length > 0 && [...inApl, ...inAbc].every((o) => o < 0.5),
    JSON.stringify({ inApl, inAbc }));
  const attnAt = await page.evaluate(() => {
    const a = document.querySelector('#board .attn').getBoundingClientRect();
    const one = document.querySelector('#board .hit[data-plain="1"]').getBoundingClientRect();
    return { left: a.left < one.left, top: Math.abs(a.top - one.top) < 3, wider: a.width > one.width };
  });
  check('ATTN is top left, and wider than a key', attnAt.left && attnAt.top && attnAt.wider,
    JSON.stringify(attnAt));
  const returnBig = await page.evaluate(() => {
    const r = document.querySelector('#board .controls .return').getBoundingClientRect();
    const e = document.querySelector('#board .controls .erase').getBoundingClientRect();
    return r.width > e.width;
  });
  check('and Return is the biggest control', returnBig, 'it is not');

  // ")LIB 1" by tapping alone, and no physical key touched. ")" is
  // the quote key's glyph face, where a 2741 put it; the letters and
  // the digit come from the plain layer.
  const tapKey = (cap) => page.click(`#board .key[data-plain="${cap}"]`);
  await tapKey("'");                 // ) -- the APL face, the default mode
  await page.click('#board [data-mode=abc]');
  for (const cap of ['L', 'I', 'B']) await tapKey(cap);
  await page.click('#board .key.space');
  await tapKey('1');
  await page.click('#board .controls .key[aria-label^="return"]');
  await page.waitForTimeout(900);
  const shown = await paper(page);
  check('a line entered by tapping alone runs',
    ['EDIT', 'LIFE', 'RACE'].every((n) => shown.includes(n)), JSON.stringify(shown.slice(-200)));

  // The picture's own Return key sends the line, as the button does.
  await page.click('#board [data-mode=idioms]');
  await page.click('#board .list[data-kind=idioms] .entry[title="×/⍳5"]');
  await page.click('#board [data-mode=apl]');
  await page.click('#board .hit.return');
  await page.waitForTimeout(800);
  check("the picture's Return sends the line", (await paper(page)).trim().endsWith('120'),
    JSON.stringify((await paper(page)).slice(-40)));

  // Commands and Idioms insert, and send nothing on their own: a reader
  // always presses Return.
  for (const [modeId, text] of [['commands', ')LIB 1'], ['idioms', '+/⍳10']]) {
    await page.click(`#board [data-mode=${modeId}]`);
    await page.click(`#board .list[data-kind=${modeId}] .entry[title="${text}"]`);
    const line = await page.evaluate(() =>
      document.getElementById('before').textContent + document.getElementById('after').textContent);
    check(`${modeId} inserts ${text} without sending it`, line === text, JSON.stringify(line));
    await page.click('#board .controls .key[aria-label^="return"]');
    await page.waitForTimeout(700);
  }
  await page.click('#board [data-mode=apl]');

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
    /\w+ .*\d{4}-/.test(said.built), said.built);
  check('and the repository and the licence',
    said.repo.includes('github.com/sw-vibe-coding/sw-apl'), said.repo);
  check('and it is one wrapping line, not paragraphs',
    (await page.evaluate(() => document.querySelectorAll('.colophon p').length)) === 0
      && (await page.evaluate(() => getComputedStyle(document.querySelector('.colophon')).flexWrap))
        === 'wrap',
    'still prose');

  // The keyboard picture is somebody else's work; the credit is
  // shown exactly when the board is, and points at the licence.
  await page.click('#show-board');
  const credit = await page.evaluate(() => {
    const a = document.querySelector('#board .credit a');
    const board = document.getElementById('board');
    return { href: a?.href, visible: !!a && !board.hidden };
  });
  check('the keyboard credit shows with the board',
    credit.visible && credit.href?.includes('images/redistributed/apl-keyboard'),
    JSON.stringify(credit));
  const hidden = await page.evaluate(() => {
    document.getElementById('show-board').click();
    return document.getElementById('board').hidden;
  });
  check('and goes away with it', hidden, 'still shown');

  // The board pins to the top inside the session, not into the
  // colophon: its ground moved when the session became a wrapper.
  await page.click('#show-board');
  await page.click('#board .key.pin');
  const inside = await page.evaluate(() =>
    document.getElementById('board').parentElement.id === 'session');
  check('and the board still pins inside the session', inside, 'it escaped');
  await page.context().close();
}

// 8. What the viewport does to the page. The session must be the
//    height the viewport really is, in portrait and in landscape,
//    and a transcript long enough to overflow must scroll inside
//    the paper without pushing the line or the bar off screen.
{
  const page = await visit();
  for (const [name, size] of [
    ['portrait', { width: 390, height: 844 }],
    ['landscape', { width: 844, height: 390 }],
  ]) {
    await page.setViewportSize(size);
    await page.waitForTimeout(400);
    const fits = await page.evaluate(() => {
      const session = document.getElementById('session');
      const line = document.getElementById('line').getBoundingClientRect();
      return {
        height: Math.round(session.getBoundingClientRect().height),
        want: Math.round(visualViewport?.height ?? innerHeight),
        lineOn: line.bottom <= innerHeight + 1 && line.top >= 0,
      };
    });
    check(`the session is the viewport's height in ${name}`,
      Math.abs(fits.height - fits.want) <= 1, JSON.stringify(fits));
    check(`and the line being typed is on screen in ${name}`, fits.lineOn, JSON.stringify(fits));
  }

  // A transcript that overflows: RACE fills the paper many times over.
  await page.setViewportSize({ width: 390, height: 844 });
  await send(page, ')LOAD 1 RACE');
  await send(page, 'RACE');
  await page.waitForTimeout(2500);
  const long = await page.evaluate(() => {
    const paper = document.getElementById('paper');
    const line = document.getElementById('line').getBoundingClientRect();
    const bar = document.querySelector('.bar').getBoundingClientRect();
    return {
      overflows: paper.scrollHeight > paper.clientHeight + 10,
      scrolled: paper.scrollTop > 0,
      lineOn: line.bottom <= innerHeight + 1,
      barOn: bar.bottom <= innerHeight + 1,
    };
  });
  check('a long transcript scrolls inside the paper',
    long.overflows && long.scrolled, JSON.stringify(long));
  check('and does not push the line or the bar off screen',
    long.lineOn && long.barOn, JSON.stringify(long));
  await page.context().close();
}

// 9. History: what the CLI's line editor does, and what the page
//    could not do until now. Up recalls, down walks back, and down
//    past the end returns the line that was being typed.
{
  const page = await visit();
  const line = () => page.evaluate(() =>
    document.getElementById('before').textContent + document.getElementById('after').textContent);
  const arrow = async (which) => {
    await page.keyboard.press(which);
    await page.waitForTimeout(150);
  };

  await send(page, '2+2');
  await send(page, '3+3');
  await page.waitForTimeout(400);

  await arrow('ArrowUp');
  check('up recalls the last line entered', (await line()) === '3+3', await line());
  await arrow('ArrowUp');
  check('and again the one before it', (await line()) === '2+2', await line());
  await arrow('ArrowUp');
  check('and stops at the oldest', (await line()) === '2+2', await line());
  await arrow('ArrowDown');
  check('down walks back towards the present', (await line()) === '3+3', await line());
  await arrow('ArrowDown');
  check('and past the end is an empty line again', (await line()) === '', await line());

  // A half-typed line is kept, not thrown away by a look at history.
  await page.evaluate(() => {
    const d = new DataTransfer();
    d.setData('text', '9+');
    dispatchEvent(new ClipboardEvent('paste', { clipboardData: d, bubbles: true, cancelable: true }));
  });
  await page.waitForTimeout(150);
  await arrow('ArrowUp');
  check('looking at history does not lose the line being typed',
    (await line()) === '3+3', await line());
  await arrow('ArrowDown');
  check('and coming back gives it back', (await line()) === '9+', await line());

  // A recalled line really runs.
  await arrow('ArrowUp');
  await page.keyboard.press('Enter');
  await page.waitForTimeout(900);
  check('a recalled line runs', (await paper(page)).trim().endsWith('6'),
    JSON.stringify((await paper(page)).slice(-60)));
  await page.context().close();
}

// 9b. An overstrike made on the board strikes, exactly as one made
//     at a physical keyboard does. The board sends its glyphs by
//     paste, and paste used to drop a pending strike and put the glyph
//     beside its base. The owner's own sequence, both ways, compared.
{
  const line = (page) => page.evaluate(() =>
    document.getElementById('before').textContent + document.getElementById('after').textContent);

  const typed = await visit();
  await typed.keyboard.press('a');
  await typed.keyboard.press('Control+BracketRight');
  await typed.keyboard.press('Shift+F');
  const atKeyboard = await line(typed);
  await typed.context().close();

  const tapped = await visit();
  await tapped.click('#show-board');
  await tapped.click('#board [data-mode=abc]');
  await tapped.click('#board .hit[data-plain="A"]');
  await tapped.click('#board .hit[data-plain="BACKSPACE"]');
  await tapped.click('#board [data-mode=apl]');
  await tapped.click('#board .hit[data-plain="F"]');
  const onBoard = await line(tapped);
  await tapped.context().close();

  check('A, overstrike, _ at the keyboard makes one struck glyph',
    atKeyboard === 'A\u0332', JSON.stringify(atKeyboard));
  check('and the same on the board makes the same glyph',
    onBoard === atKeyboard, `board ${JSON.stringify(onBoard)}, keyboard ${JSON.stringify(atKeyboard)}`);
}

// 10. ATTN in the browser. A loop that runs forever must stop, by
//     Escape, by Ctrl-[, and by the board's key -- the last being the
//     only way on a touch screen, which has no Escape at all. Each
//     starts a real `→1` loop and waits for INTERRUPT with a deadline,
//     because a loop that cannot be stopped would otherwise hang the
//     check.
{
  const LOOP = ['∇SPIN', 'X←1', '→1', '∇'];
  const stopped = (page) => page.evaluate(() =>
    document.getElementById('paper').textContent.includes('INTERRUPT'));
  const until = async (page, test, ms) => {
    for (let waited = 0; waited < ms; waited += 100) {
      if (await test(page)) return true;
      await page.waitForTimeout(100);
    }
    return false;
  };
  const busy = (page) => page.evaluate(() =>
    document.getElementById('line').classList.contains('waiting'));

  for (const [how, press] of [
    ['Escape', (page) => page.keyboard.press('Escape')],
    ['Ctrl-[', (page) => page.keyboard.press('Control+BracketLeft')],
    ['the board key', (page) => page.click('#board .key.attn')],
  ]) {
    const page = await visit();
    await page.setViewportSize({ width: 390, height: 844 });
    if (how === 'the board key') await page.click('#show-board');
    for (const line of LOOP) await send(page, line);
    await page.evaluate(() => {
      const d = new DataTransfer();
      d.setData('text', 'SPIN');
      dispatchEvent(new ClipboardEvent('paste', { clipboardData: d, bubbles: true, cancelable: true }));
      dispatchEvent(new KeyboardEvent('keydown', { key: 'Enter', bubbles: true }));
    });
    const running = await until(page, busy, 2000);
    await page.waitForTimeout(300);
    await press(page);
    const ok = running && await until(page, stopped, 8000);
    check(`${how} stops a loop that runs forever`, ok,
      JSON.stringify((await paper(page)).slice(-80)));
    await page.context().close();
  }

  // An ATTN at the prompt has nothing to stop, and must not linger and
  // stop the next statement the instant it starts.
  {
    const page = await visit();
    await page.keyboard.press('Escape');
    await send(page, '2+2');
    const said = await paper(page);
    check('an ATTN at the prompt does not stop the next line',
      !said.includes('INTERRUPT') && said.trim().endsWith('4'), JSON.stringify(said.slice(-40)));
    await page.context().close();
  }
}

// 11. The header: the name, then the modes as a row of tabs, the
//     current one marked -- (B) on a first visit -- each described to
//     mouse, touch and screen reader alike.
{
  const page = await visit();
  const header = () => page.evaluate(() => ({
    name: document.querySelector('header .name')?.textContent,
    tabs: [...document.querySelectorAll('header [role=tablist] [role=tab]')].map((tab) => ({
      shows: tab.textContent.trim(),
      title: tab.title,
      spoken: tab.getAttribute('aria-label'),
      selected: tab.getAttribute('aria-selected'),
    })),
    about: document.getElementById('mode-about').textContent,
    aboutHidden: document.getElementById('mode-about').hidden,
  }));
  const head = await header();
  const [a, b] = head.tabs;
  check('the header is sw-apl and then the modes',
    head.name === 'sw-apl' && a?.shows === "\u24b6 '70" && b?.shows === "\u24b7 '75",
    JSON.stringify(head));
  check('two tabs, (B) marked current on a first visit',
    head.tabs.length === 2 && a.selected === 'false' && b.selected === 'true', JSON.stringify(head));
  check("(A)'s tooltip says APL\\360-inspired", a.title === 'APL\\360-inspired', a.title);
  check("(B)'s tooltip says IBM 5100-inspired", b.title === 'IBM 5100-inspired', b.title);
  check('and a screen reader hears the words, not the circled letters',
    a.spoken === '1970, APL\\360-inspired' && b.spoken === '1975, IBM 5100-inspired',
    `${a.spoken} / ${b.spoken}`);
  await page.click('#mode-tab-b');
  check('a tap on the current tab shows its words where there is no hover',
    head.aboutHidden && !(await page.evaluate(() => document.getElementById('mode-about').hidden))
      && head.about === 'IBM 5100-inspired',
    'the description did not appear');

  // The session is (B): execute and the system variables, no )ORIGIN.
  await send(page, "\u234e'2+3'");
  check('(B) executes', (await paper(page)).trim().endsWith('5'),
    JSON.stringify((await paper(page)).slice(-60)));
  await send(page, '\u2395IO');
  check('(B) has the quad system variables', (await paper(page)).trim().endsWith('1'),
    JSON.stringify((await paper(page)).slice(-60)));
  await send(page, ')ORIGIN 0');
  check("(B) has no )ORIGIN", /INCORRECT COMMAND/.test((await paper(page)).slice(-60)),
    JSON.stringify((await paper(page)).slice(-60)));

  // The board is built for (B): execute struck from up tack jot's pair.
  const line = () => page.evaluate(() =>
    document.getElementById('before').textContent + document.getElementById('after').textContent);
  await page.evaluate(() => {
    const d = new DataTransfer();
    d.setData('text', '\u22a5');
    dispatchEvent(new ClipboardEvent('paste', { clipboardData: d, bubbles: true, cancelable: true }));
  });
  await page.keyboard.press('Control+BracketRight');
  await page.keyboard.press('Shift+J');
  check("the board composes (B)'s overstrikes", (await line()) === '\u234e', await line());
  await page.keyboard.press('Backspace');

  // The other tab asks first, and says what becomes of the workspace.
  await send(page, 'X\u219042');
  await page.click('#mode-tab-a');
  const asked = await page.evaluate(() => {
    const d = document.getElementById('switch');
    return { open: d.open, said: d.textContent };
  });
  check('the other tab asks before it switches', asked.open, JSON.stringify(asked));
  check('and says the workspace in hand is not carried across',
    /workspace in hand is not carried across/.test(asked.said) && asked.said.includes(')SAVE'),
    JSON.stringify(asked.said));
  await page.click('#switch button[value=stay]');
  await page.waitForTimeout(300);
  await send(page, 'X');
  check('Stay keeps the session and its workspace',
    (await paper(page)).trim().endsWith('42') && (await header()).tabs[1].selected === 'true',
    JSON.stringify((await paper(page)).slice(-40)));

  // Switch: a new session, in (A).
  await page.click('#mode-tab-a');
  await Promise.all([page.waitForNavigation(), page.click('#switch button[value=switch]')]);
  await page.waitForTimeout(1500);
  await settle(page);
  const now = await header();
  check('Switch starts the session in (A)',
    now.tabs[0].selected === 'true' && now.tabs[1].selected === 'false'
      && now.about === 'APL\\360-inspired' && new URL(page.url()).searchParams.get('mode') === 'A',
    JSON.stringify(now));
  await send(page, 'X');
  check('in a clear workspace', /VALUE ERROR/.test(await paper(page)),
    JSON.stringify((await paper(page)).slice(-60)));
  await send(page, "\u234e'2+3'");
  check('(A) has no execute', /ERROR/.test((await paper(page)).slice(-60)),
    JSON.stringify((await paper(page)).slice(-60)));
  await send(page, ')ORIGIN 0');
  check('(A) has )ORIGIN', (await paper(page)).trim().endsWith('WAS 1'),
    JSON.stringify((await paper(page)).slice(-60)));

  // The mode is remembered: the next visit, at the bare address, is (A).
  await page.goto(url, { waitUntil: 'load' });
  await page.waitForTimeout(1500);
  await settle(page);
  check('the next visit comes back in (A)', (await header()).tabs[0].selected === 'true',
    JSON.stringify(await header()));
  await page.context().close();
}

// 12. Installable: the manifest a browser reads before it offers to
//    install, and the icons it shows afterwards. Relative start_url
//    and scope, so it installs from a project page's sub-path too.
{
  const page = await visit();
  const missed = [];
  page.on('response', (r) => { if (r.status() >= 400) missed.push(r.url()); });
  const manifest = await page.evaluate(async () => {
    const link = document.querySelector('link[rel=manifest]');
    if (!link) return null;
    return fetch(link.href).then((r) => (r.ok ? r.json() : null));
  });
  check('there is a manifest', manifest !== null, 'none');
  check('it names the app and installs standalone',
    manifest?.short_name === 'sw-apl' && manifest?.display === 'standalone',
    JSON.stringify(manifest && { n: manifest.short_name, d: manifest.display }));
  check('its start_url and scope are relative',
    !String(manifest?.start_url).startsWith('/') && !String(manifest?.scope).startsWith('/'),
    `${manifest?.start_url} ${manifest?.scope}`);
  check('it offers the sizes an install needs, one of them maskable',
    ['192x192', '512x512'].every((s) => manifest?.icons?.some((i) => i.sizes === s))
      && manifest?.icons?.some((i) => i.purpose === 'maskable'),
    JSON.stringify(manifest?.icons?.map((i) => `${i.sizes} ${i.purpose}`)));

  // Every icon, the favicon and the apple-touch icon must actually
  // be there: a manifest naming a missing icon installs a blank tile.
  const icons = [
    ...(manifest?.icons ?? []).map((i) => i.src),
    'favicon.ico',
    'apple-touch-icon.png',
  ];
  const got = await page.evaluate(async (list) => {
    const out = {};
    for (const src of list) {
      const r = await fetch(new URL(src, location.href));
      out[src] = r.ok ? (await r.blob()).size : 0;
    }
    return out;
  }, icons);
  check('and every icon it names is really there',
    Object.values(got).every((size) => size > 500), JSON.stringify(got));
  check('and nothing 404s on the way', missed.length === 0, missed.join(' '));
  await page.context().close();
}

// 13. Espanso, and any other OS-level expander. It watches the
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

// 14. Under a sub-path, which is where GitHub Pages actually serves
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
