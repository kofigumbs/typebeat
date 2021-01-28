const ffiGet = (method) => toUi(method);
const ffiPut = (method, value) => fromUi(method, value|0);

const bindKeys = (caps, f) => new Map(Array.from(caps, (cap, i) => [cap, f(i)]));
const bindingsByModifier = new Map([
  ['Q', { symbol: '#', mode: 'Song', actions: new Map([
  ])}],
  ['W', { symbol: '.-', mode: 'Sequence', actions: new Map([
  ])}],
  ['E', { symbol: '~~', mode: 'Sample', actions: new Map([
  ])}],
  ['R', { symbol: '', mode: '', actions: new Map([
  ])}],
  ['T', { symbol: '!~', mode: 'Mute', actions: new Map([
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
  ['Z', { symbol: '', mode: '', actions: new Map([
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
    ...bindKeys("NM,./HJKL;YUIOP", i => down => ffiPut(`key${down ? 'down' : 'up'}`, i))
  ])}],
]);
