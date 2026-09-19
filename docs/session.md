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
name and the line number in place of the six-space indent, and the
caret moves with it:

```
LENGTH ERROR
RACE[4]  POS+?NH 3
            ^
```

## Suspension and the state indicator

A function whose line fails does not unwind. It stays on the stack
suspended: its arguments and locals are still there to look at, and
the state indicator gains an entry. A function that called it is
pendent -- stopped, but waiting on a call rather than on the error.

```
      )SI
INNER[2]*
OUTER[2]
```

Innermost first. The star marks the suspended function, the one
the error came from and the one that can be taken up again;
entries without it are pendent. `)SIV` adds the names each call
made local, in a column:

```
      )SIV
INNER[2]*  Q
OUTER[2]   P
```

A bare right arrow clears the top entry, and takes the pendent
callers with it, since they have nowhere to return to. A second
suspension underneath is left alone. A right arrow with a line
number takes the suspended function up again there; when it
returns, a caller waiting on it carries on at its next line.

sw-apl suspends a function that was called as a whole statement.
A call written inside a larger expression unwinds instead: the
error still names the function and the line, but there is no
half-evaluated expression to come back to, so there is nothing to
resume. Calls may nest 128 deep before DEPTH ERROR, and the state
indicator may grow no longer.

## Reading a line part way through a statement

Quad on the right of an expression prompts with `⎕:` on a line of
its own and reads the next line, which is evaluated as a whole
statement where it was asked for. Quote-quad on the right reads
the characters as typed, with no prompt. Quote-quad on the left
writes with no line ending, so this pair share a line:

```
      ∇R←GREET;WHO
[1]   ⍞←'NAME: '
[2]   WHO←⍞
[3]   R←'HELLO ',WHO
[4]   ∇
      GREET
NAME: MIKE
HELLO MIKE
```

The line stays open within a statement. Two statements typed in
immediate execution do not share a line: the first one ends it.

In batch mode the line a statement reads is the next line of the
script, taken from the same place the run takes its own, so the
run carries on after whatever the read consumed. A read with no
more input to take is INTERRUPT.

## System commands

Recognised when the first non-blank character is a right
parenthesis. Names are case-insensitive. Unknown commands, and
known ones given an argument they do not take, report INCORRECT
COMMAND. The tables below say what each command means in
APL\360; `parity.md` says which of them sw-apl answers yet.

Workspace control:

| Command | Meaning |
|---|---|
| `)CLEAR` | Fresh workspace; prints `CLEAR WS` |
| `)WSID [name]` | Show or set the workspace id; setting replies `WAS` the old one |
| `)SAVE [name]` | Save; prints the timestamp and id |
| `)LOAD name` | Load; prints `SAVED` and the timestamp |
| `)DROP name` | Delete a saved workspace |
| `)LIB [n]` | List workspaces in a library |
| `)COPY name [objects]` | Copy objects from a saved workspace |
| `)PCOPY name [objects]` | Copy without overwriting existing names |
| `)CONTINUE` | Save as CONTINUE and sign off |
| `)OFF` | End the session |

`commands-reference.md` is the long version: every command with
what it replies and what it refuses, including the ones sw-apl does
not have and why.

Inquiry and settings:

| Command | Meaning |
|---|---|
| `)FNS [letter]` | List defined functions, alphabetically, from a letter |
| `)VARS [letter]` | List global variables, alphabetically, from a letter |
| `)GRPS [letter]` | List group names, alphabetically, from a letter |
| `)GRP name` | List what a group gathers, as it was gathered |
| `)GROUP name [members]` | Gather names under one name; one name alone disperses it |
| `)ERASE names` | Remove global objects; a group name takes its members |
| `)SI`, `)SIV` | State indicator; `)SIV` adds local names |
| `)ORIGIN n` | Set index origin (0 or 1); replies `WAS n` |
| `)DIGITS n` | Set print precision (1 to 16); replies `WAS n` |
| `)WIDTH n` | Set print width (30 to 254); replies `WAS n` |
| `)SYMBOLS` | How many names are held, and how many would fit |

## What the workspace is

