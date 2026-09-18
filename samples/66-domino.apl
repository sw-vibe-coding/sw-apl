⍝ Domino: matrix inverse and matrix divide. It is the one primitive
⍝ that is not in the 1968 manual -- it was added to APL\360 in 1970.
⍝ Monadic, it inverts. This matrix is chosen so that its inverse
⍝ is exact in binary floating point -- halves and eighths, not
⍝ thirds -- because a transcript prints what it is given.
A←2 2⍴2 1 0 4
A
⌹A
⍝ A matrix times its inverse is the identity. It is exactly the
⍝ identity here only because this example was chosen to be; in
⍝ general the off-diagonal zeros come out as something times
⍝ ten to the minus sixteenth, which is what floating point is.
A+.×⌹A
⍝ A scalar inverts to its reciprocal.
⌹2
⍝ Dyadic, it solves: A⌹B is the X for which B+.×X is A.
B←3 3⍴1 0 0 1 1 0 1 1 1
B
(1 2 3)⌹B
⍝ Check it: B times the solution gives back what we divided.
B+.×(1 2 3)⌹B
⍝ When B has more rows than columns there is no exact answer, and
⍝ domino gives the least squares fit. Four points on the line
⍝ Y←1+2×X, with X held in the second column and a column of ones in
⍝ the first, so the fit is the intercept and the slope.
X←4 2⍴1 0 1 1 1 2 1 3
X
Y←1 3 5 7
Y⌹X
⍝ Two vectors fit one coefficient: the slope of a line through the
⍝ origin.
(2 4 6)⌹1 2 3
⍝ A singular matrix has no inverse: the second row is twice the
⍝ first, so DOMAIN ERROR.
⌹2 2⍴1 2 2 4
⍝ More columns than rows is underdetermined, and APL does not choose
⍝ a solution for you.
⌹2 3⍴1 0 0 0 1 0
⍝ Rank 3 is not a matrix.
⌹2 1 1⍴1 1
)OFF
