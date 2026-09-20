// The page: a channel, a worker, and a paper to print on.

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
const typed = document.getElementById("typed");

// What the carriage has put on the page, as one string, so that a
// line the terminal has not finished can be carried on.
let page = "";
const put = (text) => {
  page += text;
  paper.textContent = page;
  paper.scrollTop = paper.scrollHeight;
};

const stop = (why) => {
  typed.disabled = true;
  prompt.textContent = "";
  put(why + "\n");
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
  const channel = new SharedArrayBuffer(SIZE);
  const header = new Int32Array(channel, 0, 2);
  const body = new Uint8Array(channel, BODY);
  const worker = new Worker("worker.js", { type: "module" });
  worker.onmessage = (event) => show(JSON.parse(event.data));
  worker.onerror = () => stop("The session could not be started.");
  worker.postMessage(channel);
  listen(header, body);
  // A tab that goes away is a terminal that hung up.
  addEventListener("pagehide", () => {
    Atomics.store(header, STATE, CLOSED);
    Atomics.notify(header, STATE);
  });
}

// Print one frame and take the typing it asks for.
function show(frame) {
  for (const line of frame.lines) put(line + "\n");
  if (frame.off || frame.prompt === null) return stop("Session ended.");
  prompt.textContent = frame.prompt;
  typed.disabled = false;
  typed.focus();
}

// Put a typed line in the channel and wake the session.
function listen(header, body) {
  typed.addEventListener("keydown", (event) => {
    if (event.key !== "Enter" || typed.disabled) return;
    // The paper keeps what was typed, as a printing terminal would:
    // the prompt, the line, and the answer under it.
    put(prompt.textContent + typed.value + "\n");
    const bytes = new TextEncoder().encode(typed.value);
    const fits = Math.min(bytes.length, body.length);
    body.set(bytes.subarray(0, fits));
    Atomics.store(header, LENGTH, fits);
    Atomics.store(header, STATE, READY);
    Atomics.notify(header, STATE);
    typed.value = "";
    typed.disabled = true;
    prompt.textContent = "";
  });
}

run();
