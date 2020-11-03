/*
 * globals
 */

const config = {
  keys: {
    // layout
    row1: Array.from("qwertyuiop"),
    row2: Array.from("asdfghjkl;"),
    row3: Array.from("zxcvbnm,./"),
    // role
    sequence: Array.from("wertyuiosdfghjkl"),
    control: Array.from("qpa;z/"),
    navigation: Array.from("xcvbnm,."),
    // left hand
    effects: {
      "q": "", "w": "", "e": "", "r": "", "t": "",
      "a": "", "s": "", "d": "", "f": "", "g": "",
      "z": "", "x": "", "c": "", "v": "\u276E", "b": "\u276F",
    },
    // right hand
    keyboard: Array.from("nm,./hjkl;yuiop"),
    // modifier layers
    shift: {
      "q": "", "w": "1", "e": "2", "r": "3", "t": "4",
      "y": "5", "u": "6", "i": "7", "o": "8", "p": "\u25B6",
      "a": "", "s": "9", "d": "10", "f": "11", "g": "12",
      "h": "13", "j": "14", "k": "15", "l": "16", ";": "\u25CF",
      "z": "", "x": "T1", "c": "T2", "v": "T3", "b": "T4",
      "n": "T5", "m": "T6", ",": "T7", ".": "T8", "/": "S",
    },
    alt: {
      "q": "", "w": "+", "e": "+", "r": "+", "t": "+",
      "y": "+", "u": "+", "i": "+", "o": "+", "p": "",
      "a": "", "s": "-", "d": "-", "f": "-", "g": "-",
      "h": "-", "j": "-", "k": "-", "l": "-", ";": "",
      "z": "", "x": "M", "c": "M", "v": "M", "b": "M",
      "n": "M", "m": "M", ",": "M", ".": "M", "/": "",
    },
  },
  scales: {
    major: [ 0, 2, 4, 5, 7, 9, 11 ],
    minor: [ 0, 2, 3, 5, 7, 8, 10 ],
    ionian: [ 0, 2, 4, 5, 7, 9, 11 ],
    dorian: [ 0, 2, 3, 5, 7, 9, 10 ],
    phrygian: [ 0, 1, 3, 5, 7, 8, 10 ],
    lydian: [ 0, 2, 4, 6, 7, 9, 11 ],
    mixolydian: [ 0, 2, 4, 5, 7, 9, 10 ],
    aeolian: [ 0, 2, 3, 5, 7, 8, 10 ],
    locrian: [ 0, 1, 3, 5, 6, 8, 10 ],
    harmonicMinor: [ 0, 2, 3, 5, 7, 8, 11 ],
    harmonicMajor: [ 0, 2, 4, 5, 7, 8, 11 ],
    melodicMinor: [ 0, 2, 3, 5, 7, 9, 11 ],
    melodicMinorDesc: [ 0, 2, 3, 5, 7, 8, 10 ],
    melodicMajor: [ 0, 2, 4, 5, 7, 8, 10 ],
    bartok: [ 0, 2, 4, 5, 7, 8, 10 ],
    hindu: [ 0, 2, 4, 5, 7, 8, 10 ],
  },
  kitMax: 13,
  notes: [ "c", "c#", "d", "d#", "e", "f", "f#", "g", "g#", "a", "a#", "b" ],
};

const state = {
  key: "c",
  octave: 0,
  scale: null,
  track: 0,
  channel: 0,
};


/*
 * midi to native
 */

const packMidiIn = (byte1, byte2 = 0, byte3 = 0) => {
  if (window.midiIn)
    midiIn((byte1 << 16) | (byte2 << 8) | byte3);
};

const midiNote = key => {
  const index = config.keys.keyboard.indexOf(key);
  if (state.scale === null)
    return index + state.octave * config.keys.keyboard.length;
  const scale = config.scales[state.scale];
  return scale[index % scale.length]
    + 12 * (state.octave + Math.floor(index / scale.length))
    + config.notes.indexOf(state.key);
};

const keyName = key => {
  return config.notes[midiNote(key) % config.notes.length];
};

const getKeyElement = key => {
  return document.querySelector(`[data-key="${key}"]`);
};

const onKeyChange = event => {
  if (event.ctrlKey || event.metaKey || event.repeat) {
    return;
  }
  if (config.keys.keyboard.includes(event.key)) {
    event.preventDefault();
    getKeyElement(event.key).classList.toggle("down", event.type === "keydown");
    packMidiIn(
      state.channel | (event.type === "keydown" ? 144 : 128),
      midiNote(event.key),
      event.type === "keydown" ? 100 : 0
    );
  }
  if (event.key === "v") {
    event.preventDefault();
    getKeyElement(event.key).classList.toggle("down", event.type === "keydown");
    state.channel = Math.max(state.channel - 1, 0);
  }
  if (event.key === "b") {
    event.preventDefault();
    getKeyElement(event.key).classList.toggle("down", event.type === "keydown");
    state.channel = Math.min(state.channel + 1, config.kitMax);
  }
  if (event.key === "Shift") {
    event.preventDefault();
    document.body.classList.toggle("shift", event.type === "keydown");
  }
  if (event.key === "Alt") {
    event.preventDefault();
    document.body.classList.toggle("alt", event.type === "keydown");
  }
};

document.addEventListener("keydown", onKeyChange);
document.addEventListener("keyup", onKeyChange);
document.addEventListener("keypress", event => event.preventDefault());


/*
 * midi from native
 */

const onMidiOut = midi => {
  for (const message of midi) {
    switch (message >> 16 /* status byte */) {
      case 242: /* song position pointer */
        const beat = ((message & 127) << 7) | ((message >> 8) & 127);
        const sequencePosition = (beat / 4) % config.keys.sequence.length;
        config.keys.sequence.forEach((key, index) => {
          getKeyElement(key).classList.toggle("current", index === sequencePosition);
        });
        break;
    }
  }
};

if (window.midiOut)
  setInterval(() => midiOut().then(onMidiOut), 40 /* just over 24 fps */);


/*
 * create keys
 */

const div = (attributes, children) => {
  const element = document.createElement("div");
  for (const [ key, value ] of Object.entries(attributes))
    element.setAttribute(key, value);
  for (const child of children)
    element.appendChild(child)
  return element;
};

const newKey = (key, index) => {
  const attributes = {
    "class": "key",
    "data-key": key,
    "data-shift": config.keys.shift[key],
    "data-alt": config.keys.alt[key],
  };
  if (config.keys.effects[key])
    attributes["data-name"] = config.keys.effects[key];
  if (config.keys.keyboard.includes(key))
    attributes["data-name"] = keyName(key);
  if (config.keys.sequence.includes(key))
    attributes.class += " sequence";
  if (config.keys.navigation.includes(key))
    attributes.class += " navigation";
  if (config.keys.control.includes(key))
    attributes.class += " control";
  if (config.keys.navigation.indexOf(key) === state.track)
    attributes.class += " current";
  if (index === 5)
    attributes.class += " stagger-right";
  return div(attributes, [ div({ class: "shadow" }, []) ]);
};

const newRow = (row, stagger) => {
  return div({ class: `flex row centered ${stagger}` }, row.map(newKey));
};

document.body.appendChild(newRow(config.keys.row1, "stagger-left"));
document.body.appendChild(newRow(config.keys.row2, ""));
document.body.appendChild(newRow(config.keys.row3, "stagger-right"));
