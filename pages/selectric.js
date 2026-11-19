// (A) '70's terminal, the 2741: what the session sends is printed a
// character at a time, at the 2741's speed, with its sounds if the
// reader wants them. Sound and Fast are buttons in the bar, each
// remembered: Sound off and paced printing until chosen otherwise.

// At the version this module was fetched at, as apl.js fetches
// everything, so a cache cannot pair it with older ones.
const at = new URL(import.meta.url).search;
const { printer } = await import(`./printer.js${at}`);
const { sound, wake } = await import(`./sound.js${at}`);

const SOUND = "apl-2741-sound";
const FAST = "apl-2741-fast";

// The printer for the page: `ink` puts what an event prints on the
// paper, `parts` is the overstrike table, `bar` holds the buttons, and
// `follow` is told of each event once it is on the paper.
export function selectric(bar, ink, parts, follow = () => {}) {
  const fast = toggle(bar, "Fast", FAST, "Print at once rather than at the 2741's speed");
  const loud = toggle(bar, "Sound", SOUND, "The 2741's sounds as it prints");
  const printing = printer({
    show: (event) => {
      if (event.text) ink(event.text);
      follow(event);
    },
    sound: (event) => loud.on() && sound(event),
    fast: fast.on,
    parts,
  });
  fast.button.addEventListener("click", () => fast.on() && printing.flush());
  // Sound kept on from an earlier visit wakes with the first touch.
  addEventListener("pointerdown", () => loud.on() && wake(), { once: true });
  addEventListener("keydown", () => loud.on() && wake(), { once: true });
  loud.button.addEventListener("click", () => loud.on() && wake());
  // The 2741 printed as it was typed: each key sounds as it goes down.
  const line = document.getElementById("line");
  addEventListener("keydown", (e) => {
    if (!loud.on() || e.metaKey || e.ctrlKey || line.classList.contains("waiting")) return;
    const kind = { Enter: "return", " ": "space", Backspace: "back" }[e.key]
      ?? ([...e.key].length === 1 ? "strike" : null);
    if (kind) sound({ kind, glyph: e.key });
  });
  return printing;
}

// A button that turns something on and off, kept under `key`.
function toggle(bar, label, key, title) {
  const button = document.createElement("button");
  button.type = "button";
  button.textContent = label;
  button.title = title;
  const set = (on) => {
    button.setAttribute("aria-pressed", String(on));
    try {
      localStorage.setItem(key, on ? "on" : "off");
    } catch {
      // A browser that keeps nothing forgets it next time; this visit holds it.
    }
  };
  const on = () => button.getAttribute("aria-pressed") === "true";
  let kept = null;
  try {
    kept = localStorage.getItem(key);
  } catch {
    kept = null;
  }
  set(kept === "on");
  button.addEventListener("click", () => set(!on()));
  bar.append(button);
  return { button, on };
}
