Owner direction 2026-09-20, third (docs/plan.md). The demo is
installable.

A manifest, the meta tags an installed page needs, and icons.
`sw-fun/suduko/pages/manifest.json` and the head of its
`index.html` are the owner's pointer: name and short name, a
description, scope and start_url that work both at the root
locally and under a Pages sub-path, display standalone, background
and theme colours that match the paper, and the
`apple-mobile-web-app-*` tags plus `viewport-fit=cover`.

The icons are ours to make and must not be borrowed. A glyph on
paper is the obvious answer; whatever it is, it needs the 192 and
512 sizes and a maskable 512, and `scripts/check-provenance.sh`
must be happy that they are ours.

Do not add offline caching here. That is the next step, and it is
the one that can break what already works.

Check it: Chrome's Application panel shows the manifest with no
errors and offers to install, and the installed window opens on the
prompt rather than on a scrollbar.
