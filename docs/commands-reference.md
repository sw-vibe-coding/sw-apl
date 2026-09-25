# System Commands Reference

A line beginning with `)` is a command to the session rather than
APL to evaluate. Commands act on the workspace and the libraries,
and are never part of a function: the manual says a system command
entered during function definition "will not be accepted as a
statement in the definition".

Most run at once, even with a definition open. Four are refused,
because each would store or copy a workspace in the middle of being
changed:

```
      )SAVE
NOT WITH OPEN DEFINITION
```

`)SAVE`, `)COPY`, `)PCOPY` and `)CONTINUE`. The definition stays
open and the next line is still the statement it was going to be.

`)LOAD` is not among them, which is worth knowing rather than
guessing at: a load replaces the whole workspace, so nothing
half-written is left to be inconsistent with it. An open definition
belonged to the workspace being replaced and goes with it.

`session.md` is the short table and the surrounding prose.
`parity.md` says which of these is done. This is what each one does,
what it replies, and what it refuses.

The groups below are the manual's own.

## Which commands each mode has

Both modes have every command on this page but six, which only
(A) '70 has:

| Commands | In (A) '70 | In (B) '75 |
|---|---|---|
| `)ORIGIN` `)DIGITS` `)WIDTH` | The index origin, print precision and print width | `INCORRECT COMMAND`; the settings are the system variables `⎕IO`, `⎕PP` and `⎕PW` |
| `)GROUP` `)GRP` `)GRPS` | Groups of names | `INCORRECT COMMAND`; the IBM 5100 family has no groups |

In (B) each of the six answers as any command the session does not
know. The settings themselves are the same in both modes: a
workspace saved with `)ORIGIN 0` in (A) loads in (B) with `⎕IO` 0,
and the other way round. `--mode 70` or `--mode 75` chooses the
mode at the command line, and the tab does in the browser; the
tables below mark the six "(A) only".

## How much of a name to type

Only the first four characters of a command name are significant.
The manual: "Where the first word of a command form is more than
four characters long, only the first four are significant. The
others are included only for mnemonic reasons, and may be dropped or
replaced, as desired. For example, )CLEAR, )CLEA, )CLEAVER, etc.,
are all equivalent."

```
      )CLEA
CLEAR WS
      )CLEAVER
CLEAR WS
      )ORIG 0
WAS 1
```

What follows the fourth character is ignored rather than forgiven,
so `)CLEAVER` is `)CLEAR` and not a near miss that happens to be
let through.

A name of four characters or fewer has nothing to cut and must be
exact: `)VARS` is the command and `)VAR` is not, and `)SI` does not
extend to `)SIX`. The commands that can be shortened are the ten
longer than four characters: `)CLEAR`, `)CONTINUE`, `)DIALECT`,
`)DIGITS`, `)ERASE`, `)GROUP`, `)ORIGIN`, `)PCOPY`, `)SYMBOLS`,
`)WIDTH`. No two of them agree in their first four. In (B), which has
not got `)DIGITS`, `)GROUP`, `)ORIGIN` or `)WIDTH`, it is the other
six.

## Terminal control

| Form | |
|---|---|
| `)OFF` | End the session |
| `)CONTINUE` | Save the workspace as CONTINUE, then end the session |

`)OFF` prints the moment, how long the session was connected, and
how much processor time it used:

```
      )OFF
17.00.12 09/18/26
CONNECTED 0.00.00
CPU TIME 0.00.00
```

APL\360 named the port and the user as well, and carried totals to
date. sw-apl has no accounts and keeps no such records.

`)CONTINUE` saves first, so it prints the `)SAVE` reply and then the
sign-off. Loading CONTINUE afterwards puts you back where you were,
except for a suspended function -- see `workspaces.md`.

Ctrl-D at the prompt ends the session as `)OFF` does.

## Workspace control

| Form | |
|---|---|
| `)CLEAR` | Replace the workspace with an empty one |
| `)WSID` | Show the workspace's name |
| `)WSID name` | Rename the workspace |
| `)COPY [lib] name [objects]` | Bring names out of a stored workspace |
| `)PCOPY [lib] name [objects]` | As `)COPY`, but keep any name already here |
| `)GROUP name [members]` | Gather names under one name; one name alone disperses. (A) only |
| `)ERASE names` | Remove global objects |
| `)ORIGIN n` | Index origin, 0 or 1. (A) only |
| `)DIGITS n` | Print precision, 1 to 16. (A) only |
| `)WIDTH n` | Print width, 30 to 254 (APL\360 stopped at 130). (A) only |

`)CLEAR` replies `CLEAR WS`. Everything goes: names, settings, the
workspace's own name, and the state indicator with them, so a
suspended function is gone rather than resumed.

