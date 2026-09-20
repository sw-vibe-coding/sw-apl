Bug found 2026-09-19 while answering an owner question about set
operations: monadic iota rejects a one-element vector.

components/prims/crates/apl-prims-mixed/src/iota.rs demands a
strict scalar:

    if !r.shape.is_empty() { return Err(AplError::new(ErrorKind::Rank)); }

APL\360 accepts a non-negative integer scalar OR a one-element
vector there. The shape of a vector is a one-element vector, so
this breaks the index generator idiom that APL\360 programs use
more than any other. Verified at HEAD:

    A<-3 1 4 1 5
    iota rho A          RANK ERROR, caret on the iota
    iota ,5             RANK ERROR

It takes the nub idiom with it -- ((A iota A) = iota rho A)/A --
which is how APL\360 does unique, and therefore how it does set
union and intersection, since the language has no primitive for
either. The workaround today is (rho A)[1], indexing the shape
back down to a scalar, which no APL\360 programmer would write.

Fix: accept an argument of at most one element, as the manual
states. RANK ERROR stays for anything of higher rank or more than
one element; DOMAIN ERROR stays for a negative or non-integer.
Check whether any other monadic function in apl-prims-mixed makes
the same strict-scalar assumption and state in the commit what was
checked.

Tests: RED first. Unit tests for iota of a one-element vector, of
a scalar, of a 1 1 matrix (RANK ERROR -- rank, not count), and of
a two-element vector (RANK ERROR). A session test for iota rho A
and for the nub idiom.

Sample: the set operations APL\360 has no primitives for are worth
a transcript of their own -- intersection (A in B)/A, union
A,(~B in A)/B, difference (~A in B)/A, and the nub -- on numbers
and on a character vector. Number it into samples/, add the line
to samples/README.md, seed with scripts/reg-seed.sh.

Docs: docs/parity.md iota row, and docs/language.md if it states
the argument iota takes.

Gates: just fmt, just test, just clippy, just gates.