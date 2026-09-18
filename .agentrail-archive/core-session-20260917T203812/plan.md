# core-session

Phase 3 of docs/plan.md: defined functions, branching, the del
editor, the state indicator, quad input, and the I-beam system
functions. This is the phase that turns sw-apl from a calculator
into APL\360: after step 2 the horse race and Life run as functions.

Pure APL\360 throughout: del editor (no line editor for function
bodies), dynamic scoping, I-beams for system information, no quad
names. Every step: format first, then tests, clippy, and gates
(see /mw-cp); TDD; update docs/parity.md rows in the same commit;
commit, push, report.

## Steps

1. user-functions -- the del definition form and the function call.
   Headers: NAME, NAME B, A NAME B, each with or without R←;
   locals after semicolons; the symbol table holds functions beside
   variables; calls bind arguments, shadow locals, restore on exit;
   recursion; VALUE ERROR for a result-less function used for its
   value; SYNTAX ERROR for wrong valence. Multi-line definition in
   immediate execution ends at the closing del.
2. branch-and-labels -- labels as local constants holding line
   numbers, → with an expression (first element selects the line,
   empty vector falls through, 0 or out-of-range exits), the
   →LABEL×⍳COND idiom, execution order and the line counter. After
   this step samples 50 (horse race) and a function form of Life
   run; seed their baselines.
3. del-editor -- definition mode prompt [n], display [⎕] and [n⎕],
   replace [n], insert at fractional line numbers, delete [∆n],
   header edit [0], close with del or del-tilde (locked), reopen an
   existing function with ∇NAME, DEFN ERROR cases.
4. error-display-and-state-indicator -- FN[n] prefixes on errors
   inside functions, suspended functions, )SI and )SIV, a bare →
   clearing the top entry, resumption, DEPTH ERROR guard.
5. quad-input -- ⎕ on the right evaluates a typed line (in the
   session and inside functions), ⍞ character input and output
   without a newline, interrupt handling.
6. i-beams -- ⌶20 through ⌶27 (time of day, CPU time, workspace
   available, terminals, sign-on time, date, current line, state
   indicator lines) in sixtieths of a second where APL\360 used
   them; DOMAIN ERROR elsewhere. A STIR idiom sample showing how to
   advance the random link from the clock.
7. session-polish -- the )OFF sign-off line with connect and CPU
   time, interrupt during a running statement, and a pass over the
   session transcript against docs/session.md.
