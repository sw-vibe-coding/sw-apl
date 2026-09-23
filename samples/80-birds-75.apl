⍝!MODES (B)
⍝ BIRDS in (B) '75: library 1 has a BIRDS for each mode, and )LOAD 1
⍝ BIRDS takes the one for the mode it is in. With execute a bird is
⍝ given a function by name, primitive or defined, so the birds that
⍝ APL\360 could not write are here.
)LOAD 1 BIRDS
DESCRIBE
HOWBIRDS
⍝ The Thrush and the Bluebird reach a defined function as easily as
⍝ a primitive.
6 T 'FACT'
'FACT ⌈' B 2.5
⍝ The Mockingbird gives a function its own name.
M 'SHOUT'
⍝ The Sage: FSTEP recurses without naming itself.
'FSTEP' Y 5
⍝ What is still missing: a function as a value.
NOTHERE
)OFF
