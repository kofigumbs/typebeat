"use strict";

/*
 * global mutable state 🙈
 */

const noModifier = "";
let modifier = noModifier;

const active = {
    "q": [], "w": [], "e": [], "r": [], "t": [],
    "a": [], "s": [], "d": [], "f": [], "g": [],
    "z": [], "x": [], "c": [], "v": [], "b": [],
};

const before = {
  [noModifier]: {
    "q": "Seq", "w": "Typ", "e": "Ins", "r": "Oct", "t": "Scl",
    "y": "∿",   "u": "∿",   "i": "∿",   "o": "∿",   "p": "∿",
    "a": "",    "s": "",    "d": "",    "f": "",    "g": "",
    "h": "∿",   "j": "∿",   "k": "∿",   "l": "∿",   ";": "∿",
    "z": "",    "x": "",    "c": "",    "v": "",    "b": "",
    "n": "∿",   "m": "∿",   ",": "∿",   ".": "∿",   "/": "∿",
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
    "q": "",   "w": "1",  "e": "",   "r": "",   "t": "",
    "y": "5",  "u": "",   "i": "",   "o": "",   "p": "▶",
    "a": "",   "s": "9",  "d": "",   "f": "",   "g": "",
    "h": "13", "j": "",   "k": "",   "l": "",   ";": "●",
    "z": "",   "x": "T1", "c": "T2", "v": "T3", "b": "T4",
    "n": "T5", "m": "T6", ",": "T7", ".": "T8", "/": "",
  },
  w: {
    "n": "Kit", "m": "Mon", ",": "Pol", ".": "Arp", "/": "Chr"
  },
  e: {
    "y": "Vrm", "u": "Cmd", "i": "DMG", "o": "FX4", "p": "Syn",
    "h": "Dp",  "j": "Tch", "k": "Mod", "l": "Gab", ";": "Brg",
    "n": "808", "m": "909", ",": "DMX", ".": "DNB", "/": "Drk",
  },
  r: {
    "h": "5", "j": "6", "k": "7", "l": "8",
    "n": "0", "m": "1", ",": "2", ".": "3", "/": "4",
  },
  t: {
    "y": "MMD", "u": "MMa",
    "h": "Mix", "j": "Loc", "k": "HMi", "l": "HMa", ";": "MMi",
    "n": "Maj", "m": "Min", ",": "Dor", ".": "Phr", "/": "Lyd",
  },
};

const ffi = (label, float) => {
  const method = "groovebox:" + label;
  if (window[method])
    return window[method](float|0);
  if (float !== undefined)
    console.log(method, float);
  return Promise.resolve(0);
}


/*
 * to native
 */

const keys = document.querySelectorAll(".key");
const sequence = document.querySelectorAll(".sequence");
const tracklist = document.querySelectorAll(".tracklist");

const right = Array.from("nm,./hjkl;yuiop");
const sequenceAfters = Array.from(sequence).map(x => x.dataset.after);
const tracklistAfters = Array.from(tracklist).map(x => x.dataset.after);

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
    key.classList.toggle("active", active[modifier].includes(key.dataset.after));
  }
};

const sendMethod = value => {
  if (modifier === noModifier)
    return "key:" + right.indexOf(value);
  if (modifier === "q" && value === "p")
    return "play";
  if (modifier === "q" && value === ";")
    return "arm";
  if (modifier === "q" && sequenceAfters.includes(value))
    return "step:" + sequenceAfters.indexOf(value);
  if (modifier === "q" && tracklistAfters.includes(value))
    return "track:" + tracklistAfters.indexOf(value);
  if (modifier === "w" && right.includes(value))
    return "trackType:" + right.indexOf(value);
  if (modifier === "e" && right.includes(value))
    return "instrument:" + right.indexOf(value);
  if (modifier === "r" && right.includes(value))
    return "octave:" + right.indexOf(value);
  if (modifier === "t" && right.includes(value))
    return "scale:" + right.indexOf(value);
};

const handleModifier = (event, value) => {
  if (modifier === noModifier && event.type === "keydown")
    modifier = value;
  else if (modifier === value && event.type === "keyup")
    modifier = noModifier;
  else
    ffi(sendMethod(value), event.type === "keydown");
  redraw();
};

const handleKeyboardKey = (event, key) => {
  event.preventDefault();
  key.classList.toggle("down", event.type === "keydown");
  if (key.dataset.control === "modify")
    handleModifier(event, key.dataset.after);
  else if (key.dataset.control === "play")
    ffi(sendMethod(key.dataset.after), event.type === "keydown");
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

const update = async () => {
  const playing = await ffi("playing");
  const armed = await ffi("armed");
  const track = await ffi("track");
  const trackType = await ffi("trackType");
  const instrument = await ffi("instrument");
  const root = await ffi("root");
  const octave = await ffi("octave");
  const scale = await ffi("scale");
  const beat = await ffi("beat");
  const key = await ffi("key");
  const hits = await Promise.all(Array.from(sequence, (_, i) => ffi("hit:" + i)));

  let activeHits = [];
  hits.forEach((hit, i) => { if (hit) activeHits.push(sequenceAfters[i]) });

  Object.assign(active, {
    [noModifier]: [right[key]],
    q: activeHits, w: [right[trackType]], e: [right[instrument]], r: [right[octave]], t: [right[scale]],
  });

  document.body.classList.toggle("armed", armed);
  sequence.forEach((key, i) => key.classList.toggle("highlight", playing && i === beat));
  tracklist.forEach((key, i) => key.classList.toggle("highlight", i === track));
  redraw();
}

(async function loop() {
  try { await update() }
  catch(e) { console.error(e) }
  finally { requestAnimationFrame(loop) }
})();
