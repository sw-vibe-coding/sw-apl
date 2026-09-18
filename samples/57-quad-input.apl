⍝ Quad on the right of an expression reads a line and evaluates it in
⍝ the current environment. Quote-quad reads characters as they were
⍝ typed, and on the left of an assignment writes them with no line
⍝ ending, so a prompt and its answer share a line. In batch the lines
⍝ come from this file, exactly as they would from a terminal.
X←⎕
2 3 4
X
X×2
⍝ The reply is a whole statement, so it can compute.
Y←⎕
+/⍳10
Y
⍝ A line with no value prompts again: a comment, then the answer.
Z←⎕
⍝ not an answer
99
Z
⍝ Quote-quad takes the characters and does not evaluate them.
T←⍞
2+2
T
⍴T
⍝ A function can ask for what it needs. Quad prints its own prompt on
⍝ a line of its own.
∇R←ASK;N
N←⎕
R←N⍴'*'
∇
ASK
12
⍝ A prompt written with quote-quad leaves the line open, so the answer
⍝ is typed straight after it. Quote-quad reads no prompt of its own.
∇R←GREET;WHO
⍞←'NAME: '
WHO←⍞
R←'HELLO ',WHO
∇
GREET
MIKE
⍝ Inside a function the reply sees the locals, because it is evaluated
⍝ where it was asked for.
∇R←DOUBLE N
R←⎕
∇
DOUBLE 21
N×2
⍝ With nothing left to read, a quad is an INTERRUPT.
)OFF
