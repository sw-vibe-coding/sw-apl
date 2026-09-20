// The board a reader taps.
//
// A touch screen has no keyboard to intercept, so there has to be one
// on the page. This draws it from `keymap.json` -- the same file
// `apl-keyboard` compiles in, copied into the bundle by `just pages`
// -- so what a key sends here is what the same key sends at a
// terminal, and there is no second copy of the map.
//
// It is laid out like the 2741 on purpose. A palette of glyphs would
// be easier to build and would teach nothing; a board teaches where
// the key is, so a reader who taps `⍴` today can type Shift-R
// tomorrow.
//
// The redistributed keyboard picture is not this. That is an Inkscape
// drawing whose keys are not addressable and whose licence travels
// with it; it is reference, shown on request, and this is ours.

// The ASCII each key is painted with, unshifted, row by row. This is
// where a US keyboard puts them -- presentation, not the map.
const ROWS = [
  "1234567890-=",
  "QWERTYUIOP[]",
  "ASDFGHJKL;'",
  "ZXCVBNM,./",
];

// What holding shift makes of a key. Letters shift to themselves
// here, because a 2741's unshifted letters are already capitals.
const SHIFTED = {
  1: "!", 2: "@", 3: "#", 4: "$", 5: "%", 6: "^", 7: "&", 8: "*",
  9: "(", 0: ")", "-": "_", "=": "+", "[": "{", "]": "}",
  ";": ":", "'": '"', ",": "<", ".": ">", "/": "?",
};

const shifted = (cap) => SHIFTED[cap] ?? cap;

// What sw-apl calls a glyph, for a screen reader and for a hover. The
// board is unusable without it: "⍴" read aloud is nothing.
let named = {};

export const name = (glyph) => named[glyph] ?? glyph;

// Build the board once, from the map. `tap` is called with the text a
// key sends, or with one of the words the control keys stand for.
export async function build(root, stamped, tap) {
  const map = await fetch(stamped("keymap.json")).then((r) => r.json());
  named = await fetch(stamped("glyph-names.json"))
    .then((r) => r.json())
    .catch(() => ({}));

  for (const row of ROWS) {
    const line = document.createElement("div");
    line.className = "row";
    for (const cap of row) {
      line.append(key(map[shifted(cap)] ?? cap, cap, tap));
    }
    root.append(line);
  }
  root.append(controls(tap));
  return root;
}

// One key, carrying both its faces: the glyph it sends and the ASCII
// it is painted with. Which one it sends is the layer, which the
// board's own class decides, so a layer change is one attribute and
// not a redraw.
function key(glyph, cap, tap) {
  const button = document.createElement("button");
  button.type = "button";
  button.className = "key";
  button.dataset.glyph = glyph;
  button.dataset.plain = cap;
  button.innerHTML =
    `<span class="g"></span><span class="p"></span>`;
  button.querySelector(".g").textContent = glyph;
  button.querySelector(".p").textContent = cap === glyph ? "" : cap;
  const sends = () =>
    button.closest(".board").classList.contains("plain") ? cap : glyph;
  button.addEventListener("click", () => tap(sends()));
  // Named for whoever is not looking at it.
  button.title = `${glyph} ${name(glyph)}`;
  button.setAttribute("aria-label", `${name(glyph)}, or ${cap}`);
  return button;
}

// Space, the two deletions, the overstrike key and return -- and the
// layer toggle, without which a touch screen cannot reach a letter.
function controls(tap) {
  const row = document.createElement("div");
  row.className = "row controls";
  // `sends` of null is a key that types nothing: the layer toggle
  // changes which face the board sends and must never reach the line.
  const add = (label, aria, sends, cls = "") => {
    const button = document.createElement("button");
    button.type = "button";
    button.className = `key ${cls}`;
    button.textContent = label;
    button.setAttribute("aria-label", aria);
    if (sends !== null) button.addEventListener("click", () => tap(sends));
    row.append(button);
    return button;
  };
  const layer = add("ABC", "type letters and digits instead of glyphs", null, "wide");
  layer.addEventListener("click", () => {
    const plain = row.closest(".board").classList.toggle("plain");
    layer.textContent = plain ? "APL" : "ABC";
    layer.setAttribute(
      "aria-label",
      plain ? "type APL glyphs instead of letters" : "type letters and digits instead of glyphs",
    );
  });
  add("⎵", "space", " ", "space");
  add("⌫", "backspace", "backspace");
  add("○*", "overstrike the last glyph", "overstrike");
  add("⏎", "return, send the line", "return", "wide");
  return row;
}
