⍝ Saving and loading. )SAVE writes the workspace into library 0 and
⍝ replies with the moment and the name; )LOAD reads it back and says
⍝ only when it was saved, as APL\360 did. Typing DESCRIBE is the
⍝ reader's move, not the loader's.
⍝ The moments )SAVE and )LOAD report vary, so the reg-rs filter keeps
⍝ their shape and masks their values; see docs/testing.md.
⍝ )LIB is not shown here: it lists library 0, which is yours, and a
⍝ sample cannot know what you keep in it. The command tests check it
⍝ in a directory of their own.
)WSID CLASS
A←5
∇R←HYP B
R←B×2
∇
)ORIGIN 0
)SAVE
⍝ Clearing loses everything; loading brings it back, settings and all.
)CLEAR
A
)LOAD CLASS
)WSID
A
HYP 4
⍳3
⍝ Copying is not loading. It takes the names and leaves the settings,
⍝ so the index origin here is untouched by the donor's. That matters:
⍝ see docs/index-origin-considerations.md.
)CLEAR
A←99
)ORIGIN 1
)COPY CLASS
A
HYP 4
⍳3
)WSID
⍝ )PCOPY keeps what is already here.
)CLEAR
A←99
)PCOPY CLASS
A
⍝ And copying can name what it wants.
)CLEAR
)COPY CLASS HYP
HYP 3
A
)DROP CLASS
)OFF
