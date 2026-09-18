Phase 6 step 3 (docs/plan.md). A colon in a workspace name.

APL\360's )SAVE took an optional lock -- a password -- after the
workspace name, and )LOAD and )COPY took a matching key:

  )SAVE WSID [LOCK]
  )LOAD WSID [KEY]

sw-apl does not parse either, so the colon is taken as part of the
name and this stores a workspace literally called `WS:PASS`:

      )SAVE WS:PASS
16.54.20 09/18/26 WS:PASS

It writes the file `WS:PASS.apl.ws`, which is a strangely named
workspace nobody asked for, and on a system where a colon is not a
legal filename character it would fail in some other way.

Locks are the multi-user surface again -- they exist so that a
workspace in a shared library cannot be read by whoever finds it --
and sw-apl has no accounts, so implementing them is not wanted.
Refusing the form is.

Decide the reply against the manual's trouble-report table and say
in the commit which one you chose and why the others do not fit.
INCORRECT COMMAND is the likely answer, since a lock is an argument
the command does not take here, but check the table before deciding.

While you are there, settle what a workspace name may contain at
all. A name goes into a filename, so it is not only APL's business:
a slash, a leading dot, `..`, an empty name and a name that is only
punctuation are all worth deciding about deliberately rather than
finding out later. Say what the rule is in
docs/commands-reference.md and docs/workspaces.md, and test the
edges.

Watch that the tests use a library root of their own: this one
writes files, and the repository's work/ is the owner's.
