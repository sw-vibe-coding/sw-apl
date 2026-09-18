Phase 4 step 4 (docs/plan.md, owner direction 2026-09-17). Give the
workspace a size.

Today a workspace is however much memory the Rust process will give
it, which is too much to be a workspace at all. I-beam 22 answers a
constant it did not measure, and WS FULL is a parity row that is
never raised. Both are the same missing thing: an account of how
much the workspace holds, and a limit it is held against.

What to build:

1. A size for a value, in bytes, that is a property of the workspace
   model and not of Rust's allocator -- stable across runs, machines
   and optimisation levels, because a test has to be able to state
   it. Numbers, characters, the shape, and the per-name overhead of
   a symbol table entry. A defined function costs what its text
   costs. Say in the code what the accounting is and why, because a
   reader will otherwise assume it is measuring the heap.
2. A quota on the workspace, with a sensible default and a way to
   change it. The quota belongs to the SESSION, not the workspace:
   like the console and the clock it is not saved, so a workspace
   written under a large quota need not fit under a small one.
   That is what happened on a real APL\360 and is the reason )COPY
   of a few names mattered as much as )LOAD.
3. I-beam 22 reports the space still free -- quota less what is
   held -- rather than the constant it reports now.
4. WS FULL raised where it can be: assignment of a value that will
   not fit, a definition that will not fit, and a )LOAD or )COPY
   of a workspace too big for the quota. A failed operation must
   leave the workspace as it was; a half-loaded workspace is worse
   than a refused one.

Tests: unit tests on the accounting itself (a known shape has a
known size), and a sample that shows I-beam 22 falling as names are
defined and rising again after )ERASE or )CLEAR. Drive the WS FULL
paths from a session test with a small quota rather than by
allocating something enormous -- a test that needs a gigabyte to
fail is a test nobody will run.

Watch the reg-rs baselines: sample 58-ibeams prints I-beam 22 and
its answer is about to stop being a constant. Either the sample
masks it as (VARIES) or it prints something the default quota makes
deterministic; decide which, and say why in the commit.

Docs: docs/workspaces.md has a section "How much room is left" that
currently says the figure is nominal and WS FULL is never raised --
it is the first thing that must change. Update docs/parity.md's
WS FULL row, docs/session.md if the option is user-visible, and the
README if it gains a flag.
