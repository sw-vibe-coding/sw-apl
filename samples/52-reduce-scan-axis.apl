⍝ Reduce and scan on either axis, identities on empties
M←2 3⍴⍳6
+/M
+⌿M
+/[1]M
-/M
×\M
+⍀M
∨/0 0 1
∧/1 1 0
+/⍳0
×/⍳0
⌈/⍳0
⌊/⍳0
=/⍳0
-\1 2 3
)OFF
