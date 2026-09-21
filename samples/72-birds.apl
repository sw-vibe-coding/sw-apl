⍝ BIRDS: the combinators APL\360 can write, after Smullyan's
⍝ To Mock a Mockingbird. APL\360 cannot hand one function to another,
⍝ so the birds come in three kinds: those on values, which it writes
⍝ in general; those over a fixed table of primitives, each named by
⍝ its glyph; and those it cannot write at all, which NOTHERE names.
)LOAD 1 BIRDS
DESCRIBE
⍝ Every bird, flying.
HOWBIRDS
⍝ The Kestrel keeps its left argument and the Kite its right, whatever
⍝ the other is -- a character vector as happily as a number.
'KEEP' K 1 2 3
'DROP' KI 1 2 3
⍝ The Thrush reads as a pipeline: the value first, the function after.
⍳5
(⍳5) T '⌽'
⍝ The Warbler duplicates its argument, the Cardinal flips a pair, the
⍝ Starling feeds a value to a function and to the function's partner.
'+' W 2 3 4
'÷' C 2 10
'×⍳' S 4
⍝ A glyph outside the tables is an INDEX ERROR: APL\360 has no execute,
⍝ so APPLY and DYAD reach only the primitives they list. The error
⍝ suspends in APPLY, called from B; a bare branch clears it.
'⍉⍳' B 5
)SI
→
⍝ What is not here, and why. The Sage and Z are not needed to recurse:
⍝ a function calls itself by name, as FACT does.
NOTHERE
FACT 10
)OFF
