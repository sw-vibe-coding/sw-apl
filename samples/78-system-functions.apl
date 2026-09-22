⍝!MODES (B)
⍝ The system functions, in (B) '75: a function as characters and
⍝ back, and the names in the workspace.
∇R←INTG A
R←A⍴0
I←1
START:R[I]←A
I←I+1
→(I≤A)/START
∇
INTG 4
⍝ The canonical representation: the header and each line, flush
⍝ left, without line numbers or dels.
M←⎕CR 'INTG'
M
⍴M
⍝ Change the matrix, and fix it as the function again: the result
⍝ is the function's name.
M[4;12]←'I'
⎕FX M
INTG 5
∇INTG[⎕]∇
⍝ A matrix the editor could not have made fixes nothing; the result
⍝ is the line at fault, the header being 0.
⎕FX 2 4⍴'INTG[1] '
INTG 3
⍝ A locked function's characters are not to be had.
∇R←SECRET
R←42
⍫
⍴⎕CR 'SECRET'
⍝ Name classification: 0 free, 1 label, 2 variable, 3 function,
⍝ 4 not a name.
ALPHA←1
BETA←2
⎕NC 4 5⍴'INTG ALPHAFREE 2X   '
⍝ The name list, by class, and by initial letter.
⎕NL 2
⎕NL 2 3
'B' ⎕NL 2
⍝ Expunge: erase, and say whether the name is free.
⎕EX 'BETA'
⎕NL 2
⍝ The 5110's console control checks what it is asked and answers;
⍝ sw-apl has no screen, alarm or printer to act on.
2 ⎕CC 2 2
4 ⎕CC 17
⍝ The delay is a fixed value, as on the 5110.
⎕DL
