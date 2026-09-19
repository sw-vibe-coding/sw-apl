# commands

Phase 6 of docs/plan.md, raised by the owner on 2026-09-18: there is
no reference for the system commands, and `)MSG`, `)OPR` and
`)PORTS` are mentioned in one parity row and nowhere else. Checking
that turned up two parity gaps behind the documentation one.

The commands are documented today in three places and none of them
is a reference: a 21-row table of one-line glosses in
`session.md`, status rows in `parity.md`, and a gloss in
`sw-apl --help`. A reader who wants to know what `)COPY` replies
when the workspace is not there has to find the trouble report
table in `session.md` and join it up themselves.

APL\360 had 26 system commands. sw-apl implements 20. The six it
does not are the multi-user surface of a shared machine -- accounts,
ports, an operator, other users to send messages to -- and they are
out of scope by the rule already in `parity.md`. Saying so per
command, in the reference, is the point: a reader should not have to
infer absence.

Read the manual rather than recalling it. Table 2.1 summarises every
command with its form, its normal response and its trouble reports,
and the detailed sections that follow give the wording. The text is
at
https://archive.org/stream/bitsavers_ibmaplAPL3_8068299/APL_360_Users_Manual_Aug68_djvu.txt
and Table 2.1 is around line 4640. Quote it where it settles a
question, and where it does not, say what was chosen instead.

Every step: format first, then tests, clippy, and gates (see
/mw-cp); TDD; reg-rs for anything run through the binary; update
docs/parity.md rows in the same commit; commit, push, report.

## Steps

1. commands-reference -- `docs/commands-reference.md`, every
   command including the refused ones and why.
2. command-abbreviation -- only the first four characters of a
   command name are significant, which sw-apl does not honour.
3. save-lock-syntax -- `)SAVE NAME:PASSWORD` stores a workspace
   literally called `NAME:PASSWORD`; the colon should be refused.
