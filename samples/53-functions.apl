⍝ Defined functions: the del header forms, locals, dynamic scoping.
⍝ Branching is not part of this file, so every body runs straight
⍝ through from line 1 to the last line.
∇R←DOUBLE N
R←N+N
∇
DOUBLE 21
DOUBLE DOUBLE 3
2×DOUBLE 1+2
∇R←A HYP B
R←((A*2)+B*2)*0.5
∇
3 HYP 4
5 HYP 12
∇R←TEN
R←10
∇
TEN
TEN+TEN
⍝ A function with no result is a statement, never a value.
∇GREET
'HELLO FROM APL'
∇
GREET
1+GREET
⍝ Locals named after the semicolons shadow globals while the call
⍝ runs, and the globals come back when it returns.
T←99
G←7
∇R←SUMTO N;T
T←⍳N
R←G++/T
∇
SUMTO 4
T
⍝ The valence written must be the valence the header declared.
2 DOUBLE 3
DOUBLE
⍝ Arguments and locals are gone once the call returns.
N
⍝ Recursion is allowed; without a branch to stop it, the guard does.
∇R←DOWN N
R←DOWN N-1
∇
DOWN 1
)OFF
