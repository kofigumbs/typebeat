"use strict";

/*
 * to native
 */

let modifiers = [];

const keys = document.querySelectorAll(".key");

const beforeByModiferSend = {
  1: /* z */ {},
  2: /* x */ {},
  3: /* c */ {},
  4: /* v */ {},
  5: /* b */ {},
  6: /* a */ {},
  7: /* s */ {},
  8: /* d */ {},
  9: /* f */ {},
  10: /* g */ {},
  11: /* q */ {},
  12: /* w */ {},
  13: /* e */ {},
  14: /* r */ {},
  15: /* t */ {},
  101: /* shift */ {
    "q": "",   "w": "1",  "e": "2",  "r": "3",  "t": "4",
    "y": "5",  "u": "6",  "i": "7",  "o": "8",  "p": "\u25B6",
    "a": "",   "s": "9",  "d": "10", "f": "11", "g": "12",
    "h": "13", "j": "14", "k": "15", "l": "16", ";": "\u25CF",
    "z": "",   "x": "T1", "c": "T2", "v": "T3", "b": "T4",
    "n": "T5", "m": "T6", ",": "T7", ".": "T8", "/": "S",
  },
  102: /* alt */ {
    "q": "", "w": "+", "e": "+", "r": "+", "t": "+",
    "y": "+", "u": "+", "i": "+", "o": "+", "p": "",
    "a": "", "s": "-", "d": "-", "f": "-", "g": "-",
    "h": "-", "j": "-", "k": "-", "l": "-", ";": "",
    "z": "", "x": "M", "c": "M", "v": "M", "b": "M",
    "n": "M", "m": "M", ",": "M", ".": "M", "/": "",
  },
  default: {},
};

const onModify = (event, value) => {
  event.preventDefault();
  modifiers = event.type === "keydown" ? [ value, ...modifiers ] : modifiers.filter(x => x !== value);
  for(const key of keys)
    key.dataset.before = beforeByModiferSend[modifiers[0] || "default"][key.dataset.after] || "";
};

const onNote = (event, channel, value) => {
  const message =
      ((event.type === "keyup" ? 8 : 9) << 20)
        | (channel << 16)
        | (value << 8)
        | (modifiers[0] || 100);
  window.midiIn ? midiIn(message) : console.log(message);
};

const onKeyboardKey = (event, key) => {
  event.preventDefault();
  key.classList.toggle("down", event.type === "keydown");
  if (key.dataset.control === "play")
    onNote(event, 0, key.dataset.send);
  if (key.dataset.control === "modify" && (event.shiftKey || event.altKey))
    onNote(event, 1, key.dataset.send);
  if (key.dataset.control === "modify")
    onModify(event, key.dataset.send);
};

const onDocumentKey = event => {
  if (event.ctrlKey || event.metaKey || event.repeat)
    return;
  if (event.key === "Shift")
    onModify(event, 101);
  if (event.key === "Alt")
    onModify(event, 102);
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

const setBeat = value => {
  sequence.forEach((key, index) => {
    key.classList.toggle("current", index === value);
  });
};

const update = (context, value) => {
  switch (context) {
    case 0: return setBeat(value);
    case 1: return setTrack(value);
    case 2: return setKit(value);
    case 3: return setKey(value);
  }
};

const onMidiOut = midi => {
  for (const message of midi)
    update(message >> 16, message & 0x00ffff);
};

if (window.midiOut)
  setInterval(() => midiOut().then(onMidiOut), 40 /* just over 24 fps */);
