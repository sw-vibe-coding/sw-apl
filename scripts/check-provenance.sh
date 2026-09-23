#!/usr/bin/env bash
# Every workspace tracked in ws/ must say it is ours.
#
# The point is not the line; it is what committing without it would
# mean. Historical APL workspaces -- APLCOURSE and the rest of
# APL\360's library 1 -- are IBM material of unclear copyright, and
# converting one is easy enough that it could land in ws/ by
# accident. This gate makes that a deliberate false claim rather than
# an oversight. Anything from elsewhere belongs in work/, which is
# gitignored; see work/README.md and docs/aplcourse-how-to.md.
#
# `)SAVE` deliberately does NOT write this line. A mark a program
# stamps on everything asserts nothing.
set -euo pipefail
cd "$(git rev-parse --show-toplevel)"
mark='⍝!SOURCE sw-apl'
status=0
tracked="$(git ls-files 'ws/**/*.apl.ws' 'ws/*.apl.ws')"
for f in $tracked; do
    if grep -qF "$mark" "$f"; then
        echo "  ok   $f"
    else
        echo "  FAIL $f: no '$mark' line"
        status=1
    fi
done
# A workspace's file name says the modes it runs in, so a list of the
# files shows it (owner, 2026-09-22): NAME.a-70.apl.ws for (A) only,
# NAME.b-75.apl.ws for (B) only, NAME.apl.ws for both. The modes line
# inside is the authority, and the two must agree.
for f in $tracked; do
    case "$f" in
        *.a-70.apl.ws) want='(A)' ;;
        *.b-75.apl.ws) want='(B)' ;;
        *) want='(A)(B)' ;;
    esac
    have="$(grep -m1 '^⍝!MODES ' "$f" | cut -d' ' -f2- || true)"
    if [ "$have" = "$want" ]; then
        echo "  ok   $f runs in $want"
    else
        echo "  FAIL $f: named for $want, but its modes line says '${have:-nothing}'"
        status=1
    fi
done
# An untracked workspace under ws/ is the mistake this guards against
# one step earlier: it is on its way to being added.
untracked="$(git ls-files --others --exclude-standard 'ws/')"
if [ -n "$untracked" ]; then
    echo "  FAIL untracked files under ws/ (material from elsewhere belongs in work/):"
    echo "$untracked" | sed 's/^/    /'
    status=1
fi
# Borrowed material that IS redistributed lives under
# images/redistributed/, one directory per work, and may not travel
# without the terms it travels under. A licence file nobody wrote is
# the whole risk here: copying a file is one command, and finding out
# afterwards what it was licensed under is not.
for dir in images/redistributed/*/; do
    [ -d "$dir" ] || continue
    for needed in LICENSE ATTRIBUTION.md; do
        if [ -f "$dir$needed" ]; then
            echo "  ok   $dir$needed"
        else
            echo "  FAIL $dir: no $needed"
            status=1
        fi
    done
done
borrowed="$(git ls-files --others --exclude-standard 'images/redistributed/')"
if [ -n "$borrowed" ]; then
    echo "  FAIL untracked files under images/redistributed/:"
    echo "$borrowed" | sed 's/^/    /'
    status=1
fi
# pages/redistributed/ is the same material again, tracked because
# the published demo is served from the folder as committed. That
# makes it a redistribution in the plainest sense, so the terms have
# to be in the bundle and not only in the repository.
for dir in pages/redistributed/*/; do
    [ -d "$dir" ] || continue
    for needed in LICENSE ATTRIBUTION.md; do
        if [ -f "$dir$needed" ]; then
            echo "  ok   $dir$needed"
        else
            echo "  FAIL $dir: no $needed (the published bundle must carry the terms)"
            status=1
        fi
    done
done

[ "$status" = 0 ] && echo "check-provenance: ws/ is ours, and what is borrowed says so"
exit "$status"
