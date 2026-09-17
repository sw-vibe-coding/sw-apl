# Using the del editor

The del editor is how you write and change functions in sw-apl.
There is no separate text editor and no file to open: you type a
del, the session leaves immediate execution, and every line you
type afterwards goes into the function until you type a del again.

This guide is a walkthrough, and it is one continuous session: the
prompts carry on from section to section, because where the prompt
stands is most of what there is to learn. `session.md` has the same
commands as a reference table, and `samples/55-del-editor.apl` is a
transcript you can run with `sw-apl -f samples/55-del-editor.apl`.

## The two prompts

Immediate execution prompts with six spaces:

```
      2+2
4
```

Definition mode prompts with the number of the line you are about
to write, in brackets, in the same six columns:

```
      ∇R←AREA H
[1]   
```

The prompt is the whole state of the editor. `[1]` means "the next
thing you type becomes line 1". Reading the prompt tells you where
you are; nothing else does.

## Writing a new function

Open with a del and a header, then type the body, then close with
a del:

```
      ∇R←AREA H;W
[1]   W←H+1
[2]   R←H×W
[3]   ∇
      AREA 3
12
```

The header is `NAME`, `NAME B`, or `A NAME B`, with `R←` in front
when the function returns a result, and local names after
semicolons. `AREA` above takes one argument `H`, returns `R`, and
makes `W` local for the length of the call.

The closing del is typed at the `[3]` prompt. That is the usual
shape: the last line of a transcript shows a prompt with nothing
in the function at that number, and a del.

DEFN ERROR when you open means the header did not parse, or the
name is already in use. sw-apl will not silently replace a
function or a variable; erase it first, or reopen it (below).

## Reopening a function

Type a del and the name alone -- no header:

```
      ∇AREA
[3]   
```

You are positioned after the last line, ready to append. Type a
del to close again. Reopening never loses anything: the lines are
exactly as you left them.

## Seeing what is there

`[⎕]` displays the whole function, framed by its dels the way you
would type it:

```
[3]   [⎕]
      ∇R←AREA H;W
[1]   W←H+1
[2]   R←H×W
      ∇
[3]   
```

`[2⎕]` displays from line 2 onward, without the header. Displaying
does not move the prompt, so you can look and carry on typing.

From immediate execution, `∇AREA[⎕]∇` opens, displays, and closes
in one line. That is the usual way to read a function without
meaning to change it.

## Changing a line

Type the number in brackets and the new text:

```
[3]   [1] W←H+10
[2]   
```

Line 1 is replaced. Note where the prompt went: after writing line
1 it offers line 2, so typing a second line now would replace line
2 as well. That is deliberate -- retyping a run of lines is the
common case -- but it means you should glance at the prompt before
typing anything you did not plan.

`[1]` on its own, with no text after it, just moves there:

```
[2]   [1]
[1]   
```

## Inserting a line

Line numbers may be fractional. To put a line between 1 and 2,
give it a number between 1 and 2:

```
[1]   [1.5] W←W+100
[1.6] 
```

The prompt then steps at the grain you used: after `[1.5]` comes
`[1.6]`, so you can keep inserting. To go finer, use another
decimal: `[1.55]` sits between 1.5 and 1.6. Three decimal places
is the limit, which gives you room to insert between any two
lines three times over.

Fractional numbers only exist while the function is open. Closing
it renumbers every line from 1:

```
[1.6] ∇
      ∇AREA[⎕]∇
      ∇R←AREA H;W
[1]   W←H+10
[2]   W←W+100
[3]   R←H×W
      ∇
```

**This is why you branch to a label and not to a number.** A
label follows its line through every insert and delete; a literal
line number in a `→` does not.

## Deleting a line

`[∆n]` deletes line n. The delta is U+2206, the same character
used in names. Reopening AREA -- now three lines -- puts us at
`[4]`:

```
      ∇AREA
[4]   [∆2]
[4]   
```

Deleting does not move the prompt. Deleting a line that is not
there is DEFN ERROR -- a typed number that matches nothing is far
more likely to be a slip than an intention.

## Changing the header

`[0]` is the header. Give it new text to replace it:

```
[4]   [0] R←AREA H;W;T
[1]   
```

This is how you add a local, change an argument name, give a
function a result it did not have, or rename it. A rename moves
the function: the old name is erased, not left holding a copy.

Watch the prompt after a header edit. It offers `[1]`, not the end
of the function, so the next thing you type replaces line 1 unless
you move first. After `[0]`, type a del.

## Closing and locking

`∇` closes the definition, renumbers the lines from 1, and stores
the function. Everything you typed takes effect at that moment.

`⍫` (del-tilde) closes it locked. A locked function still runs,
but it can never be reopened or displayed again -- both are DEFN
ERROR -- and there is no way to unlock it. Lock a function only
when you are certain, and only after you have a copy of the source
somewhere else.

```
      ∇SECRET
[1]   'THE ANSWER IS 42'
[2]   ⍫
      SECRET
THE ANSWER IS 42
      ∇SECRET
DEFN ERROR
      ∇SECRET
      ^
```

## Doing several things at once

Anything after a bracketed command on the same line is taken as
the next thing you typed. That is why `[1] W←H+1` works: it is
`[1]` followed by text. It is also why `∇AREA[⎕]∇` works: reopen,
display, close.

## Errors while editing

A DEFN ERROR inside definition mode prints and leaves you exactly
where you were, with the function untouched. Nothing you have
typed is lost, and you do not have to start again:

```
[4]   [∆9]
DEFN ERROR
      [∆9]
      ^
[4]   
```

You can also leave a broken function open, close it, and fix it
later -- the editor does not check that lines are valid APL. A
line that will not parse fails when the function runs, not when
you write it.

## Quick reference

| Typed | What it does |
|---|---|
| `∇HEADER` | Open a new function |
| `∇NAME` | Reopen NAME after its last line |
| `∇NAME[cmd]` | Reopen and run one command at once |
| `TEXT` | Write TEXT at the prompt's line, then move on |
| `[n]` | Move to line n |
| `[n] TEXT` | Write TEXT at line n |
| `[⎕]` | Display the whole function |
| `[n⎕]` | Display from line n on |
| `[∆n]` | Delete line n |
| `[0]` | Move to the header |
| `[0] HEADER` | Replace the header |
| `∇` | Close, renumbering from 1 |
| `⍫` | Close locked |
