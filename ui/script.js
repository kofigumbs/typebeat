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

const sends = {
  [noModifier]: {
    "y": { trig: "key:10" }, "u": { trig: "key:11" }, "i": { trig: "key:12" }, "o": { trig: "key:13" }, "p": { trig: "key:14" },
    "h": { trig: "key:5" },  "j": { trig: "key:6" },  "k": { trig: "key:7" },  "l": { trig: "key:8" },  ";": { trig: "key:9" },
    "n": { trig: "key:0" },  "m": { trig: "key:1" },  ",": { trig: "key:2" },  ".": { trig: "key:3" },  "/": { trig: "key:4" },
  },
  "q": {
    "p": { trig: "play" },
    ";": { trig: "arm" },
    "w": { trig: "step:0" },  "e": { trig: "step:1" },  "r": { trig: "step:2" },  "t": { trig: "step:3" },
    "y": { trig: "step:4" },  "u": { trig: "step:5" },  "i": { trig: "step:6" },  "o": { trig: "step:7" },
    "s": { trig: "step:8" },  "d": { trig: "step:9" },  "f": { trig: "step:10" }, "g": { trig: "step:11" },
    "h": { trig: "step:12" }, "j": { trig: "step:13" }, "k": { trig: "step:14" }, "l": { trig: "step:15" },
    "x": { trig: "track:0" }, "c": { trig: "track:1" }, "v": { trig: "track:2" }, "b": { trig: "track:3" },
    "n": { trig: "track:4" }, "m": { trig: "track:5" }, ",": { trig: "track:6" }, ".": { trig: "track:7" },
  },
  "w": {
    "n": { trig: "trackType:0" },  "m": { trig: "trackType:1" },  ",": { trig: "trackType:2" },  ".": { trig: "trackType:3" },  "/": { trig: "trackType:4" },
  },
  "e": {
    "y": { trig: "instrument:10" }, "u": { trig: "instrument:11" }, "i": { trig: "instrument:12" }, "o": { trig: "instrument:13" }, "p": { trig: "instrument:14" },
    "h": { trig: "instrument:5" },  "j": { trig: "instrument:6" },  "k": { trig: "instrument:7" },  "l": { trig: "instrument:8" },  ";": { trig: "instrument:9" },
    "n": { trig: "instrument:0" },  "m": { trig: "instrument:1" },  ",": { trig: "instrument:2" },  ".": { trig: "instrument:3" },  "/": { trig: "instrument:4" },
  },
  "r": {
    "h": { trig: "octave:5" },  "j": { trig: "octave:6" },  "k": { trig: "octave:7" },  "l": { trig: "octave:8" },  ";": { trig: "octave:9" },
    "n": { trig: "octave:0" },  "m": { trig: "octave:1" },  ",": { trig: "octave:2" },  ".": { trig: "octave:3" },  "/": { trig: "octave:4" },
  },
  "t": {
    "y": { trig: "scale:10" }, "u": { trig: "scale:11" },
    "h": { trig: "scale:5" },  "j": { trig: "scale:6" },  "k": { trig: "scale:7" },  "l": { trig: "scale:8" },  ";": { trig: "scale:9" },
    "n": { trig: "scale:0" },  "m": { trig: "scale:1" },  ",": { trig: "scale:2" },  ".": { trig: "scale:3" },  "/": { trig: "scale:4" },
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

const handleSend = (event, value) => {
  if (!sends[modifier] || !sends[modifier][value])
    return;
  if (sends[modifier][value].trig)
    return ffi(sends[modifier][value].trig, event.type === "keydown");
};

const handleModifier = (event, value) => {
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
    handleModifier(event, key.dataset.after);
  else if (key.dataset.control === "send")
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
