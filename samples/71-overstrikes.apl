⍝ Overstrikes. A 2741 had no key for most APL glyphs: you typed
⍝ one character, backspaced, and typed another over it, and the
⍝ golf ball struck both on one position.
⍝ This file carries the backspace a 2741 sent, 0x08, between the
⍝ two characters. At a terminal the key is Ctrl-] instead, because
⍝ a line editor needs backspace for deleting.
⍝ Circle struck with star is log.
A←○*3
A
⍝ Quad struck with divide is domino.
⎕÷2 2⍴4 7 2 6
⍝ Either order forms the same glyph: on paper there is no
⍝ difference, since both impressions land on one spot.
∧/,(÷⎕2 2⍴4 7 2 6)=⎕÷2 2⍴4 7 2 6
⍝ Some characters on the 2741 have no meaning of their own. The cap
⍝ is there so the lamp can be struck from it and the circle; the cup
⍝ is struck into nothing. Each is a character of the set, so alone
⍝ it is a SYNTAX ERROR, not a CHARACTER ERROR, and in quotes it is
⍝ character data like any other.
∩
2∪3
∩○ this is a comment
⍴'∩∪'
⍝ A pair that forms no glyph is what the manual calls an
⍝ illegitimate overstrike, and gives as a cause of CHARACTER ERROR.
QZ
⍝ The underbar struck over a letter is one of the twenty-six
⍝ underscored letters: A̲ through Z̲, characters of the APL\360 set
⍝ in their own right and valid in names.
X←2
X_←3
X
X_
X×X_
⍝ They are letters, not decoration: X and X̲ are two names.
)VARS
⍝ The low line prints on its letter and takes no column, so the
⍝ caret points where it should.
X_←1 2
X_+4 5 6
⍝ Alone it underscores nothing, and is a CHARACTER ERROR.
̲
)OFF
