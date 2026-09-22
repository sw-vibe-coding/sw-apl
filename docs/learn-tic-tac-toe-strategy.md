# Learning tic-tac-toe in 64 KB

How an APL program in (B) '75 can learn to play tic-tac-toe by playing
itself, and then play a person with what it learned, inside the
memory of an IBM 5110 (at most 64 KB). The method was prototyped in
sw-apl's (B) mode, and the listing below is that prototype. It is the
design for the library 1 workspace `TTTML`; `plan.md` has the plan for
the workspace itself.

## The short answer

The program learns one number per board position, not one per
position and move, and it counts two positions as one when a rotation
or a reflection turns one into the other. Self-play reaches about 750
such positions. The whole "model" is two vectors of about 750 numbers
each, 12 KB, and the functions and board tables are under 3 KB more.
That leaves most of a 5110's memory free, so there is nothing to swap
out to tape.

## Why the obvious table does not fit

The usual textbook approach is Q-learning: a table with a row for
every state and a column for every move. Encoding a board as a
base-3 number gives 3*9 = 19,683 states, and with 9 moves that is
177,147 numbers, about 1.4 MB. That does not fit in a 5110, and it
does not fit in sw-apl's default workspace either (WS FULL at 1 MB).

Most of that table is empty. Most codes are boards no game can
reach, such as nine Xs. Of the moves in a row, only the empty
squares' ever mean anything. And each real position turns up eight
times, once for each way of rotating or reflecting the board.

## What is learned instead

**A value per afterstate.** An afterstate is the board just after a
move. Its value is how good that board has turned out to be for the
player who made the move: towards 1 for a win, 0 for a loss, 0.5 for
a draw. To choose a move, the program builds the board each legal
move would give, looks up their values, and takes the best. One
number per board replaces nine per board, and the same table serves
both players, because an X afterstate and an O afterstate never
share a board (X has made one more move than O, or the same number).

**One entry per shape.** A board is encoded as `3⊥S+1`, where `S` is
the 9 squares as `¯1 0 1` (O, empty, X). The 8 symmetries of the
square are the rows of an 8 by 9 matrix of square numbers, `SYM`, so
`S[SYM]` gives all 8 versions of the board at once, and the least of
their 8 codes names them all. There are 5,478 legal positions and 765
up to symmetry; self-play meets about 750 of them.

**A table that grows.** The model is two vectors, `KEYS` (the codes
met so far) and `VALS` (their values), searched with `⍳`. A position
not seen before is appended with the value 0.5, meaning "no idea
yet". Nothing is kept for a board no game has reached.

## How it learns

1. **Play a game against itself.** At each move, `CHOOSE` values
   every legal afterstate in one step: the candidate boards are one
   matrix, `T+M×P∘.=⍳9` (the board `T` repeated, plus the mover's
   mark `M` on each empty square `P`). One time in ten the program
   plays a random legal move instead, so that it keeps trying moves
   it currently thinks are bad (exploration).
2. **Record the afterstates.** Each board after a move is kept as one
   row of the game's history, `H`.
3. **Find the result** with an inner product: `WL+.×S` sums the board
   along its 8 lines (`WL` is an 8 by 9 matrix of 0s and 1s), so a 3
   means X has a line and a `¯3` means O has.
4. **Learn from the end backwards.** For each afterstate, from the
   last to the first, move its value a fifth of the way (0.2) towards
   a target. For the last two moves the target is the result, as
   seen by whoever made the move. For any earlier move it is the
   value of that player's next afterstate, two rows on. This is
   temporal-difference learning, as in Sutton and Barto's
   tic-tac-toe example, and it is how a win or a loss at the end of
   a game reaches back to the moves that led to it.

After enough games the values settle. A position that can be forced
to a win drifts up, one that loses drifts down, and the greedy choice
(always the best value, no exploration) plays well.

## Memory, measured

Measured in sw-apl's (B) mode with `⎕WA`, after 6,000 training games:

| What | Size |
|---|---|
| The functions, and the tables `WL` and `SYM` | 2.7 KB |
| The model: 749 keys and 749 values | 12 KB |
| The largest working array, while choosing a move | 648 numbers, about 5 KB, for an instant |

About 20 KB at the peak, so a 64 KB machine has room to spare.
`)SAVE` keeps the model with the workspace, which is how a 5110 kept
anything, on its tape or diskette. Swapping the model out to tape is
not needed. A 5100's programs could reach tape files only through
shared variables, which sw-apl does not have (`mode-b.md`, decision
2).

