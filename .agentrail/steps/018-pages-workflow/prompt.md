Owner direction 2026-09-20, ninth: publish with a lightweight
GitHub Actions workflow, not a branch deploy.

The branch-deploy instruction given at the end of the last step was
wrong. GitHub's "Deploy from a branch" offers only the repository
root or `/docs` -- never an arbitrary folder -- so `main` + `/pages`
was never a setting that existed. Set to `main` + `/`, which is what
it became, Pages serves the repository root: the README through
Jekyll, not the demo.

The two sibling repositories do it differently and the owner has
chosen between them:

- `sw-ml-study/sw-mlpl` is `build_type: legacy` on a `gh-pages`
  branch at its root, mirrored there by `scripts/deploy-pages.sh`
  through a second worktree.
- `sw-fun/suduko` is `build_type: workflow` with a `static.yml`
  that builds nothing: checkout, configure-pages,
  upload-pages-artifact with `path: './pages'`, deploy-pages.

Take the second. It needs no second branch and no mirroring
script, and the artifact is exactly the `pages/` that was committed
-- which is the property `just publish` and `build-info.json` were
built around.

Add the workflow, push it, and switch the repository's Pages source
to GitHub Actions. Then load the published URL and check it is the
demo and not the README: the owner has been told twice now that a
link would work, and it should not be a third time without looking.

Keep `.nojekyll`. The artifact upload does not run Jekyll, so it is
belt and braces rather than load-bearing, and it costs nothing.
