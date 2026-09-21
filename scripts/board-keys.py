#!/usr/bin/env python3
"""Measure where each key is on the 2741 keyboard picture.

The on-screen board is the redistributed picture with a layer of hit
regions laid over it. Those regions have to sit exactly on the keys,
so their positions are read out of the picture here rather than typed
in: if the picture were ever replaced, the board would follow it.

The picture places its keys with group transforms -- every key's
rectangle sits at the same local coordinates, and a chain of
enclosing groups moves it into place -- so the transforms are
resolved and the result is each key's box in the picture's own units.

The picture is read and never written. It is CC BY-SA and is served
exactly as redistributed; see images/redistributed/apl-keyboard/.

Usage: scripts/board-keys.py > pages/board-keys.json
"""

import json
import re
import sys
import xml.etree.ElementTree as ET

PICTURE = "images/redistributed/apl-keyboard/APL-keybd2.svg"
SVG = "{http://www.w3.org/2000/svg}"

# What each key is, by where it is: the ASCII a US keyboard sends from
# the same place, which is what keymap.json is keyed by. The top row
# begins with a key the picture leaves blank, which the board makes
# ATTN, and ends with backspace.
CAPS = [
    ["ATTN", "1", "2", "3", "4", "5", "6", "7", "8", "9", "0", "-", "=", "BACKSPACE"],
    ["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P", "["],
    ["A", "S", "D", "F", "G", "H", "J", "K", "L", ";", "'"],
    ["Z", "X", "C", "V", "B", "N", "M", ",", ".", "/"],
]


def times(a, b):
    """Compose two SVG affine matrices, a then b."""
    return [
        a[0] * b[0] + a[2] * b[1],
        a[1] * b[0] + a[3] * b[1],
        a[0] * b[2] + a[2] * b[3],
        a[1] * b[2] + a[3] * b[3],
        a[0] * b[4] + a[2] * b[5] + a[4],
        a[1] * b[4] + a[3] * b[5] + a[5],
    ]


def matrix(transform):
    """The matrix an SVG transform attribute stands for."""
    m = [1, 0, 0, 1, 0, 0]
    for name, args in re.findall(r"(\w+)\(([^)]*)\)", transform or ""):
        v = [float(x) for x in re.split(r"[ ,]+", args.strip()) if x]
        if name == "translate":
            n = [1, 0, 0, 1, v[0], v[1] if len(v) > 1 else 0]
        elif name == "scale":
            n = [v[0], 0, 0, v[1] if len(v) > 1 else v[0], 0, 0]
        elif name == "matrix":
            n = v
        else:
            sys.exit(f"board-keys: a {name} transform in the picture, which this cannot place")
        m = times(m, n)
    return m


def boxes(el, m, found):
    """Every key's outer box, in the picture's units."""
    m = times(m, matrix(el.get("transform")))
    # A key is an outer rectangle and a smaller face; a tap should land
    # anywhere on the outer one.
    if el.tag == SVG + "rect" and float(el.get("width", 0)) > 25:
        x, y, w, h = (float(el.get(k)) for k in ("x", "y", "width", "height"))
        left, top = m[0] * x + m[2] * y + m[4], m[1] * x + m[3] * y + m[5]
        right, bottom = m[0] * (x + w) + m[2] * (y + h) + m[4], m[1] * (x + w) + m[3] * (y + h) + m[5]
        found.append([round(left, 2), round(top, 2), round(right - left, 2), round(bottom - top, 2)])
    for child in el:
        boxes(child, m, found)
    return found


def rows(found):
    """Keys into rows by height, left to right. A key is in a row when
    its top is within a few units of the row's; the top row's last key
    sits a fraction above the rest, so this cannot be an equality."""
    out = []
    for box in sorted(found, key=lambda b: b[1]):
        if out and abs(out[-1][0][1] - box[1]) < 5:
            out[-1].append(box)
        else:
            out.append([box])
    return [sorted(row) for row in out]


def main():
    root = ET.parse(PICTURE).getroot()
    measured = rows(boxes(root, [1, 0, 0, 1, 0, 0], []))
    shape = [len(r) for r in measured]
    if shape != [len(r) for r in CAPS]:
        sys.exit(f"board-keys: the picture's rows hold {shape} keys, "
                 f"expected {[len(r) for r in CAPS]}")
    keys = [
        {"cap": cap, "box": box}
        for row, caps in zip(measured, CAPS)
        for box, cap in zip(row, caps)
    ]
    size = [float(root.get("width")), float(root.get("height"))]
    json.dump({"size": size, "keys": keys}, sys.stdout)


if __name__ == "__main__":
    main()
