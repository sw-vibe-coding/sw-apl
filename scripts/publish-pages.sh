#!/usr/bin/env bash
# Build pages/ for publishing, and say what was built.
#
# The demo is published on GitHub Pages from the tracked pages/
# folder: built here and pushed, as sw-ml-study/sw-mlpl does it,
# with no workflow. So the whole of pages/ is committed, and the one
# thing that can go wrong is pushing a stale bundle. This recipe is
# the answer to that -- it builds, stamps, and records the commit it
# was built from, and `just publish` runs the gates before it.
#
# build-info.json is not decoration. A reader looking at the live
# page has no other way to tell what they are running.
set -euo pipefail
cd "$(dirname "$0")/.."

# wasm-pack writes a .gitignore of its own into the out-dir, holding
# a single `*`. That is right for a package you rebuild and wrong for
# one you publish from the repository, and it silently keeps the
# whole bundle out of the commit -- which is the one way this recipe
# could ship nothing at all.
rm -f pages/wasm/.gitignore

# The TypeScript declarations are for something importing the bundle
# as a package. Nothing serves them.
rm -f pages/wasm/*.d.ts

commit="$(git rev-parse --short HEAD)"
dirty=""
git diff --quiet HEAD -- . ':!pages' || dirty=" (working tree had changes)"

cat > pages/build-info.json <<JSON
{"commit":"${commit}${dirty}","built_at":"$(date -u +%Y-%m-%dT%H:%M:%SZ)","version":"$(cat pages/version.txt)","gates":"just fmt, test, clippy, gates, check-pages"}
JSON

echo "publish-pages: built from ${commit}${dirty}, version $(cat pages/version.txt)"
