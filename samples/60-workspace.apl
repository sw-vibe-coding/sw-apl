⍝ The workspace is a thing you can name and clear. What )SAVE will
⍝ write and )LOAD will read back is the symbol table, the state
⍝ indicator, and the settings kept beside them; the terminal it runs
⍝ on is not part of it.
⍝ An unnamed workspace is CLEAR WS.
)WSID
)WSID CLASS
)WSID
⍝ Names, functions and settings all belong to the workspace.
A←5
∇R←DOUBLE N
R←N+N
∇
)ORIGIN 0
)DIGITS 3
DOUBLE A
⍳3
1÷3
⍝ A function that stops is part of it too: the state indicator holds
⍝ it, with its locals, until something clears it.
∇R←BAD;T
T←1
R←T÷0
∇
BAD
)SI
T
⍝ )CLEAR gives a fresh workspace: the names go, the settings go back
⍝ to where a clear workspace starts, the name goes, and so does the
⍝ suspended function.
)CLEAR
)WSID
)SI
A
⍳3
1÷3
)OFF
