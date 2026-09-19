Phase 8 step 3 (docs/plan.md). Build it and put it somewhere.

The deploy: GitHub Pages, built by trunk, from a script that a
person can run and CI can run the same way. `just` already holds the
project's commands and should hold this one.

Points to settle rather than assume:

- Where it is served from. A project page lives under a path, not
  at a domain root, so the asset URLs have to be right for that or
  the page loads blank. Check it against the real URL, not against
  a local server at `/`.
- What is committed. The built wasm and JS are generated, so they
  do not belong in the repository's source; `.gitignore` already
  excludes `components/web/dist/`. Deploy from CI or from a branch
  that is clearly generated, and say which.
- Caching. A wasm blob with a stable name and a long cache life is
  a page that shows the old interpreter after an update.
- Whether the demo says what it is. A reader arriving cold should
  be able to tell within a sentence that this is APL\360, that it
  runs in their browser with nothing sent anywhere, and where the
  source is.

Then close the loop in the README: it links to docs today and should
link to the live thing, with a line saying what it is.

Check it in a browser before claiming it works -- the page loading
and the interpreter answering are two different things. If the
browser tools are available, drive it: type an expression, take the
transcript, and compare it to what the CLI prints for the same
input.
