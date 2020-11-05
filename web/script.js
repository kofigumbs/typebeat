"use strict";

/*
 * to native
 */

let modifiers = [];

const keys = document.querySelectorAll(".key");

const noModifier = 60;
const shift      = 61;
const alt        = 62;

const befores = {
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
  10: /* q */ {},
  11: /* w */ {},
  12: /* e */ {},
  13: /* r */ {},
  14: /* t */ {},
  kit: {
    "y": "CH", "u": "CY", "i": "FX", "o": "FX", "p": "FX",
    "h": "SD", "j": "SD", "k": "CP", "l": "OH", ";": "OH",
    "n": "BD", "m": "BD", ",": "BD", ".": "LT", "/": "SD",
  },
  [shift]: {
    "q": "",   "w": "1",  "e": "",   "r": "",   "t": "",
    "y": "5",  "u": "",   "i": "",   "o": "",   "p": "\u25B6",
    "a": "",   "s": "9",  "d": "",   "f": "",   "g": "",
    "h": "13", "j": "",   "k": "",   "l": "",   ";": "\u25CF",
    "z": "",   "x": "T1", "c": "T2", "v": "T3", "b": "T4",
    "n": "T5", "m": "T6", ",": "T7", ".": "T8", "/": "S",
  },
  [alt]: {
    "q": "",  "w": "+", "e": "+", "r": "+", "t": "+",
    "y": "+", "u": "+", "i": "+", "o": "+", "p": "",
    "a": "",  "s": "-", "d": "-", "f": "-", "g": "-",
    "h": "-", "j": "-", "k": "-", "l": "-", ";": "",
    "z": "",  "x": "M", "c": "M", "v": "M", "b": "M",
    "n": "M", "m": "M", ",": "M", ".": "M", "/": "",
  },
  [noModifier]: {}, // this is mutated with setKey and setKit
};

const getModifier = () => {
  if (modifiers.includes(shift)) return shift;
  if (modifiers.includes(alt)) return alt;
  return modifiers.length === 0 ? noModifier : modifiers[0];
};

const updateBefores = () => {
  for(const key of keys)
    key.dataset.before = befores[getModifier()][key.dataset.after] || "";
};

const onModify = (event, value) => {
  event.preventDefault();
  modifiers = event.type === "keydown" ? [ value, ...modifiers ] : modifiers.filter(x => x !== value);
  updateBefores();
};

const onSend = (event, channel, value) => {
  const message =
      ((event.type === "keyup" ? 8 : 9) << 20)
        | (channel << 16)
        | (value << 8)
        | (getModifier() * 2 + 1);
  window.midiIn ? midiIn(message) : console.log(message);
};

const onKeyboardKey = (event, key) => {
  event.preventDefault();
  key.classList.toggle("down", event.type === "keydown");
  if (key.dataset.control === "play")
    onSend(event, 0, key.dataset.send);
  if (key.dataset.control === "modify" && (event.shiftKey || event.altKey))
    onSend(event, 1, key.dataset.send);
  if (key.dataset.control === "modify")
    onModify(event, key.dataset.send);
};

const onDocumentKey = event => {
  if (event.ctrlKey || event.metaKey || event.repeat)
    return;
  if (event.key === "Shift")
    return onModify(event, shift);
  if (event.key === "Alt")
    return onModify(event, alt);
  for (const key of keys)
    if (event.keyCode == key.dataset.code)
      return onKeyboardKey(event, key);
};

document.addEventListener("keydown", onDocumentKey);
document.addEventListener("keyup", onDocumentKey);
document.addEventListener("keypress", event => event.preventDefault());


/* 
 * from native
 */

const sequence = document.querySelectorAll(".sequence");
const navigation = document.querySelectorAll(".navigation");
const right = document.querySelectorAll(".right");

const setBeat = value => {
  sequence.forEach((key, index) => {
    key.classList.toggle("current", index === value);
  });
};

const setTrack = value => {
  navigation.forEach((key, index) => {
    key.classList.toggle("current", index === value);
  });
};

const setKit = () => {
  befores[noModifier] = befores.kit;
  updateBefores();
};

const setArmed = value => {
  document.body.classList.toggle("armed", !!value);
};

const update = (context, value) => {
  switch (context) {
    case 0: return setBeat(value);
    case 1: return setTrack(value);
    case 2: return setKit();
    case 3: return setKey(value);
    case 4: return setArmed(value);
  }
};

const onMidiOut = midi => {
  for (const message of midi)
    update(message >> 16, message & 0x00ffff);
};

if (window.midiOut)
  setInterval(() => midiOut().then(onMidiOut), 40 /* just over 24 fps */);
