// (A) '70's 3D view: the owner's IBM 2741 (console-3D, MIT), drawn by
// three.js, printing what the session prints. The type element's
// carrier moves to each column as the printer strikes it and back at
// a return, the element tilting and turning to each character as
// console-3D's rig does it (the angles are illustrative, not the
// Selectric's own coding); the sheet in the machine is green-bar
// paper, the printed lines on it, rising a line at each feed.
//
// Loaded only when the 3D view is chosen: three.js is a megabyte.

// At the version this module was fetched at, as apl.js fetches.
const at = new URL(import.meta.url).search;
const here = (path) => new URL(`${path}${at}`, import.meta.url).href;

// Six lines to the inch, the 2741's.
const LINE_M = 0.0254 / 6;
// The canvas the sheet is drawn on, pixels to a meter.
const PX_M = 4000;

// Build the view in `host`. What is returned follows the printer's
// events and the line being typed.
export async function model2741(host) {
  const THREE = await import("three");
  const { GLTFLoader } = await import(here("./redistributed/three/addons/loaders/GLTFLoader.js"));
  const { OrbitControls } = await import(here("./redistributed/three/addons/controls/OrbitControls.js"));
  const config = await fetch(here("./machines/2741-mechanism.json")).then((r) => r.json());
  const gltf = await new GLTFLoader().loadAsync(here("./machines/2741.glb"));
  const stage = stageFor(THREE, host);
  const rig = rigFor(THREE, gltf.scene, config);
  stage.scene.add(gltf.scene);
  const sheet = sheetFor(THREE, gltf.scene, config);
  stage.scene.add(sheet.mesh);
  const look = aim(THREE, stage, rig, config);
  const controls = new OrbitControls(stage.camera, stage.renderer.domElement);
  controls.addEventListener("change", stage.draw);
  controls.minDistance = 0.04;
  controls.maxDistance = 2;
  zooms(host, stage, controls, look);
  const motion = mover(rig, stage.draw);
  return {
    // A printer event: the carrier moves, the sheet takes the character.
    follow(event) {
      sheet.take(event);
      motion.to(event, sheet.column());
      stage.draw();
    },
    // What was printed before the view was first shown.
    history(text) {
      sheet.history(text);
      motion.to({ kind: "return" }, sheet.column());
      stage.draw();
    },
    // The line being typed, or null when the session is not waiting.
    typing(text) {
      sheet.typing(text);
      if (text !== null) motion.to({ kind: "type" }, text.length);
      stage.draw();
    },
    resize: stage.resize,
  };
}

// The renderer, scene, camera and lights in `host`, drawn on demand.
function stageFor(THREE, host) {
  const renderer = new THREE.WebGLRenderer({ antialias: true });
  renderer.setPixelRatio(Math.min(devicePixelRatio, 2));
  host.append(renderer.domElement);
  const scene = new THREE.Scene();
  scene.background = new THREE.Color("#2a2620");
  scene.add(new THREE.HemisphereLight(0xffffff, 0x39444c, 2));
  const camera = new THREE.PerspectiveCamera(35, 1, 0.001, 20);
  let queued = false;
  const draw = () => {
    if (queued) return;
    queued = true;
    setTimeout(() => {
      queued = false;
      renderer.render(scene, camera);
    }, 16);
  };
  const resize = () => {
    const { clientWidth: w, clientHeight: h } = host;
    if (!w || !h) return;
    camera.aspect = w / h;
    camera.updateProjectionMatrix();
    renderer.setSize(w, h, false);
    draw();
  };
  new ResizeObserver(resize).observe(host);
  return { renderer, scene, camera, draw, resize, target: new THREE.Vector3() };
}

