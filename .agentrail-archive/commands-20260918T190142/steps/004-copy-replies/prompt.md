Phase 6 step 4 (docs/plan.md). )COPY and )PCOPY say nothing and owe
two replies.

Found while writing docs/commands-reference.md. The manual gives
both a normal response and sw-apl prints neither.

For )COPY and )COPY of everything (WC3, WC3a):

  "SAVED, followed by the time of day and the date that the source
   workspace was last stored."

For )PCOPY (WC4, WC4a), the same, and then:

  "NOT COPIED:, followed by the names of objects not copied, will be
   printed if appropriate."

Today:

      )COPY DEMO
      )PCOPY DEMO A

Both silent, whether they copied everything, something or nothing.
A protected copy that quietly skipped the name you asked for is the
worse of the two: nothing distinguishes it from one that worked.

The SAVED line is the one )LOAD already prints, from the file's
`⍝!SAVED` directive, so the moment is there to be read -- see
apl-commands' load(). The NOT COPIED list is the names )PCOPY
skipped because the active workspace already held them, which the
copy loop already knows: it computes `kept` and then does nothing
with the fact.

Note the punctuation rule the manual follows and this repo now
keeps: a comma introduces a reason, a colon introduces a list. NOT
COPIED: is a list.

Decide what an ordinary )COPY that skips nothing prints -- the
manual gives it only the SAVED line -- and whether )COPY of a name
the workspace does not hold still reaches here at all, given that
OBJECT NOT FOUND is now raised first.

TDD; the session command tests already cover copying, so extend
them rather than starting a new file; reg-rs for sample 61, which
shows )COPY and will change; update docs/parity.md and
docs/commands-reference.md in the same commit.
