# core-mixed

Phase 2 of docs/plan.md: the mixed (structural) functions and the
operators, giving meaning to every form the parser already accepts.
Pure APL\360: flat arrays, index origin from )ORIGIN, the random
link for deal, no APL2 extensions (compress takes booleans only, no
replicate; no nested results). Every step: TDD, sw-checklist gates
as design constraints (4 modules/crate incl. lib.rs, 4 fns/module,
25 LOC/fn; split crates freely), update docs/parity.md rows in the
same commit, commit, push, report. Seed reg-rs baselines for every
sample that becomes fully runnable.

## Steps

1. ravel-catenate-laminate -- catenate along the last axis for
   matrices and higher rank, first-axis and axis forms A,[k]B,
   laminate with a fractional axis, scalar extension of a scalar
   argument. Samples 08, 44 as applicable.
2. take-drop-reverse-rotate-transpose -- ↑ ↓ per axis with
   negative counts and overtake fill; ⌽ ⊖ reverse and rotate
   (vector left argument rotates rows); ⍉ monadic and dyadic.
   Samples 07, 10, 13, 34, 35, 44.
3. compress-expand-membership-indexof -- boolean compress and
   expand on either axis (/ ⌿ \ ⍀ with an array left), ∊, dyadic
   ⍳ (index of, one past the end when absent). Samples 17, 30,
   41 (compress part).
4. grade-encode-decode-deal -- ⍋ ⍒ stable, ⊥ ⊤ mixed radix, deal
   M?N via the random link. Samples 32, 36, 41.
5. reduce-scan -- reduce on the first axis and with an axis
   bracket, scan f\ f⍀ on either axis, identity elements for
   every scalar dyadic function. Samples 06 (rest), 33, 24-or-and.
6. inner-outer-product -- f.g for matrices and vectors (+.× as
   the model), ∘.f with scalar extension. Samples 38, 39.
7. indexing-and-indexed-assignment -- A[I], M[I;J], elided axes,
   INDEX ERROR, index origin, results shaped by the index arrays;
   A[I]←V with scalar extension. Samples 20-bracket-index (add).
8. axis-forms-and-rank-checks -- every structural function with
   its axis bracket where APL\360 allows it, RANK and LENGTH
   errors audited against the manual, sample 09 and the horse-race
   pieces that do not need functions yet.
