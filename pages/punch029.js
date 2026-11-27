// (U) '74's keypunch: a deck being punched and edited on the 029, as
// plain data. The page draws it and hands it keys; nothing here
// touches the page, so node checks it (scripts/check-punch.mjs).
//
// A deck is { cards, card, column }: the cards as text, trailing
// blanks dropped; the card in the punch station; the column under
// the punch, counted from 0.

const COLUMNS = 80;

// A deck at its first card, column one.
export function start(cards) {
  return { cards: [...cards], card: 0, column: 0 };
}

// Punch `c` at the column and move on; `punchable` says whether the
// 029 has a code for it. A letter punches as its capital.
export function punch(deck, c, punchable) {
  const ch = c.toUpperCase();
  if (ch.length !== 1 || !punchable(ch)) return deck;
  const text = deck.cards[deck.card].padEnd(COLUMNS);
  const cards = [...deck.cards];
  cards[deck.card] = (text.slice(0, deck.column) + ch + text.slice(deck.column + 1)).trimEnd();
  return { ...deck, cards, column: Math.min(COLUMNS - 1, deck.column + 1) };
}

// Back a column, punching nothing.
export function back(deck) {
  return { ...deck, column: Math.max(0, deck.column - 1) };
}

// Move `cards` cards and `columns` columns, staying in the deck.
export function move(deck, cards, columns) {
  const card = Math.max(0, Math.min(deck.cards.length - 1, deck.card + cards));
  const column = Math.max(0, Math.min(COLUMNS - 1, deck.column + columns));
  return { ...deck, card, column };
}

// Release the card and feed the next, column one; past the last
// card, a blank one.
export function release(deck) {
  const cards = deck.card === deck.cards.length - 1 ? [...deck.cards, ""] : deck.cards;
  return { ...deck, cards, card: deck.card + 1, column: 0 };
}

// Feed a blank card in after this one and go to it.
export function feed(deck) {
  const cards = [...deck.cards];
  cards.splice(deck.card + 1, 0, "");
  return { ...deck, cards, card: deck.card + 1, column: 0 };
}

// Duplicate the column of the card before into this one.
export function dup(deck) {
  if (deck.card === 0) return deck;
  const c = deck.cards[deck.card - 1].padEnd(COLUMNS)[deck.column];
  return punch(deck, c, () => true);
}

// Take the card out of the deck. A deck keeps one card, blank.
export function remove(deck) {
  const cards = deck.cards.filter((_, i) => i !== deck.card);
  if (!cards.length) return { ...deck, cards: [""], card: 0, column: 0 };
  return { ...deck, cards, card: Math.min(deck.card, cards.length - 1), column: 0 };
}

// What a key of the 029 keyboard (legend "upper\nlower") punches:
// its lower legend, or with NUMERIC held its upper one. Keys that
// punch no character of ours (the function keys, 0-8-2) give null.
const SAME = { "−": "-", "—": "-" };
export function keyChar(legend, numeric) {
  const [upper, lower = upper] = legend.split("\n");
  const c = SAME[numeric ? upper : lower] ?? (numeric ? upper : lower);
  return c.length === 1 ? c : null;
}

// The card and column a printout line points at: an error's
// "CARD n COLUMN c", "CARD n", or a card as listed, "nnnn  text".
export function spot(line) {
  const error = /^CARD (\d+)(?: COLUMN (\d+))?$/.exec(line.trim());
  if (error) return { card: Number(error[1]) - 1, column: Math.max(0, Number(error[2] ?? 1) - 1) };
  const listed = /^(\d{4}) {2}/.exec(line);
  return listed ? { card: Number(listed[1]) - 1, column: 0 } : null;
}

// A deck begun from nothing: its control cards, a blank card between
// them for the APL, and the punch on that card.
const NEW_DECK = ["@RUN   USER,ACCOUNT,PROJECT,1,20", "@APL,I", "", "@FIN"];
export function fresh() {
  return { ...start(NEW_DECK), card: 2 };
}

// Punching `text` at the punch, a column at a time: each step the deck
// after it and the sound it makes. What the 029 cannot punch is left
// out.
export function strokes(deck, text, punchable) {
  const steps = [];
  for (const c of text) {
    const next = punch(steps.at(-1)?.deck ?? deck, c, punchable);
    if (next === (steps.at(-1)?.deck ?? deck)) continue;
    steps.push({ deck: next, noise: c === " " ? "space" : "strike" });
  }
  return steps;
}

// Punching whole cards, each on a card of its own: into the card in
// the station if it is blank, else into one fed in after it.
export function cardStrokes(deck, cards, punchable) {
  const steps = [];
  for (const text of cards) {
    let at = steps.at(-1)?.deck ?? deck;
    if (at.cards[at.card] !== "") {
      at = feed(at);
      steps.push({ deck: at, noise: "return" });
    }
    steps.push(...strokes({ ...at, column: 0 }, text, punchable));
  }
  return steps;
}
