⍝ How much of a command name has to be typed. The manual: "Where the
⍝ first word of a command form is more than four characters long,
⍝ only the first four are significant. The others are included only
⍝ for mnemonic reasons, and may be dropped or replaced, as desired.
⍝ For example, )CLEAR, )CLEA, )CLEAVER, etc., are all equivalent."
⍝ Four characters is enough for a long name.
A←1
)VARS
)CLEA
)VARS
⍝ What follows the fourth character is ignored rather than forgiven,
⍝ so this is not a typo being let through: it is the rule.
B←2
)CLEAVER
)VARS
⍝ The settings take it too, and reply as they always do.
)ORIG 0
⍳3
)DIGI 5
1÷3
)SYMB
⍝ A name of four characters or fewer has nothing to cut, so it must
⍝ be exact. )VARS is the command and )VAR is not.
)VAR
⍝ Nor is a short name a prefix to be extended.
)SIX
)FNSX
⍝ And a name that is no command at all is still no command.
)NOSUCH
)OFF
