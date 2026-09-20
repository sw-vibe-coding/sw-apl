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
    # The keyboard picture is not ours and is share-alike, so it goes
    # into the bundle with the terms it travels under. A page that
    # shows it is a redistribution like any other.
    mkdir -p pages/redistributed/apl-keyboard
    cp images/redistributed/apl-keyboard/APL-keybd2.svg \
       images/redistributed/apl-keyboard/LICENSE \
       images/redistributed/apl-keyboard/ATTRIBUTION.md \
       pages/redistributed/apl-keyboard/

# Serve pages/ the way a static host would, so the demo can be tried
# before anything is published. The page installs a service worker to
# give itself the headers SharedArrayBuffer needs, which means the
# first visit loads twice; that is the same thing it will do on
# GitHub Pages.
#
# Serve the browser demo at http://127.0.0.1:8361/.
pages-serve: pages
    @echo "sw-apl in a browser: http://127.0.0.1:8361/"
    cd pages && python3 -m http.server 8361 --bind 127.0.0.1

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
