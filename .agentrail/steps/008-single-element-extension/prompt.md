Found 2026-09-21 surveying the one-element-arguments bug, as that step
asked: two more places refuse a one-element vector, and whether they
should is a question of APL\360's rule, which must be sourced.

    1 2 3+,5        LENGTH ERROR   every scalar dyadic function
    (,5)+1 2 3      LENGTH ERROR
    1 0 1/,7        LENGTH ERROR   compress, a one-element right argument
    (1 1⍴5)+1 2 3   RANK ERROR     a one-element array of higher rank

sw-apl extends a scalar and nothing else. Many APLs extend any
one-element argument, and APL\360 may have extended a one-element
vector at least. That would change every scalar function, so it is
not changed from memory. The sources step (just before this one) finds
the APL\360 User's Manual; find its statement of the rule for scalar
functions and for compression and expansion, cite it, and implement
exactly that -- scalar only, one-element vectors, or one-element
arrays of any rank.

If the rule changes behaviour, say which existing transcripts move,
and rebase them with the manual's sentence as the reason. docs/
parity.md's "Scalar extension" row states the rule and its source.
