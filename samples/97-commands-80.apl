⍝!MODES (C)
⍝ (C) '80's commands, after APL80. Functions are defined with )DEF
⍝ and changed with )EDIT; an empty line, for BREAK, ends one. There
⍝ is no del.
∇Z←F X
)DEF Z←ROWS M
Z←(¯1⌽M)+M+1⌽M

)DEF Z←SUM M
Z←⍉ROWS⍉ROWS M

)DEF Z←GEN M;S
S←SUM M
Z←(S=3)∨M∧S=4

)DEF RUN N
' *'[1+G]
G←GEN G
N←N-1
→(N>0)/1

G←8 8↑3 3⍴0 1 0 0 0 1 1 1 1
RUN 2
⍝ )EDIT reopens one after its last line; )? shows it, and )n
⍝ replaces line n at once, renumbering the lines.
)EDIT RUN
)?
)3
N←N-2
)?

)FNS
⍝ )TRON names each line before it runs.
)DEF Z←SQ X
Z←X×X

)TRON
SQ 4
)TROFF
⍝ )PS prints six digits, as a clear workspace does; )PD fifteen.
÷3
)PD
÷3
)PS
⍝ The format statement: pairs of width and decimals, until characters.
⍕8 2
3.14159 2.71828
'DONE'
3.14159
⍝ Status (s): the line running, and the space free.
)DEF Z←WHERE
Z←⌶4

WHERE
⍝ # turns characters into APL80's codes and back.
#'APL80'
#65 80 76
#'⍴⍳'
)AUTO 'READY'
)EXEC
)OFF
