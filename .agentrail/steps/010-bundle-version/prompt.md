Bug found 2026-09-20 while checking the browser-workspaces step in a
real browser. A returning visitor gets a dead prompt.

The page, the worker and the WebAssembly are one thing and are
cached as three. The browser-workspaces step changed the worker's
message from the channel alone to `{channel, stored}`, and `start`
from one argument to two. `sw.js` caches nothing, so nothing in
sw-apl asked for this -- ordinary HTTP caching is enough. A browser
holding the old `worker.js` against the new bundle calls
`start(event.data)`, `stored` arrives as `undefined`, and
`passStringToWasm0` throws:

  TypeError: Cannot read properties of undefined (reading 'length')
      at passStringToWasm0 (apl_wasm.js:407)
      at start (apl_wasm.js:101)
      at self.onmessage (worker.js:16)

The page then shows a prompt that does nothing, which is the worst
of the available failures: a reader cannot tell it from a slow load
and has no reason to clear site data. A hard reload does not fix it
-- Chrome does not evict a module worker's script that way.

This is not local only. GitHub Pages serves the same three files
under the same names, so every returning visitor hits it the next
time the bundle changes.

Two halves, and the second is the one that matters:

- Make the pairing impossible. The page and the worker should
  fetch as one version, so a new bundle cannot meet an old worker.
  A query string the build stamps is the cheap answer; decide
  whether the stamp belongs in `just pages` or in the page.
- Make the failure loud. Whatever is left unpaired must say so on
  the paper, in the page's own voice, the way the isolation refusal
  already does -- not a prompt that silently ignores typing. The
  worker should refuse a message it does not recognise rather than
  handing it to `start`, and `start` should not be reachable with
  an argument it cannot use.

Check it the way the bug was found: load the page, rebuild the
bundle under it, and load it again in the same profile without
clearing anything. That sequence is the test, and it fails today.

Keep it to the browser. The CLI, the service and reg-rs are not
involved and must stay green.
