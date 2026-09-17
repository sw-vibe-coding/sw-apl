⍝ Conway's Life as defined functions: a glider on a 6 by 6 torus.
⍝ GEN counts the eight neighbours and applies the birth and survival
⍝ rules; RUN steps N generations, displaying each. Still pure APL\360:
⍝ eight explicit rotations, a label, and the →LABEL×⍳COND idiom.
∇R←GEN B;N
N←(¯1⊖¯1⌽B)+(¯1⊖B)+(¯1⊖1⌽B)+(¯1⌽B)+(1⌽B)+(1⊖¯1⌽B)+(1⊖B)+(1⊖1⌽B)
R←(3=N)∨B∧2=N
∇
∇RUN N;G
G←0
LOOP:'GENERATION ';G
BOARD
→0×⍳G≥N
BOARD←GEN BOARD
G←G+1
→LOOP
∇
BOARD←6 6⍴0
BOARD[1;2]←1
BOARD[2;3]←1
BOARD[3;1 2 3]←1
RUN 4
⍝ After four generations the glider has moved one cell down and right.
⍝ GEN is a function now, so one generation is one call.
BOARD←GEN BOARD
BOARD
)OFF
