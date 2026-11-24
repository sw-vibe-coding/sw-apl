⍝!MODES (D)
⍝ The transfer form in (D) '83: an object as one character vector,
⍝ and back. The manual's own examples first.
THIS←2 3⍴⍳6
⎕TF 'THIS'
THAT←3 4⍴'ABCDEFGHIJKL'
⎕TF 'THAT'
⍴⎕TF 'THAT'
∇Z←L PLUS R
Z←L+R
∇
⎕TF 'PLUS'
⍴⎕TF 'PLUS'
⍝ Numbers go to 15 digits; a scalar has rank 0 and no shape.
PI←○1
⎕TF 'PI'
⎕TF '⎕IO'
⍝ Nothing by the name: an empty vector.
⍴⎕TF 'NOTHING'
⍝ A transfer form establishes its object and gives the name.
⎕TF 'NNEW 1 3 1 2 ¯3'
NEW
Z←⎕TF 'PLUS'
)ERASE PLUS
⎕TF Z
2 PLUS 3
⍝ A name class conflict is ignored: a function replaces a variable.
⎕TF 'FNEW 2 2 5 Z←NEWZ←42 '
⎕NC 'NEW'
NEW
⍝ A form that is not valid establishes nothing.
⍴⎕TF 'NBAD 1 3 1 2'
