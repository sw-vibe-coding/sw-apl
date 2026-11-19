// Check pages/carriage.js, the 2741's carriage as events and how long
// each takes, with node alone. `just test` runs it.
import { strokes, delay, CHAR_MS } from "../pages/carriage.js";

let failed = 0;
const check = (what, got, want) => {
  const [g, w] = [JSON.stringify(got), JSON.stringify(want)];
  if (g !== w) {
    failed += 1;
    console.error(`FAIL ${what}: ${g}, wanted ${w}`);
  }
};
const kinds = (text, column = 0, parts = {}) =>
  strokes(text, column, parts).events.map((e) => e.kind);

check("each character a strike, a blank a space",
  strokes("A B", 0).events, [
    { kind: "strike", glyph: "A", column: 0, text: "A" },
    { kind: "space", column: 1, text: " " },
    { kind: "strike", glyph: "B", column: 2, text: "B" },
  ]);
check("and the carriage ends past them", strokes("A B", 0).column, 3);
check("a new line is a return and a feed",
  strokes("1\n", 4).events.slice(1), [
    { kind: "return", from: 5, text: "" },
    { kind: "feed", text: "\n" },
  ]);
check("and leaves the carriage at the margin", strokes("1\n", 4).column, 0);
check("printing goes on from where the carriage is",
  strokes("X", 7).events[0].column, 7);
check("an overstrike is its two parts with a backspace between",
  strokes("⍋", 0, { "⍋": ["∆", "|"] }).events, [
    { kind: "strike", glyph: "∆", column: 0, text: "" },
    { kind: "back", column: 0, text: "" },
    { kind: "strike", glyph: "|", column: 0, text: "⍋" },
  ]);
check("so the carriage moves one place for it",
  strokes("⍋", 0, { "⍋": ["∆", "|"] }).column, 1);
check("a backspace in the text backs the carriage up",
  kinds("AB\b_"), ["strike", "strike", "back", "strike"]);
check("but not past the margin", strokes("\b", 0).column, 0);
check("nothing to print is no events", strokes("", 3).events, []);

const still = () => 0.5;
check("a character takes the 2741's time, about 15 a second",
  delay({ kind: "strike" }, still), CHAR_MS);
check("give or take a little",
  [delay({ kind: "strike" }, () => 0), delay({ kind: "strike" }, () => 0.999)]
    .map(Math.round), [CHAR_MS - 15, CHAR_MS + 15]);
check("a space and a backspace a character's time",
  [delay({ kind: "space" }, still), delay({ kind: "back" }, still)], [CHAR_MS, CHAR_MS]);
check("a return is longer the further it comes back",
  delay({ kind: "return", from: 40 }, still) > delay({ kind: "return", from: 2 }, still), true);
check("a feed a character's time", delay({ kind: "feed" }, still), CHAR_MS);

if (failed) {
  console.error(`check-carriage: ${failed} failed`);
  process.exit(1);
}
console.log("check-carriage: ok");
