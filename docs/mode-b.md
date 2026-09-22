# The (B) '75 mode

The mode is called (B) '75, and only that. IBM product names appear
here to say which historical machines and manuals the mode is
modelled on, never as the name of the mode or of sw-apl.

A planning document. It records, from IBM's manuals, what the APL of
the IBM 5100 family is and how it differs from APL\360, so that the
(B) mode is built from sources rather than from memory. Every
statement below is paraphrased from the sources listed in
`docs/citations.md`; a statement that is a guess or a decision still
to be made says so.

## What (B) is

The owner's direction (2026-09-21): (A) recreates learning APL on an
IBM 2741 connected to APL\360; (B) recreates using APL professionally
on the IBM 5100 to 5120 desktop computers. The goal is an IBM 5120 on
a phone, not a System/370 APLSV. The year in the mode's name is the
5100's release: IBM announced it, with APL, in September 1975,
for delivery the same month, and its first
APL reference manual is dated August 1975.

IBM described the 5100's APL, and then the 5110's, as APLSV with the
parts removed that only make sense for a time-shared system of many
terminals, plus commands for the machine's own tape, diskette and
printer. Each reference manual carries an appendix listing exactly
those differences (5100 manual, Appendix D; 5110 manual, Appendix E),
and the 5110 manual carries a second listing what the 5110 changed
from the 5100 (Appendix D). Those three appendices are the backbone
of this document.

**Which manual (B) follows.** (B) is modelled on the 5100 family
(5100, 5110, 5120). The reference manual it is built from
is the 5110's. It is the later of the
two manuals, it is the one the 5120 ran (the 5120 is a 5110-class
machine with the display and diskettes built in; bitsavers carries no
separate 5120 APL manual), and every 5100 workspace loads on it. Where
the 5100 differs, the 5100's behaviour is noted but not built.

## Sources found, and not found

Found, and read (all listed in `docs/citations.md`):

- IBM 5110 APL Reference Manual, SA21-9303-0, December 1977 -- the
  primary source for (B).
- IBM 5110 APL User's Guide, SA21-9302-1, August 1978 -- tutorial;
  used for examples only.
- IBM 5100 APL Reference Manual, SA21-9213-2, May 1976.
- APL Language, GC26-3847-0, March 1975, and the APLSV User's Guide,
  SH20-1460-1, March 1975 -- what the 5100 family inherited, read
  where the 5110 manual defers to APLSV behaviour.

Not found: any APL manual specific to the 5120. The bitsavers 5120
directory holds only maintenance and logic manuals. That the 5120's
APL is the 5110's is therefore a strong inference (same system
software, same publications), not a statement from a manual.

## What (B) adds over APL\360

From the 5110 manual, Chapters 4 and 5:

- **Execute**, `⍎B`: evaluates a character scalar or vector as an APL
  expression; monadic only.
- **Format**, `⍕B` and `A⍕B`: monadic gives the characters the display
  of `B` would show; dyadic takes width-and-precision pairs, a
  negative precision giving scaled form, a zero width meaning "wide
  enough for one blank between numbers". Semicolon catenation for
  output, which APLSV replaced by format, is still accepted.
- **System variables**: `⎕CT` comparison tolerance, `⎕IO` index origin,
  `⎕PP` printing precision, `⎕PW` printing width, `⎕RL` random link,
  `⎕LC` line counter, `⎕WA` workspace available, `⎕LX` latent
  expression, `⎕AV` atomic vector. The 5110 also has `⎕AI`, `⎕DL`,
  `⎕TS`, `⎕TT` and `⎕UL` for syntactic compatibility with APLSV only:
  the machine has no clock and one user, so they hold fixed values
  (the time stamp is 1900 0 0 0 0 0 0). These five are 5110
  additions; the 5100 lacks them.
- **System functions**: `⎕CR` canonical representation, `⎕FX` fix,
  `⎕EX` expunge, `⎕NL` name list, `⎕NC` name classification, and the
  5110's own `⎕CC` console control (screen and keyboard control; the
  5100 lacks it).
