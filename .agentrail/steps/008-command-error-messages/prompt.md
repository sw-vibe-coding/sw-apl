Phase 4 step 8 (docs/plan.md, owner report 2026-09-18). The system
commands answer INCORRECT COMMAND where they should say what is
actually wrong.

Reported:

      )LOAD RACE
INCORRECT COMMAND
      )LOAD 1 RAXE
INCORRECT COMMAND

Neither command is incorrect. The workspace is not there.

The trouble-report table in the IBM APL\360 User's Manual (Aug 1968)
numbers the replies, and the ones that matter here are:

  7   WS NOT FOUND
  9   OBJECT NOT FOUND
  14  IMPROPER LIBRARY REFERENCE
  16  INCORRECT COMMAND

INCORRECT COMMAND is for a command given an argument it does not
take -- `)LIB 9 9`, `)DROP` with no name, `)SYMBOLS 500`. A name
that is simply not there is a different answer.

Work through every command that can fail and give it the right one:
`)LOAD`, `)COPY`, `)PCOPY` and `)DROP` against a missing workspace;
`)COPY NAME OBJECT` against an object the workspace does not hold;
a library number that is not a library. `)DROP` currently replies
`NOT FOUND name`, which is ours rather than the manual's -- check it
against the table and change it or record why not.

Read the manual rather than recalling it: it is at
https://archive.org/stream/bitsavers_ibmaplAPL3_8068299/APL_360_Users_Manual_Aug68_djvu.txt
and the table is around line 4771 of that text. Quote what you find
in the commit, and say which replies the manual does not settle.

Watch for the reverse mistake too: a reply that is too specific is
as wrong as one that is too vague, and `)LIB 9` listing nothing
silently is its own small version of this bug.

TDD; reg-rs for anything run through the binary; update
docs/parity.md and docs/session.md in the same commit; commit, push,
report.
