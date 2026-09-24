# modes

## Phase 9 (owner direction 2026-09-21): modes, '68 and '72

The owner, after BIRDS: sw-apl gains modes. The header already shows
the current one as a tab, `Ⓐ '68`, built as a row holding one tab so
another could be added; the second is `Ⓑ '72`, APLSV. This reverses
the non-goal that kept APLSV out, for APLSV only.

What a mode is:

- A language. '68 is APL\360 exactly as sw-apl has it; '72 is APLSV.
- A pair of libraries. Library 1 differs per mode -- '72 gets its own
  workspaces, and BIRDS there can fly the birds that `NOTHERE` says
  need an execute. Library 0 differs per mode too: a workspace saved
  in '72 is saved in '72's library 0, and `)LIB` in '68 never sees
  it.
- Recorded in what it saves, so a '72 workspace cannot be loaded into
  '68 as though it were one.

How the implementation is arranged, which the owner settled: three
parts, visible in the code and not only in checks.

- **Shared.** What exists today is the shared core, taken as it
  stands. APLSV is very nearly a superset of APL\360, so it is most of
  the code, and it does not move to make room.
- **'68-only.** What APLSV dropped is pulled out of the shared core
  into crates of its own, so that it is plainly APL\360's and '72 does
  not carry it. The list is short and is established from the APLSV
  manual, not from memory -- the I-beams, and perhaps the `)ORIGIN`,
  `)DIGITS` and `)WIDTH` commands, are candidates only until then.
- **'72-only.** What APLSV adds -- execute, format, the quad system
  variables and functions -- is new code in new crates.

A profile on the workspace, set by the host, decides which of the
mode-only parts are reachable, and it is read at the few places the
two differ -- from data where it can be, as the glyph table's
`[[later]]` entries already are.

The '68 mode must not move. Every existing sample is a '68 sample and
reg-rs is the proof: a step that changes a '68 transcript is wrong,
not rebased.

Sources: the APL\360 User's Manual has been the reference for
everything so far. '72 needs its own -- the APLSV manual -- and the
first '72 step finds it and records what differs, before any '72
behaviour is built. Where the manual is not to hand, a guess is
labelled as one.

Not in this phase: shared variables (see non-goals), and APL2 -- the
owner mentioned a '84 tab, but where APL2 is to live is not settled,
and nothing is planned for it here.

Order:

1. The iota bug carried from Phase 8, first: `⍳⍴A` is a RANK ERROR,
   and correctness comes before features.
2. Scope: CLAUDE.md and prd.md say what sw-apl now is. The README and
   the user docs wait until '72 is something a reader can use.
3. The APLSV sources, and a record of how '72 differs from '68.
4. The profile, per-mode libraries, and the mode in a saved
   workspace, with '68 unchanged.
5. The '68-only parts the sources name, pulled out of the shared core
   into '68-only crates, still with '68 unchanged.
6. The glyph table gains the mode a glyph arrives in; execute.
7. Format.
8. The quad system variables.
9. The quad system functions.
10. Library 1 for '72, BIRDS first.
11. The docs, the README, and the `Ⓑ '72` tab.

Then the Phase 8 steps still pending: cup and cap, the base
conversion sample, the Linux regression fixtures, and the offline
shell.

