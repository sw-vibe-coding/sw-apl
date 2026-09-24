⍝!MODES (B)
⍝ Format, in (B) '75: a number as characters. The examples are the
⍝ IBM 5110 APL Reference Manual's.
⍝ Monadic: the display, as a character array.
B←3 4⍴⍳12
X←⍕B
X
⍴X
⍴⍕123
⍝ Dyadic: width and precision. A positive precision is decimal form.
B←3 2⍴12.34 ¯34.567 0 12 ¯0.26 ¯123.45
B
9 2⍕B
⍝ A negative precision is scaled form, to that many digits.
9 ¯2⍕B
⍝ A width of 0 leaves one space between numbers; one number is a
⍝ precision with a width of 0.
2⍕B
⍝ A pair for each column.
6 2 6 1⍕B
⍝ The sign is kept when the digits are not.
4 2⍕¯.0004
⍝ Too narrow a field is a DOMAIN ERROR.
3 2⍕B
⍝ Format makes the numbers into text a line can be built from.
'TOTAL: ',8 2⍕+/12.5 7.25 3
