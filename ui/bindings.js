const bindKeys = (caps, f) => Array.from(caps, (cap, i) => [cap, f(i)]);

const bindingsByModifier = new Map([
  ['Q', {
    mode: 'Sample',
    actions: new Map([
    ]),
  }],
  ['W', {
    mode: 'Osc.',
    actions: new Map([
    ]),
  }],
  ['E', {
    mode: 'LFO',
    actions: new Map([
    ]),
  }],
  ['R', {
    mode: '',
    actions: new Map([
    ]),
  }],
  ['T', {
    mode: 'Note',
    actions: new Map([
      ...bindKeys('NM,./HJKL;YUIOP', i => ({
        onDown: () => $push('noteDown', i),
        onUp: () => $push('noteUp', i),
      })),
    ]),
  }],
  ['A', {
    mode: 'Seq.',
    actions: new Map([
    ]),
  }],
  ['S', {
    mode: 'Filter',
    actions: new Map([
    ]),
  }],
  ['D', {
    mode: 'Hold',
    actions: new Map([
    ]),
  }],
  ['F', {
    mode: 'FX',
    actions: new Map([
    ]),
  }],
  ['G', {
    mode: 'Tape',
    actions: new Map([
    ]),
  }],
  ['Z', {
    mode: 'File',
    actions: new Map([
    ]),
  }],
  ['X', {
    mode: '',
    actions: new Map([
    ]),
  }],
  ['C', {
    mode: '',
    actions: new Map([
    ]),
  }],
  ['V', {
    mode: '',
    actions: new Map([
    ]),
  }],
  ['B', {
    mode: 'Mute',
    actions: new Map([
      ...bindKeys('NM,./HJKL;YUIOP', i => ({
        onDown: () => $push('mute', i),
      })),
    ]),
  }],
  [undefined, {
    actions: new Map([
      ...bindKeys('NM,./HJKL;YUIOP', i => ({
        onDown: () => $push('auditionDown', i),
        onUp: () => $push('auditionUp', i),
      })),
    ]),
  }],
]);
