// (A) '70's two views of the 2741: 2D, the green-bar paper on its own
// (greenbar.js), and 3D, the owner's model of the terminal printing
// on it (model2741.js). A button in the bar switches them and the
// choice is remembered. The 3D view is loaded the first time it is
// chosen, and its sheet begins with what is printed already.

const at = new URL(import.meta.url).search;
const KEPT = "apl-2741-view";

// The views, beside `paper`, with their button in `bar`. What is
// returned follows each printer event: `place` moves the 2D ball.
export function views(bar, paper, line, place) {
  const host = document.createElement("div");
  host.id = "model";
  host.hidden = true;
  paper.before(host);
  const button = document.createElement("button");
  button.type = "button";
  bar.append(button);
  let model = null;
  let loading = null;
  const typing = () => (line.classList.contains("waiting") ? null : line.textContent);
  const printed = () => paper.textContent.slice(0, paper.textContent.length - line.textContent.length);
  const show = (three) => {
    document.body.classList.toggle("model3d", three);
    host.hidden = !three;
    button.textContent = three ? "2D" : "3D";
    button.title = three ? "The green-bar paper alone" : "The 2741 printing, in 3D";
    keep(three ? "3d" : "2d");
    if (!three) return place();
    loading ??= import(`./model2741.js${at}`)
      .then(({ model2741 }) => model2741(host))
      .then((m) => {
        model = m;
        m.history(printed());
        m.typing(typing());
      })
      .catch((e) => {
        host.textContent = `The 3D view could not be shown here: ${e.message}`;
      });
  };
  button.addEventListener("click", () => show(host.hidden));
  new MutationObserver(() => model?.typing(typing())).observe(line, {
    subtree: true, childList: true, characterData: true, attributes: true,
  });
  show(kept() === "3d");
  return (event) => {
    place(event);
    model?.follow(event);
  };
}

function keep(value) {
  try {
    localStorage.setItem(KEPT, value);
  } catch {
    // Not kept; this visit holds it.
  }
}

function kept() {
  try {
    return localStorage.getItem(KEPT);
  } catch {
    return null;
  }
}
