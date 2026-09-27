#!/usr/bin/env bash
# Run, tangle and publish the literate documents in docs/literate/:
# record every block's result, write the workspace each tangles to,
# and export each to HTML in pages/literate/. Needs Emacs with htmlize
# and the release build. `just literate` runs it; commit what changes.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
emacs=${EMACS:-}
[ -n "$emacs" ] || ! command -v emacs >/dev/null 2>&1 || emacs=emacs
[ -n "$emacs" ] || [ ! -x /Applications/Emacs.app/Contents/MacOS/Emacs ] || emacs=/Applications/Emacs.app/Contents/MacOS/Emacs
[ -n "$emacs" ] || { echo "literate: no Emacs" >&2; exit 1; }
mkdir -p pages/literate
for org in docs/literate/*.org; do
    "$emacs" --batch -Q -l scripts/literate.el \
        --eval "(progn (sw-apl-literate-run \"$org\") (sw-apl-literate-tangle \"$org\") (sw-apl-literate-export \"$org\"))" \
        </dev/null >/dev/null 2>&1 || { echo "literate: $org failed" >&2; exit 1; }
    mv "${org%.org}.html" pages/literate/
    echo "literate: $org"
done
