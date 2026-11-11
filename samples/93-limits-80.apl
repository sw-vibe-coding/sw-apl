⍝!MODES (C)
⍝ (C) '80, after APL80 on the TRS-80: the core, with scan, six
⍝ digits in a line of 64, and what its manual's Limitations leave out.
)DIALECT
÷3
+\1 2 3 4
⍝ Not implemented: domino, lamination, the diagonal transpose and
⍝ the hyperbolic functions.
⌹2 2⍴4 0 0 2
1 2,[0.5]3 4
1 1⍉2 2⍴⍳4
5○1
1○0
⍝ Limited: factorial and binomial take whole numbers, encode and
⍝ decode take vectors.
!5
!2.5
10⊥1 2 3
10 10⊤2 2⍴⍳4
⍝ One specification a statement, at its left.
X←3
X←Y←9
÷X←4
X
)OFF
