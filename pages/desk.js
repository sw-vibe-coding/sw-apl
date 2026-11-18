// (D) '83's desk: the owner's render of an IBM PC with both displays,
// the session typed and shown in the green monochrome monitor's glass
// and the color monitor's 320 by 200 frame in its own. Each glass is
// where the render has it, its four corners measured, and what is in
// it is mapped onto them in perspective, so the text and the frame lie
// on the glass the way the render's own did. The view zooms to the
// live monitor: Alt-F1 the monochrome, Alt-F4 or Alt-F8 the color, as
// the PC switched; Desk shows it all. Or, flat (2D), the two screens
// stand side by side, square on, each in a bezel.

// The render's size, and each glass's corners in its pixels: top left,
// top right, bottom right, bottom left.
const RENDER = { width: 1000, height: 800 };
const MONO = [[280, 264], [412, 282], [415, 405], [286, 380]];
const COLOR = [[571, 394], [716, 431], [712, 555], [570, 513]];
// What each glass holds before it is mapped: the 80 by 25 screen at
// 9 by 14 a character, the monochrome adapter's cell; the CGA's frame.
const SCREEN = { mono: [720, 350], color: [320, 200] };
// What each zoom shows, in the render's pixels.
const VIEWS = {
  mono: [250, 240, 450, 430],
  color: [540, 370, 750, 585],
  desk: [0, 0, RENDER.width, RENDER.height],
};
const KEPT = "apl-desk-view";
const FLAT = "apl-desk-flat";
// A flat bezel's width, and the room between and around the screens,
// as parts of a screen's width; a screen is 4 by 3.
const BEZEL = 0.08;
const ROOM = 0.06;

// Build the desk, put the session in it, and wire the keys and the
// buttons. What is returned paints a picture from AP205 on it.
export function desk(paper, line) {
  const view = part("div", "desk");
  view.id = "desk";
  const machine = part("div", "machine");
  const mono = part("div", "glass mono");
  const frame = part("canvas", "glass color");
  [frame.width, frame.height] = SCREEN.color;
  const bezels = [part("div", "bezel"), part("div", "bezel")];
  machine.append(...bezels, mono, frame);
  const buttons = part("div", "views");
  for (const name of ["mono", "color", "desk"]) buttons.append(button(name, () => show(name)));
  const flat = button("flat", () => {
    view.toggleAttribute("data-flat");
    keep(view.hasAttribute("data-flat") ? "2d" : "3d", FLAT);
    show(view.dataset.view);
  });
  buttons.append(flat);
  view.append(machine, buttons);
  paper.before(view);
  mono.append(paper, line);
  const show = (name) => {
    view.dataset.view = name;
    keep(name);
    flat.textContent = view.hasAttribute("data-flat") ? "3D" : "2D";
    if (view.hasAttribute("data-flat")) side(view, machine, [mono, frame], bezels);
    else fit(view, machine, mono, frame);
  };
  if (kept(FLAT) === "2d") view.setAttribute("data-flat", "");
  // A picture shows the color display; the next key typed brings the
  // session's green one back, as the typing is there.
  let drawn = false;
  addEventListener("keydown", (e) => {
    if (switched(e, show)) drawn = false;
    else if (drawn && !e.altKey) {
      drawn = false;
      show("mono");
    }
  }, true);
  new ResizeObserver(() => show(view.dataset.view)).observe(view);
  show(kept(KEPT, "mono"));
  return (picture) => {
    const color = paint(frame, JSON.parse(picture));
    if (view.hasAttribute("data-flat")) return;
    if (color && view.dataset.view === "mono") {
      drawn = true;
      show("color");
    }
    if (!color && view.dataset.view === "color") show("mono");
  };
}

// An element with a class.
function part(tag, name) {
  const el = document.createElement(tag);
  el.className = name;
  return el;
}

// One of the view buttons.
function button(name, go) {
  const b = part("button", name);
  b.type = "button";
  b.textContent = { mono: "Mono", color: "Color", desk: "Desk", flat: "2D" }[name];
  b.addEventListener("click", go);
  return b;
}

// Alt-F1, F4 and F8, the PC's keys for its displays: true when it was
// one of them.
function switched(e, show) {
  if (!e.altKey) return false;
  const name = { F1: "mono", F4: "color", F8: "color" }[e.key];
  if (!name) return false;
  e.preventDefault();
  e.stopPropagation();
  show(name);
  return true;
}

// The palette AP205's color 40 gives an attribute: dark, cyan, red,
// white.
const PALETTE = [[0, 0, 0], [85, 255, 255], [255, 85, 85], [255, 255, 255]];

