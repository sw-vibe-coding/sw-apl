// (U) '74: University of Maryland APL, run from punched cards as a
// batch job (docs/mode-u.md). Pick a deck, look through its cards --
// on the card itself, or standing in the 029 that punched it --
// repunch, add or take out cards on the 029's keyboard or the
// computer's, submit it, and read the printout that comes back on
// green-bar paper. The run is the CLI's own, compiled to WebAssembly
// (apl-u74-web); nothing here knows APL.

import { drawCard } from "./card029.js";
import { start, punch, back, move, release, feed, dup, remove, keyChar, fresh, strokes, cardStrokes } from "./punch029.js";
import { helpers } from "./helpers74.js";
import { keyboard } from "./keyboard029.js";
import { run, spellings } from "./job74.js";
import { wake, sound } from "./sound.js";

const at = new URL(import.meta.url).search;
const here = (path) => new URL(`${path}${at}`, import.meta.url).href;
const json = (path) => fetch(here(path)).then((r) => r.json());

const wasm = await import(here("./u74/apl_u74_web.js"));
await wasm.default({ module_or_path: here("./u74/apl_u74_web_bg.wasm") });
const [{ keys }, names] = await Promise.all([json("./keys029.json"), json("./glyph-names.json").catch(() => ({}))]);

const $ = (id) => document.getElementById(id);
const VIEW = "u74-view";
const SOUND = "u74-sound";
const kept = (title) => `u74-deck:${title}`;
const store = {
  get: (k) => { try { return localStorage.getItem(k); } catch { return null; } },
  set: (k, v) => { try { localStorage.setItem(k, v); } catch { /* private mode */ } },
  drop: (k) => { try { localStorage.removeItem(k); } catch { /* private mode */ } },
};

// The decks the WebAssembly carries: a title, then a card to a line.
const decks = wasm.decks().split("\f").map((text) => {
  const [title, ...cards] = text.split("\n");
  return { title, cards };
});
// And a deck begun from nothing, its control cards punched.
decks.push({ title: "New deck", cards: fresh().cards, fresh: true });
const punchable = (c) => wasm.card(c)[0] !== 0xffff;
const ACTS = ["REL", "FEED", "DUP", "SKIP", "SPACE", "NUMERIC", "ALPHA"];

let chosen = decks[0];
let deck = start(chosen.cards);
let numeric = false;
let model = null;
let busy = false;

// Open deck `d`, as last punched here if it was.
function open(d) {
  chosen = d;
  const saved = JSON.parse(store.get(kept(d.title)) ?? "null");
  if (Array.isArray(saved)) deck = start(saved);
  else deck = d.fresh ? fresh() : start(d.cards);
  $("printout").hidden = true;
  draw();
}

// Play `steps` (punch029's strokes), a column at a time with each
// one's sound; keys wait until it is done.
async function play(steps) {
  if (busy || !steps.length) return;
  busy = true;
  document.body.dataset.busy = "punching";
  for (const step of steps) {
    edit(step.deck, step.noise);
    await new Promise((done) => setTimeout(done, step.noise === "return" ? 180 : 28));
  }
  busy = false;
  delete document.body.dataset.busy;
}

// The deck changed: keep it, draw it, and sound what the 029 did.
function edit(next, noise) {
  if (next === deck) return;
  const ejected = next.card !== deck.card && noise === "return";
  deck = next;
  const same = deck.cards.join("\n") === chosen.cards.join("\n");
  if (same) store.drop(kept(chosen.title));
  else store.set(kept(chosen.title), JSON.stringify(deck.cards));
  if (ejected) eject();
  if (noise) noisy(noise);
  draw();
}

// The deck as a list, the card in the station, and where the punch is.
function draw() {
  listCards();
  const text = deck.cards[deck.card];
  const holes = wasm.card(text);
  drawCard($("card"), text, holes, deck.column);
  model?.show(text, holes, deck.column);
  $("where").textContent = `Card ${deck.card + 1} of ${deck.cards.length}`;
  $("column").textContent = `Column ${deck.column + 1}`;
  $("prev").disabled = deck.card === 0;
  $("next").disabled = deck.card === deck.cards.length - 1;
  $("edited").hidden = deck.cards.join("\n") === chosen.cards.join("\n");
  $("numeric").hidden = !numeric;
  document.querySelector('#keys [data-key="NUMERIC"]')?.classList.toggle("held", numeric);
}

// The deck, a card to a line, numbered as the printout numbers them;
// choosing one puts it in the station.
function listCards() {
  const list = $("cards");
  list.replaceChildren(...deck.cards.map((text, i) => {
    const li = document.createElement("li");
    const b = document.createElement("button");
    b.type = "button";
    b.textContent = text.padEnd(80).slice(0, 80);
    b.addEventListener("click", () => edit(move(deck, i - deck.card, 0)));
    li.append(b);
    li.toggleAttribute("aria-current", i === deck.card);
    return li;
  }));
  within(list, list.children[deck.card]);
}

// Scroll `list` alone, never the page, to show `item`: stepping
// through the deck leaves the arrows where the pointer is.
function within(list, item) {
  if (!item) return;
  const top = item.offsetTop;
  const bottom = top + item.offsetHeight;
  if (top < list.scrollTop) list.scrollTop = top;
  else if (bottom > list.scrollTop + list.clientHeight) list.scrollTop = bottom - list.clientHeight;
}

