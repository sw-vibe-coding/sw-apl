Phase 6 step 2 (docs/plan.md). Only the first four characters of a
command name are significant.

The manual, in the summary preceding Table 2.1:

  "Where the first word of a command form is more than four
   characters long, only the first four are significant. The others
   are included only for mnemonic reasons, and may be dropped or
   replaced, as desired. For example, )CLEAR, )CLEA, )CLEAVER, etc.,
   are all equivalent."

sw-apl requires the exact spelling, so all of these answer INCORRECT
COMMAND today and should not:

      )CLEA
      )CLEAVER
      )ORIG 0
      )DIGI 5
      )SYMB

Implement the rule. Points to decide and to say out loud:

- A name of four characters or fewer is exact: )FNS is not )FNSX,
  and )VAR is not )VARS. The rule is about truncating a longer name
  to four, not about prefixes in general.
- Which commands collide at four characters? Work it out rather
  than assuming none do, and if two do, say what happens.
- )CLEAVER is equivalent to )CLEAR, so the characters after the
  fourth are ignored entirely rather than having to match. That is
  what "may be dropped or replaced" means, and it is worth a test
  of its own because it looks like a typo tolerance and is not.
- What this does to the trouble reports: )CLEAVER is a correct
  command, and )CLEAVE R is )CLEAR given an argument it does not
  take.

The dispatch is in apl-commands' system_command, which upper-cases
the name already; apl-inquiry answers for the names it knows. Keep
the rule in one place.

Add a parity.md row: it has never had one. Sample and reg-rs as
usual, and update docs/commands-reference.md from step 1 with the
rule.
