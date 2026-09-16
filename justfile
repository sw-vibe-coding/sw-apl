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

# Repo-wide standards gates.
gates:
    sw-markdown-checker -f "**/*.md"
    sw-checklist

# Build the release CLI binary into target/release/sw-apl.
release:
    cd components/cli && cargo build --release -p sw-apl

# Run the conformance corpus against the release binary.
conformance:
    ./scripts/run-samples.sh

# Regenerate CHANGES.md from git log.
changes:
    ./scripts/gen-changes.sh

_select component:
    #!/usr/bin/env bash
    if [ -n "{{component}}" ]; then echo "{{component}}"; else ls -1 components; fi
