// The board a reader taps: the IBM 2741's own keyboard.
//
// The board is the redistributed 2741 picture with a layer of hit
// regions laid over it -- the modern image map. The picture is CC
// BY-SA and is referenced exactly as redistributed, never edited: it
// goes in as an SVG <image>, and everything interactive is drawn over
// it in the same coordinate system, so the keys and their hit regions
// scale together at any size. Where each key is comes from
// board-keys.json, measured out of the picture at build time; what
// each key sends comes from keymap.json, the same file apl-keyboard
// compiles in, so a key sends here what it sends at a terminal.
//
// Four modes. APL and ABC are the two faces every 2741 key carries --
// the picture shows both on each key -- so they are the two shift
// states of one picture, not two layouts. Commands and Idioms were
// never on a 2741; they are the board's own, from board.json, where a
// test runs every entry.

const NS = "http://www.w3.org/2000/svg";
const PICTURE = "redistributed/apl-keyboard/APL-keybd2.svg";

// Room to the left of the picture for ATTN, which is wider than the
// key it replaces so its label reads at a normal size.
const PAD = 30;

// What holding shift makes of a US key. keymap.json is keyed by what
// the keyboard sends, so a key's shifted face is the map's entry for
// its shifted character.
const SHIFTED = {
  1: "!", 2: "@", 3: "#", 4: "$", 5: "%", 6: "^", 7: "&", 8: "*",
  9: "(", 0: ")", "-": "_", "=": "+", "[": "{", "]": "}",
  ";": ":", "'": '"', ",": "<", ".": ">", "/": "?",
};

const MODES = [
  ["apl", "APL", "the glyph each key carries"],
  ["abc", "ABC", "the letter or digit each key carries"],
  ["commands", "Commands", "system commands"],
  ["idioms", "Idioms", "short APL expressions"],
];

// What sw-apl calls a glyph, for a screen reader and for a hover.
let named = {};

export const name = (glyph) => named[glyph] ?? glyph;

// Both faces of a key. A 2741's unshifted letters are capitals and its
// shifted ones are glyphs; every other key has a glyph or a character
// on each face, and the map says which.
function faces(cap, map) {
  if (/^[A-Z]$/.test(cap)) return { normal: cap, shifted: map[cap] ?? cap };
  const up = SHIFTED[cap] ?? cap;
  return { normal: map[cap] ?? cap, shifted: map[up] ?? up };
}

// Remembered across visits, and never fatal when storage is refused.
const remembered = (key, fallback) => {
  try {
    return localStorage.getItem(key) ?? fallback;
  } catch {
    return fallback;
  }
};
const remember = (key, value) => {
  try {
    localStorage.setItem(key, value);
  } catch { /* private mode: it just will not be remembered */ }
};

const SIZE = "apl-board-size";
const EDGE = "apl-board-edge";
const MODE = "apl-board-mode";
const KEYS = { min: 1.5, max: 3.4, step: 0.35, default: 2.4 };

// Build the board once. `tap` is given the text a key sends, or one of
// the words a control key stands for; `attn` stops a run, and is apart
// from `tap` because a tap is ignored while the session is busy and
// busy is exactly when ATTN is wanted.
export async function build(root, stamped, tap, attn) {
  const load = (file) => fetch(stamped(file)).then((r) => r.json());
  const [map, layout, lists] = await Promise.all([
    load("keymap.json"), load("board-keys.json"), load("board.json"),
  ]);
  named = await load("glyph-names.json").catch(() => ({}));

  root.append(modes(root));
  root.append(picture(layout, map, stamped, tap, attn));
  root.append(list("commands", lists.commands, tap));
  root.append(list("idioms", lists.idioms, tap));
  root.append(controls(tap));
  root.append(placing(root));
  root.append(credit());
  size(root, Number(remembered(SIZE, KEYS.default)));
  place(root, remembered(EDGE, "bottom"));
  mode(root, remembered(MODE, "apl"));
  return root;
}

