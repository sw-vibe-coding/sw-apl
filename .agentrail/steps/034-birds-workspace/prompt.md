Owner direction 2026-09-21: a `)LIB 1` workspace, BIRDS, that shows
the combinators APL\360 can implement.

References the owner gave:
- The combinator demos live at https://mlpl.softwarewrighter.com/ ,
  from `../../sw-ml-study/sw-mlpl`.
- The owner's post: https://blog.softwarewrighter.com/2026/09/21/rabbit-hole-sage-y-combinator/
  whose source is in `../../software-wrighter-lab/blog`.

"Replace Y with Z": the Sage bird Y is the fixed-point combinator for
a lazy language, and loops forever under applicative order. APL is
applicative, so the workspace shows Z, the strict fixed point, in its
place.

Read the post and sw-mlpl first, and take the list of birds and how
they are presented from there rather than from memory.

The constraint to face, not to discover halfway: APL\360 cannot pass a
defined function to another function. There is no execute -- sw-apl
leaves it out on purpose -- and no user-defined operators. A combinator
is a function of functions, so what APL\360 can do is narrow, and the
workspace has to be honest about how each bird is realised: which are
ordinary APL\360 functions of values (I, K, and the like), which are
built from primitive operators such as reduction and outer product
over a fixed set of primitives, and which cannot be written at all
without a feature APL\360 did not have. A bird that needs one is
listed as such, not faked. Z in particular -- ask the owner how it
should appear before building it, if the post does not settle it,
since a fixed-point combinator is exactly the thing that needs a
function as an argument.

Like every workspace in library 1: a DESCRIBE function, the
`⍝!SOURCE sw-apl` mark `check-provenance` looks for, a sample under
`samples/` that loads it and runs every bird, a reg-rs transcript, and
tests in `library_tests.rs` that it loads and that DESCRIBE names only
what the workspace holds. It is ours: no text copied from the post or
from sw-mlpl beyond names.
