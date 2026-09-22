⍝!MODES (B)
⍝ The system variables, in (B) '75: settings and reports named with
⍝ a quad. A clear workspace has the 5110's settings.
⎕IO
⎕CT
⎕PP
⎕PW
⎕RL
⍝ Five digits for a fraction, but a whole number of up to ten digits
⍝ is shown in full.
÷3
2*20
2*40
⍝ A setting takes a new value by assignment, and the value is the
⍝ same one the directives in a saved workspace set.
⎕PP←10
÷3
⎕IO←0
⍳5
⎕IO←1
⍝ A value a setting cannot take is refused.
⎕IO←2
⎕PP←17
⍝ The random link moves each time a random number is drawn, and
⍝ setting it replays the sequence.
⎕RL←16807
?6 6 6 6
⎕RL←16807
?6 6 6 6
⍝ The line counter: the lines being executed, innermost first.
∇R←INNER
R←⎕LC
∇
∇R←OUTER
R←0
R←INNER
∇
OUTER
⍝ The atomic vector: 256 characters, in the 5110's order.
⍴⎕AV
⎕AV⍳'APL'
⎕AV[87+⍳3]
⍝ The workspace available, and what reports cannot be set.
0<⎕WA
⎕WA←0
0<⎕WA
⍝ Kept for compatibility: the 5110 has one user and no clock.
⎕TS
⎕AI
⍝ The latent expression, run when a saved workspace is loaded.
⎕LX←'''WELCOME'''
⎕LX
⍝ A quad name the system does not have.
⎕XY
