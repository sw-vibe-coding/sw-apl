⍝!MODES (D)
⍝ IBM PC APL's extensions in (D) '83: a dyadic function called with
⍝ no left argument, execute alternate, the grades with a collating
⍝ sequence, and )RESET.
∇Z←N ROOT A
→(0≠⎕NC 'N')/RN
N←2
RN:Z←A*÷N
∇
3 ROOT 27
ROOT 16
'0' ⎕EA '1 2 3'
'0' ⎕EA '1÷0'
'''NO''' ⎕EA '1 2+1 2 3'
'1÷0' ⎕EA '2÷0'
'ABCDE' ⍋ 'DEAL'
R←5 4⍴'dealDealdeadDeadDEED'
L←2 5⍴'abcdeABCDE'
R[L⍋R;]
R[L⍒R;]
∇BAD
1÷0
∇
BAD
BAD
)SI
)RESET
)SI
