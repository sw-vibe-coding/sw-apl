#!/usr/bin/env bash
# Write pages/version.txt: the version every file below index.html is
# fetched at.
#
# The page, the worker and the WebAssembly are one build served as
# three files, and a browser caches them separately. index.html asks
# for this file uncached and stamps everything it loads with what it
# says, so the three cannot drift apart. See docs/terminal.md.
#
# A digest rather than a timestamp: a rebuild that changed nothing
# leaves the version alone and a visitor's cache stands, and a
# rebuild that changed anything moves it and the whole bundle is
# refetched together.
set -euo pipefail
cd "$(dirname "$0")/.."

digest() {
    if command -v sha256sum >/dev/null; then sha256sum "$@"; else shasum -a 256 "$@"; fi
}

files=(pages/apl.js pages/board.js pages/worker.js pages/sw.js
       pages/keymap.json pages/glyph-names.json pages/manifest.json
       pages/board-keys.json pages/board.json
       images/redistributed/apl-keyboard/APL-keybd2-board.svg
       pages/wasm/apl_wasm.js pages/wasm/apl_wasm_bg.wasm)
for file in "${files[@]}"; do
    if [ ! -f "$file" ]; then
        echo "stamp-pages: $file is missing; run just pages" >&2
        exit 1
    fi
done

digest "${files[@]}" | digest | cut -c1-16 > pages/version.txt
echo "stamp-pages: pages/version.txt is $(cat pages/version.txt)"
