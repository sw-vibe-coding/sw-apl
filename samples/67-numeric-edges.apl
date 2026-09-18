⍝ The numeric edges. Everything is a double, as it was on a 360: the
⍝ manual says )DIGITS "has no effect on the precision of internal
⍝ calculations, which is approximately 16 decimal digits".
⍝ The fuzz. The manual: "For operations such as floor and ceiling,
⍝ and in comparisons, a fuzz of about 1E¯13 is applied in order to
⍝ avoid anomalous results that might otherwise be engendered by
⍝ doing decimal arithmetic on a binary machine."
⍝ A tenth and a fifth do not add to exactly three tenths in binary.
(0.1+0.2)=0.3
⌊(0.1+0.2)×10
⍝ Counting is such an operation too, so a length that is a whole
⍝ number within the fuzz counts.
⍳(0.1+0.2)×10
((0.1+0.2)×10)⍴7
⍝ The fuzz is narrow. Half is not nearly a whole number.
⍳2.5
⍝ Where a number stops being exact: two to the fifty-third is the
⍝ last integer a double holds, so beyond it consecutive integers are
⍝ the same number.
)DIGITS 16
((2*53)-1)=(2*53)-2
(2*53)=(2*53)+1
⍝ )DIGITS caps how much is shown, not what is held. Sixteen digits
⍝ shows the whole of this; ten does not, and the number is written
⍝ in exponential form instead.
10000000000
)DIGITS 10
10000000000
⍝ It is the same number either way.
10000000000=1E10
⍝ One digit rounds hard, and the value is untouched by the setting.
)DIGITS 1
1÷3
3×1÷3
)DIGITS 10
⍝ Exponential form on the way in and on the way out, and only where
⍝ it is needed: a small exponent still prints in full.
1E¯10
1.5E300
¯1.5E¯5
⍝ What a workspace holds is bounded and what an expression builds is
⍝ not, so counting to two hundred thousand is fine and keeping it is
⍝ not. See docs/workspaces.md.
⍴⍳200000
+/⍳200000
BIG←⍳200000
)OFF
