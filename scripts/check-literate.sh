#!/usr/bin/env bash
# Gate for the literate documents in docs/literate/. In a scratch copy
# of what they read and write, run scripts/literate.el exactly as
# scripts/literate.sh does, then check that nothing changed:
#
#   1. each workspace a document tangles to is the committed one, byte
#      for byte;
#   2. each document's recorded results are what sw-apl prints today;
#   3. each exported page is the committed one in pages/literate/.
#
# Needs Emacs with htmlize and the release build. Where there is no
# Emacs it says so and passes, as `just test-emacs` does.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
emacs=${EMACS:-}
[ -n "$emacs" ] || ! command -v emacs >/dev/null 2>&1 || emacs=emacs
[ -n "$emacs" ] || [ ! -x /Applications/Emacs.app/Contents/MacOS/Emacs ] || emacs=/Applications/Emacs.app/Contents/MacOS/Emacs
if [ -z "$emacs" ]; then echo "check-literate: no Emacs; skipped"; exit 0; fi
[ -x target/release/sw-apl ] || { echo "check-literate: build first (just release)" >&2; exit 1; }

scratch=$(mktemp -d)
trap 'rm -rf "$scratch"' EXIT
mkdir -p "$scratch/docs" "$scratch/scripts" "$scratch/ws" "$scratch/target/release"
cp -R docs/literate docs/emacs "$scratch/docs/"
cp scripts/literate.el "$scratch/scripts/"
cp -R ws/lib1 "$scratch/ws/"
cp target/release/sw-apl "$scratch/target/release/"

status=0
for org in docs/literate/*.org; do
    (cd "$scratch" && "$emacs" --batch -Q -l scripts/literate.el \
        --eval "(progn (sw-apl-literate-run \"$org\") (sw-apl-literate-tangle \"$org\") (sw-apl-literate-export \"$org\"))" \
        </dev/null >/dev/null 2>&1) || { echo "  FAIL $org: Emacs could not run it"; status=1; continue; }
    name=$(basename "$org" .org)
    if cmp -s "$org" "$scratch/$org"; then
        echo "  ok   $org: its recorded results are what sw-apl prints"
    else
        echo "  FAIL $org: a recorded result has drifted (just literate, then look at the diff)"
        diff "$org" "$scratch/$org" | head -20 || true
        status=1
    fi
    if cmp -s "pages/literate/$name.html" "$scratch/docs/literate/$name.html"; then
        echo "  ok   pages/literate/$name.html is its export"
    else
        echo "  FAIL pages/literate/$name.html is not its export (just literate)"
        status=1
    fi
done
if diff -r ws/lib1 "$scratch/ws/lib1" >/dev/null; then
    echo "  ok   every workspace a document tangles to is the one committed"
else
    echo "  FAIL a tangled workspace differs from the committed one:"
    diff -r ws/lib1 "$scratch/ws/lib1" | head -20 || true
    status=1
fi
[ "$status" = 0 ] && echo "check-literate: the documents, their workspaces and their pages agree"
exit "$status"
