# Literate APL: Emacs, Org Babel and sw-apl

sw-apl comes with two Emacs files for writing about APL in Org mode,
with the APL run as you read:

| File | What it is |
|---|---|
| `docs/emacs/sw-apl-mode.el` | A major mode for sw-apl source: `.apl` scripts, `.apl.ws` workspaces, and the APL blocks of an Org document. It colours primitives, the arrows, numbers, strings, comments, quad names, system commands, the del and the function it opens, and labels |
| `docs/emacs/ob-sw-apl.el` | An Org Babel language, `sw-apl`: a source block is run by sw-apl, and what it prints is the block's result |

`docs/emacs/apl-input.el`, the input method, is the third; see
`glyph-entry.md`.

## Installing

```elisp
(add-to-list 'load-path "path/to/sw-apl/docs/emacs")
(require 'sw-apl-mode)
(require 'ob-sw-apl)
(org-babel-do-load-languages 'org-babel-load-languages
                             '((sw-apl . t)))
```

Blocks run the `sw-apl` on your `PATH`, or in
`~/.local/softwarewrighter/bin`. To run a particular build, name it:

```elisp
(setq org-babel-sw-apl-command "path/to/sw-apl/target/release/sw-apl")
```

## A block

```org
#+begin_src sw-apl
∇R←AVG X
R←(+/X)÷⍴X
∇
AVG 3 5 10
#+end_src

#+RESULTS:
: 6
```

`C-c C-c` in the block runs it. Each block is a run of its own, in a
clear workspace, the way `sw-apl -f FILE` runs a script: a function
one block defines is not there in the next. A block that needs a
workspace loads it (`)LOAD 1 BIRDS`), or takes the definitions it
needs from another block by noweb reference (`:noweb yes` and
`<<name>>`).

| Header argument | |
|---|---|
| `:mode 75` or `:mode 70` | The mode, (B) '75 or (A) '70. (B) is the default |
| `:echo yes` | Show each line as typed, six spaces in, so the result reads as a session: the line, then what it printed |
| `:library DIR` | Where libraries 0 and 1 are, as `sw-apl --library` |
| `:lib "2=DIR,NAME"` | A library beyond them, as `sw-apl --lib` |
| `:session` | Refused: a block runs and ends |

An error is part of the result, as it would be on the terminal:

```org
#+begin_src sw-apl
1 2+1 2 3
#+end_src

#+RESULTS:
: LENGTH ERROR
:       1 2+1 2 3
:          ^
```

Blocks tangle to `.apl` files, and are shown and edited in
`sw-apl-mode`.

## Publishing with colour

Exported to HTML with htmlize set to CSS classes, every kind the
mode colours has a class of its own, named for its face:

```elisp
(setq org-html-htmlize-output-type 'css)
```

| Class | Kind |
|---|---|
| `org-sw-apl-primitive` | A primitive function or operator |
| `org-sw-apl-arrow` | `←` and `→` |
| `org-sw-apl-number` | A number, high minus included |
| `org-sw-apl-system` | `⎕`, `⍞` and the quad names |
| `org-sw-apl-command` | A system command line |
| `org-sw-apl-del` | `∇` and `⍫` |
| `org-function-name` | The function a header names |
| `org-sw-apl-label` | A label |
| `org-string`, `org-comment` | Strings and comments |

The document's own stylesheet (an `#+HTML_HEAD:` line) says what
colour each is.

## Tests

`just test-emacs` runs the ERT tests in `docs/emacs/test/` under a
batch Emacs, against `target/release/sw-apl`. Where there is no
Emacs it says so and passes.
