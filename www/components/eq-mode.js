import bind from '../bind';

export const cap = 'F';

export const actions = (local, proxy, set) => new Map([
  ...bind.oneOf('YUIOP', 'eqBand', ['low', 'band 1', 'band 2', 'band 3', 'high'], local),
  ...bind.oneOf('NM', 'eqFilter', ['freq.', 'res.'], local),
  ...bind.nudge(() => proxy[bind.join(local.eqBand, local.eqFilter)], i => set(bind.join(local.eqBand, local.eqFilter), i)),
]);
