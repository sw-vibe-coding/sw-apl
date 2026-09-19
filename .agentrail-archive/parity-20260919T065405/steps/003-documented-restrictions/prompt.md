Phase 7 step 3 (docs/plan.md, owner direction 2026-09-18). Say what
this implementation does not do.

Three rows in docs/parity.md are carried as `todo` and are not work
outstanding. They are what sw-apl is:

- A saved workspace does not keep a suspended function. APL\360's
  workspaces were binary images and `)SAVE` really did preserve a
  suspension. Ours are re-executable text, and re-executing a file
  cannot put execution back in the middle of a call. `)SAVE` leaves
  the state indicator out and `)LOAD` gives you a workspace with an
  empty one.
- WS LOCKED cannot arise. It meant a stored workspace was behind a
  password, and sw-apl has one user, no accounts and no shared
  library, so there is nothing to lock against.
- NOT SAVED, WS QUOTA USED UP cannot arise. It meant the account's
  library allocation was full. sw-apl saves into a directory; if the
  disk is full the operating system says so, and that is a different
  sentence.

Carrying these as `todo` says the wrong thing to whoever reads the
file next, including the next agent, who may set about implementing
one.

Give docs/parity.md a Restrictions section: what the restriction is,
why it follows from a decision already made, and where that decision
is written down (design.md D7 for the text workspace format, and the
out-of-scope rule at the top of parity.md for the multi-user
surface). Move the rows there and out of the tables.

Check the rest of the file for the same confusion before finishing:
a row marked `todo` that is really a restriction, or a row that
quietly became done. `grep -c "| todo" docs/parity.md` should end at
a number you can name every member of.

Then make the parity summary at the top of the file true: it is the
first thing anyone reads and it should say how many rows are open,
how many are restrictions, and that nothing is unaccounted for.

This step writes no code. If it turns up something that is a real
gap after all, say so and add a step rather than widening this one.