// The mode switch: one segment per mode, the current one marked, so a
// reader can see where they are going rather than cycle to find out.
function modes(root) {
  const bar = document.createElement("div");
  bar.className = "modes";
  bar.setAttribute("role", "tablist");
  bar.setAttribute("aria-label", "keyboard mode");
  for (const [id, label, about] of MODES) {
    const tab = document.createElement("button");
    tab.type = "button";
    tab.dataset.mode = id;
    tab.textContent = label;
    tab.title = about;
    tab.setAttribute("role", "tab");
    tab.addEventListener("click", () => mode(root, id));
    bar.append(tab);
  }
  return bar;
}

// Switch mode: which panel shows, which tab is marked, and what each
// key says it sends.
function mode(root, id) {
  const known = MODES.some(([m]) => m === id) ? id : "apl";
  root.dataset.mode = known;
  const keyboard = known === "apl" || known === "abc";
  root.querySelector(".picture").hidden = !keyboard;
  for (const panel of root.querySelectorAll(".list")) {
    panel.hidden = panel.dataset.kind !== known;
  }
  for (const tab of root.querySelectorAll(".modes [role=tab]")) {
    tab.setAttribute("aria-selected", String(tab.dataset.mode === known));
  }
  const face = known === "abc" ? "normal" : "shifted";
  for (const hit of root.querySelectorAll(".hit")) {
    hit.setAttribute("aria-label", name(hit.dataset[face]));
  }
  remember(MODE, known);
}

// The 2741 itself: the picture, untouched, and a hit region over each
// key in the picture's own units.
function picture(layout, map, stamped, tap, attn) {
  const [w, h] = layout.size;
  const holder = document.createElement("div");
  holder.className = "picture";
  const svg = document.createElementNS(NS, "svg");
  svg.setAttribute("viewBox", `${-PAD} 0 ${w + PAD} ${h}`);
  svg.setAttribute("role", "group");
  svg.setAttribute("aria-label", "IBM 2741 APL keyboard");

  // A plate under the picture: its keys are drawn dark on nothing, and
  // would vanish on a dark page without one.
  svg.append(shape("rect", { class: "plate", x: -PAD, y: 0, width: w + PAD, height: h, rx: 4 }));
  svg.append(shape("image", { href: stamped(PICTURE), x: 0, y: 0, width: w, height: h }));

  const root = () => holder.closest(".board");
  for (const { cap, box } of layout.keys) {
    if (cap === "ATTN") {
      svg.append(attention(box, attn));
      continue;
    }
    const [x, y, kw, kh] = box;
    const hit = shape("rect", { class: "key hit", x, y, width: kw, height: kh, rx: 2 });
    hit.setAttribute("role", "button");
    if (cap === "BACKSPACE") {
      // On a 2741 backspace moved the carriage back so the next key
      // struck over the last glyph: it was the overstrike key. On the
      // board it is again. Erasing is the Erase key below.
      hit.dataset.plain = "BACKSPACE";
      hit.dataset.normal = hit.dataset.shifted = "backspace";
      hit.setAttribute("aria-label", "backspace: strike the next glyph over the last");
      hit.addEventListener("click", () => press(hit, () => tap("overstrike")));
    } else {
      const { normal, shifted } = faces(cap, map);
      Object.assign(hit.dataset, { plain: cap, normal, shifted });
      hit.addEventListener("click", () => {
        const face = root().dataset.mode === "abc" ? normal : shifted;
        press(hit, () => tap(face));
      });
    }
    svg.append(hit);
  }
  holder.append(svg);
  return holder;
}

// ATTN: the key the picture leaves blank at the top left, made wider
// -- outdented into the space left of the row -- so the label reads
// at a normal size. It is where a reader's eye goes for an escape.
function attention(box, attn) {
  const [x, y, kw, kh] = box;
  const left = -PAD + 3;
  const g = shape("g", { class: "key attn" });
  g.setAttribute("role", "button");
  g.setAttribute("aria-label", "ATTN, stop what is running");
  g.append(shape("rect", { x: left, y, width: x + kw - left, height: kh, rx: 3 }));
  const label = shape("text", { x: (left + x + kw) / 2, y: y + kh / 2 });
  label.textContent = "ATTN";
  g.append(label);
  g.addEventListener("click", () => press(g, attn));
  return g;
}

