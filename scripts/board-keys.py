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

The picture draws its ordinary keys as rectangles and its wide ones --
Tab, Caps Lock, the Shifts and Return -- as outlined paths, so both are
measured: the rectangles by their boxes, the outlines by the points
their path data names.

With `--adapt OUT` it also writes the adaptation the board draws: the
picture with a class on every glyph saying which face of its key it is
on, and on the parts of the Caps Lock key. See ATTRIBUTION.md beside
the picture for what that changes and under what licence.

Usage: scripts/board-keys.py [--adapt OUT.svg] > pages/board-keys.json
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


# A path's coordinates, for a bounding box. Every command's points are
# taken, control points included: a curve lies inside the polygon of
# its control points, so the box can only be too big, never too small,
# and a hit region a hair too big is harmless.
TOKENS = re.compile(r"[MmLlHhVvCcSsQqTtAaZz]|-?(?:\d+\.?\d*|\.\d+)(?:[eE][-+]?\d+)?")
ARITY = {"M": 2, "L": 2, "H": 1, "V": 1, "C": 6, "S": 4, "Q": 4, "T": 2, "A": 7, "Z": 0}


def path_points(d):
    """Every point a path's data names, made absolute."""
    tokens, i, x, y, start, command, out = TOKENS.findall(d), 0, 0.0, 0.0, (0.0, 0.0), None, []
    while i < len(tokens):
        if tokens[i].isalpha():
            command, i = tokens[i], i + 1
        if command in "Zz":
            (x, y), command = start, None
            continue
        up, rel = command.upper(), command.islower()
        args = [float(v) for v in tokens[i:i + ARITY[up]]]
        i += ARITY[up]
        base = (x, y) if rel else (0.0, 0.0)
        if up == "H":
            x = args[0] + base[0]
        elif up == "V":
            y = args[0] + base[1]
        elif up == "A":
            x, y = args[5] + base[0], args[6] + base[1]
        else:
            pairs = [(args[k] + base[0], args[k + 1] + base[1]) for k in range(0, len(args), 2)]
            out.extend(pairs[:-1])
            x, y = pairs[-1]
        out.append((x, y))
        if up == "M":
            start, command = (x, y), ("l" if rel else "L")
    return out


def wide(el, m, found):
    """The keys the picture draws as outlines rather than rectangles:
    Tab, Caps Lock, the Shifts and Return. Each is an outer outline and
    a face inside it; only outlines bigger than an ordinary key are
    kept, and a label that happens to be wide is not an outline."""
    m = times(m, matrix(el.get("transform")))
    if el.tag == SVG + "path" and el.get("d") and not el.get("id", "").startswith("text"):
        pts = [(m[0] * x + m[2] * y + m[4], m[1] * x + m[3] * y + m[5]) for x, y in path_points(el.get("d"))]
        xs, ys = [p[0] for p in pts], [p[1] for p in pts]
        box = [round(min(xs), 2), round(min(ys), 2), round(max(xs) - min(xs), 2), round(max(ys) - min(ys), 2)]
        if box[2] > 30 or box[3] > 30:
            found.append(box)
    for child in el:
        wide(child, m, found)
    return found


def outers(boxes):
    """Of the wide outlines, the outer ones: drop any box that lies
    inside another, which is the face drawn within a key."""
    inside = lambda a, b: a is not b and a[0] >= b[0] and a[1] >= b[1] \
        and a[0] + a[2] <= b[0] + b[2] and a[1] + a[3] <= b[1] + b[3]
    return [a for a in boxes if not any(inside(a, b) for b in boxes)]


