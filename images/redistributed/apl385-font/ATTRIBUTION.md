# APL385 Unicode

`APL385.woff2` is the APL385 Unicode font: a monospaced code font
with every APL glyph, which the browser demo uses so that the glyphs
look the same on every machine, whether or not the font is installed.

It is not ours. It is redistributed here under its own terms, kept in
`images/redistributed/` so that what sw-apl wrote and what sw-apl
borrowed are never in the same directory.

## Credit

- **Title:** APL385 Unicode, version of 21 August 2016
- **Author:** Adrian Smith
- **Source:** <https://www.apl385.com/fonts/>, the file `apl385.zip`,
  which holds `Apl385.ttf` (212256 bytes, SHA-256
  `4ea113fc852569d4b7fdd6c19dc934c197f43d785cb58c96352565b34fad2343`).
  The same file is carried by the APL386 project,
  <https://github.com/abrudz/APL386>, byte for byte.
- **Terms:** public domain, by the author's dedication on the page
  above; see `LICENSE` beside this file.
- **Changes:** the TrueType file was converted to WOFF2, a compressed
  container for the web, with fontTools 4.66.0. No glyph, metric or
  name was changed.

## What it lacks

The font has no glyph for U+0332, the combining low line that
underscores a letter. A browser draws that one from the next font in
the page's list.
