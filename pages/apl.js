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
// loaded again under it, once.
async function isolate() {
  if (self.crossOriginIsolated) return true;
  if (!("serviceWorker" in navigator)) return false;
  try {
    await navigator.serviceWorker.register("sw.js");
    await navigator.serviceWorker.ready;
  } catch {
    return false;
  }
  if (sessionStorage.getItem("apl-isolated")) return false;
  sessionStorage.setItem("apl-isolated", "1");
  location.reload();
  return false;
}

// Start the session and wire the keyboard to it.
async function run() {
  if (!(await isolate())) {
    stop(
      "This browser will not give the page shared memory, which the " +
        "interpreter needs in order to wait for a line. Try a window " +
        "that is not private, or run sw-apl locally instead.",
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
