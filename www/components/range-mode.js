import bind from '../bind';

export const cap = 'R';

export const actions = (state) => new Map([
  ['Y', bind.toggle('use key', () => state.activeTrack().useKey, () => state.send('useKey')) ],
  ['J', bind.one({ label: () => 'oct. -', onDown: () => state.send('octave', 1) }) ],
  ['K', bind.title(() => state.activeTrack().octave) ],
  ['L', bind.one({ label: () => 'oct. +', onDown: () => state.send('octave', 2) }) ],
]);
