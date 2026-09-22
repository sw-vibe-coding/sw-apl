// The page: a keyboard, a channel, a worker, a paper to print on,
// and somewhere to keep what )SAVE writes.
//
// The keyboard is the 2741's, and it is the same keyboard aplterm
// has: apl-keyboard compiled to WebAssembly, holding the keymap, the
// overstrike table and the cells a line is made of. The page sends it
// keystrokes and draws what it hands back, and never learns what an
// overstrike is.
// The version this page was fetched at, passed on to everything it
// fetches. index.html reads it from version.txt, which the build
// writes and the page asks for uncached, so the page, the worker and
// the WebAssembly are always one build and never three.
const VERSION = new URL(import.meta.url).searchParams.get("v") ?? "";
const stamped = (path) =>
  `${path}${VERSION && `?v=${encodeURIComponent(VERSION)}`}`;

const { default: init, Board } = await import(stamped("./wasm/apl_wasm.js"));
const { build } = await import(stamped("./board.js"));

// Must agree with channel.rs, which is where these are documented.
const SIZE = 64 * 1024;
const BODY = 12;
const STATE = 0;
const LENGTH = 1;
const ATTN = 2;
const WAITING = 0;
const READY = 1;
const CLOSED = 2;

const paper = document.getElementById("paper");
const boardEl = document.getElementById("board");
const prompt = document.getElementById("prompt");
const line = document.getElementById("line");
const before = document.getElementById("before");
const after = document.getElementById("after");

// Lines entered, and where the reader is looking in them.
//
// The CLI has this through rustyline and the page had nothing, which
// was the one thing the terminal could do that the page could not.
// It lives here rather than in the keyboard crate because recall is
// a property of this terminal, not of the 2741 map: `ArrowUp` and
// `ArrowDown` reach `Act::Ignore` in `act`, so the page may claim
// them without touching anything below it.
const history = [];

// Where in `history` the reader is. At `history.length` they are on
// the line they are typing, which is held in `draft` so that walking
// down past the end gives it back -- a reader half way through a line
// who looked at history has not abandoned it.
let at = 0;
let draft = "";

// The keyboard, and whether it is our turn to type.
let board = null;
let typing = false;

// The channel, once run() has made it. The board taps through the
// same send() the physical keyboard does, so there is one path from
// a finished line to the session and not two.
let wire = null;

// What the carriage has put on the page, as one string, so that a
// line the terminal has not finished can be carried on.
let page = "";
const put = (text) => {
  page += text;
  paper.textContent = page;
  paper.scrollTop = paper.scrollHeight;
};

const stop = (why) => {
  typing = false;
  line.classList.add("waiting");
  prompt.textContent = "";
  put(why + "\n");
};

// Draw the line being typed, with the carriage where the keyboard
// says it is.
const draw = (state) => {
  before.textContent = state.before;
  after.textContent = state.text.slice(state.before.length);
  line.classList.toggle("pending", state.pending);
  if (state.bell) {
    line.animate([{ opacity: 1 }, { opacity: 0.3 }, { opacity: 1 }], 140);
  }
};

// Cross-origin isolation, without which there is no SharedArrayBuffer
// and so no way for the session's thread to block. The service worker
// adds the headers a static host will not; the page then has to be
// built again under it, because the response that built this one was
// fetched before the worker could touch it.
//
// The count is what stops a reload loop on a browser that will not
// isolate whatever we do, and it is cleared on success so that a
// later visit is not still carrying a refusal from an older build.
const RELOADS = "apl-isolation-reloads";

// Where library 0 lives between visits. A worker cannot reach local
// storage, so the page reads it before the session starts and writes
// it back whenever the session says library 0 has changed.
//
// It is this browser's, on this machine: a workspace saved here is
// not in another browser, not on another machine, and not anywhere
// but the tab it was typed in. Nothing is sent anywhere.
const LIBRARY = "apl-library-0";

// Whether the reader has already been told there is nowhere to keep
// a workspace, so they are told once and not after every )SAVE.
let warned = false;

const stored = () => {
  try {
    return localStorage.getItem(LIBRARY) || "{}";
  } catch {
    return "{}";
  }
};

const keep = (work) => {
  try {
    localStorage.setItem(LIBRARY, JSON.stringify(work));
  } catch {
    if (warned) return;
    warned = true;
    put(
      "THIS BROWSER WILL NOT KEEP WORKSPACES. )SAVE AND )LOAD WORK\n" +
        "UNTIL THIS TAB IS CLOSED, AND NOT AFTER IT.\n",
    );
  }
};

