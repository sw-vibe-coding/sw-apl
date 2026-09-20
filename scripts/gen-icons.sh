#!/usr/bin/env bash
# The demo's icons: a favicon, and what an installed PWA shows.
#
# Two sources, because they answer two questions. The favicon is a
# single glyph -- del, which opens a function definition and is the
# most APL thing that stays legible at sixteen pixels. The install
# icons are the project logo the README shows, scaled down, so an
# installed sw-apl looks like sw-apl.
#
# Run when either source changes; the results are tracked, because
# pages/ is published as committed. Needs the `favicon` CLI and
# ImageMagick.
set -euo pipefail
cd "$(dirname "$0")/.."

paper="#fbf7ef"
logo="images/sw-apl-logo.png"
tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

# The mark is `⍝⍴⌊` turned 33 degrees: three APL glyphs that, turned,
# read as the letters apl. The owner found it; it says what this is
# in a way a single glyph cannot, and it survives a 16-pixel tab.
#
# Both glyphs took a try to settle. Turned, cap has no counter and
# reads as an n; the lamp has one and makes an a. Turned, the
# horseshoe reads as a D; rho has a bowl and a descender and makes a
# p. The mark said nPL, then aDL, and now apl.
#
# Bright green on nothing, so it reads on a light tab bar and a dark
# one alike -- a favicon with a background is a square of somebody
# else's colour in a strip of tabs.
#
# The `favicon` CLI leaves the string low and left of centre with the
# rotation applied, so the mark is trimmed to its ink and re-centred
# here: an icon is mostly seen small, and the dead corner was most of
# the tile.
mark="⍝⍴⌊"
green="#22c55e"

favicon -T -f "$green" -t "$mark" -R 33 --font-size 52 -s 256 \
    --png -o "$tmp/mark.png" >/dev/null
centre() {
    magick "$tmp/mark.png" -trim +repage -background none -gravity center \
        -extent '%[fx:max(w,h)*1.18]x%[fx:max(w,h)*1.18]' \
        -resize "$1x$1" -background "$2" -gravity center -extent "$1x$1" "$3"
}
# An ICO at the sizes a tab bar actually asks for. As one 256-pixel
# frame this file was 270 KB -- most of what a tab bar ever downloads
# and none of what it displays. 16 and 32 bring it to about 5 KB.
#
# Not 48: an ICO frame is stored as raw BGRA, so 48 alone is nine
# kilobytes and triples the file, and nothing in a browser asks for
# it -- Windows tiles are served by the manifest's PNGs instead.
# Reducing colours does nothing for the same reason, raw BGRA having
# no palette to reduce.
#
# Not an SVG either, tempting as it is at a tenth the size: the mark
# is glyphs, and an SVG carrying text depends on the reader having an
# APL font. Outlining them needs a tracer this repository does not
# require. A bitmap that always draws beats a vector that sometimes
# does not.
centre 256 none "$tmp/icon.png"
magick "$tmp/icon.png" -define icon:auto-resize=16,32 pages/favicon.ico
# iOS puts black behind a transparent touch icon, so this one has
# paper under it rather than nothing.
centre 180 "$paper" pages/apple-touch-icon.png

# The install icons, from the logo. It is a round badge on nothing --
# the source is a JPEG with white corners, masked to its circle in
# `docs/` terms rather than flood-filled, because a JPEG's edge
# pixels are part white and flood-filling leaves a fringe that is
# invisible on white and obvious on anything else. Transparent,
# because a launcher puts its own background behind it.
magick "$logo" -resize 192x192 -background none -gravity center -extent 192x192 \
    pages/icon-192.png
magick "$logo" -resize 512x512 -background none -gravity center -extent 512x512 \
    pages/icon-512.png
# Maskable: a launcher may crop this to a circle, so the logo sits
# inside the safe zone -- 80% of the width -- and the rest is paper.
magick "$logo" -resize 320x320 -background "$paper" -gravity center -extent 512x512 \
    pages/icon-512-maskable.png

echo "gen-icons: favicon.ico, apple-touch-icon.png, icon-192, icon-512, icon-512-maskable"
