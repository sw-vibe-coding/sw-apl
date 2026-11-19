// The 2741 printing what the session sends, a character at a time.
//
// Text written goes onto a queue of the carriage's events
// (carriage.js); each is shown, and heard, then the next follows
// after the time it takes. Nothing waits on the printing but the
// keyboard: the page asks `idle` before it takes a line, as the
// 2741's keyboard stayed locked while the terminal printed.

// At the version this module was fetched at, as apl.js fetches
// everything, so a cache cannot pair it with an older carriage.
const { strokes, delay } = await import(`./carriage.js${new URL(import.meta.url).search}`);

// A printer. `show` puts an event on the views, `sound` plays it,
// `fast` says whether to print at once instead of paced, `parts` is
// the overstrike table, and `wait` and `random` are the clock and the
// jitter, which a test replaces.
export function printer({
  show, sound = () => {}, fast = () => false, parts = {},
  wait = setTimeout, random = Math.random,
}) {
  const queue = [];
  const waiting = [];
  let column = 0;
  let busy = false;
  // Show the next event, then wait its time; idle when none is left.
  const next = () => {
    const event = queue.shift();
    if (!event) {
      busy = false;
      for (const f of waiting.splice(0)) f();
      return;
    }
    busy = true;
    show(event);
    sound(event);
    wait(next, delay(event, random));
  };
  // Every event still queued, shown now and not heard.
  const drain = () => {
    while (queue.length) show(queue.shift());
  };
  return {
    // Print `text`; `now` when it was typed, and so is printed already.
    write(text, now = false) {
      const done = strokes(text, column, parts);
      column = done.column;
      queue.push(...done.events);
      if (now || fast()) drain();
      if (!busy) next();
    },
    // Everything queued, at once: ATTN, or Fast chosen part way.
    flush: drain,
    // Run `f` once nothing is left to print.
    idle(f) {
      if (busy) waiting.push(f);
      else f();
    },
    busy: () => busy,
    column: () => column,
  };
}
