// The session's thread.
//
// Everything that can block happens here and nowhere else: the page's
// own thread is forbidden to call Atomics.wait, and a worker's is
// free to. start() does not return while the session lasts -- it is
// apl-serve holding a Session, and a statement that reads stops this
// thread until the page puts a line in the shared channel.
//
// The bundle is loaded at the version this worker was fetched at, so
// a page and a worker that arrived together cannot end up on
// different builds of the WebAssembly.

const version = new URL(import.meta.url).searchParams.get("v") ?? "";
const bundle = `./wasm/apl_wasm.js${version && `?v=${encodeURIComponent(version)}`}`;

// Say on the page's own paper what went wrong here. A worker that
// dies quietly leaves a prompt that ignores typing, which a reader
// cannot tell from a slow load.
const refuse = (said) =>
  self.postMessage(JSON.stringify({ lines: said, prompt: null, off: true }));

self.onmessage = async (event) => {
  // One message, ever: the channel, and what the page kept of
  // library 0 from an earlier visit. Everything after it arrives
  // through shared memory, because this thread is about to stop
  // reading messages for good. A worker cannot reach local storage
  // itself, which is why the page reads it and sends it here.
  self.onmessage = null;
  try {
    const { default: init, start } = await import(bundle);
    await init();
    start(event.data);
  } catch (error) {
    refuse([
      "THE SESSION COULD NOT BE STARTED.",
      "THIS BROWSER MAY BE HOLDING AN OLDER COPY OF THE PAGE.",
      "CLEAR SITE DATA FOR THIS SITE AND RELOAD.",
      String(error),
    ]);
  }
};
