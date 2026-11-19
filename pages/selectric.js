// (A) '70's terminal, the 2741: what the session sends is printed a
// character at a time, at the 2741's speed, with its sounds. Sound and
// Fast are buttons in the bar that say whether they are on, each
// remembered: sound on and paced printing until chosen otherwise.
//
// A browser plays nothing until the reader has done something on the
// page, so each visit opens on a card that starts the terminal: with
// sound, the default and what Enter chooses, or silently.

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
  const fast = toggle(bar, "Fast", FAST, "Print at once rather than at the 2741's speed", false);
  const loud = toggle(bar, "Sound", SOUND, "The 2741's sounds as it prints and as you type", true);
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
  // Turned on, it says so at once: a key's strike.
  loud.button.addEventListener("click", () => {
    if (!loud.on()) return;
    wake();
    sound({ kind: "strike", glyph: "A" });
  });
  // The 2741 printed as it was typed: each key sounds as it goes down.
  const line = document.getElementById("line");
  addEventListener("keydown", (e) => {
    if (!loud.on() || e.metaKey || e.ctrlKey || line.classList.contains("waiting")) return;
    const kind = { Enter: "return", " ": "space", Backspace: "back" }[e.key]
      ?? ([...e.key].length === 1 ? "strike" : null);
    if (kind) sound({ kind, glyph: e.key });
  });
  start(loud);
  return printing;
}

// The card each visit opens on. Either button, or Enter, starts the
// terminal; with sound, sound is woken by that same gesture.
function start(loud) {
  const card = document.createElement("div");
  card.id = "start";
  card.setAttribute("role", "dialog");
  card.setAttribute("aria-label", "Start the 2741");
  const box = document.createElement("div");
  box.className = "card";
  const title = document.createElement("p");
  title.textContent = "APL\\360 on the IBM 2741";
  const go = (withSound) => {
    loud.set(withSound);
    if (withSound) wake();
    card.remove();
    removeEventListener("keydown", enter, true);
  };
  const enter = (e) => {
    if (e.key !== "Enter" && e.key !== "Escape") return;
    e.preventDefault();
    e.stopImmediatePropagation();
    go(e.key === "Enter");
  };
  const button = (label, withSound) => {
    const b = document.createElement("button");
    b.type = "button";
    b.textContent = label;
    b.addEventListener("click", () => go(withSound));
    return b;
  };
  const loudly = button("Start with sound", true);
  box.append(title, loudly, button("Start silently", false));
  card.append(box);
  document.body.append(card);
  addEventListener("keydown", enter, true);
  loudly.focus();
}

// A button that turns something on and off, kept under `key`, `first`
// until it has been chosen; its label says which it is.
function toggle(bar, label, key, title, first) {
  const button = document.createElement("button");
  button.type = "button";
  button.title = title;
  const set = (on) => {
    button.setAttribute("aria-pressed", String(on));
    button.textContent = `${label} ${on ? "on" : "off"}`;
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
  set(kept === null ? first : kept === "on");
  button.addEventListener("click", () => set(!on()));
  bar.append(button);
  return { button, on, set };
}
