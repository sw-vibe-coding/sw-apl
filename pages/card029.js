// A punched card as the 029 leaves it, drawn on a canvas: cream stock
// with its corner cut, the 80 columns' digit rows printed in each
// column, the holes punched, and what was punched printed along the
// top edge. The layout is ibm-1130-rs's keypunch card (the owner's,
// MIT): a card 7 3/8 by 3 1/4 inches, the punching inset from its
// edges, twelve rows, 12 and 11 at the top and 0 to 9 below them.

const ASPECT = 2.269;
const ROWS = 12;
const COLUMNS = 80;

// Draw the card punched from `text`, whose holes are `holes` (80
// numbers, bit r for row r from the top; all ones for a character the
// 029 cannot punch), on `canvas`, filling its width. A `column`
// (from 0) is marked as the one under the punch.
export function drawCard(canvas, text, holes, column = null) {
  const w = canvas.width;
  const h = Math.round(w / ASPECT);
  canvas.height = h;
  const g = canvas.getContext("2d");
  const grid = layout(w, h);
  stock(g, w, h, grid);
  if (column !== null) station(g, grid, column, h);
  digits(g, grid);
  punches(g, grid, holes);
  printed(g, grid, text);
}

// Where the columns and rows are.
function layout(w, h) {
  const left = w * 0.025;
  const top = h * 0.045;
  return {
    left, top,
    col: (w - 2 * left) / COLUMNS,
    row: (h - 2 * top) / ROWS,
    cut: left * 1.6,
  };
}

// The card itself: cream, edged, its top left corner cut away.
function stock(g, w, h, grid) {
  g.clearRect(0, 0, w, h);
  g.beginPath();
  g.moveTo(grid.cut, 0);
  g.lineTo(w, 0);
  g.lineTo(w, h);
  g.lineTo(0, h);
  g.lineTo(0, grid.cut);
  g.closePath();
  g.fillStyle = "#f4e8d0";
  g.fill();
  g.strokeStyle = "#a89f8c";
  g.lineWidth = Math.max(1, w / 800);
  g.stroke();
}

// The column under the punch, shaded down the card.
function station(g, grid, column, h) {
  g.fillStyle = "#e0b04a66";
  g.fillRect(grid.left + column * grid.col, 0, grid.col, h);
}

// Rows 0 to 9 printed in every column, as a card comes from the box.
function digits(g, grid) {
  g.fillStyle = "#b9ab8e";
  g.font = `${Math.round(grid.row * 0.55)}px "Courier New", monospace`;
  g.textAlign = "center";
  g.textBaseline = "middle";
  for (let c = 0; c < COLUMNS; c++) {
    for (let d = 0; d <= 9; d++) {
      const [x, y] = center(grid, c, d + 2);
      g.fillText(String(d), x, y);
    }
  }
}

// The holes: a dark rectangle for each row punched.
function punches(g, grid, holes) {
  g.fillStyle = "#1c1a17";
  const pw = grid.col * 0.6;
  const ph = grid.row * 0.7;
  holes.forEach((bits, c) => {
    for (let r = 0; r < ROWS; r++) {
      if (!(bits & (1 << r))) continue;
      const [x, y] = center(grid, c, r);
      g.fillRect(x - pw / 2, y - ph / 2, pw, ph);
    }
  });
}

// What the 029 printed over each column as it punched it.
function printed(g, grid, text) {
  g.fillStyle = "#000";
  g.font = `${Math.round(grid.col * 1.1)}px "Courier New", monospace`;
  g.textAlign = "center";
  g.textBaseline = "middle";
  [...text].slice(0, COLUMNS).forEach((ch, c) => {
    const [x] = center(grid, c, 0);
    g.fillText(ch, x, grid.top * 0.6);
  });
}

// The middle of column `c`, row `r`.
function center(grid, c, r) {
  return [grid.left + (c + 0.5) * grid.col, grid.top + (r + 0.5) * grid.row];
}
