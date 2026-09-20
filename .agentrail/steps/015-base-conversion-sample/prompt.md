Owner direction 2026-09-19: a sample that converts number bases.

A transcript that converts between binary, octal, decimal and
hexadecimal, exercising encode (the up tack) and decode (the down
tack) as APL\360 has them. This is the pair of functions the manual
spends the most space on and sw-apl has no sample showing them
worked, only unit tests.

What the sample should show:

- Decode: a radix vector and a digit vector giving a scalar. The
  same digits read in base 2, 8, 10 and 16.
- Encode: the inverse, a radix vector and a scalar giving digits.
  Round-trip a number out and back.
- Hexadecimal needs digits above 9, so show how a character vector
  indexes into a digit string to print them -- that is how APL\360
  did it, having no format function.
- A mixed radix that is not a number base at all, which is what
  decode is really for: hours, minutes and seconds, or pounds,
  shillings and pence.
- The traps worth showing on paper: encode with too short a radix
  vector truncates silently, and index origin changes the indexing
  in the hex step.

Number the file into the existing samples/ sequence and add the
matching line to samples/README.md. Seed the reg-rs baseline with
scripts/reg-seed.sh, run scripts/reg.sh run, and commit the .out
and .rgt with the source.

Check docs/parity.md: if the encode and decode rows are not already
done, this sample is evidence for them, and the rows should name it.

Gates: just fmt, just test, just clippy, just gates.