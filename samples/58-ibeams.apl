⍝ The I-beam system functions. Four of them read the clock, so what
⍝ they print is different every run. The transcript still shows the
⍝ values: the sample labels them (VARIES) and the reg-rs preprocess
⍝ filter masks what follows, so a baseline holds the shape of the
⍝ answer without pinning a number that cannot come back.
⍝ Space available, in bytes, and the terminals connected.
⌶22
⌶23
⍝ Time of day and sign-on time, in sixtieths of a second since
⍝ midnight; processor time used so far, in the same units.
'TIME OF DAY (VARIES): ';⌶20
'CPU TIME (VARIES): ';⌶21
'SIGNED ON (VARIES): ';⌶24
⍝ Today's date as MMDDYY.
'DATE (VARIES): ';⌶25
⍝ Sixtieths divide into seconds, and seconds into hours, like any
⍝ other number: an I-beam is a function, not a special form.
'HOUR (VARIES): ';⌊(⌶20)÷60×60×60
⍝ 26 is the line now executing and 27 is every line in the state
⍝ indicator, innermost first. Neither moves with the clock, so both
⍝ are shown outright. In immediate execution there is no line.
⌶26
⍴⌶27
∇R←WHERE
R←0
R←⌶26
∇
WHERE
∇R←STACK
R←⌶27
∇
∇R←CALLER
R←0
R←STACK
∇
CALLER
⍝ STIR: advance the random link by an amount taken from the clock, so
⍝ a game deals differently on every run. The rolls it makes are thrown
⍝ away; only their effect on the link is wanted.
∇STIR;N
N←?(1+60|⌶20)⍴2
∇
STIR
X←?5⍴10
'DEAL (VARIES): ';X
⍝ Whatever X holds, it is five numbers between 1 and 10.
⍴X
∧/(X≥1)∧X≤10
)OFF
