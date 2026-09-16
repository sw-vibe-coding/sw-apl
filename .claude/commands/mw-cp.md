---
description: "Checkpoint: scoped quality gates, docs, commit, push, CHANGES.md refresh"
---

# /mw-cp -- checkpoint process

Run the full pre-commit gate for the work in the current session,
then commit and push it. Order: gates -> docs -> commit -> push ->
CHANGES.md refresh -> report. If this session is an agentrail
step, `agentrail complete` comes AFTER the commit lands (the step
records HEAD), and nothing may change after `complete`.

## 1. Discover scope

```bash
git status --short
git diff --stat
```

List the `components/<name>/` workspaces that changed, plus any
workspace whose tests compile the changed code through a path
dependency. Everything in steps 2 and 3 is scoped to that list.

## 2. Tests (scoped)

For each changed workspace: `cargo test --workspace` (or
`cargo test -p <crate>` when only one crate changed). All
selected tests pass. No exceptions, no skipped failures.

## 3. Lint and format (scoped)

```bash
cargo clippy --workspace --all-targets -- -D warnings
cargo fmt --all
cargo fmt --all -- --check
```

Zero warnings. Fix, never `#[allow]` or suppress.

## 4. Repo-wide checks

- `sw-markdown-checker -f "**/*.md"` when any `.md` changed. ASCII
  only; glyph material goes in `.txt`/`.apl`/config files. Known
  non-blocking failure: the agentrail-managed block in CLAUDE.md
  and AGENTS.md contains em dashes emitted by `agentrail
  instructions apply` (fix belongs upstream); everything you
  hand-edited must pass.
- `sw-checklist` always. Zero failures; warnings at zero except
  "Binary Freshness" (sw-install is owner-only). If your change
  introduced a warning, split to the gate before committing.
- `scripts/reg.sh run -q` when interpreter or session behaviour
  changed. Rebase a baseline only intentionally; name the test
  and the reason in the commit message.

## 5. Docs

- Update `docs/language.md` / `docs/session.md` when behaviour
  changed (what and how, no chronology).
- Update `docs/plan.md` when scope or ordering changed; the saga
  plan must match (`agentrail plan`).
- Add or update a `samples/*.apl` when a feature is transcript-
  visible.

## 6. Commit

Stage by explicit path (never `git add -A`). Include `.agentrail/`
metadata with the source commit. Message: what + why, saga and
step, test scope and rationale, reg-rs notes, and the trailer:

```
Co-Authored-By: Claude Fable 5.1 <noreply@anthropic.com>
```

Never `--no-verify`. If a hook fails, fix the cause.

## 7. Push

```bash
git push
```

If pushing is impossible, say so explicitly in the handoff; never
leave commits silently stranded. Non-fast-forward on a personal
branch: `git pull --rebase`, never force-push shared branches.

## 8. CHANGES.md refresh

```bash
./scripts/gen-changes.sh
```

Commit as `docs(changes): refresh CHANGES.md to HEAD` (or fold
into the following agentrail-complete commit), then push.

## 9. Report

Tell the owner: what was pushed (commits, files, tests), the next
step(s) from the saga, and any blockers.

## Never

- NEVER run `sw-install` as part of this flow.
- Never run every workspace's tests for a scoped change.
- Never suppress warnings to get to green.