// The carrier, tilt and spin nodes, with their rest poses, and the
// pose setter console-3D's viewer uses: the carrier along +X a pitch a
// column, tilt about local X and spin about local Y from rest. The
// model's materials are made unlit, as console-3D's animation does, so
// motion stays quick on a phone.
function rigFor(THREE, root, config) {
  root.traverse((o) => {
    if (!o.isMesh) return;
    const m = o.material;
    o.material = new THREE.MeshBasicMaterial({ color: m.color, map: m.map, side: m.side });
  });
  const node = (role) => root.getObjectByName(config.nodes[role]);
  const [carrier, tilt, spin] = ["carrier", "tilt", "spin"].map(node);
  const rest = { tilt: tilt.quaternion.clone(), spin: spin.quaternion.clone() };
  const q = new THREE.Quaternion();
  const x = new THREE.Vector3(1, 0, 0);
  const y = new THREE.Vector3(0, 1, 0);
  return {
    carrier,
    pose({ column, tilt: t, rotate }) {
      carrier.position.x = config.left_x_m + column * config.pitch_m;
      tilt.quaternion.copy(rest.tilt).multiply(q.setFromAxisAngle(x, THREE.MathUtils.degToRad(t)));
      spin.quaternion.copy(rest.spin).multiply(q.setFromAxisAngle(y, THREE.MathUtils.degToRad(rotate)));
      carrier.updateMatrixWorld(true);
    },
  };
}

// The camera on the print line, close enough to read what is printed:
// well above the type element, looking down over it at the platen
// and the paper rising.
function aim(THREE, stage, rig, config) {
  const where = new THREE.Vector3();
  rig.carrier.getWorldPosition(where);
  const line = new THREE.Vector3(config.left_x_m + config.pitch_m * 30, where.y + 0.03, where.z);
  const middle = new THREE.Vector3(0, where.y - 0.01, 0);
  const high = new THREE.Vector3(0, where.y + 0.06, -0.02);
  // Each view: what it looks at, and from where -- the turn about the
  // upright, the angle down from overhead, and the distance, as
  // console-3D's viewer gives a view.
  const views = {
    close: { target: line, at: [0, 24, 0.17] },
    half: { target: middle, at: [10, 42, 0.48] },
    whole: { target: high, at: [30, 58, 1.1] },
  };
  return (name) => {
    const { target, at } = views[name] ?? views.close;
    const [theta, phi, r] = at.map((v, i) => (i < 2 ? THREE.MathUtils.degToRad(v) : v));
    stage.target.copy(target);
    stage.camera.position.set(
      target.x + r * Math.sin(phi) * Math.sin(theta),
      target.y + r * Math.cos(phi),
      target.z + r * Math.sin(phi) * Math.cos(theta),
    );
    stage.camera.lookAt(target);
  };
}

// The carrier's motion: to each event's column at its pace, the element
// to the character's tilt and turn for a strike and level for a return.
function mover(rig, draw) {
  let pose = { column: 0, tilt: 0, rotate: 0 };
  let from = pose;
  let to = pose;
  let began = 0;
  let span = 0;
  const step = () => {
    const t = span ? Math.min(1, (performance.now() - began) / span) : 1;
    const u = t * t * (3 - 2 * t);
    pose = Object.fromEntries(Object.keys(to).map((k) => [k, from[k] + (to[k] - from[k]) * u]));
    rig.pose(pose);
    draw();
    if (t < 1) setTimeout(step, 16);
  };
  return {
    to(event, column) {
      const code = event.glyph ? event.glyph.codePointAt(0) : 0;
      const struck = event.kind === "strike";
      from = pose;
      to = {
        column,
        tilt: struck ? [-12, -4, 4, 12][code % 4] : 0,
        rotate: struck ? ((code * 73) % 280) - 140 : 0,
      };
      span = event.kind === "return" ? 67 + 3 * Math.abs(from.column) : 60;
      began = performance.now();
      step();
    },
  };
}

