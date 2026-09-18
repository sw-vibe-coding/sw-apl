#!/usr/bin/env bash
# Preprocess filter for the reg-rs baselines of sw-apl samples. Reads
# a transcript on stdin, writes the normalized transcript on stdout.
# Each test names it through reg-rs's -P/--preprocess flag, so a
# baseline can hold a transcript that is not the same twice.
#
# There is one rule, because APL prints bare numbers and a filter
# cannot tell a clock reading from arithmetic. A sample that prints a
# value which cannot reproduce labels it, in its own mixed output:
#
#     'TIME OF DAY (VARIES): ';⌶20
#
# and everything after `(VARIES): ` is replaced by `...`. The label is
# ordinary APL, so it reads as part of the session and is visible in
# the transcript; the sample still shows what the feature returns,
# which a sample contorted into determinism does not.
#
# The rule matches output only, never the echoed input line that
# produced it: output starts in column one, while an input line is
# indented six spaces or headed by its `[n]` definition prompt. That
# is why the label must be upper case and must begin the line -- an
# echoed line can then never match it, so the statement stays legible
# beside its masked answer.
#
# Mask only what genuinely cannot reproduce: a clock, a process time,
# a host name. Never a value that is merely inconvenient -- that turns
# a regression test into a test of nothing. Where a whole test is
# unreliable rather than one value, say so with reg-rs's --flaky-note
# instead of widening this filter.
#
# Usage:
#   reg-rs create -t apl-sample-NAME -c '...' \
#                 -P 'bash scripts/normalize-apl-output.sh'

# The sign-off `)OFF` prints is the one varying output a sample cannot
# label, because the session writes it rather than the program. Its
# shape is kept and its values are masked, so a change to the format
# still fails while the clock moving does not.
#
#     20.01.12 09/17/26   ->   H.MM.SS MM/DD/YY
#     CONNECTED 0.00.07   ->   CONNECTED H.MM.SS
#     CPU TIME 0.00.01    ->   CPU TIME H.MM.SS
#
# )SAVE and )LOAD report a moment the same way, one with the name it
# saved under and one behind the word SAVED. Both keep their shape.
#
#     20.01.12 09/17/26 CLASS  ->  H.MM.SS MM/DD/YY CLASS
#     SAVED 20.01.12 09/17/26  ->  SAVED H.MM.SS MM/DD/YY

# The version block names the machine it was built on and the commit
# and moment it was built from. Those cannot be a baseline, and the
# (VARIES) convention cannot reach them: the binary prints them about
# itself, so no program can label them. The field names are kept and
# their values masked, so a field going missing still fails.
#
#     Host: max            ->   Host: ...
#     Commit: 5364d78      ->   Commit: ...
#     Timestamp: 2026-...  ->   Timestamp: ...
#
# The error text for a file that is not there comes from the operating
# system and is worded differently on each, so it is masked too.

set -euo pipefail

sed -E \
    -e 's/^([A-Z][A-Z0-9 ,.-]*\(VARIES\): ).*/\1.../' \
    -e 's/^[0-9]+\.[0-9]{2}\.[0-9]{2} [0-9]{2}\/[0-9]{2}\/[0-9]{2}( |$)/H.MM.SS MM\/DD\/YY\1/' \
    -e 's/^SAVED [0-9]+\.[0-9]{2}\.[0-9]{2} [0-9]{2}\/[0-9]{2}\/[0-9]{2}$/SAVED H.MM.SS MM\/DD\/YY/' \
    -e 's/^(CONNECTED|CPU TIME) [0-9]+\.[0-9]{2}\.[0-9]{2}$/\1 H.MM.SS/' \
    -e 's/^(  (Host|Commit|Timestamp): ).*/\1.../' \
    -e 's/^(sw-apl: ).*\(os error [0-9]+\)$/\1.../'
