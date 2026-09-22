Phase 9 step 7 (docs/plan.md). '75 gains format, in a new '75-only
crate: monadic, and dyadic with width and precision, per
docs/aplsv.md and the APLSV sources, not from a later APL's format.
The display crate already knows how APL\360 prints a number; format
should agree with it and reuse it rather than copy it. Samples for
'75; '68 unchanged.