// The paper: continuous fan-fold green-bar, one strip from the platen
// up past the top of the machine, over, and down behind it, with the
// lines printed so far, the newest at the platen. It starts where the
// model's own sheet does and leans back as that one does; the model's
// sheet is hidden.
function sheetFor(THREE, root, config) {
  // The loader writes a space in a node's name as an underscore.
  const sheet = root.getObjectByName("static_loaded_cream_paper");
  const box = new THREE.Box3().setFromObject(sheet);
  const size = box.getSize(new THREE.Vector3());
  sheet.visible = false;
  const path = strip(box, size);
  const canvas = document.createElement("canvas");
  canvas.width = Math.round(size.x * PX_M);
  canvas.height = Math.round(path.length * PX_M);
  const texture = new THREE.CanvasTexture(canvas);
  texture.colorSpace = THREE.SRGBColorSpace;
  texture.anisotropy = 4;
  const mesh = new THREE.Mesh(ribbon(THREE, size.x, path.points),
    new THREE.MeshBasicMaterial({ map: texture, side: THREE.DoubleSide }));
  mesh.position.x = (box.min.x + box.max.x) / 2;
  const lines = [""];
  let gone = 0;
  let typed = null;
  let column = 0;
  const draw = () => {
    paint(canvas, { lines, gone, typed }, { left: (config.left_x_m - box.min.x) * PX_M, pitch: config.pitch_m * PX_M });
    texture.needsUpdate = true;
  };
  const trim = () => {
    const over = lines.length - 200;
    if (over > 0) {
      lines.splice(0, over);
      gone += over;
    }
  };
  draw();
  return {
    mesh,
    column: () => column,
    take(event) {
      // A prompt moves the carriage; the typed line puts it on paper.
      if (event.prompt) {
        column = event.kind === "return" ? 0 : event.column + 1;
        return;
      }
      const last = lines.length - 1;
      if (event.kind === "feed") lines.push("");
      if (event.kind === "return") column = 0;
      if (event.kind === "strike" || event.kind === "space") {
        const ch = event.text || event.glyph || " ";
        const row = lines[last].padEnd(event.column, " ");
        lines[last] = row.slice(0, event.column) + ch + row.slice(event.column + 1);
        column = event.column + 1;
      }
      if (event.kind === "back") column = event.column;
      trim();
      draw();
    },
    typing(text) {
      typed = text;
      draw();
    },
    history(text) {
      lines.splice(0, lines.length, ...text.split("\n"));
      trim();
      column = lines.at(-1).length;
      draw();
    },
  };
}

// The strip's path in the machine's YZ plane, from the platen: up and
// back along the model's sheet and beyond it, over a fold, and down
// behind. Each point has its distance along the strip, `s`.
function strip(box, size) {
  const lean = Math.atan2(size.z, size.y);
  const up = [Math.cos(lean), -Math.sin(lean)];
  const rise = Math.hypot(size.y, size.z) + 0.12;
  const radius = 0.03;
  const points = [];
  let [y, z, s] = [box.min.y, box.max.z, 0];
  const to = (ny, nz) => {
    s += Math.hypot(ny - y, nz - z);
    [y, z] = [ny, nz];
    points.push({ y, z, s });
  };
  points.push({ y, z, s });
  for (let i = 1; i <= 24; i++) to(box.min.y + up[0] * rise * i / 24, box.max.z + up[1] * rise * i / 24);
  // Over the fold: from leaning back to hanging straight down.
  const [cy, cz] = [y - Math.sin(lean) * radius, z - Math.cos(lean) * radius];
  const start = Math.atan2(y - cy, z - cz);
  for (let i = 1; i <= 16; i++) {
    const a = start + ((Math.PI - start) * i) / 16;
    to(cy + radius * Math.sin(a), cz + radius * Math.cos(a));
  }
  for (let i = 1; i <= 12; i++) to(y - 0.2 / 12, z);
  return { points, length: s };
}

// A strip of `width` along `points`, its texture running the length:
// the bottom of the texture at the platen.
function ribbon(THREE, width, points) {
  const total = points.at(-1).s;
  const position = [];
  const uv = [];
  const index = [];
  points.forEach(({ y, z, s }, i) => {
    position.push(-width / 2, y, z, width / 2, y, z);
    uv.push(0, s / total, 1, s / total);
    if (i) index.push(2 * i - 2, 2 * i - 1, 2 * i, 2 * i - 1, 2 * i + 1, 2 * i);
  });
  const geometry = new THREE.BufferGeometry();
  geometry.setAttribute("position", new THREE.Float32BufferAttribute(position, 3));
  geometry.setAttribute("uv", new THREE.Float32BufferAttribute(uv, 2));
  geometry.setIndex(index);
  return geometry;
}

