// The 2741's carriage: what printing a piece of text does to it, as
// events a view can follow, and how long each takes.
//
// Each character is a strike of the type element at a column; a
// blank is a space, the carriage moving on without a strike; a new
// line is a return, the carriage back to the margin, and a feed, the
// paper up a line. A glyph the 2741 made by overstriking -- the grade
// ⍋ is ∆ over | -- is struck as its two parts with a backspace
// between, as the terminal printed it. Each event carries what it
// puts on the paper: an overstrike's first part puts nothing, and its
// last the whole glyph.

// A character's time: the 2741 printed about 15 a second.
export const CHAR_MS = 67;
// How much a character's time varies, either way.
const JITTER = 15;
// What a return takes for each column it carries back.
const RETURN_MS = 3;

// The events printing `text` makes, the carriage at `column`, and
// where the carriage ends. `parts` maps a glyph to the two characters
// struck to make it.
export function strokes(text, column, parts = {}) {
  const events = [];
  for (const ch of text) {
    if (ch === "\n") {
      events.push({ kind: "return", from: column, text: "" }, { kind: "feed", text: "\n" });
      column = 0;
    } else if (ch === "\b") {
      column = Math.max(0, column - 1);
      events.push({ kind: "back", column, text: "" });
    } else if (ch === " ") {
      events.push({ kind: "space", column, text: " " });
      column += 1;
    } else {
      events.push(...struck(ch, column, parts[ch]));
      column += 1;
    }
  }
  return { events, column };
}

// One glyph at `column`: a strike, or two with a backspace between.
function struck(glyph, column, pair) {
  if (!pair) return [{ kind: "strike", glyph, column, text: glyph }];
  return [
    { kind: "strike", glyph: pair[0], column, text: "" },
    { kind: "back", column, text: "" },
    { kind: "strike", glyph: pair[1], column, text: glyph },
  ];
}

// How long `event` takes, in milliseconds. A strike varies a little,
// by `random` (0 to 1), as a mechanism does; a return takes longer
// the further it carries back.
export function delay(event, random = Math.random) {
  switch (event.kind) {
    case "strike":
      return CHAR_MS + (random() * 2 - 1) * JITTER;
    case "return":
      return CHAR_MS + RETURN_MS * event.from;
    default:
      return CHAR_MS;
  }
}
