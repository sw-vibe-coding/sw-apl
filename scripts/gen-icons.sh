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

# The mark is `∩⊃⌊` turned 33 degrees: three APL glyphs that, turned,
# read as the letters APL. The owner found it; it says what this is
# in a way a single glyph cannot, and it survives a 16-pixel tab.
#
# Bright green on nothing, so it reads on a light tab bar and a dark
# one alike -- a favicon with a background is a square of somebody
# else's colour in a strip of tabs.
#
# The `favicon` CLI leaves the string low and left of centre with the
# rotation applied, so the mark is trimmed to its ink and re-centred
# here: an icon is mostly seen small, and the dead corner was most of
# the tile.
mark="∩⊃⌊"
green="#22c55e"

favicon -T -f "$green" -t "$mark" -R 33 --font-size 52 -s 256 \
    --png -o "$tmp/mark.png" >/dev/null
centre() {
    magick "$tmp/mark.png" -trim +repage -background none -gravity center \
        -extent '%[fx:max(w,h)*1.18]x%[fx:max(w,h)*1.18]' \
        -resize "$1x$1" -background "$2" -gravity center -extent "$1x$1" "$3"
}
centre 256 none pages/favicon.ico
# iOS puts black behind a transparent touch icon, so this one has
# paper under it rather than nothing.
centre 180 "$paper" pages/apple-touch-icon.png

# The install icons, from the logo. Transparent, because the logo is
# a round badge and a launcher puts its own background behind it.
magick "$logo" -resize 192x192 -background none -gravity center -extent 192x192 \
    pages/icon-192.png
magick "$logo" -resize 512x512 -background none -gravity center -extent 512x512 \
    pages/icon-512.png
# Maskable: a launcher may crop this to a circle, so the logo sits
# inside the safe zone -- 80% of the width -- and the rest is paper.
magick "$logo" -resize 320x320 -background "$paper" -gravity center -extent 512x512 \
    pages/icon-512-maskable.png

echo "gen-icons: favicon.ico, apple-touch-icon.png, icon-192, icon-512, icon-512-maskable"