// Fan-fold green-bar paper with `lines` on it, the last at the platen
// and the earlier ones up the strip: stripes three lines deep, a
// sprocket hole every half inch in each tractor margin, and a
// perforation every eleven inches, all counted from the first line
// printed, so they move with the paper. `gone` lines have run off.
function paint(canvas, { lines, gone, typed }, { left, pitch }) {
  const g = canvas.getContext("2d");
  const lh = LINE_M * PX_M;
  const shown = typed === null ? lines : [...lines.slice(0, -1), lines.at(-1) + typed];
  // The newest line just above the paper bail, where it can be read.
  const base = canvas.height - lh * 2.6;
  const y = (i) => base - (shown.length - 1 - i) * lh;
  const first = Math.max(0, shown.length - Math.ceil(canvas.height / lh) - 1);
  g.fillStyle = "#fbfcf7";
  g.fillRect(0, 0, canvas.width, canvas.height);
  const margin = lh * 3;
  for (let i = first - 6; i < shown.length + 6; i++) {
    const n = i + gone;
    const top = y(i) - lh * 0.8;
    if (Math.floor(n / 3) % 2 === 0) {
      g.fillStyle = "#d9ecd6";
      g.fillRect(margin, top, canvas.width - 2 * margin, lh);
    }
    g.fillStyle = "#2a2620";
    if (n % 3 === 0) {
      for (const x of [margin / 2, canvas.width - margin / 2]) {
        g.beginPath();
        g.arc(x, top, lh * 0.3, 0, Math.PI * 2);
        g.fill();
      }
    }
    if (n % 66 === 0) {
      g.fillStyle = "#b9b2a2";
      for (let x = 0; x < canvas.width; x += lh * 0.5) g.fillRect(x, top - 1, lh * 0.25, 2);
    }
  }
  g.fillStyle = "#1f2420";
  g.font = `${pitch * 1.6}px "APL385 Unicode", monospace`;
  g.textBaseline = "alphabetic";
  for (let i = first; i < shown.length; i++) {
    [...shown[i]].forEach((ch, c) => g.fillText(ch, left + c * pitch, y(i)));
  }
}

// The views as buttons over the model -- Close, to read what is printed
// while typing; Half, the platen and keyboard; Whole, the machine, for
// showing it -- remembered, Close at first; and closer and farther in
// whichever is shown, for a reader without a wheel or a pinch. The
// wheel, a pinch and a drag do the same through the controls.
const VIEW = "apl-2741-camera";

function zooms(host, stage, controls, look) {
  const bar = document.createElement("div");
  bar.className = "zoom";
  const buttons = {};
  const go = (name) => {
    look(name);
    controls.target.copy(stage.target);
    controls.update();
    for (const [n, b] of Object.entries(buttons)) b.setAttribute("aria-pressed", String(n === name));
    try {
      localStorage.setItem(VIEW, name);
    } catch {
      // Not kept; this visit holds it.
    }
  };
  const zoom = (by) => {
    const away = stage.camera.position.clone().sub(controls.target).multiplyScalar(by);
    stage.camera.position.copy(controls.target).add(away);
    controls.update();
  };
  const add = (label, title, act) => {
    const button = document.createElement("button");
    button.type = "button";
    button.textContent = label;
    button.title = title;
    button.addEventListener("click", act);
    bar.append(button);
    return button;
  };
  buttons.close = add("Close", "Close up on the printing, to read it", () => go("close"));
  buttons.half = add("Half", "The platen and the keyboard", () => go("half"));
  buttons.whole = add("Whole", "The whole 2741", () => go("whole"));
  add("+", "Closer", () => zoom(0.7));
  add("−", "Farther", () => zoom(1 / 0.7));
  host.append(bar);
  let kept = null;
  try {
    kept = localStorage.getItem(VIEW);
  } catch {
    kept = null;
  }
  go(kept in buttons ? kept : "close");
}
