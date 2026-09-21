Found reviewing the new board, 2026-09-20: the picture's own Return,
Shift, Tab and Caps Lock cannot be tapped.

The board step made the 2741 picture the board and laid a hit region
over each key -- but only over the 46 keys the picture draws as
rectangle pairs, which is what `scripts/board-keys.py` measures. The
wide keys are drawn as outlined paths instead: Tab and Caps Lock at
the left of rows two and three, the two Shifts at the ends of row
four, and the tall L-shaped Return at the right of rows two and
three. So a reader taps the picture's Return and nothing happens,
and has to find the Return button below the picture instead.

Make them work, where the 2741 meaning carries over:

- Return on the picture sends the line, as the button does.
- Shift on the picture switches between the APL and ABC faces --
  which is what those two modes are, so the picture's own key is the
  most natural control for them.
- Tab and Caps Lock have no meaning at an APL prompt. Leave them
  inert rather than inventing one.

Measure their outlines out of the picture as the other keys are, not
by typing in coordinates. The Return key is not a rectangle; its hit
region should follow its shape or cover its bounding box, and say
which.
