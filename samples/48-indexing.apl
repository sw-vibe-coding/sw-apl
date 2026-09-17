⍝ Indexing and indexed assignment
V←10 20 30 40 50
V[2]
V[5 1]
V[2 2⍴1 2 3 4]
M←3 3⍴⍳9
M[2;3]
M[1 3;]
M[;2]
M[2;1 3]
'ABCDE'[3 1]
)ORIGIN 0
V[0]
)ORIGIN 1
V[6]
V[2]←99
V
M[2;]←0
M
M[1;1 2]←7 8
M
V[1 2 3]←1 2
)OFF
