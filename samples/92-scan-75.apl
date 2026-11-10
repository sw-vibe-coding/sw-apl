⍝!MODES (B)
⍝ Scan, in (B) '75: every reduction along the way, on either axis.
⍝ The 5100 family has it from APLSV; (A) does not, sample 33.
+\1 2 3 4
×\1 2 3 4 5
⌈\3 1 4 1 5 9
⌊\9 5 1 4 3
∨\0 0 1 0 0
∧\1 1 1 0 1
-\1 2 3
M←2 3⍴⍳6
×\M
+⍀M
+\[1]M
)OFF
