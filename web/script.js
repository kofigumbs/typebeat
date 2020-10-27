/*
 * globals
 */

const config = {
  keys: {
    layout: {
      row1: Array.from("qwertyuiop"),
      row2: Array.from("asdfghjkl;"),
      row3: Array.from("zxcvbnm,./"),
    },
    role: {
      sequence: Array.from("wertyuiosdfghjkl"),
      control: Array.from("qpa;z/"),
      navigation: Array.from("xcvbnm,."),
    },
    midi: {
      left: Array.from("zxcvbasdfgqwert"),
      right: Array.from("nm,./hjkl;yuiop"),
    },
    shift: {
      "q": "", "w": "", "e": "", "r": "", "t": "",
      "y": "", "u": "", "i": "", "o": "", "p": "\uf169",
      "a": "", "s": "", "d": "", "f": "", "g": "",
      "h": "", "j": "", "k": "", "l": "", ";": "",
      "z": "", "x": "", "c": "", "v": "", "b": "",
      "n": "", "m": "", ",": "", ".": "", "/": "",
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
    none: [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 ],
  },
  notes: [ "c", "c#", "d", "d#", "e", "f", "f#", "g", "g#", "a", "a#", "b" ],
};

const state = {
  key: "c",
  scale: "major",
  octave: 4,
};


/*
 * midi to native
 */

const packMidiIn = (byte1, byte2 = 0, byte3 = 0) => {
  if (window.midiIn)
    midiIn((byte1 << 16) | (byte2 << 8) | byte3);
};

const midiNote = key => {
  const index = config.keys.midi.right.indexOf(key);
  const scale = config.scales[state.scale];
  return scale[index % scale.length]
    + 12 * (state.octave + Math.floor(index / scale.length))
    + config.notes.indexOf(state.key);
};

const getKeyElement = key => {
  return document.querySelector(`[data-key="${key}"]`);
};

const onKeyChange = (event, { down, noteStatus, noteVelocity }) => {
  if (event.ctrlKey || event.altKey || event.metaKey) {
    return true;
  }
  if (config.keys.midi.right.includes(event.key)) {
    getKeyElement(event.key).classList.toggle("down", down);
    packMidiIn(noteStatus, midiNote(event.key), noteVelocity);
    return false;
  }
  if (event.key === "P") {
    packMidiIn(250); // play/pause
    return false;
  }
  if (event.key === "Shift") {
    document.body.classList.toggle("shift", down);
    return false;
  }
};

document.addEventListener("keydown", event => onKeyChange(event, { down: true, noteStatus: 144, noteVelocity: 100 }));
document.addEventListener("keyup", event => onKeyChange(event, { down: false, noteStatus: 128, noteVelocity: 0 }));


/*
 * midi from native
 */

const onMidiOut = midi => {
  for (const message of midi) {
    switch (message >> 16 /* status byte */) {
      case 242: /* song position pointer */
        const beat = ((message & 127) << 7) | ((message >> 8) & 127);
        const sequencePosition = (beat / 4) % config.keys.role.sequence.length;
        config.keys.role.sequence.forEach((key, index) => {
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

const newKey = key => {
  let role, name = "";
  if (config.keys.midi.right.includes(key))
    name = config.notes[midiNote(key) % config.notes.length];
  if (config.keys.role.sequence.includes(key))
    role = "sequence";
  if (config.keys.role.navigation.includes(key))
    role = "navigation";
  if (config.keys.role.control.includes(key))
    role = "control";
  const attributes = {
    class: `key ${role}`,
    "data-key": key,
    "data-name": name,
    "data-shift": config.keys.shift[key],
  };
  return div(attributes, [ div({ class: "shadow" }, []) ]);
};

const newRow = row => {
  return div({ class: "flex row centered staggered" }, row.map(newKey));
};

document.body.appendChild(newRow(config.keys.layout.row1));
document.body.appendChild(newRow(config.keys.layout.row2));
document.body.appendChild(newRow(config.keys.layout.row3));
