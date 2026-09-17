⍝ The del editor. Definition mode prompts with the number the next
⍝ line will hold; a bracket moves about, displays, inserts, deletes,
⍝ or edits the header. Closing with ∇ renumbers the lines from 1.
∇R←AREA H
R←H
∇
AREA 3
⍝ Reopen it. The prompt starts after the last line.
∇AREA
R←H×W
∇
⍝ Line 2 wants W, so insert one before it. Fractional numbers make
⍝ room between lines that are already there.
∇AREA
[1.5] W←H+1
[⎕]
∇
AREA 3
⍝ The close renumbered them, so 1.5 is line 2 now.
∇AREA[⎕]∇
⍝ Replace a line by typing its number, and delete one with ∆.
⍝ [n⎕] shows from line n onward, without the header.
∇AREA
[2] W←H+10
[2⎕]
[∆3]
R←H×W
[⎕]
∇
AREA 3
⍝ [0] edits the header. Here it gives AREA a local and a new name.
∇AREA
[0] R←SIDES H;W
∇
∇SIDES[⎕]∇
SIDES 3
⍝ The old name is gone, not left behind as a copy.
AREA
⍝ A line number that is not there cannot be deleted.
∇SIDES
[∆9]
∇
⍝ Del-tilde closes a definition locked: it can no longer be reopened
⍝ or displayed.
∇SECRET
'THE ANSWER IS 42'
⍫
SECRET
∇SECRET
∇SECRET[⎕]∇
⍝ A header may not name something the workspace already holds.
∇R←SIDES H
)OFF