async function isolate() {
  if (self.crossOriginIsolated) {
    try { sessionStorage.removeItem(RELOADS); } catch { /* private mode */ }
    return true;
  }
  if (!("serviceWorker" in navigator)) return false;
  try {
    await navigator.serviceWorker.register("sw.js");
    await navigator.serviceWorker.ready;
  } catch (error) {
    console.error("sw-apl: the isolation worker would not install:", error);
    return false;
  }
  let tries = 0;
  try {
    tries = Number(sessionStorage.getItem(RELOADS) || 0);
    sessionStorage.setItem(RELOADS, String(tries + 1));
  } catch {
    // Without session storage there is no way to count, so reload
    // once and let the load after it decide.
    tries = navigator.serviceWorker.controller ? 2 : 0;
  }
  if (tries >= 2) {
    console.error("sw-apl: still not isolated after", tries, "reloads");
    return false;
  }
  location.reload();
  return false;
}

// How long to wait for the session's first frame before saying it is
// not coming. Starting it is a worker, a fetch and a WebAssembly
// instantiation, which is slow on a cold cache and not this slow.
const PATIENCE = 10000;

// Keep the session exactly as tall as the viewport really is.
//
// `100dvh` is the viewport with the browser's own UI retracted, which
// is not the same as the viewport there is right now: the URL bar
// slides in and out as a page is scrolled, and pinch-zoom changes it
// again. visualViewport is the one that knows, so the session is
// sized from it and follows it.
//
// Not for an on-screen keyboard, which is the usual reason to reach
// for this: nothing on this page is focusable -- no input, no
// textarea, nothing contenteditable, because keystrokes are read off
// the window -- so a phone cannot raise its keyboard over it. That is
// what the board on the page is for.
function viewport() {
  const seen = window.visualViewport;
  if (!seen) return;
  const fit = () =>
    document.documentElement.style.setProperty("--vh", `${seen.height}px`);
  seen.addEventListener("resize", fit);
  seen.addEventListener("scroll", fit);
  fit();
}

// Start the session and wire the keyboard to it.
async function run() {
  viewport();
  if (!(await isolate())) {
    // One line. A reader who cannot run it needs to know that and
    // where to look, not an essay on service workers -- the rest is
    // under Help.
    stop("NO SHARED MEMORY IN THIS BROWSER. SEE HELP.");
    return;
  }
  await init();
  board = new Board("A");
  const channel = new SharedArrayBuffer(SIZE);
  const header = new Int32Array(channel, 0, 3);
  const body = new Uint8Array(channel, BODY);
  const worker = new Worker(stamped("worker.js"), { type: "module" });
  // A worker that dies before it says anything leaves a prompt that
  // ignores typing, and a reader cannot tell that from a slow load
  // and has no reason to suspect their own cache. So say it.
  const watchdog = setTimeout(
    () =>
      stop(
        "THE SESSION DID NOT START. THIS BROWSER MAY BE HOLDING AN\n" +
          "OLDER COPY OF THE PAGE OR OF THE INTERPRETER.\n" +
          "CLEAR SITE DATA FOR THIS SITE AND RELOAD -- a plain reload\n" +
          "will not replace a worker that is already cached.",
      ),
    PATIENCE,
  );
  // A frame is a string; library 0 is an object. That is the whole
  // of the difference, because only one of the two is the protocol.
  worker.onmessage = (event) => {
    clearTimeout(watchdog);
    if (typeof event.data === "string") return show(JSON.parse(event.data));
    return keep(event.data.work);
  };
  worker.onerror = () => {
    clearTimeout(watchdog);
    stop("The session could not be started.");
  };
  worker.postMessage({ channel, stored: stored() });
  wire = { header, body };
  listen(header, body);
  await keyboard();
  await built();
  // A tab that goes away is a terminal that hung up.
  addEventListener("pagehide", () => {
    Atomics.store(header, STATE, CLOSED);
    Atomics.notify(header, STATE);
  });
}

// Whether the board was up last time. A reader who wants it should
// not have to ask on every visit, and one who does not must not be
// nagged.
const SHOWN = "apl-board-shown";

// Draw the board, wire its keys, and hang the two controls off it.
// A tap goes through the same Board the physical keyboard does, so
// an overstrike begun by tapping can be finished by typing.
async function keyboard() {
  const tap = (sends) => {
    if (!typing) return;
    const state = JSON.parse(strike(sends));
    if (state.submit) return send(wire.header, wire.body, board.take());
    draw(state);
  };
  await build(boardEl, stamped, tap, attention);

  const button = document.getElementById("show-board");
  const reveal = (show) => {
    boardEl.hidden = !show;
    button.setAttribute("aria-pressed", String(show));
    try {
      localStorage.setItem(SHOWN, show ? "1" : "");
    } catch { /* private mode: it just will not be remembered */ }
  };
  button.addEventListener("click", () => reveal(boardEl.hidden));
  let was = false;
  try {
    was = Boolean(localStorage.getItem(SHOWN));
  } catch { /* as above */ }
  reveal(was);

  // The mode tab: a tap shows what its tooltip says, because a touch
  // screen has no hover to show it.
  const about = document.getElementById("mode-about");
  document.getElementById("mode-tab")
    .addEventListener("click", () => { about.hidden = !about.hidden; });

  const help = document.getElementById("help");
  document.getElementById("show-help").addEventListener("click", () => help.showModal());
}

