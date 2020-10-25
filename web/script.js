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
    sequence: Array.from("qwertyuiasdfghjk"),
    control: Array.from("opl;./"),
    navigation: Array.from("zxcvbnm,"),
  },
  midi: {
    // left hand, ends at middle c
    "q": 58, "w": 60, "e": 62, "r": 64, "t": 65,
    "a": 50, "s": 52, "d": 53, "f": 55, "g": 57,
    "z": 41, "x": 43, "c": 45, "v": 46, "b": 48,
    // right hand, starts at middle c
    "y": 82, "u": 84, "i": 86, "o": 88, "p": 89,
    "h": 74, "j": 76, "k": 77, "l": 79, ";": 81,
    "n": 65, "m": 67, ",": 69, ".": 70, "/": 72,
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
  return div({ class: "key centered " + role, "data-key": key }, [ text(key) ]);
};

document.body.appendChild(div({ class: "row centered" }, keys.layout.row1.map(newKey)));
document.body.appendChild(div({ class: "row centered" }, keys.layout.row2.map(newKey)));
document.body.appendChild(div({ class: "row centered" }, keys.layout.row3.map(newKey)));


/*
 * midi to native
 */

const getKeyElement = key => {
  return document.querySelector(`[data-key="${key}"]`);
};

const onKey = (event, status, velocity) => {
  if (!event.ctrlKey && !event.altKey && !event.metaKey && keys.midi[event.key]) {
    event.preventDefault();
    getKeyElement(event.key).classList.toggle("down", velocity !== 0);
    if (window.putMidi)
      putMidi(status, keys.midi[event.key], velocity);
  }
};

document.addEventListener("keydown", event => onKey(event, 144, 100));
document.addEventListener("keyup", event => onKey(event, 128, 0));


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