def named_wide(boxes, width):
    """What each wide key is, by where it is: the left-hand ones by row,
    the right-hand one that spans two rows is Return."""
    out = []
    for box in outers(boxes):
        left, tall, row = box[0] < width / 2, box[3] > 40, round(box[1] / 28)
        if tall:
            cap = "RETURN"
        elif left:
            cap = {1: "TAB", 2: "CAPS", 3: "SHIFT"}.get(row)
        else:
            cap = "SHIFT" if row == 3 else None
        if cap is None:
            sys.exit(f"board-keys: a wide outline at {box} that is no key this knows")
        out.append({"cap": cap, "box": box})
    return sorted(out, key=lambda k: (k["box"][1], k["box"][0]))


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


def glyph_boxes(el, m, found):
    """Every small outline in the picture -- the glyphs on the keys --
    with the element and the centre of its box."""
    m = times(m, matrix(el.get("transform")))
    if el.tag == SVG + "path" and el.get("d"):
        pts = [(m[0] * x + m[2] * y + m[4], m[1] * x + m[3] * y + m[5]) for x, y in path_points(el.get("d"))]
        xs, ys = [p[0] for p in pts], [p[1] for p in pts]
        centre = ((min(xs) + max(xs)) / 2, (min(ys) + max(ys)) / 2)
        found.append((el, centre, max(xs) - min(xs), max(ys) - min(ys)))
    for child in el:
        glyph_boxes(child, m, found)
    return found


def within(point, box):
    x, y, w, h = box
    return x <= point[0] <= x + w and y <= point[1] <= y + h


def mark(el, cls):
    el.set("class", (el.get("class", "") + " " + cls).strip())


def adapt(tree, keys, special, out):
    """Write the adaptation: the picture with a class on each glyph
    saying which face of its key it is on, and on the parts of the Caps
    Lock key. A page can then emphasise one face and dim the other.

    A 2741 key carries its shifted glyph above and its unshifted one
    below, and every ordinary key on the picture has exactly one of
    each, so the face is which half of the key a glyph's centre is in.
    Nothing is moved, recoloured or removed: only classes are added.
    """
    found = glyph_boxes(tree.getroot(), [1, 0, 0, 1, 0, 0], [])
    for key in keys:
        if key["cap"] in ("ATTN", "BACKSPACE"):
            continue
        x, y, w, h = key["box"]
        on = [(el, c) for el, c, gw, gh in found if gw < 20 and gh < 20 and within(c, key["box"])]
        if len(on) != 2:
            sys.exit(f"board-keys: key {key['cap']} carries {len(on)} glyphs, not two")
        for el, centre in on:
            mark(el, "face-shifted" if centre[1] < y + h / 2 else "face-normal")
    caps = next(k["box"] for k in special if k["cap"] == "CAPS")
    parts = sorted((el for el, c, gw, gh in found if within(c, caps)), key=lambda e: 0)
    for el, c, gw, gh in found:
        if within(c, caps):
            mark(el, "caps-lock caps-lock-" + ("edge" if gw > 40 else "face" if gh > 10 else "label"))
    tree.write(out, encoding="unicode", xml_declaration=True)


def keep_namespaces(path):
    """Register the picture's own namespace prefixes, so that writing it
    back keeps `inkscape:` and `sodipodi:` rather than inventing ns0."""
    for _, (prefix, uri) in ET.iterparse(path, events=["start-ns"]):
        ET.register_namespace(prefix, uri)


def main():
    keep_namespaces(PICTURE)
    parser = ET.XMLParser(target=ET.TreeBuilder(insert_comments=True))
    tree = ET.parse(PICTURE, parser)
    root = tree.getroot()
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
    special = named_wide(wide(root, [1, 0, 0, 1, 0, 0], []), size[0])
    found = sorted(k["cap"] for k in special)
    if found != ["CAPS", "RETURN", "SHIFT", "SHIFT", "TAB"]:
        sys.exit(f"board-keys: the picture's wide keys are {found}")
    json.dump({"size": size, "keys": keys, "wide": special}, sys.stdout)
    if len(sys.argv) > 2 and sys.argv[1] == "--adapt":
        adapt(tree, keys, special, sys.argv[2])


if __name__ == "__main__":
    main()
