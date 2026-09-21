Two bugs found 2026-09-21 comparing sw-apl's roll and deal with
Roger Hui's "Roll" (https://www.jsoftware.com/papers/roll.htm). The
paper confirmed the generator -- Lehmer, multiplier 7^5, modulus
2^31-1, first link 7^5, all as sw-apl has them. Checking against it
found these, which the paper is not about.

**1. `?N` is silently wrong for large N.** `roll` computes
`N×(link-1)` in u64, which overflows once N passes about 8.6×10^9.
A release build -- the CLI and the browser -- wraps; a debug build
would panic. After `)CLEAR`, replaying the link:

    ?1E15   gives 7,753,573,564   should be 458,650,131,671,364

Compute the product in 128 bits, which is exact and changes nothing
below the overflow point, so no transcript moves.

Separately, a limit: there are only 2^31-2 distinct links, so `?N`
for N past that cannot reach every value however it is computed.
Decide whether that is DOMAIN ERROR, and say in the commit that the
historical limit is not established, rather than presenting a guess
as APL\360's.

**2. Deal's memory is proportional to N, not to what is dealt, and
escapes the workspace quota.** `l?r` builds a list of all `r`
indexes to pick `l` of them:

    1?1000000    15 MB
    1?50000000   407 MB   under a 100 MB workspace: should be WS FULL

`1?1E9` wants about 8 GB, past a browser's 4 GB WebAssembly ceiling,
which would end the session. Use a sparse Fisher-Yates: the same
swaps in the same order, so identical results and no transcript
change, holding only the positions that were swapped -- O(l), not
O(r). The result is `l` long and is what the quota should see.

**Not in scope:** the paper maps a link onto 0..N-1 as `⌊N×link÷P`
and sw-apl as `⌊N×(link-1)÷(P-1)`. They agree almost always for small
N and differ by one at large N. The paper's formula is J's, and
nothing shows it was APL\360's; changing it would move every
transcript that rolls. Record it in docs/language.md as a known
difference, and do not change the formula.

Tests: the replay above, as a test that `?1E15` gives the exact
value; that the sparse deal gives exactly the dense deal's results
across a range of arguments and several links, which is the property
that keeps transcripts still; and that `1?1E9` completes in bounded
memory. reg-rs must stay 80/80 without a rebase.
