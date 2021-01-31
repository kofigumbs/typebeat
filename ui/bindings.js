const dot = (method, value) => state => state[method] === value ? '●' : '';
const bindKeys = (caps, f) => Array.from(caps, (cap, i) => [cap, f(i)]);

const bindingsByModifier = new Map([
  ['Q', { symbol: '#', mode: 'Song', actions: new Map([
  ])}],
  ['W', { symbol: '.-', mode: 'Track', actions: new Map([
    ...bindKeys('NM,./HJKL;YUIOP', i => ({
      symbol: dot('track', i),
      onDown: () => $push('track', i),
    })),
  ])}],
  ['E', { symbol: '~~', mode: 'Sample Pack', actions: new Map([
    ...bindKeys('NM,./HJKL;YUIOP', i => ({
      symbol: dot('samplePack', i),
      onDown: () => $push('samplePack', i),
    })),
  ])}],
  ['R', { symbol: '', mode: '', actions: new Map([
  ])}],
  ['T', { symbol: '!~', mode: 'Mute', actions: new Map([
    ...bindKeys('NM,./HJKL;YUIOP', i => ({
      symbol: dot(`mute:${i}`, 1),
      onDown: () => $push('mute', i),
    })),
  ])}],
  ['A', { symbol: '||=', mode: 'Source', actions: new Map([
  ])}],
  ['S', { symbol: '>>=', mode: 'Filter', actions: new Map([
  ])}],
  ['D', { symbol: '/=/', mode: 'Envelope', actions: new Map([
  ])}],
  ['F', { symbol: '=:=', mode: 'Effects', actions: new Map([
  ])}],
  ['G', { symbol: '=|', mode: 'Tape', actions: new Map([
  ])}],
  ['Z', { symbol: '~>', mode: 'LFO', actions: new Map([
  ])}],
  ['X', { symbol: '', mode: '', actions: new Map([
  ])}],
  ['C', { symbol: '', mode: '', actions: new Map([
  ])}],
  ['V', { symbol: '', mode: '', actions: new Map([
  ])}],
  ['B', { symbol: '[|', mode: 'File', actions: new Map([
  ])}],
  [undefined, { actions: new Map([
    ...bindKeys('NM,./HJKL;YUIOP', i => ({
      symbol: () => '',
      onDown: () => $push('keyDown', i),
      onUp: () => $push('keyUp', i),
    })),
  ])}],
]);
