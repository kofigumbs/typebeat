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
    "q": "Seq", "w": "Typ", "e": "Snd", "r": "Oct", "t": "",
    "y": "~",   "u": "~",   "i": "~",   "o": "~",   "p": "~",
    "a": "Trk", "s": "",    "d": "",    "f": "",    "g": "",
    "h": "~",   "j": "~",   "k": "~",   "l": "~",   ";": "~",
    "z": "Key", "x": "",    "c": "",    "v": "",    "b": "",
    "n": "~",   "m": "~",   ",": "~",   ".": "~",   "/": "~",
  },
  z: {
    "q": "",    "w": "G#",  "e": "A",   "r": "A#",  "t": "B",
    "y": "MMD", "u": "MMa", "i": "",    "o": "",    "p": "",
    "a": "",    "s": "E",   "d": "F",   "f": "F#",  "g": "G",
    "h": "Mix", "j": "Loc", "k": "HMi", "l": "HMa", ";": "MMi",
    "z": "",    "x": "C",   "c": "C#",  "v": "D",   "b": "D#",
    "n": "Maj", "m": "Min", ",": "Dor", ".": "Phr", "/": "Lyd",
  },
  x: {},
  c: {},
  v: {},
  b: {},
  a: {
    "y": "",  "u": "",  "i": "",  "o": "",  "p": "▶",
    "a": "Mut",
    "h": "",  "j": "",  "k": "",  "l": "",  ";": "●",
    "z": "",  "x": "1", "c": "2", "v": "3", "b": "4",
    "n": "5", "m": "6", ",": "7", ".": "8", "/": "",
  },
  s: {},
  d: {},
  f: {},
  g: {},
  q: {
    "q": "", "w": "Stp", "e": "", "r": "", "t": "",
    "y": "", "u": "",    "i": "", "o": "", "p": "",
    "a": "", "s": "",    "d": "", "f": "", "g": "",
    "h": "", "j": "",    "k": "", "l": "", ";": "",
    "z": "", "x": "Len", "c": "", "v": "", "b": "",
    "n": "",  "m": "",   ",": "", ".": "", "/": "",
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
  t: {},
};

