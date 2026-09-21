Owner direction 2026-09-20: the browser REPL has no history.

The CLI recalls earlier input with the up arrow and walks it with
up and down; the browser does not, and it is the one thing the CLI
has that the page has not. Give the page the same.

Physical keyboard only for now. The owner has said the on-screen
board is to be refactored later and history on it belongs with
that; do not add a control to the board here.

`ArrowUp` and `ArrowDown` reach `Act::Ignore` in `apl-board`'s
`act`, so they are unclaimed and the page may take them without
touching the keyboard crate or the 2741 map. Prefer that: the
recall is a property of this terminal, not of the keyboard.

Behave as the line editor at the CLI does. Up from the line being
typed goes to the last line entered; down walks back towards it;
down past the end returns the line that was being typed rather
than an empty one, because a reader who was half way through a
line and looked at history has not abandoned it.

Whether history survives a reload is a separate question from
recall and is not asked for here. Say in the commit which was done.

Check it with a real browser: type two lines, recall both, walk
back down, and confirm the half-typed line comes back.
