#!/usr/bin/env bash
# reg-rs wrapper: keeps the test database inside the repo.
# Usage: scripts/reg.sh run [-v|-vv] [-p pattern]
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
export REG_RS_DATA_DIR="$PWD/tests/reg-rs"
mkdir -p "$REG_RS_DATA_DIR"
exec reg-rs "$@"
