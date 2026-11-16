// (D) '83's desk: the owner's render of an IBM PC with both displays,
// the session typed and shown in the green monochrome monitor's glass
// and the color monitor's 320 by 200 frame in its own. Each glass is
// where the render has it, its four corners measured, and what is in
// it is mapped onto them in perspective, so the text and the frame lie
// on the glass the way the render's own did. The view zooms to the
// live monitor: Alt-F1 the monochrome, Alt-F4 or Alt-F8 the color, as
// the PC switched; Desk shows it all.

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

// Build the desk, put the session in it, and wire the keys and the
// buttons. The color frame's canvas is returned for what draws on it.
export function desk(paper, line) {
  const view = part("div", "desk");
  view.id = "desk";
  const machine = part("div", "machine");
  const mono = part("div", "glass mono");
  const frame = part("canvas", "glass color");
  [frame.width, frame.height] = SCREEN.color;
  machine.append(mono, frame);
  const buttons = part("div", "views");
  for (const name of ["mono", "color", "desk"]) buttons.append(button(name, () => show(name)));
  view.append(machine, buttons);
  paper.before(view);
  mono.append(paper, line);
  const show = (name) => {
    view.dataset.view = name;
    keep(name);
    fit(view, machine, mono, frame);
  };
  addEventListener("keydown", (e) => switched(e, show), true);
  new ResizeObserver(() => fit(view, machine, mono, frame)).observe(view);
  show(kept());
  return frame;
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
  b.textContent = { mono: "Mono", color: "Color", desk: "Desk" }[name];
  b.addEventListener("click", go);
  return b;
}

// Alt-F1, F4 and F8, the PC's keys for its displays.
function switched(e, show) {
  if (!e.altKey) return;
  const name = { F1: "mono", F4: "color", F8: "color" }[e.key];
  if (!name) return;
  e.preventDefault();
  e.stopPropagation();
  show(name);
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

const kept = () => {
  try {
    return localStorage.getItem(KEPT) || "mono";
  } catch {
    return "mono";
  }
};

const keep = (name) => {
  try {
    localStorage.setItem(KEPT, name);
  } catch { /* not remembered */ }
};