// What this page is running, for the colophon. It comes out of
// build-info.json, which `just publish` writes, so there is nowhere
// else for it to be written down and go stale. A page served
// straight out of a checkout has no such file and says so.
async function built() {
  const said = document.getElementById("built");
  try {
    const info = await fetch(stamped("build-info.json"), { cache: "no-store" })
      .then((r) => (r.ok ? r.json() : null));
    if (!info) throw new Error("no build-info.json");
    const host = info.host ? ` on ${info.host}` : "";
    said.textContent =
      `Built ${info.commit}${host} at ${info.built_at}, bundle ${info.version}.`;
  } catch {
    said.textContent = "Built from a working tree; no build was recorded.";
  }
}

// What a tapped key does to the line. The control keys stand for the
// keystrokes they are: the board does not get its own state machine,
// because then the two could disagree about a pending overstrike.
function strike(sends) {
  switch (sends) {
    case "backspace":
      return board.press("Backspace", false, false);
    case "overstrike":
      return board.press("]", true, false);
    case "return":
      return board.press("Enter", false, false);
    default:
      return board.paste(sends);
  }
}

// Print one frame and take the typing it asks for.
function show(frame) {
  for (const text of frame.lines) put(text + "\n");
  if (frame.off || frame.prompt === null) return stop("Session ended.");
  prompt.textContent = frame.prompt;
  typing = true;
  line.classList.remove("waiting");
}

// Put a typed line in the channel and wake the session.
function send(header, body, text) {
  // The paper keeps what was typed, as a printing terminal would:
  // the prompt, the line, and the answer under it.
  put(prompt.textContent + text + "\n");
  // Remember it, unless it is blank or the line just entered: a
  // history full of one repeated line is not worth walking.
  if (text.trim() && text !== history.at(-1)) history.push(text);
  at = history.length;
  draft = "";
  const bytes = new TextEncoder().encode(text);
  const fits = Math.min(bytes.length, body.length);
  body.set(bytes.subarray(0, fits));
  // An attention raised as the last run was finishing would otherwise
  // sit in its slot and stop this line the instant it started.
  Atomics.store(header, ATTN, 0);
  Atomics.store(header, LENGTH, fits);
  Atomics.store(header, STATE, READY);
  Atomics.notify(header, STATE);
  typing = false;
  line.classList.add("waiting");
  prompt.textContent = "";
  draw(JSON.parse(board.press("Unidentified", false, false)));
}

// ATTN: stop the run in progress.
//
// It is a store into the channel's attention slot and nothing else,
// because nothing else can reach a busy worker: a worker reads its
// messages only when its thread is idle, and a run never is. The
// interpreter polls the slot inside every primitive and between the
// lines of every function, so a loop of lines and one long statement
// both stop. Only while the session is busy -- at the prompt there is
// nothing to stop, and a flag left set would stop the next line.
function attention() {
  if (typing || !wire) return;
  Atomics.store(wire.header, ATTN, 1);
}

// Walk the history. `step` is -1 for up and 1 for down.
//
// The line being typed is kept the moment the reader steps off it,
// so that coming back down returns it rather than an empty line.
function recall(step) {
  if (!history.length && step < 0) return;
  if (at === history.length && step < 0) draft = board.take();
  const to = Math.min(history.length, Math.max(0, at + step));
  if (to === at) return;
  at = to;
  board.press("c", true, false);
  draw(JSON.parse(board.paste(at === history.length ? draft : history[at])));
}

// Every keystroke goes to the keyboard, and only the ones it claims
// are taken from the browser -- copy still copies, reload still
// reloads.
function listen(header, body) {
  addEventListener("keydown", (event) => {
    // ATTN, before the guard below: that guard ignores keys while the
    // session is busy, and busy is exactly when ATTN is wanted. Escape,
    // and Ctrl-[ which is the same byte. Cmd-[ is left alone: on a Mac
    // it is Back.
    const escape = event.key === "Escape"
      || (event.ctrlKey && !event.metaKey && event.key === "[");
    if (escape && !typing) {
      event.preventDefault();
      return attention();
    }
    if (!typing || event.metaKey) return;
    // The arrows the keyboard does not claim are this terminal's.
    if (!event.ctrlKey && !event.altKey
        && (event.key === "ArrowUp" || event.key === "ArrowDown")) {
      event.preventDefault();
      return recall(event.key === "ArrowUp" ? -1 : 1);
    }
    const state = JSON.parse(board.press(event.key, event.ctrlKey, event.altKey));
    if (!state.acted) return;
    event.preventDefault();
    if (state.submit) return send(header, body, board.take());
    draw(state);
  });
  addEventListener("paste", (event) => {
    if (!typing) return;
    event.preventDefault();
    draw(JSON.parse(board.paste(event.clipboardData.getData("text"))));
  });
}

run();
