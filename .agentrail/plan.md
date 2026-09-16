# bootstrap

Phase 0 of docs/plan.md: stand up the repository, its process, its
planning documents, and a runnable `sw-apl` CLI skeleton so every
later saga starts from a green tree. Derived from docs/plan.md
"Phase 0: bootstrap"; keep the two in sync.

## Steps

0. repo-scaffold (the saga init step) -- COPYRIGHT, LICENSE,
   .gitignore, .gitattributes, .cargo/config.toml (shared target),
   justfile, scripts (gen-changes, run-samples, reg wrappers),
   /mw-cp checkpoint command, CLAUDE.md project notes, README with
   logo, samples corpus copied from sw-cor24-apl filtered to
   APL\360 scope.
1. planning-docs -- docs/plan.md (master plan), prd, architecture,
   design, language, session, glyphs.txt, testing, input-methods
   (Espanso + Emacs), saga log.
2. cli-skeleton -- components/cli workspace with the sw-apl binary:
   -h/--help/-V/--version per sw-checklist, -f FILE and stdin batch
   with six-space echo, interactive loop, NOT IMPLEMENTED stub
   session; reg-rs harness scripts; README build instructions
   verified. TDD: cli_tests.rs first.