The workspace is what `)SAVE` writes and `)LOAD` reads back: the
names it holds (variables and defined functions), the state
indicator, the index origin, the print precision and width, the
random link, and the name it answers to. The terminal it is
running on is not part of it, so a loaded workspace does not
carry someone else's console, clock or sign-on time along.

An unnamed workspace is called `CLEAR WS`, which is what `)WSID`
reports until it is given a name.

`)CLEAR` gives a fresh one: every name goes, the settings go back
to where a clear workspace starts, the name goes, and so does a
suspended function -- the state indicator is part of the
workspace, not of the session, and is cleared with everything
else. A function you were in the middle of is gone, not resumed.

## When a command cannot do it

The APL\360 manual calls these trouble reports, and the distinction
it draws is worth knowing: **INCORRECT COMMAND is about the
command**, not about the workspace.

| Reply | Means |
|---|---|
| `INCORRECT COMMAND` | The command was given an argument it does not take: a missing name, one word too many, a value out of range |
| `WS NOT FOUND` | No stored workspace of that name. `)LOAD`, `)COPY`, `)PCOPY`, `)DROP` |
| `OBJECT NOT FOUND` | The workspace is there but holds no such name. A `)COPY` that named one |
| `IMPROPER LIBRARY REFERENCE` | That number is not a library. An *empty* library is a different thing and says nothing |
| `NOT SAVED, THIS WS IS name` | `)SAVE name` where a stored workspace of that name is not this one. It is not overwritten |
| `NOT GROUPED, NAME IN USE` | The first name of a `)GROUP` already holds a function or a variable |
| `NOT ERASED: names` | `)ERASE` left those alone: they are functions on the state indicator |

A comma introduces a reason and a colon introduces a list, which is
the manual's own convention.

`)COPY name A B` where the workspace holds A but not B copies
neither, so a copy that half worked cannot be mistaken for one that
worked.

## Listing what a workspace holds

`)FNS` lists the defined functions and `)VARS` the global
variables, alphabetically, wrapped at the print width. A letter
starts the listing there:

```
      )VARS
ALPHA MID ZED
      )VARS M
MID ZED
```

`)VARS` lists *global* variables. Inside a suspended function the
call's locals are in scope and a local may shadow a global of the
same name, but the listing is of the globals either way -- which is
what you want when you are deciding what to save.

`)ERASE` removes global objects. A function on the state indicator
is waiting to be taken up again, so it is left alone and named:

```
      )ERASE STUCK MID
NOT ERASED: STUCK
```

## Groups

A group gives one name to a collection of names, so that they can
be copied or erased together, or gathered into a larger group.

```
      )GROUP TRIG HYP TWICE LATER
      )GRPS
TRIG
      )GRP TRIG
HYP TWICE LATER
```

A group holds names, not what they refer to: a member need not
exist, and dispersing a group leaves its members alone. The rules:

- The first name must not already hold a function or a variable,
  or the reply is `NOT GROUPED, NAME IN USE`.
- Naming a group again supersedes it. To add to it instead, name
  it among its own members: `)GROUP TRIG TRIG ALPHA`.
- `)GROUP name` with no members disperses the group. The names it
  gathered keep whatever they held.
- `)ERASE` of a group name erases the group and the objects its
  members name. That is what a group is for.
- `)COPY name GROUPNAME` brings the group and its members.

`)GRPS` sorts, as the name listings do. `)GRP` does not: it shows
the group as it was gathered.

## How many names

`)SYMBOLS` reports `IS n, USED m`: how many names the workspace
holds, and how many it could hold. APL\360 set a symbol table
aside when you signed on, and `)SYMBOLS n` in a clear workspace
resized it. sw-apl sets nothing aside -- names are charged against
the workspace like everything else -- so the size is what the space
still free would hold if every further name were the shortest one,
and it falls as the workspace fills. There is nothing to set, so
`)SYMBOLS n` is `INCORRECT COMMAND`.

## How big a workspace is

A workspace holds a fixed number of bytes. `⌶22` reports how many
are still free; `--ws-size` sets the size, and the default is
1048576 bytes. Anything that will not fit is `WS FULL`, and
nothing is stored.

The size belongs to the session, not to the workspace: it is not
saved, so a workspace saved under a large size need not load under
a small one. `workspaces.md` has what each thing costs.

