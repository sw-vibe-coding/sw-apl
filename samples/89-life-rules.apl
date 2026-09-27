⍝ LIFE keeps Conway's rules, B3/S23: an empty cell with exactly 3
⍝ neighbours is born, a live cell with 2 or 3 survives, and every
⍝ other cell is empty next generation -- of loneliness with fewer
⍝ than 2, of overcrowding with more than 3. GEN counts the eight
⍝ neighbours by rotating the board, so the edges wrap: a torus.
)LOAD 1 LIFE
⍝ Each check is 1 when it holds. Boards are compared whole:
⍝ ∧/,A=B is 1 when every square of A equals B's.
⍝ A lone cell dies, and so does one with four neighbours.
GEN 3 3⍴0 0 0 0 1 0 0 0 0
(GEN 5 5⍴0 0 0 0 0 0 1 0 1 0 0 0 1 0 0 0 1 0 1 0 0 0 0 0 0)[3;3]
⍝ An empty cell with exactly three neighbours is born.
(GEN 3 3⍴1 1 1 0 0 0 0 0 0)[2;2]
⍝ A block, 2 by 2, is a still life: it never changes.
B←6 6⍴0
B[2 3;2 3]←1
∧/,B=GEN B
⍝ A blinker turns from a column to a row, and back again.
L←5 5⍴0
L[2 3 4;3]←1
GEN L
∧/,L=GEN GEN L
⍝ The same blinker lying across the edge still blinks: the board
⍝ wraps round.
E←5 5⍴0
E[5 1 2;1]←1
∧/,E=GEN GEN E
⍝ The glider on BOARD, after four generations, is the same shape
⍝ one row down and one column to the right.
G←BOARD
∧/,(GEN GEN GEN GEN G)=¯1⊖¯1⌽G
⍝ On a 6 by 6 torus that is back where it began after 24.
∇R←N GENS B
R←B
L:→(N=0)/0
R←GEN R
N←N-1
→L
∇
∧/,(24 GENS G)=G
)OFF
