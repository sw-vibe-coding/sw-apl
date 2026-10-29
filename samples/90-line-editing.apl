⍝ Line editing, [N⎕M]: change part of a statement without retyping
⍝ it. The editor displays statement N and the marks line starts at
⍝ position M, counted from the left margin. Under the statement: /
⍝ deletes the character above, a digit K puts K blanks before it, a
⍝ letter 5×R blanks (A 5, B 10, ...). The statement is displayed
⍝ again, and the next line is typed from its first blank: a blank
⍝ keeps what is above, a character fills a blank.
⍝ The manual's example: 33 becomes 3, Q gets its parenthesis, R
⍝ becomes T.
∇Z←ABC X
Z←(33×Q+(R×5)-6
[1⎕9]
/   1 /1
)  T
∇
∇ABC[⎕]∇
⍝ Statement 0 is the header: open two blanks before F, write R←.
∇F
X←1
[0⎕6]
2
R←
∇
∇F[⎕]∇
⍝ Changing the number moves the statement and leaves the old one:
⍝ delete the 1 of [1], open a blank before ], and type 2.
∇F
[1⎕1]
/1
2
∇
∇F[⎕]∇
⍝ A request for a statement that is not there is a DEFN ERROR.
∇F[5⎕3]
∇
