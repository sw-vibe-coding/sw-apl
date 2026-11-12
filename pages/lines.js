// A paste into the demo's line, which holds one line: the clipboard's
// line breaks would be lost and its lines run together. As in
// reva-fork's web REPL (pasteLines), each complete pasted line is run
// as if it were typed and entered, and the last, unfinished one stays
// in the line. CR LF, CR and LF each end a line. Kept apart from the
// page so node can check it (scripts/check-lines.mjs).

// The paste of `pasted` into a line holding `before` the caret and
// `after` it: the complete lines to run, in order, what stays in the
// line, and where in it the caret goes.
export function pasteLines(before, after, pasted) {
  const parts = (before + pasted).split(/\r\n|\r|\n/);
  const last = parts.pop();
  return { lines: parts, rest: last + after, at: last.length };
}
