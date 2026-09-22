Owner direction 2026-09-20, third (docs/plan.md). The demo opens
with the network off.

This is the part that can break what is already working, which is
why it is last.

sw-apl's service worker exists for one reason: adding the two
headers `SharedArrayBuffer` needs. It deliberately caches nothing,
and the bundle-version step built the page's freshness on exactly
that -- only `index.html` is fetched fresh, and everything below is
stamped from `version.txt`. A cache-first service worker dropped on
top would re-create the bug that step fixed: a stale shell that no
reload evicts, and a prompt that ignores typing.

So caching and the version stamp are one design, not two. The
version is already a digest of what a visitor runs, which is a
cache name; decide whether it becomes one, and say in the commit how
an old cache is dropped and when.

The claim is about the interpreter too, not only the shell. The
session is WebAssembly and local storage and reaches for nothing, so
an installed sw-apl should run with the network off -- and the page
should say so rather than leaving a reader to find out.

Check it: `just check-pages` stays green, and add a case that loads
the page, goes offline, and reloads. Then the sequence that found
the last bug -- load, rebuild underneath, reload in the same profile
without clearing -- which must still give the new build and not a
cached old one.
