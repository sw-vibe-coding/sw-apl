// The 2741's sounds, from recordings rather than tones.
//
// A key's strike and the space bar are the trimmed samples of the
// owner's IBM 1130 printer demo (sw-comp-history/ibm-1130-rs, MIT),
// each strike pitched a little by its character so a line does not
// sound like one key; the carriage return is the thud the owner's
// text-to-sfx-gen generates (thud_generator.py, MIT), a click, a body
// falling from 200 to 80 hertz and a knock that small speakers carry.
// A feed comes with its return and has no sound of its own.
//
// A browser plays nothing until the reader has done something on the
// page, so the sounds are loaded when Sound is turned on, which is
// that gesture.

// The samples, at the version this module was fetched at.
const at = new URL(import.meta.url).search;
const FILES = { key: "key.wav", space: "space.wav", thud: "return.wav" };

let context = null;
let samples = null;

// Make sound possible: called from the click that turned it on.
export function wake() {
  context ??= new AudioContext();
  if (context.state === "suspended") context.resume();
  samples ??= load();
}

// Each sample, decoded; one that cannot be had is silence.
async function load() {
  const got = {};
  for (const [name, file] of Object.entries(FILES)) {
    try {
      const bytes = await fetch(new URL(`sounds/${file}${at}`, import.meta.url))
        .then((r) => r.arrayBuffer());
      got[name] = await context.decodeAudioData(bytes);
    } catch {
      got[name] = null;
    }
  }
  return got;
}

// How loud each plays. The samples were not made at one level: the
// loudest 50 ms of a strike is 0.0044 of full scale, of the space bar
// 0.0017, of the thud 0.31. These bring each to about 0.03, the thud a
// little over, none past full scale at its peak.
// Play `event`, once sound is awake and the samples are in.
export async function sound(event) {
  if (!samples) return;
  const s = await samples;
  switch (event.kind) {
    case "strike":
      return play(s.key, 8, 0.9 + (event.glyph.codePointAt(0) % 20) * 0.01);
    case "back":
      return play(s.key, 4, 0.8);
    case "space":
      return play(s.space, 15, 1);
    case "return":
      return play(s.thud, 0.13, 1);
    default:
  }
}

// One sample, at `loud` and `rate`.
function play(buffer, loud, rate) {
  if (!buffer) return;
  const source = context.createBufferSource();
  const gain = context.createGain();
  source.buffer = buffer;
  source.playbackRate.value = rate;
  gain.gain.value = loud;
  source.connect(gain);
  gain.connect(context.destination);
  source.start();
}
