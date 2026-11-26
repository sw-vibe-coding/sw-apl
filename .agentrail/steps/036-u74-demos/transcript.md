# Transcript: step 036-u74-demos

- session: `837bcc12-de7d-448a-a103-977a7ae14f58`
- log lines: 36338–36789 (452 lines)
- log sha256: `8f6706fc9edbd0650c3283239720f1704d863ac5e84ba4c07072cb399bc4997f` (the log's first 36789 lines)
- commits: 9a28e8e4492a09d59f8f935913030c3ed3a6bca1
- tokens: 98 input, 20779 output, 9991650 cache read, 42555 cache write

Prompts and replies only. Tool calls, tool results, thinking and metadata are
omitted; the full log is kept outside the repository (`agentrail sessions path`).

---

**Assistant** · 2026-11-26T22:36:30.373Z

Now publish, so the page goes live:

The first deploy failed. I'll check why; the second run is in progress.

That's a transient GitHub artifact error: the two pushes landed 2 seconds apart. The second run deploys the same `pages/`. I'll wait for it.

It's deployed. One quick check that the live page runs and not just that the files load:

U '74 is live on GitHub now: **https://sw-apl.softwarewrighter.com/demo.html?mode=U**

I checked it in Chrome. The link forwards to `deck.html`, the Domino deck loads, and Submit returns the green-bar printout with `1 3` and END OF RUN. If your browser shows the old site, reload once. The timeline doesn't link to it yet; that comes in step 037.

**Step 035 report (u74-deck)**
1. **Pushed:**
   - `f8f19a0` is the deck page:
     - It shows the demo decks a card at a time, with the holes drawn from the 029's card code.
     - It has a 3D view of your console-3D IBM 029 with the current card in its punch station.
     - Submit runs the deck in WebAssembly and prints a 132-column green-bar listing.
     - It adds two crates, `apl-u74-code` and `apl-u74-web`.
   - Then `53e704e` (saga complete), `cbd6216` (the publish) and `00a73c4` (CHANGES.md).
   - fmt, clippy, the tests, the gates and all 119 regression transcripts pass.
   - The first Pages deploy failed on a GitHub artifact error because two pushes landed 2 seconds apart. The second deploy succeeded with the same files.
2. **Next:** 036 u74-keypunch, about 3–4 hours:
   - punching your own cards on a virtual 029 keyboard;
   - the reference of the `$` symbols;
   - the "A few moments later..." card;
   - the printout beside the deck, for repunching and resubmitting.

   Then 037 links the timeline to U.
3. **Blockers:** none. Two notes:
   - agentrail warns that 29 session logs (406 MiB) sit inside `.agentrail/`; `agentrail sessions migrate` would move them out.
   - The build cache will grow back over time. A pruning recipe would keep `just test` fast; I can add it as a small step.

---

**User** · 2026-11-26T22:53:38.818Z

go

---

**Assistant** · 2026-11-26T22:53:40.328Z

Starting step 036.

Tests first for the reference export. `face.rs` becomes `punch.rs`, covering the card's holes and how to spell what the 029 can't punch.

---

**User** · 2026-11-26T22:54:58.131Z

what time is it now UTC?  PT is 14:44

---

**Assistant** · 2026-11-26T22:55:47.119Z

I'll move step 037 (u74-demos: the U tab live, the timeline linked, check-pages) ahead of the keypunch. My uncommitted keypunch work stays in the tree and won't be in that commit.

Status: I'm linking the timeline's University of Maryland entry to the live U '74 demo, test first. Then I'll rebuild, check the page and deploy (about 20 minutes).

RED. Now the entry:

An existing test pins the demos as four. I'll fold my new test into it rather than keep two tests of the same thing.

GREEN. Now the timeline doc's UMD row. The D'83 row is also stale ("a demo still to come"), so I'll fix both. Then check-pages.

Updating the timeline checks for five demos and two "Future demo" pictures, then adding a U '74 section: the forward, the card viewer, the 3D model, Submit and the printout.

Now the U '74 section, inserted after the timeline check:

The pages build compiles `apl-u74-web`, which holds my uncommitted keypunch changes. I'll stash those first so this step's build and commit carry only this step's work.

`check-pages` rebuilds the pages itself, and that rebuild needs the uv Python shim on PATH. Rerunning with it:

All pass. Gates next, then commit and publish.

I'll record the reordering in `docs/plan.md`, since owner direction lands there first. Then commit.
