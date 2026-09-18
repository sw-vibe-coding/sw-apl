Phase 3 step 8 (docs/plan.md, owner direction 2026-09-17). Make a .apl file executable from the shell.

Today `sw-apl -f FILE`, `sw-apl < FILE`, a shell here-doc, and a pipe all work. A shebang line does not: the kernel invocation succeeds, but sw-apl reads the `#!` line as APL and prints CHARACTER ERROR: U+0023 with the echoed line and a caret before carrying on, so the script works but the transcript starts with three lines of noise. Exit status is already 0.

Skip a first line that starts with `#!` in batch mode, the same way `shell.rs::lines` already tolerates CRLF: the line is neither echoed nor evaluated. Only the very first line, and only when it starts with those two characters -- a `#` anywhere else stays a CHARACTER ERROR, because it is not in the APL\360 character set. The interactive session is unaffected.

Two shebang forms must be covered by CLI tests, both with flags BEFORE `-f`, since clap refuses `--no-echo` as the value of `-f`:

  #!/path/to/sw-apl --no-echo -f
  #!/usr/bin/env -S /path/to/sw-apl --no-echo -f

The bare form splits its arguments on macOS but not on Linux, so `env -S` is the portable one; say so in the docs rather than pretending the bare form is portable.

Documentation: docs/session.md gains a short section on running a file from the shell (the four forms that already work, plus the shebang and which form is portable); the `--help` MODES block gains a line; docs/parity.md gains a row under "Syntax and the session". README only if the Status paragraph needs it. Keep README and other top-level markdown ASCII-only.

TDD: a CLI test per shebang form plus one asserting `#` elsewhere is still CHARACTER ERROR. Add samples/55-script.apl only if a transcript adds something a CLI test does not -- a sample carrying a shebang would be odd, so a CLI test is probably the right home. Commit, push, report.
