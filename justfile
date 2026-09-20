# Each components/<name>/ is its own cargo workspace. These recipes
# iterate over every workspace; scope to one with `just check cli`.

default:
    @just --list

# List the component workspaces.
components:
    @ls -1 components

check component="":
    #!/usr/bin/env bash
    set -euo pipefail
    for c in $(just _select "{{component}}"); do
        echo "== $c"; (cd "components/$c" && cargo check --workspace --all-targets)
    done

test component="":
    #!/usr/bin/env bash
    set -euo pipefail
    for c in $(just _select "{{component}}"); do
        echo "== $c"; (cd "components/$c" && cargo test --workspace)
    done

clippy component="":
    #!/usr/bin/env bash
    set -euo pipefail
    for c in $(just _select "{{component}}"); do
        echo "== $c"; (cd "components/$c" && cargo clippy --workspace --all-targets -- -D warnings)
    done

fmt component="":
    #!/usr/bin/env bash
    set -euo pipefail
    for c in $(just _select "{{component}}"); do
        (cd "components/$c" && cargo fmt --all)
    done

fmt-check component="":
    #!/usr/bin/env bash
    set -euo pipefail
    for c in $(just _select "{{component}}"); do
        (cd "components/$c" && cargo fmt --all -- --check)
    done

# Repo-wide standards gates: run AFTER `just fmt`, since sw-checklist
# measures LOC and function counts on formatted code. README and other
# top-level markdown are ASCII-only; docs/*.md may contain glyphs.
# CLAUDE.md/AGENTS.md carry an agentrail-managed block with em dashes
# (upstream), so they are checked only outside that block by eye.
gates:
    sw-markdown-checker -f README.md
    sw-markdown-checker -f CHANGES.md
    sw-markdown-checker -f "samples/*.md"
    ./scripts/check-provenance.sh
    sw-checklist

# The full pre-commit gate, in order: format, test, lint, standards.
precommit: fmt test clippy fmt-check gates

# Build the release binaries: sw-apl, sw-apl-server, aplterm.
release:
    cd components/cli && cargo build --release -p sw-apl
    cd components/web && cargo build --release -p sw-apl-server
    cd components/term && cargo build --release -p aplterm

# Everything runs on this machine: the interpreter is a process here
# and the page talks to it over the loopback address. Nothing typed
# is sent anywhere. Library 0 is target/demo/work, so a demo session
# saves into scratch rather than into the checkout's work/; ws/ is
# linked through so )LOAD 1 LIFE finds the shipped workspaces.
#
# Start the local service and open the terminal in a browser.
demo: release
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p target/demo/work
    ln -sfn "$PWD/ws" target/demo/ws
    url=http://127.0.0.1:8360/
    ( sleep 1
      if command -v open >/dev/null; then open "$url"
      elif command -v xdg-open >/dev/null; then xdg-open "$url"
      else echo "open $url"; fi ) &
    echo "A 2741 in a terminal instead: target/release/aplterm"
    exec target/release/sw-apl-server --library target/demo

# Build the browser demo into pages/. Needs wasm-pack and the
# wasm32-unknown-unknown target:
#   cargo install wasm-pack
#   rustup target add wasm32-unknown-unknown
#
# Build the WebAssembly session into pages/wasm.
pages:
    cd components/web/crates/apl-wasm && \
      wasm-pack build --release --target web --no-pack \
        --out-dir ../../../../pages/wasm
    # The version every file below index.html is fetched at. It is a
    # digest of what a visitor actually runs, so a rebuild that
    # changed nothing does not invalidate a cache, and one that
    # changed anything invalidates all of it at once. The page asks
    # for this file uncached; it is build output, like pages/wasm.
    # The on-screen board's data: the keymap it draws itself from
    # and what each glyph is called. Both are copies made here, so
    # that neither is copied in the repository.
    ./scripts/gen-board.sh
    ./scripts/stamp-pages.sh
    # The keyboard picture is not ours and is share-alike, so it goes
    # into the bundle with the terms it travels under. A page that
    # shows it is a redistribution like any other.
    mkdir -p pages/redistributed/apl-keyboard
    cp images/redistributed/apl-keyboard/APL-keybd2.svg \
       images/redistributed/apl-keyboard/LICENSE \
       images/redistributed/apl-keyboard/ATTRIBUTION.md \
       pages/redistributed/apl-keyboard/

# Serve pages/ so the demo can be tried before anything is published.
# This sends the two headers SharedArrayBuffer needs, so the page is
# isolated on the first response and loads once. A published demo has
# a static host that will not send them and falls back to the service
# worker; `just check-pages` serves without them to keep that covered.
#
# Serve the browser demo at http://127.0.0.1:8361/.
pages-serve: pages
    ./scripts/serve-pages.py 8361 pages

# Check the browser demo in a browser: a first visit, a visit
# holding the worker from an older bundle, and a worker that never
# answers. Needs a Chrome and playwright-core:
#
#   npm install playwright-core
#   CHROME_PATH=/path/to/chrome just check-pages   (if not the default)
#
# Not part of `just test`: a checkout without a browser must still be
# able to run the suite.
check-pages: pages
    node scripts/check-pages.mjs

# Build the demo for publishing and record what was built. GitHub
# Pages serves the tracked pages/ folder, so this is the whole of
# deployment: run it, commit pages/, and push.
#
# Pages sends no headers of its own, so the published page gets its
# cross-origin isolation from the service worker and loads twice on
# a first visit. That path is what `just check-pages` covers by
# serving without the headers.
publish: pages
    ./scripts/publish-pages.sh
    @echo "publish: commit pages/ and push; GitHub Pages serves it"

# Run the conformance corpus against the release binary.
conformance:
    ./scripts/run-samples.sh

# Re-render the README session recording (needs vhs, ttyd, ffmpeg).
tape: release
    vhs docs/tapes/mvp.tape

# Regenerate docs/glyphs.txt from data/glyphs.toml.
glyphs:
    ./scripts/gen-glyphs.sh

# Regenerate CHANGES.md from git log.
changes:
    ./scripts/gen-changes.sh

_select component:
    #!/usr/bin/env bash
    if [ -n "{{component}}" ]; then echo "{{component}}"; else ls -1 components; fi
