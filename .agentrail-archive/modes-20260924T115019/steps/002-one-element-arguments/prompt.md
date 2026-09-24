Found 2026-09-21 checking the iota bug for its siblings, as that step
asked: two more primitives refuse a one-element vector where APL\360
takes one as a scalar.

    3?⍴A          RANK ERROR     deal, the right argument
    (,3)?10       RANK ERROR     deal, the left
    (,2)⌽⍳5       RANK ERROR     rotate, a one-element left argument

The first is the one that matters: A[(⍴A)?⍴A] is how APL\360 shuffles
a vector, and ⍴A is always a one-element vector. random.rs demands
both of deal's arguments be scalars; rotate.rs demands a left argument
shaped like the right's cells unless it is a scalar.

Accept a one-element vector wherever a scalar is accepted, as the
iota step did: RANK ERROR stays for higher rank or more than one
element. Check the rest of the primitives for the same assumption --
the iota step's grep is in its commit -- and say what was checked.
Tests first; a line in a sample for the shuffle idiom.
