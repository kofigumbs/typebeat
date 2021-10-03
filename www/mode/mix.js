import bind from '../bind';

export const bindings = (local, proxy, sete) => new Map([
  ...bind.oneOf('YUIOP', 'mix', ['main', 'pan', 'reverb', 'echo', 'drive'], local),
  ...bind.nudge(async () => await proxy[local.mix], i => set(local.mix, i)),
]);

export const visual = () => {};
export const sync = () => {};
