// The page: a keyboard, a channel, a worker, a paper to print on,
// and somewhere to keep what )SAVE writes.
//
// The keyboard is the 2741's, and it is the same keyboard aplterm
// has: apl-keyboard compiled to WebAssembly, holding the keymap, the
// overstrike table and the cells a line is made of. The page sends it
// keystrokes and draws what it hands back, and never learns what an
// overstrike is.
import init, { Board } from "./wasm/apl_wasm.js";

// Must agree with channel.rs, which is where these are documented.
const SIZE = 64 * 1024;
const BODY = 8;
const STATE = 0;
const LENGTH = 1;
const WAITING = 0;
const READY = 1;
const CLOSED = 2;

const paper = document.getElementById("paper");
const prompt = document.getElementById("prompt");
const line = document.getElementById("line");
const before = document.getElementById("before");
const after = document.getElementById("after");

// The keyboard, and whether it is our turn to type.
let board = null;
let typing = false;

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

// Start the session and wire the keyboard to it.
async function run() {
  if (!(await isolate())) {
    stop(
      "This browser will not give the page shared memory, which the " +
        "interpreter needs in order to stop and wait for a line.\n" +
        "Close the tab and open it again, which is enough if an " +
        "earlier visit left a stale refusal behind. A private window " +
        "will not work, because it refuses the service worker that " +
        "asks for the headers.\n" +
        "Failing that, run the interpreter locally: just demo.",
    );
    return;
  }
  await init();
  board = new Board();
  const channel = new SharedArrayBuffer(SIZE);
  const header = new Int32Array(channel, 0, 2);
  const body = new Uint8Array(channel, BODY);
  const worker = new Worker("worker.js", { type: "module" });
  // A frame is a string; library 0 is an object. That is the whole
  // of the difference, because only one of the two is the protocol.
  worker.onmessage = (event) =>
    typeof event.data === "string"
      ? show(JSON.parse(event.data))
      : keep(event.data.work);
  worker.onerror = () => stop("The session could not be started.");
  worker.postMessage({ channel, stored: stored() });
  listen(header, body);
  // A tab that goes away is a terminal that hung up.
  addEventListener("pagehide", () => {
    Atomics.store(header, STATE, CLOSED);
    Atomics.notify(header, STATE);
  });
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
  const bytes = new TextEncoder().encode(text);
  const fits = Math.min(bytes.length, body.length);
  body.set(bytes.subarray(0, fits));
  Atomics.store(header, LENGTH, fits);
  Atomics.store(header, STATE, READY);
  Atomics.notify(header, STATE);
  typing = false;
  line.classList.add("waiting");
  prompt.textContent = "";
  draw(JSON.parse(board.press("Unidentified", false, false)));
}

// Every keystroke goes to the keyboard, and only the ones it claims
// are taken from the browser -- copy still copies, reload still
// reloads.
function listen(header, body) {
  addEventListener("keydown", (event) => {
    if (!typing || event.metaKey) return;
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
