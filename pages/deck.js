// (U) '74: University of Maryland APL, run from punched cards as a
// batch job (docs/mode-u.md). Pick a deck, look through its cards one
// at a time -- on the card itself, or standing in the 029 that punched
// it -- submit it, and read the printout that comes back on green-bar
// paper. The run is the CLI's own, compiled to WebAssembly
// (apl-u74-web); nothing here knows APL.

import { drawCard } from "./card029.js";

const at = new URL(import.meta.url).search;
const here = (path) => new URL(`${path}${at}`, import.meta.url).href;

const wasm = await import(here("./u74/apl_u74_web.js"));
await wasm.default({ module_or_path: here("./u74/apl_u74_web_bg.wasm") });

const $ = (id) => document.getElementById(id);
const VIEW = "u74-view";

// The decks the WebAssembly carries: a title, then a card to a line.
const decks = Array.from({ length: wasm.decks() }, (_, i) => {
  const [title, ...cards] = wasm.deck(i).split("\n");
  return { title, cards };
});

let deck = decks[0];
let shown = 0;
let model = null;

// The deck picker, one entry a deck.
function picker() {
  const select = $("deck");
  decks.forEach((d, i) => select.append(new Option(d.title, String(i))));
  select.addEventListener("change", () => {
    deck = decks[Number(select.value)];
    listCards();
    show(0);
    $("printout").hidden = true;
  });
}

// The deck as a list, a card to a line, numbered as the printout
// numbers them; choosing one shows it.
function listCards() {
  const list = $("cards");
  list.replaceChildren(...deck.cards.map((text, i) => {
    const li = document.createElement("li");
    const b = document.createElement("button");
    b.type = "button";
    b.textContent = text.padEnd(80).slice(0, 80);
    b.addEventListener("click", () => show(i));
    li.append(b);
    return li;
  }));
}

// Show card `i`: on the card, in the model when it is open, and as
// the current card in the list.
function show(i) {
  shown = Math.max(0, Math.min(deck.cards.length - 1, i));
  const text = deck.cards[shown];
  const holes = wasm.card(text);
  drawCard($("card"), text, holes);
  model?.show(text, holes);
  $("where").textContent = `Card ${shown + 1} of ${deck.cards.length}`;
  $("prev").disabled = shown === 0;
  $("next").disabled = shown === deck.cards.length - 1;
  [...$("cards").children].forEach((li, n) => li.toggleAttribute("aria-current", n === shown));
  $("cards").children[shown]?.scrollIntoView({ block: "nearest" });
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
    try { localStorage.setItem(VIEW, three ? "3d" : "2d"); } catch { /* private mode */ }
    if (!three) return;
    loading ??= import(here("./model029.js")).then((m) => m.model029($("model")));
    model = await loading;
    model.resize();
    show(shown);
  };
  button.addEventListener("click", () => set(!document.body.classList.contains("model3d")));
  let kept = null;
  try { kept = localStorage.getItem(VIEW); } catch { /* private mode */ }
  set(kept === "3d");
}

// Submit the deck and print what comes back, a line to a stripe.
function submit() {
  const lines = wasm.submit(deck.cards.join("\n")).split("\n");
  const paper = $("paper");
  paper.replaceChildren(...lines.map((text) => {
    const div = document.createElement("div");
    div.textContent = text || " ";
    return div;
  }));
  $("printout").hidden = false;
  $("printout").scrollIntoView({ behavior: "smooth", block: "start" });
}

picker();
listCards();
views();
show(0);
$("prev").addEventListener("click", () => show(shown - 1));
$("next").addEventListener("click", () => show(shown + 1));
$("submit").addEventListener("click", submit);
document.addEventListener("keydown", (e) => {
  if (e.target.closest("select")) return;
  if (e.key === "ArrowLeft") show(shown - 1);
  if (e.key === "ArrowRight") show(shown + 1);
});
