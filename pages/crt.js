// (B) '75's machine: the owner's render of the 5100, with the session
// typed and shown inside its CRT, and a zoom that fills the session's
// room with the CRT alone. The glass's own reflection, cut from the
// render, lies over the text as a translucent scrim.
//
// The paper and the line move into the glass and back out; nothing
// else changes, so the keyboard, the board and paste work as they do
// without it.

// The render's size, and where its glass is in it, in its pixels.
const RENDER = { width: 1630, height: 942 };
const GLASS = { left: 450, top: 222, right: 1000, bottom: 636 };
// The 5100's screen is 64 characters wide; APL385's are .6em.
const COLUMNS = 64 * 0.6 + 1;
// Off, on, or zoomed, between visits: "", "on", "zoom".
const KEPT = "apl-crt";

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
  toggle.addEventListener("click", () => set(view.hidden ? "on" : ""));
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

// Scale the render to the room: all of it, or zoomed, just the glass.
// The text is sized so 64 columns fill the glass's width.
function fit(view, machine, glass) {
  if (view.hidden) return;
  const zoomed = view.classList.contains("zoomed");
  const [x0, y0, x1, y1] = zoomed
    ? [GLASS.left, GLASS.top, GLASS.right, GLASS.bottom]
    : [0, 0, RENDER.width, RENDER.height];
  const { width, height } = view.getBoundingClientRect();
  const s = Math.min(width / (x1 - x0), height / (y1 - y0));
  machine.style.width = `${RENDER.width * s}px`;
  machine.style.height = `${RENDER.height * s}px`;
  machine.style.left = `${width / 2 - (s * (x0 + x1)) / 2}px`;
  machine.style.top = `${height / 2 - (s * (y0 + y1)) / 2}px`;
  glass.style.fontSize = `${((GLASS.right - GLASS.left) * s) / COLUMNS}px`;
}

const kept = () => {
  try {
    return localStorage.getItem(KEPT) || "";
  } catch {
    return "";
  }
};

const keep = (state) => {
  try {
    localStorage.setItem(KEPT, state);
  } catch { /* not remembered */ }
};

// (C) '80's screen, 64 by 16, as large as the room between the header
// and the bar will take: the room fills, and the text is sized to it.
export function room(paper) {
  const space = part("div", "room");
  paper.before(space);
  space.append(paper);
  const fit = () => {
    const { width, height } = space.getBoundingClientRect();
    // JuliaMono's characters are .6em; the line is 1.25em; the padding
    // is .8em a side across and .5em down.
    const size = Math.min(width / (64 * 0.6 + 1.6), height / (16 * 1.25 + 1));
    paper.style.fontSize = `${Math.max(8, Math.floor(size * 10) / 10)}px`;
  };
  new ResizeObserver(fit).observe(space);
}
