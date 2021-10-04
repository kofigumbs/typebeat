import bind from '../bind';

export const cap = 'B';

export const actions = (local, proxy, set) => new Map([
  ...bind.all(i => ({
    label: async () => await proxy[`muted ${i}`] ? '</>' : '==',
    onDown: () => set('muted', i),
  })),
]);
