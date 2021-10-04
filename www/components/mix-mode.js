import bind from '../bind';

export const cap = 'G';

export const actions = (local, proxy, sete) => new Map([
  ...bind.oneOf('YUIOP', 'mix', ['main', 'pan', 'reverb', 'echo', 'drive'], local),
  ...bind.nudge(async () => await proxy[local.mix], i => set(local.mix, i)),
]);
