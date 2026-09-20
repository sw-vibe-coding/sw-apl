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

ink="#2b2118"
paper="#fbf7ef"
logo="images/sw-apl-logo.png"

# Del, U+2207. Font size against canvas so the glyph is not clipped.
favicon --unicode U+2207 --output-path pages/favicon.ico \
    -s 256 --font-size 170 -f "$ink" -b "$paper" >/dev/null
favicon --unicode U+2207 --output-path pages/apple-touch-icon.png --png \
    -s 180 --font-size 120 -f "$ink" -b "$paper" >/dev/null

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
