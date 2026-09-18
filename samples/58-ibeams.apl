⍝ The I-beam system functions. Three of them read the clock, so this
⍝ transcript can only show that their values are sensible, never what
⍝ they are: a baseline made from them would fail a second later.
⍝ The ones that do not move are shown outright.
⌶22
⌶23
⍝ Time of day, in sixtieths of a second since midnight, so somewhere
⍝ in the day. Processor time is never negative.
((⌶20)≥0)∧(⌶20)<5184000
(⌶21)≥0
⍝ Sign-on was earlier today, or today began after it.
(⌶24)≥0
⍝ The date is MMDDYY, so the month is a month and the day is a day.
MONTH←⌊(⌶25)÷10000
DAY←⌊100|(⌶25)÷100
((MONTH≥1)∧MONTH≤12)∧(DAY≥1)∧DAY≤31
⍝ 26 is the line now executing and 27 is every line in the state
⍝ indicator, innermost first. In immediate execution there is none.
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
⍝ What X holds now differs from run to run, so only its shape and the
⍝ range of its values can be shown.
⍴X
∧/(X≥1)∧X≤10
)OFF