## How well it plays

In the prototype, after 6,000 self-play games (about 9 seconds in the
CLI), the learned player against a random player:

| Learned player plays | Won | Lost | Drawn |
|---|---|---|---|
| X, 500 games | 496 | 0 | 4 |
| O, 500 games | 449 | 0 | 51 |

Playing itself with no exploration, it draws, which is the result of
perfect play.

Speed is the one thing a real 5110 would not match. It interpreted
APL far more slowly than a phone does, so it could not play 6,000
training games in seconds. A workspace that ships with its model
already trained, and a `TRAIN` that reports its progress, cover both.

## The prototype

Every name is an APL name (no underscores), there are no diamonds,
and it uses nothing the 5110 lacks. `⎕RL` seeds the random numbers,
so a run can be repeated.

```apl
⍝ The 8 lines of the board, and its 8 symmetries as square numbers.
WL←8 9⍴1 1 1 0 0 0 0 0 0 0 0 0 1 1 1 0 0 0 0 0 0 0 0 0 1 1 1 1 0 0 1 0 0 1 0 0 0 1 0 0 1 0 0 1 0 0 0 1 0 0 1 0 0 1 1 0 0 0 1 0 0 0 1 0 0 1 0 1 0 1 0 0
SYM←8 9⍴1 2 3 4 5 6 7 8 9 7 4 1 8 5 2 9 6 3 9 8 7 6 5 4 3 2 1 3 6 9 2 5 8 1 4 7 3 2 1 6 5 4 9 8 7 7 8 9 4 5 6 1 2 3 1 4 7 2 5 8 3 6 9 9 6 3 8 5 2 7 4 1
KEYS←⍳0
VALS←⍳0
⍝ 1 X has won, ¯1 O has, 2 a draw, 0 the game goes on.
∇R←OUTCOME S;L
L←WL+.×S
R←2×~0∊S
→(~3∊L)/NX
R←1
NX:→(~¯3∊L)/0
R←¯1
∇
⍝ The code of each row of T, the least of its 8 symmetries.
∇C←CODES T;N
N←1↑⍴T
C←⌊/(N,8)⍴3⊥⍉((N×8),9)⍴(T+1)[;,SYM]
∇
⍝ The value of each row of T, adding the ones not seen before.
∇V←VALUE T;C;N
C←CODES T
N←(KEYS⍳C)>⍴KEYS
N←N∧(⍳⍴C)=C⍳C
KEYS←KEYS,N/C
VALS←VALS,0.5+0×N/C
V←VALS[KEYS⍳C]
∇
⍝ The move whose afterstate is worth the most to the player to move.
∇A←CHOOSE S;P;V;M
P←(S=0)/⍳9
M←1-2×2|+/S≠0
V←VALUE(((⍴P),9)⍴S)+M×P∘.=⍳9
A←P[V⍳⌈/V]
∇
⍝ One game against itself, exploring one move in 1÷EPS, then learn.
∇W←GAME EPS;S;H;A;P;M
S←9⍴0
H←0 9⍴0
M←1
LOOP:P←(S=0)/⍳9
A←CHOOSE S
→(EPS≤(?1000)÷1000)/PUT
A←P[?⍴P]
PUT:S[A]←M
H←H,[1]S
W←OUTCOME S
M←-M
→(W=0)/LOOP
H LEARN W
∇
⍝ Back each afterstate up towards the mover's next one, or the result.
∇H LEARN W;I;J;N;T
I←KEYS⍳CODES H
N←⍴I
J←N
BACK:T←(W=2)×0.5
T←T+W=1-2×0=2|J
→(J≥N-1)/SET
T←VALS[I[J+2]]
SET:VALS[I[J]]←VALS[I[J]]+0.2×T-VALS[I[J]]
J←J-1
→(J>0)/BACK
∇
⍝ N games; the result is how many X won, O won, and were drawn.
∇R←TRAIN N
R←0 0 0
L:R←R+(GAME 0.1)=1 ¯1 2
N←N-1
→(N>0)/L
∇
```

To try it, run sw-apl with `--mode 75`, enter the listing, then:

```apl
      ⎕RL←16807
      TRAIN 6000
1375 414 4211
      ⍴KEYS
749
      GAME 0
2
```

`GAME 0` plays one game with no exploration (it still learns from
it), and 2 is a draw.
