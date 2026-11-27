// (U) '74's helpers for a deck punched from nothing: the control
// cards, and idioms -- a card or a few that do one thing in UMD APL's
// $ spellings. Ours, written for this page and checked by running
// them (check-pages punches every one into a new deck and runs it).
// Those after the vector use V.

// The control cards a run needs around its APL (docs/mode-u.md).
export const CONTROL = [
  { name: "@RUN", cards: ["@RUN   USER,ACCOUNT,PROJECT,1,20"] },
  { name: "@APL,I", cards: ["@APL,I"] },
  { name: "@FIN", cards: ["@FIN"] },
];

export const IDIOMS = [
  { name: "A comment", cards: ["$CM WHAT THIS DECK DOES"] },
  { name: "A vector, V", cards: ["V $AL 3 1 4 1 5 9 2 6"] },
  { name: "A table, shaped from 1 to 12", cards: ["M $AL 3 4 $RH $IO 12", "M"] },
  { name: "Print with quad: the sum of V", cards: ["$QD $AL +/V"] },
  { name: "The mean of V", cards: ["(+/V) $DI $RH V"] },
  { name: "The largest and smallest of V", cards: ["($CE/V),$FL/V"] },
  { name: "V sorted, by grade and brackets", cards: ["V$LB$GU V$RB"] },
  { name: "What in V is over 3", cards: ["(V>3)/V"] },
  { name: "A times table, by outer product", cards: ["($IO 5) $SO.$MU $IO 5"] },
  { name: "A function: the mean", cards: ["$DL Z $AL AVG X", "Z $AL (+/X) $DI $RH X", "$DL", "AVG V"] },
  { name: "A loop: count down", cards: ["$DL COUNT N", "L: $QD $AL N", "N $AL N - 1", "$RA (N>0)/L", "$DL", "COUNT 3"] },
  { name: "Read the next card with quad", cards: ["N $AL $QD", "42"] },
  { name: "Print a message", cards: ["'HELLO, WORLD'"] },
];
