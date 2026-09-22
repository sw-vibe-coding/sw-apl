⍝ A local name hides every referent of the name while its function
⍝ runs: a global function as well as a global variable. The name is
⍝ given back when the function returns.
∇R←G X
R←X+1
∇
∇F;G
G←5
G
∇
F
G 1
⍝ Under a suspension, the listings show the global names: G is a
⍝ function to )FNS, and not a variable to )VARS, while the local
⍝ holds 5.
∇H;G
G←5
NOSUCH
∇
H
)FNS
)VARS
G
→
G 1
