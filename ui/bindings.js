const bindKeys = (caps, f) => Array.from(caps, (cap, i) => [cap, f(i)]);

const bindingsByModifier = new Map([
  ['Q', {
    symbol: '~~',
    mode: 'Sample',
    actions: new Map([
    ]),
  }],
  ['W', {
    symbol: '-<',
    mode: 'Oscillator',
    actions: new Map([
    ]),
  }],
  ['E', {
    symbol: '~>',
    mode: 'LFO',
    actions: new Map([
    ]),
  }],
  ['R', {
    symbol: '',
    mode: '',
    actions: new Map([
    ]),
  }],
  ['T', {
    symbol: '<~>',
    mode: 'Note',
    actions: new Map([
      ...bindKeys('NM,./HJKL;YUIOP', i => ({
        onDown: () => $push('noteDown', i),
        onUp: () => $push('noteUp', i),
      })),
    ]),
  }],
  ['A', {
    symbol: '.=',
    mode: 'Sequence',
    actions: new Map([
    ]),
  }],
  ['S', {
    symbol: '>>=',
    mode: 'Filter',
    actions: new Map([
    ]),
  }],
  ['D', {
    symbol: '/=/',
    mode: 'Envelope',
    actions: new Map([
    ]),
  }],
  ['F', {
    symbol: '=:=',
    mode: 'Effects',
    actions: new Map([
    ]),
  }],
  ['G', {
    symbol: '=|',
    mode: 'Tape',
    actions: new Map([
    ]),
  }],
  ['Z', {
    symbol: '#',
    mode: 'Project',
    actions: new Map([
    ]),
  }],
  ['X', {
    symbol: '',
    mode: '',
    actions: new Map([
    ]),
  }],
  ['C', {
    symbol: '',
    mode: '',
    actions: new Map([
    ]),
  }],
  ['V', {
    symbol: '',
    mode: '',
    actions: new Map([
    ]),
  }],
  ['B', {
    symbol: '!~',
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
