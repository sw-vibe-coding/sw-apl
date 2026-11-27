// (U) '74's helpers for punching a deck from nothing: a button for
// each control card and a menu of idioms (idioms74.js). Each punches
// its cards on cards of their own, a column at a time, as a keypunch
// operator would; `punchCards(cards)` does the punching.

import { CONTROL, IDIOMS } from "./idioms74.js";

// Fill `bar` with the control-card buttons and `menu` with the idioms.
export function helpers(bar, menu, punchCards) {
  bar.append(...CONTROL.map(({ name, cards }) => {
    const b = document.createElement("button");
    b.type = "button";
    b.textContent = name;
    b.dataset.control = name;
    b.title = `Punch ${cards[0].trimEnd()} on a card of its own`;
    b.addEventListener("click", () => punchCards(cards));
    return b;
  }));
  menu.append(new Option("Punch an idiom...", ""), ...IDIOMS.map((idiom, i) => {
    const o = new Option(idiom.name, String(i));
    o.title = idiom.cards.join(" / ");
    return o;
  }));
  menu.addEventListener("change", () => {
    const idiom = IDIOMS[Number(menu.value)];
    menu.value = "";
    if (idiom) punchCards(idiom.cards);
  });
}
