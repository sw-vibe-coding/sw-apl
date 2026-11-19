// Check pages/printer.js, the paced printer, with node alone and a
// clock of its own: nothing waits. `just test` runs it.
import { printer } from "../pages/printer.js";
import { CHAR_MS } from "../pages/carriage.js";

let failed = 0;
const check = (what, got, want) => {
  const [g, w] = [JSON.stringify(got), JSON.stringify(want)];
  if (g !== w) {
    failed += 1;
    console.error(`FAIL ${what}: ${g}, wanted ${w}`);
  }
};

// A clock that runs only when told: `tick()` runs the next wait.
function clock() {
  const waits = [];
  let spent = 0;
  return {
    wait: (f, ms) => {
      spent += ms;
      waits.push({ f, ms });
    },
    tick: () => waits.shift()?.f(),
    pending: () => waits.length,
    total: () => spent,
  };
}
const make = (options = {}) => {
  const c = clock();
  const shown = [], heard = [];
  const p = printer({
    show: (e) => shown.push(e.text),
    sound: (e) => heard.push(e.kind),
    wait: c.wait,
    random: () => 0.5,
    ...options,
  });
  return { c, p, shown, heard };
};

{
  const { c, p, shown } = make();
  p.write("AB\n");
  check("the first character prints at once", shown, ["A"]);
  c.tick();
  check("the next after a character's time", shown, ["A", "B"]);
  while (c.pending()) c.tick();
  check("then the return and the feed", shown.join(""), "AB\n");
  check("and it is idle", p.busy(), false);
}
{
  const { c, p, heard } = make();
  p.write("A \n");
  while (c.pending()) c.tick();
  check("each event is heard", heard, ["strike", "space", "return", "feed"]);
}
{
  const { c, p, shown } = make();
  let ready = 0;
  p.write("XYZ");
  p.idle(() => (ready += 1));
  check("a wait for idle waits while printing", ready, 0);
  while (c.pending()) c.tick();
  check("and runs when it is done", [ready, shown.join("")], [1, "XYZ"]);
  p.idle(() => (ready += 1));
  check("and at once when nothing is printing", ready, 2);
}
{
  const { c, p, shown, heard } = make();
  p.write("HELLO");
  p.flush();
  check("flush prints the rest at once", shown.join(""), "HELLO");
  check("silently", heard, ["strike"]);
  c.tick();
  check("and it is idle once the stroke under way is done", p.busy(), false);
}
{
  const { c, p, shown, heard } = make({ fast: () => true });
  p.write("FAST\n");
  check("fast prints at once", [shown.join(""), c.pending()], ["FAST\n", 0]);
  check("and silently", heard, []);
}
{
  const { c, p, shown } = make();
  p.write("AB", true);
  check("what was typed is printed at once", [shown.join(""), c.pending()], ["AB", 0]);
  p.write("C");
  check("and the carriage goes on from it", p.column(), 3);
}
{
  const { c, p } = make();
  p.write("ABC");
  while (c.pending()) c.tick();
  check("paced, a line takes the 2741's time", Math.round(c.total()), 3 * CHAR_MS);
}

{
  const { c, p, shown, heard } = make();
  p.prompt("      ");
  while (c.pending()) c.tick();
  check("a prompt is heard, a space at a time", heard, Array(6).fill("space"));
  check("but puts nothing on the paper", shown.join(""), "");
  p.write("      1+1\n", true);
  check("and the typed line goes on from where the carriage began", p.column(), 0);
}

if (failed) {
  console.error(`check-printer: ${failed} failed`);
  process.exit(1);
}
console.log("check-printer: ok");
