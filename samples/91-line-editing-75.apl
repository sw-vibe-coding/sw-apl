⍝!MODES (B)
⍝ Editing a line in place, as the 5110 does: [n⎕] puts line n,
⍝ number and all, in the input line to be typed over, and Return
⍝ enters it as it then stands. Here, in a batch file, there is
⍝ nothing to type over: the next line of the file is the whole line.
∇Z←AREA H
Z←H×H
[1⎕]
[1]   Z←H×H+1
∇
AREA 3
⍝ A changed number puts the line at the new number and leaves the
⍝ old one. [n⎕m] does the same as [n⎕]; the m is ignored.
∇AREA
[1⎕7]
[2]   Z←Z÷2
∇
∇AREA[⎕]∇
AREA 3
⍝ APL\360's slash line, with its marks, is (A)'s only.
