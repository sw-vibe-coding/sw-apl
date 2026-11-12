// Check pages/lines.js, the demo's split of a paste into lines, with
// node alone: no browser, no packages. `just test` runs it.
import { pasteLines } from "../pages/lines.js";

let failed = 0;
const check = (what, got, want) => {
  const [g, w] = [JSON.stringify(got), JSON.stringify(want)];
  if (g !== w) {
    failed += 1;
    console.error(`FAIL ${what}: ${g}, wanted ${w}`);
  }
};

check("one line stays in the box",
  pasteLines("", "", "1+2"), { lines: [], rest: "1+2", at: 3 });
check("each complete line runs; the last stays",
  pasteLines("", "", "X←1\nX+1\nX"), { lines: ["X←1", "X+1"], rest: "X", at: 1 });
check("a paste ending in a line break leaves the box empty",
  pasteLines("", "", "A\nB\n"), { lines: ["A", "B"], rest: "", at: 0 });
check("CR LF, CR and LF all end a line",
  pasteLines("", "", "A\r\nB\rC\nD"), { lines: ["A", "B", "C"], rest: "D", at: 1 });
check("what was typed before the caret joins the first line",
  pasteLines("2×", "", "3\n4"), { lines: ["2×3"], rest: "4", at: 1 });
check("what was after the caret follows the last",
  pasteLines("", "+1", "5\n6"), { lines: ["5"], rest: "6+1", at: 1 });
check("an empty line is a line, as Return on an empty box is",
  pasteLines("", "", ")DEF F\n1\n\nF"), { lines: [")DEF F", "1", ""], rest: "F", at: 1 });

if (failed) {
  console.error(`check-lines: ${failed} failed`);
  process.exit(1);
}
console.log("check-lines: ok");
