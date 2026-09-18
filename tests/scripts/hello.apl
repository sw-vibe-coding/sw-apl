#!/usr/bin/env -S target/release/sw-apl --no-echo -f
⍝ An executable APL program. The shebang belongs to the shell: sw-apl
⍝ drops that first line, so the transcript starts with the program.
⍝ Only the first line, and only `#!`; a `#` anywhere else is a
⍝ CHARACTER ERROR, since it is not an APL\360 character.
'HELLO FROM AN EXECUTABLE APL FILE'
+/⍳10
)OFF
