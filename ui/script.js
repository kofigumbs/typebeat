"use strict";

/*
 * global mutable state 🙈
 */

const noModifier = "";
let modifier = noModifier;

const currentValue = {
};

const before = {
  [noModifier]: {
    "q": "I", "w": "", "e": "", "r": "Typ", "t": "Ins",
    "a": "",  "s": "", "d": "", "f": "",    "g": "",
    "z": "",  "x": "", "c": "", "v": "",    "b": "",
  },
  z: {},
  x: {},
  c: {},
  v: {},
  b: {},
  a: {},
  s: {},
  d: {},
  f: {},
  g: {},
  q: {
    "q": "",  "w": "1",  "e": "",   "r": "",   "t": "",
    "y": "5",  "u": "",   "i": "",   "o": "",   "p": "",
    "a": "",   "s": "9",  "d": "",   "f": "",   "g": "",
    "h": "13", "j": "",   "k": "",   "l": "",   ";": "●",
    "z": "",   "x": "T1", "c": "T2", "v": "T3", "b": "T4",
    "n": "T5", "m": "T6", ",": "T7", ".": "T8", "/": "",
  },
  w: {},
  e: {},
  r: {
    "n": "EKt", "m": "ESy",
  },
  t: {
  },
  kits: {
    "y": "Vrm", "u": "Cmd", "i": "DMG", "o": "FX4", "p": "",
    "h": "Dp",  "j": "Tch", "k": "Mod", "l": "Gab", ";": "Brg",
    "n": "808", "m": "909", ",": "DMX", ".": "DNB", "/": "Drk",
  },
  hits: {
    "y": "CH", "u": "CY", "i": "FX", "o": "FX", "p": "FX",
    "h": "SD", "j": "SD", "k": "CP", "l": "OH", ";": "OH",
    "n": "BD", "m": "BD", ",": "BD", ".": "LT", "/": "SD",
  },
  synths: {
  },
};

const scale = {
  keys: [ "n", "m", ",", ".", "/", "h", "j", "k", "l", ";", "y", "u", "i", "o", "p" ],
  notes: [ "C", "C#", "D", "D#", "E", "F", "F#", "G", "G#", "A", "A#", "B" ],
  names: [ "Maj", "Min", "Dor", "Phr", "Lyd", "Mix", "Loc", "HMi", "HMa", "MMi", "MMD", "MMa" ],
  offsets: [
    [ 0, 2, 4, 5, 7, 9, 11 ],
    [ 0, 2, 3, 5, 7, 8, 10 ],
    [ 0, 2, 3, 5, 7, 9, 10 ],
    [ 0, 1, 3, 5, 7, 8, 10 ],
    [ 0, 2, 4, 6, 7, 9, 11 ],
    [ 0, 2, 4, 5, 7, 9, 10 ],
    [ 0, 1, 3, 5, 6, 8, 10 ],
    [ 0, 2, 3, 5, 7, 8, 11 ],
    [ 0, 2, 4, 5, 7, 8, 11 ],
    [ 0, 2, 3, 5, 7, 9, 11 ],
    [ 0, 2, 3, 5, 7, 8, 10 ],
    [ 0, 2, 4, 5, 7, 8, 10 ],
  ],
};

const beforeScale = (index, rootNote) => {
  const legend = {};
  scale.keys.forEach((key, i) => {
    const offsets = scale.offsets[index];
    legend[key] = scale.notes[(rootNote + offsets[i % offsets.length]) % scale.notes.length];
  });
  return legend;
};

const engine = (label, float) => {
  const method = "groovebox:" + label;
  if (window[method])
    return window[method](float);
  if (float !== undefined)
    console.log(method, float);
  return Promise.resolve(0);
}


/*
 * to engine
 */

const keys = document.querySelectorAll(".key");
const right = Array.from("nm,./hjkl;yuiop");

const toCode = after => {
  switch (after) {
    case ";": return "Semicolon";
    case ",": return "Comma";
    case ".": return "Period";
    case "/": return "Slash";
    default:  return "Key" + after.toUpperCase();
  }
};

const redraw = () => {
  for (const key of keys) {
    if (key.dataset.after !== modifier)
      key.dataset.before = before[modifier][key.dataset.after] || "";
    key.classList.toggle("currentValue", right.indexOf(key.dataset.after) === currentValue[modifier]);
  }
};

const interpret = (event, value) => {
  let method, argument = event.type === "keyup" ? 0 : 1;
  if (modifier === noModifier)
    method = "note:" + right.indexOf(value);
  else if (modifier === "q" && value === "p")
    method = "play";
  else if (modifier === "q" && value === ";")
    method = "arm";
  else if (modifier === "t")
    method = "setInstrument", argument = right.indexOf(value);
  engine(method, argument);
};

const handleModifier = (event, value) => {
  if (modifier === noModifier && event.type === "keydown")
    modifier = value;
  else if (modifier === value && event.type === "keyup")
    modifier = noModifier;
  else
    interpret(event, value);
  redraw();
};

const handleKeyboardKey = (event, key) => {
  event.preventDefault();
  key.classList.toggle("down", event.type === "keydown");
  if (key.dataset.control === "modify")
    handleModifier(event, key.dataset.after);
  else if (key.dataset.control === "play")
    interpret(event, key.dataset.after);
};

const handleDocumentKey = event => {
  if (event.ctrlKey || event.metaKey || event.shiftKey || event.altKey || event.repeat)
    return;
  for (const key of keys)
    if (event.code === toCode(key.dataset.after))
      return handleKeyboardKey(event, key);
};

document.addEventListener("keydown", handleDocumentKey);
document.addEventListener("keyup", handleDocumentKey);
document.addEventListener("keypress", event => event.preventDefault());


/* 
 * from engine
 */

const sequence = document.querySelectorAll(".sequence");
const tracklist = document.querySelectorAll(".tracklist");

const update = async () => {
  const playing = await engine("playing");
  const beat = await engine("beat");
  const armed = await engine("armed");
  const track = await engine("track");
  const scale = await engine("scale");
  const trackType = await engine("trackType");
  const instrument = await engine("instrument");
  const rootNote = await engine("rootNote");
  before.q.p = playing ? "■" : "▶";
  document.body.classList.toggle("armed", armed);
  sequence.forEach((key, i) => key.classList.toggle("selected", playing && i === beat));
  tracklist.forEach((key, i) => key.classList.toggle("selected", i === track));
  currentValue.r = trackType;
  currentValue.t = instrument;
  before.t = trackType === 0 ? before.kits : before.synths;
  Object.assign(before[noModifier], trackType === 0 ? before.hits : beforeScale(scale, rootNote));
  redraw();
}

(async function loop() {
  try { await update() }
  catch(e) { console.error(e) }
  finally { requestAnimationFrame(loop) }
})();
