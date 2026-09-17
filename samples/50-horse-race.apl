⍝ Horse race, written in pure APL\360 (target program; see docs/parity.md).
⍝ Names live in a character matrix, output mixes text and numbers with
⍝ semicolons, branching uses the →LABEL×⍳COND idiom. No ⍕, ⊃, ⎕DL.
HORSES←5 7⍴'LUCKY  THUNDERSHADOW COMET  BLAZE  '
∇SHOW;I
I←1
N:HORSES[I;],'│',(POS[I]⍴'░'),'▓'
I←I+1
→N×⍳I≤5
∇
∇RACE;POS;ROUND
POS←5⍴0
ROUND←0
'THE RACE IS ON!'
LOOP:ROUND←ROUND+1
'--- ROUND ';ROUND;' ---'
POS←POS+?5⍴3
SHOW
→LOOP×⍳~∨/POS≥15
'WINNER: ',HORSES[(POS=⌈/POS)⍳1;]
∇
RACE
)OFF
