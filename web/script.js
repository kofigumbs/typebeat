/*
 * hyperscript helpers
 */

const text = string => {
  return document.createTextNode(string);
}

const div = (attributes, children) => {
  const element = document.createElement("div");
  for (const [ key, value ] of Object.entries(attributes))
    element.setAttribute(key, value);
  for (const child of children)
    element.appendChild(child)
  return element;
};


/*
 * setup keys
 */

const keys = {
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
};

const newKey = key => {
  let role;
  if (keys.role.sequence.includes(key))
    role = "sequence";
  if (keys.role.navigation.includes(key))
    role = "navigation";
  if (keys.role.control.includes(key))
    role = "control";
  return div({ class: `key ${role}`, "data-key": key }, [ div({ class: "shadow" }, []) ]);
};

const newRow = row => {
  return div({ class: "flex row centered staggered" }, row.map(newKey));
};

document.body.appendChild(newRow(keys.layout.row1));
document.body.appendChild(newRow(keys.layout.row2));
document.body.appendChild(newRow(keys.layout.row3));


/*
 * midi to native
 */

const tuning = {
  key: "c",
  scale: "major",
  octave: 4,
};

const scales = {
  major: [ 0, 2, 4, 5, 7, 9, 11 ],
  ionian: [ 0, 2, 4, 5, 7, 9, 11 ],
  dorian: [ 0, 2, 3, 5, 7, 9, 10 ],
  phrygian: [ 0, 1, 3, 5, 7, 8, 10 ],
  lydian: [ 0, 2, 4, 6, 7, 9, 11 ],
  mixolydian: [ 0, 2, 4, 5, 7, 9, 10 ],
  aeolian: [ 0, 2, 3, 5, 7, 8, 10 ],
  minor: [ 0, 2, 3, 5, 7, 8, 10 ],
  locrian: [ 0, 1, 3, 5, 6, 8, 10 ],
  harmonicMinor: [ 0, 2, 3, 5, 7, 8, 11 ],
  harmonicMajor: [ 0, 2, 4, 5, 7, 8, 11 ],
  melodicMinor: [ 0, 2, 3, 5, 7, 9, 11 ],
  melodicMinorDesc: [ 0, 2, 3, 5, 7, 8, 10 ],
  melodicMajor: [ 0, 2, 4, 5, 7, 8, 10 ],
  bartok: [ 0, 2, 4, 5, 7, 8, 10 ],
  hindu: [ 0, 2, 4, 5, 7, 8, 10 ],
  none: [ 0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11 ],
};

const note = index => {
  const scale = scales[tuning.scale];
  return scale[index % scale.length] + 12 * (octave + Math.floor(index / scale.length));
};

const getKeyElement = key => {
  return document.querySelector(`[data-key="${key}"]`);
};

const onKey = (event, { down, noteStatus, noteVelocity }) => {
  if (event.ctrlKey || event.altKey || event.metaKey)
    return true;
  if (keys.midi.right.includes(event.key)) {
    getKeyElement(event.key).classList.toggle("down", down);
    if (window.putMidi)
      putMidi(noteStatus, note(keys.midi.right.indexOf(event.key)), noteVelocity);
    return false;
  }
};

document.addEventListener("keydown", event => onKey(event, { down: true, noteStatus: 144, noteVelocity: 100 }));
document.addEventListener("keyup", event => onKey(event, { down: false, noteStatus: 128, noteVelocity: 0 }));


/*
 * midi from native
 */

const onMidiIn = midi => {
  for (const message of midi) {
    switch (message >> 16 /* status byte */) {
      case 242: /* song position pointer */
        const beat = ((message & 127) << 7) | ((message >> 8) & 127);
        const sequencePosition = (beat / 4) % keys.role.sequence.length;
        keys.role.sequence.forEach((key, index) => {
          getKeyElement(key).classList.toggle("current", index === sequencePosition);
        });
        break;
    }
  }
};

if (window.getMidi)
  setInterval(() => getMidi().then(onMidiIn), 40 /* just over 24 fps */);
