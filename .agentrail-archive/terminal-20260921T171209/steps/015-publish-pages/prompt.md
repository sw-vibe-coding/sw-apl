Owner direction 2026-09-20, seventh: publish the demo on GitHub
Pages, the way `sw-ml-study/sw-mlpl` does it.

That repository tracks the whole of `pages/` -- the built wasm
bundle included -- alongside a `.nojekyll` and a `build-info.json`
saying which commit it was built from. There is no workflow: it is
built locally and pushed, and Pages serves the folder. Follow that.

What has to change here:

- `.gitignore` currently excludes everything `just pages` writes:
  `pages/wasm/`, `pages/redistributed/`, `pages/version.txt`,
  `pages/keymap.json`, `pages/glyph-names.json`. Publishing this way
  means tracking them. The comment there already anticipates the
  choice; make it and say which was chosen and why.
- `.nojekyll`, or Pages will not serve `pages/wasm/`.
- A `build-info.json` as sw-mlpl has: the commit, when, and that the
  gates passed. It is the only way a reader can tell what they are
  running.
- A recipe that does the whole of it in one go, so a stale bundle
  cannot be pushed by hand.

Two things that are ours and not sw-mlpl's:

- Pages sends no headers, so the service worker is the only way the
  published page gets cross-origin isolation. That path is already
  built and `just check-pages` already covers it by serving without
  the headers -- but it has never run under a sub-path. A project
  page is served from `/sw-apl/`, so check every URL the page and
  the worker fetch is relative and that the worker's scope still
  covers them.
- `pages/redistributed/` becomes tracked, which makes the published
  bundle a redistribution of the keyboard picture in the plainest
  sense. `check-provenance.sh` guards `images/redistributed/` and
  must guard this too: the LICENSE and ATTRIBUTION.md travel with
  the picture or the gate fails.

Do not turn Pages on in the repository settings. Prepare everything,
push it, and tell the owner the one setting to flip -- publishing is
theirs to do.

Check it by serving the built `pages/` under a `/sw-apl/` prefix and
loading it there, not just at a root.
