Phase 8 (docs/plan.md). Del-tilde opens a definition as well as
closing one.

Found while writing the overstrike sample: `⍫R←SECRET` is a SYNTAX
ERROR. It should open a locked definition.

The manual, on locked functions:

  "If the symbol ⍫ (formed by a ∇ overstruck with a ~ and called
   del-tilde) is used instead of ∇ to open or close a function
   definition, the function becomes locked."

sw-apl accepts `⍫` to close, which is how every sample and test
writes a locked function, and not to open. The dispatch in
apl-session's commands.rs looks for `∇` alone; `⍫` never reaches
`open_definition`.

Decide what opening with `⍫` means for a definition that is then
closed with a plain `∇`: the manual's "used instead of ∇ to open or
close" reads as either being enough to lock it. Say which you
concluded and why.

Check the reopening case too: `⍫NAME` on an existing unlocked
function -- does it reopen and lock, or is that DEFN ERROR? The
manual says a locked function cannot be revised, which is about the
already-locked case, not this one.

TDD; the editor tests are in apl-session; a sample line if the
behaviour is worth showing. Update docs/parity.md,
docs/del-editor-guide.md and docs/workspaces.md, which all say
del-tilde closes.
