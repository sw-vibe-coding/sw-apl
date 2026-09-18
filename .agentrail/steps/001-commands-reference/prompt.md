Phase 6 step 1 (docs/plan.md, owner request 2026-09-18). Write
docs/commands-reference.md.

The system commands are documented in three places today and none of
them is a reference: a 21-row table of one-line glosses in
session.md, status rows in parity.md, and a gloss in the CLI help. A
reader who wants to know what )COPY replies when the workspace is
not there has to find the trouble-report table in session.md and
join it up for themselves.

Write the reference. Every system command APL\360 had, in the
manual's own groups -- terminal control, workspace control, library
control, inquiry, communication -- and for each one:

- The form, with its optional parts.
- What it does.
- What it replies when it works, exactly as sw-apl prints it.
- What it refuses and with which trouble report.
- A worked example where the form is not obvious: )COPY with and
  without object names, )GROUP adding to a group, )LOAD from a
  numbered library.

Include the commands sw-apl does NOT implement, and say why per
command rather than leaving the reader to infer absence:
)NUMBER, )OFF HOLD, )CONTINUE HOLD, )MSG, )MSGN, )OPR, )OPRN,
)PORTS. They are the multi-user surface of a shared machine with
accounts, ports and an operator; parity.md already puts that class
out of scope, and this is where a reader finds out. Note that
)MSGN and )OPRN are not even in parity.md's row today.

Also say what sw-apl adds that APL\360 had no need of: --library,
--ws-size, and that )SYMBOLS reports but cannot be set.

Check every reply against the binary before writing it down. The
last reference step caught two of its own claims that way. Where a
reply is the time or the date, say so rather than printing one run's
value as though it were fixed.

session.md keeps its tables and links here rather than repeating;
parity.md keeps the status. Link the reference from README.md's docs
list. The user-facing docs rule applies: what and how, never when or
plans.

If writing it turns up a behaviour that is wrong rather than merely
undocumented, do not fix it here -- steps 2 and 3 already exist for
the two known ones. Add a step for anything new.
