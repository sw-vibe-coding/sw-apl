⍝ A line inside a function that fails does not unwind it. The error
⍝ names the function and the line, the function stays suspended with
⍝ its locals intact, and )SI shows where everything stopped. A bare →
⍝ clears the top entry; → with a line number takes it up again.
∇R←SCALE N;F
F←N×FACTOR
R←F+1
∇
⍝ FACTOR is not defined, so SCALE stops on line 1.
SCALE 5
)SI
⍝ The argument and the locals are still there to look at.
N
)SIV
⍝ Supply what was missing and take it up at the line that failed.
FACTOR←10
→1
)SI
⍝ A caller waiting on a function that stopped is pendent, not
⍝ suspended: only the innermost carries the star.
∇INNER;Q
Q←1
Q←Q÷0
∇
∇OUTER;P
P←2
INNER
'NEVER REACHED'
∇
OUTER
)SI
)SIV
P
⍝ Clearing the top takes the pendent caller with it, since it has
⍝ nowhere to go back to.
→
)SI
P
)OFF
