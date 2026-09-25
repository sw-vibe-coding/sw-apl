⍝ A reply to quad input that is not an answer. A system command is
⍝ executed and the request for input is made again; an invalid
⍝ entry gets its error report and the request is made again, with
⍝ nothing left suspended. Quote-quad takes a command as characters.
∇R←ASK
'HOW MANY?'
R←⎕
∇
∇T
N←ASK
N⍴'*'
∇
T
)SI
)FNS
1 2 3+4 5
3
)SI
⍝ Quote-quad input: the parenthesis is only a character.
Y←⍞
)SI
Y
⍴Y
⍝ A command that replaces the workspace gives the request up.
T
)CLEAR
)SI
)FNS
