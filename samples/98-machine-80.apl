⍝!MODES (C)
⍝ (C) '80's reach into the TRS-80, after APL80, typed in APL80's
⍝ characters. )RAM opens a modeled memory: the screen at 15360 to
⍝ 16383, plain RAM above it, and no ROM. The shifted U peeks and pokes;
⍝ dyadic # calls the two ROM routines the manual shows. )DOS takes
⍝ DIR, KILL and RETURN.
u 16384
)RAM
7 u 16384
u 16384 16385
1 2 3 u 30000 30001 30002
u 30000+0 1 2
u 1000
256 u 16384
X]51 # 65
X
43 # 0
44 # 0
65 u 16000
⍝ The screen scrolls, and a poked character with it: a peek reads
⍝ what is there now.
u 16000
)DOS
FORMAT
RETURN
X
