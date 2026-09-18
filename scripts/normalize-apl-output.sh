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

set -euo pipefail

sed -E -e 's/^([A-Z][A-Z0-9 ,.-]*\(VARIES\): ).*/\1.../'
