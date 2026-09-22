Owner direction 2026-09-20: the demo has a custom domain.

https://sw-apl.softwarewrighter.com/ is set and serving. Put it in
the README in place of the github.io URL.

Add `pages/CNAME` holding the domain. The domain is in the
repository's Pages settings and the site works without the file,
but this deploys by uploading an artifact, and a CNAME in the
artifact is what keeps a custom domain from being dropped if that
setting is ever lost or reset. It costs one line.

The sub-path worry goes away -- a custom domain serves from the
root -- but do not make anything depend on that. Every URL the page
fetches stays relative, and `just check-pages` keeps its sub-path
case: the github.io address still works and a reader may arrive on
it.
