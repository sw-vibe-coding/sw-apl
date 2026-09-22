⍝!MODES (B)
⍝ Execute, in (B) '75: a character vector run as a line of APL.
⍎'1+2'
E←'(3*2)+4*2'
⍎E
⍝ Its value is a value like any other.
(⍎E)*0.5
⍝ A line that shows nothing shows nothing: an assignment ...
⍎'X←42'
X
⍝ ... or no line at all. Execute an expression only when a test holds.
A←1
B←2
⍎(A=B)/'''EQUAL'''
B←1
⍎(A=B)/'''EQUAL'''
⍝ A name built at run time: one of several variables, by number.
V1←'FIRST'
V2←'SECOND'
N←'2'
⍎'V',N
⍝ Execute is struck from ⊥ and ∘, as on the 5100. This file carries
⍝ a backspace, 0x08, between the two, as sample 71 does.
⊥∘'6×7'
