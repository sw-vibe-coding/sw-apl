# vhs tape: the sw-apl MVP REPL. Render with `just tape`.
# Slow typing and pauses so the glyphs can be read.
# Note: vhs turns a typed left arrow into the cursor key and the
# clipboard is unavailable headless, so this tape avoids assignment.
Output images/sw-apl-mvp.gif

Set Shell "bash"
Set FontSize 26
Set FontFamily "Menlo"
Set Width 900
Set Height 620
Set Padding 24
Set Theme "Catppuccin Mocha"
Set TypingSpeed 120ms

Hide
Type "clear; target/release/sw-apl" Enter
Sleep 500ms
Show

Sleep 1s
Type "2+3×4" Sleep 1s Enter Sleep 1.5s
Type "⍳10" Sleep 1s Enter Sleep 1.5s
Type "+/⍳10" Sleep 1s Enter Sleep 1.5s
Type "2 3⍴⍳6" Sleep 1s Enter Sleep 1.5s
Type "10×2 3⍴⍳6" Sleep 1s Enter Sleep 1.5s
Type "⍴2 3⍴⍳6" Sleep 1s Enter Sleep 1.5s
Type "1 2 3+4 5" Sleep 1s Enter Sleep 2.5s
Type ")OFF" Sleep 1.5s Enter
Hide
Sleep 500ms
