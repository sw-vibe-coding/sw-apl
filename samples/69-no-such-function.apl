⍝ A glyph used where it has no such form. APL\360 has no function
⍝ there, so the sentence does not parse and the answer is SYNTAX
⍝ ERROR -- not a report that the feature is missing, because it is
⍝ not missing: there is no such feature.
⍝ Not is monadic only.
~0
1~0
⍝ So are grade-up and grade-down.
⍋3 1 2
1⍋2
1⍒2
⍝ Reduce and scan need a function with a dyadic scalar form. Iota
⍝ has none, so there is nothing for the slash to reduce with.
⍳/1 2
⍳\1 2
⍝ Nor have the products.
1 2∘.⍳1 2
⍝ An axis bracket on a glyph that takes none, and on a form that
⍝ belongs to APL2 rather than APL\360, are the same answer.
⍳[1]3
,[1]2 2⍴⍳4
⍝ A glyph that does have the form, given an argument outside it, is
⍝ a different matter: that is a DOMAIN ERROR.
÷0
1⍟0
⍝ And reducing an empty vector by a function with no identity
⍝ element is a domain error too, not a syntax one.
⍲/⍳0
+/⍳0
)OFF
