// (U) '74's 029 keyboard, drawn from keys029.json: the combination
// keyboard of IBM A24-3332-1, Figure 8, as the owner measured it for
// the console-3D model (MIT). Each key is a button placed where the
// figure has it; keys that punch nothing of ours are dimmed.

import { keyChar } from "./punch029.js";

// The figure's keys lie within this box, in its own units.
const BOX = { x: 50, y: 45, w: 2133, h: 793 };
const pct = (v, from, size) => `${((v - from) / size) * 100}%`;
const span = (a, b, size) => `${((b - a) / size) * 100}%`;

// Draw `keys` (from keys029.json) in `host`. A key pressed calls
// `press(legend)`; `punchable(c)` says whether the 029 codes `c`, and
// `acts` names the function keys this page works.
export function keyboard(host, keys, press, punchable, acts) {
  host.replaceChildren(...keys.map((k) => keyButton(k, press, punchable, acts)));
}

// One key: placed, shaped, labeled, dimmed when it does nothing.
function keyButton(k, press, punchable, acts) {
  const [x0, y0, x1, y1] = k.rect;
  const b = document.createElement("button");
  b.type = "button";
  b.tabIndex = -1;
  b.dataset.key = k.legend.replace("\n", " ");
  b.className = k.blue ? "key blue" : "key";
  Object.assign(b.style, {
    left: pct(x0, BOX.x, BOX.w), top: pct(y0, BOX.y, BOX.h),
    width: span(x0, x1, BOX.w), height: span(y0, y1, BOX.h),
  });
  if (k.shape) b.style.clipPath = `polygon(${k.shape.map(([x, y]) =>
    `${pct(x, x0, x1 - x0)} ${pct(y, y0, y1 - y0)}`).join(",")})`;
  label(b, k.legend, (c) => acts.includes(k.legend) || (c !== null && punchable(c)));
  if (!works(k.legend, punchable, acts)) b.classList.add("dim");
  b.addEventListener("mousedown", (e) => e.preventDefault());
  b.addEventListener("click", () => press(k.legend));
  return b;
}

// The legends, upper above lower, as the keytop prints them; one the
// page cannot punch (`punches` says) is dimmed.
function label(b, legend, punches) {
  const parts = legend.split("\n");
  b.replaceChildren(...parts.map((text, i) => {
    const span = document.createElement("span");
    span.textContent = text;
    const shift = parts.length === 2 ? i === 0 : false;
    if (!punches(keyChar(legend, shift))) span.className = "dim";
    return span;
  }));
}

// Whether a key does anything here: a function key the page works,
// or a legend the 029 punches in either shift.
function works(legend, punchable, acts) {
  if (acts.includes(legend)) return true;
  return [false, true].some((n) => {
    const c = keyChar(legend, n);
    return c !== null && punchable(c);
  });
}
