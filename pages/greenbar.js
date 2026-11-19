// (A) '70's paper, as the 2741 printed on it: continuous green-bar
// paper, the sprocket holes down both edges, each line on its stripe,
// and the line being typed printed into it where the carriage is, as
// a typed line was. The type element, a ball over the print position,
// moves along the line as it prints and back at a return; the paper
// rises a line at each feed.

// A return's time for each column it carries back, and a character's,
// as carriage.js has them, so the ball keeps pace with the printing.
const RETURN_MS = 3;
const CHAR_MS = 67;

// Lay the paper out and put the ball on it. What is returned is called
// after each event the printer shows, and puts the ball where the next
// character will print.
export function greenbar(paper, line) {
  document.body.classList.add("greenbar");
  const point = part("span", "point");
  const ball = part("div", "ball");
  paper.append(ball, point, line);
  let x = 0;
  let queued = false;
  // Once a frame at most: printing at once shows many events a frame.
  // A timer, not requestAnimationFrame, which a hidden window never
  // runs: the ball would stop where it was and stay there.
  const move = () => {
    queued = false;
    // Printing goes on at the point; typing, at the caret.
    paper.append(point, line);
    const at = line.classList.contains("waiting") ? point : line.querySelector("#caret");
    const width = point.getBoundingClientRect().height / 2 || 9;
    const back = (x - at.offsetLeft) / width;
    ball.style.transitionDuration = back > 2 ? `${CHAR_MS + RETURN_MS * back}ms` : "";
    x = at.offsetLeft;
    ball.style.transform = `translate(${x}px, ${at.offsetTop}px)`;
  };
  const place = () => {
    if (queued) return;
    queued = true;
    setTimeout(move, 16);
  };
  // Typing moves the caret, and the ball follows it.
  new MutationObserver(place).observe(line, {
    subtree: true, childList: true, characterData: true, attributes: true,
  });
  place();
  return place;
}

function part(tag, name) {
  const el = document.createElement(tag);
  el.className = name;
  el.setAttribute("aria-hidden", "true");
  return el;
}
