import bind from '../bind';

export const cap = 'C';

export const actions = (local, proxy, set) => new Map([
  ...bind.oneOf('YUI', 'effect', ['reverb', 'echo', 'drive'], local),
  ...bind.nudge(() => proxy[bind.join(local.effect, local.effectControl)], i => set(bind.join(local.effect, local.effectControl), i)),
  ...bind.oneOf('NM,', 'effectControl', ['gain', 'feed', 'space'], local),
]);
