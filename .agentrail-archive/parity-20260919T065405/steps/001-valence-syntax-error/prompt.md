Phase 7 step 1 (docs/plan.md). A glyph used where it has no meaning
is a SYNTAX ERROR.

sw-apl answers NOT IMPLEMENTED in four places, and they are all the
same mistake: a glyph used in a valence, or a form, that APL\360
does not have. There is no such function, so the sentence does not
parse, and SYNTAX ERROR is what APL\360 says.

      1~0
NOT IMPLEMENTED
      1⍋2
NOT IMPLEMENTED

The four sites:

- apl-prims-scalar's apply_monadic and apply_dyadic, where a glyph
  falls past every family. This is `1~0`, `1⍋2`, `1⍒2`.
- apl-prims-ops: scan, outer and inner when the glyph has no dyadic
  scalar form, and reduce's identity table for the same.
- apl-prims' no_axis, where an axis bracket is allowed on the glyph
  but the particular form is APL2 rather than APL\360.

That last one deserves a thought rather than a sweep: the comment
there says monadic ravel with an axis is APL2, and an APL2 form is
not a form APL\360 has, so it is a SYNTAX ERROR by the same
argument. Satisfy yourself of that before changing it, and if you
conclude otherwise, say so and leave the row open rather than
forcing it.

When every site is gone, remove `ErrorKind::NotImplemented` itself.
The parity row says "temporary, must reach zero", and the variant
existing at all is what keeps it above zero. The compiler will find
anything missed.

Watch for the reverse mistake: a glyph that HAS the valence but was
given a bad argument must still be DOMAIN ERROR. `1⍟0` and `÷0` are
domain errors and must stay that way. Reducing an empty vector by a
function with no identity element is DOMAIN ERROR too, and that is
a different arm of the same match as the one being changed.

TDD; there are existing tests asserting NOT IMPLEMENTED -- find them
and change what they assert, since each one is a decision being
made. reg-rs for anything run through the binary; a sample showing
the forms that do not parse is worth having. Update docs/parity.md
and docs/language.md in the same commit.