// Paint AP205's picture on the color frame -- its pixels, then each
// line's characters in their colors, eight by eight -- and say
// whether it is the color display's. The monochrome display's fields
// are not drawn: that glass holds the session.
function paint(frame, { display, width, height, pixels, rows, colors }) {
  if (display === 1) return false;
  [frame.width, frame.height] = [width, height];
  const g = frame.getContext("2d");
  const image = g.createImageData(width, height);
  for (let i = 0; i < pixels.length; i++) {
    const [r, gr, b] = PALETTE[pixels.charCodeAt(i) - 48] ?? PALETTE[0];
    image.data.set([r, gr, b, 255], i * 4);
  }
  g.putImageData(image, 0, 0);
  g.font = "8px APL385, monospace";
  g.textBaseline = "top";
  rows.forEach((row, y) => [...row].forEach((ch, x) => {
    if (ch === " ") return;
    g.fillStyle = `rgb(${PALETTE[colors[y].charCodeAt(x) - 48] ?? PALETTE[3]})`;
    g.fillText(ch, x * 8, y * 8);
  }));
  return true;
}

// Scale the render so the view's part of it fills the room, then map
// each glass's contents onto its corners.
function fit(view, machine, mono, frame) {
  const [x0, y0, x1, y1] = VIEWS[view.dataset.view] ?? VIEWS.mono;
  const { width, height } = view.getBoundingClientRect();
  if (!width || !height) return;
  const s = Math.min(width / (x1 - x0), height / (y1 - y0));
  machine.style.width = `${RENDER.width * s}px`;
  machine.style.height = `${RENDER.height * s}px`;
  machine.style.left = `${width / 2 - (s * (x0 + x1)) / 2}px`;
  machine.style.top = `${height / 2 - (s * (y0 + y1)) / 2}px`;
  onto(mono, SCREEN.mono, MONO, s);
  onto(frame, SCREEN.color, COLOR, s);
}

// Flat: the two screens side by side, each 4 by 3 in its bezel, as
// large as the room allows, centered. The render is not shown.
function side(view, machine, glasses, bezels) {
  const { width, height } = view.getBoundingClientRect();
  if (!width || !height) return;
  const across = 2 * (1 + 2 * BEZEL) + 3 * ROOM;
  const down = 0.75 + 2 * BEZEL + 2 * ROOM;
  const g = Math.min(width / across, (height - 40) / down);
  Object.assign(machine.style, { width: `${width}px`, height: `${height}px`, left: "0px", top: "0px" });
  const [b, h] = [g * BEZEL, g * 0.75];
  const left = (width - g * (across - 2 * ROOM)) / 2;
  const top = (height - 40 - (h + 2 * b)) / 2;
  glasses.forEach((el, i) => {
    const x = left + b + i * (g + 2 * b + g * ROOM);
    const y = top + b;
    const size = i ? SCREEN.color : SCREEN.mono;
    onto(el, size, [[x, y], [x + g, y], [x + g, y + h], [x, y + h]], 1);
    Object.assign(bezels[i].style, {
      left: `${x - b}px`, top: `${y - b}px`, width: `${g + 2 * b}px`, height: `${h + 2 * b}px`,
    });
  });
}

// Place `el`, `size` pixels across and down, so that its corners land
// on `corners` scaled by `s`: the perspective transform that does it,
// as a CSS matrix3d.
function onto(el, [w, h], corners, s) {
  el.style.width = `${w}px`;
  el.style.height = `${h}px`;
  const to = corners.map(([x, y]) => [x * s, y * s]);
  el.style.transform = `matrix3d(${homography([[0, 0], [w, 0], [w, h], [0, h]], to)})`;
}

// The 3 by 3 projective map taking the four points `from` to `to`,
// written out as the 4 by 4 CSS matrix, column by column.
function homography(from, to) {
  const rows = [];
  const rhs = [];
  from.forEach(([x, y], i) => {
    const [u, v] = to[i];
    rows.push([x, y, 1, 0, 0, 0, -u * x, -u * y]);
    rows.push([0, 0, 0, x, y, 1, -v * x, -v * y]);
    rhs.push(u, v);
  });
  const [a, b, c, d, e, f, g, h] = solve(rows, rhs);
  return [a, d, 0, g, b, e, 0, h, 0, 0, 1, 0, c, f, 0, 1].join(",");
}

// Gaussian elimination, with partial pivoting, of an 8 by 8 system.
function solve(m, r) {
  const n = r.length;
  const a = m.map((row, i) => [...row, r[i]]);
  for (let c = 0; c < n; c++) {
    let p = c;
    for (let i = c + 1; i < n; i++) if (Math.abs(a[i][c]) > Math.abs(a[p][c])) p = i;
    [a[c], a[p]] = [a[p], a[c]];
    for (let i = 0; i < n; i++) {
      if (i === c) continue;
      const k = a[i][c] / a[c][c];
      for (let j = c; j <= n; j++) a[i][j] -= k * a[c][j];
    }
  }
  return a.map((row, i) => row[n] / row[i]);
}

const kept = (key = KEPT, otherwise = "") => {
  try {
    return localStorage.getItem(key) || otherwise;
  } catch {
    return otherwise;
  }
};

const keep = (name, key = KEPT) => {
  try {
    localStorage.setItem(key, name);
  } catch { /* not remembered */ }
};
