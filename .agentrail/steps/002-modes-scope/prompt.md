Phase 9 step 2 (docs/plan.md, owner direction 2026-09-21). Say what
sw-apl now is, where the agents and the requirements will read it.

sw-apl has modes: '68 is APL\360 exactly as built, '72 is APLSV. The
places that told an agent or a reviewer otherwise must stop doing so:

- CLAUDE.md, outside the agentrail-managed block: the Project
  Overview says "pure APL\360 ... Not APLSV"; the Key Rules say
  "not APL2" and should keep saying it. Say '68 and '72, that the
  existing implementation is the shared core, that '68 must not move,
  and that APL2 is not planned here.
- docs/prd.md: the requirements gain the '72 mode, with shared
  variables and APL2 listed as out of scope.

The README and the user-facing docs do not change yet: they say what
a reader can use, and '72 is not usable. They change in the last
step of the phase. No code in this step.
