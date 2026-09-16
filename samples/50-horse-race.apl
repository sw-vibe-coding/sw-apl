⍝ Horse race, written in pure APL\360 (target program; see docs/parity.md).
⍝ Names live in a character matrix, output mixes text and numbers with
⍝ semicolons, branching uses the →LABEL×⍳COND idiom. No ⍕, ⊃, ⎕DL.
HORSES←5 7⍴'LUCKY  THUNDERSHADOW COMET  BLAZE  '
∇SHOW;I
[1] I←1
[2] N:HORSES[I;],'│',(POS[I]⍴'░'),'▓'
[3] I←I+1
[4] →N×⍳I≤5
∇
∇RACE;POS;ROUND
[1] POS←5⍴0
[2] ROUND←0
[3] 'THE RACE IS ON!'
[4] LOOP:ROUND←ROUND+1
[5] '--- ROUND ';ROUND;' ---'
[6] POS←POS+?5⍴3
[7] SHOW
[8] →LOOP×⍳~∨/POS≥15
[9] 'WINNER: ',HORSES[(POS=⌈/POS)⍳1;]
∇
RACE
)OFF
