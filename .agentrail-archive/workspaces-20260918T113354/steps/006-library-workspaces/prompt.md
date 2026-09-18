Phase 4 step 5 (docs/plan.md). Numbered public libraries and the workspaces sw-apl ships.

Library 0 is work/ and library 1 is ws/lib1/, which apl-commands already knows. This step fills library 1: starter workspaces, each carrying a DESCRIBE that says what it holds and how to start, and each in the plain re-executable format so a reader can open it in an editor.

)LOAD 1 NAME and )LIB 1 already work; check them against what you ship. )LOAD prints the SAVED line and nothing else -- no DESCRIBE hint, settled in design.md D8.

Tracked workspaces are ones we wrote. Give each a provenance line and gate on it, so material from elsewhere cannot be committed by mistake; work/ is where anything converted from elsewhere belongs, and it is gitignored. See docs/aplcourse-how-to.md.

Decide what to ship. Candidates: a workspace of the worked examples from docs/language.md, one demonstrating the del editor, one carrying the horse race and Life from samples/. Each has to earn its place by being worth loading, not by filling the directory.

TDD; reg-rs for anything run through the binary; update docs/parity.md rows in the same commit; commit, push, report.
