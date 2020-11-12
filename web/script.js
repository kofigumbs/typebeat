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


/*
 * to native
 */

const keys = document.querySelectorAll(".key");

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
    if (key.dataset.control === "play" || key.dataset.after !== modifier)
      key.dataset.before = before[modifier][key.dataset.after] || "";
    if (key.dataset.control === "play")
      key.classList.toggle("currentValue", key.dataset.after === currentValue[modifier]);
  }
};

const handleSend = (event, value) => {
  const label = modifier + value;
  const float = event.type === "keyup" ? 0 : 1;
  window[label] ? window[label](float) : console.log(label, float);
};

const handleModify = (event, value) => {
  if (modifier === noModifier && event.type === "keydown")
    modifier = value;
  else if (modifier === value && event.type === "keyup")
    modifier = noModifier;
  else
    handleSend(event, value);
  redraw();
};

const handleKeyboardKey = (event, key) => {
  event.preventDefault();
  key.classList.toggle("down", event.type === "keydown");
  if (key.dataset.control === "modify")
    handleModify(event, key.dataset.after);
  else if (key.dataset.control === "play")
    handleSend(event, key.dataset.after);
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
 * from native
 */

const sequence = document.querySelectorAll(".sequence");
const tracklist = document.querySelectorAll(".tracklist");

(async function loop() {
  try {
    const playing = await window.playing();
    const beat = await window.beat();
    console.log(playing);
    before["q"]["p"] = playing ? "■" : "▶";
    sequence.forEach((key, i) => key.classList.toggle("selected", playing && i === beat));
  } catch(e) {
    console.error(e);
  } finally {
    redraw();
    requestAnimationFrame(loop);
  }
})();
