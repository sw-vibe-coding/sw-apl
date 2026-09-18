#!target/release/sw-apl --no-echo -f
⍝ The bare form. It works here because macOS splits a shebang's
⍝ arguments; Linux passes them as one string, so use the env -S form
⍝ in anything that has to run on both. See docs/session.md.
2+2
)OFF
