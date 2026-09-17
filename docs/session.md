# sw-apl Session: terminal look and feel, system commands

sw-apl reproduces the APL\360 terminal session. Everything here is
observable behaviour the reg-rs transcripts pin down.

## Prompt and transcript

- Immediate execution prompts by printing six spaces; the user
  types on the same line. Output starts at column one. The
  transcript therefore reads like a 2741 printout: indented input,
  flush-left output.
- Blank input lines are accepted silently.
- Function definition mode prompts with the bracketed line number
  followed by spaces, for example `[1]   `. A fractional number
  fills the same six columns, for example `[1.5] `.
- Output wider than `)WIDTH` wraps; continuation lines are indented
  six spaces.
- Batch mode (`-f FILE` or stdin) echoes each input line with the
  six-space indent before its output so the transcript matches an
  interactive session. `--no-echo` suppresses the echo.

## Error display

APL\360 style: the error name, then the statement, then a caret
under the point of detection.

```
      2 3+4 5 6
LENGTH ERROR
      2 3+4 5 6
         ^
```

Inside a defined function the second line carries the function
name and line number, and the state indicator gains an entry:

```
RACE[4]  LENGTH ERROR
      POS+?NH 3
         ^
```

`)SI` lists suspended functions; a bare right arrow clears the
top entry; `)SIV` adds the local names.

## System commands

Recognised when the first non-blank character is a right
parenthesis. Names are case-insensitive. Unknown commands report
INCORRECT COMMAND.

Workspace control:

| Command | Meaning |
|---|---|
| `)CLEAR` | Fresh workspace; prints `CLEAR WS` |
| `)WSID [name]` | Show or set the workspace id |
| `)SAVE [name]` | Save; prints the timestamp and id |
| `)LOAD name` | Load; prints `SAVED` and the timestamp |
| `)DROP name` | Delete a saved workspace |
| `)LIB [n]` | List workspaces in a library |
| `)COPY name [objects]` | Copy objects from a saved workspace |
| `)PCOPY name [objects]` | Copy without overwriting existing names |
| `)CONTINUE` | Save as CONTINUE and sign off |
| `)OFF` | End the session |

Inquiry and settings:

| Command | Meaning |
|---|---|
| `)FNS [letter]` | List functions (from a letter) |
| `)VARS [letter]` | List variables (from a letter) |
| `)GRPS`, `)GRP name`, `)GROUP name members` | Groups |
| `)ERASE names` | Remove objects |
| `)SI`, `)SIV` | State indicator |
| `)ORIGIN n` | Set index origin (0 or 1); replies `WAS n` |
| `)DIGITS n` | Set print precision (1 to 16); replies `WAS n` |
| `)WIDTH n` | Set print width (30 to 254); replies `WAS n` |
| `)SYMBOLS [n]` | Report or set symbol table size |

Library form: `)LOAD 1 CLASS` loads workspace CLASS from library
1. Libraries map to directories through a small configuration
file; library 0 (the default) is the current working directory
or `--lib DIR`. Workspace names follow APL rules and the file on
disk is `NAME.apl.ws` (UTF-8 text, see `design.md` D7).

## The del editor

`del-editor-guide.md` is the walkthrough; this is the table.

A del opens definition mode. The text after it is either the
header of a new function or the name of one to reopen; the lines
that follow are the body, typed behind the number each will hold.

| Typed | Meaning |
|---|---|
| `∇HEADER` | Open a new function. DEFN ERROR if the name is taken |
| `∇NAME` | Reopen NAME, positioned after its last line |
| `∇NAME[cmd]` | Reopen and run one editor command at once |
| `TEXT` | Put TEXT at the line the prompt offers, then move on |
| `[n]` | The next line typed becomes line n |
| `[n] TEXT` | Put TEXT at line n |
| `[⎕]` | Display the whole function, header and dels included |
| `[n⎕]` | Display from line n on, without the header |
| `[∆n]` | Delete line n. DEFN ERROR if there is no such line |
| `[0]` | The next line typed becomes the header |
| `[0] HEADER` | Replace the header now |
| `∇` | Close, renumbering the lines from 1 |
| `⍫` | Close locked |

Line numbers may carry up to three decimal places, which is how a
line is inserted between two that are already there: `[1.5]` goes
between 1 and 2, `[1.55]` between 1.5 and 1.6. The prompt then
steps at the grain the number itself uses, so `[1.5]` is followed
by `[1.6]`. Closing the definition renumbers every line from 1,
which is why a branch should name a label rather than a number.

A display leaves the prompt where it was, so `∇NAME[⎕]∇` shows a
function and returns to immediate execution without changing it.

`[0]` may rename the function as well as change its arguments and
locals; the old name is erased rather than left holding a copy.
The line after a header edit is line 1.

A function closed with del-tilde is locked: reopening or
displaying it is DEFN ERROR. Nothing unlocks it.

## DESCRIBE convention

Every workspace shipped in `ws/lib1/` defines a niladic function
DESCRIBE that prints a short description of the workspace, the
main functions, and how to start. After `)LOAD`, when DESCRIBE
exists the session prints a one-line hint to type it. Longer
per-function help follows the `HOWNAME` convention, one niladic
function per documented function.

## Input and editing

- Unicode glyph input is expected; see `input-methods.md` for
  Espanso and Emacs setups.
- Line editing with history: up and down arrows recall earlier
  input for editing or re-submission (so `+/?6 6` can be rolled
  again with up arrow and enter); the history persists in
  `~/.sw-apl_history` across sessions.
- Interrupt (Ctrl-C) during a running statement reports INTERRUPT
  and returns to the prompt with the state indicator preserved.
- Ctrl-D at the prompt behaves like `)OFF`.

## Sign-off

`)OFF` prints the APL\360 style sign-off line with connect time
and CPU time, then exits with status 0.
