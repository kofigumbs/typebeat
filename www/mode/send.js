import bind from '../bind';

export const bindings = (local, proxy, set) => new Map([
  ...bind.oneOf('YUI', 'effect', ['reverb', 'echo', 'drive'], local),
  ...bind.nudge(() => proxy[join(local.effect, local.effectControl)], i => set(join(local.effect, local.effectControl), i)),
  ...bind.oneOf('NM,', 'effectControl', ['gain', 'feed', 'space'], local),
]);

export const visual = () => {};
export const sync = () => {};
