import bind from '../bind';

export const bindings = (local, proxy, set) => new Map([
  ...bind.oneOf('YUIOP', 'hold', ['attack', 'decay', 'sustain', 'release', 'cutoff'], local),
  ...bind.nudge(async () => await proxy[local.hold], i => set(local.hold, i)),
  ['N', bind.toggle('sample', async () => await proxy.holdSample, () => set('holdSample')) ],
]);

export const visual = () => {};
export const sync = () => {};
