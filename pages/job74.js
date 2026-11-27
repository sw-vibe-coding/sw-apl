// (U) '74's batch job: the deck goes in, a title card says the
// operator has it, and the printout comes back on green-bar paper.
// A line naming a card -- an error's CARD n COLUMN c, or a card as
// listed -- goes to that card when chosen, to repunch it.

import { spot } from "./punch029.js";

const quick = matchMedia("(prefers-reduced-motion: reduce)").matches;

// Run `cards` with `submit` (the WebAssembly's), showing `later`, the
// title card, while it is out; print the result in `paper`. Choosing
// a line that names a card calls `go(card, column)`.
export async function run(submit, cards, { later, paper, go }) {
  later.hidden = false;
  const lines = submit(cards.join("\n")).split("\n");
  await new Promise((done) => setTimeout(done, quick ? 300 : 1600));
  paper.replaceChildren(...lines.map((text) => line(text, go)));
  later.hidden = true;
}

// One printout line, a button when it names a card.
function line(text, go) {
  const at = spot(text);
  const el = document.createElement(at ? "button" : "div");
  el.textContent = text || " ";
  if (!at) return el;
  el.type = "button";
  el.className = "spot";
  el.title = `Card ${at.card + 1}, column ${at.column + 1}: repunch it`;
  el.addEventListener("click", () => go(at.card, at.column));
  return el;
}

// The $ spellings, a glyph to each, in `host`: `text` is a line to a
// glyph ("$RH ⍴"), `names` the glyphs' names. Choosing one calls
// `chosen(spelling)`, to punch it.
export function spellings(host, text, names, chosen) {
  host.replaceChildren(...text.split("\n").filter(Boolean).map((row) => {
    const [spelled, glyph] = row.split(" ");
    const b = document.createElement("button");
    b.type = "button";
    b.dataset.spelled = spelled;
    b.title = `Punch ${spelled}${names[glyph] ? `, ${names[glyph]}` : ""}`;
    const [dt, dd] = [document.createElement("dt"), document.createElement("dd")];
    dt.textContent = spelled;
    dd.textContent = glyph;
    b.append(dt, dd);
    b.addEventListener("click", () => chosen(spelled));
    return b;
  }));
}