`)WSID` on its own reports the name, and `CLEAR WS` when there is
none. Setting it replies with the name it replaced:

```
      )WSID
CLEAR WS
      )WSID DEMO
WAS CLEAR WS
      )WSID OTHER
WAS DEMO
```

The three settings reply the same way, and refuse a value outside
their range:

```
      )ORIGIN 0
WAS 1
      )ORIGIN 2
INCORRECT COMMAND
      )DIGITS 5
WAS 10
      )WIDTH 40
WAS 120
```

They belong to the workspace and are saved with it.

`)COPY` takes the definitions and leaves the settings, which is the
difference from `)LOAD` that matters: a copied function runs under
*your* index origin, which may not be the one it was written for.
See `index-origin-considerations.md`.

```
      )COPY DEMO              ⍝ everything
      )COPY DEMO A F          ⍝ these names
      )COPY 1 EDIT MEAN       ⍝ from library 1
```

A group among the names brings its members with it. `)PCOPY` is
`)COPY` that will not overwrite a name you already hold.

Both reply with the moment the source workspace was stored, which is
the line `)LOAD` prints, and `)PCOPY` names what it kept:

```
      )PCOPY CLASS
SAVED 18.52.48 09/18/26
NOT COPIED: A
```

Without that second line a protected copy that skipped the name you
asked for would look like one that worked. An ordinary `)COPY`
overwrites, so it never keeps anything and never prints it.

`)GROUP` and `)ERASE` are in `session.md` under **Groups**, with the
rules for adding to a group and dispersing one.

## Library control

| Form | |
|---|---|
| `)SAVE` | Store this workspace under its own name |
| `)SAVE name` | Name it and store it |
| `)LOAD [lib] name` | Replace this workspace with a stored one |
| `)DROP name` | Forget a stored workspace |
| `)LIB [n]` | List the workspaces in a library |

`)SAVE` replies with the moment and the name it wrote:

```
      )SAVE
17.00.12 09/18/26 DEMO
```

`)LOAD` replies with the moment the workspace was *stored*, and
nothing else. Typing `DESCRIBE` is the reader's move, not the
loader's:

```
      )LOAD DEMO
SAVED 17.00.12 09/18/26
```

`)DROP` replies with the moment it dropped, as APL\360's did -- it
does not echo the name. It takes no library number, because library
0 is the only one you can write to.

`)LIB` lists library 0, `)LIB 1` library 1, `)LIB 2` a configured
one: one at a time. An empty
library prints nothing, which is not the same as one that does not
exist.

### What a workspace may be called

A workspace name is an APL name: a letter, `∆` or `⍙`, then letters,
those two, and digits. Anything else is `INCORRECT COMMAND`, and it
is refused before it reaches a filename:

```
      )SAVE WS:PASS
INCORRECT COMMAND
      )SAVE A/B
INCORRECT COMMAND
      )LOAD ../DONOR
INCORRECT COMMAND
```

The rule is APL\360's own, and it is also what keeps a name from
naming something other than a workspace. `)WSID` holds it to the
same rule, so a workspace cannot be given a name it could not be
saved under.

Lower case is a name here, as it is everywhere else in sw-apl,
though APL\360 had not got it. Take care on a case-insensitive
filesystem: `ABC` and `abc` are one workspace on macOS and two on
Linux.

`workspaces.md` has the libraries, the file format, and what a
workspace holds.

## Inquiry

| Form | |
|---|---|
| `)FNS [letter]` | Defined function names, alphabetically, from a letter |
| `)VARS [letter]` | Global variable names, the same |
| `)GRPS [letter]` | Group names, the same. (A) only |
| `)GRP name` | What a group gathers, in the order it was gathered. (A) only |
| `)SI` | The state indicator: each function with the line it stopped on |
| `)SIV` | As `)SI`, with the names each call made local |
| `)SYMBOLS` | How many names are held, and how many would fit |

`)VARS` lists *global* variables. Inside a suspended function the
call's locals are in scope, and the listing is of the globals
either way.

`)SI` stars the function an error came from -- the one you can take
up again -- while the callers waiting on it are pendent:

```
      )SI
FAILS[1]*
```

With nothing suspended both print nothing. A bare `→` clears the top
entry.

`)SYMBOLS` reports `IS n, USED m`. The number cannot be set here:
sw-apl sets no symbol table aside, so `)SYMBOLS n` is `INCORRECT
COMMAND`. `session.md` has why.

## What sw-apl does not have

APL\360 ran on a shared machine: accounts signed on at ports, an
operator watched over them, and users sent each other messages.
sw-apl runs on yours. These commands are therefore not implemented,
and answer `INCORRECT COMMAND` like any other name the session does
not know.

