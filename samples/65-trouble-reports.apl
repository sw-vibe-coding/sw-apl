⍝ What a system command says when it cannot do what was asked. The
⍝ APL\360 User's Manual numbers these "trouble report forms", and the
⍝ distinction worth knowing is that INCORRECT COMMAND is about the
⍝ command, not about the workspace.
⍝ A command given an argument it does not take: report 16.
)LOAD
)DROP
)LIB TWO
)LIB 1 2
⍝ A workspace that is not stored: report 7. The command was fine.
)LOAD NOSUCH
)LOAD 1 NOSUCH
)COPY NOSUCH
)DROP NOSUCH
⍝ A workspace that is there but does not hold the name asked for:
⍝ report 9. EDIT is in library 1 and holds MEAN, but not MEDIAN.
)COPY 1 EDIT MEDIAN
⍝ Asking for one name it does not hold refuses the lot, so a copy
⍝ that half worked cannot be mistaken for one that worked.
)COPY 1 EDIT MEAN MEDIAN
)FNS
⍝ The name it does hold copies.
)COPY 1 EDIT MEAN
MEAN 1 2 3 4
⍝ A number that names no library: report 14. An empty library is a
⍝ different thing and says nothing.
)LIB 9
)LOAD 9 EDIT
⍝ Report 13, NOT SAVED, THIS WS IS name, is the one report this
⍝ sample cannot show: it needs a workspace stored in library 0,
⍝ which is yours, and a sample must not write there. The session
⍝ tests cover it in a directory of their own.
)OFF
