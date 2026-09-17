⍝ Conway's Life in APL\360 style: a glider on a 6 by 6 torus.
⍝ The famous one-liner (life←{↑1 ⍵∨.∧3 4=+/,¯1 0 1∘.⊖¯1 0 1⌽¨⊂⍵})
⍝ needs dfns, each, and enclose, which are APL2. The APL\360 way is
⍝ eight explicit neighbour rotations; ⊖ rotates rows, ⌽ columns.
B←6 6⍴0
B[1;2]←1
B[2;3]←1
B[3;1 2 3]←1
B
N←(¯1⊖¯1⌽B)+(¯1⊖B)+(¯1⊖1⌽B)+(¯1⌽B)+(1⌽B)+(1⊖¯1⌽B)+(1⊖B)+(1⊖1⌽B)
N
B←(3=N)∨B∧2=N
B
N←(¯1⊖¯1⌽B)+(¯1⊖B)+(¯1⊖1⌽B)+(¯1⌽B)+(1⌽B)+(1⊖¯1⌽B)+(1⊖B)+(1⊖1⌽B)
B←(3=N)∨B∧2=N
B
N←(¯1⊖¯1⌽B)+(¯1⊖B)+(¯1⊖1⌽B)+(¯1⌽B)+(1⌽B)+(1⊖¯1⌽B)+(1⊖B)+(1⊖1⌽B)
B←(3=N)∨B∧2=N
B
N←(¯1⊖¯1⌽B)+(¯1⊖B)+(¯1⊖1⌽B)+(¯1⌽B)+(1⌽B)+(1⊖¯1⌽B)+(1⊖B)+(1⊖1⌽B)
B←(3=N)∨B∧2=N
⍝ after four generations the glider has moved one cell down and right
B
)OFF