const sends = {
  [noModifier]: {
    "y": { trig: "key:10" }, "u": { trig: "key:11" }, "i": { trig: "key:12" }, "o": { trig: "key:13" }, "p": { trig: "key:14" },
    "h": { trig: "key:5" },  "j": { trig: "key:6" },  "k": { trig: "key:7" },  "l": { trig: "key:8" },  ";": { trig: "key:9" },
    "n": { trig: "key:0" },  "m": { trig: "key:1" },  ",": { trig: "key:2" },  ".": { trig: "key:3" },  "/": { trig: "key:4" },
  },
  "q": {
    "w": { trig: "step:0" },  "e": { trig: "step:1" },  "r": { trig: "step:2" },  "t": { trig: "step:3" },
    "y": { trig: "step:4" },  "u": { trig: "step:5" },  "i": { trig: "step:6" },  "o": { trig: "step:7" },
    "s": { trig: "step:8" },  "d": { trig: "step:9" },  "f": { trig: "step:10" }, "g": { trig: "step:11" },
    "h": { trig: "step:12" }, "j": { trig: "step:13" }, "k": { trig: "step:14" }, "l": { trig: "step:15" },
    "x": { trig: "length:0" }, "c": { trig: "length:1" }, "v": { trig: "length:2" }, "b": { trig: "length:3" },
    "n": { trig: "length:4" }, "m": { trig: "length:5" }, ",": { trig: "length:6" }, ".": { trig: "length:7" },
  },
  "a": {
    "p": { trig: "play" },
    ";": { trig: "arm" },
    "/": { tapTempo: "setBpm" },
    "x": { trig: "track:0" }, "c": { trig: "track:1" }, "v": { trig: "track:2" }, "b": { trig: "track:3" },
    "n": { trig: "track:4" }, "m": { trig: "track:5" }, ",": { trig: "track:6" }, ".": { trig: "track:7" },
  },
  "w": {
    "n": { trig: "trackType:0" },  "m": { trig: "trackType:1" },  ",": { trig: "trackType:2" },  ".": { trig: "trackType:3" },  "/": { trig: "trackType:4" },
  },
  "e": {
    "y": { trig: "sounds:10" }, "u": { trig: "sounds:11" }, "i": { trig: "sounds:12" }, "o": { trig: "sounds:13" }, "p": { trig: "sounds:14" },
    "h": { trig: "sounds:5" },  "j": { trig: "sounds:6" },  "k": { trig: "sounds:7" },  "l": { trig: "sounds:8" },  ";": { trig: "sounds:9" },
    "n": { trig: "sounds:0" },  "m": { trig: "sounds:1" },  ",": { trig: "sounds:2" },  ".": { trig: "sounds:3" },  "/": { trig: "sounds:4" },
  },
  "r": {
    "h": { trig: "octave:5" },  "j": { trig: "octave:6" },  "k": { trig: "octave:7" },  "l": { trig: "octave:8" },  ";": { trig: "octave:9" },
    "n": { trig: "octave:0" },  "m": { trig: "octave:1" },  ",": { trig: "octave:2" },  ".": { trig: "octave:3" },  "/": { trig: "octave:4" },
  },
  "z": {
    "w": { trig: "root:8" }, "e": { trig: "root:9" }, "r": { trig: "root:10" }, "t": { trig: "root:11" },
    "s": { trig: "root:4" }, "d": { trig: "root:5" }, "f": { trig: "root:6" },  "g": { trig: "root:7" },
    "x": { trig: "root:0" }, "c": { trig: "root:1" }, "v": { trig: "root:2" },  "b": { trig: "root:3" },
    "y": { trig: "scale:8" }, "u": { trig: "scale:9" }, "i": { trig: "scale:10" }, "o": { trig: "scale:11" },
    "h": { trig: "scale:4" }, "j": { trig: "scale:5" }, "k": { trig: "scale:6" },  "l": { trig: "scale:7" },
    "n": { trig: "scale:0" }, "m": { trig: "scale:1" }, ",": { trig: "scale:2" },  ".": { trig: "scale:3" },
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

const left12 = Array.from("xcvbsdfgwert");
const right12 = Array.from("nm,.hjklyuio");
const right15 = Array.from("nm,./hjkl;yuiop");
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

const tapTempo = {
  taps: [],
  reset() {
    this.taps = [];
  },
  push(method, timeStamp) {
    this.taps.push(timeStamp);
    if (this.taps.length > 1) {
      let diffs = 0;
      for (let i = 1; i < this.taps.length; i++)
        diffs += this.taps[i] - this.taps[i-1];
      ffi(method, Math.round(60000 / (diffs / (this.taps.length - 1))));
    }
  },
};

const resetModifier = () => {
  tapTempo.reset();
  modifier = noModifier;
};

const handleSend = (event, value) => {
  if (!sends[modifier] || !sends[modifier][value])
    return;
  if (sends[modifier][value].trig)
    return ffi(sends[modifier][value].trig, event.type === "keydown");
  if (sends[modifier][value].tapTempo)
    return event.type === "keydown" && tapTempo.push(sends[modifier][value].tapTempo, event.timeStamp);
};

const handleModifier = (event, value) => {
  if (modifier === noModifier && event.type === "keydown")
    modifier = value;
  else if (modifier === value && event.type === "keyup")
    resetModifier();
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
  const bpm = await ffi("bpm");
  const playing = await ffi("playing");
  const armed = await ffi("armed");
  const track = await ffi("track");
  const trackType = await ffi("trackType");
  const page = await ffi("page");
  const length = await ffi("length");
  const sounds = await ffi("sounds");
  const octave = await ffi("octave");
  const root = await ffi("root");
  const scale = await ffi("scale");
  const beat = await ffi("beat");
  const key = await ffi("key");
  const hits = await Promise.all(Array.from(sequence, (_, i) => ffi("hit:" + i)));

  let activeHits = [];
  hits.forEach((hit, i) => { if (hit) activeHits.push(sequenceAfters[i]) });

  Object.assign(active, {
    [noModifier]: [right15[key]],
    q: activeHits, w: [right15[trackType]], e: [right15[sounds]], r: [right15[octave]],
    a: [tracklistAfters[track]],
    z: [left12[root], right12[scale]],
  });

  before["a"]["/"] = bpm;

  document.body.classList.toggle("armed", armed);
  sequence.forEach((key, i) => key.classList.toggle("highlight", playing && i === beat));
  tracklist.forEach((key, i) => key.classList.toggle("highlight", i <= length));
  tracklist.forEach((key, i) => key.classList.toggle("lowlight", i === page));
  redraw();
}

(async function loop() {
  try { await update() }
  catch(e) { console.error(e) }
  finally { requestAnimationFrame(loop) }
})();