## Where workspaces live

Libraries are numbered, as in APL\360: library 0 is your own and
the numbered ones are public.

| Library | Directory | |
|---|---|---|
| 0 | `work/` | Yours. `)SAVE` writes here, and `)LOAD NAME` reads here. Not tracked; the first `)SAVE` creates it |
| 1 | `ws/lib1/` | The workspaces sw-apl ships, each with a DESCRIBE function |

`--library DIR` sets the directory they are all under; the default
is the current one, so running sw-apl from a checkout finds the
shipped workspaces. Point it somewhere else and library 0 goes with
it, which is how a test or a script keeps out of your own `work/`.

Library 1 holds:

| Workspace | |
|---|---|
| `LIFE` | Conway's Life on a torus: `GLIDER` then `RUN 4` |
| `RACE` | A horse race, written to be read: character matrix, mixed output, the conditional branch idiom |
| `EDIT` | A workspace to practise the del editor on. `FACT` is wrong by one on purpose |

`)LIB` lists library 0 and `)LIB 1` lists library 1; one library
at a time, as APL\360 did. A workspace is `NAME.apl.ws`, UTF-8
text (see `design.md` D7).

A saved workspace is APL you could have typed, so it can also be
run as a program:

```
      )LOAD CLASS
      sw-apl -f work/CLASS.apl.ws     # the same thing, from a shell
```

What it holds is the names, the settings and the random link. It
does not hold a suspended function: re-executing a file cannot
put execution back in the middle of a call, so `)LOAD` gives a
workspace with an empty state indicator.

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

`workspaces.md` is the guide; this is the summary.

Every workspace shipped in `ws/lib1/` defines a niladic function
DESCRIBE that prints a short description of the workspace, the
main functions, and how to start. `)LOAD` prints only the line
saying when the workspace was saved, as APL\360 did; typing
DESCRIBE is the reader's move, not the loader's. Longer
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
  and returns to the prompt. The function it stopped is suspended
  like any other failure, so `)SI` shows where it stopped and a
  branch takes it up again:

```
      SPIN
INTERRUPT
SPIN[4]  R←R+I
         ^
      )SI
SPIN[4]*
```

  A body is read between its lines, so a statement that has not
  finished a line of its own -- a long reduction over a large
  array -- cannot yet be stopped. At a prompt Ctrl-C cancels the
  line, as it always did: the line editor holds the terminal then,
  and no interrupt is sent.
- Ctrl-D at the prompt signs off, as `)OFF` does.

## Running a file from the shell

Four ways, all equivalent:

```sh
sw-apl -f prog.apl          # batch, with the transcript echo
sw-apl --no-echo -f prog.apl   # output only
sw-apl < prog.apl           # the same, from stdin
printf '2+2\n)OFF\n' | sw-apl
```

A here-document puts a program inside a shell script. Quote the
delimiter so the shell leaves `$` and backticks alone:

```sh
sw-apl --no-echo <<'APL'
3 HYP 4
)OFF
APL
```

A `.apl` file can also be executable. sw-apl drops a first line
that starts with `#!`, since the kernel has already acted on it,
so the transcript begins with the program:

```apl
#!/usr/bin/env -S sw-apl --no-echo -f
'HELLO'
)OFF
```

Two things about that line. The flags come before `-f`, because
`-f` would otherwise take `--no-echo` as its filename. And the
`env -S` form is the portable one: a bare `#!/path/sw-apl -f`
works on macOS, which splits a shebang's arguments, but not on
Linux, which passes them as a single argument.

Only the first line, and only those two characters: `#` is not an
APL\360 character, so it is a CHARACTER ERROR anywhere else.

## Sign-off

`)OFF` signs off and exits with status 0: the time and date the
session ended, then how long it was connected and how much
processor time it used.

```
      )OFF
20.11.38 09/17/26
CONNECTED 0.05.12
CPU TIME 0.00.03
```

Durations read as hours, minutes and seconds. APL\360 also named
the port and the user and carried totals to date; sw-apl has no
accounts and keeps no such records, so it does not.

Ctrl-D at the prompt signs off the same way.
