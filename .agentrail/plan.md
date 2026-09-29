# help-and-libraries

Phase 10 of docs/plan.md (owner direction 2026-09-24): three sw-apl
system commands the historical systems never had -- )DIALECT (which
mode), )HELP (discovery, historical and extension marked), )LIBS (the
libraries a session reaches) -- library configuration outside the
language so workspaces kept in another repository can be found and
used at the CLI, the service and the browser, a LEARN workspace in
library 1, and docs/emulation-policy.md. Then the Phase 8 steps
carried from the modes saga: cup and cap, the base-conversion sample,
and the offline shell. The Linux regression fixtures come first.

docs/plan.md, "Phase 10", holds the design and the decisions the
steps take until the owner says otherwise.

Owner report 2026-09-28: repair the Linux clone's build before the
remaining host-clock flag. Reproduce the compiler mismatch, declare
the Rust requirement, and verify the release build and quality gates.

Owner direction 2026-09-28: cross-build sw-apl for LicheeRV Nano
and Luckfox Pico RV1103 using the hardwarewrighter hello-world Rust
configs. This preempts host-clock work and includes repairing any
32-bit compilation failures, validation, and deployment documentation.

Owner direction 2026-09-28: install CLI, server, and workspace libraries
persistently under /root/sw-apl on both boards. Run each board's server
and connect host aplterm; verify library loading over both connections.
This preempts the host-clock flag.
