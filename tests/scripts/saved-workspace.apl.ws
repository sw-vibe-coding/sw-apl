⍝ sw-apl workspace. Re-executable APL: loading it runs it.
⍝!MODES (A)(B)
⍝!SAVED 20.00.00 09/17/26
⍝!LINK 282475249
⍝!ORIGIN 0
⍝!DIGITS 3
⍝!WIDTH 80
)WSID CLASS
A←5
E←0⍴0
F←¯1.5
M←2 3⍴0 1 2 3 4 5
ONE←1⍴7
R←13
T←'IT''S'
V←1 2 3
∇R←A HYP B
R←((A*2)+B*2)*0.5
∇
∇R←SUM N;I
R←0
I←0
LOOP:I←I+1
R←R+I
→LOOP×I<N
∇
