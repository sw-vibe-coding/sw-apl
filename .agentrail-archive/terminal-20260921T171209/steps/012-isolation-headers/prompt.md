Owner direction 2026-09-20, fifth: the isolation refusal is
annoying, unhelpful and not wanted.

A five-line lecture fills the paper when the page cannot get shared
memory. The owner is right twice: the wall of text is not wanted,
and it should almost never be reachable in the first place.

The cause is that nothing sw-apl serves sends the two headers
`SharedArrayBuffer` needs. `sw-apl-server` serves the terminal page
and sends neither; `just pages-serve` is `python3 -m http.server`
and sends neither. So every local visit installs a service worker
purely to add headers to its own responses and then loads a second
time under it -- and when any of that does not take, the reader
gets the lecture.

Both are ours and both can simply send the headers:

    Cross-Origin-Opener-Policy: same-origin
    Cross-Origin-Embedder-Policy: require-corp

Then a local visit is isolated on the first response, loads once,
and never registers the worker at all. Keep the service worker: it
is still the only answer on GitHub Pages, which serves what it is
given. It should be the fallback, not the path.

Cut the message to one line. A reader who cannot run it needs to
know that and where to go, not an essay on service workers; put
whatever is worth keeping under Help, which now exists.

Tests: `just check-pages` serves without the headers on purpose, so
the service-worker path stays covered -- keep that case and add one
that a host which does send them needs no worker and loads once.
Check the terminal page `sw-apl-server` serves too; it is the same
question and the same two headers.
