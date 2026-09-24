#!target/release/sw-apl -f
⍝ The bare form, with one argument. A shebang line is the interpreter
⍝ and at most one argument on Linux, which passes everything after the
⍝ interpreter as one string; macOS splits it. One argument works on
⍝ both. For more than one -- --no-echo -f, say -- use the env -S form,
⍝ as hello.apl does. See docs/session.md.
2+2
)OFF
