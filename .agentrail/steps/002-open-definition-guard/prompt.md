Phase 7 step 2 (docs/plan.md). NOT WITH OPEN DEFINITION.

The manual's trouble report 6. A command typed while a function
definition is open is refused, because the workspace is in the
middle of being changed.

      ∇FOO
[1]   )COPY DONOR
      ⍝ that is a body line today, not a command at all

Careful: inside definition mode a `)` line is currently taken as a
body line, which is its own decision and may be the right one. Find
out what APL\360 did -- whether the command is refused with report 6
or swallowed as text -- before implementing. The manual's Table 2.1
gives report 6 to )COPY, )PCOPY, )SAVE and )CONTINUE, and NOT to
)LOAD, and the detailed sections say what each means. That )LOAD is
exempt is a fact worth understanding rather than working around: a
load replaces the whole workspace, open definition and all, so there
is nothing half-written left to be inconsistent.

Decide and record:

- Which commands carry the report. Follow the manual's list.
- What happens to the open definition when a command is refused: it
  stays open, and the next line is still a body line.
- Whether a `)` line in definition mode should be tried as a command
  at all, or whether only the four named ones are, with the rest
  staying body lines. This is the real design question of the step;
  answer it in the commit.

The del editor is apl-editor, and `Session.defining` is what says a
definition is open -- see apl-session's respond(), which routes to
definition_line before it looks for a `)`.

TDD; reg-rs; a sample showing the refusal and that the definition
survives it. Update docs/parity.md, docs/commands-reference.md and
docs/del-editor-guide.md in the same commit.
