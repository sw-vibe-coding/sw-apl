Phase 4 step 4 (docs/plan.md). The inquiry commands: )FNS )VARS )GRPS )GRP )GROUP )ERASE )SYMBOLS.

)FNS and )VARS list the names a workspace holds, in the order and column layout APL\360 used, and each takes an optional letter to start from. )ERASE removes names. )GROUP, )GRPS and )GRP are the group facility, which collects names under one name so they can be erased or copied together. )SYMBOLS reports, and in APL\360 could set, the size of the symbol table; sw-apl has no fixed symbol table, so report and say what the number means.

reg-rs has a lines-unordered diff mode, which is the right one for a listing whose order sw-apl does not promise to match. Use it if, and only if, the order genuinely is not promised -- if APL\360's order is known and reproducible, pin the order instead and say so.

apl-session was split for this step: the commands live in apl-commands, which is at three modules. Expect to split again rather than fold.

TDD; reg-rs for anything run through the binary; update docs/parity.md rows in the same commit; commit, push, report.
