⍝!MODES (B)
⍝ TTTML: a machine that learns tic-tac-toe by playing itself, and
⍝ then plays you with what it learned. It comes trained.
)LOAD 1 TTTML
DESCRIBE
⍝ What it knows: a value for each position it has met.
⍴KEYS
⍝ Against a random player it wins most and loses none, as X and as
⍝ O. The rows are won, lost and drawn.
TRIAL 50
⍝ A game, you first, as X. A reply that is not an empty square is
⍝ asked for again. Played well on both sides, it is a draw.
PLAY 1
5
1
3
4
8
9
⍝ A game it begins. Leave it a fork and it takes it.
PLAY 2
2
4
6
9
⍝ Learning again from nothing: every 1000 games it says how many
⍝ positions it knows and how many bytes are still free, and at the
⍝ end how many X won, O won and were drawn.
⎕RL←16807
TRAIN 3000
TRIAL 50
)OFF
