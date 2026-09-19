Phase 7 step 4 (docs/plan.md). An open output line does not survive
a statement in immediate execution.

Found while sorting the parity rows into gaps and restrictions. This
one is a gap.

`⍞←` leaves the output line open so that what comes next continues
on it. That is the whole point of the prompt idiom, `⍞←'NAME: '`
followed by a read on the same line. Inside a function it works:

      ∇F
[1]   ⍞←'P'
[2]   ⍞←'Q'
[3]   ∇
      F
PQ

In immediate execution it does not: each statement ends the line, so
two `⍞←` lines print P and Q on separate lines where APL\360 would
print PQ.

The cause is the session's shape rather than the evaluator's.
`Session::respond` answers a `Reply` holding complete lines, and an
unterminated line cannot cross that boundary: the shell prints each
with a newline. `Shown` already carries an `open` flag inside a
statement, which is why the function case works -- it is the handoff
to the caller that loses it.

So the fix is to let `Reply` say that its last line is still open,
and to have the shells honour it: the batch runner and the REPL
both print replies, and the web demo in Phase 8 will be a third
caller of the same API. Doing this before the demo is worth more
than doing it after, since the demo would otherwise inherit the
wrong shape.

Decide and record what happens to an open line when the next thing
is not output at all: an error report, a command reply, a prompt.
APL\360's terminal had no choice -- the carriage was where it was --
but a line-oriented API does, and what it chooses should be written
down rather than discovered.

The interactive REPL is the awkward case: rustyline prints its own
prompt. Work out what an open line means there before changing it,
and if the answer is that it cannot be honoured interactively, say
so in the row rather than pretending.

TDD; reg-rs for anything run through the binary, which is where this
is visible; update docs/parity.md and docs/session.md in the same
commit.
