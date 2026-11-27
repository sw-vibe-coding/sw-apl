// Check pages/punch029.js, punching and editing a deck on the 029,
// with node alone. `just test` runs it.
import { start, punch, back, move, release, feed, dup, remove, keyChar, spot, fresh, strokes, cardStrokes } from "../pages/punch029.js";
import { IDIOMS, CONTROL } from "../pages/idioms74.js";

let failed = 0;
const check = (what, got, want) => {
  const [g, w] = [JSON.stringify(got), JSON.stringify(want)];
  if (g !== w) {
    failed += 1;
    console.error(`FAIL ${what}: ${g}, wanted ${w}`);
  }
};
const ok = (c) => /[A-Z0-9 &\-/.<(+|!$*);¬,%_>?:#@'="]/.test(c);
const typed = (s, text) => [...text].reduce((t, c) => punch(t, c, ok), s);

const s = start(["AB", "CD"]);
check("a deck starts on its first card, column one", [s.card, s.column], [0, 0]);
check("a punch overwrites the column and moves on",
  [punch(s, "X", ok).cards, punch(s, "X", ok).column], [["XB", "CD"], 1]);
check("punching past the end of the text pads it with blanks",
  typed({ ...s, column: 4 }, "Z").cards[0], "AB  Z");
check("a character the 029 cannot punch is refused", punch(s, "[", ok), s);
check("a lower-case letter punches as its capital", punch(s, "x", ok).cards[0], "XB");
check("nothing punches past column 80",
  typed({ ...s, column: 79 }, "QR").cards[0].length, 80);
check("and the column stays on 80", typed({ ...s, column: 79 }, "QR").column, 79);
check("trailing blanks are not kept", typed({ ...s, column: 1 }, " ").cards[0], "A");
check("backspace steps back a column and punches nothing",
  [back({ ...s, column: 2 }).column, back({ ...s, column: 2 }).cards], [1, ["AB", "CD"]]);
check("and stops at column one", back(s).column, 0);
check("left and right move the column", move(s, 0, 3).column, 3);
check("up and down move the card, kept in the deck", move(s, 5, 0).card, 1);
check("release goes to the next card, column one",
  [release({ ...s, column: 2 }).card, release({ ...s, column: 2 }).column], [1, 0]);
check("release from the last card feeds a blank one",
  release({ ...s, card: 1 }).cards, ["AB", "CD", ""]);
check("feed puts a blank card after this one and goes to it",
  [feed(s).cards, feed(s).card], [["AB", "", "CD"], 1]);
check("dup copies the column from the card before",
  dup({ ...s, card: 1, column: 1 }).cards, ["AB", "CB"]);
check("and moves on", dup({ ...s, card: 1, column: 1 }).column, 2);
check("dup on the first card punches nothing", dup(s), s);
check("remove takes the card out", remove(s).cards, ["CD"]);
check("removing the last card leaves the one before current",
  remove({ ...s, card: 1 }).card, 0);
check("the last card left is kept, blanked", remove(start(["A"])).cards, [""]);
check("a key punches its lower legend", keyChar("0\n/", false), "/");
check("or, with NUMERIC held, its upper one", keyChar("0\n/", true), "0");
check("a key with one legend punches it either way", keyChar("A", true), "A");
check("the minus and dash legends punch an 11", [keyChar("−\n-", true), keyChar("—\nW", true)], ["-", "-"]);
check("a function key punches nothing", keyChar("DUP", false), null);
check("the 0-8-2 legend punches nothing here", keyChar("0–8–2\nT", true), null);
check("the printout's places are card and column",
  [spot("CARD 4 COLUMN 11"), spot("CARD 8"), spot("0012  HALF 9"), spot("4.5")],
  [{ card: 3, column: 10 }, { card: 7, column: 0 }, { card: 11, column: 0 }, null]);

const last = (steps) => steps[steps.length - 1].deck;
check("a new deck is its control cards, the punch on the blank card between",
  [fresh().cards, fresh().card, fresh().column],
  [["@RUN   USER,ACCOUNT,PROJECT,1,20", "@APL,I", "", "@FIN"], 2, 0]);
check("text punches a column at a time, each with its sound",
  strokes(s, "A B", ok).map((t) => [t.deck.column, t.noise]), [[1, "strike"], [2, "space"], [3, "strike"]]);
check("and ends punched", last(strokes(s, "Q", ok)).cards[0], "QB");
check("what the 029 cannot punch is left out", strokes(s, "[", ok), []);
check("a card punched into a blank card stays on it",
  last(cardStrokes(fresh(), ["X $AL 1"], ok)).cards, ["@RUN   USER,ACCOUNT,PROJECT,1,20", "@APL,I", "X $AL 1", "@FIN"]);
check("after a card with something on it, a card is fed first, with the thud",
  cardStrokes(s, ["Z"], ok).map((t) => t.noise), ["return", "strike"]);
check("so the card lands after the one in the station",
  last(cardStrokes(s, ["Z", "Y"], ok)).cards, ["AB", "Z", "Y", "CD"]);
check("the control cards are @RUN, @APL and @FIN",
  CONTROL.map((c) => c.cards[0].split(/[ ,]/)[0]), ["@RUN", "@APL", "@FIN"]);
check("every idiom has a name and cards the 029 can punch",
  IDIOMS.filter((i) => !i.name || !i.cards.length || !i.cards.every((c) => [...c].every(ok))).map((i) => i.name), []);

if (failed) {
  console.error(`check-punch: ${failed} failed`);
  process.exit(1);
}
console.log("check-punch: all passed");
