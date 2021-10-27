const noOp = () => '';
const one = options => Object.assign({ label: noOp, title: noOp, onDown: noOp, onUp: noOp }, options);

const title = label => one({ label, title: () => true });
const toggle = (label, title, onDown) => one({ label: () => label, title, onDown });

const group = (caps, f) => Array.from(caps, (cap, i) => [cap, one(f(i))]);
const oneOf = (caps, name, labels, state) => {
  state[name] = labels[0];
  return group(caps, i => ({
    label: () => labels[i],
    title: () => state[name] === labels[i],
    onDown: () => state[name] = labels[i],
  }));
};

const all = f => group('NM,./HJKL;YUIOP', f);
const nudge = (value, onDown) => [
  ['H', one({ label: () => '-10', onDown: () => onDown(0) }) ],
  ['J', one({ label: () => '-1',  onDown: () => onDown(1) }) ],
  ['K', title(value) ],
  ['L', one({ label: () => '+1',  onDown: () => onDown(2) }) ],
  [';', one({ label: () => '+10', onDown: () => onDown(3) }) ],
];

const join = (a, b) => `${a}${b[0].toUpperCase()}${b.substring(1)}`.replace(/[ .]/g, '');

const note = n => {
  const name = ['C', 'C#', 'D', 'D#', 'E', 'F', 'F#', 'G', 'G#', 'A', 'A#', 'B'][n % 12];
  const octave = Math.floor(n / 12 - 1);
  return `${name}${octave}`;
};

const comingSoon = new Map([
  ['N', one({ label: () => 'coming' })],
  ['M', one({ label: () => 'soon...' })],
]);

export default { one, title, toggle, group, oneOf, all, nudge, join, note, comingSoon };
