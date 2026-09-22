⍝ Single-element extension. A dyadic scalar function takes two
⍝ arguments of the same shape, or a scalar or a one-element array
⍝ of any rank with anything; the single element goes with each
⍝ element of the other.
1 2 3+,5
(,5)×1 2 3
(1 1⍴100)+1 2 3
(2 2⍴1 2 3 4)×1 1 1⍴10
⍝ Against an empty array the result is empty.
⍴(,5)+⍳0
⍝ Arrays of more than one element must agree.
1 2 3+1 2
⍝ Compression extends a scalar or one-element left argument, but not
⍝ a scalar right argument: the left argument must be one element too.
1/1 2 3
(,0)/1 2 3
1 0 1/7
1/7
⍝ Decode takes a scalar or a one-element vector on either side.
(,10)⊥1 7 7 6
10 10 10⊥,5
⍝ An axis may be a one-element array.
M←2 3⍴⍳6
+/[,1]M
⌽[,2]M
