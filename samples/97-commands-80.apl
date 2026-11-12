⍝!MODES (C)
⍝ (C) '80's commands, after APL80, typed in APL80's characters.
⍝ Functions are defined with )DEF and changed with )EDIT; an empty
⍝ line, for BREAK, ends one. There is no del.
∇Z←F X
)DEF Z←ROWS M
Z←(-1rM)+M+1rM

)DEF Z←SUM M
Z←gROWS gROWS M

)DEF Z←GEN M;S
S←SUM M
Z←(S=3)vM&S=4

)DEF RUN N
' *'(1+G)
G←GEN G
N←N_1
→(N>0)/1

G←8 8↑3 3p0 1 0 0 0 1 1 1 1
RUN 2
⍝ )EDIT reopens one after its last line; )? shows it, and )n
⍝ replaces line n at once, renumbering the lines.
)EDIT RUN
)?
)3
N←N_2
)?

)FNS
⍝ )TRON names each line before it runs.
)DEF Z←SQ X
Z←XxX

)TRON
SQ 4
)TROFF
⍝ )PS prints six digits, as a clear workspace does; )PD fifteen.
%3
)PD
%3
)PS
⍝ The format statement: pairs of width and decimals, until characters.
f8 2
3.14159 2.71828
'DONE'
3.14159
⍝ Status (s): the line running.
)DEF Z←WHERE
Z←s4

WHERE
⍝ # turns characters into APL80's codes and back.
#'APL80'
#65 80 76
#'pi'
⍝ Names may hold . and -; round brackets index; 1d chooses the axis.
M-N←2 3pi6
M-N(2;3)
+/1dM-N
↑3 1 2
)AUTO 'READY'
)EXEC
)OFF
