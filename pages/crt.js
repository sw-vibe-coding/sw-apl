// (B) '75's machine: the owner's render of the 5100, with the session
// typed and shown inside its CRT, and a zoom that fills the session's
// room with the CRT alone. The glass's own reflection, cut from the
// render, lies over the text as a translucent scrim.
//
// The paper and the line move into the glass and back out; nothing
// else changes, so the keyboard, the board and paste work as they do
// without it.

// The render's size, and where its glass is in it, in its pixels.
const RENDER = { width: 1200, height: 900 };
const GLASS = { left: 238, top: 280, right: 450, bottom: 438 };
// The 5100's screen is 64 characters wide; APL385's are .6em.
const COLUMNS = 64 * 0.6 + 1;
// Off, the whole machine, or zoomed, between visits: "off", "on",
// "zoom". Zoomed is where a first visit opens, as (C) opens on its
// monitor.
const KEPT = "apl-crt-view";
// How much of the bezel a zoom keeps round the glass, in the
// render's pixels.
const BEZEL = 18;

// Build the view, hang its switch on the bar, and bring it back as
// the last visit left it.
export function crt(paper, line) {
  const view = part("div", "crt");
  view.id = "crt";
  const machine = part("div", "machine");
  const glass = part("div", "glass");
  const zoom = part("button", "zoom", "Zoom");
  machine.append(glass, part("div", "scrim"));
  view.append(machine, zoom);
  view.hidden = true;
  paper.before(view);
  const toggle = part("button", "show-crt", "5100");
  toggle.id = "show-crt";
  toggle.title = "The session on the 5100's screen";
  document.getElementById("show-help").after(toggle);
  const set = (state) => {
    show(state, { view, glass, paper, line, toggle, zoom });
    fit(view, machine, glass);
    keep(state);
  };
  toggle.addEventListener("click", () => set(view.hidden ? "zoom" : "off"));
  zoom.addEventListener("click", () => set(view.classList.contains("zoomed") ? "on" : "zoom"));
  new ResizeObserver(() => fit(view, machine, glass)).observe(view);
  set(kept());
}

// One element, with a class and maybe words.
function part(tag, name, words = "") {
  const el = document.createElement(tag);
  el.className = name;
  if (tag === "button") el.type = "button";
  el.textContent = words;
  return el;
}

// Put the session where the state says, and the buttons with it.
function show(state, { view, glass, paper, line, toggle, zoom }) {
  const on = state === "on" || state === "zoom";
  view.hidden = !on;
  view.classList.toggle("zoomed", state === "zoom");
  document.body.classList.toggle("crt-zoomed", state === "zoom");
  toggle.setAttribute("aria-pressed", String(on));
  zoom.textContent = state === "zoom" ? "Whole machine" : "Zoom";
  if (on) glass.append(paper, line);
  else view.after(paper, line);
  paper.scrollTop = paper.scrollHeight;
}

// Scale the render to the room: all of it, or zoomed, the glass and a
// little bezel.
// The text is sized so 64 columns fill the glass's width.
function fit(view, machine, glass) {
  if (view.hidden) return;
  const zoomed = view.classList.contains("zoomed");
  const [x0, y0, x1, y1] = zoomed
    ? [GLASS.left - BEZEL, GLASS.top - BEZEL, GLASS.right + BEZEL, GLASS.bottom + BEZEL]
    : [0, 0, RENDER.width, RENDER.height];
  const s = place(view, machine, [x0, y0, x1, y1], RENDER);
  glass.style.fontSize = `${((GLASS.right - GLASS.left) * s) / COLUMNS}px`;
}

// Size a picture so the part of it given fills the room, centered,
// and say by how much it was scaled.
function place(room, picture, [x0, y0, x1, y1], size) {
  const { width, height } = room.getBoundingClientRect();
  const s = Math.min(width / (x1 - x0), height / (y1 - y0));
  picture.style.width = `${size.width * s}px`;
  picture.style.height = `${size.height * s}px`;
  picture.style.left = `${width / 2 - (s * (x0 + x1)) / 2}px`;
  picture.style.top = `${height / 2 - (s * (y0 + y1)) / 2}px`;
  return s;
}

const kept = () => {
  try {
    return localStorage.getItem(KEPT) || "zoom";
  } catch {
    return "zoom";
  }
};

const keep = (state) => {
  try {
    localStorage.setItem(KEPT, state);
  } catch { /* not remembered */ }
};

// (C) '80's monitor, the owner's render: its glass and a little of its
// bezel fill the room between the header and the bar, and the screen,
// 64 by 16, is sized to the glass. The render's glare lies over the
// text as it does over (B)'s.
const MONITOR = { width: 650, height: 472 };
const SCREEN = { left: 78, top: 106, right: 458, bottom: 362 };
export function room(paper) {
  const space = part("div", "room");
  const monitor = part("div", "monitor");
  const glass = part("div", "glass");
  monitor.append(glass, part("div", "scrim"));
  space.append(monitor);
  paper.before(space);
  glass.append(paper);
  new ResizeObserver(() => {
    const { left, top, right, bottom } = SCREEN;
    const s = place(space, monitor, [left - 24, top - 24, right + 24, bottom + 24], MONITOR);
    // JuliaMono's characters are .6em and its lines 1.25em, with .8em
    // and .5em of padding; a twentieth of the glass is its curved edge.
    const w = (SCREEN.right - SCREEN.left) * s * 0.95;
    const h = (SCREEN.bottom - SCREEN.top) * s * 0.95;
    paper.style.fontSize = `${Math.min(w / (64 * 0.6 + 1.6), h / (16 * 1.25 + 1))}px`;
  }).observe(space);
}
