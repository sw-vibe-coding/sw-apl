// The session's thread.
//
// Everything that can block happens here and nowhere else: the page's
// own thread is forbidden to call Atomics.wait, and a worker's is
// free to. start() does not return while the session lasts -- it is
// apl-serve holding a Session, and a statement that reads stops this
// thread until the page puts a line in the shared channel.
import init, { start } from "./wasm/apl_wasm.js";

self.onmessage = async (event) => {
  // One message, ever: the channel. Everything after it arrives
  // through shared memory, because this thread is about to stop
  // reading messages for good.
  self.onmessage = null;
  await init();
  start(event.data);
};
