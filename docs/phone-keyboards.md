# Phone keyboards

How a reader on an Android phone, in the browser or in the demo
installed as an app, can type APL into sw-apl -- and why the board on
the page is the main answer. This is an analysis; `glyph-entry.md`
says how each way in works today.

## A web page cannot supply a keyboard

An Android system keyboard is an input method: a native app, built
on the platform's input method service, that the reader installs and
then enables in the system settings. Once enabled, it types into
every app.

A web page cannot be one, and installing it as a progressive web app
does not change that. An installed web app is still a page in a
browser engine, and the browser gives a page no way to register an
input method, replace the system keyboard, or extend it. The same is
true on iOS. So there is no "APL keyboard for this PWA" in the sense
of a system keyboard the demo ships.

What remains is three kinds of answer: a keyboard drawn by the page
itself, a system keyboard the reader installs separately, and a
physical keyboard.

## How the demo takes keystrokes

This decides which of those can work, so it comes first.

Nothing on the page is focusable: no input, no textarea, nothing
contenteditable. Keystrokes are read off the window as `keydown`
events and passed to the 2741 keyboard compiled into the page
(`apl-keyboard`, through `apl-board`), which owns the keymap, the
overstrike and the line. Paste is read the same way and goes in as
Unicode.

Two consequences on a phone:

- **The system keyboard never rises over the demo.** A phone raises
  its soft keyboard only for a focused editable element, and there is
  none. So the board on the page is never covered, and there is
  nothing to suppress (`inputmode="none"` would have nothing to
  apply to).
- **No soft keyboard can type into the demo either**, for the same
  reason -- including an APL one. Paste still works: text typed in
  any other app with any keyboard can be copied and pasted in.

## The ways in

### 1. The board on the page

The board is a keyboard drawn by the page: the 2741's own picture
with a hit region over each key, plus what a 2741 never had -- the
row of overstruck glyphs, the underscore key, the Commands and
Idioms lists, and (B)'s CMD legends. It is the only way in that:

- works on every phone, in the browser and installed, with nothing
  to install or enable;
- knows the modes, offering only what the mode has;
- does 2741 overstriking, and says under the line when a strike is
  waiting or the underscore key is on, which matters on a phone
  where tooltips never show.

It is the primary way in on a phone, and the analysis below does not
change that.

### 2. A system keyboard with an APL layout

Some Android keyboards accept a layout file from the reader. One is
Unexpected Keyboard, which is open source and has a custom-layout
setting. A layout file generated from `keymap.json`, putting each
glyph where the 2741 puts it, would give a reader APL glyphs in every
app on the phone.

Whether a dedicated APL keyboard for Android already exists has not
been checked; it should be, before a layout is written.

For the demo itself this does not work as things stand, because the
demo has nothing to focus (above). Making it work means a second
input path:

- A focusable element the reader can choose to type into, which
  raises the system keyboard -- and, when it does, covers half the
  screen and competes with the board. It would have to be a choice,
  not the default.
- Reading what the keyboard types from `beforeinput`/`input` events
  rather than `keydown`. A soft keyboard on Android typically reports
  `keydown` with no usable key name (`Unidentified`, key code 229)
  and delivers the text through the input events, sometimes a word at
  a time through composition. Each piece of text would go to the
  board as a paste, which already strikes when a strike is waiting.
- 2741 overstriking would not come with it: a layout types finished
  glyphs, so the composed ones (`⍟`, `⍝`, `⌹`, ...) need keys of their
  own, and there is no Ctrl-] on a soft keyboard.

Until then, a layout file still helps a reader who types APL in a
notes app or a chat and pastes it in.

### 3. A native APL keyboard of sw-apl's own

A small Android app -- an input method reading the same
`keymap.json` -- would type in every app and could do overstriking
itself, holding a glyph and striking the next key over it, as
`apl-keyboard` does. It is a separate project: an Android toolchain,
signing, store listing or sideloading, and upkeep across Android
releases. It has the same limit as a layout for the demo: without a
focusable input path, it cannot type into the page.

### 4. A physical keyboard

A Bluetooth or USB keyboard works today. Its keys arrive as ordinary
`keydown` events, the page maps them to the 2741 layout as it does on
a desktop, and Ctrl-] is the overstrike key. On a tablet this is the
closest thing to a terminal.

## Comparison

| Way in | Installs | Works in the demo | Other apps | Overstrikes | Knows the mode |
|---|---|---|---|---|---|
| Board on the page | nothing | yes | no | yes | yes |
| Layout for an existing keyboard | a keyboard app, a layout | only with a second input path; paste meanwhile | yes | no | no |
| sw-apl's own Android keyboard | an app | only with a second input path | yes | could | no |
| Physical keyboard | nothing | yes | yes | yes, Ctrl-] | yes, in the demo |

## Recommendation

- Keep the board as the way in on a phone, and keep making it explain
  itself, since a phone has no hover.
- If readers want APL glyphs outside the demo, the cheapest useful
  step is a layout file for an existing keyboard, generated from
  `keymap.json` so it cannot drift, with a page saying how to
  install it. Check first whether an APL keyboard for Android
  already exists.
- Add a second, opt-in input path to the demo only if readers ask to
  type into it with a system keyboard; it is what makes options 2 and
  3 reach the demo at all.
- A native keyboard of sw-apl's own is not worth its upkeep unless
  phone use grows well beyond a demo.