| Form | What it did |
|---|---|
| `)NUMBER [key]` | Sign on an account and start a session |
| `)OFF HOLD` | End the session but hold the dial-up connection |
| `)CONTINUE HOLD` | Save, end the session, hold the connection |
| `)MSG port text` | Send a message to another terminal and wait for it to be received |
| `)MSGN port text` | Send one without waiting |
| `)OPR text` | Send a message to the operator and wait |
| `)OPRN text` | Send one without waiting |
| `)PORTS` | List the ports in use and who is signed on at each |

The locks and keys go with them. `)SAVE WSID:LOCK` stored a
workspace behind a password so that someone else with access to the
library could not read it, and `)LOAD WSID:KEY` gave the password.
sw-apl has one user and no shared library, so there is nothing to
lock against, and the colon is not a name character: the form is
refused rather than taken as part of the name.

## What sw-apl adds

Commands no historical system had, in both modes. They are system
commands, never quad names, so a program or a saved workspace cannot
depend on them, and a workspace written for either mode stays one its
historical system could run.

| Form | |
|---|---|
| `)DIALECT` | The session's mode: `(A) '70` or `(B) '75` |
| `)LIBS` | The libraries the session reaches, by number and name |
| `)HELP [NAME]` | The commands this mode has, by group; with a name, a page on a command or a topic |

```
      )DIALECT
(B) '75
```

`)DIALECT` only asks. The mode is chosen when a session starts --
`--mode` at the command line and the service, the tab in a browser --
and `)DIALECT 70` is `INCORRECT COMMAND`. Like any command longer
than four characters, it can be cut short: `)DIAL`.

`)LIBS` lists the libraries: 0 and 1 always, and any configured
beyond them. Only library 0 is written to.

```
      )LIBS
0 USER
1 CORE
2 EXTENDED
```

`)HELP` lists the commands the session's mode has, grouped as the
manual groups them, with sw-apl's own on a line of their own:

```
      )HELP
SYSTEM COMMANDS IN (B) '75. )HELP NAME FOR ONE.
WORKSPACE    )CLEAR )WSID )COPY )PCOPY )ERASE
LIBRARY      )SAVE )LOAD )DROP )LIB
INQUIRY      )FNS )VARS )SI )SIV )SYMBOLS
TERMINAL     )OFF )CONTINUE
SW-APL'S OWN )DIALECT )LIBS )HELP
)HELP TOPICS FOR MORE: THE MODES, AND SW-APL'S OWN.
```

`)HELP NAME` shows a page: a command, with or without its
parenthesis (`)HELP LOAD`, `)HELP )LOAD`), or a topic -- `TOPICS`,
`MODES`, `EXTENSIONS`, `RESTRICTIONS`. A command the mode has not got
says so (`(B) '75 HAS NO )ORIGIN.`), and a name with no page says to
ask `)HELP TOPICS`. The pages are `data/help.txt` in the repository,
and every line fits the 64 columns of the (B) screen.

## What sw-apl has and APL\360 did not

These are not commands. A workspace on a 360 was sized and placed by
the system you signed on to; here they are yours to set, from the
command line:

| | |
|---|---|
| `--library DIR` | The directory the libraries sit under. Library 0 is `DIR/work`, library 1 is `DIR/ws/lib1` |
| `--lib N=DIR[,NAME]` | Library N (2 and up) is the workspaces in DIR, read-only; again for another |
| `--config FILE` | Read libraries from FILE's `[[library]]` tables, in place of `./sw-apl.toml` or `~/.config/sw-apl/config.toml`. See `workspaces.md` |
| `--ws-size BYTES` | How much the workspace may hold before `WS FULL` |
| `--mode 70` or `--mode 75` | The mode, (A) '70 or (B) '75; the default is 70 |

`sw-apl --help` has the rest of the command line.

## When a command cannot do it

The manual calls these trouble reports. The distinction it draws:
**INCORRECT COMMAND is about the command**, not about the workspace.

| Reply | Means |
|---|---|
| `INCORRECT COMMAND` | A missing argument, one word too many, or a value out of range |
| `WS NOT FOUND` | No stored workspace of that name |
| `OBJECT NOT FOUND` | The workspace is there but holds no such name |
| `IMPROPER LIBRARY REFERENCE` | That number is not a library |
| `NOT SAVED, THIS WS IS name` | `)SAVE name` would overwrite a stored workspace that is not this one |
| `NOT GROUPED, NAME IN USE` | The first name of a `)GROUP` already holds a function or a variable |
| `NOT ERASED: names` | `)ERASE` left those: they are functions on the state indicator |
| `WS FULL` | What was asked for does not fit in the workspace |

A comma introduces a reason and a colon introduces a list, which is
the manual's own convention. `samples/65-trouble-reports.apl` walks
them.
