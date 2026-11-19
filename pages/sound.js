// The 2741's sounds, made as they play by WebAudio: nothing recorded.
// The tones are apl-horse-race's (sw-comp-history, the owner's): a
// key's click a short square wave, pitched by the character so each
// sounds its own; a space a softer click; the return a low thunk; the
// feed a falling whir.
//
// A browser plays nothing until the reader has done something on the
// page, so the context is made when Sound is turned on, which is that
// gesture.

let context = null;

// Make sound possible: called from the click that turned it on.
export function wake() {
  context ??= new AudioContext();
  if (context.state === "suspended") context.resume();
}

// Play `event`, when sound is awake.
export function sound(event) {
  if (!context) return;
  switch (event.kind) {
    case "strike":
      return tone("square", 1600 + (event.glyph.codePointAt(0) % 26) * 25, 0.1, 0.03);
    case "space":
    case "back":
      return tone("sine", 1200, 0.08, 0.025);
    case "return":
      return tone("sine", 120, 0.2, 0.04);
    case "feed":
      return tone("sawtooth", 300, 0.08, 0.05, 200);
    default:
  }
}

// One tone: its wave, pitch, loudness and length in seconds, falling
// away as it plays, and sliding to `to` when given.
function tone(type, pitch, loud, seconds, to) {
  const at = context.currentTime;
  const osc = context.createOscillator();
  const gain = context.createGain();
  osc.type = type;
  osc.frequency.value = pitch;
  if (to) osc.frequency.linearRampToValueAtTime(to, at + seconds);
  gain.gain.value = loud;
  gain.gain.exponentialRampToValueAtTime(0.001, at + seconds);
  osc.connect(gain);
  gain.connect(context.destination);
  osc.start(at);
  osc.stop(at + seconds);
}
