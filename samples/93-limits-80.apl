⍝!MODES (C)
⍝ (C) '80, after APL80 on the TRS-80, typed in APL80's characters:
⍝ the core, with scan, six digits in a line of 64, and what its
⍝ manual's Limitations leave out.
)DIALECT
%3
+w1 2 3 4
⍝ Domino has no character in APL80, so it cannot be typed.
⌹2 2p4 0 0 2
⍝ Not implemented: lamination, the diagonal transpose and the
⍝ hyperbolic functions.
1 2,0.5d3 4
1 1g2 2pi4
5o1
1o0
⍝ Limited: factorial and binomial take whole numbers, encode and
⍝ decode take vectors.
!5
!2.5
10b1 2 3
10 10t2 2pi4
⍝ One specification a statement, at its left.
X←3
X←Y←9
%X←4
X
)OFF