// Show that a key went down, then do what it does.
function press(el, action) {
  el.classList.add("down");
  setTimeout(() => el.classList.remove("down"), 130);
  action();
}

function shape(tag, attrs) {
  const el = document.createElementNS(NS, tag);
  for (const [k, v] of Object.entries(attrs)) el.setAttribute(k, v);
  return el;
}

// Commands or Idioms: one tap each. A command that takes an argument
// is inserted with a space after it and the carriage left there; none
// is ever sent on its own, so a reader always presses Return.
function list(kind, entries, tap) {
  const panel = document.createElement("div");
  panel.className = "list";
  panel.dataset.kind = kind;
  for (const entry of entries) {
    const button = document.createElement("button");
    button.type = "button";
    button.className = "entry";
    const said = entry.label ?? entry.insert.trim();
    // A command that takes an argument says so, or `)LOAD` and `)LOAD `
    // look like one button twice.
    const shown = entry.insert.endsWith(" ") ? `${entry.insert.trim()} \u2026` : entry.insert;
    button.innerHTML = `<span class="text"></span><span class="label"></span>`;
    button.querySelector(".text").textContent = shown;
    button.querySelector(".label").textContent = entry.label ? said : "";
    button.title = entry.insert.trim();
    button.setAttribute("aria-label", entry.label ? `${said}: ${entry.insert}` : entry.insert);
    button.addEventListener("click", () => tap(entry.insert));
    panel.append(button);
  }
  return panel;
}

// Space, erase and return. Return is the most important key on the
// board and is drawn so.
function controls(tap) {
  const row = document.createElement("div");
  row.className = "row controls";
  const add = (label, aria, sends, cls) => {
    const button = document.createElement("button");
    button.type = "button";
    button.className = `key ${cls}`;
    button.textContent = label;
    button.setAttribute("aria-label", aria);
    button.addEventListener("click", () => tap(sends));
    row.append(button);
  };
  add("Erase", "erase the last glyph", "backspace", "erase");
  add("space", "space", " ", "space");
  add("Return ⏎", "return, send the line", "return", "return");
  return row;
}

// Put the board where it was left. Inside the session, which owns the
// window; top means above the paper.
export function place(root, edge) {
  const session = document.getElementById("session");
  const before = edge === "top"
    ? document.getElementById("paper")
    : session.querySelector(".bar");
  session.insertBefore(root, before);
  root.dataset.edge = edge;
  remember(EDGE, edge);
}

// Set the board's size, clamped, and remember it.
function size(root, rem) {
  const at = Math.min(KEYS.max, Math.max(KEYS.min, rem));
  root.style.setProperty("--key", `${at}rem`);
  root.dataset.size = String(at);
  remember(SIZE, String(at));
  return at;
}

// Where the board sits and how big it is.
function placing(root) {
  const row = document.createElement("div");
  row.className = "row placing";
  const add = (label, aria, onclick, cls = "") => {
    const button = document.createElement("button");
    button.type = "button";
    button.className = `key small ${cls}`;
    button.textContent = label;
    button.setAttribute("aria-label", aria);
    button.addEventListener("click", onclick);
    row.append(button);
  };
  add("⬍", "move the keyboard to the other edge", () => {
    place(root, root.dataset.edge === "top" ? "bottom" : "top");
  }, "pin");
  add("−", "smaller keys", () => size(root, Number(root.dataset.size) - KEYS.step));
  add("+", "bigger keys", () => size(root, Number(root.dataset.size) + KEYS.step));
  return row;
}

// Whose keyboard this is a picture of. Shown exactly when the board
// is, beside the thing it is about.
const ATTRIBUTION =
  "https://github.com/sw-vibe-coding/sw-apl/tree/main/images/redistributed/apl-keyboard";

function credit() {
  const row = document.createElement("div");
  row.className = "row credit";
  const link = document.createElement("a");
  link.href = ATTRIBUTION;
  link.target = "_blank";
  link.rel = "noopener";
  link.textContent = "2741 keyboard image — attribution";
  row.append(link);
  return row;
}