- **Shared variables** (`⎕SVO` and relatives): on the 5100 family
  they are the only way to reach tape, diskette, printer and screen
  files -- one partner, processor 1, always accepted. See the
  decisions below.
- Characters `$ # @ & % "` and others, added for exchange with BASIC.
  The 5110 also accepts lowercase letters from the keyboard.

## What (B) drops from APL\360

Sourced, not guessed -- both reference manuals say so in their APLSV
appendices:

- **The I-beam functions.** Replaced by system variables and
  functions and not supported; using one is a NONCE ERROR (5110
  manual, Appendix E and Appendix F under NONCE).
- **`)ORIGIN`, `)WIDTH`, `)DIGITS`.** Not supported; the manuals send
  the user to `⎕IO`, `⎕PW` and `⎕PP` instead.
- Commands for a time-shared system: `)OFF`, `)OFF HOLD`,
  `)CONTINUE HOLD`, `)PORTS`, the message commands, the operator
  commands, and the group commands `)GROUP`, `)GRP`, `)GRPS`. Power
  on is sign on.

These are the lists step 006 pulls out into '68-only crates. The
I-beams and the three settings commands are confirmed.

**Done (step 007):** the I-beams (`apl-ibeam`) and the settings and
group commands (`apl-a68-commands`) are in `components/a68/`, and the
shared core reaches them only in (A). In (B) an I-beam is a NONCE
ERROR, as the 5110 manual gives, and the six commands are INCORRECT
COMMAND, as any command the system does not have is. `)OFF` stays in
both modes: it is sw-apl's only way to end a session, which on the
5100 was the power switch. The time-sharing commands were never in
sw-apl.

## What (B) changes

- **Clear workspace**: `⎕IO` 1, `⎕CT` 1E¯13, `⎕PW` 64, `⎕PP` 5,
  `⎕RL` 16807, WSID CLEAR WS. The width and precision follow from the
  screen, which is 64 characters by 16 lines. Setting `⎕PW` above 64
  wraps output on the screen.
- **Library numbers are device and file numbers.** A one- to
  five-digit number: up to three digits is a file on the default
  device, otherwise the leading digits name the device.
- **Library messages** carry no time stamp: `SAVED`, `LOADED`,
  `COPIED`, `CONTINUED`, `DROPPED`, each followed by device/file and
  workspace name. `)LIB` lists every file on the device.
- **`)CONTINUE`** saves a workspace with suspended functions;
  `)SAVE` refuses one (NOT WITH SUSPENDED FUNCTION).
- **Symbols**: 125 by default, not 256.
- **Line length**: statements over 115 characters cannot be saved or
  edited (LINE TOO LONG).
- **Bare output then bare input**: the `⍞` input line is prefixed by
  the previous `⍞` output itself, not by the same number of blanks.
- **Unbalanced quotes**: the 5110 closes an odd quote for the user.
- **Interrupts**: ATTN is a weak interrupt -- execution stops at the
  end of the current statement, resumable with `→⎕LC`. Shift+ATTN is
  a strong interrupt: it stops as soon as possible and reports
  INTERRUPT with a caret (the strong interrupt is a 5110 addition).
  sw-apl's ATTN today behaves as the strong one.
- **SYSTEM ERROR** keeps the workspace for diagnosis, and only
  `)CLEAR` is accepted afterwards.

## The del editor differs

The owner assumed the editor was the same in both modes. The sources
say otherwise, because the 5100 family edits on a screen rather than
on paper (5110 manual, Chapter 6 and Appendix E):

- `[n]`, insertion by decimal numbers, `[0]` for the header, and
  reopening with `∇R` are the same.
- `[n⎕]` displays line n in the two input lines for editing in
  place: characters are overtyped, CMD+backspace deletes the
  character at the cursor, CMD+forward-space opens a gap, ATTN erases
  to the end of the line, EXECUTE enters the line. `[n⎕m]` does the
  same (the m is dropped). There is no slash line: APL\360's
  character-deletion line under the statement does not exist.
- `[∆n]` deletes line n; the closing `∇` may not share its line.
- `⎕PW` is 128 while a definition is open, so a long statement shows
  whole, and returns afterwards.
- Editing a pendent function is a DEFN ERROR, as in APLSV.