// The card released: a copy of it leaves the station to the left and
// up, over the next card being drawn in its place.
function eject() {
  const card = $("card");
  if (!card.offsetWidth) return;
  const copy = document.createElement("canvas");
  [copy.width, copy.height] = [card.width, card.height];
  copy.getContext("2d").drawImage(card, 0, 0);
  copy.className = "ejected";
  copy.addEventListener("animationend", () => copy.remove());
  card.after(copy);
}

// The 2741's sounds (sound.js) stand in for the 029's: a key's strike
// for a column punched, the carriage return's thud for a card ejected.
function noisy(kind) {
  if (!$("sound").checked) return;
  wake();
  sound({ kind, glyph: kind === "strike" ? deck.cards[deck.card][deck.column - 1] ?? "A" : "" });
}

// Punch `c`, sounding a strike or the space bar.
function type(c) {
  edit(punch(deck, c, punchable), c === " " ? "space" : "strike");
}

// A key of the 029 pressed.
function press(legend) {
  if (busy) return;
  const acts = {
    REL: () => edit(release(deck), "return"),
    FEED: () => edit(feed(deck), "return"),
    DUP: () => edit(dup(deck), "strike"),
    SKIP: () => edit(move(deck, 0, 1), "space"),
    SPACE: () => type(" "),
    NUMERIC: () => { numeric = !numeric; draw(); },
    ALPHA: () => { numeric = false; draw(); },
  };
  if (acts[legend]) return acts[legend]();
  const c = keyChar(legend, numeric);
  numeric = false;
  if (c) type(c);
  else draw();
}

// The computer's keyboard: a character punches; Enter releases;
// Backspace backs up; the arrows move the punch and the cards.
function typed(e) {
  if (busy || e.target.closest("select, input") || e.metaKey || e.ctrlKey || e.altKey) return;
  const moves = {
    Enter: () => edit(release(deck), "return"),
    Backspace: () => edit(back(deck), "back"),
    ArrowLeft: () => edit(move(deck, 0, -1)),
    ArrowRight: () => edit(move(deck, 0, 1)),
    ArrowUp: () => edit(move(deck, -1, -deck.column)),
    ArrowDown: () => edit(move(deck, 1, -deck.column)),
    PageUp: () => edit(move(deck, -1, -deck.column)),
    PageDown: () => edit(move(deck, 1, -deck.column)),
  };
  if (moves[e.key]) moves[e.key]();
  else if ([...e.key].length === 1) type(e.key);
  else return;
  e.preventDefault();
}

// Submit the deck: the title card, then the printout beside the deck.
async function submit() {
  await run(wasm.submit, deck.cards, {
    later: $("later"),
    paper: $("paper"),
    go: (card, column) => {
      edit(move(deck, card - deck.card, column - deck.column));
      $("card").scrollIntoView({ behavior: "smooth", block: "nearest" });
    },
  });
  document.body.classList.add("ran");
  $("printout").hidden = false;
}

// The 2D card or the 029 in 3D, remembered.
function views() {
  const button = $("view");
  let loading = null;
  const set = async (three) => {
    document.body.classList.toggle("model3d", three);
    $("model").hidden = !three;
    button.textContent = three ? "2D" : "3D";
    button.title = three ? "The card alone" : "The card in the 029 keypunch, in 3D";
    store.set(VIEW, three ? "3d" : "2d");
    if (!three) return;
    loading ??= import(here("./model029.js")).then((m) => m.model029($("model")));
    model = await loading;
    model.resize();
    draw();
  };
  button.addEventListener("click", () => set(!document.body.classList.contains("model3d")));
  set(store.get(VIEW) === "3d");
}

// The controls: the deck picker, the buttons, the keys, sound.
function controls() {
  const select = $("deck");
  decks.forEach((d, i) => select.append(new Option(d.title, String(i))));
  select.addEventListener("change", () => open(decks[Number(select.value)]));
  $("prev").addEventListener("click", () => edit(move(deck, -1, -deck.column)));
  $("next").addEventListener("click", () => edit(move(deck, 1, -deck.column)));
  $("feed").addEventListener("click", () => edit(feed(deck), "return"));
  $("remove").addEventListener("click", () => edit(remove(deck)));
  $("restore").addEventListener("click", () => { store.drop(kept(chosen.title)); open(chosen); });
  $("submit").addEventListener("click", submit);
  $("sound").checked = store.get(SOUND) !== "off";
  $("sound").addEventListener("change", () => {
    store.set(SOUND, $("sound").checked ? "on" : "off");
    if ($("sound").checked) { wake(); sound({ kind: "strike", glyph: "A" }); }
  });
  keyboard($("keys"), keys, press, punchable, ACTS);
  spellings($("spellings"), wasm.spellings(), names, (spelled) => play(strokes(deck, spelled, punchable)));
  helpers($("control"), $("idiom"), (cards) => play(cardStrokes(deck, cards, punchable)));
  document.addEventListener("keydown", typed);
}

controls();
views();
open(chosen);
