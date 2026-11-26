// (U) '74's 3D view: the owner's IBM 029 keypunch (console-3D, MIT),
// drawn by three.js, with the card being shown standing in its punch
// station. The model's own card there is printed with a stock
// picture; this draws the deck's card over it instead.
//
// Loaded only when the 3D view is chosen: three.js is a megabyte.

import { drawCard } from "./card029.js";

const at = new URL(import.meta.url).search;
const here = (path) => new URL(`${path}${at}`, import.meta.url).href;

// The card in the punch station. The loader writes a space in a
// node's name as an underscore and drops the dot before a number.
const CARD = "static_029_punched_card_print001";

// Build the view in `host`. What is returned shows a card.
export async function model029(host) {
  const THREE = await import("three");
  const { GLTFLoader } = await import(here("./redistributed/three/addons/loaders/GLTFLoader.js"));
  const { OrbitControls } = await import(here("./redistributed/three/addons/controls/OrbitControls.js"));
  const gltf = await new GLTFLoader().loadAsync(here("./machines/029.glb"));
  const stage = stageFor(THREE, host);
  unlit(THREE, gltf.scene);
  stage.scene.add(gltf.scene);
  const face = faceFor(THREE, gltf.scene);
  const controls = new OrbitControls(stage.camera, stage.renderer.domElement);
  controls.addEventListener("change", stage.draw);
  controls.minDistance = 0.08;
  controls.maxDistance = 4;
  const look = aim(THREE, stage, controls, face.center);
  zooms(host, look);
  look("card");
  return {
    show(text, holes) {
      face.draw(text, holes);
      stage.draw();
    },
    resize: stage.resize,
  };
}

// The renderer, scene and camera in `host`, drawn on demand.
function stageFor(THREE, host) {
  const renderer = new THREE.WebGLRenderer({ antialias: true });
  renderer.setPixelRatio(Math.min(devicePixelRatio, 2));
  host.append(renderer.domElement);
  const scene = new THREE.Scene();
  scene.background = new THREE.Color("#2a2620");
  const camera = new THREE.PerspectiveCamera(35, 1, 0.01, 20);
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
  return { renderer, scene, camera, draw, resize };
}

// The model's materials made unlit, as console-3D's viewer does, so
// it draws quickly on a phone and needs no lights.
function unlit(THREE, root) {
  root.traverse((o) => {
    if (!o.isMesh) return;
    const m = o.material;
    o.material = new THREE.MeshBasicMaterial({ color: m.color, map: m.map, side: m.side });
  });
}

// The card in the punch station, drawn from a canvas.
function faceFor(THREE, root) {
  const mesh = root.getObjectByName(CARD);
  const canvas = document.createElement("canvas");
  canvas.width = 1600;
  const texture = new THREE.CanvasTexture(canvas);
  texture.colorSpace = THREE.SRGBColorSpace;
  texture.flipY = false;
  texture.anisotropy = 4;
  mesh.material = new THREE.MeshBasicMaterial({ map: texture, transparent: true, side: THREE.DoubleSide });
  const center = new THREE.Box3().setFromObject(mesh).getCenter(new THREE.Vector3());
  return {
    center,
    draw(text, holes) {
      drawCard(canvas, text, holes);
      texture.needsUpdate = true;
    },
  };
}

// The camera: on the card, reading it, or back far enough to see the
// whole machine. The front of the 029 faces +Z.
function aim(THREE, stage, controls, card) {
  const views = {
    card: { target: card, from: [0, 0.02, 0.34] },
    whole: { target: new THREE.Vector3(0, 0.75, 0), from: [0.6, 0.5, 2.3] },
  };
  return (name) => {
    const { target, from } = views[name] ?? views.card;
    controls.target.copy(target);
    stage.camera.position.set(target.x + from[0], target.y + from[1], target.z + from[2]);
    controls.update();
    stage.draw();
  };
}

// The zoom buttons over the view.
function zooms(host, look) {
  const bar = document.createElement("div");
  bar.className = "zoom";
  for (const [name, label] of [["card", "Card"], ["whole", "029"]]) {
    const b = document.createElement("button");
    b.type = "button";
    b.textContent = label;
    b.addEventListener("click", () => look(name));
    bar.append(b);
  }
  host.append(bar);
}
