"use strict";

/*
 * global mutable state 🙈
 */

const transport  = 10; // q
const instrument = 14; // t
const noModifier = 60;

let modifier = noModifier;
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
  [transport]: {
    "q": "",  "w": "1",  "e": "",   "r": "",   "t": "",
    "y": "5",  "u": "",   "i": "",   "o": "",   "p": "",
    "a": "",   "s": "9",  "d": "",   "f": "",   "g": "",
    "h": "13", "j": "",   "k": "",   "l": "",   ";": "●",
    "z": "",   "x": "T1", "c": "T2", "v": "T3", "b": "T4",
    "n": "T5", "m": "T6", ",": "T7", ".": "T8", "/": "",
  },
  11: /* w */ {},
  12: /* e */ {},
  13: /* r */ {},
  [instrument]: {
  },
  [noModifier]: {
    "q": "I", "w": "", "e": "", "r": "", "t": "Ins",
    "a": "",  "s": "", "d": "", "f": "", "g": "",
    "z": "",  "x": "", "c": "", "v": "", "b": "",
  },
};

const reference = {
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
};


/*
 * to native
 */

const keys = document.querySelectorAll(".key");

const redraw = () => {
  for (const key of keys) {
    if (key.dataset.control === "play" || key.dataset.send !== modifier)
      key.dataset.before = before[modifier][key.dataset.after] || "";
    if (key.dataset.control === "play")
      key.classList.toggle("currentValue", key.dataset.send == currentValue[modifier]);
  }
};

const handleSend = (event, channel, value) => {
  const message =
      ((event.type === "keyup" ? 8 : 9) << 20)
        | (channel << 16)
        | (value << 8)
        | (modifier * 2 + 1);
  window.midiIn ? midiIn(message) : console.log(message);
};

const handleModify = (event, value) => {
  if (modifier === noModifier && event.type === "keydown")
    modifier = value;
  else if (modifier === value && event.type === "keyup")
    modifier = noModifier;
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
    if (event.keyCode == key.dataset.code)
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

const setBeat = value => {
  before[transport]["p"] = value < 16 ? "■" : "▶";
  sequence.forEach((key, index) => {
    key.classList.toggle("selected", index === value);
  });
};

const setTrack = value => {
  tracklist.forEach((key, index) => {
    key.classList.toggle("selected", index === value);
  });
};

const setKit = value => {
  currentValue[instrument] = value;
  before[instrument] = reference.kits;
  Object.assign(before[noModifier], reference.hits);
};

const setKey = value => {
  currentValue[instrument] = value;
  // TODO
}

const setArmed = value => {
  document.body.classList.toggle("armed", !!value);
};

const set = (context, value) => {
  switch (context) {
    case 0: return setBeat(value);
    case 1: return setTrack(value);
    case 2: return setKit(value);
    case 3: return setKey(value);
    case 4: return setArmed(value);
  }
};

const handleMidi = midi => {
  for (const message of midi)
    set(message >> 16, message & 0x00ffff);
  redraw();
};

if (window.midiOut)
  setInterval(() => midiOut().then(handleMidi), 40 /* just over 24 fps */);
