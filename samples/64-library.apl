⍝ The numbered libraries. Library 0 is yours, where )SAVE writes;
⍝ library 1 holds the workspaces sw-apl ships. )LIB 1 lists one and
⍝ )LOAD 1 NAME reads from it.
)LIB 1
⍝ )LOAD says when the workspace was saved and nothing else, as
⍝ APL\360 did. Typing DESCRIBE is the reader's move, not the
⍝ loader's; the convention is the name.
)LOAD 1 EDIT
DESCRIBE
)FNS
)VARS
⍝ EDIT exists to be edited. FACT is wrong by one, which is a line to
⍝ find and a line to change.
FACT 5
∇FACT[⎕]∇
⍝ Line 5 branches back while I is less than N, so the last multiply
⍝ never happens. Reopen, write line 5 again, close.
∇FACT
[5] →LOOP×⍳I≤N
∇
FACT 5
⍝ The fix is this session's, not the library's: )SAVE would write it
⍝ into library 0, and the shipped workspace is untouched.
∇FACT[⎕]∇
⍝ A library workspace is a plain text file you can read in an
⍝ editor, so nothing here is hidden.
)OFF