For sw-apl this means the (B) editor is the shared editor plus `[∆n]`
and a screen form of `[n⎕]`; on a phone, "display the line in the
input area to be edited" maps naturally onto the input field. Which
parts of APL\360's `[n⎕m]` stay reachable in (B) is a decision for
the step that builds it.

## The workspace file

Today a saved workspace writes its settings as `)ORIGIN`, `)DIGITS`
and `)WIDTH` lines. (B) has none of those commands, so by the modes
rule every workspace saved so far would be (A)-only. Step 005 must
write the settings in a form both modes read -- `⍝!` directives, as
the random link already is -- and read the old lines when loading in
(A).

The defaults differ, so a workspace records its values rather than
relying on the clear-workspace ones: a (A)(B) workspace loaded in (B)
keeps the width and digits it was saved with.

## Could (B) load an APL\360 workspace?

The sources say only that a 5110 loads workspaces the 5100 saved with
`)SAVE` (not its continued ones), and that nothing moved between a
5100 and a System/370 except by exchanging data files. A 5100 never
read an APL\360 workspace. For sw-apl the modes line decides instead:
an (A)(B) workspace is shared by construction. **Decision, not
history.**

## The keyboard

- **5100 and 5110**: the same layout, per the 5110 manual's list of
  changes from the 5100, which names no layout change. The 5110 adds
  a lowercase mode (HOLD, then shift+scroll-down; back with
  shift+scroll-up) in which the APL symbols move to the CMD key, and
  the shift+ATTN strong interrupt.
- **5120**: the same layout as the 5100, differing in keycap colour
  (a secondary source, the keyboard pages collected by Deskthority;
  treat as likely, not confirmed by IBM).
- **How it differs from the 2741** (5110 manual, Chapter 1): letters
  unshifted, APL symbols shifted, as on the 2741; a numeric keypad
  with its own `+ - × ÷`; a CMD key, which with the top row gives the
  system command keywords engraved above it and with other keys gives
  the characters not engraved on any key; ATTN, HOLD, EXECUTE (in
  place of Return), scroll keys, and COPY DISPLAY.
- **A picture**: Marcin Wichary's photograph of an IBM 5100 keyboard
  is on Wikimedia Commons (`File:Ibm5100_(2297950254).jpg`). Its
  licence is to be checked on the file page before anything is carried;
  a photograph is not an adaptable drawing as the 2741 SVG is, so a
  (B) board may have to be drawn from scratch from the layout.

## Decisions for the owner

Guesses and choices the sources do not make:

1. **Tape, diskette and device commands** -- `)MARK`, `)REWIND`,
   `)OUTSEL`, `)FILEID`, `)FREE`, `)PROTECT`, `)VOLID`, `)LINK`,
   `)PROC`, `)SORT`, `)PATCH`, `)MODE`. Proposed: not implemented, a
   present-tense "Not implemented." in the docs, with device/file
   numbers in `)SAVE`/`)LOAD` accepted as library numbers.
2. **Shared variables** -- on a 5100 they are how programs read and
   write files, so real 5100 programs use them. Proposed: stay out,
   as the non-goals say, until asked.
3. **The 16-line, 64-column screen** -- (B) could show output as the
   5110 display does (scrolling screen, input on the bottom two
   lines) rather than as paper. Proposed: keep the paper transcript,
   use `⎕PW` 64, and leave the screen look to the keyboard step.
4. **Weak and strong interrupt** -- proposed: ATTN weak and
   shift+ATTN strong in (B); (A) unchanged.
5. **The tabs' tooltips** -- settled by the owner (2026-09-21): no
   IBM product names as the name of anything, so the tooltips read
   "APL\360-inspired" for (A) and "IBM 5100-inspired" for (B). Step
   014 changes them.
6. **Tape and diskette libraries** -- the 5100 kept workspaces in
   numbered files on a tape cartridge (`)MARK` formatted a tape into
   files, and `)LIB` listed every file on it with its type and size);
   the 5110 and 5120 added diskettes. Proposed: (B) keeps sw-apl's
   libraries 0 and 1, and a device/file number is not implemented.
