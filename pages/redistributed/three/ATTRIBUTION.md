# three.js

The 3D view of (A)'s demo, the 2741 in the owner's model, is drawn
with three.js. It is not ours. It is redistributed here under its own
terms, kept in `images/redistributed/` so that what sw-apl wrote and
what sw-apl borrowed are never in the same directory, and copied into
`pages/redistributed/three/` by `just pages`.

## Credit

- **Title:** three.js, release 170 (npm package `three` 0.170.0)
- **Authors:** the three.js authors
- **Source:** <https://unpkg.com/three@0.170.0/>, as the owner's
  console-3D project vendored it (`web/vendor/`, with its
  `three-provenance.json`): `build/three.module.js`,
  `examples/jsm/loaders/GLTFLoader.js`,
  `examples/jsm/utils/BufferGeometryUtils.js` and
  `examples/jsm/controls/OrbitControls.js`, each unchanged, under
  `addons/` as the package's `examples/jsm/` is.
- **Terms:** the MIT License, in `LICENSE` beside this file.

## Use

The page names `three` in an import map, so the loader's own
`import ... from 'three'` finds `three.module.js` here. It is loaded
only when the 3D view is chosen.
