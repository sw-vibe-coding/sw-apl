# Transcript: step 037-u74-keypunch

- session: `837bcc12-de7d-448a-a103-977a7ae14f58`
- log lines: 36790–37588 (799 lines)
- log sha256: `186a5bbfc56205826ebc8a4e874a9d0232ab6c2b7b22368f5a236c3e06bc25b1` (the log's first 37588 lines)
- commits: 201a901054c248882a3aa750274e28033c582089
- tokens: 156 input, 46418 output, 8253974 cache read, 288467 cache write

Prompts and replies only. Tool calls, tool results, thinking and metadata are
omitted; the full log is kept outside the repository (`agentrail sessions path`).

---

**Assistant** · 2026-11-26T23:17:38.055Z

This time I'll push the step, the publish and the CHANGES refresh together, so only one deploy runs.

It's live, build `0b4d031`. Restoring the stashed keypunch work for the next step:

The timeline now links to U '74, and it's live. The University of Maryland APL entry at **https://sw-apl.softwarewrighter.com/** shows "U'74 · my live demo" and opens the deck page. The deploy finished at 23:18 UTC (15:18 PT). Reload once if your browser still shows "Future demo".

**Step report (u74-demos, now step 036)**
1. **Pushed:**
   - `9a28e8e` links the timeline entry to `demo.html?mode=U`:
     - The timeline tests now pin five demos, each linking to its own mode.
     - check-pages now covers the timeline's five links and its two remaining "Future demo" pictures.
     - It also covers mode U opening the deck page, stepping to the next card, the 029 in 3D, and Submit returning the printout. Everything passes.
     - `docs/apl-timeline.md` now says the Maryland demo is live. It also fixes the D'83 row, which still said that demo was to come.
     - `docs/plan.md` records your reordering.
   - Then `0b4d031` (saga), `7d8fa69` (publish) and `ff20e66` (CHANGES.md).
   - All went out in one push, so only one deploy ran, and it succeeded.
2. **Next:** 037 u74-keypunch, about 3–4 hours:
   - punching your own cards on the virtual 029;
   - the `$` reference;
   - the "A few moments later..." card;
   - the deck and printout side by side for repunching.

   The work I'd started on it is restored in the tree, and I'm resuming it now.
3. **Blockers:** none.

---

**User** · 2026-11-27T00:13:37.807Z

This session is being continued from a previous conversation that ran out of context. The summary below covers the earlier portion of the conversation.


Summary:
1. Primary Request and Intent:
   - **Saga discipline:** continue the sw-apl agentrail saga on main.
     - Each step: TDD, then fmt → clippy → test → gates → commit → agentrail complete → push → CHANGES refresh.
     - Give a step completion report each time: what was pushed, next steps, blockers.
   - **U '74:** simulate UMD APL (1974) from punched cards. Not an emulator.
     - Phase 1 is done and live: pick a deck, view cards in 2D/3D, Submit, green-bar printout.
     - Phase 2 (current step 037-u74-keypunch): punch cards on a virtual 029 keyboard beside a reference of the UMD $ symbols; submit; an original "A few moments later..." title card (our own art, not the show's), then the printout with errors; printout and deck side by side to repunch, replace, insert or remove cards and resubmit.
   - **Owner rules:** UMD has no disk, no workspaces and no )COMMANDS; decks build the workspace. The virtual APL keyboard is n/a for U; input is the keypunch virtual keyboard plus the normal keyboard.
   - **This session's user asks:**
     - Diagnose the slow tests; done. "go" approved the clean.
     - ETA questions about the live demo; answered.
     - X_eTaL pin question; answered: keep the pin for now. Moving it is console-3D's decision. A candidate-only 029 rebuild at HEAD was offered but not requested.
     - "the timeline says Future Demo" led to the reorder and linking; done and live.
     - "what time is it now UTC? PT is 14:44": answered 22:44 UTC (PST, UTC−8).
   - **Standing constraints (verbatim or near-verbatim):**
     - Never use the system python3; use uv (`uv run --no-project python`). The just pages/publish recipes need PATH=/private/tmp/claude-501/-Users-mike-github-sw-vibe-coding-sw-apl/837bcc12-de7d-448a-a103-977a7ae14f58/scratchpad/bin:$PATH (the uv python3 shim).
     - Never run sw-install without an explicit request.
     - Downloads need explicit permission.
     - Don't redistribute apl80 source.
     - Don't commit cloud.png, docs/rubiks-cube-apl-eigencube.txt or docs/research3.txt (the last has Gemini's sidebar and the user's name).
     - Never use third-party photos; console-3D references/ photos are third-party, don't use them.
     - Never commit third-party material; paraphrase it and cite it in docs/citations.md.
     - American spelling.
     - Stage files by name.
     - No backticks in agentrail args.
     - Run clippy before tests; rerun tests after a clippy fix.
     - Expand, don't squeeze (new crate or module rather than cramming).
     - The letter E is reserved for 3279G.
     - README and top-level markdown are ASCII-only.
     - User-facing docs: no "planned"/"new".
     - Record owner direction in docs/plan.md first.
     - Push together to avoid concurrent Pages deploys; two pushes 2 s apart caused an artifact failure.
   - **Reads:** blockReadsOutsideWorkingDirectories applies. Reading files outside the working directories (e.g., console-3D images) is blocked; copy them in via Bash if they need viewing.

2. Key Technical Concepts:
   - **Repo layout:** multi-workspace Rust repo, components/<name>/, shared target/.
   - **sw-checklist budgets:** ≤25 LOC per fn, ≤4 fns per module, ≤4 modules per crate (lib.rs counts). The 3 expected warnings are Binary Freshness.
   - **Build cache:** macOS dev builds leave .o files in target/debug/deps (unpacked debuginfo) and they accumulate. A pruning recipe was suggested as a future small step.
   - **pages/:** tracked in full; pushing to main deploys via .github/workflows/static.yml to https://sw-apl.softwarewrighter.com (CNAME).
     - `just publish` = just pages + scripts/publish-pages.sh, which writes build-info.json.
     - stamp-pages.sh computes version.txt from a file list.
     - sw.js SHELL caches files.
     - Pages fetch version.txt with no-store and import modules with ?v=&ts=.
   - **Check scripts:**
     - scripts/check-pages.mjs is Playwright, run by `just check-pages` (which depends on pages, so it needs the PATH shim).
     - scripts/check-{lines,carriage,printer}.mjs are node-only checks run by `just test`.
   - **wasm-pack:** `--target web --no-pack`; init accepts `{module_or_path}`.
   - **three.js:** loaded via importmap "three" → ./redistributed/three/three.module.js. GLTFLoader sanitizes names: spaces become `_`, dots are dropped.
   - **029 model:** console-3D Ibm029.xtl/make-ibm029.xtl, built with `XETAL_EXTENSIONS_ROOT=<scratchpad>/xext uv run --no-project python scripts/build.py 029 --candidate-only --device METAL --timeout 1200`.
     - The scratchpad xext clone is at pinned 0f93e06 with work/xetal at d284a8c, built.
     - Output: ~/github/softwarewrighter/console-3D/work/ibm029/candidate-20261126-101542-353480/.
     - The card mesh "static 029 punched card print.001" sits at the punch station (0.187×0.082 m; the front faces +Z in glTF).
   - **029 card code:** rows 12, 11, 0, 1–9 map to bits 0–11.
   - **Keyboard layout:** console-3D models/029-keyboard.json (source: IBM A24-3332-1 Figure 8), keys with legend "upper\nlower", rect, blue, shape.

3. Files and Code Sections:
   - **components/u74/crates/apl-u74-code/** (new; committed in 035)
     - src/lib.rs: `mod code; pub use code::{Holes, ROWS, holes};`
     - src/code.rs:
       - `pub const ROWS: [&str;12] = ["12","11","0","1",...,"9"]`;
       - `SPECIALS: [(char,&[&str]);26]` (the 029 specials incl. ' ' → [], '&' → ["12"], '$' → ["11","3","8"], '¬' → ["11","7","8"], '@' → ["4","8"], '"' → ["7","8"], ...);
       - `pub struct Holes(pub u16)` with `rows()`;
       - `pub fn holes(c) -> Option<Holes>`: digits '0'..='9' → ROWS[d+2]; A-I zone 12; J-R zone 11; S-Z zone 0 digit+2; else specials.
     - tests/code_tests.rs: 7 tests.
   - **components/u74/crates/apl-u74-web/** (new; committed in 035)
     - Cargo.toml:
       - cdylib + rlib;
       - deps: apl-session, apl-u74-batch, apl-u74-card, apl-u74-code; wasm-bindgen for wasm32;
       - dev-dep: apl-u74-listing.
     - src/decks.rs:
       - `pub struct Deck{title, cards}`;
       - `pub const DECKS: [Deck;3]` = Domino, "A program", "Typos", via include_str!("../../../../../samples/99-{domino,program,errors}-74.deck");
       - `pub fn printout(cards:&str)->String` runs apl_u74_batch::run with a Memory host, Mode::U, TimeSource::None, joined by "\n".
     - src/face.rs (committed): `pub fn holes(line)->Vec<u16>` gives 80 entries; an unpunchable character is u16::MAX.
     - src/wasm.rs (cfg wasm32): `decks()`, `deck(i)` ("title\ncards"), `card(line)->Vec<u16>`, `submit(cards)`.
     - tests/web_tests.rs:
       - decks run to END_OF_RUN or NO_INPUT; domino contains "\n1 3\n";
       - holes("A1 $") gives [0b1001, 0b1000, 0, 0b0100_0010_0010];
       - unpunchable → MAX; length 80.
     - **Uncommitted keypunch WIP (restored from stash):**
       - face.rs renamed to punch.rs (git mv staged); lib.rs `mod punch; pub use punch::{holes, reference};`.
       - punch.rs adds `use apl_u74_mnemonic::MNEMONICS;` and `pub fn reference() -> String` (lines "$RH ⍴"... joined by "\n"; 45 lines).
       - Cargo.toml adds apl-u74-mnemonic.
       - wasm.rs adds `#[wasm_bindgen] pub fn spellings() -> String { reference() }`.
       - web_tests adds `the_reference_spells_every_glyph_a_card_cannot_hold` (45 lines, contains "$RH ⍴", "$NA ⍲", "$LB [").
       - 6 tests pass.
   - **components/u74/Cargo.toml:** members include apl-u74-code and apl-u74-web.
   - **pages/deck.html** (new):
     - header (sw-apl link to index.html, "U '74", subtitle);
     - bar: #deck select, #prev, #where, #next, #view (3D/2D), #submit;
     - canvas#card width 1600; div#model; ol#cards; p.about; section#printout hidden > div#paper (green-bar CSS, 132ch min-width, APL385 font);
     - inline module that fetches version.txt and imports deck.js?v=&ts=.
   - **pages/deck.js:**
     - loads wasm `./u74/apl_u74_web.js` and init with `{module_or_path}`;
     - decks from wasm; picker(), listCards(), show(i) (drawCard plus model?.show, aria-current, scrollIntoView);
     - views() (localStorage "u74-view", lazy-imports model029.js);
     - submit() (renders lines as divs into #paper and unhides it);
     - arrow keys navigate.
   - **pages/card029.js:** `export function drawCard(canvas, text, holes)`.
     - ASPECT 2.269; the layout is from the ibm-1130-rs keypunch.rs (left margin 2.5%, top 4.5%).
     - It draws the stock with a corner cut, #f4e8d0; the digit rows; the punches (#1c1a17, 0.6col × 0.7row); the printed characters at the top. Helper `center()`.
   - **pages/model029.js:** `export async function model029(host)`.
     - Loads the glb; unlit materials.
     - CARD="static_029_punched_card_print001" gets a CanvasTexture with flipY=false; orientation verified correct.
     - Views: card (from [0, 0.02, 0.34]) and whole (target (0, 0.75, 0), from [0.6, 0.5, 2.3]); zoom buttons "Card" and "029".
     - Returns {show(text, holes), resize}.
   - **pages/machines/029.glb:** copied from the candidate render (786 KB).
   - **pages/u74/apl_u74_web.js and _bg.wasm:** build output.
   - **pages/demo.html:** inline script after <title> runs location.replace("deck.html") when mode=U.
   - **justfile pages recipe:** builds apl-u74-web to pages/u74 and removes .gitignore and *.d.ts.
   - **scripts/stamp-pages.sh:** adds deck.html, deck.js, card029.js, model029.js, machines/029.glb, and the u74 wasm files.
   - **pages/sw.js SHELL:** adds deck.html, deck.js, card029.js, and the u74 js and wasm.
   - **Docs:**
     - docs/mode-u.md: "## The page" section; also fixed the missing blank line before "## The first deck".
     - docs/citations.md: ibm-1130-rs keypunch.rs layout; console-3D 029 glb at pinned commits (console-3D a312163, X_eTaL-extensions 0f93e06).
     - docs/architecture.md: deck.html and apl-u74-web.
     - docs/parity.md: two U rows (card holes; the page).
     - docs/plan.md: the owner's reorder note.
     - docs/apl-timeline.md: the Maryland row is live; the D'83 row was fixed.
   - **data/timeline.json:** UMD entry has href "demo.html?mode=U", demo "U'74 · my live demo", tip "My live demo of University of Maryland APL, from punched cards".
   - **components/timeline/crates/apl-timeline/tests/timeline_tests.rs:** test renamed to `the_five_demos_are_named_link_their_modes_and_say_so`. Names are ["A'70","U'74","B'75","C'80","D'83"], and each href must be demo.html?mode=<first letter of demo>.
   - **scripts/check-pages.mjs:**
     - timeline links now list five demos; "two pictures say Future demo";
     - new U section: deck page, next card, 029 3D canvas, Submit printout (banner, "1 3", END OF RUN), nothing missing. All passed.

4. Errors and fixes:
   - **Tests took about 3 hours:** 773k stale .o files in target/debug/deps. Moved target to the scratchpad and rm -rf'd it, so a cold test run takes 4m16s. Gates then failed on a missing release binary; `just release` fixed that.
   - **xext scratchpad clone was broken (no .git):** removed and recloned.
   - **Read of console-3D images was blocked:** copied a sips-resized preview into the repo temporarily, viewed it, then deleted it.
   - **web_tests had a wrong bitmask for '$':** fixed to 0b0100_0010_0010.
   - **"Typos" deck ends with NO CARD LEFT TO READ:** the test now accepts END_OF_RUN or NO_INPUT.
   - **'0' in SPECIALS:** folded into the digit arm.
   - **Spelling "centre":** changed to center in card029.js.
   - **just check-pages failed:** it rebuilds pages without the python shim; reran with PATH set.
   - **First Pages deploy failed** (artifact "github-pages-1" not found) because two pushes landed 2 s apart; the second deploy succeeded. Later pushes were done as a single push.
   - **Earlier session:** never use the system python3.

5. Problem Solving:
   - 034, 035 and 036 (u74-demos) are complete, pushed and live. The live build is 0b4d031, verified.
   - check-pages: all passed. reg-rs: 119/119. Gates: 618 passed, 3 warnings.
   - agentrail warns that 29 session logs (406 MiB) are inside .agentrail (`agentrail sessions migrate`); I told the user.

6. All user messages:
   - "continue" (after the summary)
   - "status"
   - "iTerm updated"
   - "it should not take hours; diagnose it"
   - "go"
   - "does it make sense to move the X_eTaL pin?  what is X_eTaL used for and what do the many recent changes mean (their impact)?"
   - "local preview URL?"
   - "local preview url, when, where?"
   - "eta until live demo and sw-apl timeline updated?"
   - "continue"
   - "when will u'74 be live on gh?" (sent twice)
   - "go"
   - "when will deploy complete?"
   - "what time is it now UTC?  PT is 14:44"
   - "deploy should be now-ish not an hour away, right?"
   - "the timeline says Future Demo"

7. Pending Tasks:
   - Step 037-u74-keypunch (current, in progress): the virtual 029 keyboard, the $ reference, the "A FEW MOMENTS LATER..." title card, the printout side by side with the deck, repunch/insert/remove/restore, resubmit; then publish.
   - Then 038-d83-ap210.
   - Optional, offered and not requested: a target pruning recipe step; a candidate 029 rebuild at X_eTaL HEAD; `agentrail sessions migrate`.

8. Current Work:
   - After 036-u74-demos went live, I restored the stashed keypunch WIP (`git stash pop`). It holds punch.rs with reference(), spellings() in wasm.rs, the mnemonic dep and the new test; tests passed.
   - The step is 037-u74-keypunch. Agentrail shows it advanced as current; it was begun before the reorder. If agentrail requires it, run `agentrail begin`.
   - Planned design (as told to the user):
     - The keyboard follows console-3D models/029-keyboard.json. Copy it into pages (e.g., pages/keys029.json) and cite it.
     - Each key punches its lower legend, or its upper legend with NUMERIC held.
     - Legends we can't punch (¢, †, 0–8–2) are dimmed; | " _ come from the computer keyboard.
     - REL or Enter releases to the next card; FEED inserts a blank card; DUP copies the column from the previous card.
     - Backspace steps back a column (our convenience). Left/Right move the column; Up/Down/PageUp/PageDown move between cards.
     - Insert, remove and restore per deck; edits are saved in localStorage per deck.
     - The $ reference table comes from wasm spellings(), with names from glyph-names.json.
     - Submit shows our own "A FEW MOMENTS LATER..." title card, then the printout.
     - The deck and printout sit side by side on wide screens.
     - Clicking "CARD n COLUMN c" in the printout selects that card and column.
     - drawCard gets an optional current-column highlight.
   - The deck-editing logic goes in a pure JS module (e.g., pages/punch029.js) with a node check scripts/check-punch.mjs, added to the `just test` loop (`for check in lines carriage printer` in the justfile), written TDD-first.

9. Optional Next Step:
   Continue step 037-u74-keypunch. Write the failing node check scripts/check-punch.mjs for a pure pages/punch029.js covering: punching at a column and advancing, Backspace, release to the next card, inserting and removing cards, DUP, numeric shift, and refusing unpunchable characters. Then implement it, wire the keyboard, reference, title card and side-by-side UI into deck.html/deck.js, extend check-pages, run the gates, commit, complete, publish with a single push, and report.
   - The user's last directive on this work was "go" (start 036, the keypunch).
   - My last statement: "Restoring the stashed keypunch work for the next step".

If you need specific details from before compaction (like exact code snippets, error messages, or content you generated), read the full transcript at: /Users/mike/.claude/projects/-Users-mike-github-sw-vibe-coding-sw-apl/837bcc12-de7d-448a-a103-977a7ae14f58.jsonl
Continue the conversation from where it left off without asking the user any further questions. Resume directly — do not acknowledge the summary, do not recap what was happening, do not preface with "I'll continue" or similar. Pick up the last task as if the break never happened.

The messages after this summary are the most recent messages from before compaction, kept verbatim. The summary was written without seeing them, so something it says has not happened yet may already have happened in them.

go

---

**Assistant** · 2026-11-27T02:38:59.272Z

I'm building the keypunch for step 037. First up is a pure JS module that edits the deck, with a node check written before the code; after that, the keyboard and printout UI.

Next I'll add the column marker to the card drawing and copy in the keyboard layout.

Now the browser check for the keypunch, written before the UI:

Recording your new direction in the plan and queuing it as a step after the keypunch:

Back on the keypunch: the 029 keyboard module.

Now the page controller:

---

**User** · 2026-11-27T04:21:33.645Z

can we reuse the sound from the 2741 (a'70) demo to provide a first-pass at sound when key punching?  The thud (CR) can be reused for when a card "ejects" to the left and up.

---

**Assistant** · 2026-11-27T04:21:55.507Z

Wiring the 2741's sounds into the keypunch: a strike per column punched, and the carriage-return thud when a card ejects (it'll slide left and up on the page). Writing the controller now.

---

**User** · 2026-11-27T04:22:29.876Z

assuming we have a mode where one can punch the UMD symbols and create a deck from scractch.  Would be nice to have helpers to type control-cards or idioms.

---

**Assistant** · 2026-11-27T04:22:45.648Z

I've stopped. The last edit didn't go through: the slide-out animation for an ejected card, and passing the punch column to the 3D view.

**Helpers for a deck from scratch.** This fits the keypunch well. Here is what I'd add:
- **A "New deck" entry in the picker.** It opens with the control cards already punched: `@RUN` with placeholder user and account fields, `@APL,I`, a blank card, then `@FIN`. The punch starts on the blank card.
- **Control-card buttons** for `@RUN`, `@APL,I` and `@FIN`. Each feeds a new card after the current one with that card punched.
- **An idioms menu** that punches a whole card at the punch, in `$` spellings, each with a short label. For example:
  - assignment, `X $AL `
  - the start of a function, `$DL Z $AL F X` ... `$DL`
  - a branch, `$RA `
  - a shaped table, `M $AL 3 4 $RH $IO 12`
  - output, `$QD $AL `
  - a comment, `$CM `
- **The `$` reference table punches when you click it.** Clicking `$RH ⍴` punches `$RH` at the column, so you don't have to remember the codes.

Each helper puts real punches on the card, column by column, with the key sound for each column. The card comes out exactly as if you'd typed it.

**Your other two asks:**
- **More decks (life, plotting, and so on).** I searched again and still found no UMD APL or APL 1100 manual online. Bitsavers has only Sperry sales literature and Datapro summaries that name APL 1100 ([sales manual](https://www.bitsavers.org/pdf/univac/1100/U7265_Sperry_Univac_Series_1100_Sales_Information_Manual_Jul82.pdf), [Datapro 1100/60](https://www.bitsavers.org/pdf/univac/1100/datapro/70C-877-12_8112_UNIVAC_1100_60.pdf)). So the decks would be our own, in the style of the APL demos of the time. I've recorded your direction in `docs/plan.md` and queued step 038, `u74-more-decks`, after the keypunch. It covers:
  - life on a small grid that wraps around;
  - a character plot of a function;
  - primes;
  - simple statistics;
  - a sort.

  Each deck gets a regression transcript and an entry in the picker.
- **Sound.** Done in the code but not yet run or tested. It reuses the 2741 demo's samples (`pages/sound.js`): a key strike for each column punched, the space-bar sound for blanks, and the carriage-return thud when a card ejects. A Sound checkbox stays on or off between visits.

**Where step 037 stands.** None of this has been run in a browser or committed yet:
- The deck-editing module passes its node check.
- The 029 keyboard, the printout lines that jump to a card, the title card and the `$` reference are written.
- The browser checks are written but not run, because the `deck.html` markup they need isn't done yet.

Should the from-scratch helpers go into 037, or become their own step after it (`u74-scratch-helpers`, before the extra decks)? I'd make them their own step: 037 is already large, and that keeps it shippable. Say "go" and I'll resume 037 where it stopped.

---

**User** · 2026-11-27T04:25:18.705Z

Did the UMD APL on UNIVAC have access to disk-stored "workspaces"?  Could a deck specify the "inclusion" of such a workspace (like COPY or PCOPY)?  We should emulate this only if it was actually used in practice in general (and can we tell if it was used specifically at SUNY Albany ca. 1976)

---

**Assistant** · 2026-11-27T04:25:52.794Z

**Short answer:** almost certainly yes, workspaces existed. Whether a deck could include a saved workspace, as `)COPY` or `)PCOPY` would, is something no source I can find says. And nothing at all survives about how it was used at SUNY Albany. So I'd keep U as it is, with no workspaces.

**What the evidence shows**
- **Workspaces existed.** Minker's Maryland history is our one primary source. It says APL 1100 users could change their workspace size while running, from 16K to 256K bytes ([Minker, p. 38](http://prism.cs.umd.edu/papers/M02:facultytimeline/history_new.pdf)). That describes an interactive APL with active workspaces in the APL\360 style.
- **It was sold as interactive.** Datapro calls APL 1100 an interpretive processor that runs under the 1100's Conversational Time-Sharing System ([Datapro 1100/60](https://www.bitsavers.org/pdf/univac/1100/datapro/70C-877-12_8112_UNIVAC_1100_60.pdf)). A time-sharing APL of 1973–74 would normally have had `)SAVE`, `)LOAD` and `)COPY`, kept as files on the operating system's disk. That's a reasonable inference, but no document says it.
- **Saved workspaces in batch decks: no evidence.** The operating system let batch runs assign cataloged disk files, and Datapro describes local and remote batch alongside demand mode ([Datapro 1100 series](https://www.bitsavers.org/pdf/univac/1100/datapro/70C-877-11_7812_UNIVAC_1100.pdf)). So a deck that assigned a saved-workspace file and then ran `)LOAD` or `)COPY` was technically possible. No source says APL 1100 supported it, and none says anyone did it.
- **Nothing on SUNY Albany survives in what I can reach.** The only record of APL there in 1976 is your memory, and that is a run of self-contained decks.
- **No manual is online.** Nothing UMD or Sperry published about APL 1100 is online beyond sales literature. The `)SAVE`/`)LOAD` in the Gemini notes is an LLM's reconstruction, not a source.

**Recommendation:** don't emulate it. Your rule was to emulate only what was actually common practice, and that isn't established. A student account in batch, with a disk quota and cards as the medium, makes self-contained decks the likely norm, and that's what U already does: a deck builds the workspace from a clear one. `docs/mode-u.md` already records that decision. I can add a line there giving this reasoning and listing the evidence we're missing.

**What could change this:**
- your own memory: did anyone at Albany keep APL on disk, or use terminals for APL?
- an APL 1100 programmer's reference, if a scan turns up; I can keep checking bitsavers
- UMD Computer Science Center records
- SUNY Albany computing-center newsletters from 1975–77, which often listed how to use each language processor

Sources:
- [Minker, Beginning of Computing and Computer Science at the University of Maryland](http://prism.cs.umd.edu/papers/M02:facultytimeline/history_new.pdf)
- [Datapro 70C-877-12, Sperry Univac 1100/60](https://www.bitsavers.org/pdf/univac/1100/datapro/70C-877-12_8112_UNIVAC_1100_60.pdf)
- [Datapro 70C-877-11, UNIVAC 1100 Series](https://www.bitsavers.org/pdf/univac/1100/datapro/70C-877-11_7812_UNIVAC_1100.pdf)
- [Sperry Univac Series 1100 Sales Information Manual, 1982](https://www.bitsavers.org/pdf/univac/1100/U7265_Sperry_Univac_Series_1100_Sales_Information_Manual_Jul82.pdf)

Step 037 is still paused where you stopped it, and nothing is committed. Say "go" to resume. Also tell me whether the from-scratch helpers (control-card buttons and idioms) go into 037 or get their own step; I recommend their own step.

---

**User** · 2026-11-27T04:27:12.871Z

go, helpers as their own step; skip any work related to UNIVAC disks.  Our demo is batch card deck only.

---

**Assistant** · 2026-11-27T04:27:22.732Z

Now redoing the edit that was stopped:

Now the page markup and styles:

Next, registering the new files in the build stamp, the offline cache and the test loop:

Running the build and the browser checks:

The browser checks are running now.

**U '74 today, live at sw-apl.softwarewrighter.com:**
1. **Open it.** The University of Maryland entry on the timeline ("U'74 · my live demo") opens the deck page, and so does `demo.html?mode=U`.
2. **Pick a deck.** Three are included:
   - Domino: a 2-by-2 system solved with domino, which prints `1 3`.
   - A program: a function defined with `$DL` cards and then called.
   - Typos: a deck with deliberate errors, to show what the printout says about them.
3. **Look through the cards.**
   - In 2D: a cream 029 card with its holes from the real IBM card code, and the punched text printed along the top.
   - In 3D: the card standing in the punch station of your 029 model, with Card and 029 zoom buttons and orbit controls.
   - Move with the arrow buttons or by clicking a card in the deck list. The page remembers whether you last used 2D or 3D.
4. **Submit the deck and read the printout.** The real interpreter runs the deck as a batch job, compiled to WebAssembly. The printout comes back on green-bar paper and shows:
   - the UMD banner and CLEAR WS;
   - each card as punched, with its sequence number;
   - the statement in APL glyphs beneath it;
   - the results;
   - any error in APL\360's words, with a caret, the card and the column;
   - END OF RUN.

**Step 037, in progress (being checked now):**
1. **Punch cards.** Use the on-screen 029 keyboard, laid out from IBM's combination-keyboard figure, or your computer's keyboard.
   - A key punches its lower legend, or its upper legend after NUMERIC.
   - Keys with no code of ours are dimmed.
   - Lowercase letters punch as capitals.
   - The current column is shaded on the card, in 2D and 3D, and shown as "Column n".
2. **Use the 029's functions:**
   - REL or Enter ejects the card, which slides off left and up, and moves to the next one;
   - FEED inserts a blank card;
   - DUP copies the column from the card before;
   - SKIP moves on a column;
   - Backspace and the arrows move the punch;
   - Up and Down move between cards.
3. **Edit the deck:**
   - feed a blank card in, or remove a card;
   - Restore puts the deck back as it came;
   - edits are kept in your browser per deck, marked "Punched here".
4. **Find a spelling.** The reference lists the `$` spelling for each APL character, with its name on hover.
5. **Submit and repunch.** Submit shows our own "A FEW MOMENTS LATER..." title card, then the printout. On a wide screen the printout sits beside the deck. Clicking a listed card, or an error's CARD n COLUMN c, takes you to that card and column so you can fix it and resubmit.
6. **Hear it.** The 2741's sounds stand in for the 029's: a key strike for each column, the space-bar sound for a blank, and the thud when a card ejects. A Sound checkbox stays on or off between visits.

**Queued:**
- **038, scratch helpers:**
  - a New deck that starts with its control cards;
  - buttons for `@RUN`, `@APL,I` and `@FIN`;
  - an idioms menu that punches a whole card;
  - clicking a `$` spelling in the reference punches it.
- **039, more decks:** life, a character plot, primes, statistics and a sort.

**Not supported, by design:** disks, saved workspaces, `)` commands, terminals and the APL keyboard. A U run is a self-contained deck, read once in a batch job.

---

**User** · 2026-11-27T15:36:28.345Z

<task-notification>
<task-id>bnacgmyv0</task-id>
<tool-use-id>toolu_01B4acFgim5M7iac5Y5pd7R4</tool-use-id>
<output-file>/private/tmp/claude-501/-Users-mike-github-sw-vibe-coding-sw-apl/837bcc12-de7d-448a-a103-977a7ae14f58/tasks/bnacgmyv0.output</output-file>
<status>completed</status>
<summary>Background command "Syntax check and run check-pages" completed (exit code 0)</summary>
</task-notification>

---

**Assistant** · 2026-11-27T16:17:19.592Z

Most of the U checks failed together, which points to a load error. I'm probing the page in headless Chrome to find it.

All browser checks pass. Next, a screenshot to see how the page actually looks:

The page looks right: the shaded column on the card, the 029 keyboard with its dimmed legends, and the printout beside the deck. Starting the Rust gates in the background while I update the docs.

Gates pass except one new warning: `wasm.rs` has 5 functions, over the budget of 4. I'll fold `decks()` and `deck(i)` into a single `decks()` that returns every deck, separated by form feeds.

Writing the test first:

Green. Rerunning the full gate chain: fmt, clippy, tests, browser checks with the WebAssembly rebuilt, and gates.

Every gate passes, with only the 3 expected Binary Freshness warnings. Spelling check, then the commit:
