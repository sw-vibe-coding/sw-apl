⍝ A system command typed while a function definition is open. The
⍝ manual: "A system command entered during function definition will
⍝ not be accepted as a statement in the definition. Some commands,
⍝ such as )COPY, will be rejected with the message NOT WITH OPEN
⍝ DEFINITION; most will be executed immediately."
⍝
⍝ So there are two rules, and the first is the larger: a command is
⍝ never a body line. Below, FOO is opened and eight commands are
⍝ typed into it; none of them goes in, the prompt does not move for
⍝ any of them, and the four that would store or copy a workspace in
⍝ the middle of being changed are refused outright.
⍝
⍝ A comment is a different matter. It is an APL statement, so one
⍝ typed in definition mode does go into the function -- which is
⍝ why this narration is out here.
V←7
∇FOO
A←1
)VARS
)FNS
)ORIGIN 0
)SAVE
)COPY SOMEWS
)PCOPY SOMEWS
)CONTINUE
)CONT
B←⍳3
∇
⍝ Two statements, not ten.
∇FOO[⎕]∇
⍝ The setting that ran at once took effect: B is in origin 0.
FOO
B
⍝ And )VARS listed what the workspace held at the time, which did
⍝ not yet include anything of FOO's.
)VARS
)OFF
