"use strict";

/*
 * global mutable state 🙈
 */

const modify = {
  transport: 10,  // q
  instrument: 14, // t
  trackType: 13,  // r
  none: 60,
};

let currentModify = modify.none;
let currentValue = {};

const before = {
  0:  /* z */ {},
  1:  /* x */ {},
  2:  /* c */ {},
  3:  /* v */ {},
  4:  /* b */ {},
  5:  /* a */ {},
  6:  /* s */ {},
  7:  /* d */ {},
  8:  /* f */ {},
  9:  /* g */ {},
  [modify.transport]: {
    "q": "",  "w": "1",  "e": "",   "r": "",   "t": "",
    "y": "5",  "u": "",   "i": "",   "o": "",   "p": "",
    "a": "",   "s": "9",  "d": "",   "f": "",   "g": "",
    "h": "13", "j": "",   "k": "",   "l": "",   ";": "●",
    "z": "",   "x": "T1", "c": "T2", "v": "T3", "b": "T4",
    "n": "T5", "m": "T6", ",": "T7", ".": "T8", "/": "",
  },
  11: /* w */ {},
  12: /* e */ {},
  [modify.trackType]: {
    "n": "EKt", "m": "ESy",
  },
  [modify.instrument]: {
  },
  [modify.none]: {
    "q": "I", "w": "", "e": "", "r": "Typ", "t": "Ins",
    "a": "",  "s": "", "d": "", "f": "",    "g": "",
    "z": "",  "x": "", "c": "", "v": "",    "b": "",
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
    if (key.dataset.control === "play" || key.dataset.send !== currentModify)
      key.dataset.before = before[currentModify][key.dataset.after] || "";
    if (key.dataset.control === "play")
      key.classList.toggle("currentValue", key.dataset.send == currentValue[currentModify]);
  }
};

const handleSend = (event, channel, value) => {
  const message =
      ((event.type === "keyup" ? 8 : 9) << 20)
        | (channel << 16)
        | (value << 8)
        | (currentModify * 2 + 1);
  window.midiIn ? midiIn(message) : console.log(message);
};

const handleModify = (event, value) => {
  if (currentModify === modify.none && event.type === "keydown")
    currentModify = value;
  else if (currentModify === value && event.type === "keyup")
    currentModify = modify.none;
  else
    handleSend(event, 0, value);
  redraw();
};

const handleKeyboardKey = (event, key) => {
  event.preventDefault();
  key.classList.toggle("down", event.type === "keydown");
  if (key.dataset.control === "modify")
    handleModify(event, key.dataset.send);
  else if (key.dataset.control === "play")
    handleSend(event, 1, key.dataset.send);
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

const readBits = (n, message) => {
  const value = message.data >> (message.length - n);
  message.data &= Math.pow(2, message.length - n) - 1;
  message.length -= n;
  return value;
};

const parseBits = message => {
  switch(readBits(4, message)) {
    case 1:
      const playing  = readBits(1, message);
      const armed    = readBits(1, message);
      const position = readBits(4, message);
      const track    = readBits(3, message);
      before[modify.transport]["p"] = playing ? "■" : "▶";
      document.body.classList.toggle("armed", armed);
      sequence.forEach((key, i) => key.classList.toggle("selected", playing && i === position));
      tracklist.forEach((key, i) => key.classList.toggle("selected", i === track));
      break;
    case 2:
      const scale      = readBits(4, message);
      const trackType  = readBits(2, message);
      const instrument = readBits(4, message);
      const rootNote   = readBits(7, message);
      currentValue[modify.trackType] = trackType;
      currentValue[modify.instrument] = instrument;
      before[modify.instrument] = trackType === 0 ? before.kits : before.synths;
      Object.assign(before[modify.none], trackType === 0 ? before.hits : beforeScale(scale, rootNote));
      break;
  }
};

const handleMidi = midi => {
  for (const data of midi)
    parseBits({ data, length: 24 });
  redraw();
};

if (window.midiOut)
  setInterval(() => midiOut().then(handleMidi), 40 /* just over 24 fps */);

// TODO request sync
