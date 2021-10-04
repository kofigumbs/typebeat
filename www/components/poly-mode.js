import bind from '../bind';

export const cap = 'R';

export const actions = (local, proxy, set) => new Map([
  ['Y', bind.toggle('use key', () => proxy.useKey, () => set('useKey')) ],
  ['J', bind.one({ label: () => 'oct. -', onDown: () => set('octave', 1) }) ],
  ['K', bind.title(() => proxy.octave) ],
  ['L', bind.one({ label: () => 'oct. +', onDown: () => set('octave', 2) }) ],
]);
